use super::belt::*;
use super::belt_type::*;
use super::bouncer::*;
use super::cell::*;
use super::canvas::*;
use super::cli_serialize::*;
use super::config::*;
use super::craft::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::init::*;
use super::options::*;
use super::machine::*;
use super::maze::*;
use super::offer::*;
use super::part::*;
use super::paste::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::quest_state::*;
use super::quick_save::*;
use super::quote::*;
use super::state::*;
use super::truck::*;
use super::utils::*;
use super::zone::*;
use super::log;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AutoBuildPhase {
  None,
  Startup,
  PickQuest,
  PickMachine,
  DragMachine,
  PlaceMachine,
  MoveToTargetPart,
  DragTargetPart,
  ReleaseTargetPart,
  MoveToInputPart,
  MoveToEdge,
  CreateSupplier,
  TrackToMachineStart,
  TrackToMachine,
  TrackToMachineStep,
  TrackFromMachineStart,
  TrackFromMachine,
  TrackFromMachineStep,
  Finishing,
  Finished,
}

const MOUSE_SPEED_MODIFIER_PX_P_MS: f64 = 0.1;

pub fn auto_build_next_step(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  match factory.auto_build_phase {
    AutoBuildPhase::None => {}
    AutoBuildPhase::Startup => factory.auto_build_phase = AutoBuildPhase::PickQuest,
    AutoBuildPhase::PickQuest => factory.auto_build_phase = AutoBuildPhase::PickMachine,
    AutoBuildPhase::PickMachine => factory.auto_build_phase = AutoBuildPhase::DragMachine,
    AutoBuildPhase::DragMachine => factory.auto_build_phase = AutoBuildPhase::PlaceMachine,
    AutoBuildPhase::PlaceMachine => factory.auto_build_phase = AutoBuildPhase::MoveToTargetPart,
    AutoBuildPhase::MoveToTargetPart => factory.auto_build_phase = AutoBuildPhase::DragTargetPart,
    AutoBuildPhase::DragTargetPart => factory.auto_build_phase = AutoBuildPhase::ReleaseTargetPart,
    AutoBuildPhase::ReleaseTargetPart => factory.auto_build_phase = AutoBuildPhase::MoveToInputPart,
    AutoBuildPhase::MoveToInputPart => factory.auto_build_phase = AutoBuildPhase::MoveToEdge,
    AutoBuildPhase::MoveToEdge  => factory.auto_build_phase = AutoBuildPhase::CreateSupplier,
    AutoBuildPhase::CreateSupplier => factory.auto_build_phase = AutoBuildPhase::TrackToMachineStart,
    AutoBuildPhase::TrackToMachineStart => factory.auto_build_phase = AutoBuildPhase::TrackToMachine,
    AutoBuildPhase::TrackToMachine => factory.auto_build_phase = AutoBuildPhase::TrackToMachineStep,
    AutoBuildPhase::TrackToMachineStep => factory.auto_build_phase = AutoBuildPhase::TrackToMachine, // loop
    AutoBuildPhase::TrackFromMachineStart => factory.auto_build_phase = AutoBuildPhase::TrackFromMachine,
    AutoBuildPhase::TrackFromMachine => factory.auto_build_phase = AutoBuildPhase::TrackFromMachineStep, // loop
    AutoBuildPhase::TrackFromMachineStep => factory.auto_build_phase = AutoBuildPhase::TrackFromMachine,
    AutoBuildPhase::Finishing => factory.auto_build_phase = AutoBuildPhase::Finished,
    AutoBuildPhase::Finished => factory.auto_build_phase = AutoBuildPhase::None,
  }

  // By default, pause briefly between steps
  let wait = 2000.0;
  let pause = wait / (ONE_MS as f64 * options.speed_modifier_ui);
  factory.auto_build_phase_pause = pause as u64;
  // Each step should set their own duration
  factory.auto_build_phase_duration = 0;

  factory.auto_build_phase_at = factory.ticks;
  factory.auto_build_mouse_offset_x = factory.auto_build_mouse_target_x;
  factory.auto_build_mouse_offset_y = factory.auto_build_mouse_target_y;
  auto_build_init(options, state, config, factory);
}

pub fn auto_build_init(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  log!("auto_build_init({:?})", factory.auto_build_phase);

  match factory.auto_build_phase {
    AutoBuildPhase::None => {
    }
    AutoBuildPhase::Startup => {
      auto_build_init_startup(options, state, config, factory);
    }
    AutoBuildPhase::PickQuest => {
      auto_build_init_pick_quest(options, state, config, factory);
    }
    AutoBuildPhase::PickMachine => {
      auto_build_init_pick_machine(options, state, config, factory);
    }
    AutoBuildPhase::DragMachine => {
      auto_build_init_drag_machine(options, state, config, factory);
    }
    AutoBuildPhase::PlaceMachine => {
      auto_build_init_place_machine(options, state, config, factory);
    }
    AutoBuildPhase::MoveToTargetPart => {
      auto_build_init_move_to_target_part(options, state, config, factory);
    }
    AutoBuildPhase::DragTargetPart => {
      auto_build_init_drag_target_part(options, state, config, factory);
    }
    AutoBuildPhase::ReleaseTargetPart => {
      auto_build_init_release_target_part(options, state, config, factory);
    }
    AutoBuildPhase::MoveToInputPart => {
      auto_build_init_move_to_input_part(options, state, config, factory);
    }
    AutoBuildPhase::MoveToEdge => {
      auto_build_init_move_to_edge(options, state, config, factory);
    }
    AutoBuildPhase::CreateSupplier => {
      auto_build_init_create_supplier(options, state, config, factory);
    }
    AutoBuildPhase::TrackToMachineStart => {
      auto_build_init_track_to_machine_start(options, state, config, factory);
    }
    AutoBuildPhase::TrackToMachine => {
      auto_build_init_track_to_machine(options, state, config, factory);
    }
    AutoBuildPhase::TrackToMachineStep => {
      auto_build_init_track_to_machine_step(options, state, config, factory);
    }
    AutoBuildPhase::TrackFromMachineStart => {
      auto_build_init_track_from_machine_start(options, state, config, factory);
    }
    AutoBuildPhase::TrackFromMachine => {
      auto_build_init_track_from_machine(options, state, config, factory);
    }
    AutoBuildPhase::TrackFromMachineStep => {
      auto_build_init_track_from_machine_step(options, state, config, factory);
    }
    AutoBuildPhase::Finishing => {
      auto_build_init_finishing(options, state, config, factory);
    }
    AutoBuildPhase::Finished => {
      auto_build_init_finished(options, state, config, factory);
    }
  }
}

