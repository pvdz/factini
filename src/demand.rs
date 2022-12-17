use super::belt::*;
use super::cell::*;
use super::config::*;
use super::direction::*;
use super::machine::*;
use super::factory::*;
use super::options::*;
use super::part::*;
use super::state::*;
use super::utils::*;
use super::log;

// Clone but not Copy... I don't want to accidentally clone cells when I want to move them
#[derive(Debug, Clone)]
pub struct Demand {
  pub neighbor_coord: usize, // Cell coord of the only neighbor this demand has
  pub incoming_dir: Direction,
  pub neighbor_outgoing_dir: Direction,
  pub part_price: i32, // Amount of money you receive when supplying the proper part
  pub last_part_at: u64, // When did it take the last part?
  pub last_part_kind: PartKind,
  pub speed: u64, // How many ticks does it take to process a part. Governs animation.
  pub cooldown: u64, // How long is the delay between receiving parts. Will not accept new parts until ticks>last_part_at+speed+cooldown
  pub trash_price: i32, // Penalty you pay for giving the wrong part
  pub received: Vec<(PartKind, u32)>,
}

pub const fn demand_none() -> Demand {
  return Demand {
    neighbor_coord: 0,
    incoming_dir: Direction::Up,
    neighbor_outgoing_dir: Direction::Up,
    part_price: 0,
    last_part_at: 0,
    last_part_kind: PARTKIND_NONE,
    speed: 100,
    cooldown: 100,
    trash_price: 0,
    received: vec!(),
  };
}

pub fn demand_new(neighbor_coord: usize, incoming_dir: Direction, neighbor_outgoing_dir: Direction, speed: u64, cooldown: u64) -> Demand {
  return Demand {
    neighbor_coord,
    incoming_dir,
    neighbor_outgoing_dir,
    part_price: 0,
    last_part_at: 0,
    last_part_kind: PARTKIND_NONE,
    speed,
    cooldown,
    trash_price: 0,
    received: vec!(),
  };
}

pub fn tick_demand(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) {

}

pub fn demand_ready(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, demand_coord: usize) -> bool {
  return factory.ticks > factory.floor[demand_coord].demand.last_part_at + factory.floor[demand_coord].demand.speed + factory.floor[demand_coord].demand.cooldown;
}

pub fn demand_receive_part(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, demand_coord: usize, belt_coord: usize) {
  let kind = factory.floor[belt_coord].belt.part.kind;
  factory.floor[demand_coord].demand.last_part_at = factory.ticks;
  factory.floor[demand_coord].demand.last_part_kind = kind;

  for i in 0..factory.floor[demand_coord].demand.received.len() {
    if factory.floor[demand_coord].demand.received[i].0 == kind {
      factory.floor[demand_coord].demand.received[i].1 += 1;
      break;
    }
  }

  factory.floor[demand_coord].demand.received.push( ( kind, 1 ) );
}
