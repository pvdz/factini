use std::collections::VecDeque;

use super::belt::*;
use super::bouncer::*;
use super::cell::*;
use super::cli_serialize::*;
use super::cli_deserialize::*;
use super::config::*;
use super::demand::*;
use super::floor::*;
use super::machine::*;
use super::maze::*;
use super::options::*;
use super::part::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::quest_state::*;
use super::quote::*;
use super::state::*;
use super::supply::*;
use super::truck::*;
use super::utils::*;
use super::zone::*;
use super::log;

pub struct Factory {
  pub ticks: u64,
  pub floor: [Cell; FLOOR_CELLS_WH],
  pub prio: Vec<usize>,
  /**
   * Current available parts to use as supply or craft in machine.
   * These are painted in the right hand menu. The bool tells us whether to actually paint it.
   * ( icon, available )
   */
  pub available_parts_rhs_menu: Vec< (PartKind, bool ) >, // Which part and whether the player can use it yet

  pub changed: bool, // Was any part of the factory changed since last tick? Resets counters and (p)recomputes tracks.

  pub last_day_start: u64, // 1 Day is a minute worth of ticks; ONE_MS*1000*60 ticks, no matter the speed or frame rate
  pub modified_at: u64, // Track last time the factory was user manipulated. Any changes or part removal count. Score is mulled if this happens during the day.
  pub curr_day_progress: f64,
  pub finished_at: u64, // Do not set for invalid scores. If at any point before the end of day the targets have been fulfilled, set this value so it sticks at it in the UI. Zero value is ignored.
  pub finished_with: u64, // Do not set for invalid scores. If at the end of day the targets have not been fulfilled, set this value to the % of progress where it failed so it sticks in the UI. Zero value is ignored.
  pub machines: Vec<usize>, // List of main_coord for all machines, actively maintained

  pub supplied: u64,
  pub produced: u64,
  pub accepted: u64,
  pub trashed: u64,

  pub trucks: Vec<Truck>,
  pub quests: Vec<QuestState>,
  pub quest_updated: bool,
  pub parts_in_transit: Vec<(PartKind, f64, f64, u8)>,

  pub day_corrupted: bool, // Used trash as jokers to create parts in machines?

  // mouse xy, offer xy, tick start, tick duration
  pub edge_hint: (PartKind, (f64, f64), (f64, f64), u64, u64),

  // Maze. Each index tells us whether that cell is connected to the up and left, then a visited count, where 255 means dead end. then a "last direction" flag.
  pub maze: Vec<MazeCell>,
  // xorshift (poor man's "prng") seed for initial maze
  pub maze_seed: u64,
  // Position of the runner currently inside the maze
  pub maze_runner: MazeRunner,
  // Prepared stats for the next maze runner
  // TODO: maybe this should be a vec of parts that contribute to the total current value? So we can paint them in order received etc
  pub maze_prep: ( u16, u16, u16, u16 ),
}

fn dnow() -> u64 {
  js_sys::Date::now() as u64
}

