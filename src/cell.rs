use super::belt::*;
use super::belt_codes::*;
use super::belt_frame::*;
use super::belt_meta::*;
use super::belt_type::*;
use super::belt_type::*;
use super::config::*;
use super::demand::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::port::*;
use super::port_auto::*;
use super::supply::*;
use super::state::*;
use super::utils::*;
use super::log;

// Clone but not Copy... I don't want to accidentally clone cells when I want to move them
#[derive(Debug, Clone)]
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
  pub ins: Vec<(Direction, usize, usize, Direction)>, // Ordered priority list of tuples: (curr outgoing dir, curr coord (relevant for machines), target coord, target incoming dir)
  pub outs: Vec<(Direction, usize, usize, Direction)>, // Ordered priority list of tuples: (curr outgoing dir, curr coord (relevant for machines), target coord, target incoming dir)

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

pub fn empty_cell(config: &Config, x: usize, y: usize) -> Cell {
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

    port_u: Port::None,
    port_r: Port::None,
    port_d: Port::None,
    port_l: Port::None,

    marked: false,

    belt: belt_none(config),
    machine: machine_none(config, coord),
    demand: demand_none(),
    supply: supply_none(config),
  }
}

pub fn belt_cell(config: &Config, x: usize, y: usize, meta: BeltMeta) -> Cell {
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
    coord_u: Some(to_coord_up(coord)), // Note: always a Some since belt cells cannot appear on the edge, so there's always at least an edge cell next to it, never oob
    coord_r: Some(to_coord_right(coord)),
    coord_d: Some(to_coord_down(coord)),
    coord_l: Some(to_coord_left(coord)),
    ins: vec!(), // To be filled by the auto layout func
    outs: vec!(), // To be filled by the auto layout func

    port_u: meta.port_u,
    port_r: meta.port_r,
    port_d: meta.port_d,
    port_l: meta.port_l,

    marked: false,

    belt: belt_new(config, meta),
    machine: machine_none(config, coord),
    demand: demand_none(),
    supply: supply_none(config),
  };
}

pub fn machine_any_cell(options: &Options, state: &State, config: &Config, id: char, x: usize, y: usize, cell_width: usize, cell_height: usize, kind: MachineKind, wants: Vec<Part>, output: Part, speed: u64, machine_production_price: i32, machine_trash_price: i32) -> Cell {
  assert!(x > 0 && y > 0 && x < FLOOR_CELLS_W - 1 && y < FLOOR_CELLS_H - 1);

  let coord = x + y * FLOOR_CELLS_W;

  let coord_u = Some(to_coord_up(coord)); // if y == 0 { None } else { Some(to_coord_up(coord)) };
  let coord_r = Some(to_coord_right(coord)); // if x == FLOOR_CELLS_W - 1 { None } else { Some(to_coord_right(coord)) };
  let coord_d = Some(to_coord_down(coord)); // if y == FLOOR_CELLS_H - 1 { None } else { Some(to_coord_down(coord)) };
  let coord_l = Some(to_coord_left(coord)); // if x == 0 { None } else { Some(to_coord_left(coord)) };

  return Cell {
    kind: CellKind::Machine,

    ticks: 0,
    auto_counter: 0,
    x,
    y,

    is_edge: false, // x == 0 || y == 0 || x == FLOOR_CELLS_W - 1 || y == FLOOR_CELLS_H - 1,
    is_side: false, // x == 0 || x == FLOOR_CELLS_W - 1,
    is_zero: false, // x == 0 || y == 0,
    coord,
    coord_u,
    coord_r,
    coord_d,
    coord_l,
    ins: vec!(),
    outs: vec!(),

    port_u: Port::Unknown,
    port_r: Port::Unknown,
    port_d: Port::Unknown,
    port_l: Port::Unknown,

    marked: false,

    belt: belt_none(config),
    machine: machine_new(options, state, config, kind, cell_width, cell_height, id, coord, wants, output, speed),
    demand: demand_none(),
    supply: supply_none(config),
  };
}

pub fn machine_main_cell(options: &Options, state: &State, config: &Config, id: char, x: usize, y: usize, cell_width: usize, cell_height: usize, wants: Vec<Part>, output: Part, speed: u64, machine_production_price: i32, machine_trash_price: i32) -> Cell {
  assert!(x > 0 && y > 0 && x < FLOOR_CELLS_W - 1 && y < FLOOR_CELLS_H - 1);

  let coord = x + y * FLOOR_CELLS_W;

  let coord_u = Some(to_coord_up(coord)); // if y == 0 { None } else { Some(to_coord_up(coord)) };
  let coord_r = Some(to_coord_right(coord)); // if x == FLOOR_CELLS_W - 1 { None } else { Some(to_coord_right(coord)) };
  let coord_d = Some(to_coord_down(coord)); // if y == FLOOR_CELLS_H - 1 { None } else { Some(to_coord_down(coord)) };
  let coord_l = Some(to_coord_left(coord)); // if x == 0 { None } else { Some(to_coord_left(coord)) };

  return Cell {
    kind: CellKind::Machine,

    ticks: 0,
    auto_counter: 0,
    x,
    y,

    is_edge: false, // x == 0 || y == 0 || x == FLOOR_CELLS_W - 1 || y == FLOOR_CELLS_H - 1,
    is_side: false, // x == 0 || x == FLOOR_CELLS_W - 1,
    is_zero: false, // x == 0 || y == 0,
    coord,
    coord_u,
    coord_r,
    coord_d,
    coord_l,
    ins: vec!(),
    outs: vec!(),

    port_u: Port::Unknown,
    port_r: Port::Unknown,
    port_d: Port::Unknown,
    port_l: Port::Unknown,

    marked: false,

    belt: belt_none(config),
    machine: machine_new(options, state, config, MachineKind::Main, cell_width, cell_height, id, coord, wants, output, speed),
    demand: demand_none(),
    supply: supply_none(config),
  };
}

