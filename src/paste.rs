// Paste action abstracted

use super::belt::*;
use super::cell::*;
use super::cli_serialize::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::init::*;
use super::offer::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::state::*;
use super::utils::*;


pub fn paste(options: &mut Options, state: &mut State, factory: &mut Factory, cell_selection: &CellSelection) {
  if state.mouse_mode_selecting && cell_selection.on && state.selected_area_copy.len() > 0 {
    let selected_ox = cell_selection.x.min(cell_selection.x2) as usize;
    let selected_oy = cell_selection.y.min(cell_selection.y2) as usize;
    let clipboard_w = state.selected_area_copy[0].len();
    let clipboard_h = state.selected_area_copy.len();

    for y in 0..clipboard_h {
      for x in 0..clipboard_w {
        let cx = selected_ox + x;
        let cy = selected_oy + y;
        let coord = to_coord(cx, cy);

        paste_one_cell(options, state, factory, x, y, clipboard_w, clipboard_h, cx, cy, coord);
      }
    }

    for y in 0..clipboard_h {
      for x in 0..clipboard_w {
        let cx = selected_ox + x;
        let cy = selected_oy + y;
        if is_middle(cx, cy) {
          let coord = to_coord(cx, cy);
          log(format!("patching edge: {} {} -> {}", x, y, coord));
          log(format!("  fixing belt {:?} {:?} {:?} {:?}", factory.floor[coord].port_u, factory.floor[coord].port_r, factory.floor[coord].port_d, factory.floor[coord].port_l));

          // Belt may be in an inconsistent state if it was connected to a machine, demand, or supply, since we don't copy those.
          // For that reason we have to do a sanity check on each cell and remove/update any incorrectly connected ports
          // This should only affect ports connecting to other cells of the paste but I suppose that overhead doesn't matter here.
          belt_fix_semi_connected_ports_partial(options, state, factory, coord);

          // At this point the ports and that of the neighbor should be fixed. Fix the meta.
          fix_belt_meta(factory, coord);
          // And fix the .ins and .outs of this and its neighbor
          belt_discover_ins_and_outs(factory, coord);
        } else {
          log(format!("Skipping {} {} because they are not in the middle of the floor", cx, cy));
        }
      }
    }

    factory.changed = true;
  }
}