pub fn create_factory(options: &Options, state: &mut State, config: &Config, floor_str: String) -> Factory {
  let ( floor, unlocked_part_icons ) = floor_from_str(options, state, config, &floor_str);
  let available_parts: Vec<PartKind> = unlocked_part_icons.iter().map(|icon| part_icon_to_kind(config,*icon)).collect();
  let available_parts_rhs_menu: Vec<(PartKind, bool)> = available_parts.iter().filter(|part| {
    // Search for this part in the default story (system nodes) and the current active story.
    // If it is part of the node list for either story then include it, otherwise exclude it.
    for (story_index, story) in config.stories.iter().enumerate() {
      if story_index == 0 || story_index == state.active_story_index {
        if story.part_nodes.contains(&(**part as usize)) {
          return true;
        }
      }
    }
    return false;
  }).map(|kind| ( *kind, true )).collect();
  let quests = get_fresh_quest_states(options, state, config, 0, &available_parts);
  log!("initial available_parts (all): {:?}", available_parts.iter().map(|index| (index, config.nodes[*index].name.clone())).collect::<Vec<_>>());
  log!("initial available_parts (active story): {:?}", available_parts_rhs_menu.iter().map(|(index, _)| config.nodes[*index].name.clone()).collect::<Vec<_>>());
  log!("active story {} nodes: {:?}", state.active_story_index, config.stories[state.active_story_index].part_nodes);
  log!("available quests: {:?}", quests.iter().filter(|quest| quest.status == QuestStatus::Active).map(|quest| quest.name.clone()).collect::<Vec<_>>());
  log!("target quest parts: {:?}", quests.iter().filter(|quest| quest.status == QuestStatus::Active).map(|quest| config.nodes[quest.production_part_index].name.clone()).collect::<Vec<_>>());

  let maze_seed = dnow();

  let mut factory = Factory {
    ticks: 0,
    floor,
    prio: vec!(),
    available_parts_rhs_menu,
    changed: true,
    last_day_start: 0,
    modified_at: 0,
    curr_day_progress: 0.0,
    finished_at: 0,
    finished_with: 0,
    machines: vec!(),
    supplied: 0,
    produced: 0,
    accepted: 0,
    trashed: 0,
    trucks: vec!(),
    parts_in_transit: vec!(),
    day_corrupted: false,
    edge_hint: (PARTKIND_NONE, (0.0, 0.0), (0.0, 0.0), 0, 0),
    quests,
    quest_updated: true,
    maze: create_maze(maze_seed),
    maze_seed,
    maze_runner: create_maze_runner(0, 0),
    maze_prep: (0, 0, 0, 0),
  };

  // log!("The maze: {:?}", factory.maze);

  auto_layout(options, state, config, &mut factory);
  auto_ins_outs(options, state, config, &mut factory);
  factory.machines = factory_collect_machines(&factory.floor);
  let prio = create_prio_list(options, config, &mut factory.floor);

  factory.prio = prio;
  factory.changed = false;
  return factory;
}

