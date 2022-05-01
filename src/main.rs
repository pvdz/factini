use std::collections::VecDeque;

use crate::belt::{BELT_NONE, BeltDirection};
use crate::cell::CellKind;
use crate::part::{part_none, PartKind};

pub mod belt;
pub mod cell;
pub mod demand;
pub mod factory;
pub mod floor;
pub mod machine;
pub mod options;
pub mod part;
pub mod state;
pub mod supply;

const ONE_SECOND: u64 = 10000;

const OUT: u8 = 0;
const CENTER: u8 = 1;
const IN: u8 = 2;

fn should_be_marked(cell: &cell::Cell, cell_u: &cell::Cell, cell_r: &cell::Cell, cell_d: &cell::Cell, cell_l: &cell::Cell) -> bool {
  if cell.marked {
    return false;
  }

  match cell.kind {
    CellKind::Empty => return false,
    CellKind::Belt => {
      // All ports are
      // - connected to an empty cell, or
      // - not outports, or
      // - connected to marked cells, or
      // - not connected to inports and not connected to machine

      return
        (cell_u.marked || matches!(cell_u.kind, CellKind::Empty) || !matches!(cell.belt.direction_u, BeltDirection::Out) || (matches!(cell_u.kind, CellKind::Belt) && !matches!(cell_u.belt.direction_d, BeltDirection::In))) &&
        (cell_r.marked || matches!(cell_r.kind, CellKind::Empty) || !matches!(cell.belt.direction_r, BeltDirection::Out) || (matches!(cell_r.kind, CellKind::Belt) && !matches!(cell_r.belt.direction_l, BeltDirection::In))) &&
        (cell_d.marked || matches!(cell_d.kind, CellKind::Empty) || !matches!(cell.belt.direction_d, BeltDirection::Out) || (matches!(cell_d.kind, CellKind::Belt) && !matches!(cell_d.belt.direction_u, BeltDirection::In))) &&
        (cell_l.marked || matches!(cell_l.kind, CellKind::Empty) || !matches!(cell.belt.direction_l, BeltDirection::Out) || (matches!(cell_l.kind, CellKind::Belt) && !matches!(cell_l.belt.direction_r, BeltDirection::In)))
      ;
    },
    CellKind::Machine => {
      // All ports are
      // - connected to an empty cell, or
      // - connected to marked cells, or
      // - connected to another machine (which does not work), or
      // - not connected to inports

      return
        (cell_u.marked || !matches!(cell_u.kind, CellKind::Belt) || (matches!(cell_u.kind, CellKind::Belt) && !matches!(cell_u.belt.direction_d, BeltDirection::In))) &&
        (cell_r.marked || !matches!(cell_r.kind, CellKind::Belt) || (matches!(cell_r.kind, CellKind::Belt) && !matches!(cell_r.belt.direction_l, BeltDirection::In))) &&
        (cell_d.marked || !matches!(cell_d.kind, CellKind::Belt) || (matches!(cell_d.kind, CellKind::Belt) && !matches!(cell_d.belt.direction_u, BeltDirection::In))) &&
        (cell_l.marked || !matches!(cell_l.kind, CellKind::Belt) || (matches!(cell_l.kind, CellKind::Belt) && !matches!(cell_l.belt.direction_r, BeltDirection::In)))
      ;
    }
  }
}

fn sort_cells(factory: &mut factory::Factory) -> Vec<(usize, usize)> {
  // Collect cells by marking them and putting their coords in a vec. In the end the vec must have
  // all non-empty cells and should tick in that order. This way you work around the belt wanting
  // to unload onto another belt that is currently full but would be empty after this tick as well.
  //
  // While there are tiles left;
  // - start with unprocessed Demands
  //   - mark all their neighbors with outgoing paths to this Demand
  // - while there are cells not in the list yet
  //   - find all cells where all outgoing paths are connected to marked cells or not at all
  //     - mark them and put them in the list
  // - while there are still unprocessed cells left
  //   - pick a random one. maybe prefer
  //     - ones connected to at least one marked cell
  //     - not connected to suppliers
  //     - pick furthest distance to supplier?

  let mut cell_none = cell::empty_cell(0, 0);

  let mut out: Vec<(usize, usize)> = vec!();

  // println!("sort_cells\n- start with demanders");
  for demand in &factory.demanders {
    let x = demand.x;
    let y = demand.y;
    // The cell will exist, regardless of whether it's a belt, machine or none cell.
    factory.floor.cells[x][y].marked = true;
    out.push((x, y));
  }

  // println!("- iteratively follow the trail");
  let mut found_something = true;
  let mut some_left = true;
  let w = factory.floor.width;
  let h = factory.floor.height;
  while found_something && some_left {
    // println!("  - loop");
    found_something = false;
    for x in 0..w {
      for y in 0..h {
        if should_be_marked(
          &factory.floor.cells[x][y],
          if y == 0 { &cell_none } else { &factory.floor.cells[x][y-1] },
          if x == w-1 { &cell_none } else { &factory.floor.cells[x+1][y] },
          if y == h-1 { &cell_none } else { &factory.floor.cells[x][y+1] },
          if x == 0 { &cell_none } else { &factory.floor.cells[x-1][y] },
        ) {
          // println!("    - adding {} {}", x, y);
          factory.floor.cells[x][y].marked = true;
          out.push((x, y));
          found_something = true;
        } else {
          some_left = true;
        }
      }
    }
  }

  // println!("- now add the remaining {} in random order", 25-out.len());
  for x in 0..factory.floor.width {
    for y in 0..factory.floor.height {
      if !matches!(factory.floor.cells[x][y].kind, CellKind::Empty) {
        if !factory.floor.cells[x][y].marked {
          out.push((x, y));
          // println!("- adding {} {}", x, y);
        } else {
          // println!("- skipping {} {} because marked", x, y);
        }
      } else {
        // println!("- skipping {} {} because empty", x, y);
      }
    }
  }

  return out;
}

