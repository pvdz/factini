use super::belt::*;
use super::demand::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::port::*;
use super::supply::*;
use super::state::*;
use super::utils::*;

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
  pub port_u: Port,
  pub port_r: Port,
  pub port_d: Port,
  pub port_l: Port,
  pub ins: Vec<(Direction, usize, usize, Direction)>, // (curr outgoing dir, curr coord (relevant for machines), target coord, target incoming dir)
  pub outs: Vec<(Direction, usize, usize, Direction)>, // (curr outgoing dir, curr coord (relevant for machines), target coord, target incoming dir)
  pub inrot: u64, // Rotate ins vec
  pub outrot: u64, // Rotate outs vec

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
    ins: vec!(),
    outs: vec!(),
    inrot: 0,
    outrot: 0,

    port_u: Port::None,
    port_r: Port::None,
    port_d: Port::None,
    port_l: Port::None,

    marked: false,

    belt: belt_none(),
    machine: machine_none(coord),
    demand: demand_none(),
    supply: supply_none(),
  }
}

pub fn belt_cell(x: usize, y: usize, meta: BeltMeta) -> Cell {
  let coord = x + y * FLOOR_CELLS_W;

  // belt cells do not appear on the edge
  assert!(x > 0);
  assert!(y > 0);
  assert!(x < FLOOR_CELLS_W - 1);
  assert!(y < FLOOR_CELLS_H - 1);

  println!("{:?}", meta);

  return Cell {
    kind: CellKind::Belt,

    ticks: 0,
    auto_counter: 0,
    x,
    y,

    is_edge: false,
    is_side: false,
    is_zero: false,
    coord,
    coord_u: Some(to_coord_up(coord)),
    coord_r: Some(to_coord_right(coord)),
    coord_d: Some(to_coord_down(coord)),
    coord_l: Some(to_coord_left(coord)),
    ins: vec!(), // To be filled by the auto layout func
    outs: vec!(), // To be filled by the auto layout func
    inrot: 0,
    outrot: 0,

    port_u: meta.port_u, // Port::Unknown,
    port_r: meta.port_r,
    port_d: meta.port_d,
    port_l: meta.port_l,

    marked: false,

    belt: belt_new(meta),
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
    ins: vec!(),
    outs: vec!(),
    inrot: 0,
    outrot: 0,

    port_u: Port::Unknown,
    port_r: Port::Unknown,
    port_d: Port::Unknown,
    port_l: Port::Unknown,

    marked: false,

    belt: belt_none(),
    machine: machine_new(kind, 999, coord, input1, input2, input3, output),
    demand: demand_none(),
    supply: supply_none(),
  };
}

pub fn supply_cell(x: usize, y: usize, part: Part, speed: u64, cooldown: u64, price: i32) -> Cell {
  let coord = x + y * FLOOR_CELLS_W;

  let coord_u = if y == 0                 { None } else { Some(to_coord_up(coord)) };
  let coord_r = if x == FLOOR_CELLS_W - 1 { None } else { Some(to_coord_right(coord)) };
  let coord_d = if y == FLOOR_CELLS_H - 1 { None } else { Some(to_coord_down(coord)) };
  let coord_l = if x == 0                 { None } else { Some(to_coord_left(coord)) };

  let ( neighbor_coord, outgoing_dir, neighbor_incoming_dir ) = get_edge_neighbor(x, y, coord);

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
    ins: vec!(),
    outs: vec!(( outgoing_dir, coord, neighbor_coord, neighbor_incoming_dir )),
    inrot: 0,
    outrot: 0,

    port_u: if y == FLOOR_CELLS_H - 1 { Port::Outbound } else { Port::None },
    port_r: if x == 0                 { Port::Outbound } else { Port::None },
    port_d: if y == 0                 { Port::Outbound } else { Port::None },
    port_l: if x == FLOOR_CELLS_W - 1 { Port::Outbound } else { Port::None },

    marked: false,

    belt: belt_none(),
    machine: machine_none(coord),
    demand: demand_none(),
    supply: supply_new(part, neighbor_coord, outgoing_dir, neighbor_incoming_dir, speed, cooldown, price),
  };
}