pub fn tick_factory(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory) {
  factory.ticks += 1;

  for n in 0..factory.prio.len() {
    let coord = factory.prio[n];
    factory.floor[coord].ticks += 1;

    match factory.floor[coord].kind {
      CellKind::Empty => panic!("should not have empty cells in the prio list:: prio index: {}, coord: {}, cell: {:?}", n, coord, factory.floor[coord]),
      CellKind::Belt => tick_belt(options, state, config, factory, coord),
      CellKind::Machine => tick_machine(options, state, config, factory, coord),
      CellKind::Supply => tick_supply(options, state, factory, coord),
      CellKind::Demand => tick_demand(options, state, factory, coord),
    }
  }

  tick_maze(options, state, config, factory);

  if factory.ticks % 100 == 0 {
    // Step the round-way, where parts from demanders are put and moved to the maze/furnace
    // This is not the actual painting, just updating the location in a very manual way.

    const INIT: u8 = 0;
    const WAY: u8 = 1;
    const BURN1: u8 = 2; // down
    const BURN2: u8 = 3; // left
    const BURN3: u8 = 4; // down2
    const SPECIAL: u8 = 5; // right
    const SORTING: u8 = 6; // down
    const SORTED: u8 = 7; // right
    const END: u8 = 8;

    // The way-belt size is deliberately half of the floor-cell. This way we can predictably stack
    // the way-belt around the floor and start at arbitrary offsets with little computational overhead.
    let tsize = (CELL_H/2.0).floor();

    // We need the track to be a little wider than the floor
    let roundway_min_len = FLOOR_WIDTH + 4.0;
    let roundway_track_pieces = (roundway_min_len / tsize).ceil();
    let roundway_len = roundway_track_pieces * tsize;
    // At each end of this line there will be a corner piece
    let roundway_len_full = roundway_len + 2.0 * tsize;
    // Center it and we will find our x offset
    let x = (UI_FLOOR_OFFSET_X + FLOOR_WIDTH / 2.0 - roundway_len_full / 2.0 + 3.0).floor() + 0.5;
    let y = (UI_FLOOR_OFFSET_Y + FLOOR_HEIGHT / 2.0 - roundway_len_full / 2.0 + 2.0).floor() + 0.5;
    let x2 = x + roundway_len + tsize;
    let y2 = y + roundway_len + tsize;
    let x3 = x2 + 3.0 * tsize;


    let midx = x + roundway_len_full / 2.0;
    let midy = y + roundway_len_full / 2.0;

    for t in factory.parts_in_transit.iter_mut() {
      // if t.3 == 3 {
      //   log!("phase {}, xy <{},{}>, <{}, >{}, ^{}, v{}", t.3, t.1, t.2, UI_FLOOR_OFFSET_X + CELL_W, UI_FLOOR_OFFSET_X + FLOOR_WIDTH - CELL_W, UI_FLOOR_OFFSET_Y + CELL_H, UI_FLOOR_OFFSET_Y + FLOOR_HEIGHT - CELL_H);
      // }

      if t.3 == INIT {
        // First phase: move to the roundway

        if t.1 <= UI_FLOOR_OFFSET_X + CELL_W {
          t.1 -= 1.0;
          if t.1 <= x {
            t.1 = x;
            t.3 = WAY;
          }
        }
        else if t.1 >= UI_FLOOR_OFFSET_X + FLOOR_WIDTH - CELL_W {
          t.1 += 1.0;
          if t.1 >= x2 {
            t.1 = x2;
            t.3 = WAY;
          }
        }
        else if t.2 <= UI_FLOOR_OFFSET_Y + CELL_H {
          t.2 -= 1.0;
          if t.2 <= y {
            t.2 = y;
            t.3 = WAY;
          }
        }
        else if t.2 >= UI_FLOOR_OFFSET_Y + FLOOR_HEIGHT - CELL_H {
          t.2 += 1.0;
          if t.2 >= y2 {
            t.2 = y2;
            t.3 = WAY;
          }
        }
        else {
          panic!("which side?");
        }
      }
      else if t.3 == WAY {
        // Move from left side downward
        if t.1 == x && t.2 < y2 {
          t.2 += 1.0;
          if t.2 > y2 {
            t.2 = y2;
          }
        }
        // Move from right side downward
        else if t.1 == x2 && t.2 < y2 {
          t.2 += 1.0;
          if t.2 > y2 {
            t.2 = y2;
          }
        }
        // Move from top side rightward
        else if t.2 == y && t.1 < x2 {
          t.1 += 1.0;
          if t.1 > x2 {
            t.1 = x2;
          }
        }
        // Move from bottom side rightward
        else if t.2 == y2 && t.1 < x2 {
          t.1 += 1.0;
          if t.1 > x2 {
            t.1 = x2;
          }
        }
        else if t.1 >= x2 && t.2 >= y2 {
          // Determine where to go
          if config.nodes[t.0].special.1 > 0 {
            // This is a special part, move to the right and sort into the right box
            t.1 += 1.0;
            t.3 = SPECIAL;
          } else {
            // This is a useless trash item. Move down to burn it.
            t.2 += 1.0;
            t.3 = BURN1;
          }
        }
        else {
          panic!("is this the end? <{}, {}>, x={}, x2={}, y={}, y2={}", t.1, t.2, x, x2, y, y2);
        }
      } else if t.3 == BURN1 {
        t.2 += 1.0;
        if t.2 >= y2 + 1.0 * tsize {
          t.2 = y2 + 1.0 * tsize;
          t.3 = BURN2;
        }
      } else if t.3 == BURN2 {
        t.1 -= 1.0;
        if t.1 <= x2 - 1.0 * tsize {
          t.1 = x2 - 1.0 * tsize;
          t.3 = BURN3;
        }
      } else if t.3 == BURN3 {
        t.2 += 1.0;
        if t.2 >= y2 + 2.0 * tsize {
          t.3 = END;
        }
      } else if t.3 == SPECIAL {
        t.1 += 1.0;
        if t.1 >= x3 {
          t.1 = x3;
          t.3 = SORTING;
        }
      } else if t.3 == SORTING {
        t.2 += 1.0;

        if config.nodes[t.0].special.0 == 'e' {
          if t.2 > y2 + 1.0 * tsize {
            t.2 = y2 + 1.0 * tsize;
            t.3 = SORTED;
          }
        }
        else if config.nodes[t.0].special.0 == 's' {
          if t.2 > y2 + 3.0 * tsize {
            t.2 = y2 + 3.0 * tsize;
            t.3 = SORTED;
          }
        }
        else if config.nodes[t.0].special.0 == 'p' {
          if t.2 > y2 + 5.0 * tsize {
            t.2 = y2 + 5.0 * tsize;
            t.3 = SORTED;
          }
        }
        else if config.nodes[t.0].special.0 == 'v' {
          if t.2 > y2 + 7.0 * tsize {
            t.2 = y2 + 7.0 * tsize;
            t.3 = SORTED;
          }
        }
        else {
          t.3 = END;
        }
      } else if t.3 == SORTED {
        t.1 += 1.0;
        if t.1 > x3 + 2.0 * tsize {
          match config.nodes[t.0].special.0 {
            'n' => panic!("Special kind 0 should not end up here"),
            'e' => factory.maze_prep.0 += config.nodes[t.0].special.1 as u16,
            's' => factory.maze_prep.1 += config.nodes[t.0].special.1 as u16,
            'p' => factory.maze_prep.2 += config.nodes[t.0].special.1 as u16,
            'v' => factory.maze_prep.3 += config.nodes[t.0].special.1 as u16,
            x => panic!("What is special kind {}?", x),
          }
          t.3 = END;
        }
      }
      else {
        panic!("what phase? {}", t.3);
      }
    }

    factory.parts_in_transit = factory.parts_in_transit.iter().filter(|t| t.3 != END).map(|t| *t).collect::<Vec<(PartKind, f64, f64, u8)>>();
  }

  if factory.finished_at == 0 {
    let day_ticks = ONE_MS * 1000 * 60; // one day a minute (arbitrary)
    let day_progress = (factory.ticks - factory.last_day_start) as f64 / (day_ticks as f64);
    factory.curr_day_progress = day_progress;

    factory_collect_stats(config, options, state, factory);

    if options.game_enable_clean_days && factory.finished_at <= 0 && day_progress >= 1.0 {
      factory.finished_at = factory.ticks;
      // factory.finished_with = target_progress as u64 * 100; // Store whole percentage of progress
    }
  }

  factory_tick_bouncers(options, state, config, factory);
  factory_tick_trucks(options, state, config, factory);
}

