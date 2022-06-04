use std::borrow::Borrow;
use std::convert::TryInto;

use super::belt::*;
use super::cell::*;
use super::demand::*;
use super::direction::*;
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
  let mut ems = 0;

  match floor[coord].direction_u {
    Port::Inbound => ins += 1,
    Port::Outbound => outs += 1,
    Port::Unknown => uns += 1,
    Port::None => ems += 1,
  };
  match floor[coord].direction_r {
    Port::Inbound => ins += 1,
    Port::Outbound => outs += 1,
    Port::Unknown => uns += 1,
    Port::None => ems += 1,
  };
  match floor[coord].direction_d {
    Port::Inbound => ins += 1,
    Port::Outbound => outs += 1,
    Port::Unknown => uns += 1,
    Port::None => ems += 1,
  };
  match floor[coord].direction_l {
    Port::Inbound => ins += 1,
    Port::Outbound => outs += 1,
    Port::Unknown => uns += 1,
    Port::None => ems += 1,
  };

  if uns == 0 {
    // No unknown ports. Nothing left to do.
    return (true, false); // No changes
  }

  if ins > 0 && outs == 0 && uns == 1 {
    // There is one unknown port and we already have at least one outbound port
    // so the unknown port must be inbound (or the config is broken)
    if floor[coord].direction_u == Port::Unknown {
      floor[coord].direction_u = Port::Outbound;
    } else if floor[coord].direction_r == Port::Unknown {
      floor[coord].direction_r = Port::Outbound;
    } else if floor[coord].direction_d == Port::Unknown {
      floor[coord].direction_d = Port::Outbound;
    } else if floor[coord].direction_l == Port::Unknown {
      floor[coord].direction_l = Port::Outbound;
    } else {
      panic!("should be an unknown port");
    }

    // No other ports need updating
    return (true, true);
  } else if outs > 0 && ins == 0 && uns == 1 {
    // There is one unknown port and we already have at least one inbound port
    // so the unknown port must be outbound (or the config is broken)

    if floor[coord].direction_u == Port::Unknown {
      floor[coord].direction_u = Port::Inbound;
    } else if floor[coord].direction_r == Port::Unknown {
      floor[coord].direction_r = Port::Inbound;
    } else if floor[coord].direction_d == Port::Unknown {
      floor[coord].direction_d = Port::Inbound;
    } else if floor[coord].direction_l == Port::Unknown {
      floor[coord].direction_l = Port::Inbound;
    } else {
      panic!("should be an unknown port");
    }

    // No other ports need updating
    return (true, true);
  }

  return (false, false);
}
fn auto_port_neighbor_u(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize) -> bool {
  if floor[coord].direction_u == Port::Unknown {
    let port_up: Port = match floor[coord].coord_u {
      Some(ocoord) => {
        match floor[ocoord].direction_d {
          Port::Inbound => Port::Outbound,
          Port::Outbound => Port::Inbound,
          Port::None => Port::None,
          Port::Unknown => Port::Unknown,
        }
      },
      None => Port::None,
    };
    if port_up != Port::Unknown {
      floor[coord].direction_u = port_up;
      return true;
    }
  }
  return false;
}
fn auto_port_neighbor_r(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize) -> bool {
  if floor[coord].direction_r == Port::Unknown {
    let port_right: Port = match floor[coord].coord_r {
      Some(ocoord) => {
        match floor[ocoord].direction_l {
          Port::Inbound => Port::Outbound,
          Port::Outbound => Port::Inbound,
          Port::None => Port::None,
          Port::Unknown => Port::Unknown,
        }
      },
      None => Port::None,
    };
    if port_right != Port::Unknown {
      floor[coord].direction_r = port_right;
      return true;
    }
  }
  return false;
}
fn auto_port_neighbor_d(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize) -> bool {
  if floor[coord].direction_d == Port::Unknown {
    let port_down: Port = match floor[coord].coord_d {
      Some(ocoord) => {
        match floor[ocoord].direction_u {
          Port::Inbound => Port::Outbound,
          Port::Outbound => Port::Inbound,
          Port::None => Port::None,
          Port::Unknown => Port::Unknown,
        }
      },
      None => Port::None,
    };
    if port_down != Port::Unknown {
      floor[coord].direction_d = port_down;
      return true;
    }
  }
  return false;
}
fn auto_port_neighbor_l(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize) -> bool {
  if floor[coord].direction_l == Port::Unknown {
    let port_left: Port = match floor[coord].coord_l {
      Some(ocoord) => {
        match floor[ocoord].direction_r {
          Port::Inbound => Port::Outbound,
          Port::Outbound => Port::Inbound,
          Port::None => Port::None,
          Port::Unknown => Port::Unknown,
        }
      },
      None => Port::None,
    };
    if port_left != Port::Unknown {
      floor[coord].direction_l = port_left;
      return true;
    }
  }
  return false;
}
fn auto_port_machine_u(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize, attempt: u32) -> bool {
  assert_eq!(floor[coord].kind, CellKind::Machine);

  if floor[coord].direction_u != Port::Unknown {
    return false;
  }

  let mut changed = false;

  let port_up: Port = match floor[coord].coord_u {
    Some(ocoord) => {
      if floor[ocoord].kind == CellKind::Machine {
        // Internal state; ignore ports to neighboring machine cells (even if they're not
        // part of the same machine block; we don't consider machine2machine transports)
        Port::None
      } else {
        match floor[ocoord].direction_d {
          Port::Inbound => Port::Outbound,
          Port::Outbound => Port::Inbound,
          Port::None => Port::None,
          Port::Unknown => Port::Unknown,
        }
      }
    },
    None => Port::None,
  };
  if port_up != Port::Unknown {
    floor[coord].direction_u = port_up;
    changed = true;
  }

  return changed;
}
fn auto_port_machine_r(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize, attempt: u32) -> bool {
  assert_eq!(floor[coord].kind, CellKind::Machine);

  if floor[coord].direction_r != Port::Unknown {
    return false;
  }

  let mut changed = false;

  let port_right: Port = match floor[coord].coord_r {
    Some(ocoord) => {
      if floor[ocoord].kind == CellKind::Machine {
        // Internal state; ignore ports to neighboring machine cells (even if they're not
        // part of the same machine block; we don't consider machine2machine transports)
        Port::None
      } else {
        match floor[ocoord].direction_l {
          Port::Inbound => Port::Outbound,
          Port::Outbound => Port::Inbound,
          Port::None => Port::None,
          Port::Unknown => Port::Unknown,
        }
      }
    },
    None => Port::None,
  };

  if port_right != Port::Unknown {
    floor[coord].direction_r = port_right;
    changed = true;
  }

  return changed;
}
fn auto_port_machine_d(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize, attempt: u32) -> bool {
  assert_eq!(floor[coord].kind, CellKind::Machine);

  if floor[coord].direction_d != Port::Unknown {
    return false;
  }

  let mut changed = false;

  let port_down: Port = match floor[coord].coord_d {
    Some(ocoord) => {
      if floor[ocoord].kind == CellKind::Machine {
        // Internal state; ignore ports to neighboring machine cells (even if they're not
        // part of the same machine block; we don't consider machine2machine transports)
        Port::None
      } else {
        match floor[ocoord].direction_u {
          Port::Inbound => Port::Outbound,
          Port::Outbound => Port::Inbound,
          Port::None => Port::None,
          Port::Unknown => Port::Unknown,
        }
      }
    },
    None => Port::None,
  };

  if port_down != Port::Unknown {
    floor[coord].direction_d = port_down;
    changed = true;
  }

  return changed;
}
fn auto_port_machine_l(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize, attempt: u32) -> bool {
  assert_eq!(floor[coord].kind, CellKind::Machine);

  if floor[coord].direction_l != Port::Unknown {
    return false;
  }

  let mut changed = false;

  let port_left: Port = match floor[coord].coord_l {
    Some(ocoord) => {
      if floor[ocoord].kind == CellKind::Machine {
        // Internal state; ignore ports to neighboring machine cells (even if they're not
        // part of the same machine block; we don't consider machine2machine transports)
        Port::None
      } else {
        match floor[ocoord].direction_r {
          Port::Inbound => Port::Outbound,
          Port::Outbound => Port::Inbound,
          Port::None => Port::None,
          Port::Unknown => Port::Unknown,
        }
      }
    },
    None => Port::None,
  };

  if port_left != Port::Unknown {
    floor[coord].direction_l = port_left;
    changed = true;
  }

  return changed;
}
fn auto_port_machine_neighbors(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize, attempt: u32) -> bool {
  assert_eq!(floor[coord].kind, CellKind::Machine);
  assert_eq!(floor[coord].machine.kind, MachineKind::Main);

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

    let (u1, u2, u3) = port_to_counts(floor[cur_coord].direction_u);
    let (r1, r2, r3) = port_to_counts(floor[cur_coord].direction_r);
    let (d1, d2, d3) = port_to_counts(floor[cur_coord].direction_d);
    let (l1, l2, l3) = port_to_counts(floor[cur_coord].direction_l);

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

    if floor[cur_coord].direction_u == Port::Unknown {
      floor[cur_coord].direction_u = new_port;
      return;
    }

    if floor[cur_coord].direction_r == Port::Unknown {
      floor[cur_coord].direction_r = new_port;
      return;
    }

    if floor[cur_coord].direction_d == Port::Unknown {
      floor[cur_coord].direction_d = new_port;
      return;
    }

    if floor[cur_coord].direction_l == Port::Unknown {
      floor[cur_coord].direction_l = new_port;
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

        if auto_port_neighbor_u(floor, coord) { changed = true; }
        if auto_port_neighbor_r(floor, coord) { changed = true; }
        if auto_port_neighbor_d(floor, coord) { changed = true; }
        if auto_port_neighbor_l(floor, coord) { changed = true; }
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