fn auto_build_init_startup(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  // mouse_offset_x/y are initialized
  factory.auto_build_phase = AutoBuildPhase::Startup;
  factory.auto_build_phase_at = factory.ticks; // Only for this phase. Assume "next_step" sets it in other phases.
  factory.auto_build_seed = factory.ticks;
  factory.auto_build_quest_visible_index = 0;
  factory.auto_build_machine_w = 2;
  factory.auto_build_machine_h = 2;
  factory.auto_build_step_counter = 0;

  factory.auto_build_quest_index = quest_visible_index_to_quest_index(options, state, config, factory, factory.auto_build_quest_visible_index).unwrap();

  // Determine position of machine
  let mut rng = factory.ticks as usize;
  let mut rng_coords = factory.floor.iter().enumerate().map(|(i, _)| i).collect::<Vec<usize>>();
  let len = rng_coords.len();
  // Randomize the cells. TODO: prioritize by using segments of cell rings etc
  for i in 0..len {
    rng = xorshift(rng);
    let n = rng % len;
    // swap i and n
    let t = rng_coords[i];
    rng_coords[i] = rng_coords[n];
    rng_coords[n] = t;
  }
  log!("The cells: {:?}", rng_coords);

  // Now walk the cells in fixed randomized order.
  // This way we can guarantee to test each cell at least and at most once.
  let mut target = 0; // 0 is invalid because that's the unused corner edge piece
  let mut ok = true;
  for i in 0..len {
    let coord = rng_coords[i];
    let (mx, my) = to_xy(coord);
    let mut end = false;
    ok = true;
    log!("coord {} ({}x{})", coord, mx, my);
    for x in mx..mx+factory.auto_build_machine_w {
      for y in my..my+factory.auto_build_machine_h {
        log!("testing {}x{}", x, y);
        let coord = to_coord(x, y);
        if factory.floor[coord].kind != CellKind::Empty {
          // Machine would overlap with existing cell
          end = true;
          ok = false;
          log!("- {}x{} is not empty", x, y);
          break;
        }
        if x == 0 || y == 0 || x == FLOOR_CELLS_W-1 || y == FLOOR_CELLS_H - 1 {
          // Edge cell. Part of machine can't be here
          end = true;
          ok = false;
          log!("- {}x{} is edge", x, y);
          break;
        }
        log!("- {}x{} is ok.", x, y);
      }
      if end {
        break;
      }
    }
    if ok {
      log!("- offset {}x{} is ok. now testing path finding.", mx, my);
      let fake = flood_fill_get_flooded_floor(options, state, config, factory, mx, my, factory.auto_build_machine_w, factory.auto_build_machine_h, false);
      print_fake(&fake);
      log!("- checking sides {}x{} ~ {}x{}", mx, my, mx+factory.auto_build_machine_w-1, my+factory.auto_build_machine_h-1);
      // Check if there are at least three sides to the machine that can reach the edge.
      // This may be the same edge cell. If that's the case then it will bail or backtrack.
      let mut counter = 0;
      for x in mx..mx+factory.auto_build_machine_w {
        // top and bottom
        log!("?: {}x{} : {}          {}x{} : {}", x, my - 1, fake[x + (my - 1) * FLOOR_CELLS_W], x, my + factory.auto_build_machine_h, fake[x + (my + factory.auto_build_machine_h) * FLOOR_CELLS_W]);
        if fake[x + (my - 1) * FLOOR_CELLS_W] < FLOOD_FILL_EMPTY_FRESH { counter += 1; }
        if fake[x + (my + factory.auto_build_machine_h) * FLOOR_CELLS_W] < FLOOD_FILL_EMPTY_FRESH { counter += 1; }
      }
      for y in my..my+factory.auto_build_machine_h {
        // left and right
        log!("?: {}x{} : {}          {}x{} : {}", mx - 1, y, fake[(mx - 1) + y * FLOOR_CELLS_W], mx + factory.auto_build_machine_w, y, fake[(mx + factory.auto_build_machine_w) + y * FLOOR_CELLS_W]);
        if fake[(mx - 1) + y * FLOOR_CELLS_W] < FLOOD_FILL_EMPTY_FRESH { counter += 1; }
        if fake[(mx + factory.auto_build_machine_w) + y * FLOOR_CELLS_W] < FLOOD_FILL_EMPTY_FRESH { counter += 1; }
      }
      if counter >= 3 {
        log!("Found {} machine sides with paths to edge. Found target! {}x{}", counter, mx, my);
        ok = true;
        target = to_coord(mx, my);
        break;
      } else {
        log!("Found {} machine sides with paths to edge. Not enough at {}x{}", counter, mx, my);
      }
    }
  }
  if ok {
    let (x, y) = to_xy(target);
    log!("Going to place machine at @{}, {}x{}", target, x, y);
    factory.auto_build_machine_x = x;
    factory.auto_build_machine_y = y;
  } else {
    log!("Was unable to find a suitable location for the machine. Bailing");
    factory.auto_build_phase = AutoBuildPhase::Finishing;
    auto_build_init(options, state, config, factory);
    return;
  }

  // factory.auto_build_phase = AutoBuildPhase::Finishing;
  // auto_build_init(options, state, config, factory);
  // return;


  // We want to move the cursor to selected quest
  let quest_xy = get_quest_xy(0, 0.0);
  factory.auto_build_mouse_target_x = quest_xy.0 + 100.0;
  factory.auto_build_mouse_target_y = quest_xy.1 + 10.0;

  // Determine duration based on a desired mouse speed constant
  let distance = ((factory.auto_build_mouse_target_x - factory.auto_build_mouse_offset_x).abs().powf(2.0) + (factory.auto_build_mouse_target_y - factory.auto_build_mouse_offset_y).abs().powf(2.0)).sqrt();
  let ms = distance / MOUSE_SPEED_MODIFIER_PX_P_MS;
  let duration = ms / (ONE_MS as f64 * options.speed_modifier_ui);
  factory.auto_build_phase_duration = duration as u64;

  log!("AutoBuild: Moving from {}x{} to {}x{} (distance {} px) in {} ticks", factory.auto_build_mouse_offset_x, factory.auto_build_mouse_offset_y, factory.auto_build_mouse_target_x, factory.auto_build_mouse_target_y, distance.floor(), duration.floor());
}

fn auto_build_init_pick_quest(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
}

