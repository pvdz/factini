use std::collections::VecDeque;
use crate::port::Port;

use super::belt::*;
use super::cell::*;
use super::demand::*;
use super::floor::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::port::*;
use super::prio::*;
use super::state::*;
use super::supply::*;
use super::utils::*;

pub struct Factory {
  pub ticks: u64,
  pub floor: [Cell; FLOOR_CELLS_WH],
  pub prio: Vec<usize>,
}

pub fn create_factory(options: &mut Options, _state: &mut State, floor_str: String) -> Factory {
  let mut floor = floor_from_str(floor_str);
  let prio = create_prio_list(options, &mut floor);
  let mut factory = Factory {
    ticks: 0,
    floor,
    prio,
  };
  auto_layout(&mut factory);
  return factory;
}

pub fn tick_factory(options: &mut Options, state: &mut State, factory: &mut Factory) {
  factory.ticks += 1;

  for n in 0..factory.prio.len() {
    let coord = factory.prio[n];
    factory.floor[coord].ticks += 1;

    match factory.floor[coord].kind {
      CellKind::Empty => panic!("should not have empty cells in the prio list:: prio index: {}, coord: {}, cell: {:?}", n, coord, factory.floor[coord]),
      CellKind::Belt => tick_belt(options, state, factory, coord),
      CellKind::Machine => tick_machine(options, state, factory, coord),
      CellKind::Supply => tick_supply(options, state, factory, coord),
      CellKind::Demand => tick_demand(options, state, factory, coord),
    }
  }
}
