use super::bouncer::*;
use super::config::*;
use super::factory::*;
use super::part::*;
use super::options::*;
use super::quest_state::*;
use super::state::*;
use super::utils::*;
use super::log;

#[derive(Debug, Clone)]
pub struct Story {
  // Maps to config.nodes
  pub story_node_index: usize,
  // Map to config.nodes, Parts specific to this Story.
  pub part_nodes: Vec<usize>,
  // Map to config.nodes, quests specific to this Story.
  pub quest_nodes: Vec<usize>,
}

pub fn get_available_parts_from_map_and_story(options: &mut Options, state: &mut State, config: &mut Config, initial_map_unlocked_parts: &Vec<PartKind>, story_index: usize) -> Vec<(PartKind, bool)> {
  let mut initial_unlocked_parts = initial_map_unlocked_parts.clone();

  // Add initial story parts to the unlocked map parts
  let initial_story_parts = get_initial_story_parts(options, state, config, story_index);
  initial_story_parts.iter().for_each(|part_kind| {
    if !initial_unlocked_parts.contains(part_kind) {
      initial_unlocked_parts.push(*part_kind);
    }
  });
  if options.trace_quest_status { log!("  Merged unlocked parts: {:?}", initial_unlocked_parts); }

  // For map unlocks; make sure to unlock any part necessary to create the unlocked parts.
  // In theory maps should already unlock all these parts too but since it's just text... :shrug:
  // Also, this is only doing one level so it's not fail proof either.
  let available_parts_before: Vec<(PartKind, bool)> = initial_unlocked_parts.iter().map(|kind| ( *kind, true )).collect();
  let mut available_parts = available_parts_before.clone();
  for (part_kind, _viz) in available_parts_before.iter() {
    for p2 in &config.nodes[*part_kind].pattern_by_index {
      if !available_parts.iter().any(|(p, _v)| *p == *p2) {
        available_parts.push((*p2, true));
      }
    }
  }
  if options.trace_quest_status { log!("initial available_parts: {:?}", available_parts.iter().map(|(index, _)| config.nodes[*index].name.clone()).collect::<Vec<_>>()); }

  return initial_unlocked_parts.iter().map(|p| (*p, true)).collect();
}

fn get_initial_story_parts(options: &Options, state: &State, config: &Config, story_index: usize) -> Vec<PartKind>{
  if options.trace_quest_status { log!("Discovering unlocked parts in story {}", story_index); }
  let story_initial_unlocked_parts: Vec<PartKind> = config_get_initial_unlocks(options, state, config, story_index);
  if options.trace_quest_status { log!("    Story starts with these: {:?}", story_initial_unlocked_parts); }
  return story_initial_unlocked_parts;
}