pub fn machine_sub_cell(options: &Options, state: &State, config: &Config, id: char, x: usize, y: usize, main_coord: usize, ocw: usize, och: usize) -> Cell {
  assert!(x > 0 && y > 0 && x < FLOOR_CELLS_W - 1 && y < FLOOR_CELLS_H - 1);

  let coord = x + y * FLOOR_CELLS_W;

  let coord_u = Some(to_coord_up(coord)); // if y == 0 { None } else { Some(to_coord_up(coord)) };
  let coord_r = Some(to_coord_right(coord)); // if x == FLOOR_CELLS_W - 1 { None } else { Some(to_coord_right(coord)) };
  let coord_d = Some(to_coord_down(coord)); // if y == FLOOR_CELLS_H - 1 { None } else { Some(to_coord_down(coord)) };
  let coord_l = Some(to_coord_left(coord)); // if x == 0 { None } else { Some(to_coord_left(coord)) };

  return Cell {
    kind: CellKind::Machine,

    ticks: 0,
    auto_counter: 0,
    x,
    y,

    is_edge: false, // x == 0 || y == 0 || x == FLOOR_CELLS_W - 1 || y == FLOOR_CELLS_H - 1,
    is_side: false, // x == 0 || x == FLOOR_CELLS_W - 1,
    is_zero: false, // x == 0 || y == 0,
    coord,
    coord_u,
    coord_r,
    coord_d,
    coord_l,
    ins: vec!(),
    outs: vec!(),

    port_u: Port::Unknown,
    port_r: Port::Unknown,
    port_d: Port::Unknown,
    port_l: Port::Unknown,

    marked: false,

    belt: belt_none(config),
    machine: machine_new(options, state, config, MachineKind::SubBuilding, ocw, och, id, main_coord, vec!(), part_none(config), 666),
    demand: demand_none(),
    supply: supply_none(config),
  };
}

pub fn back_of_the_line(ins_or_outs: &mut Vec<(Direction, usize, usize, Direction)>, index: usize) {
  // Make sure the element at index is at the end of the list.

  let len = ins_or_outs.len();
  assert!(len > 0, "if this function was called then there should at least be one port in this list...");

  // There's three simple cases to deal with: 1, 2, or 3 elements. For machines there's 4+

  if len == 1 {
    // Noop. The list can't change.
    return;
  }

  if len == 2 {
    // Swap if the first element was picked
    // If not then the last element is already last which is a noop
    if index == 0 {
      ins_or_outs.swap(0, 1);
    }
    return;
  }

  if len == 3 {
    // Now the case depends on the index.
    // If index is 0 then swap twice. If index is 1 then swap once. Otherwise do not swap at all.
    // (If index is zero then the needle is at the front and we move it to the middle position
    // first. If the index is one then the needle starts in the middle so don't swap 0 and 1. Then
    // swap the middle with the back in either case. Now in any case, 0, 1, or 2, the needle should
    // be at the back.)
    if index == 0 {
      ins_or_outs.swap(0, 1);
    }
    if index <= 1 {
      ins_or_outs.swap(1, 2);
    }
    return;
  }

  // Len is 4+. This happens with machines who can have one port per outward facing edge.
  // This is the most expensive one because we just use a loop but so be it. I think the
  // above set of cases still catch the majority of machines as well.

  for i in index..len-1 {
    ins_or_outs.swap(i, i+1);
  }
}

pub fn supply_cell(config: &Config, x: usize, y: usize, part: Part, speed: u64, cooldown: u64, price: i32) -> Cell {
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

    port_u: if y == FLOOR_CELLS_H - 1 { Port::Outbound } else { Port::None },
    port_r: if x == 0                 { Port::Outbound } else { Port::None },
    port_d: if y == 0                 { Port::Outbound } else { Port::None },
    port_l: if x == FLOOR_CELLS_W - 1 { Port::Outbound } else { Port::None },

    marked: false,

    belt: belt_none(config),
    machine: machine_none(config, coord),
    demand: demand_none(),
    supply: supply_new(part, neighbor_coord, outgoing_dir, neighbor_incoming_dir, speed, cooldown, price),
  };
}

pub fn demand_cell(config: &Config, x: usize, y: usize, speed: u64, cooldown: u64) -> Cell {
  // log!("demand_cell: speed: {}, cooldown: {}", speed, cooldown);
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

    port_u: if y == FLOOR_CELLS_H - 1 { Port::Inbound } else { Port::None },
    port_r: if x == 0                 { Port::Inbound } else { Port::None },
    port_d: if y == 0                 { Port::Inbound } else { Port::None },
    port_l: if x == FLOOR_CELLS_W - 1 { Port::Inbound } else { Port::None },

    marked: false,

    belt: belt_none(config),
    machine: machine_none(config, coord),
    demand: demand_new(neighbor_coord, incoming_dir, neighbor_outgoing_dir, speed, cooldown),
    supply: supply_none(config),
  };
}

pub fn fix_belt_meta(options: &Options, state: &State, config: &Config, factory: &mut Factory, coord: usize) {
  fix_belt_meta_floor(options, state, &mut factory.floor, coord);
}

pub fn fix_belt_meta_floor(options: &Options, state: &State, floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize) {
  let pu = floor[coord].port_u;
  let pr = floor[coord].port_r;
  let pd = floor[coord].port_d;
  let pl = floor[coord].port_l;
  let belt_type = belt_type_from_ports(pu, pr, pd, pl);
  // if floor[coord].belt.meta.btype != belt_type {
  //   log!("    -- fix_belt_meta() modifying @{}! from: {:?}, to: {:?} ;; old ports: {:?} {:?} {:?} {:?} ;; new ports: {:?} {:?} {:?} {:?}", coord, floor[coord].belt.meta.btype, belt_type, pu, pr, pd, pl, floor[coord].port_u, floor[coord].port_r, floor[coord].port_d, floor[coord].port_l);
  // }
  let belt_meta = belt_type_to_belt_meta(belt_type);
  // log!("--> {:?} --> {:?}", belt_type, belt_meta);
  floor[coord].belt.meta = belt_meta;
}