pub fn factory_collect_stats(config: &Config, options: &mut Options, state: &mut State, factory: &mut Factory) {
  let mut total_parts_supplied: u64 = 0;
  let mut total_parts_produced: u64 = 0;
  let mut total_parts_accepted: u64 = 0;
  let mut total_parts_trashed: u64 = 0;

  let collected: Vec<(char, u64)> = vec!();

  for coord in 0..factory.floor.len() {
    match factory.floor[coord].kind {
      CellKind::Empty => {} // Ignore empty cells here
      CellKind::Supply => {
        total_parts_supplied += factory.floor[coord].supply.supplied;
      }
      CellKind::Machine => {
        total_parts_produced += factory.floor[coord].machine.produced;
        total_parts_trashed += factory.floor[coord].machine.trashed;
      }
      CellKind::Belt => {} // Ignore
      CellKind::Demand => {
        for i in 0..factory.floor[coord].demand.received.len() {
          if factory.floor.len() <= coord  { log!("coord was incorrect... {} {}", factory.floor.len(), coord); }
          if factory.floor[coord].demand.received.len() <= i { log!("i was incorrect... {} {}", factory.floor[coord].demand.received.len(), i); }
          let (received_part_index, received_count) = factory.floor[coord].demand.received[i];

          // Update the quest counts (expensive search but these arrays should be small

          let mut visible_index = 0;
          for quest_index in 0..factory.quests.len() {
            if factory.quests[quest_index].status == QuestStatus::Active {
              if factory.quests[quest_index].production_part_index == received_part_index {
                factory.quests[quest_index].production_progress += received_count;
                if factory.quests[quest_index].production_progress >= factory.quests[quest_index].production_target {
                  log!("quest_update_status: production progress exceeds target, we finished {}", config.nodes[factory.quests[quest_index].config_node_index].raw_name);
                  quest_update_status(factory, quest_index, QuestStatus::FadingAndBouncing, factory.ticks);
                  factory.quests[quest_index].bouncer.bounce_from_index = visible_index;
                  factory.quests[quest_index].bouncer.bouncing_at = factory.ticks;
                }

                state.lasers.push(Laser {
                  coord,
                  visible_quest_index: visible_index,
                  ttl: 5,
                  color: "white".to_string().clone(),
                });

              }
              visible_index += 1;
            }
            else if factory.quests[quest_index].status == QuestStatus::FadingAndBouncing {
              visible_index += 1;

              let fade_progress = ((factory.ticks - factory.quests[quest_index].status_at) as f64 / QUEST_FADE_TIME as f64).min(1.0);
              if fade_progress >= 1.0 {
                log!("quest_update_status: fade finished {}", config.nodes[factory.quests[quest_index].config_node_index].raw_name);
                quest_update_status(factory, quest_index, QuestStatus::Finished, factory.ticks);
              }
            }
          }

          total_parts_accepted += received_count as u64;
        }
        factory.floor[coord].demand.received.clear();
      }
    }
  }

  factory.supplied = total_parts_supplied;
  factory.produced = total_parts_produced;
  factory.accepted = total_parts_accepted;
  factory.trashed = total_parts_trashed;
}