fn auto_build_init_pick_machine(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  // Move to the target machine
  factory.auto_build_mouse_target_x = UI_MENU_MACHINE_BUTTON_2X2_X;
  factory.auto_build_mouse_target_y = UI_MENU_MACHINE_BUTTON_2X2_Y;

  // Determine duration based on a desired mouse speed constant
  let distance = ((factory.auto_build_mouse_target_x - factory.auto_build_mouse_offset_x).abs().powf(2.0) + (factory.auto_build_mouse_target_y - factory.auto_build_mouse_offset_y).abs().powf(2.0)).sqrt();
  let ms = distance / MOUSE_SPEED_MODIFIER_PX_P_MS;
  let duration = ms / (ONE_MS as f64 * options.speed_modifier_ui);
  factory.auto_build_phase_duration = duration as u64;

  log!("AutoBuild: Moving from {}x{} to machine {}x{} (distance {} px) in {} ticks", factory.auto_build_mouse_offset_x, factory.auto_build_mouse_offset_y, factory.auto_build_mouse_target_x, factory.auto_build_mouse_target_y, distance.floor(), duration.floor());
}

fn auto_build_init_drag_machine(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  // Move to the target machine
  factory.auto_build_mouse_target_x = UI_FLOOR_OFFSET_X + factory.auto_build_machine_x as f64 * CELL_W;
  factory.auto_build_mouse_target_y = UI_FLOOR_OFFSET_Y + factory.auto_build_machine_y as f64 * CELL_H;

  // Determine duration based on a desired mouse speed constant
  let distance = ((factory.auto_build_mouse_target_x - factory.auto_build_mouse_offset_x).abs().powf(2.0) + (factory.auto_build_mouse_target_y - factory.auto_build_mouse_offset_y).abs().powf(2.0)).sqrt();
  let ms = distance / MOUSE_SPEED_MODIFIER_PX_P_MS;
  let duration = ms / (ONE_MS as f64 * options.speed_modifier_ui);
  factory.auto_build_phase_duration = duration as u64;

  log!("AutoBuild: Moving from {}x{} to machine {}x{} ({}x{}) (distance {} px) in {} ticks", factory.auto_build_mouse_offset_x, factory.auto_build_mouse_offset_y, factory.auto_build_machine_x, factory.auto_build_machine_y, factory.auto_build_mouse_target_x, factory.auto_build_mouse_target_y, distance.floor(), duration.floor());
}

fn auto_build_init_place_machine(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  factory.auto_build_phase_pause = 0;

  machine_add_to_factory(options, state, config, factory, factory.auto_build_machine_x, factory.auto_build_machine_y, factory.auto_build_machine_w, factory.auto_build_machine_h);

  log!("AutoBuild: Put the {}x{} machine down at {}x{}", factory.auto_build_machine_w, factory.auto_build_machine_h, factory.auto_build_machine_x, factory.auto_build_machine_y);
}

fn auto_build_init_move_to_target_part(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  // Figure out which quest it picked
  // Figure out what the target is
  // Figure out where the offer for that target is located
  // Move to it

  let part_kind: PartKind = factory.quests[factory.auto_build_quest_index.min(factory.quests.len() - 1)].production_part_kind;
  let visible_offer_index = part_kind_to_visible_offer_index(config, factory, part_kind).unwrap();
  let part_xy = get_offer_xy(visible_offer_index);

  factory.auto_build_mouse_target_x = part_xy.0;
  factory.auto_build_mouse_target_y = part_xy.1;

  // Determine duration based on a desired mouse speed constant
  let distance = ((factory.auto_build_mouse_target_x - factory.auto_build_mouse_offset_x).abs().powf(2.0) + (factory.auto_build_mouse_target_y - factory.auto_build_mouse_offset_y).abs().powf(2.0)).sqrt();
  let ms = distance / MOUSE_SPEED_MODIFIER_PX_P_MS;
  let duration = ms / (ONE_MS as f64 * options.speed_modifier_ui);
  factory.auto_build_phase_duration = duration as u64;

  log!("AutoBuild: Moving from {}x{} to offer {} at {}x{} (distance {} px) in {} ticks", factory.auto_build_mouse_offset_x, factory.auto_build_mouse_offset_y, visible_offer_index, factory.auto_build_mouse_target_x, factory.auto_build_mouse_target_y, distance.floor(), duration.floor());

}

fn auto_build_init_drag_target_part(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
// Move to the machine while dragging that part
  factory.auto_build_mouse_target_x = UI_FLOOR_OFFSET_X + factory.auto_build_machine_x as f64 * CELL_W;
  factory.auto_build_mouse_target_y = UI_FLOOR_OFFSET_Y + factory.auto_build_machine_y as f64 * CELL_H;

  // Determine duration based on a desired mouse speed constant
  let distance = ((factory.auto_build_mouse_target_x - factory.auto_build_mouse_offset_x).abs().powf(2.0) + (factory.auto_build_mouse_target_y - factory.auto_build_mouse_offset_y).abs().powf(2.0)).sqrt();
  let ms = distance / MOUSE_SPEED_MODIFIER_PX_P_MS;
  let duration = ms / (ONE_MS as f64 * options.speed_modifier_ui);
  factory.auto_build_phase_duration = duration as u64;

  log!("AutoBuild: Moving from offer {}x{} back to machine at {}x{} (distance {} px) in {} ticks", factory.auto_build_mouse_offset_x, factory.auto_build_mouse_offset_y, factory.auto_build_mouse_target_x, factory.auto_build_mouse_target_y, distance.floor(), duration.floor());
}

fn auto_build_init_release_target_part(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  factory.auto_build_phase_pause = 0;

  // Update machine to the pattern of the dragged part

  let dragged_part_kind: PartKind = factory.quests[factory.auto_build_quest_index.min(factory.quests.len() - 1)].production_part_kind;
  let main_coord = to_coord(factory.auto_build_machine_x, factory.auto_build_machine_y);
  machine_set_target_part_kind(options, state, config, factory, main_coord, dragged_part_kind);

  // Prepare for next step
  factory.auto_build_step_counter = 0;

  log!("AutoBuild: Updated the target output of the machine to {}", dragged_part_kind);
}

fn auto_build_init_move_to_input_part(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  log!("AutoBuildPhase::MoveToInputPart: {}", factory.auto_build_step_counter);
  // Pick the next input required for the current target part.

  // From the machine, find a path to an edge cell then use that
  // cell as the offset to generate a path towards the machine

  // Then schedule to move to the part and drag it to that cell.

  let current_step_index = factory.auto_build_step_counter;

  let target_part_kind: PartKind = factory.quests[factory.auto_build_quest_index.min(factory.quests.len() - 1)].production_part_kind;
  let inputs = &config.nodes[target_part_kind].pattern_unique_kinds;

  if current_step_index >= inputs.len() {
    log!("Processed all inputs. Now to create an outward path...");
    factory.auto_build_current_path = vec!();
    factory.auto_build_phase = AutoBuildPhase::TrackFromMachineStart;
    auto_build_init(options, state, config, factory);
    return;
  }

  let input_part_kind = inputs[current_step_index];

  let visible_offer_index = part_kind_to_visible_offer_index(config, factory, input_part_kind).unwrap();
  let part_xy = get_offer_xy(visible_offer_index);

  factory.auto_build_mouse_target_x = part_xy.0;
  factory.auto_build_mouse_target_y = part_xy.1;

  // Determine duration based on a desired mouse speed constant
  let distance = ((factory.auto_build_mouse_target_x - factory.auto_build_mouse_offset_x).abs().powf(2.0) + (factory.auto_build_mouse_target_y - factory.auto_build_mouse_offset_y).abs().powf(2.0)).sqrt();
  let ms = distance / MOUSE_SPEED_MODIFIER_PX_P_MS;
  let duration = ms / (ONE_MS as f64 * options.speed_modifier_ui);
  factory.auto_build_phase_duration = duration as u64;
}