pub fn connect_belt_to_existing_neighbor_cells(options: &Options, state: &State, config: &Config, factory: &mut Factory, coord: usize) {
  // Note: this still requires factory prio update but it should take care of all the other things
  log!("connect_belt_to_existing_neighbor_cells({}, options.trace_cell_set_port={})", coord, options.trace_cell_set_port);

  if let Some(ocoord) = factory.floor[coord].coord_u {
    match factory.floor[ocoord].kind {
      CellKind::Empty => {}
      CellKind::Belt => {
        if options.trace_cell_set_port { log!("- connecting to belt up"); }
        cell_set_port_d_to(options, state, config, factory, ocoord, Port::Unknown, coord);
        cell_set_port_u_to(options, state, config, factory, coord, Port::Unknown, ocoord);
      }
      CellKind::Machine => {
        if options.trace_cell_set_port {  log!("- connecting to machine up"); }
        cell_set_port_d_to(options, state, config, factory, ocoord, Port::Unknown, coord);
        cell_set_port_u_to(options, state, config, factory, coord, Port::Unknown, ocoord);
      }
      CellKind::Supply => {
        if options.trace_cell_set_port { log!("- connecting to supply up"); }
        assert_eq!(factory.floor[ocoord].port_d, Port::Outbound, "supply port is always outbound");
        cell_set_port_u_to(options, state, config, factory, coord, Port::Inbound, ocoord);
      }
      CellKind::Demand => {
        if options.trace_cell_set_port { log!("- connecting to demand up"); }
        assert_eq!(factory.floor[ocoord].port_d, Port::Inbound, "demand port is always inbound");
        cell_set_port_u_to(options, state, config, factory, coord, Port::Outbound, ocoord);
      }
    }
  }

  if let Some(ocoord) = factory.floor[coord].coord_r {
    match factory.floor[ocoord].kind {
      CellKind::Empty => {}
      CellKind::Belt => {
        if options.trace_cell_set_port { log!("- connecting to belt right"); }
        cell_set_port_l_to(options, state, config, factory, ocoord, Port::Unknown, coord);
        cell_set_port_r_to(options, state, config, factory, coord, Port::Unknown, ocoord);
      }
      CellKind::Machine => {
        if options.trace_cell_set_port {  log!("- connecting to machine right"); }
        cell_set_port_l_to(options, state, config, factory, ocoord, Port::Unknown, coord);
        cell_set_port_r_to(options, state, config, factory, coord, Port::Unknown, ocoord);
      }
      CellKind::Supply => {
        if options.trace_cell_set_port { log!("- connecting to supply right"); }
        assert_eq!(factory.floor[ocoord].port_l, Port::Outbound, "supply port is always outbound");
        cell_set_port_r_to(options, state, config, factory, coord, Port::Inbound, ocoord);
      }
      CellKind::Demand => {
        if options.trace_cell_set_port { log!("- connecting to demand right"); }
        assert_eq!(factory.floor[ocoord].port_l, Port::Inbound, "demand port is always inbound");
        cell_set_port_r_to(options, state, config, factory, coord, Port::Outbound, ocoord);
      }
    }
  }

  if let Some(ocoord) = factory.floor[coord].coord_d {
    match factory.floor[ocoord].kind {
      CellKind::Empty => {}
      CellKind::Belt => {
        if options.trace_cell_set_port { log!("- connecting {} to belt down {}", coord, ocoord); }
        cell_set_port_u_to(options, state, config, factory, ocoord, Port::Unknown, coord);
        cell_set_port_d_to(options, state, config, factory, coord, Port::Unknown, ocoord);
      }
      CellKind::Machine => {
        if options.trace_cell_set_port { log!("- connecting to machine down"); }
        cell_set_port_u_to(options, state, config, factory, ocoord, Port::Unknown, coord);
        cell_set_port_d_to(options, state, config, factory, coord, Port::Unknown, ocoord);
      }
      CellKind::Supply => {
        if options.trace_cell_set_port { log!("- connecting to supply down"); }
        assert_eq!(factory.floor[ocoord].port_u, Port::Outbound, "supply port is always outbound");
        cell_set_port_d_to(options, state, config, factory, coord, Port::Inbound, ocoord);
      }
      CellKind::Demand => {
        if options.trace_cell_set_port { log!("- connecting to demand down"); }
        assert_eq!(factory.floor[ocoord].port_u, Port::Inbound, "demand port is always inbound");
        cell_set_port_d_to(options, state, config, factory, coord, Port::Outbound, ocoord);
      }
    }
  }

  if let Some(ocoord) = factory.floor[coord].coord_l {
    match factory.floor[ocoord].kind {
      CellKind::Empty => {}
      CellKind::Belt => {
        if options.trace_cell_set_port { log!("- connecting to belt left"); }
        cell_set_port_r_to(options, state, config, factory, ocoord, Port::Unknown, coord);
        cell_set_port_l_to(options, state, config, factory, coord, Port::Unknown, ocoord);
      }
      CellKind::Machine => {
        if options.trace_cell_set_port { log!("- connecting to machine left"); }
        cell_set_port_r_to(options, state, config, factory, ocoord, Port::Unknown, coord);
        cell_set_port_l_to(options, state, config, factory, coord, Port::Unknown, ocoord);
      }
      CellKind::Supply => {
        if options.trace_cell_set_port { log!("- connecting to supply left"); }
        assert_eq!(factory.floor[ocoord].port_r, Port::Outbound, "supply port is always outbound");
        cell_set_port_l_to(options, state, config, factory, coord, Port::Inbound, ocoord);
      }
      CellKind::Demand => {
        if options.trace_cell_set_port { log!("- connecting to demand left"); }
        assert_eq!(factory.floor[ocoord].port_r, Port::Inbound, "demand port is always inbound");
        cell_set_port_l_to(options, state, config, factory, coord, Port::Outbound, ocoord);
      }
    }
  }
}

