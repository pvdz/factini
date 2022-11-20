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
use super::port::*;
use super::state::*;
use super::supply::*;
use super::utils::*;
use super::log;

fn auto_port_cell_belt(options: &Options, state: &State, factory: &mut Factory, coord: usize, force_unknowns: bool) -> (bool, bool) {
  auto_port_cell_belt2(options, state, &mut factory.floor, coord, force_unknowns)
}
pub fn auto_port_cell_belt2(options: &Options, state: &State, floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize, force_unknowns: bool) -> (bool, bool) {
  // This only affects unknown ports (!), not auto .ins/.outs from blank
  // Returns ( <unknowns left>, <unknowns changed> )

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

  if options.trace_porting_step { log!("- auto_port_cell_belt({}) has {} unknown ports, {} in ports, and {} out ports", coord, uns, ins, outs); }

  if uns == 0 {
    // No unknown ports. Nothing left to do.
    return ( false, false ); // No unknowns, no changes
  }

  if !force_unknowns || uns > 1 {
    return ( true, false ); // Some unknowns, no changes
  }

  if ins > 0 && outs == 0 && uns == 1 {
    // There is one unknown port and we already have at least one outbound port
    // so the unknown port must be inbound (or the config is broken)
    if floor[coord].port_u == Port::Unknown {
      // log!("- belt @{}; up port must be outbound", coord);
      floor[coord].port_u = Port::Outbound;
      floor[coord].outs.push(( Direction::Up, coord, floor[coord].coord_u.unwrap(), Direction::Down ));
    } else if floor[coord].port_r == Port::Unknown {
      // log!("- belt @{}; right port must be outbound", coord);
      floor[coord].port_r = Port::Outbound;
      floor[coord].outs.push(( Direction::Right, coord, floor[coord].coord_r.unwrap(), Direction::Left ));
    } else if floor[coord].port_d == Port::Unknown {
      // log!("- belt @{}; down port must be outbound", coord);
      floor[coord].port_d = Port::Outbound;
      floor[coord].outs.push(( Direction::Down, coord, floor[coord].coord_d.unwrap(), Direction::Up ));
    } else if floor[coord].port_l == Port::Unknown {
      // log!("- belt @{}; left port must be outbound", coord);
      floor[coord].port_l = Port::Outbound;
      floor[coord].outs.push(( Direction::Left, coord, floor[coord].coord_l.unwrap(), Direction::Right ));
    } else {
      panic!("should be an unknown port");
    }

    return ( false, true ); // No unknowns, some changes
  } else if outs > 0 && ins == 0 && uns == 1 {
    // There is one unknown port and we already have at least one inbound port
    // so the unknown port must be outbound (or the config is broken)

    if floor[coord].port_u == Port::Unknown {
      // log!("- belt @{}; up port must be inbound", coord);
      floor[coord].port_u = Port::Inbound;
      floor[coord].ins.push(( Direction::Up, coord, floor[coord].coord_u.unwrap(), Direction::Down ));
    } else if floor[coord].port_r == Port::Unknown {
      // log!("- belt @{}; right port must be inbound", coord);
      floor[coord].port_r = Port::Inbound;
      floor[coord].ins.push(( Direction::Right, coord, floor[coord].coord_r.unwrap(), Direction::Left ));
    } else if floor[coord].port_d == Port::Unknown {
      // log!("- belt @{}; down port must be inbound", coord);
      floor[coord].port_d = Port::Inbound;
      floor[coord].ins.push(( Direction::Down, coord, floor[coord].coord_d.unwrap(), Direction::Up ));
    } else if floor[coord].port_l == Port::Unknown {
      // log!("- belt @{}; left port must be inbound", coord);
      floor[coord].port_l = Port::Inbound;
      floor[coord].ins.push(( Direction::Left, coord, floor[coord].coord_l.unwrap(), Direction::Right ));
    } else {
      panic!("should be an unknown port");
    }

    return ( false, true ); // No unknowns, some changes
  }

  return (true, false); // One unknown, no changes
}
pub fn auto_ins_outs(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory) {
  auto_ins_outs_floor(options, state, config, &mut factory.floor);
}
pub fn auto_ins_outs_floor(options: &mut Options, state: &mut State, config: &Config, floor: &mut [Cell; FLOOR_CELLS_WH]) {
  // Clear .ins and .outs and discover them for the entire floor.
  // Only checks own ports, not if they're actually connected.

  log!("auto_ins_outs_floor()");

  for coord in 0..FLOOR_CELLS_WH {
    floor[coord].ins.clear();
    floor[coord].outs.clear();

    if floor[coord].kind == CellKind::Machine {
      // Only discover ins and outs for the main coord. They're ignored for the others, I think.
      if coord == floor[coord].machine.main_coord {
        machine_discover_ins_and_outs_floor(floor, coord);
        // log!("Machine after .ins/.outs discovery: {:?} {:?}", floor[coord].ins, floor[coord].outs)
      }
    } else {
      match floor[coord].port_u {
        Port::Inbound => floor[coord].ins.push( ( Direction::Up, coord, to_coord_up(coord), Direction::Down ) ),
        Port::Outbound => floor[coord].outs.push( ( Direction::Up, coord, to_coord_up(coord), Direction::Down ) ),
        Port::Unknown => (),
        Port::None => (),
      };
      match floor[coord].port_r {
        Port::Inbound => floor[coord].ins.push( ( Direction::Right, coord, to_coord_right(coord), Direction::Left ) ),
        Port::Outbound => floor[coord].outs.push( ( Direction::Right, coord, to_coord_right(coord), Direction::Left ) ),
        Port::Unknown => (),
        Port::None => (),
      };
      match floor[coord].port_d {
        Port::Inbound => floor[coord].ins.push( ( Direction::Down, coord, to_coord_down(coord), Direction::Up ) ),
        Port::Outbound => floor[coord].outs.push( ( Direction::Down, coord, to_coord_down(coord), Direction::Up ) ),
        Port::Unknown => (),
        Port::None => (),
      };
      match floor[coord].port_l {
        Port::Inbound => floor[coord].ins.push( ( Direction::Left, coord, to_coord_left(coord), Direction::Right ) ),
        Port::Outbound => floor[coord].outs.push( ( Direction::Left, coord, to_coord_left(coord), Direction::Right ) ),
        Port::Unknown => (),
        Port::None => (),
      };
      // If we don't do this then in some cases the initial map might load incorrect tiles (show sprite for unknown ports even when port is known)
      // TODO: I think this is making another step redundant but I don't think it should really matter?
      if floor[coord].kind == CellKind::Belt {
        fix_belt_meta_floor(options, state, floor, coord);
      }
    }
  }
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
      if options.trace_porting_step { log!("- belt @{}; setting up port to inbound", coord); }
      factory.floor[coord].port_u = Port::Inbound;
      factory.floor[coord].ins.push(( Direction::Up, coord, factory.floor[coord].coord_u.unwrap(), Direction::Down ));
    }
    Port::Outbound => {
      if options.trace_porting_step { log!("- belt @{}; setting up port to outbound", coord); }
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
      if options.trace_porting_step { log!("- belt @{}; setting right port to inbound", coord); }
      factory.floor[coord].port_r = Port::Inbound;
      factory.floor[coord].ins.push(( Direction::Right, coord, factory.floor[coord].coord_r.unwrap(), Direction::Left ));
    }
    Port::Outbound => {
      if options.trace_porting_step { log!("- belt @{}; setting right port to outbound", coord); }
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
      if options.trace_porting_step { log!("- belt @{}; setting down port to inbound", coord); }
      factory.floor[coord].port_d = Port::Inbound;
      factory.floor[coord].ins.push(( Direction::Down, coord, factory.floor[coord].coord_d.unwrap(), Direction::Up ));
    }
    Port::Outbound => {
      if options.trace_porting_step { log!("- belt @{}; setting down port to outbound", coord); }
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
      if options.trace_porting_step { log!("- belt @{}; setting left port to inbound", coord); }
      factory.floor[coord].port_l = Port::Inbound;
      factory.floor[coord].ins.push(( Direction::Left, coord, factory.floor[coord].coord_l.unwrap(), Direction::Right ));
    }
    Port::Outbound => {
      if options.trace_porting_step { log!("- belt @{}; setting left port to outbound", coord); }
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
      if options.trace_porting_step { log!("  - machine @{} up port is inbound", coord); }
      factory.floor[coord].port_u = Port::Inbound;
      factory.floor[factory.floor[coord].machine.main_coord].ins.push(( Direction::Up, coord, factory.floor[coord].coord_u.unwrap(), Direction::Down ));
    }
    Port::Outbound => {
      if options.trace_porting_step { log!("  - machine @{} up port is outbound", coord); }
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
      if options.trace_porting_step { log!("  - machine @{} right port is inbound", coord); }
      factory.floor[coord].port_r = Port::Inbound;
      factory.floor[factory.floor[coord].machine.main_coord].ins.push(( Direction::Right, coord, factory.floor[coord].coord_r.unwrap(), Direction::Left ));
    }
    Port::Outbound => {
      if options.trace_porting_step { log!("  - machine @{} right port is outbound", coord); }
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
      if options.trace_porting_step { log!("  - machine @{}; below cell is a {:?} and up port of below cell is {:?}", coord, factory.floor[ocoord].kind, factory.floor[ocoord].port_u); }
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
      if options.trace_porting_step { log!("  - machine @{}; setting down port to inbound", coord); }
      factory.floor[coord].port_d = Port::Inbound;
      factory.floor[factory.floor[coord].machine.main_coord].ins.push(( Direction::Down, coord, factory.floor[coord].coord_d.unwrap(), Direction::Up ));
    }
    Port::Outbound => {
      if options.trace_porting_step { log!("  - machine @{}; setting down port to outbound", coord); }
      factory.floor[coord].port_d = Port::Outbound;
      factory.floor[factory.floor[coord].machine.main_coord].outs.push(( Direction::Down, coord, factory.floor[coord].coord_d.unwrap(), Direction::Up ));
    }
    Port::None => {
      if options.trace_porting_step { log!("  - machine @{}; setting down port to none", coord); }
      factory.floor[coord].port_d = Port::None;
    }
    Port::Unknown => {
      if options.trace_porting_step { log!("  - machine @{}; down port remains unknown", coord); }
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
      if options.trace_porting_step { log!("  - machine @{} left port is inbound", coord); }
      factory.floor[coord].port_l = Port::Inbound;
      factory.floor[factory.floor[coord].machine.main_coord].ins.push(( Direction::Left, coord, factory.floor[coord].coord_l.unwrap(), Direction::Right ));
    }
    Port::Outbound => {
      if options.trace_porting_step { log!("  - machine @{} left port is outbound", coord); }
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

  if options.trace_porting_step { log!("- auto_port_machine_neighbors({}, {}): {:?}", coord, attempt, factory.floor[coord].machine.coords); }
  
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
  return auto_port_discover_machine_ports_floor(&mut factory.floor, coord, attempt);
}
fn auto_port_discover_machine_ports_floor(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize, attempt: u32) -> (u16, u16, u16) {
  assert_eq!(floor[coord].kind, CellKind::Machine);
  assert!(floor[coord].machine.kind == MachineKind::Main || floor[coord].machine.kind == MachineKind::Unknown, "either this is a known machine and the Main should be used or it is an Unknown machine and the top left cell should be given");
  assert_eq!(floor[coord].machine.main_coord, coord);

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

  // log!("auto_port_discover_machine_ports_floor({}) -> ins: {:?} outs: {:?}", coord, ins, ous);

  return ( ins, ous, uns );
}
fn auto_port_convert_machine_unknown_to(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize, new_port: Port, attempt: u32) {
  // Given a machine cell, find a port that is unknown and change it to the given port type.
  // Stop as soon as you find one. There should only be one such port, anyways.

  if options.trace_porting_step { log!("- auto_port_convert_machine_unknown_to({})", coord); }

  // for cur_coord in floor[coord].machine.coords {
  for i in 0..factory.floor[coord].machine.coords.len() {
    let sub_machine_coord = factory.floor[coord].machine.coords[i]; // Is this cheaper than the alt?

    if factory.floor[sub_machine_coord].port_u == Port::Unknown {
      if options.trace_porting_step { log!("  - machine @{}; up port was unknown and is now {:?}", sub_machine_coord, new_port); }
      factory.floor[sub_machine_coord].port_u = new_port;
      factory.changed = true;
      if new_port == Port::Inbound {
        factory.floor[factory.floor[sub_machine_coord].machine.main_coord].ins.push(( Direction::Up, sub_machine_coord, factory.floor[coord].coord_u.unwrap(), Direction::Down ));
      } else {
        factory.floor[factory.floor[sub_machine_coord].machine.main_coord].outs.push(( Direction::Up, sub_machine_coord, factory.floor[coord].coord_u.unwrap(), Direction::Down ));
      }
      return;
    }

    if factory.floor[sub_machine_coord].port_r == Port::Unknown {
      if options.trace_porting_step { log!("  - machine @{}; right port was unknown and is now {:?}", sub_machine_coord, new_port); }
      factory.floor[sub_machine_coord].port_r = new_port;
      factory.changed = true;
      if new_port == Port::Inbound {
        factory.floor[factory.floor[sub_machine_coord].machine.main_coord].ins.push(( Direction::Right, sub_machine_coord, factory.floor[coord].coord_r.unwrap(), Direction::Left ));
      } else {
        factory.floor[factory.floor[sub_machine_coord].machine.main_coord].outs.push(( Direction::Right, sub_machine_coord, factory.floor[coord].coord_r.unwrap(), Direction::Left ));
      }
      return;
    }

    if factory.floor[sub_machine_coord].port_d == Port::Unknown {
      if options.trace_porting_step { log!("  - machine @{}; down port was unknown and is now {:?}", sub_machine_coord, new_port); }
      factory.floor[sub_machine_coord].port_d = new_port;
      factory.changed = true;
      if new_port == Port::Inbound {
        factory.floor[factory.floor[sub_machine_coord].machine.main_coord].ins.push(( Direction::Down, sub_machine_coord, factory.floor[coord].coord_d.unwrap(), Direction::Up ));
      } else {
        factory.floor[factory.floor[sub_machine_coord].machine.main_coord].outs.push(( Direction::Down, sub_machine_coord, factory.floor[coord].coord_d.unwrap(), Direction::Up ));
      }
      return;
    }

    if factory.floor[sub_machine_coord].port_l == Port::Unknown {
      if options.trace_porting_step { log!("  - machine @{}; left port was unknown and is now {:?}", sub_machine_coord, new_port); }
      factory.floor[sub_machine_coord].port_l = new_port;
      factory.changed = true;
      if new_port == Port::Inbound {
        factory.floor[factory.floor[sub_machine_coord].machine.main_coord].ins.push(( Direction::Left, sub_machine_coord, factory.floor[coord].coord_l.unwrap(), Direction::Right ));
      } else {
        factory.floor[factory.floor[sub_machine_coord].machine.main_coord].outs.push(( Direction::Left, sub_machine_coord, factory.floor[coord].coord_l.unwrap(), Direction::Right ));
      }
      return;
    }
  }

  panic!("should find at least (and most) one unknown port in this machine...");
}

