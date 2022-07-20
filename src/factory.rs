use std::collections::VecDeque;
use crate::port::Port;

use super::belt::*;
use super::cell::*;
use super::cli_serialize::*;
use super::cli_deserialize::*;
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

  pub changed: bool, // Was any part of the factory changed since last tick? Resets counters and (p)recomputes tracks.

  pub last_day_start: u64, // 1 Day is a minute worth of ticks; ONE_MS*1000*60 ticks, no matter the speed or frame rate
  pub modified_at: u64, // Track last time the factory was user manipulated. Any changes or part removal count. Score is mulled if this happens during the day.
  pub curr_day_progress: f64,
  pub curr_target_progress: f64,
  pub finished_at: u64, // Do not set for invalid scores. If at any point before the end of day the targets have been fulfilled, set this value so it sticks at it in the UI. Zero value is ignored.
  pub finished_with: u64, // Do not set for invalid scores. If at the end of day the targets have not been fulfilled, set this value to the % of progress where it failed so it sticks in the UI. Zero value is ignored.
  pub target_production: Vec<(char, u64)>, // Icon = part, u64 = desired count by end of day
  pub actual_production: Vec<(char, u64)>, // Icon = part, u64 = received count so far

  pub supplied: u64,
  pub produced: u64,
  pub accepted: u64,
  pub trashed: u64,
}

pub fn create_factory(options: &mut Options, state: &mut State, floor_str: String) -> Factory {
  let ( floor, offers ) = floor_from_str(floor_str);
  let len = offers.len();
  for i in 0..len {
    if offers[i].kind == CellKind::Supply {
      state.available_resources.push(offers[i].supply_icon);
    }
  }
  let mut factory = Factory {
    ticks: 0,
    floor,
    prio: vec!(),
    offers,
    changed: true,
    last_day_start: 0,
    modified_at: 0,
    curr_day_progress: 0.0,
    curr_target_progress: 0.0,
    finished_at: 0,
    finished_with: 0,
    target_production: vec!(('g', 100)),
    actual_production: vec!(),
    supplied: 0,
    produced: 0,
    accepted: 0,
    trashed: 0,
  };
  auto_layout(options, state, &mut factory);
  auto_ins_outs(options, state, &mut factory);
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

  if factory.finished_at == 0 {
    let day_ticks = ONE_MS * 1000 * 60; // one day a minute (arbitrary)
    let day_progress = if factory.finished_at != 0 {
      (factory.ticks - factory.finished_at) as f64 / (day_ticks as f64)
    } else {
      (factory.ticks - factory.last_day_start) as f64 / (day_ticks as f64)
    };

    let requirements = factory.target_production.len();
    let mut target_progress = factory.finished_with as f64 / 100.0;
    // If already finished, do not calculate current values. Keep them at the finished %.
    if target_progress == 0.0 {
      for i in 0..requirements {
        let icon = factory.target_production[i].0;
        for j in 0..factory.actual_production.len() {
          if factory.actual_production[j].0 == icon {
            target_progress += ((factory.actual_production[j].1 as f64) / (factory.target_production[i].1 as f64)).min(1.0) / (requirements as f64);
            break;
          }
        }
      }
    }

    factory.curr_day_progress = day_progress;
    factory.curr_target_progress = target_progress;

    if day_progress >= 1.0 || target_progress >= 1.0{
      factory.finished_at = factory.ticks;
      factory.finished_with = target_progress as u64 * 100; // Store whole percentage of progress
    }
  }
}

pub fn factory_collect_stats(options: &mut Options, state: &mut State, factory: &mut Factory) {
  let mut supplied: u64 = 0;
  let mut produced: u64 = 0;
  let mut accepted: u64 = 0;
  let mut trashed: u64 = 0;
  let mut received: Vec<(char, u64)> = vec!();

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

        let mut found = false;
        for n in 0..received.len() {
          if received[n].0 == factory.floor[coord].demand.part.icon {
            received[n].1 += factory.floor[coord].demand.received;
            found = true;
            break;
          }
        }
        if !found {
          received.push( ( factory.floor[coord].demand.part.icon, factory.floor[coord].demand.received ) );
        }

        accepted += factory.floor[coord].demand.received;
        trashed += factory.floor[coord].demand.trashed;
      }
    }
  }

  factory.actual_production = received;
  factory.supplied = supplied;
  factory.produced = produced;
  factory.accepted = accepted;
  factory.trashed = trashed;
}

pub fn factory_reset_stats(options: &mut Options, state: &mut State, factory: &mut Factory) {
  for coord in 0..factory.floor.len() {
    match factory.floor[coord].kind {
      CellKind::Empty => {} // Ignore empty cells here
      CellKind::Supply => {
        factory.floor[coord].supply.supplied = 0;
      }
      CellKind::Machine => {
        factory.floor[coord].machine.produced = 0;
        factory.floor[coord].machine.trashed = 0;
      }
      CellKind::Belt => {} // Ignore
      CellKind::Demand => {
        factory.floor[coord].demand.received = 0;
        factory.floor[coord].demand.trashed = 0;
      }
    }
  }

  factory.actual_production = vec!();
  factory.supplied = 0;
  factory.produced = 0;
  factory.accepted = 0;
  factory.trashed = 0;
}
