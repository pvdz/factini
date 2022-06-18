use std::collections::VecDeque;
use crate::port::Port;

use super::belt::*;
use super::cell::*;
use super::demand::*;
use super::floor::*;
use super::options::*;
use super::machine::*;
use super::offer::*;
use super::part::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::state::*;
use super::supply::*;
use super::utils::*;

pub struct Factory {
  pub ticks: u64,
  pub floor: [Cell; FLOOR_CELLS_WH],
  pub prio: Vec<usize>,
  pub offers: Vec<Offer>, // Cells the player can drag'n'drop
  pub first_out_at: u64, // Tick at which first part reached a Demand

  pub changed: bool, // Was any part of the factory changed since last tick? Resets counters and (p)recomputes tracks.

  pub supplied: u64,
  pub produced: u64,
  pub accepted: u64,
  pub trashed: u64,
}

pub fn create_factory(options: &mut Options, state: &mut State, floor_str: String) -> Factory {
  let ( floor, offers ) = floor_from_str(floor_str);
  let mut factory = Factory {
    ticks: 0,
    floor,
    prio: vec!(),
    offers,
    first_out_at: 0,
    changed: true,
    supplied: 0,
    produced: 0,
    accepted: 0,
    trashed: 0,
  };
  auto_layout(options, state, &mut factory);
  let prio = create_prio_list(options, &mut factory.floor);
  factory.prio = prio;
  factory.changed = false;
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

pub fn factory_collect_stats(options: &mut Options, state: &mut State, factory: &mut Factory) {
  let mut supplied: u64 = 0;
  let mut produced: u64 = 0;
  let mut accepted: u64 = 0;
  let mut trashed: u64 = 0;

  for coord in 0..factory.floor.len() {
    match factory.floor[coord].kind {
      CellKind::Empty => {} // Ignore empty cells here
      CellKind::Supply => {
        supplied += factory.floor[coord].supply.supplied;
      }
      CellKind::Machine => {
        produced += factory.floor[coord].machine.produced;
        trashed += factory.floor[coord].machine.trashed;
      }
      CellKind::Belt => {} // Ignore
      CellKind::Demand => {
        accepted += factory.floor[coord].demand.received;
        trashed += factory.floor[coord].demand.trashed;
      }
    }
  }

  factory.supplied = supplied;
  factory.produced = produced;
  factory.accepted = accepted;
  factory.trashed = trashed;

  if factory.first_out_at == 0 {
    if accepted > 0 {
      factory.first_out_at = factory.ticks;
    }
  }
}