pub fn paste_one_cell(options: &mut Options, state: &mut State, factory: &mut Factory, x: usize, y: usize, w: usize, h: usize, cx: usize, cy: usize, coord: usize) {
  log(format!("- paste_one_cell({} {}) coord {} {} middle? {}", x, y, cx, cy, is_middle(cx, cy)));

  if is_middle(cx, cy) {
    // Are we destroying a machine? Then destroy it proper.
    if factory.floor[coord].kind == CellKind::Machine {
      log(format!("  - Dropping entire machine at target cell"));
      floor_delete_cell_at_partial(options, state, factory, coord);
    } else {
      // And otherwise it must be a belt or an empty cell. Ignore that case here.
    }

    log(format!("  - are {} {} ({} {}) paste edge? {} ({} {}), {} ({} {}), ({} ({} {}), {} ({} {})", x, y, cx, cy,
      y == 0 || cy == 1, y == 0, cy == 1,
      x == w-1 || cx == FLOOR_CELLS_W-2, x == w-1, cx == FLOOR_CELLS_W-2,
      y == h-1 || cy == FLOOR_CELLS_H-2, y == h-1, cy == FLOOR_CELLS_H-2,
      x == 0 || cx == 0, x == 0, cx == 0
    ));

    // Are we copying from a belt? Else create an empty cell.
    if state.selected_area_copy[y][x].kind == CellKind::Belt {
      log(format!("  - Copying from a belt"));
      let meta = state.selected_area_copy[y][x].belt.meta.clone();
      factory.floor[coord] = belt_cell(cx, cy, meta);
      // Copy the port directions
      factory.floor[coord].port_u = state.selected_area_copy[y][x].port_u;
      factory.floor[coord].port_r = state.selected_area_copy[y][x].port_r;
      factory.floor[coord].port_d = state.selected_area_copy[y][x].port_d;
      factory.floor[coord].port_l = state.selected_area_copy[y][x].port_l;

      if y == 0 || cy == 1 {
        log(format!("    - Upper row in copy area"));
        belt_connect_up_if_either_has_port_fix_partial(options, state, factory, coord);
      }

      if x == w-1 || cx == FLOOR_CELLS_W-2 {
        log(format!("    - Right column in copy area"));
        belt_connect_right_if_either_has_port_fix_partial(options, state, factory, coord);
      }

      if y == h-1 || cy == FLOOR_CELLS_H-2 {
        log(format!("    - Bottom row in copy area"));
        belt_connect_down_if_either_has_port_fix_partial(options, state, factory, coord);
      }

      if x == 0 || cx == 0 {
        log(format!("    - Left column in copy area"));
        belt_connect_left_if_either_has_port_fix_partial(options, state, factory, coord);
      }
    }
    else {
      log(format!("  - Copying not from a belt so adding an empty cell"));
      factory.floor[coord] = empty_cell(cx, cy);

      if y == 0 || cy == 1 {
        log(format!("    - Upper row in copy area. Clear above down port; {:?}", factory.floor[to_coord_up(coord)].port_d));
        if let Some(ocoord) = factory.floor[coord].coord_u {
          factory.floor[ocoord].port_d = Port::None;
          belt_discover_ins_and_outs(factory, ocoord);
          fix_belt_meta(factory, ocoord);
        }
      }

      if x == w-1 || cx == FLOOR_CELLS_W-2 {
        log(format!("    - Right row in copy area. Clear rightside left port; {:?}", factory.floor[to_coord_right(coord)].port_l));
        if let Some(ocoord) = factory.floor[coord].coord_r {
          factory.floor[ocoord].port_l = Port::None;
          belt_discover_ins_and_outs(factory, ocoord);
          fix_belt_meta(factory, ocoord);
        }
      }

      if y == h-1 || cy == FLOOR_CELLS_H-2 {
        log(format!("    - Down row in copy area. Clear lower up port; {:?}", factory.floor[to_coord_down(coord)].port_u));
        if let Some(ocoord) = factory.floor[coord].coord_d {
          factory.floor[ocoord].port_u = Port::None;
          belt_discover_ins_and_outs(factory, ocoord);
          fix_belt_meta(factory, ocoord);
        }
      }

      if x == 0 || cx == 0 {
        log(format!("    - Left row in copy area. Clear leftside right port; {:?}", factory.floor[to_coord_left(coord)].port_r));
        if let Some(ocoord) = factory.floor[coord].coord_l {
          factory.floor[ocoord].port_r = Port::None;
          belt_discover_ins_and_outs(factory, ocoord);
          fix_belt_meta(factory, ocoord);
        }
      }
    }

    fix_belt_meta(factory, coord);
  }
}