pub fn cell_set_port_u_to(options: &Options, state: &State, config: &Config, factory: &mut Factory, coord_from: usize, port: Port, ocoord: usize) {
  // Note: this still requires factory prio update but it should take care of all the other things

  if options.trace_cell_set_port { log!("cell_set_port_u_to; update port from @{} to @{}: meta before: {:?}, port_u before: {:?} -- ports after: {:?} {:?} {:?} {:?}", coord_from, ocoord, factory.floor[coord_from].belt.meta.btype, factory.floor[coord_from].port_u, port, factory.floor[coord_from].port_r, factory.floor[coord_from].port_d, factory.floor[coord_from].port_l); }

  if factory.floor[coord_from].port_u == port {
    // noop
    return;
  }

  // The .ins and .outs for machines are all stored on the main_coord
  let machine_friendly_coord = if factory.floor[coord_from].kind == CellKind::Machine { factory.floor[coord_from].machine.main_coord } else { coord_from };

  // If currently in/out then that will change so remove it from the .ins and .outs
  match factory.floor[coord_from].port_u {
    Port::Inbound => remove_dir_from_cell_ins(factory, machine_friendly_coord, Direction::Up),
    Port::Outbound => remove_dir_from_cell_outs(factory, machine_friendly_coord, Direction::Up),
    Port::None => {},
    Port::Unknown => {}
  }

  factory.floor[coord_from].port_u = port;
  fix_belt_meta(options, state, config, factory, coord_from);
  match port {
    Port::Inbound => factory.floor[machine_friendly_coord].ins.push(( Direction::Up, coord_from, ocoord, Direction::Down )),
    Port::Outbound => factory.floor[machine_friendly_coord].outs.push(( Direction::Up, coord_from, ocoord, Direction::Down )),
    Port::None => {},
    Port::Unknown => {}
  }
  if options.trace_cell_set_port { log!("- cell_set_port_u_to(@{}, {:?}); meta after: {:?}, ins: {:?}, outs: {:?}", coord_from, port, factory.floor[coord_from].belt.meta.btype, factory.floor[machine_friendly_coord].ins, factory.floor[machine_friendly_coord].outs); }
}
pub fn cell_set_port_r_to(options: &Options, state: &State, config: &Config, factory: &mut Factory, coord_from: usize, port: Port, ocoord: usize) {
  // Note: this still requires factory prio update but it should take care of all the other things

  if options.trace_cell_set_port { log!("cell_set_port_r_to; update port from @{} to @{}: meta before: {:?}, port_r before: {:?} -- ports after: {:?} {:?} {:?} {:?}", coord_from, ocoord, factory.floor[coord_from].belt.meta.btype, factory.floor[coord_from].port_r, factory.floor[coord_from].port_u, port, factory.floor[coord_from].port_d, factory.floor[coord_from].port_l); }

  if factory.floor[coord_from].port_r == port {
    // noop
    return;
  }

  // The .ins and .outs for machines are all stored on the main_coord
  let machine_friendly_coord = if factory.floor[coord_from].kind == CellKind::Machine { factory.floor[coord_from].machine.main_coord } else { coord_from };

  // If currently in/out then that will change so remove it from the .ins and .outs
  match factory.floor[coord_from].port_r {
    Port::Inbound => remove_dir_from_cell_ins(factory, machine_friendly_coord, Direction::Right),
    Port::Outbound => remove_dir_from_cell_outs(factory, machine_friendly_coord, Direction::Right),
    Port::None => {},
    Port::Unknown => {}
  }

  factory.floor[coord_from].port_r = port;
  fix_belt_meta(options, state, config, factory, coord_from);
  match port {
    Port::Inbound => factory.floor[machine_friendly_coord].ins.push(( Direction::Right, coord_from, ocoord, Direction::Left )),
    Port::Outbound => factory.floor[machine_friendly_coord].outs.push(( Direction::Right, coord_from, ocoord, Direction::Left )),
    Port::None => {},
    Port::Unknown => {}
  }
  if options.trace_cell_set_port { log!("- cell_set_port_r_to(@{}, {:?}); meta after: {:?}, ins: {:?}, outs: {:?}", coord_from, port, factory.floor[coord_from].belt.meta.btype, factory.floor[machine_friendly_coord].ins, factory.floor[machine_friendly_coord].outs); }
}
pub fn cell_set_port_d_to(options: &Options, state: &State, config: &Config, factory: &mut Factory, coord_from: usize, port: Port, ocoord: usize) {
  // Note: this still requires factory prio update but it should take care of all the other things

  if options.trace_cell_set_port {  log!("cell_set_port_d_to; update port from @{} to @{}: meta before: {:?}, port_d before: {:?} -- ports after: {:?} {:?} {:?} {:?}", coord_from, ocoord, factory.floor[coord_from].belt.meta.btype, factory.floor[coord_from].port_d, factory.floor[coord_from].port_u, factory.floor[coord_from].port_r, port, factory.floor[coord_from].port_l); }

  if factory.floor[coord_from].port_d == port {
    // noop
    return;
  }

  // The .ins and .outs for machines are all stored on the main_coord
  let machine_friendly_coord = if factory.floor[coord_from].kind == CellKind::Machine { factory.floor[coord_from].machine.main_coord } else { coord_from };

  // If currently in/out then that will change so remove it from the .ins and .outs
  match factory.floor[coord_from].port_d {
    Port::Inbound => remove_dir_from_cell_ins(factory, machine_friendly_coord, Direction::Down),
    Port::Outbound => remove_dir_from_cell_outs(factory, machine_friendly_coord, Direction::Down),
    Port::None => {},
    Port::Unknown => {}
  }

  factory.floor[coord_from].port_d = port;
  fix_belt_meta(options, state, config, factory, coord_from);
  match port {
    Port::Inbound => factory.floor[machine_friendly_coord].ins.push(( Direction::Down, coord_from, ocoord, Direction::Up )),
    Port::Outbound => factory.floor[machine_friendly_coord].outs.push(( Direction::Down, coord_from, ocoord, Direction::Up )),
    Port::None => {},
    Port::Unknown => {}
  }
  if options.trace_cell_set_port { log!("- cell_set_port_d_to(@{}, {:?}); meta after: {:?}, ins: {:?}, outs: {:?}", coord_from, port, factory.floor[coord_from].belt.meta.btype, factory.floor[machine_friendly_coord].ins, factory.floor[machine_friendly_coord].outs); }
}
pub fn cell_set_port_l_to(options: &Options, state: &State, config: &Config, factory: &mut Factory, coord_from: usize, port: Port, ocoord: usize) {
  // Note: this still requires factory prio update but it should take care of all the other things

  if options.trace_cell_set_port { log!("cell_set_port_l_to; update port from @{} to @{}: meta before: {:?}, port_l before: {:?} -- ports after: {:?} {:?} {:?} {:?}", coord_from, ocoord, factory.floor[coord_from].belt.meta.btype, factory.floor[coord_from].port_l, factory.floor[coord_from].port_u, factory.floor[coord_from].port_r, factory.floor[coord_from].port_d, port); }

  if factory.floor[coord_from].port_l == port {
    // noop
    return;
  }

  // The .ins and .outs for machines are all stored on the main_coord
  let machine_friendly_coord = if factory.floor[coord_from].kind == CellKind::Machine { factory.floor[coord_from].machine.main_coord } else { coord_from };

  // If currently in/out then that will change so remove it from the .ins and .outs
  match factory.floor[coord_from].port_l {
    Port::Inbound => remove_dir_from_cell_ins(factory, machine_friendly_coord, Direction::Left),
    Port::Outbound => remove_dir_from_cell_outs(factory, machine_friendly_coord, Direction::Left),
    Port::None => {},
    Port::Unknown => {}
  }

  factory.floor[coord_from].port_l = port;
  fix_belt_meta(options, state, config, factory, coord_from);
  match port {
    Port::Inbound => factory.floor[machine_friendly_coord].ins.push(( Direction::Left, coord_from, ocoord, Direction::Right )),
    Port::Outbound => factory.floor[machine_friendly_coord].outs.push(( Direction::Left, coord_from, ocoord, Direction::Right )),
    Port::None => {},
    Port::Unknown => {}
  }

  if options.trace_cell_set_port { log!("- cell_set_port_l_to(@{}, {:?}); meta after: {:?}, ins: {:?}, outs: {:?}", coord_from, port, factory.floor[coord_from].belt.meta.btype, factory.floor[machine_friendly_coord].ins, factory.floor[machine_friendly_coord].outs); }
}