fn auto_build_init_move_to_edge(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  // Assume we're now at the current input part offer.
  // Find an edge cell with at least one available path to the machine.
  // Move to it and create a demander with this input part

  let current_step_index = factory.auto_build_step_counter;

  // - Start at the machine coord. Any.
  // - Try to find a path from any cell neighboring the machine to the edge of the floor
  // - Can only traverse empty cells this way. Doesn't matter how.
  // - Once a path was found, use that edge cell.
  // - To simplify this, we can "flood fill" over empty cells the machine. First edge cell we touch is our target.

  let nearest_edge = flood_fill_find_reachable_edge_from_machine(options, state, config, factory);
  if nearest_edge == None {
    // TODO: we could blink red or something
    log!("Unable to plot a path to the edge ... bailing.");

    factory.auto_build_phase = AutoBuildPhase::Finishing;
    auto_build_init(options, state, config, factory);
    return;
  }
  let (nearest_edge_x, nearest_edge_y) = nearest_edge.unwrap();
  factory.auto_build_target_edge_x = nearest_edge_x;
  factory.auto_build_target_edge_y = nearest_edge_y;

  let target_part_kind: PartKind = factory.quests[factory.auto_build_quest_index.min(factory.quests.len() - 1)].production_part_kind;
  let inputs = &config.nodes[target_part_kind].pattern_unique_kinds;
  let input_part_kind = inputs[current_step_index];

  let visible_offer_index = part_kind_to_visible_offer_index(config, factory, input_part_kind).unwrap();
  let part_xy = get_offer_xy(visible_offer_index);

  factory.auto_build_mouse_target_x = UI_FLOOR_OFFSET_X + nearest_edge_x as f64 * CELL_W;
  factory.auto_build_mouse_target_y = UI_FLOOR_OFFSET_Y + nearest_edge_y as f64 * CELL_H;

  // Determine duration based on a desired mouse speed constant
  let distance = ((factory.auto_build_mouse_target_x - factory.auto_build_mouse_offset_x).abs().powf(2.0) + (factory.auto_build_mouse_target_y - factory.auto_build_mouse_offset_y).abs().powf(2.0)).sqrt();
  let ms = distance / MOUSE_SPEED_MODIFIER_PX_P_MS;
  let duration = ms / (ONE_MS as f64 * options.speed_modifier_ui);
  factory.auto_build_phase_duration = duration as u64;
}

fn auto_build_init_create_supplier(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  factory.auto_build_phase_pause = 0;

  // Create it on auto_build_target_edge_x/y

  let current_step_index = factory.auto_build_step_counter;
  let target_part_kind: PartKind = factory.quests[factory.auto_build_quest_index.min(factory.quests.len() - 1)].production_part_kind;
  let inputs = &config.nodes[target_part_kind].pattern_unique_kinds;
  let input_part_kind = inputs[current_step_index];

  set_edge_to_part(options, state, config, factory, factory.auto_build_target_edge_x, factory.auto_build_target_edge_y, input_part_kind);

  factory.auto_build_current_path = vec!((factory.auto_build_target_edge_x, factory.auto_build_target_edge_y));
}

fn auto_build_init_track_to_machine_start(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  // Pause here briefly
  let wait = 1000.0;
  let duration = wait / (ONE_MS as f64 * options.speed_modifier_ui);
  factory.auto_build_phase_pause = duration as u64;
}

fn auto_build_init_track_to_machine(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  // Pause here briefly
  let wait = 1000.0;
  let duration = wait / (ONE_MS as f64 * options.speed_modifier_ui);
  factory.auto_build_phase_pause = duration as u64;

  // Flood fill path finding and generate a path from the current point to the machine
  // We need to remember previously placed path parts in case we have to backtrack.

  if factory.auto_build_current_path.len() == 0 {
    log!("Path is empty. Unable to craft path. Bailing.");
    factory.auto_build_phase = AutoBuildPhase::Finishing;
    auto_build_init(options, state, config, factory);
    return;
  }

  let (current_end_x, current_end_y) = factory.auto_build_current_path[factory.auto_build_current_path.len() - 1];
  if
    current_end_x >= factory.auto_build_machine_x && current_end_x <= factory.auto_build_machine_x + factory.auto_build_machine_w - 1 &&
    current_end_y >= factory.auto_build_machine_y && current_end_y <= factory.auto_build_machine_y + factory.auto_build_machine_h - 1
  {
    // Reached a machine. TODO: or a corner of it.
    log!("Reached the machine. Path is finished! Machine is at {}x{} {}x{}, and {}x{} is within {}x{} {}x{}", factory.auto_build_machine_x, factory.auto_build_machine_y, factory.auto_build_machine_x + factory.auto_build_machine_w - 1, factory.auto_build_machine_y + factory.auto_build_machine_h - 1, current_end_x, current_end_y, factory.auto_build_machine_x - 1, factory.auto_build_machine_y - 1, factory.auto_build_machine_x + factory.auto_build_machine_w, factory.auto_build_machine_y + factory.auto_build_machine_h);

    factory.auto_build_step_counter += 1;
    factory.auto_build_phase = AutoBuildPhase::MoveToInputPart;
    auto_build_init(options, state, config, factory);
    return;
  }

  let (last_x, last_y) = factory.auto_build_current_path[factory.auto_build_current_path.len() - 1];
  let next = flood_fill_next_step_to_connect_machine(options, state, config, factory, last_x, last_y);

  if next == None {
    // TODO: backtrack
    log!("Unable to create path. Should backtrack but will bail now");
    factory.auto_build_phase = AutoBuildPhase::Finishing;
    auto_build_init(options, state, config, factory);
    return;
  }

  let (next_x, next_y) = next.unwrap();
  factory.auto_build_mouse_target_x = UI_FLOOR_OFFSET_X + next_x as f64 * CELL_W;
  factory.auto_build_mouse_target_y = UI_FLOOR_OFFSET_Y + next_y as f64 * CELL_H;
  factory.auto_build_current_path.push((next_x, next_y));

  // TODO: show track preview like you're dragging

  // Determine duration based on a desired mouse speed constant
  let distance = ((factory.auto_build_mouse_target_x - factory.auto_build_mouse_offset_x).abs().powf(2.0) + (factory.auto_build_mouse_target_y - factory.auto_build_mouse_offset_y).abs().powf(2.0)).sqrt();
  let ms = distance / MOUSE_SPEED_MODIFIER_PX_P_MS;
  let duration = ms / (ONE_MS as f64 * options.speed_modifier_ui);
  factory.auto_build_phase_duration = duration as u64;
}

