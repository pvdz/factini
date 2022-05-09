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

const ONE_SECOND: u64 = 10000;

const OUT: u8 = 0;
const CENTER: u8 = 1;
const IN: u8 = 2;

#[derive(Clone, Copy, Debug, PartialEq)]
enum HasHole {
  None,
  Supply,
  Demand,
}

pub struct Factory {
  pub ticks: u64,
  pub floor: Floor,
  pub prio: Vec<(usize, usize)>,
  pub suppliers: Vec<Supply>,
  pub demanders: Vec<Demand>,
  pub stats: (VecDeque<(i32, u64)>, usize, i32, i32, i32), // ((price_delta, time), one_second_size, one_second_sum, ten_second_sum, belt_cell_count)
}

pub fn create_factory(_options: &mut Options, _state: &mut State) -> Factory {
  let mut floor = test_floor();
  let demanders = vec!(
    Demand {x: 1, y: 4, part_kind: PartKind::GoldenBlueWand, part_price: 10000, trash_price: -1000},
  );

  let prio: Vec<(usize, usize)> = sort_cells(&mut floor, &demanders);
  println!("The prio: {:?}", prio);

  return Factory {
    ticks: 0,
    floor,
    prio,
    suppliers: vec!(
      Supply {x: 2, y: 0, ticks: 0, interval: 10000, part: part_none(0, 0), part_at: 0, last_part_out_at: 0, speed: 10000, stamp: Part { kind: PartKind::WoodenStick, icon: 'w'}, part_price: -600},
      Supply {x: 4, y: 3, ticks: 0, interval: 10000, part: part_none(0, 0), part_at: 0, last_part_out_at: 0, speed: 10000, stamp: Part { kind: PartKind::Sapphire, icon: 's'}, part_price: -800}
    ),
    demanders,
    stats: (
      VecDeque::new(), // vec<(cost_delta, at_tick)> // Up to 100k ticks worth of cost deltas
      0, // one_second_size // Number of elements in cost_deltas that wrap the last 10k ticks
      0, // one_second_total // Efficiency of last 10k ticks
      0, // ten_second_total // Efficiency of last 100k ticks (total of cost_deltas)
      14, // number of belt cells
    ),
  };
}

fn should_be_marked(cell: &Cell, cell_u: &Cell, cell_r: &Cell, cell_d: &Cell, cell_l: &Cell) -> bool {
  // This function is used when determining tick order per cell. This is called for each cell,
  // potentially multiple times, and should return whether it should get the current level of prio.

  if cell.marked {
    // This cell was already gathered in a previous step so ignore it here
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
        (cell_u.marked || cell_u.kind == CellKind::Empty || cell.belt.direction_u != BeltDirection::Out || (cell_u.kind == CellKind::Belt && cell_u.belt.direction_d != BeltDirection::In)) &&
        (cell_r.marked || cell_r.kind == CellKind::Empty || cell.belt.direction_r != BeltDirection::Out || (cell_r.kind == CellKind::Belt && cell_r.belt.direction_l != BeltDirection::In)) &&
        (cell_d.marked || cell_d.kind == CellKind::Empty || cell.belt.direction_d != BeltDirection::Out || (cell_d.kind == CellKind::Belt && cell_d.belt.direction_u != BeltDirection::In)) &&
        (cell_l.marked || cell_l.kind == CellKind::Empty || cell.belt.direction_l != BeltDirection::Out || (cell_l.kind == CellKind::Belt && cell_l.belt.direction_r != BeltDirection::In))
      ;
    },
    CellKind::Machine => {
      // All ports are
      // - connected to an empty cell, or
      // - connected to marked cells, or
      // - connected to another machine (which does not work), or
      // - not connected to inports

      return
        (cell_u.marked || cell_u.kind != CellKind::Belt || (cell_u.kind == CellKind::Belt && cell_u.belt.direction_d != BeltDirection::In)) &&
        (cell_r.marked || cell_r.kind != CellKind::Belt || (cell_r.kind == CellKind::Belt && cell_r.belt.direction_l != BeltDirection::In)) &&
        (cell_d.marked || cell_d.kind != CellKind::Belt || (cell_d.kind == CellKind::Belt && cell_d.belt.direction_u != BeltDirection::In)) &&
        (cell_l.marked || cell_l.kind != CellKind::Belt || (cell_l.kind == CellKind::Belt && cell_l.belt.direction_r != BeltDirection::In))
      ;
    }
  }
}

