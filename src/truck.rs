use std::collections::VecDeque;

use super::belt::*;
use super::cell::*;
use super::cli_serialize::*;
use super::config::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::init::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::paste::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::utils::*;
use super::log;

const GRAV: f64 = 0.008;      // How fast it drops
const BOUNCINESS: f64 = 0.80; // Velocity retained after a bounce
const LIMIT: f64 = 0.1;       // Minimum velocity required to keep animation running

#[derive(Clone, Debug)]
pub struct Truck {
  pub created_at: u64,
  pub delay: u64,
  pub part_kind: PartKind,
  pub target_menu_part_position: usize, // Reserved spot in the right menu where the part will end
  pub for_woop: bool, // Moving to the right menu (woops) or to the bottom (atoms)
}

pub fn truck_create(created_at: u64, delay: u64, part_kind: PartKind, target_menu_part_position: usize, for_woop: bool) -> Truck {
  return Truck {
    created_at,
    delay,
    part_kind,
    target_menu_part_position,
    for_woop
  };
}