pub fn factory_reset_stats(options: &mut Options, state: &mut State, factory: &mut Factory) {
  for coord in 0..factory.floor.len() {
    match factory.floor[coord].kind {
      CellKind::Empty => {} // Ignore empty cells here
      CellKind::Supply => {
        factory.floor[coord].supply.supplied = 0;
      }
      CellKind::Machine => {
        factory.floor[coord].machine.produced = 0;
        factory.floor[coord].machine.trashed = 0;
      }
      CellKind::Belt => {} // Ignore
      CellKind::Demand => {
        factory.floor[coord].demand.received = vec!();
      }
    }
  }

  factory.supplied = 0;
  factory.produced = 0;
  factory.accepted = 0;
  factory.trashed = 0;

  factory.quests.iter_mut().for_each(|quest| if quest.status == QuestStatus::Active { quest.production_progress = 0; });
}

pub fn factory_load_map(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, floor_str: String) {
  let ( floor, unlocked_part_icons ) = floor_from_str(options, state, config, &floor_str);
  log!("Active quests before: {:?}", factory.quests.iter().filter(|quest| quest.status == QuestStatus::Active));
  log!("available_parts_rhs_menu (1): {:?}", factory.available_parts_rhs_menu);
  factory.floor = floor;
  let available_parts: Vec<PartKind> = unlocked_part_icons.iter().map(|icon| part_icon_to_kind(config,*icon)).collect();
  let available_parts: Vec<PartKind> = available_parts.iter().filter(|part| {
    // Search for this part in the default story (system nodes) and the current active story.
    // If it is part of the node list for either story then include it, otherwise exclude it.
    for (story_index, story) in config.stories.iter().enumerate() {
      if story_index == 0 || story_index == state.active_story_index {
        if story.part_nodes.contains(&(**part as usize)) {
          return true;
        }
      }
    }
    return false;
  }).map(|&x| x).collect();
  let available_parts_rhs_menu = available_parts.iter().map(|kind| (*kind, true)).collect();
  factory.available_parts_rhs_menu = available_parts_rhs_menu;
  log!("available_parts_rhs_menu (2): {:?}", factory.available_parts_rhs_menu);
  factory.quests = get_fresh_quest_states(options, state, config, factory.ticks, &available_parts);
  factory.quest_updated = true;
  log!("new current_active_quests: {:?}", factory.quests.iter().map(|quest| config.nodes[quest.config_node_index as usize].name.clone()).collect::<Vec<String>>().join(", "));
  auto_layout(options, state, config, factory);
  auto_ins_outs(options, state, config, factory);
  factory.machines = factory_collect_machines(&factory.floor);
  // TODO: I think we can move this (and other steps) to the factory.changed steps but there's some time between this place and the changed place
  let prio = create_prio_list(options, config, &mut factory.floor);
  factory.prio = prio;
  factory.changed = true;
  state.reset_next_frame = false;
  state.load_example_next_frame = false;
  factory.parts_in_transit.clear();
  // Clear trucks to prevent indexing problems
  factory.trucks = vec!();
}

