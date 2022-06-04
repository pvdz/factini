use std::collections::VecDeque;

use super::belt::*;
use super::cell::*;
use super::demand::*;
use super::floor::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::port::*;
use super::state::*;
use super::supply::*;
use super::utils::*;

fn should_be_marked_for_cell_sorting_machine(options: &Options, floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize) -> bool {
  // Ignore sub machine cells
  if options.trace_priority_step { super::log(format!("   - Testing machine at {}", coord).as_str()); }
  for index in 0..floor[coord].machine.coords.len() {
    let mcoord = floor[coord].machine.coords[index];
    // Check if this cell has any outgoing ports where the neighbor is not marked
    // If there's an unmarked outbound neighbor then we have to wait for that one to be marked

    // Dunno if this performs well and it looks ugly but it does work. Apparently.

    if options.trace_priority_step { super::log(format!("    - ({}) {:?} {:?} {:?} {:?} , {:?} {:?} {:?} {:?} , {} {} {} {}",
      mcoord,
      floor[mcoord].port_u,
      floor[mcoord].port_r,
      floor[mcoord].port_d,
      floor[mcoord].port_l,

      floor[mcoord].coord_u, floor[mcoord].coord_r, floor[mcoord].coord_d, floor[mcoord].coord_l
      , match floor[mcoord].coord_u { Some(x) => format!("{}", floor[x].marked), None => "none".to_string() }
      , match floor[mcoord].coord_r { Some(x) => format!("{}", floor[x].marked), None => "none".to_string() }
      , match floor[mcoord].coord_d { Some(x) => format!("{}", floor[x].marked), None => "none".to_string() }
      , match floor[mcoord].coord_l { Some(x) => format!("{}", floor[x].marked), None => "none".to_string() }
    ).as_str()); }

    if floor[mcoord].port_u == Port::Outbound {
      if let Some(omcoord) = floor[mcoord].coord_u {
        if !floor[omcoord].marked {
          if options.trace_priority_step { super::log(format!("      - unmarked neighbor to outbound port, machine is not ready").as_str()); }
          return false;
        }
      }
    }
    if floor[mcoord].port_r == Port::Outbound {
      if let Some(omcoord) = floor[mcoord].coord_r {
        if !floor[omcoord].marked {
          if options.trace_priority_step { super::log(format!("      - unmarked neighbor to outbound port, machine is not ready").as_str()); }
          return false;
        }
      }
    }
    if floor[mcoord].port_d == Port::Outbound {
      if let Some(omcoord) = floor[mcoord].coord_d {
        if !floor[omcoord].marked {
          if options.trace_priority_step { super::log(format!("      - unmarked neighbor to outbound port, machine is not ready").as_str()); }
          return false;
        }
      }
    }
    if floor[mcoord].port_l == Port::Outbound {
      if let Some(omcoord) = floor[mcoord].coord_l {
        if !floor[omcoord].marked {
          if options.trace_priority_step { super::log(format!("      - unmarked neighbor to outbound port, machine is not ready").as_str()); }
          return false;
        }
      }
    }
  }

  // Okay. None of the ports of any of the machine cells were outbound to an unmarked cell
  // so this machine can be marked next.
  if options.trace_priority_step { super::log(format!("   - Marking machine {}", coord).as_str()); }
  return true;
}