fn auto_build_init_track_to_machine_step(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  factory.auto_build_phase_pause = 0;

  log!("Now create the track...");

  let (prev_x, prev_y) = factory.auto_build_current_path[factory.auto_build_current_path.len() - 2];
  let (next_x, next_y) = factory.auto_build_current_path[factory.auto_build_current_path.len() - 1];

  let prev_coord = to_coord(prev_x, prev_y);
  let next_coord = to_coord(next_x, next_y);

  belt_connect_cells_expensive(options, state, config, factory, prev_x, prev_y, next_x, next_y);
}

fn auto_build_init_track_from_machine_start(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  // Pause here briefly
  let wait = 1000.0;
  let duration = wait / (ONE_MS as f64 * options.speed_modifier_ui);
  factory.auto_build_phase_pause = duration as u64;
}

fn auto_build_init_track_from_machine(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  // Pause here briefly
  let wait = 1000.0;
  let duration = wait / (ONE_MS as f64 * options.speed_modifier_ui);
  factory.auto_build_phase_pause = duration as u64;


  // Starting at the machine, plot a track to an edge,
  // step by step, until you reach an edge or get stuck.

  let next_step =
    if factory.auto_build_current_path.len() == 0 {
      log!("TrackFromMachine: First step...");
      // Start of path. Flood fill from machine and pick the first cell
      // next to the machine that has a path to the edge.
      // We currently can't be too picky here, though we could improve
      // that with a reverse flood such that we know the side with the
      // shortest route to an available edge. TODO

      let fake = flood_fill_get_flooded_floor(
        options, state, config, factory,
        factory.auto_build_machine_x, factory.auto_build_machine_y, factory.auto_build_machine_w, factory.auto_build_machine_h,
        true
      );

      // Find the neighbor cell that has a path. When found,
      // add it and the machine cell next to it to the path.

      fn find(factory: &mut Factory, fake: &Vec<i32>) -> Option<(usize, usize)> {
        let mut best_from = 0;
        let mut best_coord = 0;
        let mut best_score = FLOOD_FILL_EMPTY_FRESH;

        for x in factory.auto_build_machine_x..factory.auto_build_machine_x+factory.auto_build_machine_w {
          // top and bottom
          let c = x + (factory.auto_build_machine_y - 1) * FLOOR_CELLS_W;
          let p = fake[c];
          if p > 1 && p < best_score {
            best_from = to_coord(x, factory.auto_build_machine_y);
            best_score = p;
            best_coord = c;
          }
          let c = x + (factory.auto_build_machine_y + factory.auto_build_machine_h) * FLOOR_CELLS_W;
          let p = fake[c];
          if p > 1 && p < best_score {
            best_from = to_coord(x, factory.auto_build_machine_y + factory.auto_build_machine_h - 1);
            best_score = p;
            best_coord = c;
          }
        }
        for y in factory.auto_build_machine_y..factory.auto_build_machine_y+factory.auto_build_machine_h {
          // left and right
          let c = (factory.auto_build_machine_x - 1) + y * FLOOR_CELLS_W;
          let p = fake[c];
          if p > 1 && p < best_score {
            best_from = to_coord(factory.auto_build_machine_x, y);
            best_score = p;
            best_coord = c;
          }
          let c = (factory.auto_build_machine_x + factory.auto_build_machine_w) + y * FLOOR_CELLS_W;
          let p = fake[c];
          if p > 1 && p < best_score {
            best_from = to_coord(factory.auto_build_machine_x + factory.auto_build_machine_w - 1, y);
            best_score = p;
            best_coord = c;
          }
        }

        if best_score == FLOOD_FILL_EMPTY_FRESH {
          return None;
        }

        factory.auto_build_current_path = vec!(to_xy(best_from));
        return Some(to_xy(best_coord));
      }

      // TODO: should actually first move the mouse to the machine part that we found...
      find(factory, &fake)
    } else {
      log!("TrackFromMachine: Next step... {}", factory.auto_build_current_path.len());

      fn find(options: &Options, state: &State, config: &Config, factory: &mut Factory) -> Option<(usize, usize)> {
        let (last_x, last_y) = factory.auto_build_current_path[factory.auto_build_current_path.len() - 1];
        let coord = to_coord(last_x, last_y);

        let nc = to_coord_up(coord);
        let (nx, ny) = to_xy(nc);
        if is_edge(nx as f64, ny as f64) && factory.floor[nc].kind == CellKind::Empty {
          return Some((nx, ny))
        }

        let nc = to_coord_right(coord);
        let (nx, ny) = to_xy(nc);
        if is_edge(nx as f64, ny as f64) && factory.floor[nc].kind == CellKind::Empty {
          return Some((nx, ny))
        }

        let nc = to_coord_down(coord);
        let (nx, ny) = to_xy(nc);
        if is_edge(nx as f64, ny as f64) && factory.floor[nc].kind == CellKind::Empty {
          return Some((nx, ny))
        }

        let nc = to_coord_left(coord);
        let (nx, ny) = to_xy(nc);
        if is_edge(nx as f64, ny as f64) && factory.floor[nc].kind == CellKind::Empty {
          return Some((nx, ny))
        }

        let fake = flood_fill_get_flooded_floor(options, state, config, factory, last_x, last_y, 1, 1, true);

        let mut best_coord = 0;
        let mut best_score = FLOOD_FILL_EMPTY_FRESH;

        let p = fake[coord - FLOOR_CELLS_W];
        if p > 1 && p < best_score {
          best_score = p;
          best_coord = coord - FLOOR_CELLS_W;
        }

        let p = fake[coord - 1];
        if p > 1 && p < best_score {
          best_score = p;
          best_coord = coord - 1;
        }

        let p = fake[coord + FLOOR_CELLS_W];
        if p < best_score {
          best_score = p;
          best_coord = coord + FLOOR_CELLS_W;
        }

        let p = fake[coord + 1];
        if p < best_score {
          best_score = p;
          best_coord = coord + 1;
        }

        if best_score == FLOOD_FILL_EMPTY_FRESH {
          return None
        }

        return Some(to_xy(best_coord));
      }

      find(options, state, config, factory)
    };

  log!("next step: {:?}", next_step);
  if next_step == None {
    // TODO: we could blink red or something
    log!("Unable to plot a path to the edge from this point ... bailing.");

    factory.auto_build_phase = AutoBuildPhase::Finishing;
    auto_build_init(options, state, config, factory);
    return;
  }

  let (next_x, next_y) = next_step.unwrap();

  factory.auto_build_current_path.push((next_x, next_y));

  factory.auto_build_mouse_target_x = UI_FLOOR_OFFSET_X + next_x as f64 * CELL_W;
  factory.auto_build_mouse_target_y = UI_FLOOR_OFFSET_Y + next_y as f64 * CELL_H;

  // Determine duration based on a desired mouse speed constant
  let distance = ((factory.auto_build_mouse_target_x - factory.auto_build_mouse_offset_x).abs().powf(2.0) + (factory.auto_build_mouse_target_y - factory.auto_build_mouse_offset_y).abs().powf(2.0)).sqrt();
  let ms = distance / MOUSE_SPEED_MODIFIER_PX_P_MS;
  let duration = ms / (ONE_MS as f64 * options.speed_modifier_ui);
  factory.auto_build_phase_duration = duration as u64;
}

