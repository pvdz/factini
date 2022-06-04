use std::collections::VecDeque;

use super::belt::*;
use super::cell::*;
use super::floor::*;
use super::factory::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::state::*;

#[derive(Debug)]
pub struct Supply {
  pub part: Part, // Current part that's moving out
  pub part_at: u64, // Last time part was generated
  pub neighbor_coord: usize,
  pub last_part_out_at: u64, // Last time a part left this supply
  pub interval: u64, // Generate new part every this many ticks
  pub gives: Part, // The example Part that this supply will generate
  pub part_price: i32, // Cost when dispensing one part

  // Speed at which a generated part leaves the supply
  pub speed: u64,
  // Delay between last dispensed part and generation of next part
  pub delay: u64,
}

pub const fn supply_none() -> Supply {
  return Supply {
    part: part_none(),
    part_at: 0,
    neighbor_coord: 0,
    last_part_out_at: 0,
    interval: 0,
    gives: part_none(),
    part_price: 0,
    speed: 0,
    delay: 0
  };
}

pub fn supply_new(gives: Part, neighbor_coord: usize, speed: u64, interval: u64, price: i32) -> Supply {
  return Supply {
    part: part_none(),
    part_at: 0,
    neighbor_coord,
    last_part_out_at: 0,
    interval,
    gives,
    part_price: price,
    speed,
    delay: interval,
  };
}

pub fn tick_supply(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) {
}
