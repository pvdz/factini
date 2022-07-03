use std::collections::VecDeque;

use super::belt::*;
use super::cell::*;
use super::direction::*;
use super::floor::*;
use super::factory::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::state::*;
use super::utils::*;

// Clone but not Copy... I don't want to accidentally clone cells when I want to move them
#[derive(Debug, Clone)]
pub struct Supply {
  // pub part: Part, // ex: Current part that's moving out. dont use this, just get it from .gives
  pub part_at: u64, // Last time part was generated
  pub neighbor_coord: usize, // Still need to verify that the neighbor is a belt
  pub outgoing_dir: Direction,
  pub neighbor_incoming_dir: Direction,
  pub last_part_out_at: u64, // Last time a part left this supply
  pub gives: Part, // The example Part that this supply will generate
  pub part_price: i32, // Cost when dispensing one part

  // Speed at which a generated part leaves the supply
  pub speed: u64,
  // Delay between last dispensed part and generation of next part
  pub cooldown: u64, // Generate new part every this many ticks
  pub supplied: u64,
}

pub const fn supply_none() -> Supply {
  return Supply {
    // part: part_none(),
    part_at: 0,
    neighbor_coord: 0,
    outgoing_dir: Direction::Up,
    neighbor_incoming_dir: Direction::Up,
    last_part_out_at: 0,
    gives: part_none(),
    part_price: 0,
    speed: 0,
    cooldown: 0,
    supplied: 0,
  };
}

pub fn supply_new(gives: Part, neighbor_coord: usize, outgoing_dir: Direction, neighbor_incoming_dir: Direction, speed: u64, cooldown: u64, price: i32) -> Supply {
  return Supply {
    // part: part_none(),
    part_at: 0,
    neighbor_coord,
    outgoing_dir,
    neighbor_incoming_dir,
    last_part_out_at: 0,
    gives,
    part_price: price,
    speed,
    cooldown,
    supplied: 0,
  };
}

pub fn tick_supply(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) {
  factory.floor[coord].ticks += 1;

  if factory.floor[coord].supply.gives.kind == PartKind::None {
    return;
  }

  if factory.floor[coord].supply.part_at == 0 && factory.ticks - factory.floor[coord].supply.last_part_out_at >= factory.floor[coord].supply.cooldown {
    // Cooled down, generate a new piece
    if options.print_moves || options.print_moves_supply { log(format!("({}) Created new {:?} at supply @{}", factory.ticks, factory.floor[coord].supply.gives.kind, coord)); }
    // factory.floor[coord].supply.part = factory.floor[coord].supply.gives.clone();
    factory.floor[coord].supply.part_at = factory.ticks;
  }
}

pub fn supply_clear_part(factory: &mut Factory, supply_coord: usize) {
  factory.floor[supply_coord].supply.supplied += 1;
  factory.floor[supply_coord].supply.part_at = 0;
  factory.floor[supply_coord].supply.last_part_out_at = factory.ticks;
}