pub fn demand_cell(x: usize, y: usize, part: Part) -> Cell {
  let coord = x + y * FLOOR_CELLS_W;

  let coord_u = if y == 0 { None } else { Some(to_coord_up(coord)) };
  let coord_r = if x == FLOOR_CELLS_W - 1 { None } else { Some(to_coord_right(coord)) };
  let coord_d = if y == FLOOR_CELLS_H - 1 { None } else { Some(to_coord_down(coord)) };
  let coord_l = if x == 0 { None } else { Some(to_coord_left(coord)) };

  let ( neighbor_coord, incoming_dir, neighbor_outgoing_dir) = get_edge_neighbor(x, y, coord);

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
    ins: vec!(( incoming_dir, coord, neighbor_coord, neighbor_outgoing_dir )),
    outs: vec!(),
    inrot: 0,
    outrot: 0,

    port_u: if y == FLOOR_CELLS_H - 1 { Port::Inbound } else { Port::None },
    port_r: if x == 0                 { Port::Inbound } else { Port::None },
    port_d: if y == 0                 { Port::Inbound } else { Port::None },
    port_l: if x == FLOOR_CELLS_W - 1 { Port::Inbound } else { Port::None },

    marked: false,

    belt: belt_none(),
    machine: machine_none(coord),
    demand: demand_new(part, neighbor_coord, incoming_dir, neighbor_outgoing_dir),
    supply: supply_none(),
  };
}

pub fn fix_belt_meta(factory: &mut Factory, coord: usize) {
  let belt_type = get_belt_type_for_cell_ports(factory, coord);
  let belt_meta = belt_type_to_belt_meta(belt_type);
  factory.floor[coord].belt.meta = belt_meta;
}

pub fn update_meta_to_belt_type(factory: &mut Factory, coord: usize, belt_type: BeltType) {
  let meta = belt_type_to_belt_meta(belt_type);

  if meta.port_u != Port::None {
    if factory.floor[coord].port_u == Port::None {
      factory.floor[coord].port_u = Port::Unknown;
    }
  } else {
    if factory.floor[coord].port_u != Port::None {
      factory.floor[coord].port_u = Port::None;
    }
  }

  if meta.port_r != Port::None {
    if factory.floor[coord].port_r == Port::None {
      factory.floor[coord].port_r = Port::Unknown;
    }
  } else {
    if factory.floor[coord].port_r != Port::None {
      factory.floor[coord].port_r = Port::None;
    }
  }

  if meta.port_d != Port::None {
    if factory.floor[coord].port_d == Port::None {
      factory.floor[coord].port_d = Port::Unknown;
    }
  } else {
    if factory.floor[coord].port_d != Port::None {
      factory.floor[coord].port_d = Port::None;
    }
  }

  if meta.port_l != Port::None {
    if factory.floor[coord].port_l == Port::None {
      factory.floor[coord].port_l = Port::Unknown;
    }
  } else {
    if factory.floor[coord].port_l != Port::None {
      factory.floor[coord].port_l = Port::None;
    }
  }

  factory.floor[coord].belt.meta = meta;
}

pub fn update_meta_to_belt_type_and_belt_neighbors(factory: &mut Factory, coord: usize, belt_type: BeltType, fix_neighbors_too: bool) {
  update_meta_to_belt_type(factory, coord, belt_type);
  update_ports_of_neighbor_cells(factory, coord, fix_neighbors_too);
}

pub fn update_ports_of_neighbor_cells(factory: &mut Factory, coord: usize, fix_neighbors_too: bool) {
  // For each side with a port, check if the other side is a belt, and if so, force a port there too

  if factory.floor[coord].port_u != Port::None {
    if let Some(ocoord) = factory.floor[coord].coord_u {
      if factory.floor[ocoord].kind == CellKind::Belt {
        if factory.floor[ocoord].port_d == Port::None {
          factory.floor[ocoord].port_d = Port::Unknown;
        }
        if fix_neighbors_too {
          fix_belt_meta(factory, ocoord);
        }
      }
    }
  }

  if factory.floor[coord].port_r != Port::None {
    if let Some(coord) = factory.floor[coord].coord_r {
      if factory.floor[coord].kind == CellKind::Belt {
        if factory.floor[coord].port_l == Port::None {
          factory.floor[coord].port_l = Port::Unknown;
        }
        if fix_neighbors_too {
          fix_belt_meta(factory, coord);
        }
      }
    }
  }

  if factory.floor[coord].port_d != Port::None {
    if let Some(coord) = factory.floor[coord].coord_d {
      if factory.floor[coord].kind == CellKind::Belt {
        if factory.floor[coord].port_u == Port::None {
          factory.floor[coord].port_u = Port::Unknown;
        }
        if fix_neighbors_too {
          fix_belt_meta(factory, coord);
        }
      }
    }
  }

  if factory.floor[coord].port_l != Port::None {
    if let Some(coord) = factory.floor[coord].coord_l {
      if factory.floor[coord].kind == CellKind::Belt {
        if factory.floor[coord].port_r == Port::None {
          factory.floor[coord].port_r = Port::Unknown;
        }
        if fix_neighbors_too {
          fix_belt_meta(factory, coord);
        }
      }
    }
  }
}

pub fn get_cell_kind_at(factory: &mut Factory, coord: Option<usize>) -> CellKind {
  return match coord {
    None => CellKind::Empty,
    Some(coord) => factory.floor[coord].kind,
  };
}
