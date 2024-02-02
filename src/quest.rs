// A Quest is a build order to complete. They are like achievements or build requests.
// Each Quest should have one or more things to complete and a reward upon completion, usually
// an unlock step of sorts, as well as the next Quest or set of Quests that it unlocks.

use std::collections::VecDeque;
use std::iter::Map;
use std::slice::Iter;

use super::bouncer::*;
use super::config::*;
use super::factory::*;
use super::part::*;
use super::options::*;
use super::quest_state::*;
use super::state::*;
use super::utils::*;
use super::log;

// A Quest represents a single requirement for a quest.
#[derive(Debug)]
pub struct Quest {
  pub name: String, // "{quest_name}/{part_name}"
  pub quest_name: String, // unqualified name
  pub part_name: String, // unqualified name

  pub quest_index: usize, // index on config.nodes
  pub part_kind: PartKind, // index on config.nodes
  pub target_count: u32,

  pub added_at: u64,
  pub current_count: u32,
  pub completed_at: u64,

  pub debug_production_target_by_name: Vec<(u32, String)>,
  pub debug_starting_part_by_name: Vec<String>,
  pub debug_unlocks_after_by_name: Vec<String>,
}

fn quest_get_status_first_pass(options: &Options, state: &State, config: &Config, config_node_index: usize, available_parts: &Vec<PartKind>, quest_index: usize) -> QuestStatus {
  if options.trace_quest_status { log!("quest_get_status_first_pass()"); }

  let quest = &config.nodes[config_node_index];

  // If already initialized with a particular state then use that verbatim
  if config.nodes[config_node_index].quest_init_status != QuestStatus::Waiting {
    return config.nodes[config_node_index].quest_init_status;
  }

  // Have you already unlocked this goal part? If not then you can't even start to unlock this quest.
  // If all parts are unlocked then this quest is considered at least active (maybe even finished)
  if quest.production_target_by_index.len() > 0 && quest.production_target_by_index.iter().all(|(_count, node_index)| available_parts.contains(node_index)) {
    if options.trace_quest_status { log!("quest_get_status_first_pass: {} (`{}`) is active because production_target_by_index.len>0 and all of them are available", config_node_index, config.nodes[config_node_index].raw_name); }
    return QuestStatus::Active;
  }

  // If this quest has no requirements and all its parts aren't already unlocked then it is active
  if quest.unlocks_todo_by_index.len() == 0 {
    if options.trace_quest_status { log!("quest_get_status_first_pass: {} (`{}`) is active because unlocks_todo_by_index.len==0", config_node_index, config.nodes[config_node_index].raw_name); }
    return QuestStatus::Active;
  }

  if options.trace_quest_status { log!("quest_get_status_first_pass: {} (`{}`) is waiting", config_node_index, config.nodes[config_node_index].raw_name); }
  // Start awaiting. Another loop will determine if they are active.
  return QuestStatus::Waiting;
}