fn sort_cells(floor: &mut Floor, demanders: &Vec<Demand>) -> Vec<(usize, usize)> {
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

  let cell_none = empty_cell(0, 0);

  let mut out: Vec<(usize, usize)> = vec!();

  // println!("sort_cells\n- start with demanders");
  for demand in demanders {
    let x = demand.x;
    let y = demand.y;
    // The cell will exist, regardless of whether it's a belt, machine or none cell.
    floor.cells[x][y].marked = true;
    out.push((x, y));
  }

  // println!("- iteratively follow the trail");
  let mut found_something = true;
  let mut some_left = true;
  let w = floor.width;
  let h = floor.height;
  while found_something && some_left {
    // println!("  - loop");
    found_something = false;
    for x in 0..w {
      for y in 0..h {
        if should_be_marked(
          &floor.cells[x][y],
          if y == 0 { &cell_none } else { &floor.cells[x][y-1] },
          if x == w-1 { &cell_none } else { &floor.cells[x+1][y] },
          if y == h-1 { &cell_none } else { &floor.cells[x][y+1] },
          if x == 0 { &cell_none } else { &floor.cells[x-1][y] },
        ) {
          // println!("    - adding {} {}", x, y);
          floor.cells[x][y].marked = true;
          out.push((x, y));
          found_something = true;
        } else {
          some_left = true;
        }
      }
    }
  }

  // println!("- now add the remaining {} in random order", 25-out.len());
  for x in 0..floor.width {
    for y in 0..floor.height {
      if floor.cells[x][y].kind != CellKind::Empty {
        if !floor.cells[x][y].marked {
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

pub fn tick_factory(options: &mut Options, state: &mut State, factory: &mut Factory) {
  factory.ticks += 1; // offset 1

  let ticks = factory.ticks;

  for (x, y) in factory.prio.to_vec() {
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
        factory.floor.cells[x][y].offset_segment = (current_segment_index + 1) % 4;

        // Loop three times, each iteration considers only segments of that kind (outgoing, center, incoming)
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
                factory.floor.cells[x][y].segment_u_part.kind != PartKind::None
              {
                if (ticks - factory.floor.cells[x][y].segment_u_at) >= factory.floor.cells[x][y].speed {
                  // The part reached the other side of this segment. If possible it should be handed
                  // off to the next segment, machine, or demand.
                  match factory.floor.cells[x][y].belt.direction_u {
                    BeltDirection::In => {
                      // Hand it off to the center segment
                      if factory.floor.cells[x][y].segment_c_part.kind == PartKind::None {
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
                            if demand.part_kind == kind {
                              if options.print_moves || options.print_moves_demand { println!("{} Sold a {:?} from up segment to Demand, price: {}", ticks, kind, demand.part_price); }
                              add_price_delta(options, state, &mut factory.stats, factory.ticks, demand.part_price);
                            } else {
                              if options.print_moves || options.print_moves_demand { println!("{} Trashed a {:?} from up segment to Demand, price: {}", ticks, kind, demand.trash_price); }
                              add_price_delta(options, state, &mut factory.stats, factory.ticks, demand.trash_price);
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
                              factory.floor.cells[x][y-1].belt.direction_d == BeltDirection::In &&
                              // The part would be put on the down segment so check if that's available
                              factory.floor.cells[x][y-1].segment_d_part.kind == PartKind::None
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
                              factory.floor.cells[x][y-1].machine_input_1_want.kind == segment_kind &&
                              factory.floor.cells[x][y-1].machine_input_1_have.kind == PartKind::None
                            {
                              factory.floor.cells[x][y-1].machine_input_1_have = factory.floor.cells[x][y].segment_u_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 1", ticks); }
                            }
                            else if
                              factory.floor.cells[x][y-1].machine_input_2_want.kind == segment_kind &&
                              factory.floor.cells[x][y-1].machine_input_2_have.kind == PartKind::None
                            {
                              factory.floor.cells[x][y-1].machine_input_2_have = factory.floor.cells[x][y].segment_u_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 2", ticks); }
                            }
                            else if
                              factory.floor.cells[x][y-1].machine_input_3_want.kind == segment_kind &&
                              factory.floor.cells[x][y-1].machine_input_3_have.kind == PartKind::None
                            {
                              factory.floor.cells[x][y-1].machine_input_3_have = factory.floor.cells[x][y].segment_u_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 3", ticks); }
                            } else {
                              // Destruct part
                              if options.print_moves || options.print_moves_machine { println!("{} Machine did not need part, it was trashed, cost: {}", ticks, factory.floor.cells[x][y-1].machine_trash_price); }
                              add_price_delta(options, state, &mut factory.stats, factory.ticks, factory.floor.cells[x][y-1].machine_trash_price);
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
                factory.floor.cells[x][y].segment_r_part.kind != PartKind::None
              {
                if (ticks - factory.floor.cells[x][y].segment_r_at) >= factory.floor.cells[x][y].speed {
                  // The part reached the other side of this segment. If possible it should be handed
                  // off to the next segment, machine, or demand.
                  match factory.floor.cells[x][y].belt.direction_r {
                    BeltDirection::In => {
                      // Hand it off to the center segment
                      if factory.floor.cells[x][y].segment_c_part.kind == PartKind::None {
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
                            if demand.part_kind == kind {
                              println!("{} Sold a {:?} from left segment to Demand, price: {}", ticks, kind, demand.part_price);
                              add_price_delta(options, state, &mut factory.stats, factory.ticks, demand.part_price);
                            } else {
                              println!("{} Trashed a {:?} from left segment to Demand, price: {}", ticks, kind, demand.trash_price);
                              add_price_delta(options, state, &mut factory.stats, factory.ticks, demand.trash_price);
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
                              factory.floor.cells[x+1][y].belt.direction_l == BeltDirection::In &&
                              // The part would be put on the left segment of the right belt so check if that's available
                              factory.floor.cells[x+1][y].segment_l_part.kind == PartKind::None
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
                              factory.floor.cells[x+1][y].machine_input_1_want.kind == segment_kind &&
                              factory.floor.cells[x+1][y].machine_input_1_have.kind == PartKind::None
                            {
                              factory.floor.cells[x+1][y].machine_input_1_have = factory.floor.cells[x][y].segment_r_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 1", ticks); }
                            }
                            else if
                              factory.floor.cells[x+1][y].machine_input_2_want.kind == segment_kind &&
                              factory.floor.cells[x+1][y].machine_input_2_have.kind == PartKind::None
                            {
                              factory.floor.cells[x+1][y].machine_input_2_have = factory.floor.cells[x][y].segment_r_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 2", ticks); }
                            }
                            else if
                              factory.floor.cells[x+1][y].machine_input_3_want.kind == segment_kind &&
                              factory.floor.cells[x+1][y].machine_input_3_have.kind == PartKind::None
                            {
                              factory.floor.cells[x+1][y].machine_input_3_have = factory.floor.cells[x][y].segment_r_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 3", ticks); }
                            } else {
                              // Destruct part
                              if options.print_moves || options.print_moves_machine { println!("{} Machine did not need part, it was trashed, cost: {}", ticks, factory.floor.cells[x+1][y].machine_trash_price); }
                              add_price_delta(options, state, &mut factory.stats, factory.ticks, factory.floor.cells[x+1][y].machine_trash_price);
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
                factory.floor.cells[x][y].segment_d_part.kind != PartKind::None
              {
                if (ticks - factory.floor.cells[x][y].segment_d_at) >= factory.floor.cells[x][y].speed {
                  // The part reached the other side of this segment. If possible it should be handed
                  // off to the next segment, machine, or demand.
                  match factory.floor.cells[x][y].belt.direction_d {
                    BeltDirection::In => {
                      // Hand it off to the center segment
                      if factory.floor.cells[x][y].segment_c_part.kind == PartKind::None {
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
                            if demand.part_kind == kind {
                              if options.print_moves || options.print_moves_demand { println!("{} Sold a {:?} from down segment to Demand, price: {}", ticks, kind, demand.part_price); }
                              add_price_delta(options, state, &mut factory.stats, factory.ticks, demand.part_price);
                            } else {
                              if options.print_moves || options.print_moves_demand { println!("{} Trashed a {:?} from down segment to Demand, price: {}", ticks, kind, demand.trash_price); }
                              add_price_delta(options, state, &mut factory.stats, factory.ticks, demand.trash_price);
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
                              factory.floor.cells[x][y+1].belt.direction_u == BeltDirection::In &&
                              // The part would be put on the up segment of the below belt so check if that's available
                              factory.floor.cells[x][y+1].segment_u_part.kind == PartKind::None
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
                              factory.floor.cells[x][y+1].machine_input_1_want.kind == segment_kind &&
                              factory.floor.cells[x][y+1].machine_input_1_have.kind == PartKind::None
                            {
                              factory.floor.cells[x][y+1].machine_input_1_have = factory.floor.cells[x][y].segment_d_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 1", ticks); }
                            }
                            else if
                              factory.floor.cells[x][y+1].machine_input_2_want.kind == segment_kind &&
                              factory.floor.cells[x][y+1].machine_input_2_have.kind == PartKind::None
                            {
                              factory.floor.cells[x][y+1].machine_input_2_have = factory.floor.cells[x][y].segment_d_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 2", ticks); }
                            }
                            else if
                              factory.floor.cells[x][y+1].machine_input_3_want.kind == segment_kind &&
                              factory.floor.cells[x][y+1].machine_input_3_have.kind == PartKind::None
                            {
                              factory.floor.cells[x][y+1].machine_input_3_have = factory.floor.cells[x][y].segment_d_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 3", ticks); }
                            } else {
                              // Destruct part
                              if options.print_moves || options.print_moves_machine { println!("{} Machine did not need part, it was trashed, cost: {}", ticks, factory.floor.cells[x][y+1].machine_trash_price); }
                              add_price_delta(options, state, &mut factory.stats, factory.ticks, factory.floor.cells[x][y+1].machine_trash_price);
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
                factory.floor.cells[x][y].segment_l_part.kind != PartKind::None
              {
                if (ticks - factory.floor.cells[x][y].segment_l_at) >= factory.floor.cells[x][y].speed {
                  // The part reached the other side of this segment. If possible it should be handed
                  // off to the next segment, machine, or demand.
                  match factory.floor.cells[x][y].belt.direction_l {
                    BeltDirection::In => {
                      // Hand it off to the center segment
                      if factory.floor.cells[x][y].segment_c_part.kind == PartKind::None {
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
                            if demand.part_kind == kind {
                              println!("{} Sold a {:?} from left segment to Demand, price: {}", ticks, kind, demand.part_price);
                              add_price_delta(options, state, &mut factory.stats, factory.ticks, demand.part_price);
                            } else {
                              println!("{} Trashed a {:?} from left segment to Demand, price: {}", ticks, kind, demand.trash_price);
                              add_price_delta(options, state, &mut factory.stats, factory.ticks, demand.trash_price);
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
                              factory.floor.cells[x-1][y].belt.direction_r == BeltDirection::In &&
                              // The part would be put on the right segment of the left belt so check if that's available
                              factory.floor.cells[x-1][y].segment_r_part.kind == PartKind::None
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
                              factory.floor.cells[x-1][y].machine_input_1_want.kind == segment_kind &&
                              factory.floor.cells[x-1][y].machine_input_1_have.kind == PartKind::None
                            {
                              factory.floor.cells[x-1][y].machine_input_1_have = factory.floor.cells[x][y].segment_l_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 1; wanted {:?} and this was {:?}", ticks, factory.floor.cells[x-1][y].machine_input_1_want.kind, segment_kind); }
                            }
                            else if
                              factory.floor.cells[x-1][y].machine_input_2_want.kind == segment_kind &&
                              factory.floor.cells[x-1][y].machine_input_2_have.kind == PartKind::None
                            {
                              factory.floor.cells[x-1][y].machine_input_2_have = factory.floor.cells[x][y].segment_l_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 2; wanted {:?} and this was {:?}", ticks, factory.floor.cells[x-1][y].machine_input_2_want.kind, segment_kind); }
                            }
                            else if
                              factory.floor.cells[x-1][y].machine_input_3_want.kind == segment_kind &&
                              factory.floor.cells[x-1][y].machine_input_3_have.kind == PartKind::None
                            {
                              factory.floor.cells[x-1][y].machine_input_3_have = factory.floor.cells[x][y].segment_l_part.clone();
                              if options.print_moves || options.print_moves_machine { println!("{} Machine put part in slot 3; wanted {:?} and this was {:?} so {}", ticks, factory.floor.cells[x-1][y].machine_input_3_want.kind, segment_kind, factory.floor.cells[x-1][y].machine_input_3_want.kind == segment_kind); }
                            } else {
                              // Destruct part
                              if options.print_moves || options.print_moves_machine { println!("{} Machine did not need part, it was trashed", ticks); }
                              add_price_delta(options, state, &mut factory.stats, factory.ticks, factory.floor.cells[x-1][y].machine_trash_price);
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
          // There are two points in this life cycle:
          // - 50%: determine which way the part will go (or block it while there is no valid target)
          // - 100%: move particle to the determined neighbor

          if stage == CENTER && factory.floor.cells[x][y].segment_c_part.kind != PartKind::None {
            // At 50%, while the part is marked as blocked, find a valid neighbor segment to move
            // the part to when it reaches 100%. A valid neighbor is an outgoing segment that has
            // no part currently. This is the reason that all choices are made midway a segment
            // rather than the end (which would have been slightly easier).




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
                      factory.floor.cells[x][y].belt.direction_u == BeltDirection::Out &&
                      factory.floor.cells[x][y].segment_u_part.kind == PartKind::None
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
                      factory.floor.cells[x][y].belt.direction_r == BeltDirection::Out &&
                      factory.floor.cells[x][y].segment_r_part.kind == PartKind::None
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
                      factory.floor.cells[x][y].belt.direction_d == BeltDirection::Out &&
                      factory.floor.cells[x][y].segment_d_part.kind == PartKind::None
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
                      factory.floor.cells[x][y].belt.direction_l == BeltDirection::Out &&
                      factory.floor.cells[x][y].segment_l_part.kind == PartKind::None
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
          factory.floor.cells[x][y].machine_output_have.kind == PartKind::None
        {
          // For all three inputs, if it either wants and has nothing or it wants and has something; proceed
          if
            factory.floor.cells[x][y].machine_input_1_want.kind == factory.floor.cells[x][y].machine_input_1_have.kind &&
            factory.floor.cells[x][y].machine_input_2_want.kind == factory.floor.cells[x][y].machine_input_2_have.kind &&
            factory.floor.cells[x][y].machine_input_3_want.kind == factory.floor.cells[x][y].machine_input_3_have.kind
          {
            factory.floor.cells[x][y].machine_input_1_have = part_none(0, 0);
            factory.floor.cells[x][y].machine_input_2_have = part_none(0, 0);
            factory.floor.cells[x][y].machine_input_3_have = part_none(0, 0);
            factory.floor.cells[x][y].machine_start_at = ticks;
            if options.print_moves || options.print_moves_machine { println!("{} Machine started action, cost: {}", ticks, factory.floor.cells[x][y].machine_production_price); }
            add_price_delta(options, state, &mut factory.stats, factory.ticks, factory.floor.cells[x][y].machine_production_price);
          }
        } else if
          factory.floor.cells[x][y].machine_start_at > 0 &&
          ticks - factory.floor.cells[x][y].machine_start_at > factory.floor.cells[x][y].speed
        {
          factory.floor.cells[x][y].machine_start_at = 0;
          factory.floor.cells[x][y].machine_output_have = factory.floor.cells[x][y].machine_output_want.clone();
          if options.print_moves || options.print_moves_machine { println!("{} Machine finished action", ticks); }
        } else if
          factory.floor.cells[x][y].machine_output_have.kind != PartKind::None
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
                  factory.floor.cells[x][y-1].belt.direction_d == BeltDirection::In &&
                  factory.floor.cells[x][y-1].segment_d_part.kind == PartKind::None
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
                  factory.floor.cells[x+1][y].belt.direction_l == BeltDirection::In &&
                  factory.floor.cells[x+1][y].segment_l_part.kind == PartKind::None
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
                  factory.floor.cells[x][y+1].belt.direction_u == BeltDirection::In &&
                  factory.floor.cells[x][y+1].segment_u_part.kind == PartKind::None
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
                  factory.floor.cells[x-1][y].belt.direction_d == BeltDirection::In &&
                  factory.floor.cells[x-1][y].segment_r_part.kind == PartKind::None
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
  for sn in 0..factory.suppliers.len() {

  // for supply in factory.suppliers.iter_mut() {
    let mut supply = &mut factory.suppliers[sn];
    supply.ticks += 1;

    if supply.part.kind == PartKind::None {
      if supply.last_part_out_at == 0 || ticks - supply.last_part_out_at > supply.interval {
        if options.print_moves || options.print_moves_supply { println!("{} creating part in supply at {} {}...", ticks, supply.x, supply.y); }
        // Create new part
        supply.part = Part {
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
      let mut cell: &mut Cell = &mut factory.floor.cells[x][y];
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
              if cell.belt.direction_u == BeltDirection::In && cell.segment_u_part.kind == PartKind::None {
                // Looks like we can transfer this part
                // TODO: for the time being we copy/destroy but we should check if this is relevant as the Part struct gets more complex over time
                cell.segment_u_part = supply.part.clone();
                cell.segment_u_at = ticks;
                supply.part = part_none(supply.x, supply.y);
                supply.last_part_out_at = ticks;
                if options.print_moves || options.print_moves_supply { println!("{} Handed from up supply to belt below", ticks); }
                add_price_delta(options, state, &mut factory.stats, factory.ticks, supply.part_price);
              }
            },
            (-1, 0) => {
              // Supply at the right. Does belt accept parts from the right? Does it have no part in
              // the right segment right now?
              if cell.belt.direction_r == BeltDirection::In && cell.segment_r_part.kind == PartKind::None {
                // Looks like we can transfer this part
                // TODO: for the time being we copy/destroy but we should check if this is relevant as the Part struct gets more complex over time
                cell.segment_r_part = supply.part.clone();
                cell.segment_r_at = ticks;
                supply.part = part_none(supply.x, supply.y);
                supply.last_part_out_at = ticks;
                if options.print_moves || options.print_moves_supply { println!("{} Handed from right supply to belt left", ticks); }
                add_price_delta(options, state, &mut factory.stats, factory.ticks, supply.part_price);
              }
            },
            (0, -1) => {
              // Supply at the bottom. Does belt accept parts from down? Does it have no part in
              // the down segment right now?
              if cell.belt.direction_d == BeltDirection::In && cell.segment_d_part.kind == PartKind::None {
                // Looks like we can transfer this part
                // TODO: for the time being we copy/destroy but we should check if this is relevant as the Part struct gets more complex over time
                cell.segment_d_part = supply.part.clone();
                cell.segment_d_at = ticks;
                supply.part = part_none(supply.x, supply.y);
                supply.last_part_out_at = ticks;
                if options.print_moves || options.print_moves_supply { println!("{} Handed from down supply to belt up", ticks); }
                add_price_delta(options, state, &mut factory.stats, factory.ticks, supply.part_price);
              }
            },
            (1, 0) => {
              // Supply at the left. Does belt accept parts from the left? Does it have no part in
              // the left segment right now?
              if cell.belt.direction_l == BeltDirection::In && cell.segment_l_part.kind == PartKind::None {
                // Looks like we can transfer this part
                // TODO: for the time being we copy/destroy but we should check if this is relevant as the Part struct gets more complex over time
                cell.segment_l_part = supply.part.clone();
                cell.segment_l_at = ticks;
                supply.part = part_none(supply.x, supply.y);
                supply.last_part_out_at = ticks;
                if options.print_moves || options.print_moves_supply { println!("{} Handed from left supply to belt right", ticks); }
                add_price_delta(options, state, &mut factory.stats, factory.ticks, supply.part_price);
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

  update_factory_efficiency_stats(options, state, factory);
}

fn update_factory_efficiency_stats(options: &mut Options, _state: &mut State, factory: &mut Factory) {
  if (factory.ticks % ONE_SECOND) == 0 {
    println!("- {}; efficiency: short period: {}, long period: {} {:100}", factory.ticks, factory.stats.2 + (14 * -1000), factory.stats.3 + (14 * -10000), ' ');
  }

  // Update second total by shrinking the window until the oldest entry inside it is at most 10k ticks old
  let pre_len = factory.stats.0.len();
  while factory.stats.1 > 0 && (factory.ticks - factory.stats.0[pre_len - factory.stats.1].1) > options.short_term_window {
    let delta = factory.stats.0[pre_len - factory.stats.1].0;
    factory.stats.1 -= 1;
    factory.stats.2 -= delta as i32;
  }

  // Update 10 second total by shrinking the deltas until the oldest entry is at most 100k ticks old
  while factory.stats.0.len() > 0 && (factory.ticks - factory.stats.0[0].1) > options.long_term_window {
    factory.stats.3 -= factory.stats.0[0].0 as i32;
    factory.stats.0.pop_front();
  }
}

fn is_current_phase(dir: BeltDirection, stage: u8) -> bool {
  return match dir {
    BeltDirection::In => stage == IN,
    BeltDirection::Out => stage == OUT,
    BeltDirection::None => false,
  };
}

fn add_price_delta(options: &mut Options, _state: &mut State, stats: &mut (VecDeque<(i32, u64)>, usize, i32, i32, i32), ticks: u64, delta: i32) {
  if options.print_price_deltas { println!("  - add_price_delta: {}, 1sec: {}, 10sec: {}", delta, stats.2 + (delta as i32), stats.3 + (delta as i32)); }
  stats.0.push_back((delta, ticks));
  stats.1 += 1; // new entry should not be older than 10k ticks ;)
  stats.2 += delta as i32;
  stats.3 += delta as i32;
}

fn hole_in_the_wall(factory: &Factory, x: usize, y: usize, ifn: char, ifs: char, ifd: char) -> char {
  // My apologies for anyone doing perf ;)

  let suppliers = &factory.suppliers;
  let demanders = &factory.demanders;

  let mut has = HasHole::None;
  for supply in suppliers {
    if supply.x == x && supply.y == y {
      has = HasHole::Supply;
      break;
    }
  }

  if has == HasHole::None {
    for demand in demanders {
      if demand.x == x && demand.y == y {
        has = HasHole::Demand;
        break;
      }
    }
  }

  return match has {
    HasHole::None => ifn,
    HasHole::Supply => ifs,
    HasHole::Demand => ifd,
  };
}
fn hu(factory: &Factory, x: usize) -> char {
  return hole_in_the_wall(factory, x, 0, '─', 'v', '^');
}
fn hr(factory: &Factory, y: usize) -> char {
  return hole_in_the_wall(factory, 4, y, '│', '<', '>');
}
fn hd(factory: &Factory, x: usize) -> char {
  return hole_in_the_wall(factory, x, 4, '─', '^', 'v');
}
fn hl(factory: &Factory, y: usize) -> char {
  return hole_in_the_wall(factory, 0, y, '│', '>', '<');
}

// belt entry/exits
fn br(factory: &Factory, x: usize, y: usize) -> char {
  match factory.floor.cells[x][y].belt.direction_r {
    BeltDirection::In => '<',
    BeltDirection::Out => '>',
    BeltDirection::None => '│',
  }
}
fn bl(factory: &Factory, x: usize, y: usize) -> char {
  match factory.floor.cells[x][y].belt.direction_l {
    BeltDirection::In => '>',
    BeltDirection::Out => '<',
    BeltDirection::None => '│',
  }
}
fn bu(factory: &Factory, x: usize, y: usize) -> char {
  match factory.floor.cells[x][y].belt.direction_u {
    BeltDirection::In => 'v',
    BeltDirection::Out => '^',
    BeltDirection::None => '─',
  }
}
fn bd(factory: &Factory, x: usize, y: usize) -> char {
  match factory.floor.cells[x][y].belt.direction_d {
    BeltDirection::In => '^',
    BeltDirection::Out => 'v',
    BeltDirection::None => '─',
  }
}

// cell segments
fn clu(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => ' ',
    CellKind::Machine => {
      // Print the first input part if there is more than one input
      if cell.machine_input_2_want.kind != PartKind::None {
        cell.machine_input_1_want.icon
      } else {
        ' '
      }
    }
  }
}
fn cu(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => {
      if cell.segment_u_part.kind != PartKind::None {
        cell.segment_u_part.icon
      } else {
        cell.belt.cli_u
      }
    }
    CellKind::Machine => {
      // Print the first input part if there is one input
      // Print the second input part if there are three inputs
      // Print nothing if there are two inputs
      if cell.machine_input_3_want.kind != PartKind::None {
        cell.machine_input_2_want.icon
      } else if cell.machine_input_2_want.kind == PartKind::None {
        cell.machine_input_1_want.icon
      } else {
        ' '
      }
    }
  }
}
fn cru(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => ' ',
    CellKind::Machine => {
      // Print the third input part if there are three inputs
      // Print the second input part if there are two inputs
      // Print nothing if there is one input
      if cell.machine_input_3_want.kind != PartKind::None {
        cell.machine_input_3_want.icon
      } else if cell.machine_input_2_want.kind != PartKind::None {
        cell.machine_input_2_want.icon
      } else {
        ' '
      }
    }
  }
}
fn cl(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => {
      if cell.segment_l_part.kind != PartKind::None {
        cell.segment_l_part.icon
      } else {
        cell.belt.cli_l
      }
    }
    CellKind::Machine => {
      // Print corner if there are two or three inputs. Otherwise print nothing
      if cell.machine_input_2_want.kind != PartKind::None {
        if cell.machine_input_2_have.kind != PartKind::None {
          '┗'
        } else {
          '└'
        }
      } else {
        ' '
      }
    }
  }
}
fn cc(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => {
      if cell.segment_c_part.kind != PartKind::None {
        cell.segment_c_part.icon
      } else {
        cell.belt.cli_c
      }
    }
    CellKind::Machine => {
      // Print straight line if one input
      // Print T down if two inputs
      // Print cross if three inputs
      if cell.machine_input_3_want.kind != PartKind::None {
        if cell.machine_input_1_have.kind != PartKind::None && cell.machine_input_2_have.kind != PartKind::None && cell.machine_input_3_have.kind != PartKind::None {
          '╋'
        } else if cell.machine_input_1_have.kind != PartKind::None && cell.machine_input_2_have.kind != PartKind::None {
          '╃'
        } else if cell.machine_input_1_have.kind != PartKind::None && cell.machine_input_3_have.kind != PartKind::None {
          '┿'
        } else if cell.machine_input_2_have.kind != PartKind::None && cell.machine_input_3_have.kind != PartKind::None {
          '╄'
        } else if cell.machine_input_1_have.kind != PartKind::None {
          '┽'
        } else if cell.machine_input_2_have.kind != PartKind::None {
          '╀'
        } else if cell.machine_input_3_have.kind != PartKind::None {
          '┾'
        } else {
          '┼'
        }
      } else if cell.machine_input_2_want.kind != PartKind::None {
        if cell.machine_input_1_have.kind != PartKind::None && cell.machine_input_2_have.kind != PartKind::None {
          '┳'
        } else if cell.machine_input_1_have.kind != PartKind::None {
          '┭'
        } else if cell.machine_input_2_have.kind != PartKind::None {
          '┮'
        } else {
          '┬'
        }
      } else {
        if cell.machine_input_1_have.kind != PartKind::None {
          '┃'
        } else {
          '│'
        }
      }
    }
  }
}
fn cr(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => {
      if cell.segment_r_part.kind != PartKind::None {
        cell.segment_r_part.icon
      } else {
        cell.belt.cli_r
      }
    }
    CellKind::Machine => {
      // Print corner if there are two or three inputs. Otherwise print nothing
      if cell.machine_input_2_want.kind != PartKind::None {
        if (cell.machine_input_3_want.kind != PartKind::None && cell.machine_input_3_have.kind != PartKind::None) || (cell.machine_input_3_want.kind == PartKind::None && cell.machine_input_2_have.kind != PartKind::None) {
          '┛'
        } else {
          '┘'
        }
      } else {
        ' '
      }
    }
  }
}
fn cdl(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => ' ',
    CellKind::Machine => {
      ' '
    }
  }
}
fn cd(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => {
      if cell.segment_d_part.kind != PartKind::None {
        cell.segment_d_part.icon
      } else {
        cell.belt.cli_d
      }
    }
    CellKind::Machine => {
      cell.machine_output_want.icon
    }
  }
}
fn cdr(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => ' ',
    CellKind::Machine => {
      ' '
    }
  }
}

pub fn serialize_cli_lines(factory: &Factory) -> Vec<String> {
  return vec!(
    format!("┌──{}────{}────{}────{}────{}──┐", hu(factory, 0), hu(factory, 1), hu(factory, 2), hu(factory, 3), hu(factory, 4)).to_string(),
    format!("│┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐│", bu(factory, 0, 0), bu(factory, 1, 0), bu(factory, 2, 0), bu(factory, 3, 0), bu(factory, 4, 0)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", clu(factory, 0, 0), cu(factory, 0, 0), cru(factory, 0, 0), clu(factory, 1, 0), cu(factory, 1, 0), cru(factory, 1, 0), clu(factory, 2, 0), cu(factory, 2, 0), cru(factory, 2, 0), clu(factory, 3, 0), cu(factory, 3, 0), cru(factory, 3, 0), clu(factory, 4, 0), cu(factory, 4, 0), cru(factory, 4, 0)).to_string(),
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", hl(factory, 0), bl(factory, 0, 0), cl(factory, 0, 0), cc(factory, 0, 0), cr(factory, 0, 0), br(factory, 0, 0), bl(factory, 1, 0), cl(factory, 1, 0), cc(factory, 1, 0), cr(factory, 1, 0), br(factory, 1, 0), bl(factory, 2, 0), cl(factory, 2, 0), cc(factory, 2, 0), cr(factory, 2, 0), br(factory, 2, 0), bl(factory, 3, 0), cl(factory, 3, 0), cc(factory, 3, 0), cr(factory, 3, 0), br(factory, 3, 0), bl(factory, 4, 0), cl(factory, 4, 0), cc(factory, 4, 0), cr(factory, 4, 0), br(factory, 4, 0), hr(factory, 0)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", cdl(factory, 0, 0), cd(factory, 0, 0), cdr(factory, 0, 0), cdl(factory, 1, 0), cd(factory, 1, 0), cdr(factory, 1, 0), cdl(factory, 2, 0), cd(factory, 2, 0), cdr(factory, 2, 0), cdl(factory, 3, 0), cd(factory, 3, 0), cdr(factory, 3, 0), cdl(factory, 4, 0), cd(factory, 4, 0), cdr(factory, 4, 0)).to_string(),
    format!("│└─{}─┘└─{}─┘└─{}─┘└─{}─┘└─{}─┘│", bd(factory, 0, 0), bd(factory, 1, 0), bd(factory, 2, 0), bd(factory, 3, 0), bd(factory, 4, 0)).to_string(),
    format!("│┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐│", bu(factory, 0, 1), bu(factory, 1, 1), bu(factory, 2, 1), bu(factory, 3, 1), bu(factory, 4, 1)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", clu(factory, 0, 1), cu(factory, 0, 1), cru(factory, 0, 1), clu(factory, 1, 1), cu(factory, 1, 1), cru(factory, 1, 1), clu(factory, 2, 1), cu(factory, 2, 1), cru(factory, 2, 1), clu(factory, 3, 1), cu(factory, 3, 1), cru(factory, 3, 1), clu(factory, 4, 1), cu(factory, 4, 1), cru(factory, 4, 1)).to_string(),
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", hl(factory, 1), bl(factory, 0, 1), cl(factory, 0, 1), cc(factory, 0, 1), cr(factory, 0, 1), br(factory, 0, 1), bl(factory, 1, 1), cl(factory, 1, 1), cc(factory, 1, 1), cr(factory, 1, 1), br(factory, 1, 1), bl(factory, 2, 1), cl(factory, 2, 1), cc(factory, 2, 1), cr(factory, 2, 1), br(factory, 2, 1), bl(factory, 3, 1), cl(factory, 3, 1), cc(factory, 3, 1), cr(factory, 3, 1), br(factory, 3, 1), bl(factory, 4, 1), cl(factory, 4, 1), cc(factory, 4, 1), cr(factory, 4, 1), br(factory, 4, 1), hr(factory, 1)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", cdl(factory, 0, 1), cd(factory, 0, 1), cdr(factory, 0, 1), cdl(factory, 1, 1), cd(factory, 1, 1), cdr(factory, 1, 1), cdl(factory, 2, 1), cd(factory, 2, 1), cdr(factory, 2, 1), cdl(factory, 3, 1), cd(factory, 3, 1), cdr(factory, 3, 1), cdl(factory, 4, 1), cd(factory, 4, 1), cdr(factory, 4, 1)).to_string(),
    format!("│└─{}─┘└─{}─┘└─{}─┘└─{}─┘└─{}─┘│", bd(factory, 0, 1), bd(factory, 1, 1), bd(factory, 2, 1), bd(factory, 3, 1), bd(factory, 4, 1)).to_string(),
    format!("│┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐│", bu(factory, 0, 2), bu(factory, 1, 2), bu(factory, 2, 2), bu(factory, 3, 2), bu(factory, 4, 2)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", clu(factory, 0, 2), cu(factory, 0, 2), cru(factory, 0, 2), clu(factory, 1, 2), cu(factory, 1, 2), cru(factory, 1, 2), clu(factory, 2, 2), cu(factory, 2, 2), cru(factory, 2, 2), clu(factory, 3, 2), cu(factory, 3, 2), cru(factory, 3, 2), clu(factory, 4, 2), cu(factory, 4, 2), cru(factory, 4, 2)).to_string(),
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", hl(factory, 2), bl(factory, 0, 2), cl(factory, 0, 2), cc(factory, 0, 2), cr(factory, 0, 2), br(factory, 0, 2), bl(factory, 1, 2), cl(factory, 1, 2), cc(factory, 1, 2), cr(factory, 1, 2), br(factory, 1, 2), bl(factory, 2, 2), cl(factory, 2, 2), cc(factory, 2, 2), cr(factory, 2, 2), br(factory, 2, 2), bl(factory, 3, 2), cl(factory, 3, 2), cc(factory, 3, 2), cr(factory, 3, 2), br(factory, 3, 2), bl(factory, 4, 2), cl(factory, 4, 2), cc(factory, 4, 2), cr(factory, 4, 2), br(factory, 4, 2), hr(factory, 2)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", cdl(factory, 0, 2), cd(factory, 0, 2), cdr(factory, 0, 2), cdl(factory, 1, 2), cd(factory, 1, 2), cdr(factory, 1, 2), cdl(factory, 2, 2), cd(factory, 2, 2), cdr(factory, 2, 2), cdl(factory, 3, 2), cd(factory, 3, 2), cdr(factory, 3, 2), cdl(factory, 4, 2), cd(factory, 4, 2), cdr(factory, 4, 2)).to_string(),
    format!("│└─{}─┘└─{}─┘└─{}─┘└─{}─┘└─{}─┘│", bd(factory, 0, 2), bd(factory, 1, 2), bd(factory, 2, 2), bd(factory, 3, 2), bd(factory, 4, 2)).to_string(),
    format!("│┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐│", bu(factory, 0, 3), bu(factory, 1, 3), bu(factory, 2, 3), bu(factory, 3, 3), bu(factory, 4, 3)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", clu(factory, 0, 3), cu(factory, 0, 3), cru(factory, 0, 3), clu(factory, 1, 3), cu(factory, 1, 3), cru(factory, 1, 3), clu(factory, 2, 3), cu(factory, 2, 3), cru(factory, 2, 3), clu(factory, 3, 3), cu(factory, 3, 3), cru(factory, 3, 3), clu(factory, 4, 3), cu(factory, 4, 3), cru(factory, 4, 3)).to_string(),
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", hl(factory, 3), bl(factory, 0, 3), cl(factory, 0, 3), cc(factory, 0, 3), cr(factory, 0, 3), br(factory, 0, 3), bl(factory, 1, 3), cl(factory, 1, 3), cc(factory, 1, 3), cr(factory, 1, 3), br(factory, 1, 3), bl(factory, 2, 3), cl(factory, 2, 3), cc(factory, 2, 3), cr(factory, 2, 3), br(factory, 2, 3), bl(factory, 3, 3), cl(factory, 3, 3), cc(factory, 3, 3), cr(factory, 3, 3), br(factory, 3, 3), bl(factory, 4, 3), cl(factory, 4, 3), cc(factory, 4, 3), cr(factory, 4, 3), br(factory, 4, 3), hr(factory, 3)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", cdl(factory, 0, 3), cd(factory, 0, 3), cdr(factory, 0, 3), cdl(factory, 1, 3), cd(factory, 1, 3), cdr(factory, 1, 3), cdl(factory, 2, 3), cd(factory, 2, 3), cdr(factory, 2, 3), cdl(factory, 3, 3), cd(factory, 3, 3), cdr(factory, 3, 3), cdl(factory, 4, 3), cd(factory, 4, 3), cdr(factory, 4, 3)).to_string(),
    format!("│└─{}─┘└─{}─┘└─{}─┘└─{}─┘└─{}─┘│", bd(factory, 0, 3), bd(factory, 1, 3), bd(factory, 2, 3), bd(factory, 3, 3), bd(factory, 4, 3)).to_string(),
    format!("│┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐│", bu(factory, 0, 4), bu(factory, 1, 4), bu(factory, 2, 4), bu(factory, 3, 4), bu(factory, 4, 4)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", clu(factory, 0, 4), cu(factory, 0, 4), cru(factory, 0, 4), clu(factory, 1, 4), cu(factory, 1, 4), cru(factory, 1, 4), clu(factory, 2, 4), cu(factory, 2, 4), cru(factory, 2, 4), clu(factory, 3, 4), cu(factory, 3, 4), cru(factory, 3, 4), clu(factory, 4, 4), cu(factory, 4, 4), cru(factory, 4, 4)).to_string(),
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", hl(factory, 4), bl(factory, 0, 4), cl(factory, 0, 4), cc(factory, 0, 4), cr(factory, 0, 4), br(factory, 0, 4), bl(factory, 1, 4), cl(factory, 1, 4), cc(factory, 1, 4), cr(factory, 1, 4), br(factory, 1, 4), bl(factory, 2, 4), cl(factory, 2, 4), cc(factory, 2, 4), cr(factory, 2, 4), br(factory, 2, 4), bl(factory, 3, 4), cl(factory, 3, 4), cc(factory, 3, 4), cr(factory, 3, 4), br(factory, 3, 4), bl(factory, 4, 4), cl(factory, 4, 4), cc(factory, 4, 4), cr(factory, 4, 4), br(factory, 4, 4), hr(factory, 4)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", cdl(factory, 0, 4), cd(factory, 0, 4), cdr(factory, 0, 4), cdl(factory, 1, 4), cd(factory, 1, 4), cdr(factory, 1, 4), cdl(factory, 2, 4), cd(factory, 2, 4), cdr(factory, 2, 4), cdl(factory, 3, 4), cd(factory, 3, 4), cdr(factory, 3, 4), cdl(factory, 4, 4), cd(factory, 4, 4), cdr(factory, 4, 4)).to_string(),
    format!("│└─{}─┘└─{}─┘└─{}─┘└─{}─┘└─{}─┘│", bd(factory, 0, 4), bd(factory, 1, 4), bd(factory, 2, 4), bd(factory, 3, 4), bd(factory, 4, 4)).to_string(),
    format!("└──{}────{}────{}────{}────{}──┘", hd(factory, 0), hd(factory, 1), hd(factory, 2), hd(factory, 3), hd(factory, 4)).to_string(),
  );
}

pub fn serialize_cli(factory: &Factory) -> String {
  let lines = serialize_cli_lines(factory);
  return lines.into_iter().collect::<Vec<String>>().join("\n");
}
