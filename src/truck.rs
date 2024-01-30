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
use super::zone::*;
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
  pub finished: bool, // Can be removed?
}

pub fn truck_create(created_at: u64, delay: u64, part_kind: PartKind, target_menu_part_position: usize, for_woop: bool) -> Truck {
  return Truck {
    created_at,
    delay,
    part_kind,
    target_menu_part_position,
    for_woop,
    finished: false,
  };
}

pub const WOOP_TRUCK_WP1: (f64, f64, f64, f64) = (
  UI_MENU_MACHINE_BUTTON_3X3_X + UI_MENU_MACHINE_BUTTON_3X3_WIDTH - (50.0), // Right side of the button but enough to hide under the button
  UI_MENU_MACHINE_BUTTON_3X3_Y + (UI_MENU_MACHINE_BUTTON_3X3_HEIGHT / 2.0), // Bottom of button
  0.25, // Facing right
  50.0,
);
pub const WOOP_TRUCK_WP2: (f64, f64, f64, f64) = (
  UI_MENU_MACHINE_BUTTON_3X3_X + UI_MENU_MACHINE_BUTTON_3X3_WIDTH + 10.0,
  UI_MENU_MACHINE_BUTTON_3X3_Y,
  -0.05, // Facing up and a little left
  50.0,
);
pub const WOOP_TRUCK_WP3_JUMP: (f64, f64, f64, f64) = (
  UI_MENU_MACHINE_BUTTON_3X3_X + UI_MENU_MACHINE_BUTTON_3X3_WIDTH + 10.0 - 15.0,
  UI_MENU_MACHINE_BUTTON_3X3_Y - 75.0,
  0.00, // Facing up
  80.0,
);
pub const WOOP_TRUCK_WP3_NOJUMP: (f64, f64, f64, f64) = (
  UI_MENU_MACHINE_BUTTON_3X3_X + UI_MENU_MACHINE_BUTTON_3X3_WIDTH + 10.0,
  UI_MENU_MACHINE_BUTTON_3X3_Y - 75.0,
  0.00, // Facing up
  50.0,
);
pub const WOOP_TRUCK_WP4: (f64, f64, f64, f64) = (
  UI_MENU_MACHINE_BUTTON_3X3_X + UI_MENU_MACHINE_BUTTON_3X3_WIDTH + 10.0,
  UI_MENU_MACHINE_BUTTON_3X3_Y - 90.0,
  0.0, // Facing up and a little left
  50.0,
);
pub const WOOP_TRUCK_WP5: (f64, f64, f64, f64) = (
  UI_MENU_MACHINE_BUTTON_3X3_X + UI_MENU_MACHINE_BUTTON_3X3_WIDTH + 10.0,
  UI_MENU_MACHINE_BUTTON_3X3_Y - 150.0,
  0.0, // Facing up and a little left
  50.0,
);
pub const WOOP_TRUCK_WP6: (f64, f64, f64, f64) = (
  UI_MENU_MACHINE_BUTTON_3X3_X + UI_MENU_MACHINE_BUTTON_3X3_WIDTH + 10.0,
  UI_MENU_MACHINE_BUTTON_3X3_Y - 350.0,
  0.0, // Facing up and a little left
  50.0,
);
pub const ATOM_TRUCK_WP1: (f64, f64, f64, f64) = (
  UI_MENU_MACHINE_BUTTON_3X3_X + UI_MENU_MACHINE_BUTTON_3X3_WIDTH - (50.0 + 5.0), // Right side of the button but enough to hide under the button
  UI_MENU_MACHINE_BUTTON_3X3_Y + (UI_MENU_MACHINE_BUTTON_3X3_HEIGHT / 2.0) - (50.0 / 2.0), // Middle of button
  0.25, // Facing right
  50.0,
);
pub const ATOM_TRUCK_WP2A: (f64, f64, f64, f64) = (
  UI_MENU_MACHINE_BUTTON_3X3_X + UI_MENU_MACHINE_BUTTON_3X3_WIDTH + 15.0, // Needs some forward movement for the sin
  UI_MENU_MACHINE_BUTTON_3X3_Y + UI_MENU_MACHINE_BUTTON_3X3_HEIGHT + 15.0,
  0.80, // Facing left and a little up
  50.0
);
pub const ATOM_TRUCK_WP2B: (f64, f64, f64, f64) = (
  UI_MENU_MACHINE_BUTTON_3X3_X + UI_MENU_MACHINE_BUTTON_3X3_WIDTH - (50.0 + 5.0), // Should be starting position of T2 due to the sin
  UI_MENU_MACHINE_BUTTON_3X3_Y + UI_MENU_MACHINE_BUTTON_3X3_HEIGHT + 15.0,
  0.80, // Facing left and a little up
  50.0
);
pub const ATOM_TRUCK_WP3: (f64, f64, f64, f64) = (
  UI_MENU_MACHINE_BUTTON_3X3_X + UI_MENU_MACHINE_BUTTON_3X3_WIDTH - 100.0,
  UI_MENU_MACHINE_BUTTON_3X3_Y + UI_MENU_MACHINE_BUTTON_3X3_HEIGHT + 12.0,
  0.74, // Facing right
  50.0
);