fn auto_build_init_track_from_machine_step(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  factory.auto_build_phase_pause = 0;

  log!("Now create the track...");

  let (prev_x, prev_y) = factory.auto_build_current_path[factory.auto_build_current_path.len() - 2];
  let (next_x, next_y) = factory.auto_build_current_path[factory.auto_build_current_path.len() - 1];

  let prev_coord = to_coord(prev_x, prev_y);
  let next_coord = to_coord(next_x, next_y);

  belt_connect_cells_expensive(options, state, config, factory, prev_x, prev_y, next_x, next_y);

  if is_edge(next_x as f64, next_y as f64) {
    log!("this connected to an edge so this is the end, right?");
    factory.auto_build_phase = AutoBuildPhase::Finishing;
    auto_build_init(options, state, config, factory);
    return;
  }
}

fn auto_build_init_finishing(options: &Options, state: &State, config: &Config, factory: &mut Factory) {

}

fn auto_build_init_finished(options: &Options, state: &State, config: &Config, factory: &mut Factory) {

}

fn flood_fill_find_reachable_edge_from_machine(options: &Options, state: &State, config: &Config, factory: &Factory) -> Option<(usize, usize)> {
  return flood_fill_find_reachable_edge_from(options, state, config, factory,
    factory.auto_build_machine_x, factory.auto_build_machine_y, factory.auto_build_machine_w, factory.auto_build_machine_h
  );
}

fn flood_fill_find_reachable_edge_from(options: &Options, state: &State, config: &Config, factory: &Factory, ox: usize, oy: usize, ow: usize, oh: usize) -> Option<(usize, usize)> {
  log!("flood_fill_find_reachable_edge_from_machine()");
  let fake = flood_fill_get_flooded_floor(options, state, config, factory, ox, oy, ow, oh, false);

  // log!("fake: {:?}", fake);
  log!("Determining target edge cell");

  // Check if there's an edge that is connected to a visited cell.  If so, find the one with the lowest value.
  // Note: we check p>1 because we do not allow to connect suppliers to machines without a belt
  let mut min_value = 900;
  let mut min_x = FLOOR_CELLS_W;
  let mut min_y = FLOOR_CELLS_H;
  for x in 1..FLOOR_CELLS_W-1 {
    if fake[x] == FLOOD_FILL_EDGE {
      let p = fake[x + FLOOR_CELLS_W];
      if p > 1 && p < min_value {
        min_value = p;
        min_x = x;
        min_y = 0;
      }
    }
    if fake[FLOOR_CELLS_WH - FLOOR_CELLS_W + x] == FLOOD_FILL_EDGE {
      let p = fake[FLOOR_CELLS_WH - FLOOR_CELLS_W + x - FLOOR_CELLS_W];
      if p > 1 && p < min_value {
        min_value = p;
        min_x = x;
        min_y = FLOOR_CELLS_H - 1;
      }
    }
  }
  for y in 1..FLOOR_CELLS_H-1 {
    if fake[(FLOOR_CELLS_W - 1) + y * FLOOR_CELLS_W] == FLOOD_FILL_EDGE {
      let p = fake[(FLOOR_CELLS_W - 1) + y * FLOOR_CELLS_W - 1];
      if p > 1 && p < min_value {
        min_value = p;
        min_x = FLOOR_CELLS_W - 1;
        min_y = y;
      }
    }
    if fake[y * FLOOR_CELLS_W] == FLOOD_FILL_EDGE {
      let p = fake[y * FLOOR_CELLS_W + 1];
      if p > 1 && p < min_value {
        min_value = p;
        min_x = 0;
        min_y = y;
      }
    }
  }

  // print_fake(&fake);

  log!("-- edge found: {} {} with value {}", min_x, min_y, min_value);

  if min_value == 900 {
    return None;
  }

  return Some((min_x, min_y));
}

