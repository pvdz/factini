use std::collections::VecDeque;
use crate::auto::AutoBuildPhase;

use super::auto::*;
use super::belt::*;
use super::bouncer::*;
use super::cell::*;
use super::cli_serialize::*;
use super::cli_deserialize::*;
use super::config::*;
use super::demand::*;
use super::factory::*;
use super::floor::*;
use super::machine::*;
use super::maze::*;
use super::options::*;
use super::part::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::quest_state::*;
use super::quest::*;
use super::state::*;
use super::supply::*;
use super::truck::*;
use super::utils::*;
use super::zone::*;
use super::log;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
  Up = 0,
  Right = 1,
  Down = 2,
  Left = 3,
}

pub fn direction_reverse(dir: Direction) -> Direction {

  // ((dir as u8 + 2) % 4) as Direction

  match dir {
    Direction::Up => Direction::Down,
    Direction::Right => Direction::Left,
    Direction::Down => Direction::Up,
    Direction::Left => Direction::Right,
  }
}

pub fn get_from_dir_between_xy(x0: usize, y0: usize, x1: usize, y1: usize) -> Direction {
  let dx = x1 as i8 - x0 as i8;
  let dy = y1 as i8 - y0 as i8;
  return match (dx, dy) {
    (  0,  -1 ) => Direction::Down,
    (  1,   0 ) => Direction::Left,
    (  0,   1 ) => Direction::Up,
    ( -1,   0 ) => Direction::Right,

    _ => panic!("what combination is this? {} {}", dx, dy),
  };
}

pub fn set_dir_to(factory: &mut Factory, coord: usize, dir: Direction, port: Port) {
  match dir {
    Direction::Down => factory.floor[coord].port_d = port,
    Direction::Left => factory.floor[coord].port_l = port,
    Direction::Up => factory.floor[coord].port_u = port,
    Direction::Right => factory.floor[coord].port_r = port,
  }
}