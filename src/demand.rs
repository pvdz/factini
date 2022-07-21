use super::belt::*;
use super::cell::*;
use super::direction::*;
use super::machine::*;
use super::factory::*;
use super::options::*;
use super::part::*;
use super::state::*;
use super::utils::*;

// Clone but not Copy... I don't want to accidentally clone cells when I want to move them
#[derive(Debug, Clone)]
pub struct Demand {
  pub neighbor_coord: usize, // Cell coord of the only neighbor this demand has
  pub incoming_dir: Direction,
  pub neighbor_outgoing_dir: Direction,
  pub part_price: i32, // Amount of money you receive when supplying the proper part
  pub trash_price: i32, // Penalty you pay for giving the wrong part
  pub received: Vec<(char, u64)>,
}

pub const fn demand_none() -> Demand {
  return Demand {
    neighbor_coord: 0,
    incoming_dir: Direction::Up,
    neighbor_outgoing_dir: Direction::Up,
    part_price: 0,
    trash_price: 0,
    received: vec!(),
  };
}

pub fn demand_new(neighbor_coord: usize, incoming_dir: Direction, neighbor_outgoing_dir: Direction) -> Demand {
  return Demand {
    neighbor_coord,
    incoming_dir,
    neighbor_outgoing_dir,
    part_price: 0,
    trash_price: 0,
    received: vec!(),
  };
}

pub fn tick_demand(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) {

}

pub fn demand_receive_part(options: &mut Options, state: &mut State, factory: &mut Factory, demand_coord: usize, belt_coord: usize) {
  for i in 0..factory.floor[demand_coord].demand.received.len() {
    if factory.floor[demand_coord].demand.received[i].0 == factory.floor[belt_coord].belt.part.icon {
      factory.floor[demand_coord].demand.received[i].1 += 1;
      return;
    }
  }
  factory.floor[demand_coord].demand.received.push( ( factory.floor[belt_coord].belt.part.icon, 1 ) );
  belt_receive_part(factory, belt_coord, Direction::Up, part_none());
}