pub fn cell_connect_if_possible(options: &Options, state: &State, config: &Config, factory: &mut Factory, coord_from: usize, coord_to: usize, dx: i8, dy: i8) {
  // Note: this still requires factory prio update but it should take care of all the other things

  if options.trace_cell_connect {
    log!("cell_connect_if_possible(@{} <-> @{}, options.trace_cell_connect={}) {} {} meta before: {:?} {:?}", coord_from, coord_to, options.trace_cell_connect, dx, dy, factory.floor[coord_from].belt.meta.btype, factory.floor[coord_to].belt.meta.btype);
  } else {
    log!("cell_connect_if_possible(@{} <-> @{}, options.trace_cell_connect={})", coord_from, coord_to, options.trace_cell_connect);
  }

  // The dx and dy values should reflect the coords' deltas. We assume the cells _are_ adjacent and belts or machines.
  assert!((dx == 0) != (dy == 0), "one and only one of dx or dy is zero");
  assert!(dx >= -1 && dx <= 1 && dy >= -1 && dy <= 1, "since they are adjacent they must be -1, 0, or 1");

  // Connect the two cells but:
  // - If one is a supply, force the port of the other to be inbound, regardless
  // - If one is a demand, force the port of the other to be outbound, regardless
  // - If one is empty then do not change any port
  // - If both are machine then do not change any port
  // - Connect belts with each other and with machines

  let from_kind = factory.floor[coord_from].kind;
  let to_kind = factory.floor[coord_to].kind;
  if options.trace_cell_connect { log!("  - to: {:?}, from: {:?}", to_kind, from_kind); }

  // Doing a match is going to complicate the code a lot so it'll just be if-elses to apply the rules

  if from_kind == CellKind::Supply || to_kind == CellKind::Supply {
    assert!(from_kind != CellKind::Demand || to_kind != CellKind::Demand, "not checked here so we assume this");
    match ( dx, dy ) {
      ( 0 , -1 ) => {
        if from_kind != CellKind::Supply { cell_set_port_u_to(options, state, config, factory, coord_from, Port::Inbound, coord_to); }
        if to_kind != CellKind::Supply { cell_set_port_d_to(options, state, config, factory, coord_to, Port::Inbound, coord_from); }
      }
      ( 1 , 0 ) => {
        if from_kind != CellKind::Supply { cell_set_port_r_to(options, state, config, factory, coord_from, Port::Inbound, coord_to); }
        if to_kind != CellKind::Supply { cell_set_port_l_to(options, state, config, factory, coord_to, Port::Inbound, coord_from); }
      }
      ( 0 , 1 ) => {
        if from_kind != CellKind::Supply { cell_set_port_d_to(options, state, config, factory, coord_from, Port::Inbound, coord_to); }
        if to_kind != CellKind::Supply { cell_set_port_u_to(options, state, config, factory, coord_to, Port::Inbound, coord_from); }
      }
      ( -1 , 0 ) => {
        if from_kind != CellKind::Supply { cell_set_port_l_to(options, state, config, factory, coord_from, Port::Inbound, coord_to); }
        if to_kind != CellKind::Supply { cell_set_port_r_to(options, state, config, factory, coord_to, Port::Inbound, coord_from); }
      }
      _ => panic!("already asserted the range of x and y"),
    }
  }
  else if from_kind == CellKind::Demand || to_kind == CellKind::Demand {
    match ( dx, dy ) {
      ( 0 , -1 ) => {
        if from_kind != CellKind::Demand { cell_set_port_u_to(options, state, config, factory, coord_from, Port::Outbound, coord_to); }
        if to_kind != CellKind::Demand { cell_set_port_d_to(options, state, config, factory, coord_to, Port::Outbound, coord_from); }
      }
      ( 1 , 0 ) => {
        if from_kind != CellKind::Demand { cell_set_port_r_to(options, state, config, factory, coord_from, Port::Outbound, coord_to); }
        if to_kind != CellKind::Demand { cell_set_port_l_to(options, state, config, factory, coord_to, Port::Outbound, coord_from); }
      }
      ( 0 , 1 ) => {
        if from_kind != CellKind::Demand { cell_set_port_d_to(options, state, config, factory, coord_from, Port::Outbound, coord_to); }
        if to_kind != CellKind::Demand { cell_set_port_u_to(options, state, config, factory, coord_to, Port::Outbound, coord_from); }
      }
      ( -1 , 0 ) => {
        if from_kind != CellKind::Demand { cell_set_port_l_to(options, state, config, factory, coord_from, Port::Outbound, coord_to); }
        if to_kind != CellKind::Demand { cell_set_port_r_to(options, state, config, factory, coord_to, Port::Outbound, coord_from); }
      }
      _ => panic!("already asserted the range of x and y"),
    }
  }
  else if to_kind == CellKind::Empty || from_kind == CellKind::Empty {
    // Ignore :shrug:
    log!("connecting to empty? nope");
    match ( dx, dy ) {
      ( 0 , -1 ) => {
        cell_set_port_u_to(options, state, config, factory, coord_from, Port::None, coord_to);
        cell_set_port_d_to(options, state, config, factory, coord_to, Port::None, coord_from);
      }
      ( 1 , 0 ) => {
        cell_set_port_r_to(options, state, config, factory, coord_from, Port::None, coord_to);
        cell_set_port_l_to(options, state, config, factory, coord_to, Port::None, coord_from);
      }
      ( 0 , 1 ) => {
        cell_set_port_d_to(options, state, config, factory, coord_from, Port::None, coord_to);
        cell_set_port_u_to(options, state, config, factory, coord_to, Port::None, coord_from);
      }
      ( -1 , 0 ) => {
        cell_set_port_l_to(options, state, config, factory, coord_from, Port::None, coord_to);
        cell_set_port_r_to(options, state, config, factory, coord_to, Port::None, coord_from);
      }
      _ => panic!("already asserted the range of x and y"),
    }
  }
  else if to_kind == CellKind::Machine && from_kind == CellKind::Machine {
    // Don't connect inter-machine parts. Do not connect different machines either. Just don't.
  } else {
    assert!(to_kind == CellKind::Belt || to_kind == CellKind::Machine);
    assert!(from_kind == CellKind::Belt || from_kind == CellKind::Machine);
    assert!(!((from_kind == CellKind::Machine) && (to_kind == CellKind::Machine)));

    // Regardless of whether it's belt2belt or belt2machine, set the port the same way

    match ( dx, dy ) {
      ( 0 , -1 ) => {
        cell_set_port_u_to(options, state, config, factory, coord_from, Port::Outbound, coord_to);
        cell_set_port_d_to(options, state, config, factory, coord_to, Port::Inbound, coord_from);
      }
      ( 1 , 0 ) => {
        cell_set_port_r_to(options, state, config, factory, coord_from, Port::Outbound, coord_to);
        cell_set_port_l_to(options, state, config, factory, coord_to, Port::Inbound, coord_from);
      }
      ( 0 , 1 ) => {
        cell_set_port_d_to(options, state, config, factory, coord_from, Port::Outbound, coord_to);
        cell_set_port_u_to(options, state, config, factory, coord_to, Port::Inbound, coord_from);
      }
      ( -1 , 0 ) => {
        cell_set_port_l_to(options, state, config, factory, coord_from, Port::Outbound, coord_to);
        cell_set_port_r_to(options, state, config, factory, coord_to, Port::Inbound, coord_from);
      }
      _ => panic!("already asserted the range of x and y"),
    }
  }

  if options.trace_cell_connect { log!("- @{} and @{} should now be connected; meta after: {:?} and {:?}", coord_from, coord_to, factory.floor[coord_from].belt.meta.btype, factory.floor[coord_to].belt.meta.btype); }

  // Only rediscover for belts and machines
  if options.trace_cell_connect { log!("  - rediscover .ins and .outs..."); }
  if options.trace_cell_connect { log!("    - @{} ({:?})", get_main_coord(factory, coord_from), from_kind); }
  if from_kind == CellKind::Belt {
    belt_discover_ins_and_outs(factory, get_main_coord(factory, coord_from));
  } else if from_kind == CellKind::Machine {
    machine_discover_ins_and_outs(factory, get_main_coord(factory, coord_from));
  }
  if options.trace_cell_connect { log!("    - @{} ({:?})", get_main_coord(factory, coord_to), to_kind); }
  if to_kind == CellKind::Belt {
    belt_discover_ins_and_outs(factory, get_main_coord(factory, coord_to));
  } else if to_kind == CellKind::Machine {
    machine_discover_ins_and_outs(factory, get_main_coord(factory, coord_to));
  }

  if options.trace_cell_connect {
    log!("    - .ins[@{}]: {:?}", coord_from, factory.floor[get_main_coord(factory, coord_from)].ins.iter().map(|(dir, c, _, _)| ( dir, c ) ).collect::<Vec<(&Direction, &usize)>>());
    log!("    - .outs[@{}]: {:?}", coord_from, factory.floor[get_main_coord(factory, coord_from)].outs.iter().map(|(dir, c, _, _)| ( dir, c ) ).collect::<Vec<(&Direction, &usize)>>());
    log!("    - .ins[@{}]: {:?}", coord_to, factory.floor[get_main_coord(factory, coord_to)].ins.iter().map(|(dir, c, _, _)| ( dir, c ) ).collect::<Vec<(&Direction, &usize)>>());
    log!("    - .outs[@{}]: {:?}", coord_to, factory.floor[get_main_coord(factory, coord_to)].outs.iter().map(|(dir, c, _, _)| ( dir, c ) ).collect::<Vec<(&Direction, &usize)>>());
  }
}