fn should_be_marked_for_cell_sorting_non_machine(options: &Options, cell: &Cell, cell_u: &Cell, cell_r: &Cell, cell_d: &Cell, cell_l: &Cell) -> bool {
  // When determining cell processing order, should this be considered the next priority?
  // A cell is next in the list when all its outputs are either not connected outbound ports
  // or connected to cells that are already marked.
  // Machines and belts can have multiple outbound ports and they may not be connected anyways.
  // Suppliers only have one outbound port.
  // Demanders should already be marked at this point.

  assert!(!cell.marked, "should not yet be marked");
  // if cell.marked {
  //   // This cell was already gathered in a previous step so ignore it here
  //   return false;
  // }

  if options.trace_priority_step { super::log(format!("   - marked: {} {} {} {}, kind: {:?} {:?} {:?} {:?}", cell_u.marked, cell_r.marked, cell_d.marked, cell_l.marked, cell_u.kind, cell_r.kind, cell_d.kind, cell_l.kind).as_str()); }

  match cell.kind {
    CellKind::Empty => {
      return false
    },
    CellKind::Belt => {
      return
        (cell_u.marked || match cell_u.kind {
          CellKind::Empty => true,
          CellKind::Belt => false,
          CellKind::Machine => false,
          CellKind::Supply => true,
          CellKind::Demand => false,
        }) &&
          (cell_r.marked || match cell_r.kind {
            CellKind::Empty => true,
            CellKind::Belt => false,
            CellKind::Machine => false,
            CellKind::Supply => true,
            CellKind::Demand => false,
          }) &&
          (cell_d.marked || match cell_d.kind {
            CellKind::Empty => true,
            CellKind::Belt => false,
            CellKind::Machine => false,
            CellKind::Supply => true,
            CellKind::Demand => false,
          }) &&
          (cell_l.marked || match cell_l.kind {
            CellKind::Empty => true,
            CellKind::Belt => false,
            CellKind::Machine => false,
            CellKind::Supply => true,
            CellKind::Demand => false,
          });
    },
    CellKind::Machine => {
      panic!("This function should not receive machine cells");
    }
    CellKind::Supply => {
      // Supplies are always the last cell in the priority list so return false here for simplicity
      return false
    }
    CellKind::Demand => {
      panic!("Demands should be marked by now. This should not be reachable.");
    }
  }
}

