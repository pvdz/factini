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
use super::quote::*;
use super::quest_state::*;
use super::utils::*;
use super::zone::*;
use super::log;

#[derive(Clone, Debug)]
pub struct Bouncer {
  pub created_at: u64,
  pub delay: u64,
  pub x: f64,
  pub y: f64,
  pub max_y: f64,
  pub quest_index: PartKind,
  pub part_kind: usize,
  pub dx: f64,
  pub dy: f64,
  /**
   * x, y, added at tick
   */
  pub frames: VecDeque<( f64, f64, u64 )>,
  pub last_progress: f64,
  pub bounce_from_index: usize,
  pub bouncing_at: u64,
}

pub fn bouncer_create(x: f64, y: f64, max_y: f64, quest_index: PartKind, part_kind: PartKind, init_speed: f64, created_at: u64, delay: u64) -> Bouncer {
  let mut frames = VecDeque::new();
  frames.push_back((x, y, created_at));
  return Bouncer {
    created_at,
    delay,
    x,
    y,
    max_y,
    quest_index,
    part_kind,
    dx: init_speed,
    dy: 0.0,
    frames,
    last_progress: 0.0,
    bounce_from_index: 0,
    bouncing_at: 0,
  };
}

pub fn bouncer_xy_at_t(options: &Options, tick_age: u64, initial_visibile_quest_index: usize) -> (f64, f64) {
  // This is a "damped springs" formula, with absolute y values. It sort of mimics a bouncing ball.
  let t = (tick_age as f64 / (options.bouncer_time_to_factory * ONE_SECOND as f64 * options.speed_modifier_ui)).max(0.0).min(1.0);
  let offset_y = get_quest_xy(initial_visibile_quest_index, 0.0).1;
  let bottom_y = UI_QUOTES_OFFSET_Y + UI_QUOTES_HEIGHT + 100.0;
  let start_height: f64 = bottom_y - offset_y; // start/max height
  let x: f64 = t * options.bouncer_formula_total_distance;
  let y: f64 = start_height * (-options.bouncer_decay_rate_modifier * t.powf(options.bouncer_amplitude_decay_rate)).exp() * ( (options.bouncer_initial_angle + options.bouncer_angular_freq * t.powf(options.bouncer_wave_decay_rate)).sin());
  return (x, -y.abs());
}
