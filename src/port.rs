use std::borrow::Borrow;
use std::convert::TryInto;

use super::belt::*;
use super::cell::*;
use super::demand::*;
use super::direction::*;
use super::factory::*;
use super::machine::*;
use super::options::*;
use super::part::*;
use super::state::*;
use super::supply::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Port {
  Inbound,
  Outbound,
  None,
  Unknown,
}

fn auto_port_cell_self(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize) -> (bool, bool) {
  // Returns (stop_coord, changed_coord)

  let mut ins = 0;
  let mut outs = 0;
  let mut uns = 0;
  // let mut ems = 0;

  match floor[coord].port_u {
    Port::Inbound => ins += 1,
    Port::Outbound => outs += 1,
    Port::Unknown => uns += 1,
    Port::None => (), // ems += 1,
  };
  match floor[coord].port_r {
    Port::Inbound => ins += 1,
    Port::Outbound => outs += 1,
    Port::Unknown => uns += 1,
    Port::None => (), // ems += 1,
  };
  match floor[coord].port_d {
    Port::Inbound => ins += 1,
    Port::Outbound => outs += 1,
    Port::Unknown => uns += 1,
    Port::None => (), // ems += 1,
  };
  match floor[coord].port_l {
    Port::Inbound => ins += 1,
    Port::Outbound => outs += 1,
    Port::Unknown => uns += 1,
    Port::None => (), // ems += 1,
  };

  if uns == 0 {
    // No unknown ports. Nothing left to do.
    return (true, false); // No changes
  }

  if ins > 0 && outs == 0 && uns == 1 {
    // There is one unknown port and we already have at least one outbound port
    // so the unknown port must be inbound (or the config is broken)
    if floor[coord].port_u == Port::Unknown {
      // println!("- belt @{}; up port must be outbound", coord);
      floor[coord].port_u = Port::Outbound;
      floor[coord].outs.push(( Direction::Up, coord, floor[coord].coord_u.unwrap(), Direction::Down ));
    } else if floor[coord].port_r == Port::Unknown {
      // println!("- belt @{}; right port must be outbound", coord);
      floor[coord].port_r = Port::Outbound;
      floor[coord].outs.push(( Direction::Right, coord, floor[coord].coord_r.unwrap(), Direction::Left ));
    } else if floor[coord].port_d == Port::Unknown {
      // println!("- belt @{}; down port must be outbound", coord);
      floor[coord].port_d = Port::Outbound;
      floor[coord].outs.push(( Direction::Down, coord, floor[coord].coord_d.unwrap(), Direction::Up ));
    } else if floor[coord].port_l == Port::Unknown {
      // println!("- belt @{}; left port must be outbound", coord);
      floor[coord].port_l = Port::Outbound;
      floor[coord].outs.push(( Direction::Left, coord, floor[coord].coord_l.unwrap(), Direction::Right ));
    } else {
      panic!("should be an unknown port");
    }

    // No other ports need updating
    return (true, true);
  } else if outs > 0 && ins == 0 && uns == 1 {
    // There is one unknown port and we already have at least one inbound port
    // so the unknown port must be outbound (or the config is broken)

    if floor[coord].port_u == Port::Unknown {
      // println!("- belt @{}; up port must be inbound", coord);
      floor[coord].port_u = Port::Inbound;
      floor[coord].ins.push(( Direction::Up, coord, floor[coord].coord_u.unwrap(), Direction::Down ));
    } else if floor[coord].port_r == Port::Unknown {
      // println!("- belt @{}; right port must be inbound", coord);
      floor[coord].port_r = Port::Inbound;
      floor[coord].ins.push(( Direction::Right, coord, floor[coord].coord_r.unwrap(), Direction::Left ));
    } else if floor[coord].port_d == Port::Unknown {
      // println!("- belt @{}; down port must be inbound", coord);
      floor[coord].port_d = Port::Inbound;
      floor[coord].ins.push(( Direction::Down, coord, floor[coord].coord_d.unwrap(), Direction::Up ));
    } else if floor[coord].port_l == Port::Unknown {
      // println!("- belt @{}; left port must be inbound", coord);
      floor[coord].port_l = Port::Inbound;
      floor[coord].ins.push(( Direction::Left, coord, floor[coord].coord_l.unwrap(), Direction::Right ));
    } else {
      panic!("should be an unknown port");
    }

    // No other ports need updating
    return (true, true);
  }

  return (false, false);
}
fn auto_port_belt_u(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize) -> bool {
  if floor[coord].port_u != Port::Unknown {
    return false;
  }

  let port: Port = match floor[coord].coord_u {
    Some(ocoord) => {
      match floor[ocoord].port_d {
        Port::Inbound => Port::Outbound,
        Port::Outbound => Port::Inbound,
        Port::None => Port::None,
        Port::Unknown => Port::Unknown,
      }
    },
    None => Port::None,
  };

  match port {
    Port::Inbound => {
      // println!("- belt @{}; up port is inbound", coord);
      floor[coord].port_u = Port::Inbound;
      floor[coord].ins.push(( Direction::Up, coord, floor[coord].coord_u.unwrap(), Direction::Down ));
    }
    Port::Outbound => {
      // println!("- belt @{}; up port is outbound", coord);
      floor[coord].port_u = Port::Outbound;
      floor[coord].outs.push(( Direction::Up, coord, floor[coord].coord_u.unwrap(), Direction::Down ));
    }
    Port::None => {
      floor[coord].port_u = Port::None;
    }
    Port::Unknown => {
      return false;
    }
  }
  return true;
}
fn auto_port_belt_r(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize) -> bool {
  if floor[coord].port_r != Port::Unknown {
    return false;
  }

  let port: Port = match floor[coord].coord_r {
    Some(ocoord) => {
      match floor[ocoord].port_l {
        Port::Inbound => Port::Outbound,
        Port::Outbound => Port::Inbound,
        Port::None => Port::None,
        Port::Unknown => Port::Unknown,
      }
    },
    None => Port::None,
  };

  match port {
    Port::Inbound => {
      // println!("- belt @{}; right port is inbound", coord);
      floor[coord].port_r = Port::Inbound;
      floor[coord].ins.push(( Direction::Right, coord, floor[coord].coord_r.unwrap(), Direction::Left ));
    }
    Port::Outbound => {
      // println!("- belt @{}; right port is outbound", coord);
      floor[coord].port_r = Port::Outbound;
      floor[coord].outs.push(( Direction::Right, coord, floor[coord].coord_r.unwrap(), Direction::Left ));
    }
    Port::None => {
      floor[coord].port_r = Port::None;
    }
    Port::Unknown => {
      return false;
    }
  }
  return true;
}
fn auto_port_belt_d(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize) -> bool {
  if floor[coord].port_d != Port::Unknown {
    return false;
  }

  let port: Port = match floor[coord].coord_d {
    Some(ocoord) => {
      match floor[ocoord].port_u {
        Port::Inbound => Port::Outbound,
        Port::Outbound => Port::Inbound,
        Port::None => Port::None,
        Port::Unknown => Port::Unknown,
      }
    },
    None => Port::None,
  };

  match port {
    Port::Inbound => {
      // println!("- belt @{}; down port is inbound", coord);
      floor[coord].port_d = Port::Inbound;
      floor[coord].ins.push(( Direction::Down, coord, floor[coord].coord_d.unwrap(), Direction::Up ));
    }
    Port::Outbound => {
      // println!("- belt @{}; down port is outbound", coord);
      floor[coord].port_d = Port::Outbound;
      floor[coord].outs.push(( Direction::Down, coord, floor[coord].coord_d.unwrap(), Direction::Up ));
    }
    Port::None => {
      floor[coord].port_d = Port::None;
    }
    Port::Unknown => {
      return false;
    }
  }
  return true;
}
fn auto_port_belt_l(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize) -> bool {
  if floor[coord].port_l != Port::Unknown {
    return false;
  }

  let port: Port = match floor[coord].coord_l {
    Some(ocoord) => {
      match floor[ocoord].port_r {
        Port::Inbound => Port::Outbound,
        Port::Outbound => Port::Inbound,
        Port::None => Port::None,
        Port::Unknown => Port::Unknown,
      }
    },
    None => Port::None,
  };

  match port {
    Port::Inbound => {
      // println!("- belt @{}; left port is inbound", coord);
      floor[coord].port_l = Port::Inbound;
      floor[coord].ins.push(( Direction::Left, coord, floor[coord].coord_l.unwrap(), Direction::Right ));
    }
    Port::Outbound => {
      // println!("- belt @{}; left port is outbound", coord);
      floor[coord].port_l = Port::Outbound;
      floor[coord].outs.push(( Direction::Left, coord, floor[coord].coord_l.unwrap(), Direction::Right ));
    }
    Port::None => {
      floor[coord].port_l = Port::None;
    }
    Port::Unknown => {
      return false;
    }
  }
  return true;
}
fn auto_port_machine_u(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize, attempt: u32) -> bool {
  assert_eq!(floor[coord].kind, CellKind::Machine);

  if floor[coord].port_u != Port::Unknown {
    return false;
  }

  let port: Port = match floor[coord].coord_u {
    Some(ocoord) => {
      if floor[ocoord].kind == CellKind::Machine {
        // Internal state; ignore ports to neighboring machine cells (even if they're not
        // part of the same machine block; we don't consider machine2machine transports)
        Port::None
      } else {
        match floor[ocoord].port_d {
          Port::Inbound => Port::Outbound,
          Port::Outbound => Port::Inbound,
          Port::None => Port::None,
          Port::Unknown => Port::Unknown,
        }
      }
    },
    None => Port::None,
  };

  match port {
    Port::Inbound => {
      println!("  - machine @{} up port is inbound", coord);
      floor[coord].port_u = Port::Inbound;
      floor[floor[coord].machine.main_coord].ins.push(( Direction::Up, coord, floor[coord].coord_u.unwrap(), Direction::Down ));
    }
    Port::Outbound => {
      println!("  - machine @{} up port is outbound", coord);
      floor[coord].port_u = Port::Outbound;
      floor[floor[coord].machine.main_coord].outs.push(( Direction::Up, coord, floor[coord].coord_u.unwrap(), Direction::Down ));
    }
    Port::None => {
      floor[coord].port_u = Port::None;
    }
    Port::Unknown => {
      return false;
    }
  }
  return true;
}
fn auto_port_machine_r(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize, attempt: u32) -> bool {
  assert_eq!(floor[coord].kind, CellKind::Machine);

  if floor[coord].port_r != Port::Unknown {
    return false;
  }

  let port: Port = match floor[coord].coord_r {
    Some(ocoord) => {
      if floor[ocoord].kind == CellKind::Machine {
        // Internal state; ignore ports to neighboring machine cells (even if they're not
        // part of the same machine block; we don't consider machine2machine transports)
        Port::None
      } else {
        match floor[ocoord].port_l {
          Port::Inbound => Port::Outbound,
          Port::Outbound => Port::Inbound,
          Port::None => Port::None,
          Port::Unknown => Port::Unknown,
        }
      }
    },
    None => Port::None,
  };

  match port {
    Port::Inbound => {
      println!("  - machine @{} right port is inbound", coord);
      floor[coord].port_r = Port::Inbound;
      floor[floor[coord].machine.main_coord].ins.push(( Direction::Right, coord, floor[coord].coord_r.unwrap(), Direction::Left ));
    }
    Port::Outbound => {
      println!("  - machine @{} right port is outbound", coord);
      floor[coord].port_r = Port::Outbound;
      floor[floor[coord].machine.main_coord].outs.push(( Direction::Right, coord, floor[coord].coord_r.unwrap(), Direction::Left ));
    }
    Port::None => {
      floor[coord].port_r = Port::None;
    }
    Port::Unknown => {
      return false;
    }
  }
  return true;
}
fn auto_port_machine_d(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize, attempt: u32) -> bool {
  assert_eq!(floor[coord].kind, CellKind::Machine);

  if floor[coord].port_d != Port::Unknown {
    return false;
  }

  let port: Port = match floor[coord].coord_d {
    Some(ocoord) => {
      if floor[ocoord].kind == CellKind::Machine {
        // Internal state; ignore ports to neighboring machine cells (even if they're not
        // part of the same machine block; we don't consider machine2machine transports)
        Port::None
      } else {
        match floor[ocoord].port_u {
          Port::Inbound => Port::Outbound,
          Port::Outbound => Port::Inbound,
          Port::None => Port::None,
          Port::Unknown => Port::Unknown,
        }
      }
    },
    None => Port::None,
  };

  match port {
    Port::Inbound => {
      println!("  - machine @{}; down port is inbound", coord);
      floor[coord].port_d = Port::Inbound;
      floor[floor[coord].machine.main_coord].ins.push(( Direction::Down, coord, floor[coord].coord_d.unwrap(), Direction::Up ));
    }
    Port::Outbound => {
      println!("  - machine @{}; down port is outbound", coord);
      floor[coord].port_d = Port::Outbound;
      floor[floor[coord].machine.main_coord].outs.push(( Direction::Down, coord, floor[coord].coord_d.unwrap(), Direction::Up ));
    }
    Port::None => {
      floor[coord].port_d = Port::None;
    }
    Port::Unknown => {
      return false;
    }
  }
  return true;
}
fn auto_port_machine_l(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize, attempt: u32) -> bool {
  assert_eq!(floor[coord].kind, CellKind::Machine);

  if floor[coord].port_l != Port::Unknown {
    return false;
  }

  let port: Port = match floor[coord].coord_l {
    Some(ocoord) => {
      if floor[ocoord].kind == CellKind::Machine {
        // Internal state; ignore ports to neighboring machine cells (even if they're not
        // part of the same machine block; we don't consider machine2machine transports)
        Port::None
      } else {
        match floor[ocoord].port_r {
          Port::Inbound => Port::Outbound,
          Port::Outbound => Port::Inbound,
          Port::None => Port::None,
          Port::Unknown => Port::Unknown,
        }
      }
    },
    None => Port::None,
  };

  match port {
    Port::Inbound => {
      println!("  - machine @{} left port is inbound", coord);
      floor[coord].port_l = Port::Inbound;
      floor[floor[coord].machine.main_coord].ins.push(( Direction::Left, coord, floor[coord].coord_l.unwrap(), Direction::Right ));
    }
    Port::Outbound => {
      println!("  - machine @{} left port is outbound", coord);
      floor[coord].port_l = Port::Outbound;
      floor[floor[coord].machine.main_coord].outs.push(( Direction::Left, coord, floor[coord].coord_l.unwrap(), Direction::Right ));
    }
    Port::None => {
      floor[coord].port_l = Port::None;
    }
    Port::Unknown => {
      return false;
    }
  }
  return true;
}
fn auto_port_machine_neighbors(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize, attempt: u32) -> bool {
  assert_eq!(floor[coord].kind, CellKind::Machine);
  assert_eq!(floor[coord].machine.kind, MachineKind::Main);

  println!("- auto_port_machine_neighbors({}, {}): {:?}", coord, attempt, floor[coord].machine.coords);
  
  let mut changed = false;
  // for cur_coord in floor[coord].machine.coords {
  for i in 0..floor[coord].machine.coords.len() {
    let cur_coord= floor[coord].machine.coords[i];
    if auto_port_machine_u(floor, cur_coord, attempt) { changed = true; };
    if auto_port_machine_r(floor, cur_coord, attempt) { changed = true; };
    if auto_port_machine_d(floor, cur_coord, attempt) { changed = true; };
    if auto_port_machine_l(floor, cur_coord, attempt) { changed = true; };
  }

  return changed;
}
fn port_to_counts(port: Port) -> (u16, u16, u16) {
  match port {
    Port::Inbound => (1, 0, 0),
    Port::Outbound => (0, 1, 0),
    Port::None =>  (0, 0, 0),
    Port::Unknown =>  (0, 0, 1),
  }
}
fn auto_port_discover_machine_ports(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize, attempt: u32) -> (u16, u16, u16) {
  assert_eq!(floor[coord].machine.kind, MachineKind::Main);

  // Given the main machine cell, count the number of inbound, outbound, and undefined ports for all
  // machine cells that are part of the same machine.

  let mut ins = 0;
  let mut ous = 0;
  let mut uns = 0;

  // for cur_coord in floor[coord].machine.coords {
  for i in 0..floor[coord].machine.coords.len() {
    let cur_coord= floor[coord].machine.coords[i];

    let (u1, u2, u3) = port_to_counts(floor[cur_coord].port_u);
    let (r1, r2, r3) = port_to_counts(floor[cur_coord].port_r);
    let (d1, d2, d3) = port_to_counts(floor[cur_coord].port_d);
    let (l1, l2, l3) = port_to_counts(floor[cur_coord].port_l);

    ins += u1 + r1 + d1 + l1;
    ous += u2 + r2 + d2 + l2;
    uns += u3 + r3 + d3 + l3;
  }

  return ( ins, ous, uns );
}
fn auto_port_convert_machine_unknown_to(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize, new_port: Port, attempt: u32) {
  // Given a machine cell, find a port that is unknown and change it to the given port type.
  // Stop as soon as you find one. There should only be one such port, anyways.

  // for cur_coord in floor[coord].machine.coords {
  for i in 0..floor[coord].machine.coords.len() {
    let cur_coord= floor[coord].machine.coords[i]; // Is this cheaper than the alt?

    if floor[cur_coord].port_u == Port::Unknown {
      println!("  - machine @{}; up port is {:?}", cur_coord, new_port);
      floor[cur_coord].port_u = new_port;
      if new_port == Port::Inbound {
        floor[floor[cur_coord].machine.main_coord].ins.push(( Direction::Up, cur_coord, floor[coord].coord_u.unwrap(), Direction::Down ));
      } else {
        floor[floor[cur_coord].machine.main_coord].outs.push(( Direction::Up, cur_coord, floor[coord].coord_u.unwrap(), Direction::Down ));
      }
      return;
    }

    if floor[cur_coord].port_r == Port::Unknown {
      println!("  - machine @{}; right port is {:?}", cur_coord, new_port);
      floor[cur_coord].port_r = new_port;
      if new_port == Port::Inbound {
        floor[floor[cur_coord].machine.main_coord].ins.push(( Direction::Right, cur_coord, floor[coord].coord_r.unwrap(), Direction::Left ));
      } else {
        floor[floor[cur_coord].machine.main_coord].outs.push(( Direction::Right, cur_coord, floor[coord].coord_r.unwrap(), Direction::Left ));
      }
      return;
    }

    if floor[cur_coord].port_d == Port::Unknown {
      println!("  - machine @{}; down port is {:?}", cur_coord, new_port);
      floor[cur_coord].port_d = new_port;
      if new_port == Port::Inbound {
        floor[floor[cur_coord].machine.main_coord].ins.push(( Direction::Down, cur_coord, floor[coord].coord_d.unwrap(), Direction::Up ));
      } else {
        floor[floor[cur_coord].machine.main_coord].outs.push(( Direction::Down, cur_coord, floor[coord].coord_d.unwrap(), Direction::Up ));
      }
      return;
    }

    if floor[cur_coord].port_l == Port::Unknown {
      println!("  - machine @{}; left port is {:?}", cur_coord, new_port);
      floor[cur_coord].port_l = new_port;
      if new_port == Port::Inbound {
        floor[floor[cur_coord].machine.main_coord].ins.push(( Direction::Left, cur_coord, floor[coord].coord_l.unwrap(), Direction::Right ));
      } else {
        floor[floor[cur_coord].machine.main_coord].outs.push(( Direction::Left, cur_coord, floor[coord].coord_l.unwrap(), Direction::Right ));
      }
      return;
    }
  }

  panic!("should find at least (and most) one unknown port in this machine...");
}