pub fn belt_connect_up_if_either_has_port_fix_partial(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) {
  // Fixes partial because it doesn't fix the coord meta, nor .ins and .outs, nor .prio

  if let Some(ocoord) = factory.floor[coord].coord_u {
    log(format!("      - connect_up_if_either_has_port_fix_partial({:?}) from:{:?} to:{:?}", factory.floor[coord].kind, factory.floor[coord].port_u, factory.floor[ocoord].port_d));

    match factory.floor[ocoord].kind {
      CellKind::Belt => {
        // check if this cell has a port up, check if upper cell has port going down
        // make sure the opposite cell also has a port and that they are in sync
        // the pasted cell leads, if it has a direction, pick that. if not, pick
        // the direction from the neighbor cell, if not but one has a port then
        // make them both unknown, and otherwise they will are already be set to none.
             if factory.floor[coord].port_u == Port::Inbound {
          if factory.floor[ocoord].port_d != Port::Outbound {
            log(format!("        - Setting up.port_d to outbound"));
            factory.floor[ocoord].port_d = Port::Outbound;
            belt_discover_ins_and_outs(factory, ocoord);
            fix_belt_meta(factory, ocoord);
          }
        }
        else if factory.floor[coord].port_u == Port::Outbound {
          if factory.floor[ocoord].port_d != Port::Inbound {
            log(format!("        - Setting up.port_d to inbound"));
            factory.floor[ocoord].port_d = Port::Inbound;
            belt_discover_ins_and_outs(factory, ocoord);
            fix_belt_meta(factory, ocoord);
          }
        }
        else if factory.floor[ocoord].port_d == Port::Inbound {
          if factory.floor[coord].port_u != Port::Outbound {
            log(format!("        - Setting this.port_u to outbound"));
            factory.floor[coord].port_u = Port::Outbound;
          }
        }
        else if factory.floor[ocoord].port_d == Port::Outbound {
          if factory.floor[coord].port_u != Port::Inbound {
            log(format!("        - Setting this.port_u to inbound"));
            factory.floor[coord].port_u = Port::Inbound;
          }
        }
        else if (factory.floor[coord].port_u == Port::Unknown) != (factory.floor[ocoord].port_d == Port::Unknown) {
          log(format!("        - Setting both ports to unknown"));
          factory.floor[coord].port_u = Port::Unknown;
          factory.floor[ocoord].port_d = Port::Unknown;
          belt_discover_ins_and_outs(factory, ocoord);
          fix_belt_meta(factory, ocoord);
        }
        else {
          log(format!("        - both ports are already unknown or none; {:?} {:?}", factory.floor[coord].port_u, factory.floor[ocoord].port_d));
          assert!(
            (factory.floor[coord].port_u == Port::Unknown && factory.floor[ocoord].port_d == Port::Unknown) ||
            (factory.floor[coord].port_u == Port::None    && factory.floor[ocoord].port_d == Port::None),
            "either both ports are already Unknown or both ports are None {:?} {:?}",
            factory.floor[coord].port_u,
            factory.floor[ocoord].port_d
          );
        }
      }
      CellKind::Supply => {
        // Can only be one way and not sure why we wouldn't connect it immediately
        log(format!("        - Up is supply so this.port_u must be inbound"));
        factory.floor[coord].port_u = Port::Inbound;
      }
      CellKind::Demand  => {
        // Can only be one way and not sure why we wouldn't connect it immediately
        log(format!("        - Up is demand so this.port_u must be outbound"));
        factory.floor[coord].port_u = Port::Outbound;
      }
      CellKind::Machine => {
        // This only depends on the belt since the machine port is auto
        log(format!("        - Up is machine so it depends on this port: {:?}", factory.floor[coord].port_u));

        match factory.floor[coord].port_u {
          Port::Inbound => {
            factory.floor[ocoord].port_d = Port::Outbound;
          }
          Port::Outbound => {
            factory.floor[ocoord].port_d = Port::Inbound;
          }
          Port::Unknown => {
            factory.floor[ocoord].port_d = Port::Unknown;
          }
          Port::None => {
            factory.floor[ocoord].port_d = Port::None;
          }
        }

        let main_coord = factory.floor[ocoord].machine.main_coord;
        machine_discover_ins_and_outs(factory, main_coord);
        fix_belt_meta(factory, ocoord);
      }
      CellKind::Empty => {
        log(format!("        - Up is empty so clear this port_u, which was {:?}", factory.floor[coord].port_u));

        if factory.floor[coord].port_u != Port::None {
          factory.floor[coord].port_u = Port::None;
        }
      }
    }
  }
}
pub fn belt_connect_right_if_either_has_port_fix_partial(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) {
  // Fixes partial because it doesn't fix the coord meta, nor .ins and .outs, nor .prio

  if let Some(ocoord) = factory.floor[coord].coord_r {
    log(format!("      - connect_right_if_either_has_port_fix_partial({:?}) from:{:?} to:{:?}", factory.floor[coord].kind, factory.floor[coord].port_r, factory.floor[ocoord].port_l));

    match factory.floor[ocoord].kind {
      CellKind::Belt => {
        // check if this cell has a port up, check if upper cell has port going down
        // make sure the opposite cell also has a port and that they are in sync
        // the pasted cell leads, if it has a direction, pick that. if not, pick
        // the direction from the neighbor cell, if not but one has a port then
        // make them both unknown, and otherwise they will are already be set to none.
             if factory.floor[coord].port_r == Port::Inbound {
          if factory.floor[ocoord].port_l != Port::Outbound {
            log(format!("        - Setting right.port_l to outbound"));
            factory.floor[ocoord].port_l = Port::Outbound;
            belt_discover_ins_and_outs(factory, ocoord);
            fix_belt_meta(factory, ocoord);
          }
        }
        else if factory.floor[coord].port_r == Port::Outbound {
          if factory.floor[ocoord].port_l != Port::Inbound {
            log(format!("        - Setting right.port_l to inbound"));
            factory.floor[ocoord].port_l = Port::Inbound;
            belt_discover_ins_and_outs(factory, ocoord);
            fix_belt_meta(factory, ocoord);
          }
        }
        else if factory.floor[ocoord].port_l == Port::Inbound {
          if factory.floor[coord].port_r != Port::Outbound {
            log(format!("        - Setting this.port_r to outbound"));
            factory.floor[coord].port_r = Port::Outbound;
          }
        }
        else if factory.floor[ocoord].port_l == Port::Outbound {
          if factory.floor[coord].port_r != Port::Inbound {
            log(format!("        - Setting this.port_r to inbound"));
            factory.floor[coord].port_r = Port::Inbound;
          }
        }
        else if (factory.floor[coord].port_r == Port::Unknown) != (factory.floor[ocoord].port_l == Port::Unknown) {
          log(format!("        - Setting both ports to unknown"));
          factory.floor[coord].port_r = Port::Unknown;
          factory.floor[ocoord].port_l = Port::Unknown;
          belt_discover_ins_and_outs(factory, ocoord);
          fix_belt_meta(factory, ocoord);
        }
        else {
          log(format!("        - both ports are already unknown or none; {:?} {:?}", factory.floor[coord].port_r, factory.floor[ocoord].port_l));
          assert!(
            (factory.floor[coord].port_r == Port::Unknown && factory.floor[ocoord].port_l == Port::Unknown) ||
            (factory.floor[coord].port_r == Port::None    && factory.floor[ocoord].port_l == Port::None),
            "either both ports are already Unknown or both ports are None {:?} {:?}",
            factory.floor[coord].port_r,
            factory.floor[ocoord].port_l
          );
        }
      }
      CellKind::Supply => {
        // Can only be one way and not sure why we wouldn't connect it immediately
        log(format!("        - Right is supply so this.port_r must be inbound"));
        factory.floor[coord].port_r = Port::Inbound;
      }
      CellKind::Demand  => {
        // Can only be one way and not sure why we wouldn't connect it immediately
        log(format!("        - Right is demand so this.port_r must be outbound"));
        factory.floor[coord].port_r = Port::Outbound;
      }
      CellKind::Machine => {
        // This only depends on the belt since the machine port is auto
        log(format!("        - Right is machine so it depends on this port: {:?}", factory.floor[coord].port_r));

        match factory.floor[coord].port_r {
          Port::Inbound => {
            factory.floor[ocoord].port_l = Port::Outbound;
          }
          Port::Outbound => {
            factory.floor[ocoord].port_l = Port::Inbound;
          }
          Port::Unknown => {
            factory.floor[ocoord].port_l = Port::Unknown;
          }
          Port::None => {
            factory.floor[ocoord].port_l = Port::None;
          }
        }

        let main_coord = factory.floor[ocoord].machine.main_coord;
        machine_discover_ins_and_outs(factory, main_coord);
        fix_belt_meta(factory, ocoord);
      }
      CellKind::Empty => {
        log(format!("        - Right is empty so clear this port_r, which was {:?}", factory.floor[coord].port_r));

        if factory.floor[coord].port_r != Port::None {
          factory.floor[coord].port_r = Port::None;
        }
      }
    }
  }
}
pub fn belt_connect_down_if_either_has_port_fix_partial(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) {
  // Fixes partial because it doesn't fix the coord meta, nor .ins and .outs, nor .prio

  if let Some(ocoord) = factory.floor[coord].coord_d {
    log(format!("      - connect_down_if_either_has_port_fix_partial({:?}) from:{:?} to:{:?}", factory.floor[coord].kind, factory.floor[coord].port_d, factory.floor[ocoord].port_u));

    match factory.floor[ocoord].kind {
      CellKind::Belt => {
        // check if this cell has a port up, check if upper cell has port going down
        // make sure the opposite cell also has a port and that they are in sync
        // the pasted cell leads, if it has a direction, pick that. if not, pick
        // the direction from the neighbor cell, if not but one has a port then
        // make them both unknown, and otherwise they will are already be set to none.
             if factory.floor[coord].port_d == Port::Inbound {
          if factory.floor[ocoord].port_u != Port::Outbound {
            log(format!("        - Setting down.port_u to outbound"));
            factory.floor[ocoord].port_u = Port::Outbound;
            belt_discover_ins_and_outs(factory, ocoord);
            fix_belt_meta(factory, ocoord);
          }
        }
        else if factory.floor[coord].port_d == Port::Outbound {
          if factory.floor[ocoord].port_u != Port::Inbound {
            log(format!("        - Setting down.port_u to inbound"));
            factory.floor[ocoord].port_u = Port::Inbound;
            belt_discover_ins_and_outs(factory, ocoord);
            fix_belt_meta(factory, ocoord);
          }
        }
        else if factory.floor[ocoord].port_u == Port::Inbound {
          if factory.floor[coord].port_d != Port::Outbound {
            log(format!("        - Setting this.port_d to outbound"));
            factory.floor[coord].port_d = Port::Outbound;
          }
        }
        else if factory.floor[ocoord].port_u == Port::Outbound {
          if factory.floor[coord].port_d != Port::Inbound {
            log(format!("        - Setting this.port_d to inbound"));
            factory.floor[coord].port_d = Port::Inbound;
          }
        }
        else if (factory.floor[coord].port_d == Port::Unknown) != (factory.floor[ocoord].port_u == Port::Unknown) {
          log(format!("        - Setting both ports to unknown"));
          factory.floor[coord].port_d = Port::Unknown;
          factory.floor[ocoord].port_u = Port::Unknown;
          belt_discover_ins_and_outs(factory, ocoord);
          fix_belt_meta(factory, ocoord);
        }
        else {
          log(format!("        - both ports are already unknown or none; {:?} {:?}", factory.floor[coord].port_d, factory.floor[ocoord].port_u));
          assert!(
            (factory.floor[coord].port_d == Port::Unknown && factory.floor[ocoord].port_u == Port::Unknown) ||
            (factory.floor[coord].port_d == Port::None    && factory.floor[ocoord].port_u == Port::None),
            "either both ports are already Unknown or both ports are None {:?} {:?}",
            factory.floor[coord].port_u,
            factory.floor[ocoord].port_d
          );
        }
      }
      CellKind::Supply => {
        // Can only be one way and not sure why we wouldn't connect it immediately
        log(format!("        - Down is supply so this.port_d must be inbound"));
        factory.floor[coord].port_d = Port::Inbound;
      }
      CellKind::Demand  => {
        // Can only be one way and not sure why we wouldn't connect it immediately
        log(format!("        - Down is demand so this.port_d must be outbound"));
        factory.floor[coord].port_d = Port::Outbound;
      }
      CellKind::Machine => {
        // This only depends on the belt since the machine port is auto
        log(format!("        - Down is machine so it depends on this port: {:?}", factory.floor[coord].port_d));

        match factory.floor[coord].port_d {
          Port::Inbound => {
            factory.floor[ocoord].port_u = Port::Outbound;
          }
          Port::Outbound => {
            factory.floor[ocoord].port_u = Port::Inbound;
          }
          Port::Unknown => {
            factory.floor[ocoord].port_u = Port::Unknown;
          }
          Port::None => {
            factory.floor[ocoord].port_u = Port::None;
          }
        }

        let main_coord = factory.floor[ocoord].machine.main_coord;
        machine_discover_ins_and_outs(factory, main_coord);
        fix_belt_meta(factory, ocoord);
      }
      CellKind::Empty => {
        log(format!("        - Down is empty so clear this port_d, which was {:?}", factory.floor[coord].port_d));

        if factory.floor[coord].port_d != Port::None {
          factory.floor[coord].port_d = Port::None;
        }
      }
    }
  }
}
pub fn belt_connect_left_if_either_has_port_fix_partial(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) {
  // Fixes partial because it doesn't fix the coord meta, nor .ins and .outs, nor .prio

  if let Some(ocoord) = factory.floor[coord].coord_l {
    log(format!("      - connect_left_if_either_has_port_fix_partial({:?}) from:{:?} to:{:?}", factory.floor[coord].kind, factory.floor[coord].port_l, factory.floor[ocoord].port_r));

    match factory.floor[ocoord].kind {
      CellKind::Belt => {
        // check if this cell has a port up, check if upper cell has port going down
        // make sure the opposite cell also has a port and that they are in sync
        // the pasted cell leads, if it has a direction, pick that. if not, pick
        // the direction from the neighbor cell, if not but one has a port then
        // make them both unknown, and otherwise they will are already be set to none.
             if factory.floor[coord].port_l == Port::Inbound {
          if factory.floor[ocoord].port_r != Port::Outbound {
            log(format!("        - Setting Left.port_r to outbound"));
            factory.floor[ocoord].port_r = Port::Outbound;
            belt_discover_ins_and_outs(factory, ocoord);
            fix_belt_meta(factory, ocoord);
          }
        }
        else if factory.floor[coord].port_l == Port::Outbound {
          if factory.floor[ocoord].port_r != Port::Inbound {
            log(format!("        - Setting Left.port_r to inbound"));
            factory.floor[ocoord].port_r = Port::Inbound;
            belt_discover_ins_and_outs(factory, ocoord);
            fix_belt_meta(factory, ocoord);
          }
        }
        else if factory.floor[ocoord].port_r == Port::Inbound {
          if factory.floor[coord].port_l != Port::Outbound {
            log(format!("        - Setting this.port_l to outbound"));
            factory.floor[coord].port_l = Port::Outbound;
          }
        }
        else if factory.floor[ocoord].port_r == Port::Outbound {
          if factory.floor[coord].port_l != Port::Inbound {
            log(format!("        - Setting this.port_l to inbound"));
            factory.floor[coord].port_l = Port::Inbound;
          }
        }
        else if (factory.floor[coord].port_l == Port::Unknown) != (factory.floor[ocoord].port_r == Port::Unknown) {
          log(format!("        - Setting both ports to unknown"));
          factory.floor[coord].port_l = Port::Unknown;
          factory.floor[ocoord].port_r = Port::Unknown;
          belt_discover_ins_and_outs(factory, ocoord);
          fix_belt_meta(factory, ocoord);
        }
        else {
          log(format!("        - both ports are already unknown or none; {:?} {:?}", factory.floor[coord].port_l, factory.floor[ocoord].port_r));
          assert!(
            (factory.floor[coord].port_l == Port::Unknown && factory.floor[ocoord].port_r == Port::Unknown) ||
            (factory.floor[coord].port_l == Port::None    && factory.floor[ocoord].port_r == Port::None),
            "either both ports are already Unknown or both ports are None {:?} {:?}",
            factory.floor[coord].port_l,
            factory.floor[ocoord].port_r
          );
        }
      }
      CellKind::Supply => {
        // Can only be one way and not sure why we wouldn't connect it immediately
        log(format!("        - Left is supply so this.port_l must be inbound"));
        factory.floor[coord].port_l = Port::Inbound;
      }
      CellKind::Demand  => {
        // Can only be one way and not sure why we wouldn't connect it immediately
        log(format!("        - Left is demand so this.port_l must be outbound"));
        factory.floor[coord].port_l = Port::Outbound;
      }
      CellKind::Machine => {
        // This only depends on the belt since the machine port is auto
        log(format!("        - Left is machine so it depends on this port: {:?}", factory.floor[coord].port_l));

        match factory.floor[coord].port_l {
          Port::Inbound => {
            factory.floor[ocoord].port_r = Port::Outbound;
          }
          Port::Outbound => {
            factory.floor[ocoord].port_r = Port::Inbound;
          }
          Port::Unknown => {
            factory.floor[ocoord].port_r = Port::Unknown;
          }
          Port::None => {
            factory.floor[ocoord].port_r = Port::None;
          }
        }

        let main_coord = factory.floor[ocoord].machine.main_coord;
        machine_discover_ins_and_outs(factory, main_coord);
        fix_belt_meta(factory, ocoord);
      }
      CellKind::Empty => {
        log(format!("        - Left is empty so clear this port_l, which was {:?}", factory.floor[coord].port_l));

        if factory.floor[coord].port_l != Port::None {
          factory.floor[coord].port_l = Port::None;
        }
      }
    }
  }
}

