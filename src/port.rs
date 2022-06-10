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
use super::utils::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Port {
  Inbound,
  Outbound,
  None,
  Unknown,
}

fn auto_port_cell_self(factory: &mut Factory, coord: usize) -> (bool, bool) {
  // Returns (stop_coord, changed_coord)

  let mut ins = 0;
  let mut outs = 0;
  let mut uns = 0;
  // let mut ems = 0;

  match factory.floor[coord].port_u {
    Port::Inbound => ins += 1,
    Port::Outbound => outs += 1,
    Port::Unknown => uns += 1,
    Port::None => (), // ems += 1,
  };
  match factory.floor[coord].port_r {
    Port::Inbound => ins += 1,
    Port::Outbound => outs += 1,
    Port::Unknown => uns += 1,
    Port::None => (), // ems += 1,
  };
  match factory.floor[coord].port_d {
    Port::Inbound => ins += 1,
    Port::Outbound => outs += 1,
    Port::Unknown => uns += 1,
    Port::None => (), // ems += 1,
  };
  match factory.floor[coord].port_l {
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
    if factory.floor[coord].port_u == Port::Unknown {
      // log(format!("- belt @{}; up port must be outbound", coord));
      factory.floor[coord].port_u = Port::Outbound;
      factory.floor[coord].outs.push(( Direction::Up, coord, factory.floor[coord].coord_u.unwrap(), Direction::Down ));
    } else if factory.floor[coord].port_r == Port::Unknown {
      // log(format!("- belt @{}; right port must be outbound", coord));
      factory.floor[coord].port_r = Port::Outbound;
      factory.floor[coord].outs.push(( Direction::Right, coord, factory.floor[coord].coord_r.unwrap(), Direction::Left ));
    } else if factory.floor[coord].port_d == Port::Unknown {
      // log(format!("- belt @{}; down port must be outbound", coord));
      factory.floor[coord].port_d = Port::Outbound;
      factory.floor[coord].outs.push(( Direction::Down, coord, factory.floor[coord].coord_d.unwrap(), Direction::Up ));
    } else if factory.floor[coord].port_l == Port::Unknown {
      // log(format!("- belt @{}; left port must be outbound", coord));
      factory.floor[coord].port_l = Port::Outbound;
      factory.floor[coord].outs.push(( Direction::Left, coord, factory.floor[coord].coord_l.unwrap(), Direction::Right ));
    } else {
      panic!("should be an unknown port");
    }

    // No other ports need updating
    return (true, true);
  } else if outs > 0 && ins == 0 && uns == 1 {
    // There is one unknown port and we already have at least one inbound port
    // so the unknown port must be outbound (or the config is broken)

    if factory.floor[coord].port_u == Port::Unknown {
      // log(format!("- belt @{}; up port must be inbound", coord));
      factory.floor[coord].port_u = Port::Inbound;
      factory.floor[coord].ins.push(( Direction::Up, coord, factory.floor[coord].coord_u.unwrap(), Direction::Down ));
    } else if factory.floor[coord].port_r == Port::Unknown {
      // log(format!("- belt @{}; right port must be inbound", coord));
      factory.floor[coord].port_r = Port::Inbound;
      factory.floor[coord].ins.push(( Direction::Right, coord, factory.floor[coord].coord_r.unwrap(), Direction::Left ));
    } else if factory.floor[coord].port_d == Port::Unknown {
      // log(format!("- belt @{}; down port must be inbound", coord));
      factory.floor[coord].port_d = Port::Inbound;
      factory.floor[coord].ins.push(( Direction::Down, coord, factory.floor[coord].coord_d.unwrap(), Direction::Up ));
    } else if factory.floor[coord].port_l == Port::Unknown {
      // log(format!("- belt @{}; left port must be inbound", coord));
      factory.floor[coord].port_l = Port::Inbound;
      factory.floor[coord].ins.push(( Direction::Left, coord, factory.floor[coord].coord_l.unwrap(), Direction::Right ));
    } else {
      panic!("should be an unknown port");
    }

    // No other ports need updating
    return (true, true);
  }

  return (false, false);
}
fn auto_port_belt_u(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) -> bool {
  if factory.floor[coord].port_u != Port::Unknown {
    return false;
  }

  let port: Port = match factory.floor[coord].coord_u {
    Some(ocoord) => {
      match factory.floor[ocoord].port_d {
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
      // log(format!("- belt @{}; up port is inbound", coord));
      factory.floor[coord].port_u = Port::Inbound;
      factory.floor[coord].ins.push(( Direction::Up, coord, factory.floor[coord].coord_u.unwrap(), Direction::Down ));
    }
    Port::Outbound => {
      // log(format!("- belt @{}; up port is outbound", coord));
      factory.floor[coord].port_u = Port::Outbound;
      factory.floor[coord].outs.push(( Direction::Up, coord, factory.floor[coord].coord_u.unwrap(), Direction::Down ));
    }
    Port::None => {
      factory.floor[coord].port_u = Port::None;
    }
    Port::Unknown => {
      return false;
    }
  }
  return true;
}
fn auto_port_belt_r(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) -> bool {
  if factory.floor[coord].port_r != Port::Unknown {
    return false;
  }

  let port: Port = match factory.floor[coord].coord_r {
    Some(ocoord) => {
      match factory.floor[ocoord].port_l {
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
      // log(format!("- belt @{}; right port is inbound", coord));
      factory.floor[coord].port_r = Port::Inbound;
      factory.floor[coord].ins.push(( Direction::Right, coord, factory.floor[coord].coord_r.unwrap(), Direction::Left ));
    }
    Port::Outbound => {
      // log(format!("- belt @{}; right port is outbound", coord));
      factory.floor[coord].port_r = Port::Outbound;
      factory.floor[coord].outs.push(( Direction::Right, coord, factory.floor[coord].coord_r.unwrap(), Direction::Left ));
    }
    Port::None => {
      factory.floor[coord].port_r = Port::None;
    }
    Port::Unknown => {
      return false;
    }
  }
  return true;
}
fn auto_port_belt_d(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) -> bool {
  if factory.floor[coord].port_d != Port::Unknown {
    return false;
  }

  let port: Port = match factory.floor[coord].coord_d {
    Some(ocoord) => {
      match factory.floor[ocoord].port_u {
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
      // log(format!("- belt @{}; down port is inbound", coord));
      factory.floor[coord].port_d = Port::Inbound;
      factory.floor[coord].ins.push(( Direction::Down, coord, factory.floor[coord].coord_d.unwrap(), Direction::Up ));
    }
    Port::Outbound => {
      // log(format!("- belt @{}; down port is outbound", coord));
      factory.floor[coord].port_d = Port::Outbound;
      factory.floor[coord].outs.push(( Direction::Down, coord, factory.floor[coord].coord_d.unwrap(), Direction::Up ));
    }
    Port::None => {
      factory.floor[coord].port_d = Port::None;
    }
    Port::Unknown => {
      return false;
    }
  }
  return true;
}
fn auto_port_belt_l(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) -> bool {
  if factory.floor[coord].port_l != Port::Unknown {
    return false;
  }

  let port: Port = match factory.floor[coord].coord_l {
    Some(ocoord) => {
      match factory.floor[ocoord].port_r {
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
      // log(format!("- belt @{}; left port is inbound", coord));
      factory.floor[coord].port_l = Port::Inbound;
      factory.floor[coord].ins.push(( Direction::Left, coord, factory.floor[coord].coord_l.unwrap(), Direction::Right ));
    }
    Port::Outbound => {
      // log(format!("- belt @{}; left port is outbound", coord));
      factory.floor[coord].port_l = Port::Outbound;
      factory.floor[coord].outs.push(( Direction::Left, coord, factory.floor[coord].coord_l.unwrap(), Direction::Right ));
    }
    Port::None => {
      factory.floor[coord].port_l = Port::None;
    }
    Port::Unknown => {
      return false;
    }
  }
  return true;
}
fn auto_port_machine_u(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize, attempt: u32) -> bool {
  assert_eq!(factory.floor[coord].kind, CellKind::Machine);

  if factory.floor[coord].port_u != Port::Unknown {
    return false;
  }

  let port: Port = match factory.floor[coord].coord_u {
    Some(ocoord) => {
      if factory.floor[ocoord].kind == CellKind::Machine {
        // Internal state; ignore ports to neighboring machine cells (even if they're not
        // part of the same machine block; we don't consider machine2machine transports)
        Port::None
      } else {
        match factory.floor[ocoord].port_d {
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
      if options.trace_priority_step { log(format!("  - machine @{} up port is inbound", coord)); }
      factory.floor[coord].port_u = Port::Inbound;
      factory.floor[factory.floor[coord].machine.main_coord].ins.push(( Direction::Up, coord, factory.floor[coord].coord_u.unwrap(), Direction::Down ));
    }
    Port::Outbound => {
      if options.trace_priority_step { log(format!("  - machine @{} up port is outbound", coord)); }
      factory.floor[coord].port_u = Port::Outbound;
      factory.floor[factory.floor[coord].machine.main_coord].outs.push(( Direction::Up, coord, factory.floor[coord].coord_u.unwrap(), Direction::Down ));
    }
    Port::None => {
      factory.floor[coord].port_u = Port::None;
    }
    Port::Unknown => {
      return false;
    }
  }
  return true;
}
fn auto_port_machine_r(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize, attempt: u32) -> bool {
  assert_eq!(factory.floor[coord].kind, CellKind::Machine);

  if factory.floor[coord].port_r != Port::Unknown {
    return false;
  }

  let port: Port = match factory.floor[coord].coord_r {
    Some(ocoord) => {
      if factory.floor[ocoord].kind == CellKind::Machine {
        // Internal state; ignore ports to neighboring machine cells (even if they're not
        // part of the same machine block; we don't consider machine2machine transports)
        Port::None
      } else {
        match factory.floor[ocoord].port_l {
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
      if options.trace_priority_step { log(format!("  - machine @{} right port is inbound", coord)); }
      factory.floor[coord].port_r = Port::Inbound;
      factory.floor[factory.floor[coord].machine.main_coord].ins.push(( Direction::Right, coord, factory.floor[coord].coord_r.unwrap(), Direction::Left ));
    }
    Port::Outbound => {
      if options.trace_priority_step { log(format!("  - machine @{} right port is outbound", coord)); }
      factory.floor[coord].port_r = Port::Outbound;
      factory.floor[factory.floor[coord].machine.main_coord].outs.push(( Direction::Right, coord, factory.floor[coord].coord_r.unwrap(), Direction::Left ));
    }
    Port::None => {
      factory.floor[coord].port_r = Port::None;
    }
    Port::Unknown => {
      return false;
    }
  }
  return true;
}
fn auto_port_machine_d(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize, attempt: u32) -> bool {
  assert_eq!(factory.floor[coord].kind, CellKind::Machine);

  if factory.floor[coord].port_d != Port::Unknown {
    return false;
  }

  let port: Port = match factory.floor[coord].coord_d {
    Some(ocoord) => {
      if factory.floor[ocoord].kind == CellKind::Machine {
        // Internal state; ignore ports to neighboring machine cells (even if they're not
        // part of the same machine block; we don't consider machine2machine transports)
        Port::None
      } else {
        match factory.floor[ocoord].port_u {
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
      if options.trace_priority_step { log(format!("  - machine @{}; down port is inbound", coord)); }
      factory.floor[coord].port_d = Port::Inbound;
      factory.floor[factory.floor[coord].machine.main_coord].ins.push(( Direction::Down, coord, factory.floor[coord].coord_d.unwrap(), Direction::Up ));
    }
    Port::Outbound => {
      if options.trace_priority_step { log(format!("  - machine @{}; down port is outbound", coord)); }
      factory.floor[coord].port_d = Port::Outbound;
      factory.floor[factory.floor[coord].machine.main_coord].outs.push(( Direction::Down, coord, factory.floor[coord].coord_d.unwrap(), Direction::Up ));
    }
    Port::None => {
      factory.floor[coord].port_d = Port::None;
    }
    Port::Unknown => {
      return false;
    }
  }
  return true;
}
fn auto_port_machine_l(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize, attempt: u32) -> bool {
  assert_eq!(factory.floor[coord].kind, CellKind::Machine);

  if factory.floor[coord].port_l != Port::Unknown {
    return false;
  }

  let port: Port = match factory.floor[coord].coord_l {
    Some(ocoord) => {
      if factory.floor[ocoord].kind == CellKind::Machine {
        // Internal state; ignore ports to neighboring machine cells (even if they're not
        // part of the same machine block; we don't consider machine2machine transports)
        Port::None
      } else {
        match factory.floor[ocoord].port_r {
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
      if options.trace_priority_step { log(format!("  - machine @{} left port is inbound", coord)); }
      factory.floor[coord].port_l = Port::Inbound;
      factory.floor[factory.floor[coord].machine.main_coord].ins.push(( Direction::Left, coord, factory.floor[coord].coord_l.unwrap(), Direction::Right ));
    }
    Port::Outbound => {
      if options.trace_priority_step { log(format!("  - machine @{} left port is outbound", coord)); }
      factory.floor[coord].port_l = Port::Outbound;
      factory.floor[factory.floor[coord].machine.main_coord].outs.push(( Direction::Left, coord, factory.floor[coord].coord_l.unwrap(), Direction::Right ));
    }
    Port::None => {
      factory.floor[coord].port_l = Port::None;
    }
    Port::Unknown => {
      return false;
    }
  }
  return true;
}
fn auto_port_machine_neighbors(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize, attempt: u32) -> bool {
  assert_eq!(factory.floor[coord].kind, CellKind::Machine);
  assert_eq!(factory.floor[coord].machine.kind, MachineKind::Main);

  if options.trace_priority_step { log(format!("- auto_port_machine_neighbors({}, {}): {:?}", coord, attempt, factory.floor[coord].machine.coords)); }
  
  let mut changed = false;
  // for cur_coord in floor[coord].machine.coords {
  for i in 0..factory.floor[coord].machine.coords.len() {
    let cur_coord= factory.floor[coord].machine.coords[i];
    if auto_port_machine_u(options, state, factory, cur_coord, attempt) { changed = true; };
    if auto_port_machine_r(options, state, factory, cur_coord, attempt) { changed = true; };
    if auto_port_machine_d(options, state, factory, cur_coord, attempt) { changed = true; };
    if auto_port_machine_l(options, state, factory, cur_coord, attempt) { changed = true; };
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
fn auto_port_discover_machine_ports(factory: &mut Factory, coord: usize, attempt: u32) -> (u16, u16, u16) {
  assert_eq!(factory.floor[coord].machine.kind, MachineKind::Main);

  // Given the main machine cell, count the number of inbound, outbound, and undefined ports for all
  // machine cells that are part of the same machine.

  let mut ins = 0;
  let mut ous = 0;
  let mut uns = 0;

  // for cur_coord in floor[coord].machine.coords {
  for i in 0..factory.floor[coord].machine.coords.len() {
    let cur_coord= factory.floor[coord].machine.coords[i];

    let (u1, u2, u3) = port_to_counts(factory.floor[cur_coord].port_u);
    let (r1, r2, r3) = port_to_counts(factory.floor[cur_coord].port_r);
    let (d1, d2, d3) = port_to_counts(factory.floor[cur_coord].port_d);
    let (l1, l2, l3) = port_to_counts(factory.floor[cur_coord].port_l);

    ins += u1 + r1 + d1 + l1;
    ous += u2 + r2 + d2 + l2;
    uns += u3 + r3 + d3 + l3;
  }

  return ( ins, ous, uns );
}
fn auto_port_convert_machine_unknown_to(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize, new_port: Port, attempt: u32) {
  // Given a machine cell, find a port that is unknown and change it to the given port type.
  // Stop as soon as you find one. There should only be one such port, anyways.

  // for cur_coord in floor[coord].machine.coords {
  for i in 0..factory.floor[coord].machine.coords.len() {
    let cur_coord= factory.floor[coord].machine.coords[i]; // Is this cheaper than the alt?

    if factory.floor[cur_coord].port_u == Port::Unknown {
      if options.trace_priority_step { log(format!("  - machine @{}; up port is {:?}", cur_coord, new_port)); }
      factory.floor[cur_coord].port_u = new_port;
      if new_port == Port::Inbound {
        factory.floor[factory.floor[cur_coord].machine.main_coord].ins.push(( Direction::Up, cur_coord, factory.floor[coord].coord_u.unwrap(), Direction::Down ));
      } else {
        factory.floor[factory.floor[cur_coord].machine.main_coord].outs.push(( Direction::Up, cur_coord, factory.floor[coord].coord_u.unwrap(), Direction::Down ));
      }
      return;
    }

    if factory.floor[cur_coord].port_r == Port::Unknown {
      if options.trace_priority_step { log(format!("  - machine @{}; right port is {:?}", cur_coord, new_port)); }
      factory.floor[cur_coord].port_r = new_port;
      if new_port == Port::Inbound {
        factory.floor[factory.floor[cur_coord].machine.main_coord].ins.push(( Direction::Right, cur_coord, factory.floor[coord].coord_r.unwrap(), Direction::Left ));
      } else {
        factory.floor[factory.floor[cur_coord].machine.main_coord].outs.push(( Direction::Right, cur_coord, factory.floor[coord].coord_r.unwrap(), Direction::Left ));
      }
      return;
    }

    if factory.floor[cur_coord].port_d == Port::Unknown {
      if options.trace_priority_step { log(format!("  - machine @{}; down port is {:?}", cur_coord, new_port)); }
      factory.floor[cur_coord].port_d = new_port;
      if new_port == Port::Inbound {
        factory.floor[factory.floor[cur_coord].machine.main_coord].ins.push(( Direction::Down, cur_coord, factory.floor[coord].coord_d.unwrap(), Direction::Up ));
      } else {
        factory.floor[factory.floor[cur_coord].machine.main_coord].outs.push(( Direction::Down, cur_coord, factory.floor[coord].coord_d.unwrap(), Direction::Up ));
      }
      return;
    }

    if factory.floor[cur_coord].port_l == Port::Unknown {
      if options.trace_priority_step { log(format!("  - machine @{}; left port is {:?}", cur_coord, new_port)); }
      factory.floor[cur_coord].port_l = new_port;
      if new_port == Port::Inbound {
        factory.floor[factory.floor[cur_coord].machine.main_coord].ins.push(( Direction::Left, cur_coord, factory.floor[coord].coord_l.unwrap(), Direction::Right ));
      } else {
        factory.floor[factory.floor[cur_coord].machine.main_coord].outs.push(( Direction::Left, cur_coord, factory.floor[coord].coord_l.unwrap(), Direction::Right ));
      }
      return;
    }
  }

  panic!("should find at least (and most) one unknown port in this machine...");
}

pub fn keep_auto_porting(options: &mut Options, state: &mut State, factory: &mut Factory) {
  // Start at demands, mark connected belts
  // From connected belts, mark any other connected belt if it is connected to only one unmarked
  // belt. If it is connected to a machine or belt with no unmarked neighbors, then it is looping.
  // From connected machines, do the same
  // When all paths are exhausted collect all machines and belts which are connected to at least
  // one unmarked belt. Mark those and repeat.
  // When there is no more

  let mut attempt = 1; // start at 1 because this value gets used -1, too, and it's a u32.
  while auto_port(options, state, factory, attempt) {
    attempt += 1;
  }
}
pub fn auto_port(options: &mut Options, state: &mut State, factory: &mut Factory, attempt: u32) -> bool {
  assert!(attempt > 0, "attempt must be non-zero because it gets deducted");
  if options.trace_priority_step { log(format!("auto_port({})", attempt)); }
  let mut changed = false;
  for coord in 0..FLOOR_CELLS_WH {
    match factory.floor[coord].kind {
      CellKind::Empty => {
        // could go ahead and mark all neighbor belts as none ports as well...

      }
      CellKind::Belt => {
        let (next, changed_now) = auto_port_cell_self(factory, coord);
        if changed_now { changed = true; }
        if next { continue; }

        if auto_port_belt_u(options, state, factory, coord) { changed = true; }
        if auto_port_belt_r(options, state, factory, coord) { changed = true; }
        if auto_port_belt_d(options, state, factory, coord) { changed = true; }
        if auto_port_belt_l(options, state, factory, coord) { changed = true; }
      }
      CellKind::Machine => {
        // Machines can cover multiple cells, have a main cell and sub cells (-> main.machine.subs)
        // Consider only the main a machine block, ignore the subs
        // The cells are iterated over up to three times per coord iteration (should be no big deal)

        if factory.floor[coord].machine.kind == MachineKind::Main {
          if auto_port_machine_neighbors(options, state, factory, coord, attempt) {
            changed = true;
          }

          // Change the attempt so we can reuse the value.
          let ( ins, outs, uns ) = auto_port_discover_machine_ports(factory, coord, attempt - 1);
          if ins == 0 && outs > 0 && uns == 1 {
            // Find the undetermined port and turn it to an Inbound port
            auto_port_convert_machine_unknown_to(options, state, factory, coord, Port::Inbound, attempt);
          } else if outs == 0 && ins > 0 && uns == 1 {
            // Find the undetermined port and turn it to an Outbound port
            auto_port_convert_machine_unknown_to(options, state, factory, coord, Port::Outbound, attempt);
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
