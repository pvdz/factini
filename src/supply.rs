use std::collections::VecDeque;

use super::belt::*;
use super::cell::*;
use super::floor::*;
use super::factory::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::segment::*;
use super::state::*;

pub fn tick_supply(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) {
  let w = factory.floor.width + 2;
  let h = factory.floor.width + 2;

  factory.floor.cells[coord].ticks += 1;


  let debug = false; // factory.floor.cells[coord].speed == 10;

  if debug { println!("({}): tick_supply({},{},{},{:?},part_at:{},out_at:{},speed:{},progress:{})", factory.ticks, coord, factory.floor.cells[coord].x, factory.floor.cells[coord].y, factory.floor.cells[coord].supply_part.kind, factory.floor.cells[coord].supply_part_at, factory.floor.cells[coord].supply_last_part_out_at, factory.floor.cells[coord].speed, factory.ticks - factory.floor.cells[coord].supply_part_at); }

  if factory.floor.cells[coord].supply_part.kind == PartKind::None {
    // This supply has space, has there been enough time since the last part?
    let last_at = factory.floor.cells[coord].supply_last_part_out_at;
    let create = last_at == 0 || (factory.ticks - last_at) >= factory.floor.cells[coord].supply_interval;
    if debug { println!("  - supply is empty... last out at {}, time since last out {}, delay time: {}, delay left: {}, create new? {}", last_at, factory.ticks - last_at, factory.floor.cells[coord].supply_interval, (factory.floor.cells[coord].supply_interval as f64) - (factory.ticks - last_at) as f64, create); }
    if create {
      if options.print_moves || options.print_moves_supply { println!("({}) creating part in supply at {} {}...", factory.ticks, factory.floor.cells[coord].x, factory.floor.cells[coord].y); }
      // Create new part
      factory.floor.cells[coord].supply_part = factory.floor.cells[coord].supply_gives.clone();
      factory.floor.cells[coord].supply_part_at = factory.ticks;
    }
  } else if factory.ticks - factory.floor.cells[coord].supply_part_at >= factory.floor.cells[coord].speed {
    // println!("  - this part is ready to leave the supply...");
    // Have an actual part that's at the exit of the supply
    // If there's an edge cell then it will be at the same coordinate

    // Now we need to determine where the exit is so we can check the correct segment
    let (x, y) = to_xy(coord, w);

    // Get opposite neighbor coord
    let (ocoord, odir) = if x == 0 {
      (to_coord_right(coord, w), SegmentDirection::LEFT)
    } else if y == 0 {
      (to_coord_down(coord, w), SegmentDirection::UP)
    } else if x == w - 1 {
      (to_coord_left(coord, w), SegmentDirection::RIGHT)
    } else if y == h - 1 {
      (to_coord_up(coord, w), SegmentDirection::DOWN)
    } else {
      panic!("supply must be one edge");
    };

    // Supplies can only supply to belts. Ignore machines and empty cells (part will be stuck)
    if factory.floor.cells[ocoord].kind == CellKind::Belt {
      if can_move_part_from_supply_to_cell(factory, ocoord, odir) {
        // if options.print_moves || options.print_moves_supply { println!("({}) Handed from up supply to belt below. Waiting {} ticks to create a new part.", factory.ticks, factory.floor.cells[coord].speed); }
        move_part_from_supply_to_segment(options, state, factory, coord, ocoord, odir);
        add_price_delta(options, state, &mut factory.stats, factory.ticks, factory.floor.cells[coord].supply_part_price);
      } else {
        if debug { println!("  - connected belt is blocked... unable to move part right now"); }
      }
    }
  }
}