pub fn belt_fix_semi_connected_ports_partial(options: &Options, state: &State, factory: &mut Factory, coord: usize) {
  // Note: Partial because it _only_ change the port values
  // For each port;
  // - if one side is empty, force the other side to empty
  // - if one side is unknown, and the other side is known, follow the other side
  // - else both sides must be in or out, force the other side to be the opposite port of this side

  log(format!("belt_fix_semi_connected_ports_partial({})", coord));

  belt_fix_semi_connected_port_u(options, state, factory, coord);
  belt_fix_semi_connected_port_r(options, state, factory, coord);
  belt_fix_semi_connected_port_d(options, state, factory, coord);
  belt_fix_semi_connected_port_l(options, state, factory, coord);
}

fn belt_fix_semi_connected_port_u(options: &Options, state: &State, factory: &mut Factory, coord: usize) {
  if let Some(ocoord) = factory.floor[coord].coord_u {
    // If either side is empty cell or has no port the other way, clear the other port
    if factory.floor[coord].kind == CellKind::Empty || factory.floor[coord].port_u == Port::None || factory.floor[ocoord].kind == CellKind::Empty || factory.floor[ocoord].port_d == Port::None {
      factory.floor[coord].port_u = Port::None;
      factory.floor[ocoord].port_d = Port::None;
    }
    // If curr is unknown while the other is known, follow the other
    else if factory.floor[coord].port_u == Port::Unknown && factory.floor[ocoord].port_d != Port::Unknown {
      factory.floor[coord].port_u = flip(factory.floor[ocoord].port_d);
    }
    // If curr is known while the other is unknown, follow curr
    else if factory.floor[coord].port_u != Port::Unknown && factory.floor[ocoord].port_d == Port::Unknown {
      factory.floor[ocoord].port_d = flip(factory.floor[coord].port_u);
    }
    // If both are unknown, leave them
    else if factory.floor[coord].port_u == Port::Unknown && factory.floor[ocoord].port_d == Port::Unknown {
      // noop
    }
    // The only case left is in<>out
    else if factory.floor[coord].port_u == factory.floor[ocoord].port_d {
      // Both ports are `in` or `out`. Follow the curr and flip the opposite side
      factory.floor[ocoord].port_d = flip(factory.floor[coord].port_u);
    }
    // Otherwise the ports are already in and out so noop
    else {
      assert!(factory.floor[coord].port_u == Port::Inbound || factory.floor[coord].port_u == Port::Outbound);
      assert!(factory.floor[ocoord].port_d == Port::Inbound || factory.floor[ocoord].port_d == Port::Outbound);
      assert_ne!(factory.floor[coord].port_u, factory.floor[ocoord].port_d);
    }
  } else {
    // Other side is oob. That means we put an outbound port to a Demand or Supply, oops?
    panic!("This means a Demand or Supply had a port to OOB, oops?");
  }
}
fn belt_fix_semi_connected_port_r(options: &Options, state: &State, factory: &mut Factory, coord: usize) {
  if let Some(ocoord) = factory.floor[coord].coord_r {
    // If either side is empty cell or has no port the other way, clear the other port
    if factory.floor[coord].kind == CellKind::Empty || factory.floor[coord].port_r == Port::None || factory.floor[ocoord].kind == CellKind::Empty || factory.floor[ocoord].port_l == Port::None {
      factory.floor[coord].port_r = Port::None;
      factory.floor[ocoord].port_l = Port::None;
    }
    // If curr is unknown while the other is known, follow the other
    else if factory.floor[coord].port_r == Port::Unknown && factory.floor[ocoord].port_l != Port::Unknown {
      factory.floor[coord].port_r = flip(factory.floor[ocoord].port_l);
    }
    // If curr is known while the other is unknown, follow curr
    else if factory.floor[coord].port_r != Port::Unknown && factory.floor[ocoord].port_l == Port::Unknown {
      factory.floor[ocoord].port_l = flip(factory.floor[coord].port_r);
    }
    // If both are unknown, leave them
    else if factory.floor[coord].port_r == Port::Unknown && factory.floor[ocoord].port_l == Port::Unknown {
      // noop
    }
    // The only case left is in<>out
    else if factory.floor[coord].port_r == factory.floor[ocoord].port_l {
      // Both ports are `in` or `out`. Follow the curr and flip the opposite side
      factory.floor[ocoord].port_l = flip(factory.floor[coord].port_r);
    }
    // Otherwise the ports are already in and out so noop
    else {
      assert!(factory.floor[coord].port_r == Port::Inbound || factory.floor[coord].port_r == Port::Outbound);
      assert!(factory.floor[ocoord].port_l == Port::Inbound || factory.floor[ocoord].port_l == Port::Outbound);
      assert_ne!(factory.floor[coord].port_r, factory.floor[ocoord].port_l);
    }
  } else {
    // Other side is oob. That means we put an outbound port to a Demand or Supply, oops?
    panic!("This means a Demand or Supply had a port to OOB, oops?");
  }
}
fn belt_fix_semi_connected_port_d(options: &Options, state: &State, factory: &mut Factory, coord: usize) {
  if let Some(ocoord) = factory.floor[coord].coord_d {
    // If either side is empty cell or has no port the other way, clear the other port
    if factory.floor[coord].kind == CellKind::Empty || factory.floor[coord].port_d == Port::None || factory.floor[ocoord].kind == CellKind::Empty || factory.floor[ocoord].port_u == Port::None {
      factory.floor[coord].port_d = Port::None;
      factory.floor[ocoord].port_u = Port::None;
    }
    // If curr is unknown while the other is known, follow the other
    else if factory.floor[coord].port_d == Port::Unknown && factory.floor[ocoord].port_u != Port::Unknown {
      factory.floor[coord].port_d = flip(factory.floor[ocoord].port_u);
    }
    // If curr is known while the other is unknown, follow curr
    else if factory.floor[coord].port_d != Port::Unknown && factory.floor[ocoord].port_u == Port::Unknown {
      factory.floor[ocoord].port_u = flip(factory.floor[coord].port_d);
    }
    // If both are unknown, leave them
    else if factory.floor[coord].port_d == Port::Unknown && factory.floor[ocoord].port_u == Port::Unknown {
      // noop
    }
    // The only case left is in<>out
    else if factory.floor[coord].port_d == factory.floor[ocoord].port_u {
      // Both ports are `in` or `out`. Follow the curr and flip the opposite side
      factory.floor[ocoord].port_u = flip(factory.floor[coord].port_d);
    }
    // Otherwise the ports are already in and out so noop
    else {
      assert!(factory.floor[coord].port_d == Port::Inbound || factory.floor[coord].port_d == Port::Outbound);
      assert!(factory.floor[ocoord].port_u == Port::Inbound || factory.floor[ocoord].port_u == Port::Outbound);
      assert_ne!(factory.floor[coord].port_d, factory.floor[ocoord].port_u);
    }
  } else {
    // Other side is oob. That means we put an outbound port to a Demand or Supply, oops?
    panic!("This means a Demand or Supply had a port to OOB, oops?");
  }
}
fn belt_fix_semi_connected_port_l(options: &Options, state: &State, factory: &mut Factory, coord: usize) {
  if let Some(ocoord) = factory.floor[coord].coord_l {
    // If either side is empty cell or has no port the other way, clear the other port
    if factory.floor[coord].kind == CellKind::Empty || factory.floor[coord].port_l == Port::None || factory.floor[ocoord].kind == CellKind::Empty || factory.floor[ocoord].port_r == Port::None {
      factory.floor[coord].port_l = Port::None;
      factory.floor[ocoord].port_r = Port::None;
    }
    // If curr is unknown while the other is known, follow the other
    else if factory.floor[coord].port_l == Port::Unknown && factory.floor[ocoord].port_r != Port::Unknown {
      factory.floor[coord].port_l = flip(factory.floor[ocoord].port_r);
    }
    // If curr is known while the other is unknown, follow curr
    else if factory.floor[coord].port_l != Port::Unknown && factory.floor[ocoord].port_r == Port::Unknown {
      factory.floor[ocoord].port_r = flip(factory.floor[coord].port_l);
    }
    // If both are unknown, leave them
    else if factory.floor[coord].port_l == Port::Unknown && factory.floor[ocoord].port_r == Port::Unknown {
      // noop
    }
    // The only case left is in<>out
    else if factory.floor[coord].port_l == factory.floor[ocoord].port_r {
      // Both ports are `in` or `out`. Follow the curr and flip the opposite side
      factory.floor[ocoord].port_r = flip(factory.floor[coord].port_l);
    }
    // Otherwise the ports are already in and out so noop
    else {
      assert!(factory.floor[coord].port_l == Port::Inbound || factory.floor[coord].port_l == Port::Outbound);
      assert!(factory.floor[ocoord].port_r == Port::Inbound || factory.floor[ocoord].port_r == Port::Outbound);
      assert_ne!(factory.floor[coord].port_l, factory.floor[ocoord].port_r);
    }
  } else {
    // Other side is oob. That means we put an outbound port to a Demand or Supply, oops?
    panic!("This means a Demand or Supply had a port to OOB, oops?");
  }
}

fn flip(port: Port) -> Port {
  match port {
    Port::Inbound => Port::Outbound,
    Port::Outbound => Port::Outbound,
    Port::Unknown => Port::Unknown,
    Port::None => Port::None,
  }
}