pub fn auto_port(floor: &mut [Cell; FLOOR_CELLS_WH], attempt: u32) -> bool {
  assert!(attempt > 0, "attempt must be non-zero because it gets deducted");
  println!("auto_port({})", attempt);
  let mut changed = false;
  for coord in 0..FLOOR_CELLS_WH {
    match floor[coord].kind {
      CellKind::Empty => {
        // could go ahead and mark all neighbor belts as none ports as well...
      }
      CellKind::Belt => {
        let (next, changed_now) = auto_port_cell_self(floor, coord);
        if changed_now { changed = true; }
        if next { continue; }

        if auto_port_belt_u(floor, coord) { changed = true; }
        if auto_port_belt_r(floor, coord) { changed = true; }
        if auto_port_belt_d(floor, coord) { changed = true; }
        if auto_port_belt_l(floor, coord) { changed = true; }
      }
      CellKind::Machine => {
        // Machines can cover multiple cells, have a main cell and sub cells (-> main.machine.subs)
        // Consider only the main a machine block, ignore the subs
        // The cells are iterated over up to three times per coord iteration (should be no big deal)

        if floor[coord].machine.kind == MachineKind::Main {
          if auto_port_machine_neighbors(floor, coord, attempt) {
            changed = true;
          }

          // Change the attempt so we can reuse the value.
          let ( ins, outs, uns ) = auto_port_discover_machine_ports(floor, coord, attempt - 1);
          if ins == 0 && outs > 0 && uns == 1 {
            // Find the undetermined port and turn it to an Inbound port
            auto_port_convert_machine_unknown_to(floor, coord, Port::Inbound, attempt);
          } else if outs == 0 && ins > 0 && uns == 1 {
            // Find the undetermined port and turn it to an Outbound port
            auto_port_convert_machine_unknown_to(floor, coord, Port::Outbound, attempt);
          } else {
            // At this step not able to deduce any ports for this machine
          }
        }
      }
      CellKind::Supply => {
        // noop
      }
      CellKind::Demand => {
        // noop
      }
    }
  }

  return changed;
}

