use super::belt::*;
use super::cell::*;
use super::machine::*;
use super::factory::*;
use super::options::*;
use super::part::*;
use super::state::*;

#[derive(Debug)]
pub struct Demand {
  pub part: Part, // The part that this demander is waiting for
  pub neighbor_coord: usize, // Cell coord of the only neighbor this demand has
  pub part_price: i32, // Amount of money you receive when supplying the proper part
  pub trash_price: i32, // Penalty you pay for giving the wrong part
}

pub const fn demand_none() -> Demand {
  return Demand {
    part: part_none(),
    neighbor_coord: 0,
    part_price: 0,
    trash_price: 0,
  };
}

pub fn demand_new(part: Part, neighbor_coord: usize) -> Demand {
  return Demand {
    part,
    neighbor_coord,
    part_price: 0,
    trash_price: 0,
  };
}

pub fn tick_demand(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) {
}
