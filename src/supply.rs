use std::collections::VecDeque;

use super::belt::*;
use super::cell::*;
use super::config::*;
use super::direction::*;
use super::floor::*;
use super::factory::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::state::*;
use super::utils::*;
use super::log;

// Clone but not Copy... I don't want to accidentally clone cells when I want to move them
#[derive(Debug, Clone)]
pub struct Supply {
  // pub part: Part, // ex: Current part that's moving out. dont use this, just get it from .gives
  pub part_started_at: u64, // Last time part begun generating cycle
  pub part_created_at: u64, // Last time part completed generating cycle
  pub part_tbd: bool, // Is the connected belt ready to receive this? The part does not proceed past 50% of speed until this is set to true.
  pub part_progress: u64, // Absolute ticks because it may pause arbitrarily while waiting for neighbor belt to have space available. Use speed to control speed of generation.
  pub neighbor_coord: usize, // Still need to verify that the neighbor is a belt
  pub outgoing_dir: Direction,
  pub neighbor_incoming_dir: Direction,
  pub last_part_out_at: u64, // Last time a part left this supply
  pub gives: Part, // The example Part that this supply will generate
  pub gives_raw: String, // As parsed. Either an icon, an &ord, or a full qualified name
  pub part_price: i32, // Cost when dispensing one part

  // Speed at which a generated part leaves the supply
  pub speed: u64,
  // Delay between last dispensed part and generation of next part
  pub cooldown: u64, // Generate new part every this many ticks
  pub supplied: u64,
}

pub fn supply_none(config: &Config) -> Supply {
  return Supply {
    part_started_at: 0,
    part_created_at: 0,
    part_tbd: true,
    part_progress: 0,
    neighbor_coord: 0,
    outgoing_dir: Direction::Up,
    neighbor_incoming_dir: Direction::Up,
    last_part_out_at: 0,
    gives: part_none(config),
    gives_raw: format!("{}", part_none(config).icon),
    part_price: 0,
    speed: 0,
    cooldown: 0,
    supplied: 0,
  };
}

pub fn supply_new(gives: Part, neighbor_coord: usize, outgoing_dir: Direction, neighbor_incoming_dir: Direction, speed: u64, cooldown: u64, price: i32) -> Supply {
  return Supply {
    part_started_at: 0,
    part_created_at: 0,
    part_tbd: true,
    part_progress: 0,
    neighbor_coord,
    outgoing_dir,
    neighbor_incoming_dir,
    last_part_out_at: 0,
    gives_raw: format!("{}", gives.icon),
    gives,
    part_price: price,
    speed,
    cooldown,
    supplied: 0,
  };
}

pub fn tick_supply(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) {
  factory.floor[coord].ticks += 1;

  if factory.floor[coord].supply.gives.kind == CONFIG_NODE_PART_NONE {
    return;
  }

  let ticks = factory.ticks;
  let mut supply = &mut factory.floor[coord].supply;

  if supply.last_part_out_at > 0 && ticks - supply.last_part_out_at < supply.cooldown {
    // Still in cooldown period after creating last part
    return;
  }

  if supply.part_created_at == 0 {
    if options.trace_all_moves || options.trace_moves_supply { log!("({}) Created new {:?} at supply @{}", ticks, supply.gives.kind, coord); }
    supply.part_created_at = ticks;
  }

  let speed = supply.speed.max(1);

  // Every tick where eligible, move progress forward by speed ticks. When the connected belt is
  // not available, tbd is false and we do not move progress forward until the belt has space.
  if (supply.part_tbd && supply.part_progress < (speed / 2).max(1)) || (!supply.part_tbd || supply.part_progress < speed) {
    supply.part_progress += 1;
  }
}

pub fn supply_clear_part(factory: &mut Factory, supply_coord: usize) {
  let supply = &mut factory.floor[supply_coord].supply;
  supply.supplied += 1;
  supply.part_started_at = 0;
  supply.part_created_at = 0;
  supply.part_progress = 0;
  supply.last_part_out_at = factory.ticks;
  supply.part_tbd = true;
}

pub fn supply_get_default_part(options: &Options, state: &State, config: &Config, factory: &Factory, mouse_state: &MouseState) -> PartKind {
  if options.default_supply_trash {
    log!("supply_get_default_part(options.default_supply_trash=true)");
    // This is a special case part that may be used while debugging
    return CONFIG_NODE_PART_TRASH;
  }

  // Prefer the selected atom, if any
  if mouse_state.atom_selected {
    log!("supply_get_default_part(mouse_state.atom_selected=true, {})", mouse_state.atom_selected_index);
    return factory.available_atoms[mouse_state.atom_selected_index].0 as PartKind;
  }

  return supply_get_random_atom(options, state, config, factory);
}

pub fn supply_get_random_atom(options: &Options, state: &State, config: &Config, factory: &Factory) -> PartKind {
  // Collect all available atoms that are visible (not truck in flight)
  let mut available = vec!();
  factory.available_atoms.iter().for_each(|(kind, visible)| {
    if *visible {
      available.push(*kind);
    }
  });

  let rng = xorshift(factory.ticks as usize);
  let n = rng % available.len();

  log!("supply_get_default_part(rng={})", available[n]);

  return available[n];
}