fn flood_fill_next_step_to_connect_machine(options: &Options, state: &State, config: &Config, factory: &Factory, ox: usize, oy: usize) -> Option<(usize, usize)> {
  log!("flood_fill_next_step_to_connect_machine({}, {}, to, {}, {}, {}, {}", ox, oy, factory.auto_build_machine_x, factory.auto_build_machine_y, factory.auto_build_machine_w, factory.auto_build_machine_h);

  let fake = flood_fill_get_flooded_floor(options, state, config, factory, factory.auto_build_machine_x, factory.auto_build_machine_y, factory.auto_build_machine_w, factory.auto_build_machine_h, false);

  // If the machine can reach the given ox/oy then an empty neighbor cell must now be visited

  let coord = to_coord(ox, oy);

  let mut min_value = FLOOD_FILL_EMPTY_FRESH;
  let mut min_x = FLOOR_CELLS_W;
  let mut min_y = FLOOR_CELLS_W;

  // Check all four directions. Pick the lowest visited neighbor cell.

  if coord >= FLOOR_CELLS_W {
    // log!("- Checking up: {:?} {:?} --> {}", factory.floor[to_coord_up(coord)].kind, to_coord_up(coord), fake[to_coord_up(coord)]);
    if fake[to_coord_up(coord)] == 1 || (factory.floor[to_coord_up(coord)].kind == CellKind::Empty && fake[to_coord_up(coord)] < min_value) {
      min_value = fake[to_coord_up(coord)];
      min_x = ox;
      min_y = oy - 1;
    }
  }
  if coord < FLOOR_CELLS_WH-1 {
    // log!("- Checking right: {:?} {:?} --> {}", factory.floor[to_coord_right(coord)].kind, to_coord_right(coord), fake[to_coord_right(coord)]);
    if fake[to_coord_right(coord)] == 1 || (factory.floor[to_coord_right(coord)].kind == CellKind::Empty && fake[to_coord_right(coord)] < min_value) {
      min_value = fake[to_coord_right(coord)];
      min_x = ox + 1;
      min_y = oy;
    }
  }
  if coord < FLOOR_CELLS_WH-FLOOR_CELLS_W-1 {
    // log!("- Checking down: {:?} {:?} --> {}", factory.floor[to_coord_down(coord)].kind, to_coord_down(coord + FLOOR_CELLS_W), fake[to_coord_down(coord)]);
    if fake[to_coord_down(coord)] == 1 || (factory.floor[to_coord_down(coord)].kind == CellKind::Empty && fake[to_coord_down(coord)] < min_value) {
      min_value = fake[to_coord_down(coord)];
      min_x = ox;
      min_y = oy + 1;
    }
  }
  if coord > 0 {
    // log!("- Checking left: {:?} {:?} --> {}", factory.floor[to_coord_left(coord)].kind, to_coord_left(coord - 1), fake[to_coord_left(coord)]);
    if fake[to_coord_left(coord)] == 1 || (factory.floor[to_coord_left(coord)].kind == CellKind::Empty && fake[to_coord_left(coord)] < min_value) {
      min_value = fake[to_coord_left(coord)];
      min_x = ox - 1;
      min_y = oy;
    }
  }

  if min_value == FLOOD_FILL_EMPTY_FRESH {
    // No path. Backtrack or bail.
    log!("-- No path found");
    return None;
  }

  log!("-- Next: {} {} with {}", min_x, min_y, min_value);
  return Some((min_x, min_y));
}

const FLOOD_FILL_EMPTY_FRESH: i32 = 900;
const FLOOD_FILL_EDGE: i32 = 950;
const FLOOD_FILL_FULL: i32 = 999;

fn flood_fill_get_flooded_floor(options: &Options, state: &State, config: &Config, factory: &Factory, ox: usize, oy: usize, ow: usize, oh: usize, from_edge: bool) -> Vec<i32> {
  assert!(FLOOD_FILL_EMPTY_FRESH > 500, "must be sufficiently large to be bigger than any cell visit value");
  assert!(FLOOD_FILL_EMPTY_FRESH < FLOOD_FILL_EDGE, "range check is done on this value so other constants must be larger");
  assert!(FLOOD_FILL_EMPTY_FRESH < FLOOD_FILL_FULL, "range check is done on this value so other constants must be larger");

  // Create a mirror of the floor but just with empty or non-empty
  // Start at any cell inside the given ox oy ow oh rectangle

  let mut fake = vec!();
  factory.floor.iter().enumerate().for_each(|(i, cell)| fake.push(if cell.kind != CellKind::Empty { FLOOD_FILL_FULL } else if cell.is_edge { FLOOD_FILL_EDGE } else { FLOOD_FILL_EMPTY_FRESH }));

  // print_fake(&fake);

  if from_edge {
    for x in 1..FLOOR_CELLS_W-1 {
      fake[x] = 1;
      fake[x + FLOOR_CELLS_WH - FLOOR_CELLS_W] = 1;
    }
    for y in 1..FLOOR_CELLS_H-1 {
      fake[y * FLOOR_CELLS_W] = 1;
      fake[y * FLOOR_CELLS_W + FLOOR_CELLS_W - 1] = 1;
    }
  } else {
    for x in ox..ox+ow {
      for y in oy..oy+oh {
        fake[x + y * FLOOR_CELLS_W] = 1;
      }
    }
  }

  print_fake(&fake);

  // Note: even a contrived maze example only went up to 56. 100 should be sufficient. If not just bail.
  for lop in 0..100 {
    let mut changed = false;
    // Flood fill starting with the machine cell neighbors
    for i in 0..fake.len() {
      let n = fake[i];
      if n > FLOOD_FILL_EMPTY_FRESH {
        continue;
      }
      if n == 1 {
        // Prevent oob with "from_edge". It won't get lower than 1, anyways.
        continue;
      }
      let mut m = n;

      // Note: Can't be edge cell because they are all FLOOD_FILL_EDGE (950) and we bail above 900
      //       As such we don't need to do range safety checks.
      //       We do need to confirm that the cell is visited before (<FLOOD_FILL_EMPTY_FRESH)

      let p = fake[i - FLOOR_CELLS_W];
      if p < FLOOD_FILL_EMPTY_FRESH && m > p {
        m = p;
      }
      let p = fake[i - 1];
      if p < FLOOD_FILL_EMPTY_FRESH && m > p {
        m = p;
      }
      let p = fake[i + FLOOR_CELLS_W];
      if p < FLOOD_FILL_EMPTY_FRESH && m > p {
        m = p;
      }
      let p = fake[i + 1];
      if p < FLOOD_FILL_EMPTY_FRESH && m > p {
        m = p;
      }
      if m+1 < n {
        fake[i] = m + 1;
        changed = true;
      }
    }
    if !changed {
      log!("Breaking after {} iterations", lop+1);
      break;
    }
  }

  print_fake(&fake);

  return fake;
}