fn main() {
  println!("start");

  let mut options = options::Options {
    print_moves: false,
    print_moves_belt: false,
    print_moves_machine: false,
    print_moves_supply: false,
    print_moves_demand: false,
    print_price_deltas: false,
    print_factory_interval: 10000,
    short_term_window: 10000,
    long_term_window: 600000,
  };

  let mut state = state::State {};

  let mut factory = factory::create_factory(&mut options, &mut state);

  let prio: Vec<(usize, usize)> = sort_cells(&mut factory);
  println!("The prio: {:?}", prio);

  // Do not record the cost of belt cells. assume them an ongoing 10k x belt cost cost/min modifier
  // Only record the non-belt costs, which happen far less frequently and mean the delta queue
  // will be less than 100 items. Probably slightly under 50, depending on how we tweak speeds.
  // Even 100 items seems well within acceptable ranges. We could even track 10s (1k items) which
  // might be useful to set consistency thresholds ("you need to maintain this efficiency for at
  // least 10s").


  let mut stats: (VecDeque<(i32, u64)>, usize, i32, i32, i32) = (
    VecDeque::new(), // vec<(cost_delta, at_tick)> // Up to 100k ticks worth of cost deltas
    0, // one_second_size // Number of elements in cost_deltas that wrap the last 10k ticks
    0, // one_second_total // Efficiency of last 10k ticks
    0, // ten_second_total // Efficiency of last 100k ticks (total of cost_deltas)
    14, // number of belt cells
  );

  let mut ticks: u64 = 0;
  while ticks < (120 * ONE_SECOND) {
    ticks += 1;
    tick(&mut options, &mut state, &mut factory, ticks, &prio, &mut stats);

    if (ticks % ONE_SECOND) == 0 {
      println!("- {}; efficiency: short period: {}, long period: {} {:100}", ticks, stats.2 + (14 * -1000), stats.3 + (14 * -10000), ' ');
    }

    // Update second total by shrinking the window until the oldest entry inside it is at most 10k ticks old
    let pre_len = stats.0.len();
    while stats.1 > 0 && (ticks - stats.0[pre_len - stats.1].1) > options.short_term_window {
      let delta = stats.0[pre_len - stats.1].0;
      stats.1 -= 1;
      stats.2 -= (delta as i32);
    }

    // Update 10 second total by shrinking the deltas until the oldest entry is at most 100k ticks old
    while stats.0.len() > 0 && (ticks - stats.0[0].1) > options.long_term_window {
      stats.3 -= (stats.0[0].0 as i32);
      stats.0.pop_front();
    }

    if (ticks % options.print_factory_interval) == 0 {
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      // println!("{:200}", ' ');
      println!("{:200}", ' ');
      println!("{:200}", ' ');
      println!("{}", factory::serialize_cli(&factory));
      // print!("\x1b[{}A\n", 60);
    }
  }

}

fn is_current_phase(dir: BeltDirection, stage: u8) -> bool {
  return match dir {
    BeltDirection::In => stage == IN,
    BeltDirection::Out => stage == OUT,
    BeltDirection::None => false,
  };
}

fn add_price_delta(options: &mut options::Options, _state: &mut state::State, stats: &mut (VecDeque<(i32, u64)>, usize, i32, i32, i32), ticks: u64, delta: i32) {
  if options.print_price_deltas { println!("  - add_price_delta: {}, 1sec: {}, 10sec: {}", delta, stats.2 + (delta as i32), stats.3 + (delta as i32)); }
  stats.0.push_back((delta, ticks));
  stats.1 += 1; // new entry should not be older than 10k ticks ;)
  stats.2 += delta as i32;
  stats.3 += delta as i32;
}

