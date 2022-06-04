use std::collections::VecDeque;

use super::belt::*;
use super::cell::*;
use super::demand::*;
use super::floor::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::state::*;
use super::supply::*;
use super::utils::*;

pub struct Factory {
  pub ticks: u64,
  pub floor: [Cell; FLOOR_CELLS_WH],
  pub prio: Vec<usize>,
}

pub fn create_factory(options: &mut Options, _state: &mut State, floor_str: String) -> Factory {
  return Factory {
    ticks: 0,
    floor: floor_from_str(floor_str),
    prio: vec!(),
  };
}

fn should_be_marked_for_cell_sorting(options: &Options, cell: &Cell, cell_u: &Cell, cell_r: &Cell, cell_d: &Cell, cell_l: &Cell) -> bool {
  // When determining cell processing order, should this be considered the next priority?
  // A cell is next in the list when all its outputs are either not connected outbound ports
  // or connected to cells that are already marked.
  // Machines and belts can have multiple outbound ports and they may not be connected anyways.
  // Suppliers only have one outbound port.
  // Demanders should already be marked at this point.

  if cell.marked {
    // This cell was already gathered in a previous step so ignore it here
    return false;
  }

  match cell.kind {
    CellKind::Empty => {
      return false
    },
    CellKind::Belt => {
      return
        (cell_u.marked || match cell_u.kind {
          CellKind::Empty => true,
          CellKind::Belt => false,
          CellKind::Machine => true, // I think?
          CellKind::Supply => true,
          CellKind::Demand => false,
        }) &&
        (cell_r.marked || match cell_r.kind {
          CellKind::Empty => true,
          CellKind::Belt => false,
          CellKind::Machine => true, // I think?
          CellKind::Supply => true,
          CellKind::Demand => false,
        }) &&
        (cell_d.marked || match cell_d.kind {
          CellKind::Empty => true,
          CellKind::Belt => false,
          CellKind::Machine => true, // I think?
          CellKind::Supply => true,
          CellKind::Demand => false,
        }) &&
        (cell_l.marked || match cell_l.kind {
          CellKind::Empty => true,
          CellKind::Belt => false,
          CellKind::Machine => true, // I think?
          CellKind::Supply => true,
          CellKind::Demand => false,
        });
    },
    CellKind::Machine => {
      // Machines can only connect to belts, so for anything else return "true", ignoring the state
      // Machines are next in prio when all their neighbors are not belts, or all neighbor
      return
          cell_u.kind != CellKind::Belt ||
          cell_r.kind != CellKind::Belt ||
          cell_d.kind != CellKind::Belt ||
          cell_l.kind != CellKind::Belt ||
          cell_u.marked ||
          cell_r.marked ||
          cell_d.marked ||
          cell_l.marked;
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
pub fn create_prio_list(options: &Options, floor: &mut Vec<Cell>) -> Vec<usize> {
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

  let mut demand_connects = vec!();
  for coord in 0..FLOOR_CELLS_WH {
    // Unmark all cells first
    floor[coord].marked = false;

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

  let mut stepped = 1;
  let mut found_something = true;
  let mut some_left = true;
  while found_something && some_left {
    stepped += 1;
    found_something = false;

    // Only walk inside cells. Assume there's always an up/right/down/left neighbor cell.
    for coord in FLOOR_CELLS_WH..FLOOR_CELLS_WH - FLOOR_CELLS_WH {
      if coord % FLOOR_CELLS_WH == 0 || coord % FLOOR_CELLS_WH == FLOOR_CELLS_WH - 1 { continue; } // Skip edge cells (none/supply/demand)
      assert!(!floor[coord].is_edge);

      if options.trace_priority_step && !floor[coord].marked { super::log(format!(" - kind {} {:?} {:?} {} {}", coord, floor[coord].kind, to_xy(coord), floor[coord].x, floor[coord].y).as_str()); }

      let mut noop = empty_cell(0, 0);
      noop.marked = true;

      if
        should_be_marked_for_cell_sorting(
          options,
          &floor[coord],
          if let Some(coord_u) = floor[coord].coord_u { &floor[coord_u] } else { &noop },
          if let Some(coord_r) = floor[coord].coord_r { &floor[coord_r] } else { &noop },
          if let Some(coord_d) = floor[coord].coord_d { &floor[coord_d] } else { &noop },
          if let Some(coord_l) = floor[coord].coord_l { &floor[coord_l] } else { &noop },
        )
      {
        if options.trace_priority_step { super::log(format!("    - adding {:?}", to_xy(coord)).as_str()); }
        floor[coord].marked = true;
        out.push(coord);
        found_something = true;
      } else {
        some_left = true;
      }
    }

    if options.trace_priority_step { super::log(format!("- after step {}: {:?}", stepped, out).as_str()); }
  }
  if options.trace_priority_step { super::log(format!("- done with connected cells. now adding remaining unmarked non-empty cells...").as_str()); }

  // Gather the remaining cells. This means not all cells are properly hooked up (or there's
  // a circular loop or something). In that case accept a sub-optimal tick order.
  // Alternatively, we could ignore these and not tick them at all. tbd
  for coord in 0..FLOOR_CELLS_WH {
    let (x, y) = to_xy(coord);
    if !floor[coord].marked && floor[coord].kind != CellKind::Empty {
      if options.trace_priority_step{ super::log(format!("  - adding {} {:?}", coord, (x, y)).as_str()); }
      out.push(coord);
      floor[coord].marked = true; // Kinda pointless
    }
  }

  return out;
}

pub fn tick_factory(options: &mut Options, state: &mut State, factory: &mut Factory) {
  factory.ticks += 1;

  for n in 0..factory.prio.len() {
    let coord = factory.prio[n];
    factory.floor[coord].ticks += 1;

    match factory.floor[coord].kind {
      CellKind::Empty => panic!("should not have empty cells in the prio list:: prio index: {}, coord: {}, cell: {:?}", n, coord, factory.floor[coord]),
      CellKind::Belt => tick_belt(options, state, factory, coord),
      CellKind::Machine => tick_machine(options, state, factory, coord),
      CellKind::Supply => tick_supply(options, state, factory, coord),
      CellKind::Demand => tick_demand(options, state, factory, coord),
    }
  }
}