fn print_fake(fake: &Vec<i32>) {
  fn b62(n: i32) -> String {
    if n == FLOOD_FILL_EMPTY_FRESH {
      return format!("{}", ' ');
    }
    if n == FLOOD_FILL_FULL {
      return format!("{}", '#');
    }
    if n == FLOOD_FILL_EDGE {
      return format!("{}", '%');
    }
    if n < 10 {
      return format!("{}", n);
    }
    if n < 36 {
      return format!("{}", ('a' as u8 + (n - 10) as u8) as char);
    }
    if n < 62 {
      return format!("{}", ('A' as u8 + (n - 36) as u8) as char);
    }
    if n >= 62 {
      return format!("{}", '?');
    }
    return format!("{}", '#');
  }

  log!("floor print:");
  log!(
    "\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n",
    b62(fake[0]),     b62(fake[1]),   b62(fake[2]),   b62(fake[3]),   b62(fake[4]),   b62(fake[5]),   b62(fake[6]),   b62(fake[7]),   b62(fake[8]),   b62(fake[9]),  b62(fake[10]),  b62(fake[11]),  b62(fake[12]),  b62(fake[13]),  b62(fake[14]),  b62(fake[15]),  b62(fake[16]),
    b62(fake[17]),   b62(fake[18]),  b62(fake[19]),  b62(fake[20]),  b62(fake[21]),  b62(fake[22]),  b62(fake[23]),  b62(fake[24]),  b62(fake[25]),  b62(fake[26]),  b62(fake[27]),  b62(fake[28]),  b62(fake[29]),  b62(fake[30]),  b62(fake[31]),  b62(fake[32]),  b62(fake[33]),
    b62(fake[34]),   b62(fake[35]),  b62(fake[36]),  b62(fake[37]),  b62(fake[38]),  b62(fake[39]),  b62(fake[40]),  b62(fake[41]),  b62(fake[42]),  b62(fake[43]),  b62(fake[44]),  b62(fake[45]),  b62(fake[46]),  b62(fake[47]),  b62(fake[48]),  b62(fake[49]),  b62(fake[50]),
    b62(fake[51]),   b62(fake[52]),  b62(fake[53]),  b62(fake[54]),  b62(fake[55]),  b62(fake[56]),  b62(fake[57]),  b62(fake[58]),  b62(fake[59]),  b62(fake[60]),  b62(fake[61]),  b62(fake[62]),  b62(fake[63]),  b62(fake[64]),  b62(fake[65]),  b62(fake[66]),  b62(fake[67]),
    b62(fake[68]),   b62(fake[69]),  b62(fake[70]),  b62(fake[71]),  b62(fake[72]),  b62(fake[73]),  b62(fake[74]), b62(fake[75]),   b62(fake[76]),  b62(fake[77]),  b62(fake[78]),  b62(fake[79]),  b62(fake[80]),  b62(fake[81]),  b62(fake[82]),  b62(fake[83]),  b62(fake[84]),
    b62(fake[85]),   b62(fake[86]),  b62(fake[87]),  b62(fake[88]),  b62(fake[89]), b62(fake[90]),   b62(fake[91]),  b62(fake[92]),  b62(fake[93]),  b62(fake[94]),  b62(fake[95]),  b62(fake[96]),  b62(fake[97]),  b62(fake[98]),  b62(fake[99]), b62(fake[100]), b62(fake[101]),
    b62(fake[102]), b62(fake[103]), b62(fake[104]), b62(fake[105]), b62(fake[106]), b62(fake[107]), b62(fake[108]), b62(fake[109]), b62(fake[110]), b62(fake[111]), b62(fake[112]), b62(fake[113]), b62(fake[114]), b62(fake[115]), b62(fake[116]), b62(fake[117]), b62(fake[118]),
    b62(fake[119]), b62(fake[120]), b62(fake[121]), b62(fake[122]), b62(fake[123]), b62(fake[124]), b62(fake[125]), b62(fake[126]), b62(fake[127]), b62(fake[128]), b62(fake[129]), b62(fake[130]), b62(fake[131]), b62(fake[132]), b62(fake[133]), b62(fake[134]), b62(fake[135]),
    b62(fake[136]), b62(fake[137]), b62(fake[138]), b62(fake[139]), b62(fake[140]), b62(fake[141]), b62(fake[142]), b62(fake[143]), b62(fake[144]), b62(fake[145]), b62(fake[146]), b62(fake[147]), b62(fake[148]), b62(fake[149]), b62(fake[150]), b62(fake[151]), b62(fake[152]),
    b62(fake[153]), b62(fake[154]), b62(fake[155]), b62(fake[156]), b62(fake[157]), b62(fake[158]), b62(fake[159]), b62(fake[160]), b62(fake[161]), b62(fake[162]), b62(fake[163]), b62(fake[164]), b62(fake[165]), b62(fake[166]), b62(fake[167]), b62(fake[168]), b62(fake[169]),
    b62(fake[170]), b62(fake[171]), b62(fake[172]), b62(fake[173]), b62(fake[174]), b62(fake[175]), b62(fake[176]), b62(fake[177]), b62(fake[178]), b62(fake[179]), b62(fake[180]), b62(fake[181]), b62(fake[182]), b62(fake[183]), b62(fake[184]), b62(fake[185]), b62(fake[186]),
    b62(fake[187]), b62(fake[188]), b62(fake[189]), b62(fake[190]), b62(fake[191]), b62(fake[192]), b62(fake[193]), b62(fake[194]), b62(fake[195]), b62(fake[196]), b62(fake[197]), b62(fake[198]), b62(fake[199]), b62(fake[200]), b62(fake[201]), b62(fake[202]), b62(fake[203]),
    b62(fake[204]), b62(fake[205]), b62(fake[206]), b62(fake[207]), b62(fake[208]), b62(fake[209]), b62(fake[210]), b62(fake[211]), b62(fake[212]), b62(fake[213]), b62(fake[214]), b62(fake[215]), b62(fake[216]), b62(fake[217]), b62(fake[218]), b62(fake[219]), b62(fake[220]),
    b62(fake[221]), b62(fake[222]), b62(fake[223]), b62(fake[224]), b62(fake[225]), b62(fake[226]), b62(fake[227]), b62(fake[228]), b62(fake[229]), b62(fake[230]), b62(fake[231]), b62(fake[232]), b62(fake[233]), b62(fake[234]), b62(fake[235]), b62(fake[236]), b62(fake[237]),
    b62(fake[238]), b62(fake[239]), b62(fake[240]), b62(fake[241]), b62(fake[242]), b62(fake[243]), b62(fake[244]), b62(fake[245]), b62(fake[246]), b62(fake[247]), b62(fake[248]), b62(fake[249]), b62(fake[250]), b62(fake[251]), b62(fake[252]), b62(fake[253]), b62(fake[254]),
    b62(fake[255]), b62(fake[256]), b62(fake[257]), b62(fake[258]), b62(fake[259]), b62(fake[260]), b62(fake[261]), b62(fake[262]), b62(fake[263]), b62(fake[264]), b62(fake[265]), b62(fake[266]), b62(fake[267]), b62(fake[268]), b62(fake[269]), b62(fake[270]), b62(fake[271]),
    b62(fake[272]), b62(fake[273]), b62(fake[274]), b62(fake[275]), b62(fake[276]), b62(fake[277]), b62(fake[278]), b62(fake[279]), b62(fake[280]), b62(fake[281]), b62(fake[282]), b62(fake[283]), b62(fake[284]), b62(fake[285]), b62(fake[286]), b62(fake[287]), b62(fake[288])
  );
}

pub fn auto_build_tick(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  // if factory.ticks % 100 != 0 {
  //   return;
  // }

  if factory.auto_build_phase == AutoBuildPhase::None {
    return;
  }

  let since = factory.ticks - factory.auto_build_phase_at;
  let progress =
    if since < factory.auto_build_phase_pause {
      0.0
    } else {
      (since - factory.auto_build_phase_pause) as f64 / factory.auto_build_phase_duration as f64
    }
    .max(0.0).min(1.0);

  factory.auto_build_phase_progress = progress;

  if progress >= 1.0 {
    return auto_build_next_step(options, state, config, factory);
  }
}
