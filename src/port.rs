use std::borrow::Borrow;
use std::convert::TryInto;

use super::belt::*;
use super::cell::*;
use super::config::*;
use super::demand::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::machine::*;
use super::options::*;
use super::part::*;
use super::state::*;
use super::supply::*;
use super::utils::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Port {
  Inbound,
  Outbound,
  None,
  Unknown,
}

pub fn port_disconnect_cells(config: &Config, factory: &mut Factory, coord1: usize, dir1: Direction, coord2: usize, dir2: Direction) {
  // Note: this still requires fixing the cell meta and a new factory prio
  port_disconnect_cell(config, factory, coord1, dir1);
  port_disconnect_cell(config, factory, coord2, dir2);
}
pub fn port_disconnect_cell(config: &Config, factory: &mut Factory, coord: usize, dir: Direction) {
  // Some cells have fixed ports, only belts are dynamic, machines reset to unknown
  // Note: this still requires fixing the cell meta and a new factory prio

  if factory.floor[coord].kind == CellKind::Belt || factory.floor[coord].kind == CellKind::Machine {
    match dir {
      Direction::Up => cell_set_port_u_to(factory, coord, Port::None, to_coord_up(coord)),
      Direction::Right => cell_set_port_r_to(factory, coord, Port::None, to_coord_right(coord)),
      Direction::Down => cell_set_port_d_to(factory, coord, Port::None, to_coord_down(coord)),
      Direction::Left => cell_set_port_l_to(factory, coord, Port::None, to_coord_left(coord)),
    }
  }

  belt_receive_part(factory, coord, Direction::Up, part_none(config));
}

pub fn serialize_ports(factory: &Factory, coord: usize) -> String {
  return format!("ports @{}:  {:?}  {:?}  {:?}  {:?}", coord, factory.floor[coord].port_u, factory.floor[coord].port_r, factory.floor[coord].port_d, factory.floor[coord].port_l);
}

pub fn port_count(factory: &Factory, coord: usize) -> u8 {
  return
    if factory.floor[coord].port_u != Port::None { 1 } else { 0 } +
    if factory.floor[coord].port_r != Port::None { 1 } else { 0 } +
    if factory.floor[coord].port_d != Port::None { 1 } else { 0 } +
    if factory.floor[coord].port_l != Port::None { 1 } else { 0 };
}

pub fn port_to_char(port: Port) -> char {
  return match port {
    Port::Inbound => { 'i' },
    Port::Outbound => { 'o' },
    Port::Unknown => { '?' },
    Port::None => { '-' },
  };
}