pub fn get_fresh_quest_states(options: &Options, state: &State, config: &Config, ticks: u64, available_parts: &Vec<PartKind>) -> Vec<QuestState> {
  log!("get_fresh_quest_states(options.trace_quest_status={})", options.trace_quest_status);

  // Map config quest nodes to fresh vector of quest states and make sure their
  // states are correctly initialized based on the current available parts.
  // If parts is empty then only initial quests will be active.

  if options.trace_quest_status { log!("active story {} of {}", state.active_story_index + 1, config.stories.len()); }
  if options.trace_quest_status { log!("get_fresh_quest_states() {:?}: {:?}", config.nodes[config.stories[state.active_story_index].story_node_index].raw_name, config.stories[state.active_story_index]); }

  let mut quests: Vec<QuestState> = config.stories[state.active_story_index].quest_nodes.iter().enumerate().map(|(quest_index, &quest_node_index)| {
    // log!("- quest {} node {}", quest_index, quest_node_index);
    // log!("node: {:?}", config.nodes[quest_node_index]);
    let status = quest_get_status_first_pass(options, state, config, quest_node_index, available_parts, quest_index);
    if options.trace_quest_status { log!("get_fresh_quest_states() quest {} ({}) status is {:?}, todo: {:?}", quest_node_index, config.nodes[quest_node_index].raw_name, status, config.nodes[quest_node_index].unlocks_todo_by_index); }
    // log!("  - status: {:?}", status);
    let unlock_requirement_indexes = config.nodes[quest_node_index].unlocks_todo_by_index.iter().map(|&index| config.nodes[index].quest_index).collect::<Vec<usize>>();
    // log!("  - unlock_requirement_indexes: {:?}", unlock_requirement_indexes);
    let production_part_kind = config.nodes[quest_node_index].production_target_by_index[0].1;
    // log!("  - production_part_kind: {:?}", production_part_kind);
    let production_target = config.nodes[quest_node_index].production_target_by_index[0].0;
    // log!("  - production_target: {:?}", production_target);
    return QuestState {
      name: config.nodes[quest_node_index].name.clone(),
      quest_index,
      config_node_index: quest_node_index,
      unlocks_todo: unlock_requirement_indexes,
      production_part_kind,
      production_progress: 0,
      production_target,
      status,
      status_at: ticks,
      bouncer: Bouncer {
        created_at: ticks, // TODO: deprecate this field?
        delay: 0, // TODO
        x: 0.0, // TODO
        y: 0.0, // TODO
        max_y: 0.0, // TODO
        quest_index: CONFIG_NODE_PART_NONE, // TODO
        part_kind: CONFIG_NODE_PART_NONE, // TODO
        dx: 0.0,
        dy: 0.0,
        /**
         * x, y, added at tick
         */
        frames: VecDeque::new(),
        last_progress: 0.0, // TODO
        bounce_from_index: 0,
        bouncing_at: 0,
      },
    }
  }).collect::<Vec<QuestState>>();

  if options.trace_quest_status { log!("initial quest states: {:?}", quests.iter().map(|q| q.status).collect::<Vec<_>>()); }

  if options.trace_quest_status { log!("checking active quests..."); }
  let mut changed = true;
  while changed {
    changed = false;
    for quest_index in 0..quests.len() {
      let status = quests[quest_index].status;
      if status == QuestStatus::Active {
        if options.trace_quest_status { log!("Quest {} (`{}`) is active", quest_index, quests[quest_index].name); }
        // If this quest is active and at least one quest that depends on this one is
        // also active, or finished, then this quest must also be finished.
        // While this would also be true for quests that are waiting, you still need to finish
        // that quest in order to unlock the part. So that's just a save game glitch we'll allow.
        let mut all = true;
        let mut has_any = false;
        for i in 0..quests.len() {
          if quests[i].unlocks_todo.contains(&quest_index) {

            all = false;
            has_any = true;
            if options.trace_quest_status { log!("Quest {} (`{}`) is a todo of {} which has status {:?}", quest_index, quests[quest_index].name, i, quests[i].status); }
            if quests[i].status != QuestStatus::Active && quests[i].status != QuestStatus::FadingAndBouncing {
              all = false;
              break;
            }
          }
        }
        // Finish this quest if all sub-quests that depend on it are active or finished but
        // only if there is at least one such sub quest (never finish the final quest(s))
        if has_any && all {
          if options.trace_quest_status { log!("quest_update_status: all sub quests are finished so finishing {}", config.nodes[quests[quest_index].config_node_index].raw_name); }
          quest_update_status_sans_factory(&mut quests[ quest_index], QuestStatus::FadingAndBouncing, 0);
          changed = true;
        }
      }
    }
  }

  if options.trace_quest_status { log!("get_fresh_quest_states() initial available quests # are: {:?}", quests.iter().filter(|quest| quest.status == QuestStatus::Active).map(|quest| quest.name.clone()).collect::<Vec<String>>()); }

  // Now all quest states should be correctly waiting, active, or finished.
  // We dont care/remember about the other states at load time.
  return quests;
}

pub fn quest_visible_index_to_quest_index(options: &Options, state: &State, config: &Config, factory: &mut Factory, quest_visible_index: usize) -> Option<usize> {
  let mut visible_index = 0;
  for quest_index in 0..factory.quests.len() {
    if factory.quests[quest_index].status != QuestStatus::Active && factory.quests[quest_index].status != QuestStatus::FadingAndBouncing {
      continue;
    }
    if visible_index == quest_visible_index {
      return Some(quest_index);
    }
    visible_index += 1;
  }
  return None;
}

pub fn quest_get_active_indexes(options: &Options, state: &State, config: &Config, factory: &Factory) -> Vec<usize> {
  let mut visible = vec!();
  for quest_index in 0..factory.quests.len() {
    if factory.quests[quest_index].status != QuestStatus::Active && factory.quests[quest_index].status != QuestStatus::FadingAndBouncing {
      continue;
    }
    visible.push(quest_index);
  }
  return visible;
}

pub fn quest_reset_progress(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  log!("quest_reset_progress()");
  let available_parts = config_get_initial_unlocks(options, state, config);
  let all_available_in_this_story = available_parts.iter().map(|icon| ( part_icon_to_kind(config,*icon), true ) ).filter(|(part, _visible)| {
    // Search for this part in the default story (system nodes) and the current active story.
    // If it is part of the node list for either story then include it, otherwise exclude it.
    for (story_index, story) in config.stories.iter().enumerate() {
      if story_index == 0 || story_index == state.active_story_index {
        if story.part_nodes.contains(part) {
          return true;
        }
      }
    }
    return false;
  }).collect::<Vec<_>>();
  factory.available_atoms = all_available_in_this_story.iter().filter(|(part, _)| is_atom(config, *part)).map(|(p,b)|(*p,*b)).collect::<Vec<_>>();
  factory.available_woops = all_available_in_this_story.iter().filter(|(part, _)| is_woop(config, *part)).map(|(p,b)|(*p,*b)).collect::<Vec<_>>();
  factory.trucks = vec!();
  factory.quests = get_fresh_quest_states(options, state, config, 0, &all_available_in_this_story.iter().map(|(kind, _visible)| *kind).collect());
  factory.quest_updated = true;
  factory.changed = true;
}