pub fn port_both_sides_ud(factory: &mut Factory, coord: usize) {
  if factory.floor[coord].port_u == Port::None {
    factory.floor[coord].port_u = Port::Unknown;
  }
  if let Some(coord) = factory.floor[coord].coord_u {
    if factory.floor[coord].kind == CellKind::Belt && factory.floor[coord].port_d == Port::None {
      factory.floor[coord].port_d = Port::Unknown;
    }
  }
}
pub fn port_both_sides_rl(factory: &mut Factory, coord: usize) {
  if factory.floor[coord].port_r == Port::None {
    factory.floor[coord].port_r = Port::Unknown;
  }
  if let Some(coord) = factory.floor[coord].coord_r {
    if factory.floor[coord].kind == CellKind::Belt && factory.floor[coord].port_l == Port::None {
      factory.floor[coord].port_l = Port::Unknown;
    }
  }
}
pub fn port_both_sides_du(factory: &mut Factory, coord: usize) {
  if factory.floor[coord].port_d == Port::None {
    factory.floor[coord].port_d = Port::Unknown;
  }
  if let Some(coord) = factory.floor[coord].coord_d {
    if factory.floor[coord].kind == CellKind::Belt && factory.floor[coord].port_u == Port::None {
      factory.floor[coord].port_u = Port::Unknown;
    }
  }
}
pub fn port_both_sides_lr(factory: &mut Factory, coord: usize) {
  if factory.floor[coord].port_l == Port::None {
    factory.floor[coord].port_l = Port::Unknown;
  }
  if let Some(coord) = factory.floor[coord].coord_l {
    if factory.floor[coord].kind == CellKind::Belt && factory.floor[coord].port_r == Port::None {
      factory.floor[coord].port_r = Port::Unknown;
    }
  }
}
