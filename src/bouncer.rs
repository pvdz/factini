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
  /**
   * x, y, added at tick
   */
  pub frames: VecDeque<( f64, f64, u64 )>,
  pub last_progress: f64,
}

pub fn bouncer_create(x: f64, y: f64, max_y: f64, quest_index: PartKind, part_index: usize, init_speed: f64, created_at: u64, delay: u64) -> Bouncer {
  let mut frames = VecDeque::new();
  frames.push_back((x, y, created_at));
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
    frames,
    last_progress: 0.0,
  };
}

pub fn bouncer_step(options: &Options, factory: &mut Factory, bouncer_index: usize) -> f64{
  let ticks = factory.ticks;

  // Do not process bouncer if still in delay period
  if ticks - factory.bouncers[bouncer_index].created_at < factory.bouncers[bouncer_index].delay {
    return 0.0;
  }

  // Do not process bouncer any further if it's not moving horizontally
  if factory.bouncers[bouncer_index].dx.abs() < options.bouncer_speed_limit {
    return 0.0;
  }

  let bouncer_progress = (ticks - factory.bouncers[bouncer_index].created_at) - factory.bouncers[bouncer_index].delay;
  // Dilate time depending on the ui speed
  let bouncer_progress = ((bouncer_progress as f64) / options.speed_modifier_floor) * options.speed_modifier_ui;
  let mut delta_progress = bouncer_progress - factory.bouncers[bouncer_index].last_progress;

  if delta_progress < 1.0 {
    return 0.0;
  }

  // For each frame of progress (can be multiple when game runs faster than ui speed)
  while delta_progress > 1.0 {
    factory.bouncers[bouncer_index].last_progress += 1.0;
    delta_progress -= 1.0;

    factory.bouncers[bouncer_index].last_progress = bouncer_progress;

    factory.bouncers[bouncer_index].dx *= options.bouncer_friction;
    factory.bouncers[bouncer_index].dy += options.bouncer_gravity;
    factory.bouncers[bouncer_index].x += factory.bouncers[bouncer_index].dx;
    factory.bouncers[bouncer_index].y += factory.bouncers[bouncer_index].dy;

    if factory.bouncers[bouncer_index].y > factory.bouncers[bouncer_index].max_y {
      // Step has taken us below the floor, so we need to rebound the bouncer.
      factory.bouncers[bouncer_index].y -= factory.bouncers[bouncer_index].y - factory.bouncers[bouncer_index].max_y;
      factory.bouncers[bouncer_index].dy = -factory.bouncers[bouncer_index].dy * options.bouncer_bounce;
    }

    // Add shadow frames for running bouncers

    // Create an extra still frame of existing bouncers. Only if the frame is to be painted at all.
    let framed = bouncer_progress as u64 > 0 && bouncer_should_paint(&options, &mut factory.bouncers[bouncer_index], factory.ticks);
    if framed {
      if bouncer_progress as u64 % options.bouncer_stamp_interval == 0 {
        let x = factory.bouncers[bouncer_index].x;
        let y = factory.bouncers[bouncer_index].y;
        factory.bouncers[bouncer_index].frames.push_back( ( x, y, factory.ticks ) );
      }
    }
  }

  return bouncer_progress;
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

  return true;
}