fn get_main_coord(factory: &Factory, coord: usize) -> usize {
  // Given the coord, return it if the cell is not a machine, or return the machine main_coord.
  if factory.floor[coord].kind == CellKind::Machine {
    return factory.floor[coord].machine.main_coord;
  }
  return coord;
}

pub fn get_cell_kind_at(factory: &mut Factory, coord: Option<usize>) -> CellKind {
  return match coord {
    None => CellKind::Empty,
    Some(coord) => factory.floor[coord].kind,
  };
}

pub fn clear_part_from_cell(options: &Options, state: &State, config: &Config, factory: &mut Factory, coord: usize) {
  // Clear the part from this cell
  match factory.floor[coord].kind {
    CellKind::Belt => {
      factory.floor[coord].belt.part = part_none(config);
    }
    CellKind::Empty => {
      // noop
    }
    CellKind::Machine => {
      // clear inputs on main machine cell. Output doesn't exist but we could reset the timer.
      for i in 0..factory.floor[factory.floor[coord].machine.main_coord].machine.haves.len() {
        factory.floor[factory.floor[coord].machine.main_coord].machine.haves[i] = part_none(config);
      }
      // Basically resets the "part is ready to move", since the part never actually exists
      // inside the machine. We could only clear it if it is indeed 100%+ but who cares.
      factory.floor[factory.floor[coord].machine.main_coord].machine.start_at = 0;
    }
    CellKind::Demand => {
      // Noop (received parts disappear)
    }
    CellKind::Supply => {
      // Clear the supplied part (reset timer? prolly doesn't matter)
      factory.floor[coord].supply.part_created_at = 0;
      factory.floor[coord].supply.part_tbd = true;
      factory.floor[coord].supply.part_progress = 0;
    }
  }
}

fn remove_dir_from_cell_ins(factory: &mut Factory, coord: usize, needle_dir: Direction) {
  if let Some(pos) = factory.floor[coord].ins.iter().position(|(dir, ..)| dir == &needle_dir) {
    factory.floor[coord].ins.remove(pos);
  }
}

fn remove_dir_from_cell_outs(factory: &mut Factory, coord: usize, needle_dir: Direction) {
  if let Some(pos) = factory.floor[coord].outs.iter().position(|(dir, ..)| dir == &needle_dir) {
    factory.floor[coord].outs.remove(pos);
  }
}

pub fn cell_ports_to_str(cell: &super::cell::Cell) -> String {
  return format!("up:{} right:{} down:{} left:{}",
    port_to_char(cell.port_u),
    port_to_char(cell.port_r),
    port_to_char(cell.port_d),
    port_to_char(cell.port_l),
  );
}

