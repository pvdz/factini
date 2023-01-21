// A Quote is a build order to complete. They are like achievements, quests, or build requests.
// Each Quote should have one or more things to complete and a reward upon completion, usually
// an unlock step of sorts, as well as the next Quote or set of quotes that it unlocks.

use std::iter::Map;
use std::slice::Iter;
use futures::TryStreamExt;
use super::config::*;
use super::part::*;
use super::utils::*;
use super::log;

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

pub fn quotes_get_available(config: &Config, ticks: u64, available_parts: &Vec<PartKind>) -> Vec<Quote> {
  // Find all config nodes that are available
  // A quote is available when
  // - the asked part is unlocked, and
  // - the asked part of at least one of the quests it unlocks is still locked
  log!("quotes_get_available()");

  return config.quest_nodes.iter()
    .filter(|&&quest_index| {
      let quest = &config.nodes[quest_index];
      log!("is quest {} available? check if {:?} is unlocked (-> {}) and {:?} has no child-quests ({}) or not completely unlocked (-> {:?}): available_parts: {:?}",
        quest.raw_name,
        // quest.production_target_by_index,
        // quest.production_target_by_index.iter().map(|(_c, index)| *index).collect::<Vec<usize>>(),
        quest.production_target_by_index.iter().map(|(_c, index)| config.nodes[*index].raw_name.clone()).collect::<Vec<String>>(),
        quest.production_target_by_index.iter().all(|(_count, node_index)| available_parts.contains(node_index)),
        // quest.required_by_quest_indexes,
        quest.required_by_quest_indexes.iter().map(|index| config.nodes[*index].raw_name.clone()).collect::<Vec<String>>(),
        quest.required_by_quest_indexes.len() == 0,
        quest.required_by_quest_indexes.iter().map(|&index| {
          config.nodes[index].production_target_by_index.iter().all(|(_count, node_index)| available_parts.contains(node_index))
        }).collect::<Vec<bool>>(),
        // available_parts
        available_parts.iter().map(|index| config.nodes[*index].raw_name.clone()).collect::<Vec<String>>()
      );

      // Have you already unlocked this goal part? If not then you can't even unlock this quest.
      if !quest.production_target_by_index.iter().all(|(_count, node_index)| available_parts.contains(node_index)) {
        return false;
      }

      if quest.required_by_quest_indexes.len() == 0 {
        // End goals should always be available as long as their goal part is unlocked?
        // TODO: This should perhaps be special cased...
        return true;
      }

      // Go through each quest that depends on this quest
      // For each goal part of those quests, confirm whether they are available
      // If any of those parts is not available then we enable this quest. Otherwise it must be finished already and we sleep.
      if quest.required_by_quest_indexes.iter().all(|&index| {
        config.nodes[index].production_target_by_index.iter().all(|(_count, node_index)| available_parts.contains(node_index))
      }) {
        return false;
      }

      // The goals of this quest are all available parts and at least one part the quests that
      // depend on this quest are not yet unlocked, so we open this quest for completion.
      return true;
    })
    .flat_map(|&quest_index| {
      log!("- generating quotes for {}: wants {:?} ({:?})", config.nodes[quest_index].name, config.nodes[quest_index].production_target_by_name, config.nodes[quest_index].production_target_by_index);
      // Quests can require multiple parts before completion
      let x = quote_create(config, quest_index, ticks);
      log!("~~> {:?}", x);
      return x;
    }).collect();
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