fn tick(options: &mut options::Options, state: &mut state::State, factory: &mut factory::Factory, ticks: u64, prio: &Vec<(usize, usize)>, stats: &mut (VecDeque<(i32, u64)>, usize, i32, i32, i32)) {
  for (x, y) in prio.to_vec() {
    // TODO: Is this one even useful?
    factory.floor.cells[x][y].ticks += 1;

    match factory.floor.cells[x][y].kind {
      CellKind::Empty => {

      }
      CellKind::Belt => {
        let mut stage: u8 = 0; // 0 = outgoing segments, 1 = center, 2 = incoming segments

        // By rotating the segment to start with, we can indirectly control an alternating flow
        // of objects when they are competing with other segments to give from or receive to
        // the segment in the center. If we always started from zero then in the same cadence
        // the same segment would always win and the other segments never get a chance.
        let current_segment_index: u8 = factory.floor.cells[x][y].offset_segment;
        factory.floor.cells[x][y].offset_segment = ((current_segment_index + 1) % 4);

        // Loop three times, each iteration considers only segments of that kind
        while stage <= 2 {

          if stage != CENTER {

            // Go through all segment edges, rotate the start
            let mut segments_checked: u8 = 0;
            let mut segment_index = current_segment_index;
            while segments_checked < 4 {
              segment_index += 1;
              if segment_index >= 4 {
                segment_index = 0;
              }

              // Check if any item would move off this cell. If so, check if it can move to the neighbor
              if
                segment_index == 0 &&
                is_current_phase(factory.floor.cells[x][y].belt.direction_u, stage) &&
                !matches!(factory.floor.cells[x][y].segment_u_part.kind, PartKind::None)
              {
                if (ticks - factory.floor.cells[x][y].segment_u_at) >= factory.floor.cells[x][y].speed {
                  // The part reached the other side of this segment. If possible it should be handed
                  // off to the next segment, machine, or demand.
                  match factory.floor.cells[x][y].belt.direction_u {
                    BeltDirection::In => {
                      // Hand it off to the center segment
                      if matches!(factory.floor.cells[x][y].segment_c_part.kind, PartKind::None) {
                        factory.floor.cells[x][y].segment_c_part = factory.floor.cells[x][y].segment_u_part.clone();
                        factory.floor.cells[x][y].segment_c_at = ticks;
                        factory.floor.cells[x][y].segment_u_part = part_none(x, y);
                        if options.print_moves || options.print_moves_belt { println!("{} Handed from up segment to center segment", ticks); }
                      }
                      // else do nothing. The part is stuck because the center segment already has a part
                    }
                    BeltDirection::Out => {
                      // Hand it off to the neighbor cell above this cell
                      if y == 0 {
                        // This must be a Demand or otherwise this is a noop
                        for demand in &factory.demanders {
                          if demand.x == x && demand.y == y {
                            // Sell the part off to the Demand
                            let kind = factory.floor.cells[x][y].segment_u_part.kind;
                            if matches!(demand.part_kind, kind) {
                              if options.print_moves || options.print_moves_demand { println!("{} Sold a {:?} from up segment to Demand, price: {}", ticks, kind, demand.part_price); }
                              add_price_delta(options, state, stats, ticks, demand.part_price);
                            } else {
                              if options.print_moves || options.print_moves_demand { println!("{} Trashed a {:?} from up segment to Demand, price: {}", ticks, kind, demand.trash_price); }
                              add_price_delta(options, state, stats, ticks, demand.trash_price);
                            }
                            factory.floor.cells[x][y].segment_u_part = part_none(x, y);
                            break;
                          }
                        }
                      } else {
                        match factory.floor.cells[x][y-1].kind {
                          CellKind::Empty => {
                            // noop
                          }
                          CellKind::Belt => {
                            if
                              // Check if neighbor belt actually takes from this side
                              matches!(factory.floor.cells[x][y-1].belt.direction_d, BeltDirection::In) &&
                              // The part would be put on the down segment so check if that's available
                              matches!(factory.floor.cells[x][y-1].segment_d_part.kind, PartKind::None)
                            {
                              factory.floor.cells[x][y-1].segment_d_part = factory.floor.cells[x][y].segment_u_part.clone();
                              factory.floor.cells[x][y-1].segment_d_at = ticks;
                              factory.floor.cells[x][y].segment_u_part = part_none(x, y);
                              if options.print_moves || options.print_moves_belt { println!("{} Handed from up segment to neighbor down segment", ticks); }
                            }
                            // else do nothing; there is no belt above this cell, or that belt has no
                            //                  intake from down, or its down segment already has a part.
                          }
                          CellKind::Machine => {
                            // Process item in machine
                            // Check if machine is still waiting for this part at least once
                            let segment_kind = factory.floor.cells[x][y].segment_u_part.kind;
                            if
                              matches!(factory.floor.cells[x][y-1].machine_input_1_want.kind, segment_kind) &&
                              matches!(factory.floor.cells[x][y-1].machine_input_1_have.kind, PartKind::None)
                            {
                              factory.floor.cells[x][y-1].machine_input_1_have = factory.floor.cells[x][y].segment_u_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 1", ticks); }
                            }
                            else if
                              matches!(factory.floor.cells[x][y-1].machine_input_2_want.kind, segment_kind) &&
                              matches!(factory.floor.cells[x][y-1].machine_input_2_have.kind, PartKind::None)
                            {
                              factory.floor.cells[x][y-1].machine_input_2_have = factory.floor.cells[x][y].segment_u_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 2", ticks); }
                            }
                            else if
                              matches!(factory.floor.cells[x][y-1].machine_input_3_want.kind, segment_kind) &&
                              matches!(factory.floor.cells[x][y-1].machine_input_3_have.kind, PartKind::None)
                            {
                              factory.floor.cells[x][y-1].machine_input_3_have = factory.floor.cells[x][y].segment_u_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 3", ticks); }
                            } else {
                              // Destruct part
                              if options.print_moves || options.print_moves_machine { println!("{} Machine did not need part, it was trashed, cost: {}", ticks, factory.floor.cells[x][y-1].machine_trash_price); }
                              add_price_delta(options, state, stats, ticks, factory.floor.cells[x][y-1].machine_trash_price);
                            }
                            factory.floor.cells[x][y].segment_u_part = part_none(x, y);
                            if options.print_moves || options.print_moves_machine { println!("{} Handed from up segment to neighbor machine", ticks); }
                          }
                        }
                      }
                    }
                    BeltDirection::None => {
                      panic!("If there's an u segment then there should be an upper in or out; {:?}", factory.floor.cells[x][y].belt);
                    }
                  }
                }
                // else part is not at any edge. no need to do anything here
              }

              if
                segment_index == 1 &&
                is_current_phase(factory.floor.cells[x][y].belt.direction_r, stage) &&
                !matches!(factory.floor.cells[x][y].segment_r_part.kind, PartKind::None)
              {
                if (ticks - factory.floor.cells[x][y].segment_r_at) >= factory.floor.cells[x][y].speed {
                  // The part reached the other side of this segment. If possible it should be handed
                  // off to the next segment, machine, or demand.
                  match factory.floor.cells[x][y].belt.direction_r {
                    BeltDirection::In => {
                      // Hand it off to the center segment
                      if matches!(factory.floor.cells[x][y].segment_c_part.kind, PartKind::None) {
                        factory.floor.cells[x][y].segment_c_part = factory.floor.cells[x][y].segment_r_part.clone();
                        factory.floor.cells[x][y].segment_c_at = ticks;
                        factory.floor.cells[x][y].segment_r_part = part_none(x, y);
                        if options.print_moves || options.print_moves_belt { println!("{} Handed from right segment to center segment", ticks); }
                      }
                      // else do nothing. The part is stuck because the center segment already has a part
                    }
                    BeltDirection::Out => {
                      // Hand it off to the neighbor cell to the right of this cell
                      if x >= factory.floor.width - 1 {
                        // This must be a Demand or otherwise this is a noop
                        for demand in &factory.demanders {
                          if demand.x == x && demand.y == y {
                            // Sell the part off to the Demand
                            let kind = factory.floor.cells[x][y].segment_r_part.kind;
                            if matches!(demand.part_kind, kind) {
                              println!("{} Sold a {:?} from left segment to Demand, price: {}", ticks, kind, demand.part_price);
                              add_price_delta(options, state, stats, ticks, demand.part_price);
                            } else {
                              println!("{} Trashed a {:?} from left segment to Demand, price: {}", ticks, kind, demand.trash_price);
                              add_price_delta(options, state, stats, ticks, demand.trash_price);
                            }
                            factory.floor.cells[x][y].segment_r_part = part_none(x, y);
                            break;
                          }
                        }
                      } else {
                        match factory.floor.cells[x+1][y].kind {
                          CellKind::Empty => {
                            // noop
                          }
                          CellKind::Belt => {
                            if
                              // Check if neighbor belt actually takes from this side
                              matches!(factory.floor.cells[x+1][y].belt.direction_l, BeltDirection::In) &&
                              // The part would be put on the left segment of the right belt so check if that's available
                              matches!(factory.floor.cells[x+1][y].segment_l_part.kind, PartKind::None)
                            {
                              factory.floor.cells[x+1][y].segment_l_part = factory.floor.cells[x][y].segment_r_part.clone();
                              factory.floor.cells[x+1][y].segment_l_at = ticks;
                              factory.floor.cells[x][y].segment_r_part = part_none(x, y);
                              if options.print_moves || options.print_moves_belt { println!("{} Handed from right segment to neighbor left segment", ticks); }
                            }
                            // else do nothing; there is no belt right to this cell, or that belt has no
                            //                  intake from left, or its left segment already has a part.
                          }
                          CellKind::Machine => {
                            // Process item in machine
                            // Check if machine is still waiting for this part at least once
                            let segment_kind = factory.floor.cells[x][y].segment_r_part.kind;
                            if
                              matches!(factory.floor.cells[x+1][y].machine_input_1_want.kind, segment_kind) &&
                              matches!(factory.floor.cells[x+1][y].machine_input_1_have.kind, PartKind::None)
                            {
                              factory.floor.cells[x+1][y].machine_input_1_have = factory.floor.cells[x][y].segment_r_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 1", ticks); }
                            }
                            else if
                              matches!(factory.floor.cells[x+1][y].machine_input_2_want.kind, segment_kind) &&
                              matches!(factory.floor.cells[x+1][y].machine_input_2_have.kind, PartKind::None)
                            {
                              factory.floor.cells[x+1][y].machine_input_2_have = factory.floor.cells[x][y].segment_r_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 2", ticks); }
                            }
                            else if
                              matches!(factory.floor.cells[x+1][y].machine_input_3_want.kind, segment_kind) &&
                              matches!(factory.floor.cells[x+1][y].machine_input_3_have.kind, PartKind::None)
                            {
                              factory.floor.cells[x+1][y].machine_input_3_have = factory.floor.cells[x][y].segment_r_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 3", ticks); }
                            } else {
                              // Destruct part
                              if options.print_moves || options.print_moves_machine { println!("{} Machine did not need part, it was trashed, cost: {}", ticks, factory.floor.cells[x+1][y].machine_trash_price); }
                              add_price_delta(options, state, stats, ticks, factory.floor.cells[x+1][y].machine_trash_price);
                            }

                            factory.floor.cells[x][y].segment_r_part = part_none(x, y);
                            if options.print_moves || options.print_moves_machine { println!("{} Handed from right segment to neighbor machine", ticks); }
                          }
                        }
                      }
                    }
                    BeltDirection::None => {
                      panic!("If there's an r segment then there should be a right in or out; {:?}", factory.floor.cells[x][y].belt);
                    }
                  }
                }
                // else part is not at any edge. no need to do anything here
              }

              if
                segment_index == 2 &&
                is_current_phase(factory.floor.cells[x][y].belt.direction_d, stage) &&
                !matches!(factory.floor.cells[x][y].segment_d_part.kind, PartKind::None)
              {
                if (ticks - factory.floor.cells[x][y].segment_d_at) >= factory.floor.cells[x][y].speed {
                  // The part reached the other side of this segment. If possible it should be handed
                  // off to the next segment, machine, or demand.
                  match factory.floor.cells[x][y].belt.direction_d {
                    BeltDirection::In => {
                      // Hand it off to the center segment
                      if matches!(factory.floor.cells[x][y].segment_c_part.kind, PartKind::None) {
                        factory.floor.cells[x][y].segment_c_part = factory.floor.cells[x][y].segment_d_part.clone();
                        factory.floor.cells[x][y].segment_c_at = ticks;
                        factory.floor.cells[x][y].segment_d_part = part_none(x, y);
                        if options.print_moves || options.print_moves_belt { println!("{} Handed from down segment to center segment", ticks); }
                      }
                      // else do nothing. The part is stuck because the center segment already has a part
                    }
                    BeltDirection::Out => {
                      // Hand it off to the neighbor cell below this cell
                      if y >= factory.floor.height - 1 {
                        // This must be a Demand or otherwise this is a noop
                        for demand in &factory.demanders {
                          if demand.x == x && demand.y == y {
                            // Sell the part off to the Demand
                            let kind = factory.floor.cells[x][y].segment_d_part.kind;
                            if matches!(demand.part_kind, kind) {
                              if options.print_moves || options.print_moves_demand { println!("{} Sold a {:?} from down segment to Demand, price: {}", ticks, kind, demand.part_price); }
                              add_price_delta(options, state, stats, ticks, demand.part_price);
                            } else {
                              if options.print_moves || options.print_moves_demand { println!("{} Trashed a {:?} from down segment to Demand, price: {}", ticks, kind, demand.trash_price); }
                              add_price_delta(options, state, stats, ticks, demand.trash_price);
                            }
                            factory.floor.cells[x][y].segment_d_part = part_none(x, y);
                            break;
                          }
                        }
                      } else {
                        match factory.floor.cells[x][y+1].kind {
                          CellKind::Empty => {
                            // noop
                          }
                          CellKind::Belt => {
                            if
                              // Check if neighbor belt actually takes from this side
                              matches!(factory.floor.cells[x][y+1].belt.direction_u, BeltDirection::In) &&
                              // The part would be put on the up segment of the below belt so check if that's available
                              matches!(factory.floor.cells[x][y+1].segment_u_part.kind, PartKind::None)
                            {
                              factory.floor.cells[x][y+1].segment_u_part = factory.floor.cells[x][y].segment_d_part.clone();
                              factory.floor.cells[x][y+1].segment_u_at = ticks;
                              factory.floor.cells[x][y].segment_d_part = part_none(x, y);
                              if options.print_moves || options.print_moves_belt { println!("{} Handed from down segment to neighbor up segment", ticks); }
                            }
                            // else do nothing; there is no belt below this cell, or that belt has no
                            //                  intake from up, or its up segment already has a part.
                          }
                          CellKind::Machine => {
                            // Process item in machine
                            // Check if machine is still waiting for this part at least once
                            let segment_kind = factory.floor.cells[x][y].segment_d_part.kind;
                            if
                              matches!(factory.floor.cells[x][y+1].machine_input_1_want.kind, segment_kind) &&
                              matches!(factory.floor.cells[x][y+1].machine_input_1_have.kind, PartKind::None)
                            {
                              factory.floor.cells[x][y+1].machine_input_1_have = factory.floor.cells[x][y].segment_d_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 1", ticks); }
                            }
                            else if
                              matches!(factory.floor.cells[x][y+1].machine_input_2_want.kind, segment_kind) &&
                              matches!(factory.floor.cells[x][y+1].machine_input_2_have.kind, PartKind::None)
                            {
                              factory.floor.cells[x][y+1].machine_input_2_have = factory.floor.cells[x][y].segment_d_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 2", ticks); }
                            }
                            else if
                              matches!(factory.floor.cells[x][y+1].machine_input_3_want.kind, segment_kind) &&
                              matches!(factory.floor.cells[x][y+1].machine_input_3_have.kind, PartKind::None)
                            {
                              factory.floor.cells[x][y+1].machine_input_3_have = factory.floor.cells[x][y].segment_d_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 3", ticks); }
                            } else {
                              // Destruct part
                              if options.print_moves || options.print_moves_machine { println!("{} Machine did not need part, it was trashed, cost: {}", ticks, factory.floor.cells[x][y+1].machine_trash_price); }
                              add_price_delta(options, state, stats, ticks, factory.floor.cells[x][y+1].machine_trash_price);
                            }

                            factory.floor.cells[x][y].segment_d_part = part_none(x, y);
                            if options.print_moves || options.print_moves_machine { println!("{} Handed from down segment to neighbor factory", ticks); }
                          }
                        }
                      }
                    }
                    BeltDirection::None => {
                      panic!("If there's an d segment then there should be a down in or out; {:?}", factory.floor.cells[x][y].belt);
                    }
                  }
                }
                // else part is not at any edge. no need to do anything here
              }

              if
                segment_index == 3 &&
                is_current_phase(factory.floor.cells[x][y].belt.direction_l, stage) &&
                !matches!(factory.floor.cells[x][y].segment_l_part.kind, PartKind::None)
              {
                if (ticks - factory.floor.cells[x][y].segment_l_at) >= factory.floor.cells[x][y].speed {
                  // The part reached the other side of this segment. If possible it should be handed
                  // off to the next segment, machine, or demand.
                  match factory.floor.cells[x][y].belt.direction_l {
                    BeltDirection::In => {
                      // Hand it off to the center segment
                      if matches!(factory.floor.cells[x][y].segment_c_part.kind, PartKind::None) {
                        factory.floor.cells[x][y].segment_c_part = factory.floor.cells[x][y].segment_l_part.clone();
                        factory.floor.cells[x][y].segment_c_at = ticks;
                        factory.floor.cells[x][y].segment_l_part = part_none(x, y);
                        if options.print_moves || options.print_moves_belt { println!("{} Handed from left segment to center segment", ticks); }
                      }
                      // else do nothing. The part is stuck because the center segment already has a part
                    }
                    BeltDirection::Out => {
                      // Hand it off to the neighbor cell below this cell
                      if x == 0 {
                        // This must be a Demand or otherwise this is a noop
                        for demand in &factory.demanders {
                          if demand.x == x && demand.y == y {
                            // Sell the part off to the Demand
                            let kind = factory.floor.cells[x][y].segment_u_part.kind;
                            if matches!(demand.part_kind, kind) {
                              println!("{} Sold a {:?} from left segment to Demand, price: {}", ticks, kind, demand.part_price);
                              add_price_delta(options, state, stats, ticks, demand.part_price);
                            } else {
                              println!("{} Trashed a {:?} from left segment to Demand, price: {}", ticks, kind, demand.trash_price);
                              add_price_delta(options, state, stats, ticks, demand.trash_price);
                            }
                            factory.floor.cells[x][y].segment_u_part = part_none(x, y);
                            break;
                          }
                        }
                      } else {
                        match factory.floor.cells[x-1][y].kind {
                          CellKind::Empty => {
                            // noop
                          }
                          CellKind::Belt => {
                            if
                              // Check if neighbor belt actually takes from this side
                              matches!(factory.floor.cells[x-1][y].belt.direction_r, BeltDirection::In) &&
                              // The part would be put on the right segment of the left belt so check if that's available
                              matches!(factory.floor.cells[x-1][y].segment_r_part.kind, PartKind::None)
                            {
                              factory.floor.cells[x-1][y].segment_r_part = factory.floor.cells[x][y].segment_l_part.clone();
                              factory.floor.cells[x-1][y].segment_r_at = ticks;
                              factory.floor.cells[x][y].segment_l_part = part_none(x, y);
                              if options.print_moves || options.print_moves_belt { println!("{} Handed from left segment to neighbor right segment", ticks); }
                            }
                            // else do nothing; there is no belt left of this cell, or that belt has no
                            //                  intake from down, or its right segment already has a part.
                          }
                          CellKind::Machine => {
                            // Process item in machine
                            // Check if machine is still waiting for this part at least once
                            let segment_kind = factory.floor.cells[x][y].segment_l_part.kind;
                            if
                              matches!(factory.floor.cells[x-1][y].machine_input_1_want.kind, segment_kind) &&
                              matches!(factory.floor.cells[x-1][y].machine_input_1_have.kind, PartKind::None)
                            {
                              factory.floor.cells[x-1][y].machine_input_1_have = factory.floor.cells[x][y].segment_l_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 1", ticks); }
                            }
                            else if
                              matches!(factory.floor.cells[x-1][y].machine_input_2_want.kind, segment_kind) &&
                              matches!(factory.floor.cells[x-1][y].machine_input_2_have.kind, PartKind::None)
                            {
                              factory.floor.cells[x-1][y].machine_input_2_have = factory.floor.cells[x][y].segment_l_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 2", ticks); }
                            }
                            else if
                              matches!(factory.floor.cells[x-1][y].machine_input_3_want.kind, segment_kind) &&
                              matches!(factory.floor.cells[x-1][y].machine_input_3_have.kind, PartKind::None)
                            {
                              factory.floor.cells[x-1][y].machine_input_3_have = factory.floor.cells[x][y].segment_l_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 3", ticks); }
                            } else {
                              // Destruct part
                              if options.print_moves || options.print_moves_machine { println!("{} Machine did not need part, it was trashed", ticks); }
                              add_price_delta(options, state, stats, ticks, factory.floor.cells[x-1][y].machine_trash_price);
                            }

                            factory.floor.cells[x][y].segment_l_part = part_none(x, y);
                            if options.print_moves || options.print_moves_machine {  println!("{} Handed from left segment to neighbor machine", ticks); }
                          }
                        }
                      }
                    }
                    BeltDirection::None => {
                      panic!("If there's an l segment then there should be a left in or out; {:?}", factory.floor.cells[x][y].belt);
                    }
                  }
                }
                // else part is not at any edge. no need to do anything here
              }

              segments_checked += 1;
            }
          }

          // Distribute a part in the center
          if
            stage == CENTER &&
            !matches!(factory.floor.cells[x][y].segment_c_part.kind, PartKind::None)
          {
            if (ticks - factory.floor.cells[x][y].segment_c_at) >= factory.floor.cells[x][y].speed {
              // The part reached the end of this center segment. If possible it should be handed
              // off to a neighboring segment.
              // We have to check which segments are eligible to receive parts. There must be at
              // least one and may be up to three. For each such segment, we must check if there
              // is room and then we have to consider cycling evenly between the eligible segments.

              let mut checked: u8 = 0;
              let mut index: u8 = factory.floor.cells[x][y].offset_center;
              while checked < 4 {
                index += 1;
                if index >= 4 {
                  index = 0;
                }

                // Process in lrdu order, offset after the one picked last. Pick the first segment
                // that ends in an Out and has space.

                match index {
                  0 => {
                    if
                      matches!(factory.floor.cells[x][y].belt.direction_u, BeltDirection::Out) &&
                      matches!(factory.floor.cells[x][y].segment_u_part.kind, PartKind::None)
                    {
                      factory.floor.cells[x][y].segment_u_part = factory.floor.cells[x][y].segment_c_part.clone();
                      factory.floor.cells[x][y].segment_u_at = ticks;
                      factory.floor.cells[x][y].segment_c_part = part_none(x, y);
                      if options.print_moves || options.print_moves_belt { println!("{} Handed from center segment to up segment", ticks); }
                      factory.floor.cells[x][y].offset_center = 1;
                      break;
                    }
                  },
                  1 => {
                    if
                      matches!(factory.floor.cells[x][y].belt.direction_r, BeltDirection::Out) &&
                      matches!(factory.floor.cells[x][y].segment_r_part.kind, PartKind::None)
                    {
                      factory.floor.cells[x][y].segment_r_part = factory.floor.cells[x][y].segment_c_part.clone();
                      factory.floor.cells[x][y].segment_r_at = ticks;
                      factory.floor.cells[x][y].segment_c_part = part_none(x, y);
                      if options.print_moves || options.print_moves_belt { println!("{} Handed from center segment to right segment", ticks); }
                      factory.floor.cells[x][y].offset_center = 2;
                      break;
                    }
                  },
                  2 => {
                    if
                    matches!(factory.floor.cells[x][y].belt.direction_d, BeltDirection::Out) &&
                      matches!(factory.floor.cells[x][y].segment_d_part.kind, PartKind::None)
                    {
                      factory.floor.cells[x][y].segment_d_part = factory.floor.cells[x][y].segment_c_part.clone();
                      factory.floor.cells[x][y].segment_d_at = ticks;
                      factory.floor.cells[x][y].segment_c_part = part_none(x, y);
                      if options.print_moves || options.print_moves_belt { println!("{} Handed from center segment to down segment", ticks); }
                      factory.floor.cells[x][y].offset_center = 3;
                      break;
                    }
                  },
                  3 => {
                    if
                    matches!(factory.floor.cells[x][y].belt.direction_l, BeltDirection::Out) &&
                      matches!(factory.floor.cells[x][y].segment_l_part.kind, PartKind::None)
                    {
                      factory.floor.cells[x][y].segment_l_part = factory.floor.cells[x][y].segment_c_part.clone();
                      factory.floor.cells[x][y].segment_l_at = ticks;
                      factory.floor.cells[x][y].segment_c_part = part_none(x, y);
                      if options.print_moves || options.print_moves_belt { println!("{} Handed from center segment to left segment", ticks); }
                      factory.floor.cells[x][y].offset_center = 0;
                      break;
                    }
                  },
                  _ => panic!("in the disco"),
                }

                checked += 1;
              }
            }
            // else part is not at any edge. no need to do anything here
          }

          stage += 1;
        }
      }
      CellKind::Machine => {
        // If not currently building, all require inputs are filled, and there is room; start generating the result
        // If the result is generated, try to push it onto an output (in rotating order)

        if
          factory.floor.cells[x][y].machine_start_at == 0 &&
          matches!(factory.floor.cells[x][y].machine_output_have.kind, PartKind::None)
        {
          // For all three inputs, if it either wants nothing or has something; proceed
          if
            (matches!(factory.floor.cells[x][y].machine_input_1_want.kind, PartKind::None) || !matches!(factory.floor.cells[x][y].machine_input_1_have.kind, PartKind::None)) &&
            (matches!(factory.floor.cells[x][y].machine_input_2_want.kind, PartKind::None) || !matches!(factory.floor.cells[x][y].machine_input_2_have.kind, PartKind::None)) &&
            (matches!(factory.floor.cells[x][y].machine_input_2_want.kind, PartKind::None) || !matches!(factory.floor.cells[x][y].machine_input_3_have.kind, PartKind::None))
          {
            factory.floor.cells[x][y].machine_input_1_have = part_none(0, 0);
            factory.floor.cells[x][y].machine_input_2_have = part_none(0, 0);
            factory.floor.cells[x][y].machine_input_3_have = part_none(0, 0);
            factory.floor.cells[x][y].machine_start_at = ticks;
            if options.print_moves || options.print_moves_machine { println!("{} Machine started action, cost: {}", ticks, factory.floor.cells[x][y].machine_production_price); }
            add_price_delta(options, state, stats, ticks, factory.floor.cells[x][y].machine_production_price);
          }
        } else if
          factory.floor.cells[x][y].machine_start_at > 0 &&
          ticks - factory.floor.cells[x][y].machine_start_at > factory.floor.cells[x][y].speed
        {
          factory.floor.cells[x][y].machine_start_at = 0;
          factory.floor.cells[x][y].machine_output_have = factory.floor.cells[x][y].machine_output_want.clone();
          if options.print_moves || options.print_moves_machine { println!("{} Machine finished action", ticks); }
        } else if
          !matches!(factory.floor.cells[x][y].machine_output_have.kind, PartKind::None)
        {
          // Find an exit with neighboring belt that accepts output from this machine, check if
          // it has room on that segment, and move the result onto that belt if so.
          // Start checking exits in rotating order.

          let mut index = 0;
          let mut checked = 0;
          while checked < 4 {
            match index {
              0 => {
                checked += 1;
                if
                  y > 0 &&
                  matches!(factory.floor.cells[x][y-1].belt.direction_d, BeltDirection::In) &&
                  matches!(factory.floor.cells[x][y-1].segment_d_part.kind, PartKind::None)
                {
                  factory.floor.cells[x][y-1].segment_d_part = factory.floor.cells[x][y].machine_output_have.clone();
                  factory.floor.cells[x][y-1].segment_d_at = ticks;
                  factory.floor.cells[x][y].machine_output_have = part_none(0, 0);
                  if options.print_moves || options.print_moves_machine { println!("{} Moved part from machine to up belt", ticks); }
                }
              },
              1 => {
                checked += 1;
                if
                  x < factory.floor.width - 1 &&
                  matches!(factory.floor.cells[x+1][y].belt.direction_l, BeltDirection::In) &&
                  matches!(factory.floor.cells[x+1][y].segment_l_part.kind, PartKind::None)
                {
                  factory.floor.cells[x+1][y].segment_l_part = factory.floor.cells[x][y].machine_output_have.clone();
                  factory.floor.cells[x+1][y].segment_l_at = ticks;
                  factory.floor.cells[x][y].machine_output_have = part_none(0, 0);
                  if options.print_moves || options.print_moves_machine { println!("{} Moved part from machine to right belt", ticks); }
                }
              },
              2 => {
                checked += 1;
                if
                  y < factory.floor.height - 1 &&
                  matches!(factory.floor.cells[x][y+1].belt.direction_u, BeltDirection::In) &&
                  matches!(factory.floor.cells[x][y+1].segment_u_part.kind, PartKind::None)
                {
                  factory.floor.cells[x][y+1].segment_u_part = factory.floor.cells[x][y].machine_output_have.clone();
                  factory.floor.cells[x][y+1].segment_u_at = ticks;
                  factory.floor.cells[x][y].machine_output_have = part_none(0, 0);
                  if options.print_moves || options.print_moves_machine { println!("{} Moved part from machine to up belt", ticks); }
                }
              },
              3 => {
                checked += 1;
                if
                  x > 0 &&
                  matches!(factory.floor.cells[x-1][y].belt.direction_d, BeltDirection::In) &&
                  matches!(factory.floor.cells[x-1][y].segment_r_part.kind, PartKind::None)
                {
                  factory.floor.cells[x-1][y].segment_r_part = factory.floor.cells[x][y].machine_output_have.clone();
                  factory.floor.cells[x-1][y].segment_r_at = ticks;
                  factory.floor.cells[x][y].machine_output_have = part_none(0, 0);
                  if options.print_moves || options.print_moves_machine { println!("{} Moved part from machine to up belt", ticks); }
                }
              },
              _ => panic!("index cannot reach 4+; {}", index),
            }

            index += 1;
            if index > 3 {
              index = 0;
            }
          }
        }
      }
    }
  }

  // After all of those, do the suppliers. If anything was able to clear, they should be cleared now
  for supply in factory.suppliers.iter_mut() {
    supply.ticks += 1;

    if matches!(supply.part.kind, PartKind::None) {
      if supply.last_part_out_at == 0 || ticks - supply.last_part_out_at > supply.interval {
        if options.print_moves || options.print_moves_supply { println!("{} creating part in supply at {} {}...", ticks, supply.x, supply.y); }
        // Create new part
        supply.part = part::Part {
          kind: supply.stamp.kind,
          icon: supply.stamp.icon,
        };
        supply.part_at = ticks;
      }
    } else {
      // Have an actual part. See if it would go over the edge yet
      // We need to know the direction in which the part moves and the current position of the part

      if ticks - supply.part_at < supply.speed {
        // This part is still rolling on the belt
        continue;
      }

      let x = supply.x;
      let y = supply.y;

      // Note: we paint the supplies outside of the floor but their x/y is really "above" the belt
      let mut cell: &mut cell::Cell = &mut factory.floor.cells[x][y];
      match cell.kind {
        CellKind::Empty => {
          // Noop. Part stays inside Supply. Nothing else happens.

        },
        CellKind::Belt => {
          // Confirm
          // - whether the neighbor cell has a belt coming from this direction
          // - whether the neighbor belt is available on that segment

          let (dx, dy) =
            // Part is ready to leave the supply. Is there room on the neighboring cell?
            // - Neighbor cell is at same coordinate (figure out where the connection is facing)
            // - Check if there is space on that side of the cell
            // - If so, move part to that cell
            if x == 0 {
              // Left side supply. Neighbor cell is at x + 1, y
              (1i8, 0i8)
            } else if y == 0 {
              // Up side supply. Neighbor cell is at x, y + 1
              (0, 1)
            } else if y == factory.floor.height - 1 {
              // Down side supply. Neighbor cell is at x, y - 1
              (0, -1)
            } else if x == factory.floor.width - 1 {
              // Left side supply. Neighbor cell is at x - 1, y
              (-1, 0)
            } else {
              // Supply should be on an edge, not corner. If this changes this code needs to be updated
              // to get the proper neighbor in those cases.
              panic!("assuming supply lives on non-corner edge, this one must be violating that rule, {} {}", x, y);
            };

          match (dx, dy) {
            (0, 1) => {
              // Supply at the top. Does belt accept parts from the top? Does it have no part in
              // the top segment right now?
              if matches!(cell.belt.direction_u, belt::BeltDirection::In) && matches!(cell.segment_u_part.kind, PartKind::None) {
                // Looks like we can transfer this part
                // TODO: for the time being we copy/destroy but we should check if this is relevant as the Part struct gets more complex over time
                cell.segment_u_part = supply.part.clone();
                cell.segment_u_at = ticks;
                supply.part = part_none(supply.x, supply.y);
                supply.last_part_out_at = ticks;
                if options.print_moves || options.print_moves_supply { println!("{} Handed from up supply to belt below", ticks); }
                add_price_delta(options, state, stats, ticks, supply.part_price);
              }
            },
            (-1, 0) => {
              // Supply at the right. Does belt accept parts from the right? Does it have no part in
              // the right segment right now?
              if matches!(cell.belt.direction_r, belt::BeltDirection::In) &&  matches!(cell.segment_r_part.kind, PartKind::None) {
                // Looks like we can transfer this part
                // TODO: for the time being we copy/destroy but we should check if this is relevant as the Part struct gets more complex over time
                cell.segment_r_part = supply.part.clone();
                cell.segment_r_at = ticks;
                supply.part = part_none(supply.x, supply.y);
                supply.last_part_out_at = ticks;
                if options.print_moves || options.print_moves_supply { println!("{} Handed from right supply to belt left", ticks); }
                add_price_delta(options, state, stats, ticks, supply.part_price);
              }
            },
            (0, -1) => {
              // Supply at the bottom. Does belt accept parts from down? Does it have no part in
              // the down segment right now?
              if matches!(cell.belt.direction_d, belt::BeltDirection::In) &&  matches!(cell.segment_d_part.kind, PartKind::None) {
                // Looks like we can transfer this part
                // TODO: for the time being we copy/destroy but we should check if this is relevant as the Part struct gets more complex over time
                cell.segment_d_part = supply.part.clone();
                cell.segment_d_at = ticks;
                supply.part = part_none(supply.x, supply.y);
                supply.last_part_out_at = ticks;
                if options.print_moves || options.print_moves_supply { println!("{} Handed from down supply to belt up", ticks); }
                add_price_delta(options, state, stats, ticks, supply.part_price);
              }
            },
            (1, 0) => {
              // Supply at the left. Does belt accept parts from the left? Does it have no part in
              // the left segment right now?
              if matches!(cell.belt.direction_l, belt::BeltDirection::In) &&  matches!(cell.segment_l_part.kind, PartKind::None) {
                // Looks like we can transfer this part
                // TODO: for the time being we copy/destroy but we should check if this is relevant as the Part struct gets more complex over time
                cell.segment_l_part = supply.part.clone();
                cell.segment_l_at = ticks;
                supply.part = part_none(supply.x, supply.y);
                supply.last_part_out_at = ticks;
                if options.print_moves || options.print_moves_supply { println!("{} Handed from left supply to belt right", ticks); }
                add_price_delta(options, state, stats, ticks, supply.part_price);
              }
            },
            _ => panic!("dx should only be one step left, right, up, or down. not more. {} {}", dx, dy),
          };
        },
        CellKind::Machine => {
          // Confirm if there is space in the machine to accept this part

        },
      }
    }
  }
}
