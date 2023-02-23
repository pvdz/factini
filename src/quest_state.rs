use super::belt::*;
use super::bouncer::*;
use super::cell::*;
use super::cli_serialize::*;
use super::cli_deserialize::*;
use super::config::*;
use super::demand::*;
use super::factory::*;
use super::floor::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::quote::*;
use super::state::*;
use super::supply::*;
use super::truck::*;
use super::utils::*;
use super::log;

#[derive(Clone, Debug)]
pub struct QuestState {
  pub name: String,
  pub quest_index: usize,
  pub config_node_index: usize,
  pub unlocks_todo: Vec<usize>, // Quests left to unlock before this unlocks. Reference onto config.quest_nodes_by_index / factory.quests
  pub production_part_index: usize, // What part do you need to produce for this quest?
  pub production_progress: u32, // How many of the desired item did you produce so far?
  pub production_target: u32, // How many do you need to create to achieve this quest?
  pub status: QuestStatus,
  pub status_at: u64,
  pub bouncer: Bouncer,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QuestStatus {
  Waiting,
  Active,
  FadingAndBouncing,
  Bouncing,
  Finished
}

pub fn quest_update_status(quest: &mut QuestState, status: QuestStatus, ticks: u64) {
  log!("[{}] Quest {} is now {:?}", ticks, quest.quest_index, status);
  quest.status = status;
  quest.status_at = ticks;
}