pub fn create_prio_list(options: &Options, floor: &mut [Cell; FLOOR_CELLS_WH]) -> Vec<usize> {
  super::log(format!("create_prio_list()... options.trace_priority_step={}", options.trace_priority_step).as_str());
  // Collect cells by marking them and putting their coords in a vec. In the end the vec must have
  // all non-empty cells and the factory game tick should traverse cells in that order. This way
  // you work around the belt wanting to unload onto another belt that is currently full but would
  // be empty after this tick as well.
  //
  // While there are tiles left;
  // - start with unprocessed Demands
  //   - mark all their neighbors with outgoing paths to this Demand
  // - while there are cells not in the list yet
  //   - find all cells where all outgoing paths lead to marked or inaccessible cells
  //     - mark them and put them in the list
  // - while there are still unprocessed cells left
  //   - pick a random one. maybe prefer
  //     - ones connected to at least one marked cell
  //     - not connected to suppliers
  //     - pick furthest distance to supplier?

  // This will be the priority list of cells to visit and in this order
  let mut out: Vec<usize> = vec!();

  // First unmark all cells
  for coord in 0..FLOOR_CELLS_WH {
    floor[coord].marked = false;
  }

  let mut demand_connects = vec!();
  for coord in 0..FLOOR_CELLS_WH {
    if options.trace_priority_step { super::log(format!("- kind {} {:?} {:?} {} {}", coord, floor[coord].kind, to_xy(coord), floor[coord].x, floor[coord].y).as_str()); }

    match floor[coord].kind {
      CellKind::Demand => {
        out.push(coord);

        let coord2 = floor[coord].demand.neighbor_coord;
        floor[coord].marked = true;
        floor[coord2].marked = true;
        if options.trace_priority_step { super::log(format!("- Adding {} as the cell that is connected to a Demand at {}", coord2, coord).as_str()); }
        demand_connects.push(coord2);
      }
      CellKind::Empty => {
        floor[coord].marked = true;
      }
      CellKind::Belt => {}
      CellKind::Machine => {}
      CellKind::Supply => {}
    }
  }

  if options.trace_priority_step {
    super::log(format!("- out {:?}", demand_connects).as_str());
    super::log(format!("- connected to demanders {:?}", demand_connects).as_str());
  }

  // out contains all demanders now. push them as the next step in priority.
  for coord in demand_connects {
    out.push(coord);
  }

  if options.trace_priority_step {
    super::log(format!("- after step 1, before loop {:?}", out).as_str());
  }

  let mut noop = empty_cell(0, 0);
  noop.marked = true;

  let mut stepped = 0;
  let mut found_something = true;
  let mut some_left = true;
  while found_something && some_left {
    stepped += 1;
    found_something = false;
    some_left = false;
    if options.trace_priority_step { super::log(format!("next step loop ({})", stepped).as_str()); }

    //          0 123456789012345 6
    // (   )  "┌───────────────────┐"       "┌───────────────────┐"    "┌───────────────────┐"    "┌───────────────────┐"    "┌───────────────────┐"
    // (0  )  "│         s         │"       "│         .         │"    "│         .         │"    "│         v         │"    "│         .         │"
    // (   )  "│ ┌───────────────┐ │"       "│ ┌───────────────┐ │"    "│ ┌───────────────┐ │"    "│ ┌───────────────┐ │"    "│ ┌───────────────┐ │"
    // (17 )  "│ │ααα    ║       │ │"       "│ │...    v       │ │"    "│ │...    .       │ │"    "│ │...    v       │ │"    "│ │...    .       │ │"
    // (34 )  "│ │ααα════╩═════╗ │ │"       "│ │.......v...... │ │"    "│ │..<<<<<<<<<<<. │ │"    "│ │.............^ │ │"    "│ │...<<<<<<<<<<< │ │"
    // (51 )  "│ │ααα          ║ │ │"       "│ │...          ^ │ │"    "│ │...          . │ │"    "│ │.v.          ^ │ │"    "│ │...          . │ │"
    // (68 )  "│ │ ║           ║ │ │"       "│ │ v           ^ │ │"    "│ │ .           . │ │"    "│ │ v           ^ │ │"    "│ │ .           . │ │"
    // (85 )  "│ │ ╚═════╗     ║ │ │"       "│ │ v......     ^ │ │"    "│ │ >>>>>>.     . │ │"    "│ │ ......v     ^ │ │"    "│ │ .>>>>>>     . │ │"
    // (102)  "│ │       ║     ║ │ │"       "│ │       v     ^ │ │"    "│ │       .     . │ │"    "│ │       v     ^ │ │"    "│ │       .     . │ │"
    // (119)  "│ │      βββ    ║ │ │"       "│ │      .v.    ^ │ │"    "│ │      ...    . │ │"    "│ │      ...    ^ │ │"    "│ │      ...    . │ │"
    // (136)  "│ │      βββ    ║ │ │"       "│ │      ...    ^ │ │"    "│ │      ...    . │ │"    "│ │      ...    ^ │ │"    "│ │      ...    . │ │"
    // (153)  "│ │      βββ    ║ │ │"       "│ │      ...    ^ │ │"    "│ │      ...    . │ │"    "│ │      .v.    ^ │ │"    "│ │      ...    . │ │"
    // (170)  "│ │       ║     ╚═│s│"       "│ │       v     ^.│.│"    "│ │       .     <<│.│"    "│ │       v     ..│.│"    "│ │       .     .<│<│"
    // (187)  "│ │ ╔═════╣       │ │"       "│ │ ......v       │ │"    "│ │ <<<<<<.       │ │"    "│ │ v.....v       │ │"    "│ │ .<<<<<<       │ │"
    // (204)  "│ │ ║     ║       │ │"       "│ │ v     v       │ │"    "│ │ .     .       │ │"    "│ │ v     v       │ │"    "│ │ .     .       │ │"
    // (221)  "│ │ ║     ║       │ │"       "│ │ v     v       │ │"    "│ │ .     .       │ │"    "│ │ v     v       │ │"    "│ │ .     .       │ │"
    // (238)  "│d│═╝  ╔══╝       │ │"       "│.│.v  ...v       │ │"    "│<│<.  <<<.       │ │"    "│.│..  v...       │ │"    "│.│<<  .<<<       │ │"
    // (255)  "│ │    ║          │ │"       "│ │    v          │ │"    "│ │    .          │ │"    "│ │    v          │ │"    "│ │    .          │ │"
    // (   )  "│ └───────────────┘ │"       "│ └───────────────┘ │"    "│ └───────────────┘ │"    "│ └───────────────┘ │"    "│ └───────────────┘ │"
    // (272)  "│      d            │"       "│      v            │"    "│      .            │"    "│      .            │"    "│      .            │"
    // (   )  "└───────────────────┘"       "└───────────────────┘"    "└───────────────────┘"    "└───────────────────┘"    "└───────────────────┘"
    //          0 123456789012345 6

    // Only walk inside cells. Assume there's always an up/right/down/left neighbor cell.
    for coord in FLOOR_CELLS_W..FLOOR_CELLS_WH - FLOOR_CELLS_W {
      if coord % FLOOR_CELLS_W == 0 || coord % FLOOR_CELLS_W == FLOOR_CELLS_W - 1 { continue; } // Skip edge cells (none/supply/demand)
      assert!(!floor[coord].is_edge);
      if floor[coord].marked { continue; }

      if options.trace_priority_step && !floor[coord].marked { super::log(format!(" - kind {} {:?}, marked: {}, xy: {}x{}", coord, floor[coord].kind, floor[coord].marked, floor[coord].x, floor[coord].y).as_str()); }

      let is_machine = floor[coord].kind == CellKind::Machine;

      if
      if is_machine {
        floor[coord].machine.kind == MachineKind::Main && should_be_marked_for_cell_sorting_machine(options, floor, coord)
      } else {
        should_be_marked_for_cell_sorting_non_machine(
          options,
          &floor[coord],
          if floor[coord].port_u == Port::Outbound { if let Some(coord_u) = floor[coord].coord_u { &floor[coord_u] } else { &noop } } else { &noop },
          if floor[coord].port_r == Port::Outbound { if let Some(coord_r) = floor[coord].coord_r { &floor[coord_r] } else { &noop } } else { &noop },
          if floor[coord].port_d == Port::Outbound { if let Some(coord_d) = floor[coord].coord_d { &floor[coord_d] } else { &noop } } else { &noop },
          if floor[coord].port_l == Port::Outbound { if let Some(coord_l) = floor[coord].coord_l { &floor[coord_l] } else { &noop } } else { &noop },
        )
      }
      {
        if options.trace_priority_step { super::log(format!("    - adding {:?}", to_xy(coord)).as_str()); }
        floor[coord].marked = true;
        if floor[coord].kind == CellKind::Machine && floor[coord].machine.kind == MachineKind::SubBuilding {
          // Skip actually adding the coord. Just mark it and move on. Only add the main cell for each machine.
        } else {
          out.push(coord);
        }
        found_something = true;
        if is_machine {
          // Mark all machine parts
          for index in 0..floor[coord].machine.coords.len() {
            let mcoord = floor[coord].machine.coords[index];
            floor[mcoord].marked = true;
          }
        }
      } else {
        some_left = true;
      }
    }

    if options.trace_priority_step { super::log(format!("- after step {}: {:?}, found? {} left? {}", stepped, out, found_something, some_left).as_str()); }
  }
  if options.trace_priority_step { super::log(format!("- done with connected cells. now adding remaining unmarked non-empty cells...").as_str()); }

  // Mark point of "rest". 0.0 is an unused cell anyways.
  // out.push(0);
  // out.push(0);

  // Gather the remaining cells. This means not all cells are properly hooked up (or there's
  // a circular loop or something). In that case accept a sub-optimal tick order.
  // Alternatively, we could ignore these and not tick them at all. tbd
  for coord in 0..FLOOR_CELLS_WH {
    let (x, y) = to_xy(coord);
    if !floor[coord].marked && floor[coord].kind != CellKind::Empty {
      if options.trace_priority_step{ super::log(format!("  - adding {} {:?}", coord, (x, y)).as_str()); }
      if floor[coord].kind == CellKind::Machine && floor[coord].machine.kind == MachineKind::SubBuilding {
        // Skip actually adding the coord. Just mark it and move on. Only add the main cell for each machine.
      } else {
        out.push(coord);
      }
      floor[coord].marked = true; // Kinda pointless
    }
  }

  return out;
}
