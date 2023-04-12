// A Quote is a build order to complete. They are like achievements, quests, or build requests.
// Each Quote should have one or more things to complete and a reward upon completion, usually
// an unlock step of sorts, as well as the next Quote or set of quotes that it unlocks.

use std::collections::VecDeque;
use std::iter::Map;
use std::slice::Iter;

use super::bouncer::*;
use super::config::*;
use super::log;
use super::part::*;
use super::options::*;
use super::quest_state::*;
use super::state::*;
use super::utils::*;

// A Quote represents a single requirement for a quest.
#[derive(Debug)]
pub struct Quote {
  pub name: String, // "{quest_name}/{part_name}"
  pub quest_name: String, // unqualified name
  pub part_name: String, // unqualified name

  pub quest_index: usize, // on config.nodes
  pub part_index: usize, // on config.nodes
  pub target_count: u32,

  pub added_at: u64,
  pub current_count: u32,
  pub completed_at: u64,

  pub debug_production_target_by_name: Vec<(u32, String)>,
  pub debug_starting_part_by_name: Vec<String>,
  pub debug_unlocks_after_by_name: Vec<String>,
}

fn quote_get_status_first_pass(config: &Config, config_node_index: usize, available_parts: &Vec<PartKind>, quest_index: usize) -> QuestStatus {
  let quest = &config.nodes[config_node_index as usize];

  // Have you already unlocked this goal part? If not then you can't even start to unlock this quest.
  // If all parts are unlocked then this quest is considered at least active (maybe even finished)
  if quest.production_target_by_index.len() > 0 && quest.production_target_by_index.iter().all(|(_count, node_index)| available_parts.contains(node_index)) {
    log!("quote_get_status_first_pass: {} is active because production_target_by_index.len>0 and all of them are available", config_node_index);
    return QuestStatus::Active;
  }

  // If this quest has no requirements and all its parts aren't already unlocked then it is active
  if quest.unlocks_after_by_index.len() == 0 {
    log!("quote_get_status_first_pass: {} is active because production_target_by_index.len==0", config_node_index);
    return QuestStatus::Active;
  }

  log!("quote_get_status_first_pass: {} is waiting", config_node_index);
  // Start awaiting. Another loop will determine if they are active.
  return QuestStatus::Waiting;
}

pub fn get_fresh_quest_states(options: &Options, state: &mut State, config: &Config, ticks: u64, available_parts: &Vec<PartKind>) -> Vec<QuestState> {
  // Map config quest nodes to fresh vector of quest states and make sure their
  // states are correctly initialized based on the current available parts.
  // If parts is empty then only initial quests will be active.

  log!("get_fresh_quest_states() {:?}: {:?}", config.nodes[config.stories[state.active_story_index].story_node_index].raw_name, config.stories[1]);

  let mut quests: Vec<QuestState> = config.stories[state.active_story_index].quest_nodes.iter().enumerate().map(|(quest_index, &quest_node_index)| {
    // log!("- quest {} node {}", quest_index, quest_node_index);
    // log!("node: {:?}", config.nodes[quest_node_index]);
    let status = quote_get_status_first_pass(config, quest_node_index, available_parts, quest_index);
    // log!("  - status: {:?}", status);
    let unlock_requirement_indexes = config.nodes[quest_node_index].unlocks_after_by_index.iter().map(|&index| config.nodes[index].quest_index).collect::<Vec<usize>>();
    // log!("  - unlock_requirement_indexes: {:?}", unlock_requirement_indexes);
    let production_part_index = config.nodes[quest_node_index].production_target_by_index[0].1;
    // log!("  - production_part_index: {:?}", production_part_index);
    let production_target = config.nodes[quest_node_index].production_target_by_index[0].0;
    // log!("  - production_target: {:?}", production_target);
    return QuestState {
      name: config.nodes[quest_node_index].name.clone(),
      quest_index,
      config_node_index: quest_node_index,
      unlocks_todo: unlock_requirement_indexes,
      production_part_index,
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
        quest_index: PARTKIND_NONE, // TODO
        part_index: PARTKIND_NONE, // TODO
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

  log!("initial quest states: {:?}", quests.iter().map(|q| q.status).collect::<Vec<_>>());

  log!("checking active quests...");
  let mut changed = true;
  while changed {
    changed = false;
    for quest_index in 0..quests.len() {
      let status = quests[quest_index].status;
      if status == QuestStatus::Active {
        log!("Quest {} is active", quest_index);
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
            log!("Quest {} is a todo of {} which has status {:?}", quest_index, i, quests[i].status);
            if quests[i].status != QuestStatus::Active && quests[i].status != QuestStatus::FadingAndBouncing {
              all = false;
              break;
            }
          }
        }
        // Finish this quest if all sub-quests that depend on it are active or finished but
        // only if there is at least one such sub quest (never finish the final quest(s))
        if has_any && all {
          log!("quest_update_status: all sub quests are finished so finishing {}", config.nodes[quests[quest_index].config_node_index].raw_name);
          quest_update_status(&mut quests[quest_index], QuestStatus::FadingAndBouncing, 0);
          changed = true;
        }
      }
    }
  }

  log!("get_fresh_quest_states() initial available quests # are: {:?}", quests.iter().filter(|quest| quest.status == QuestStatus::Active).map(|quest| quest.name.clone()).collect::<Vec<String>>());

  // Now all quest states should be correctly waiting, active, or finished.
  // We dont care/remember about the other states at load time.
  return quests;
}

pub fn quote_create(config: &Config, quest_index: usize, ticks: u64) -> Vec<Quote> {
  return config.nodes[quest_index].production_target_by_index.iter().map(|&(count, part_index)| {
    Quote {
      name: format!("{}/{}", config.nodes[quest_index].name, config.nodes[part_index].name),

      quest_name: config.nodes[quest_index].name.clone(),
      part_name: config.nodes[part_index].name.clone(),

      quest_index: quest_index,
      part_index: part_index,
      target_count: count,

      added_at: ticks,
      current_count: 0,
      completed_at: 0,

      // Copy for debugging
      debug_production_target_by_name: config.nodes[quest_index].production_target_by_name.clone(),
      debug_starting_part_by_name: config.nodes[quest_index].starting_part_by_name.clone(),
      debug_unlocks_after_by_name: config.nodes[quest_index].unlocks_after_by_name.clone(),
    }
  }).collect(); // TODO: js>rust noob here; looks like these are lazy evalled in serial and the collect is preventing this
}