pub fn cell_neighbors_to_auto_belt_meta(up: CellKind, right: CellKind, down: CellKind, left: CellKind) -> BeltMeta {
  // log!("cell_neighbors_to_auto_belt_meta({:?}, {:?}, {:?}, {:?})", up, right, down, left);
  return match (up, right, down, left) {
    // Empty
    (CellKind::Empty, CellKind::Empty, CellKind::Empty, CellKind::Empty) =>
      BELT_NONE,

    // Stubs
    (_, CellKind::Empty, CellKind::Empty, CellKind::Empty) =>
      BELT___U,
    (CellKind::Empty, _, CellKind::Empty, CellKind::Empty) =>
      BELT___R,
    (CellKind::Empty, CellKind::Empty, _, CellKind::Empty) =>
      BELT___D,
    (CellKind::Empty, CellKind::Empty, CellKind::Empty, _) =>
      BELT___L,

    // Straight
    (CellKind::Empty, _, CellKind::Empty, _) =>
      BELT___LR,
    (_, CellKind::Empty, _, CellKind::Empty) =>
      BELT___DU,

    // Corner
    (_, _, CellKind::Empty, CellKind::Empty) =>
      BELT___RU,
    (_, CellKind::Empty, CellKind::Empty, _) =>
      BELT___LU,
    (CellKind::Empty, CellKind::Empty, _, _) =>
      BELT___DL,
    (CellKind::Empty, _, _, CellKind::Empty) =>
      BELT___DR,

    // T
    (_, _, CellKind::Empty, _) =>
      BELT___LRU,
    (_, CellKind::Empty, _, _) =>
      BELT___DLU,
    (_, _, _, CellKind::Empty) =>
      BELT___DRU,
    (CellKind::Empty, _, _, _) =>
      BELT___DLR,

    // +
    (_, _, _, _) =>
      BELT___DLRU,
  };
}

pub fn belt_connect_cells_expensive(options: &Options, state: &State, config: &Config, factory: &mut Factory, x1: usize, y1: usize, x2: usize, y2: usize) {
  // This should connect two cells with a belt and take care of everything.
  // It will do ray tracing which is relatively expensive.
  // It will fill every cell in between with a belt.
  // Note: It will not auto-connect any of those with neighbors other than the path.

  let track = ray_trace_dragged_line_expensive(factory, x1 as f64, y1 as f64, x2 as f64, y2 as f64);

  for i in 1..track.len() {
    let ((cell_x1, cell_y1), belt_type1, _unused, _port_out_dir1) = track[i-1]; // First element has no inbound port here
    let ((cell_x2, cell_y2), belt_type2, _port_in_dir2, _unused) = track[i]; // Last element has no outbound port here

    apply_action_between_two_cells(state, options, config, factory, Action::Add, cell_x1, cell_y1, belt_type1, cell_x2, cell_y2, belt_type2);
  }
}

