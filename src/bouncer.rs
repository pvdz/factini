use std::collections::VecDeque;

use super::belt::*;
use super::cell::*;
use super::cli_serialize::*;
use super::config::*;
use crate::craft::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::init::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::paste::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::utils::*;
use super::log;

#[derive(Clone, Debug)]
pub struct Bouncer {
  pub created_at: u64,
  pub delay: u64,
  pub x: f64,
  pub y: f64,
  pub max_y: f64,
  pub quest_index: PartKind,
  pub part_index: usize,
  pub dx: f64,
  pub dy: f64,
  pub last_time: u64,
  /**
   * x, y, added at tick
   */
  pub frames: VecDeque<( f64, f64, u64 )>,
}

pub fn bouncer_create(x: f64, y: f64, max_y: f64, quest_index: PartKind, part_index: usize, init_speed: f64, created_at: u64, delay: u64) -> Bouncer {
  return Bouncer {
    created_at,
    delay,
    x,
    y,
    max_y,
    quest_index,
    part_index,
    dx: init_speed,
    dy: 0.0,
    last_time: 0,
    frames: VecDeque::new(),
  };
}

pub fn bouncer_step(options: &Options, bouncer: &mut Bouncer, ticks: u64) {
  // Do not process bouncer any further if it's not moving horizontally
  if bouncer.dx.abs() < options.bouncer_speed_limit {
    return;
  }

  // Do not process bouncer if still in delay period
  if ticks - bouncer.created_at < bouncer.delay {
    return;
  }

  if bouncer.last_time == 0 {
    bouncer.last_time = ticks;
    return;
  }

  let elapsed = ticks - bouncer.last_time;
  bouncer.last_time = ticks;

  bouncer.dx *= options.bouncer_friction;
  bouncer.dy += options.bouncer_gravity;
  bouncer.x += bouncer.dx;
  bouncer.y += bouncer.dy;

  if bouncer.y > bouncer.max_y {
    // Step has taken us below the floor, so we need to rebound the bouncer.
    bouncer.y -= bouncer.y - bouncer.max_y;
    bouncer.dy = -bouncer.dy * options.bouncer_bounce;
  }
}

pub fn bouncer_should_paint(options: &Options, bouncer: &mut Bouncer, ticks: u64) -> bool {
  // Do not process bouncer any further if it's not moving horizontally
  if bouncer.dx.abs() < options.bouncer_speed_limit {
    return false;
  }

  // Do not process bouncer if still in delay period
  if ticks - bouncer.created_at < bouncer.delay {
    return false;
  }

  if bouncer.last_time == 0 {
    return true;
  }

  return true;
}