pub fn factory_tick_bouncers(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory) {
  // Process / Step quests
  let mut visible_index = -1;
  for quest_current_index in 0..factory.quests.len() {
    if factory.quests[quest_current_index].status == QuestStatus::Active || factory.quests[quest_current_index].status == QuestStatus::FadingAndBouncing || factory.quests[quest_current_index].status == QuestStatus::Bouncing {
      visible_index += 1;
    }

    if factory.quests[quest_current_index].status == QuestStatus::FadingAndBouncing {
      let fade_progress = ((factory.ticks - factory.quests[quest_current_index].status_at) as f64 / (QUEST_FADE_TIME as f64 * options.speed_modifier_ui)).min(1.0);
      if fade_progress >= 1.0 {
        log!("quest_update_status: fade also finished {}", config.nodes[factory.quests[quest_current_index].config_node_index].raw_name);
        quest_update_status(factory, quest_current_index, QuestStatus::Bouncing, factory.ticks);
      }
    }

    if factory.quests[quest_current_index].status == QuestStatus::FadingAndBouncing || factory.quests[quest_current_index].status == QuestStatus::Bouncing {
      // At given interval (tick based, modified by ui speed) create a frame location to paint
      // This will cause something resembling a shadow trail to appear.
      let bounce_time = factory.ticks - factory.quests[quest_current_index].bouncer.bouncing_at;
      if bounce_time as f64 / (options.bouncer_time_to_factory as f64 * ONE_SECOND as f64 * options.speed_modifier_ui) < options.bouncer_stop_after && bounce_time % options.bouncer_stamp_interval == 0 {
        let xy = bouncer_xy_at_t(options, bounce_time, factory.quests[quest_current_index].bouncer.bounce_from_index);
        // log!("Adding coord {:?} to frames", xy);
        factory.quests[quest_current_index].bouncer.frames.push_back( ( xy.0, xy.1, factory.ticks ) );
      }

      // Once completely faded. Start dump truck with resources that were unlocked by quests
      // that were unlocked by finishing this one. Make sure it's had time to schedule one frame.
      // TODO: remove from tick loop and move to paint loop
      if factory.quests[quest_current_index].status == QuestStatus::Bouncing && factory.quests[quest_current_index].bouncer.frames.len() == 0 {
        log!("Marking quest {} as Finished", quest_current_index);
        factory.quests[quest_current_index].status = QuestStatus::Finished;
        // - Find out which quests were unlocked by finishing this one
        // - Find out which parts are newly available by unlocking that quest
        // - Create a dump truck with those parts
        // - Start them with some delay from each other

        // let mut new_quests: Vec<usize> = vec!();
        let mut new_parts: Vec<PartKind> = vec!();

        // Find all other waiting quests with this quest as unlock requirement
        for quest_unlock_search_index in 0..factory.quests.len() {
          if factory.quests[quest_unlock_search_index].status == QuestStatus::Waiting {
            // Note: unlock_requirement_indexes maps to factory.quests so we need the current quest index, not config node index
            let pos = factory.quests[quest_unlock_search_index].unlocks_todo.binary_search(&quest_current_index);
            if let Ok(unlock_index) = pos {
              // This quest had current_quest as a requirement. Remove it and check if it has more requirements.
              // When it doesn't, activate the quest and add all its parts to the unlocked pool.
              factory.quests[quest_unlock_search_index].unlocks_todo.remove(unlock_index);
              if factory.quests[quest_unlock_search_index].unlocks_todo.len() == 0 {
                log!("quest_update_status: unlocks todo is zero so it goes brrr {}; targets {:?} unlocks {:?}",
                  config.nodes[factory.quests[quest_unlock_search_index].config_node_index].raw_name,
                  config.nodes[factory.quests[quest_unlock_search_index].config_node_index].production_target_by_index,
                  config.nodes[factory.quests[quest_unlock_search_index].config_node_index].starting_part_by_index
                );
                quest_update_status(factory, quest_unlock_search_index, QuestStatus::Active, factory.ticks);
                // Add target parts to the new list
                for i in 0..config.nodes[factory.quests[quest_unlock_search_index].config_node_index].production_target_by_index.len() {
                  let part = config.nodes[factory.quests[quest_unlock_search_index].config_node_index].production_target_by_index[i].1;
                  // Confirm the part isn't already unlocked before starting the process to unlock it
                  if !factory.available_parts_rhs_menu.iter().any(|p| {
                    return part == p.0
                  }) && !new_parts.iter().any(|&p| {
                    p as usize == part as usize
                  }) {
                    new_parts.push(part);
                  }
                }
                // Add unlocked parts to the new list
                for i in 0..config.nodes[factory.quests[quest_unlock_search_index].config_node_index].starting_part_by_index.len() {
                  let part = config.nodes[factory.quests[quest_unlock_search_index].config_node_index].starting_part_by_index[i];
                  // Confirm the part isn't already unlocked before starting the process to unlock it
                  if !factory.available_parts_rhs_menu.iter().any(|p| {
                    return part == p.0 as usize
                  }) && !new_parts.iter().any(|&p| {
                    return p as usize == part as usize
                  }) {
                    new_parts.push(part);
                  }
                }

                factory.quest_updated = true;
              }
            }
          }
        }

        log!("Creating trucks for these new parts: {:?}", new_parts);

        // We now have a set of available quests and any starting parts that they enabled.
        // Let's create quotes and trucks for them and add them to the lists.
        new_parts.iter().enumerate().for_each(|(index, &part_index)| {
          log!("Adding truck {} for {}", index, part_index);
          factory.trucks.push(truck_create(
            factory.ticks,
            (((index + 1) as f64 * ONE_SECOND as f64) * options.speed_modifier_floor) as u64,
            part_index,
            factory.available_parts_rhs_menu.len(),
          ));
          // Add the part as a placeholder. Do not paint it yet. The truck will drive there first.
          factory.available_parts_rhs_menu.push( ( part_index , false ) );
        });
      }
    }
  }
}