pub fn ray_trace_dragged_line_expensive(factory: &Factory, ix0: f64, iy0: f64, ix1: f64, iy1: f64) -> Vec<((usize, usize), BeltType, Direction, Direction)> {
  // We raytracing
  // The dragged line becomes a ray that we trace through cells of the floor
  // We then generate a belt track such that it fits in with the existing belts, if any
  // - Figure out which cells the ray passes through
  // - If the ray crosses existing belts, generate the belt type as if the original was modified to support the new path (the pathing would not destroy existing ports)
  // - If the ray only spans one cell, force it to be invalid
  // - The first and last cells in the ray also auto-connect to any neighbor belts. Sections in the middle of the ray do not.
  // - Special case: if the line starts on an edge but finishes away from that same edge, force the second step to be away from that edge. there's some manual logic to make that work.

  // Check start of path and compensate if on edge
  let x_left0 = ix0 == 0.0 && ix1 != 0.0;
  let y_top0 = iy0 == 0.0 && iy1 != 0.0;
  let x_right0 = ix0 == ((FLOOR_CELLS_W - 1) as f64) && ix1 != ((FLOOR_CELLS_W - 1) as f64);
  let y_bottom0 = iy0 == ((FLOOR_CELLS_H - 1) as f64) && iy1 != ((FLOOR_CELLS_H - 1) as f64);
  let x0 = if x_left0 { ix0 + 1.0 } else if x_right0 { ix0 - 1.0 } else { ix0 };
  let y0 = if y_top0 { iy0 + 1.0 } else if y_bottom0 { iy0 - 1.0 } else { iy0 };

  // Check end of path and compensate if on edge
  let x_left1 = ix1 == 0.0 && x0 != 0.0;
  let y_top1 = iy1 == 0.0 && y0 != 0.0;
  let x_right1 = ix1 == ((FLOOR_CELLS_W - 1) as f64) && x0 != ((FLOOR_CELLS_W - 1) as f64);
  let y_bottom1 = iy1 == ((FLOOR_CELLS_H - 1) as f64) && y0 != ((FLOOR_CELLS_H - 1) as f64);
  let x1 = if x_left1 { ix1 + 1.0 } else if x_right1 { ix1 - 1.0 } else { ix1 };
  let y1 = if y_top1 { iy1 + 1.0 } else if y_bottom1 { iy1 - 1.0 } else { iy1 };

  let mut covered = get_cells_from_a_to_b(x0, y0, x1, y1);
  assert!(covered.len() >= 1, "Should always record at least one cell coord");

  // Now put the start/end of path back if it was moved. This way the path will never have more than one edge cell on the same side
  if x_left0 || y_top0 || x_right0 || y_bottom0 {
    // "push_front"
    let mut t = vec!((ix0 as usize, iy0 as usize));
    t.append(&mut covered);
    covered = t;
  }
  if x_left1 || y_top1 || x_right1 || y_bottom1 {
    covered.push((ix1 as usize, iy1 as usize));
  }

  if covered.len() == 1 {
    return vec!((covered[0], BeltType::INVALID, Direction::Up, Direction::Up));
  }

  // Note: in order of (dragging) appearance
  let mut track: Vec<((usize, usize), BeltType, Direction, Direction)> = vec!(); // ((x, y), new_bt)

  let (mut lx, mut ly) = covered[0];
  let mut last_from = Direction::Up; // first one ignores this value

  // Draw example tiles of the path you're drawing.
  // Take the existing cell and add one (first/last segment) or two ports to it;
  // - first one only gets the "to" port added to it
  // - last one only gets the "from" port added to it
  // - middle parts get the "from" and "to" port added to them
  for index in 1..covered.len() {
    let (x, y) = covered[index];
    // Always set the previous one.
    let new_from = get_from_dir_between_xy(lx, ly, x, y);
    let last_to = direction_reverse(new_from);
    let bt =
      if track.len() == 0 {
        add_unknown_port_to_cell(factory, to_coord(lx, ly), last_to)
      } else {
        add_two_ports_to_cell(factory, to_coord(lx, ly), last_from, last_to)
      };
    track.push(((lx, ly), bt, last_from, last_to)); // Note: first segment has undefined "from"

    lx = x;
    ly = y;
    last_from = new_from;
  }
  // Final step. Only has a from port.
  let bt = add_unknown_port_to_cell(factory, to_coord(lx, ly), last_from);
  track.push(((lx, ly), bt, last_from, last_from)); // Note: last segment has undefined "from"

  return track;
}
fn get_cells_from_a_to_b(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<(usize, usize)> {
  // https://playtechs.blogspot.com/2007/03/raytracing-on-grid.html
  // Super cover int algo, ported from:
  //
  // void raytrace(int x0, int y0, int x1, int y1)
  // {
  //   int dx = abs(x1 - x0);
  //   int dy = abs(y1 - y0);
  //   int x = x0;
  //   int y = y0;
  //   int n = 1 + dx + dy;
  //   int x_inc = (x1 > x0) ? 1 : -1;
  //   int y_inc = (y1 > y0) ? 1 : -1;
  //   int error = dx - dy;
  //   dx *= 2;
  //   dy *= 2;
  //
  //   for (; n > 0; --n)
  //   {
  //     visit(x, y);
  //
  //     if (error > 0)
  //     {
  //       x += x_inc;
  //       error -= dy;
  //     }
  //     else
  //     {
  //       y += y_inc;
  //       error += dx;
  //     }
  //   }
  // }

  let dx = (x1 - x0).abs();
  let dy = (y1 - y0).abs();
  let mut x = x0;
  let mut y = y0;
  let n = 1.0 + dx + dy;
  let x_inc = if x1 > x0 { 1.0 } else { -1.0 };
  let y_inc = if y1 > y0 { 1.0 } else { -1.0 };
  let mut error = dx - dy;

  let mut covered = vec!();
  for n in 0..n as u64 {
    covered.push((x as usize, y as usize));
    if error > 0.0 {
      x += x_inc;
      error -= dy;
    } else {
      y += y_inc;
      error += dx;
    }
  }

  return covered;
}

pub fn apply_action_between_two_cells(state: &State, options: &Options, config: &Config, factory: &mut Factory, add_or_remove: Action, cell_x1: usize, cell_y1: usize, belt_type1: BeltType, cell_x2: usize, cell_y2: usize, belt_type2: BeltType) {
  let coord1 = to_coord(cell_x1, cell_y1);
  let coord2 = to_coord(cell_x2, cell_y2);

  let dx = (cell_x2 as i8) - (cell_x1 as i8);
  let dy = (cell_y2 as i8) - (cell_y1 as i8);
  assert!((dx == 0) != (dy == 0), "one and only one of dx or dy is zero");
  assert!(dx >= -1 && dx <= 1 && dy >= -1 && dy <= 1, "since they are adjacent they must be -1, 0, or 1");

  if add_or_remove == Action::Add {
    log!(" - Connecting the two cells");

    // Convert empty cells to belt cells.
    // Create a port between these two cells, but none of the other cells.

    if is_edge(cell_x1 as f64, cell_y1 as f64) && is_edge(cell_x2 as f64, cell_y2 as f64) {
      // Noop. Just don't.
    }
    else {
      if factory.floor[coord1].kind == CellKind::Empty {
        if is_edge_not_corner(cell_x1 as f64, cell_y1 as f64) {
          // Cell is empty so place a trash supplier here as a placeholder
          factory.floor[coord1] = supply_cell(config, cell_x1, cell_y1, part_c(config, 't'), 2000, 0, 0);
        }
        else if is_middle(cell_x1 as f64, cell_y1 as f64) {
          factory.floor[coord1] = belt_cell(config, cell_x1, cell_y1, belt_type_to_belt_meta(belt_type1));
        }
      }
      if factory.floor[coord2].kind == CellKind::Empty {
        if is_edge_not_corner(cell_x2 as f64, cell_y2 as f64) {
          // Cell is empty so place a demander here
          factory.floor[coord2] = demand_cell(config, cell_x2, cell_y2, options.default_demand_speed, options.default_demand_cooldown);
        }
        else if is_middle(cell_x2 as f64, cell_y2 as f64) {
          factory.floor[coord2] = belt_cell(config, cell_x2, cell_y2, belt_type_to_belt_meta(belt_type2));
        }
      }

      cell_connect_if_possible(options, state, config, factory, coord1, coord2, dx, dy);
    }
  }
  else if add_or_remove == Action::Remove {
    log!(" - Disconnecting the two cells");

    // Delete the port between the two cells but leave everything else alone.
    // The coords must be adjacent to one side.

    let ( dir1, dir2) = match ( dx, dy ) {
      ( 0 , -1 ) => {
        // x1 was bigger so xy1 is under xy2
        (Direction::Up, Direction::Down)
      }
      ( 1 , 0 ) => {
        // x2 was bigger so xy1 is left of xy2
        (Direction::Right, Direction::Left)
      }
      ( 0 , 1 ) => {
        // y2 was bigger so xy1 is above xy2
        (Direction::Down, Direction::Up)
      }
      ( -1 , 0 ) => {
        // x1 was bigger so xy1 is right of xy2
        (Direction::Left, Direction::Right)
      }
      _ => panic!("already asserted the range of x and y"),
    };

    port_disconnect_cells(options, state, config, factory, coord1, dir1, coord2, dir2);
  }
  else {
    // Other mouse button or multi-button. ignore for now / ever.
    // (Remember: this was a drag of two cells)
    log!(" - Not left or right button; ignoring unknown button click");
  }

  fix_belt_meta(options, state, config, factory, coord1);
  fix_belt_meta(options, state, config, factory, coord2);

  if add_or_remove == Action::Remove {
    if factory.floor[coord1].kind == CellKind::Belt && factory.floor[coord1].port_u == Port::None && factory.floor[coord1].port_r == Port::None && factory.floor[coord1].port_d == Port::None && factory.floor[coord1].port_l == Port::None {
      floor_delete_cell_at_partial(options, state, config, factory, coord1);
    } else {
      clear_part_from_cell(options, state, config, factory, coord1);
    }
    if factory.floor[coord2].kind == CellKind::Belt && factory.floor[coord2].port_u == Port::None && factory.floor[coord2].port_r == Port::None && factory.floor[coord2].port_d == Port::None && factory.floor[coord2].port_l == Port::None {
      floor_delete_cell_at_partial(options, state, config, factory, coord2);
    } else {
      clear_part_from_cell(options, state, config, factory, coord2);
    }
  }

  factory.changed = true;
}
