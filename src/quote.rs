// A Quote is a build order to complete. They are like achievements, quests, or build requests.
// Each Quote should have one or more things to complete and a reward upon completion, usually
// an unlock step of sorts, as well as the next Quote or set of quotes that it unlocks.

use std::iter::Map;
use std::slice::Iter;
use futures::TryStreamExt;
use super::config::*;
use super::part::*;
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

pub fn quest_available(config: &Config, quest_index: usize) -> bool {
  return config.nodes[quest_index]
    .unlocks_after_by_index.iter()
    .all(|&quest_index| {
      return config.nodes[quest_index].current_state == ConfigNodeState::Finished;
    });
}

pub fn quotes_get_available(config: &Config, ticks: u64) -> Vec<Quote> {
  // Find all config nodes that are available
  log(format!("quotes_get_available()"));
  return config.quest_nodes.iter()
    .filter(|&&quest_index| {
      let b = config.nodes[quest_index].current_state == ConfigNodeState::Available || (config.nodes[quest_index].current_state == ConfigNodeState::Waiting && quest_available(config, quest_index));
      log(format!("- finished? {}: {}, state = {:?}", config.nodes[quest_index].name, b, config.nodes[quest_index].current_state));
      // Keep quests which have finished, where all parent unlocks have unlocked
      return b;
    })
    .flat_map(|&quest_index| {
      log(format!("- generating quotes for {}: wants {:?} ({:?})", config.nodes[quest_index].name, config.nodes[quest_index].production_target_by_name, config.nodes[quest_index].production_target_by_index));
      // Quests can require multiple parts before completion
      let x = quote_create(config, quest_index, ticks);
      log(format!("-> {:?}", x));
      return x;
    }).collect();
}

pub fn quote_create(config: &Config, quest_index: usize, ticks: u64) -> Vec<Quote> {
  log(format!("ok wtf? {:?}", config.nodes[quest_index].production_target_by_index));
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