fn quest_get_xy(height_so_far: f64) -> ( f64, f64 ) {
  // TODO: take mouse io into account when it is not in sync with index
  let x = UI_QUOTES_OFFSET_X + UI_QUOTE_X;
  let y = UI_QUOTES_OFFSET_Y + height_so_far;

  return ( x, y );
}

pub fn factory_tick_trucks(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory) {
  for t in 0..factory.trucks.len() {
    // TODO: fix this hack
    if factory.trucks[t].delay > 0 {
      factory.trucks[t].delay -= 1;
      if factory.trucks[t].delay == 0 {
        log!("Okay! truck {} is now ready to go!", t);
        factory.trucks[t].created_at = factory.ticks;
      }
    }
  }
}

pub fn factory_collect_machines(floor: &[Cell; FLOOR_CELLS_WH]) -> Vec<usize> {
  let mut unique = 0;
  let machines =
    floor
    .iter()
    .enumerate()
    .filter(|(index, cell)| {
      let is_main = cell.kind == CellKind::Machine && cell.machine.main_coord == *index;
      if is_main {
        unique += 1;
      }
      return is_main;
    })
    .map(|(index, cell)| { return index; })
    .collect::<Vec<usize>>();

  log!("factory_collect_machines(): cells: {}, unique: {}", machines.len(), unique);

  return machines;
}
