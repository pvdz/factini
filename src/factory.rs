use std::collections::VecDeque;
use crate::port::Port;

use super::belt::*;
use super::cell::*;
use super::cli_serialize::*;
use super::cli_deserialize::*;
use super::config::*;
use super::demand::*;
use super::floor::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::quote::*;
use super::state::*;
use super::supply::*;
use super::utils::*;

pub struct Factory {
  pub ticks: u64,
  pub floor: [Cell; FLOOR_CELLS_WH],
  pub prio: Vec<usize>,
  pub quotes: Vec<Quote>, // Current available achievements to unlock
  /**
   * Current available parts to use as supply or craft in machine.
   * These are painted in the right hand menu. The bool tells us whether to actually paint it.
   * ( icon, available )
   */
  pub available_parts_rhs_menu: Vec< (PartKind, bool ) >, // Which part and whether the player can use it yet

  pub changed: bool, // Was any part of the factory changed since last tick? Resets counters and (p)recomputes tracks.

  pub last_day_start: u64, // 1 Day is a minute worth of ticks; ONE_MS*1000*60 ticks, no matter the speed or frame rate
  pub modified_at: u64, // Track last time the factory was user manipulated. Any changes or part removal count. Score is mulled if this happens during the day.
  pub curr_day_progress: f64,
  pub finished_at: u64, // Do not set for invalid scores. If at any point before the end of day the targets have been fulfilled, set this value so it sticks at it in the UI. Zero value is ignored.
  pub finished_with: u64, // Do not set for invalid scores. If at the end of day the targets have not been fulfilled, set this value to the % of progress where it failed so it sticks in the UI. Zero value is ignored.

  pub supplied: u64,
  pub produced: u64,
  pub accepted: u64,
  pub trashed: u64,
}

pub fn create_factory(options: &mut Options, state: &mut State, config: &Config, floor_str: String, parts: Vec<PartKind>) -> Factory {
  let floor = floor_from_str(options, state, config, floor_str);
  let mut factory = Factory {
    ticks: 0,
    floor,
    prio: vec!(),
    quotes: quotes_get_available(config, 0),
    available_parts_rhs_menu: parts.iter().map(|&p| ( p, true )).collect::<Vec<(PartKind, bool)>>(),
    changed: true,
    last_day_start: 0,
    modified_at: 0,
    curr_day_progress: 0.0,
    finished_at: 0,
    finished_with: 0,
    supplied: 0,
    produced: 0,
    accepted: 0,
    trashed: 0,
  };
  log(format!("available quotes: {:?}", factory.quotes));
  auto_layout(options, state, config, &mut factory);
  auto_ins_outs(options, state, &mut factory);
  let prio = create_prio_list(options, config, &mut factory.floor);
  factory.prio = prio;
  factory.changed = false;
  return factory;
}

pub fn tick_factory(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory) {
  factory.ticks += 1;

  for n in 0..factory.prio.len() {
    let coord = factory.prio[n];
    factory.floor[coord].ticks += 1;

    match factory.floor[coord].kind {
      CellKind::Empty => panic!("should not have empty cells in the prio list:: prio index: {}, coord: {}, cell: {:?}", n, coord, factory.floor[coord]),
      CellKind::Belt => tick_belt(options, state, config, factory, coord),
      CellKind::Machine => tick_machine(options, state, config, factory, coord),
      CellKind::Supply => tick_supply(options, state, factory, coord),
      CellKind::Demand => tick_demand(options, state, factory, coord),
    }
  }

  if factory.finished_at == 0 {
    let day_ticks = ONE_MS * 1000 * 60; // one day a minute (arbitrary)
    let day_progress = (factory.ticks - factory.last_day_start) as f64 / (day_ticks as f64);
    factory.curr_day_progress = day_progress;

    factory_collect_stats(config, options, state, factory);

    if factory.finished_at <= 0 && day_progress >= 1.0 {
      factory.finished_at = factory.ticks;
      // factory.finished_with = target_progress as u64 * 100; // Store whole percentage of progress
    }
  }
}

pub fn factory_collect_stats(config: &Config, options: &mut Options, state: &mut State, factory: &mut Factory) {
  let mut total_parts_supplied: u64 = 0;
  let mut total_parts_produced: u64 = 0;
  let mut total_parts_accepted: u64 = 0;
  let mut total_parts_trashed: u64 = 0;

  let collected: Vec<(char, u64)> = vec!();

  for coord in 0..factory.floor.len() {
    match factory.floor[coord].kind {
      CellKind::Empty => {} // Ignore empty cells here
      CellKind::Supply => {
        total_parts_supplied += factory.floor[coord].supply.supplied;
      }
      CellKind::Machine => {
        total_parts_produced += factory.floor[coord].machine.produced;
        total_parts_trashed += factory.floor[coord].machine.trashed;
      }
      CellKind::Belt => {} // Ignore
      CellKind::Demand => {
        for i in 0..factory.floor[coord].demand.received.len() {
          if factory.floor.len() <= coord  { log(format!("coord was incorrect... {} {}", factory.floor.len(), coord)); }
          if factory.floor.len() <= coord  { log(format!("i was incorrect... {} {}", factory.floor[coord].demand.received.len(), i)); }
          let (received_part_index, received_count) = factory.floor[coord].demand.received[i];

          // Update the quote counts (expensive search but these arrays should be tiny, sub-10)
          let mut visible_index = 0;
          for j in 0..factory.quotes.len() {
            // Ignore completed quotes.
            if factory.quotes[j].completed_at > 0 {
              continue;
            }

            // Increment quote totals if demand received a matching part.
            if factory.quotes[j].part_index == received_part_index {
              factory.quotes[j].current_count += received_count;

              state.lasers.push(Laser {
                coord,
                quote_pos: visible_index,
                ttl: 5,
                color: "white".to_string().clone(),
              });

              if factory.quotes[j].current_count >= factory.quotes[j].target_count {
                log(format!("finished quote {} with {} of {}", factory.quotes[j].name, factory.quotes[j].current_count, factory.quotes[j].target_count));
                // This quote is finished so end the day // TODO: multiple parts one quote
                factory.finished_at = factory.ticks;
                state.finished_quotes.push(j); // Start visual candy for this quote in next frame
              }
              break;
            }

            visible_index += 1;
          }

          total_parts_accepted += received_count as u64;
          factory.floor[coord].demand.received.clear();
        }
      }
    }
  }

  factory.supplied = total_parts_supplied;
  factory.produced = total_parts_produced;
  factory.accepted = total_parts_accepted;
  factory.trashed = total_parts_trashed;
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
        factory.floor[coord].demand.received = vec!();
      }
    }
  }

  factory.supplied = 0;
  factory.produced = 0;
  factory.accepted = 0;
  factory.trashed = 0;

  factory.quotes.iter_mut().for_each(|quote| quote.current_count = 0);
}