pub fn keep_auto_porting(options: &mut Options, state: &mut State, factory: &mut Factory) {
  log!("keep_auto_porting(options.trace_porting_step = {})", options.trace_porting_step);
  // Start at demands, mark connected belts
  // From connected belts, mark any other connected belt if it is connected to only one unmarked
  // belt. If it is connected to a machine or belt with no unmarked neighbors, then it is looping.
  // From connected machines, do the same
  // When all paths are exhausted collect all machines and belts which are connected to at least
  // one unmarked belt. Mark those and repeat.

  let mut attempt = 1; // start at 1 because this value gets used -1, too, and it's a u32.
  let mut force_unknowns = false;
  loop {
    let ( changes, has_unknowns ) = auto_port(options, state, factory, attempt, force_unknowns);
    if !changes && (!has_unknowns || force_unknowns) {
      // Either there were no changes or there were unknowns and this was already a forced run.
      break;
    }
    force_unknowns = !changes; // If there were no changes then there were unknowns. Force them.
    attempt += 1;
  }
}
pub fn auto_port(options: &mut Options, state: &mut State, factory: &mut Factory, attempt: u32, force_unknowns: bool) -> ( bool, bool ) {
  assert!(attempt > 0, "attempt must be non-zero because it gets deducted");
  if options.trace_porting_step { log!("  - auto_port({}, {})", attempt, force_unknowns); }
  let mut changed = false;
  let mut has_unknowns = false;
  for coord in 0..FLOOR_CELLS_WH {
    match factory.floor[coord].kind {
      CellKind::Empty => {
        // could go ahead and mark all neighbor belts as none ports as well...

      }
      CellKind::Belt => {
        let ( unknowns_left, unknowns_changed) = auto_port_cell_belt(options, state, factory, coord, force_unknowns);
        if unknowns_left { has_unknowns = true; }
        if unknowns_changed { changed = true; }
        if !unknowns_left { continue; }

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
          if options.trace_porting_step { log!("    - machine at {}", coord); }

          if auto_port_machine_neighbors(options, state, factory, coord, attempt) {
            changed = true;
          }

          // // Change the attempt so we can reuse the value.
          let ( ins, outs, uns ) = auto_port_discover_machine_ports(factory, coord, attempt - 1);
          if uns > 0 {
            if force_unknowns {
              if ins == 0 && outs > 0 && uns == 1 {
                // Find the undetermined port and turn it to an Inbound port
                auto_port_convert_machine_unknown_to(options, state, factory, coord, Port::Inbound, attempt);
                changed = true;
              } else if outs == 0 && ins > 0 && uns == 1 {
                // Find the undetermined port and turn it to an Outbound port
                auto_port_convert_machine_unknown_to(options, state, factory, coord, Port::Outbound, attempt);
                changed = true;
              } else {
                // At this step not able to deduce any ports for this machine
              }
            } else {
              has_unknowns = true;
            }
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

  return ( changed, has_unknowns );
}
