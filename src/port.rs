use std::borrow::Borrow;
use std::convert::TryInto;

use super::belt::*;
use super::belt_type::*;
use super::belt_type::*;
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
use super::log;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Port {
  Inbound,
  Outbound,
  None,
  Unknown,
}

pub fn port_disconnect_cells(options: &Options, state: &State, config: &Config, factory: &mut Factory, coord1: usize, dir1: Direction, coord2: usize, dir2: Direction) {
  // Note: this still requires fixing the cell meta and a new factory prio
  port_disconnect_cell(options, state, config, factory, coord1, dir1);
  port_disconnect_cell(options, state, config, factory, coord2, dir2);
}
pub fn port_disconnect_cell(options: &Options, state: &State, config: &Config, factory: &mut Factory, coord: usize, dir: Direction) {
  // Some cells have fixed ports, only belts are dynamic, machines reset to unknown
  // Note: this still requires fixing the cell meta and a new factory prio
  // If leave_stubs is false, if the coord ends up as a BeltType::NONE, it will change to CellKind::Empty

  if factory.floor[coord].kind == CellKind::Belt || factory.floor[coord].kind == CellKind::Machine {
    let ocoord =
      match dir {
        Direction::Up => to_coord_up(coord),
        Direction::Right => to_coord_right(coord),
        Direction::Down => to_coord_down(coord),
        Direction::Left => to_coord_left(coord),
      };

    match dir {
      Direction::Up => cell_set_port_u_to(options, state, config, factory, coord, Port::None, ocoord),
      Direction::Right => cell_set_port_r_to(options, state, config, factory, coord, Port::None, ocoord),
      Direction::Down => cell_set_port_d_to(options, state, config, factory, coord, Port::None, ocoord),
      Direction::Left => cell_set_port_l_to(options, state, config, factory, coord, Port::None, ocoord),
    }

    change_none_belt_to_empty_cell(config, factory, coord);
    change_none_belt_to_empty_cell(config, factory, ocoord);
  }

  belt_receive_part(factory, coord, Direction::Up, part_none(config));
}
pub fn change_none_belt_to_empty_cell(config: &Config, factory: &mut Factory, coord: usize) {
  if factory.floor[coord].kind == CellKind::Belt && factory.floor[coord].belt.meta.btype == BeltType::NONE {
    // TOFIX: this should be done in a special step during the factory.changed check
    log!("change_none_belt_to_empty_cell: changing @{} to empty cell because it is a none belt", coord);
    let (x, y) = to_xy(coord);
    factory.floor[coord] = empty_cell(config, x, y);
    factory.changed = true;
  }
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

pub fn port_has_outbound(factory: &Factory, coord: usize) -> bool {
  return factory.floor[coord].port_u == Port::Outbound || factory.floor[coord].port_r == Port::Outbound || factory.floor[coord].port_d == Port::Outbound || factory.floor[coord].port_l == Port::Outbound;
}

pub fn port_to_char(port: Port) -> char {
  return match port {
    Port::Inbound => { 'i' },
    Port::Outbound => { 'o' },
    Port::Unknown => { '?' },
    Port::None => { '-' },
  };
}
