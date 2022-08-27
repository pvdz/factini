use std::collections::VecDeque;

use super::belt::*;
use super::cell::*;
use super::cli_serialize::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::init::*;
use super::offer::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::paste::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::utils::*;
use crate::craft::CraftInteractable;

const GRAV: f64 = 0.008;      // How fast it drops
const BOUNCINESS: f64 = 0.80; // Velocity retained after a bounce
const LIMIT: f64 = 0.1;       // Minimum velocity required to keep animation running

#[derive(Clone, Debug)]
pub struct Bouncer {
  pub created_at: u64,
  pub delay: u64,
  pub x: f64,
  pub y: f64,
  pub max_y: f64,
  pub icon: char,
  pub dx: f64,
  pub dy: f64,
  pub last_time: u64,
  /**
   * x, y, added at tick
   */
  pub frames: VecDeque<( f64, f64, u64 )>,
  pub dump_trucked_at: u64,
  pub recipe_index: usize,
}

pub fn bouncer_create(x: f64, y: f64, max_y: f64, icon: char, init_speed: f64, created_at: u64, delay: u64) -> Bouncer {
  return Bouncer {
    created_at,
    delay,
    x,
    y,
    max_y,
    icon,
    dx: init_speed,
    dy: 0.0,
    last_time: 0,
    frames: VecDeque::new(),
    dump_trucked_at: 0,
    recipe_index: 0,
  };
}

pub fn bouncer_step(bouncer: &mut Bouncer, ticks: u64) -> bool {
  // Do not process bouncer any further if it's not moving horizontally
  if bouncer.dx.abs() < LIMIT {
    return false;
  }

  // Do not process bouncer if still in delay period
  if ticks - bouncer.created_at < bouncer.delay {
    return false;
  }

  if bouncer.last_time == 0 {
    bouncer.last_time = ticks;
    return false;
  }

  let elapsed = ticks - bouncer.last_time;
  bouncer.last_time = ticks;

  bouncer.dx *= (0.987 * ((elapsed as f64) / (ONE_SECOND as f64))).max(0.987);
  bouncer.dy += GRAV * (elapsed as f64);
  bouncer.x += bouncer.dx;
  bouncer.y += bouncer.dy;

  if bouncer.y > bouncer.max_y {
    // Step has taken us below the floor, so we need to rebound the bouncer.
    bouncer.y -= bouncer.y - bouncer.max_y;
    bouncer.dy = -bouncer.dy * BOUNCINESS;
  }

  return true;
}
