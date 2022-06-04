use super::belt::*;
use super::demand::*;
use super::floor::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::port::*;
use super::supply::*;
use super::state::*;

#[derive(Debug)]
pub struct Cell {
  pub kind: CellKind,

  pub ticks: u64, // Number of ticks this tile has existed. Used for progress.
  pub auto_counter: u32, // When doing auto-layout/port stuff, this is part of the "painting" algo
  pub x: usize,
  pub y: usize,

  // Precompute commonly used values
  pub is_edge: bool, // cells in outer ring or inside of floor? (demand/supply vs belt/machine)
  pub is_side: bool, // left-right side?
  pub is_zero: bool, // top-left side?
  pub coord: usize,
  pub coord_u: Option<usize>, // Note: invalid if top edge
  pub coord_r: Option<usize>, // Note: invalid if right edge
  pub coord_d: Option<usize>, // Note: invalid if bottom edge
  pub coord_l: Option<usize>, // Note: invalid if left edge

  // Dynamic port assignments
  pub direction_u: Port,
  pub direction_r: Port,
  pub direction_d: Port,
  pub direction_l: Port,

  // This flag is used during pathing
  pub marked: bool,

  // Specific information per kind. Unused ones are "empty".
  pub belt: Belt,
  pub machine: Machine,
  pub supply: Supply,
  pub demand: Demand,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellKind {
  Empty,
  Belt,
  Machine,
  Supply,
  Demand,
}

pub const fn empty_cell(x: usize, y: usize) -> Cell {
  let coord = x + y * FLOOR_CELLS_W;

  let coord_u = if y == 0 { None } else { Some(to_coord_up(coord)) };
  let coord_r = if x == FLOOR_CELLS_W - 1 { None } else { Some(to_coord_right(coord)) };
  let coord_d = if y == FLOOR_CELLS_H - 1 { None } else { Some(to_coord_down(coord)) };
  let coord_l = if x == 0 { None } else { Some(to_coord_left(coord)) };

  Cell {
    kind: CellKind::Empty,

    ticks: 0,
    auto_counter: 0,
    x,
    y,

    is_edge: x == 0 || y == 0 || x == FLOOR_CELLS_W - 1 || y == FLOOR_CELLS_H - 1,
    is_side: x == 0 || x == FLOOR_CELLS_W - 1,
    is_zero: x == 0 || y == 0,
    coord,
    coord_u,
    coord_r,
    coord_d,
    coord_l,

    direction_u: Port::None,
    direction_r: Port::None,
    direction_d: Port::None,
    direction_l: Port::None,

    marked: false,

    belt: belt_none(),
    machine: machine_none(coord),
    demand: demand_none(),
    supply: supply_none(),
  }
}

pub fn belt_cell(x: usize, y: usize, belt: BeltMeta) -> Cell {
  let coord = x + y * FLOOR_CELLS_W;

  let coord_u = if y == 0 { None } else { Some(to_coord_up(coord)) };
  let coord_r = if x == FLOOR_CELLS_W - 1 { None } else { Some(to_coord_right(coord)) };
  let coord_d = if y == FLOOR_CELLS_H - 1 { None } else { Some(to_coord_down(coord)) };
  let coord_l = if x == 0 { None } else { Some(to_coord_left(coord)) };

  return Cell {
    kind: CellKind::Belt,

    ticks: 0,
    auto_counter: 0,
    x,
    y,

    is_edge: x == 0 || y == 0 || x == FLOOR_CELLS_W - 1 || y == FLOOR_CELLS_H - 1,
    is_side: x == 0 || x == FLOOR_CELLS_W - 1,
    is_zero: x == 0 || y == 0,
    coord,
    coord_u,
    coord_r,
    coord_d,
    coord_l,

    direction_u: Port::Unknown,
    direction_r: Port::Unknown,
    direction_d: Port::Unknown,
    direction_l: Port::Unknown,

    marked: false,

    belt: belt_new(belt),
    machine: machine_none(coord),
    demand: demand_none(),
    supply: supply_none(),
  };
}

pub fn machine_cell(x: usize, y: usize, kind: MachineKind, input1: Part, input2: Part, input3: Part, output: Part, machine_production_price: i32, machine_trash_price: i32) -> Cell {
  let coord = x + y * FLOOR_CELLS_W;

  let coord_u = if y == 0 { None } else { Some(to_coord_up(coord)) };
  let coord_r = if x == FLOOR_CELLS_W - 1 { None } else { Some(to_coord_right(coord)) };
  let coord_d = if y == FLOOR_CELLS_H - 1 { None } else { Some(to_coord_down(coord)) };
  let coord_l = if x == 0 { None } else { Some(to_coord_left(coord)) };

  return Cell {
    kind: CellKind::Machine,

    ticks: 0,
    auto_counter: 0,
    x,
    y,

    is_edge: x == 0 || y == 0 || x == FLOOR_CELLS_W - 1 || y == FLOOR_CELLS_H - 1,
    is_side: x == 0 || x == FLOOR_CELLS_W - 1,
    is_zero: x == 0 || y == 0,
    coord,
    coord_u,
    coord_r,
    coord_d,
    coord_l,

    direction_u: Port::Unknown,
    direction_r: Port::Unknown,
    direction_d: Port::Unknown,
    direction_l: Port::Unknown,

    marked: false,

    belt: belt_none(),
    machine: machine_new(kind, 999, coord, input1, input2, input3, output),
    demand: demand_none(),
    supply: supply_none(),
  };
}

pub fn supply_cell(x: usize, y: usize, part: Part, speed: u64, interval: u64, price: i32) -> Cell {
  let coord = x + y * FLOOR_CELLS_W;

  let coord_u = if y == 0                 { None } else { Some(to_coord_up(coord)) };
  let coord_r = if x == FLOOR_CELLS_W - 1 { None } else { Some(to_coord_right(coord)) };
  let coord_d = if y == FLOOR_CELLS_H - 1 { None } else { Some(to_coord_down(coord)) };
  let coord_l = if x == 0                 { None } else { Some(to_coord_left(coord)) };

  let neighbor_coord: usize = get_edge_neighbor(x, y, coord);

  return Cell {
    kind: CellKind::Supply,

    ticks: 0,
    auto_counter: 0,
    x,
    y,

    is_edge: x == 0 || y == 0 || x == FLOOR_CELLS_W - 1 || y == FLOOR_CELLS_H - 1,
    is_side: x == 0 || x == FLOOR_CELLS_W - 1,
    is_zero: x == 0 || y == 0,
    coord,
    coord_u,
    coord_r,
    coord_d,
    coord_l,

    direction_u: if y == FLOOR_CELLS_H - 1 { Port::Outbound } else { Port::None },
    direction_r: if x == 0                 { Port::Outbound } else { Port::None },
    direction_d: if y == 0                 { Port::Outbound } else { Port::None },
    direction_l: if x == FLOOR_CELLS_W - 1 { Port::Outbound } else { Port::None },

    marked: false,

    belt: belt_none(),
    machine: machine_none(coord),
    demand: demand_none(),
    supply: supply_new(part, neighbor_coord, speed, interval, price),
  };
}

pub fn demand_cell(x: usize, y: usize, part: Part) -> Cell {
  let coord = x + y * FLOOR_CELLS_W;

  let coord_u = if y == 0 { None } else { Some(to_coord_up(coord)) };
  let coord_r = if x == FLOOR_CELLS_W - 1 { None } else { Some(to_coord_right(coord)) };
  let coord_d = if y == FLOOR_CELLS_H - 1 { None } else { Some(to_coord_down(coord)) };
  let coord_l = if x == 0 { None } else { Some(to_coord_left(coord)) };

  let neighbor_coord: usize = get_edge_neighbor(x, y, coord);

  return Cell {
    kind: CellKind::Demand,

    ticks: 0,
    auto_counter: 0,
    x,
    y,

    is_edge: x == 0 || y == 0 || x == FLOOR_CELLS_W - 1 || y == FLOOR_CELLS_H - 1,
    is_side: x == 0 || x == FLOOR_CELLS_W - 1,
    is_zero: x == 0 || y == 0,
    coord,
    coord_u,
    coord_r,
    coord_d,
    coord_l,

    direction_u: if y == FLOOR_CELLS_H - 1 { Port::Inbound } else { Port::None },
    direction_r: if x == 0                 { Port::Inbound } else { Port::None },
    direction_d: if y == 0                 { Port::Inbound } else { Port::None },
    direction_l: if x == FLOOR_CELLS_W - 1 { Port::Inbound } else { Port::None },

    marked: false,

    belt: belt_none(),
    machine: machine_none(coord),
    demand: demand_new(part, neighbor_coord),
    supply: supply_none(),
  };
}
