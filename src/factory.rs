use std::collections::VecDeque;

use super::belt::*;
use super::cell::*;
use super::floor::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::segment::*;
use super::state::*;
use super::supply::*;

const ONE_SECOND: u64 = 10000;

#[derive(Clone, Copy, Debug, PartialEq)]
enum HasHole {
  None,
  Supply,
  Demand,
}

pub struct Factory {
  pub ticks: u64,
  pub floor: Floor,
  pub fsum: usize, // floor.width * floor.height
  pub prio: Vec<usize>,
  pub stats: (VecDeque<(i32, u64)>, usize, i32, i32, i32), // ((price_delta, time), one_second_size, one_second_sum, ten_second_sum, belt_cell_count)
}

pub fn create_factory(options: &mut Options, _state: &mut State) -> Factory {
  let mut floor = test_floor();
  let fsum = (floor.width + 2) * (floor.height + 2);

  let prio: Vec<usize> = create_prio_list(options, &mut floor);
  println!("The prio coords: {:?}", prio);
  let kinds : Vec<CellKind> = prio.iter().map(|coord| floor.cells[*coord].kind).collect();
  println!("The prio cells : {:?}", kinds);
  println!("\n\n");

  return Factory {
    ticks: 0,
    floor,
    fsum,
    prio,
    stats: (
      VecDeque::new(), // vec<(cost_delta, at_tick)> // Up to 100k ticks worth of cost deltas
      0, // one_second_size // Number of elements in cost_deltas that wrap the last 10k ticks
      0, // one_second_total // Efficiency of last 10k ticks
      0, // ten_second_total // Efficiency of last 100k ticks (total of cost_deltas)
      14, // number of belt cells
    ),
  };
}

fn should_mark_belt_check_port(cell2: &Cell, port: Port, port2: Port) -> bool {
  // This cell is next in priority when each port must be one of;
  // - a none port
  // - an inbound port
  // - connected to a cell that's already marked
  // - (connected to an empty cell; these are auto-marked)
  // - connected to a supply
  // - connected to the outbound port of a belt

  return cell2.marked ||
    port != Port::Outbound ||
    match cell2.kind {
      CellKind::Empty => panic!("empty cells should be .marked by default"),
      CellKind::Belt => {
        // Check if the outbound port is connected to the inbound port of a port
        // If this cell can deliver to cell2 then this cell should always be processed after
        // cell2 and since cell2 was not marked, this cell is not next.
        port2 != Port::Inbound
      }
      CellKind::Machine => {
        // Machines auto-configure the port to invert-match the other side
        // This cell should always be processed after the machine it supplies to and since
        // the machine was not .marked, this cell can not be next in line.
        false
      }
      CellKind::Supply => {
        // Outbound to suppliers? I think not. This connection is broken so ignore it.
        true
      }
      CellKind::Demand => panic!("all demanders should be .marked and not reach this point"),
    };
}
fn should_mark_machine_check_port(cell2: &Cell, port2: Port) -> bool {
  // Machines auto-configure their ports to invert-match the other side
  // Machines are next in the priority order when all the neighbors they can supply to are marked
  // They do not supply to anything other than belts

  return cell2.marked || cell2.kind != CellKind::Belt || port2 != Port::Inbound;
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
    CellKind::Empty => return false,
    CellKind::Belt => {
      // Check all ports
      if options.trace_priority_step {
        println!(
          "    - {:?} {:?} {:?} {:?} {:?},  {:?} {:?} {:?} {:?} {:?},  {:?} {:?} {:?} {:?} {:?},  {:?} {:?} {:?} {:?} {:?}",
          (cell_u.x, cell_u.y), cell_u.marked, cell_u.kind, cell.belt.direction_u, cell_u.belt.direction_d,
          (cell_r.x, cell_r.y), cell_r.marked, cell_r.kind, cell.belt.direction_r, cell_r.belt.direction_l,
          (cell_d.x, cell_d.y), cell_d.marked, cell_d.kind, cell.belt.direction_d, cell_d.belt.direction_u,
          (cell_l.x, cell_l.y), cell_l.marked, cell_l.kind, cell.belt.direction_l, cell_l.belt.direction_r,
        );
      }
      return
        should_mark_belt_check_port(cell_u, cell.belt.direction_u, cell_u.belt.direction_d) &&
          should_mark_belt_check_port(cell_r, cell.belt.direction_r, cell_r.belt.direction_l) &&
          should_mark_belt_check_port(cell_d, cell.belt.direction_d, cell_d.belt.direction_u) &&
          should_mark_belt_check_port(cell_l, cell.belt.direction_l, cell_l.belt.direction_r)
      ;
    },
    CellKind::Machine => {
      // Check all ports
      if options.trace_priority_step {
        println!(
          "    - {:?} {:?} {:?} {:?},  {:?} {:?} {:?} {:?},  {:?} {:?} {:?} {:?},  {:?} {:?} {:?} {:?}",
          (cell_u.x, cell_u.y), cell_u.marked, cell_u.kind, cell_u.belt.direction_d,
          (cell_r.x, cell_r.y), cell_r.marked, cell_r.kind, cell_r.belt.direction_l,
          (cell_d.x, cell_d.y), cell_d.marked, cell_d.kind, cell_d.belt.direction_u,
          (cell_l.x, cell_l.y), cell_l.marked, cell_l.kind, cell_l.belt.direction_r,
        );
      }
      return
        should_mark_machine_check_port(cell_u, cell_u.belt.direction_d) &&
          should_mark_machine_check_port(cell_r, cell_r.belt.direction_l) &&
          should_mark_machine_check_port(cell_d, cell_d.belt.direction_u) &&
          should_mark_machine_check_port(cell_l, cell_l.belt.direction_r)
      ;
    }
    CellKind::Supply => {
      // Supplies are always the last cell in the priority list so return false here
      false
    }
    CellKind::Demand => {
      panic!("Demands should be marked by now. This should not be reachable.");
    }
  }
}
pub fn create_prio_list(options: &Options, floor: &mut Floor) -> Vec<usize> {
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

  let w = floor.width + 2;
  let h = floor.height + 2;
  let fsum = floor.fsum;

  let mut demand_connects = vec!();
  for coord in 0..fsum {
    // Unmark all cells first
    floor.cells[coord].marked = false;

    if options.trace_priority_step { super::log(format!("- kind {} {:?} {:?} {} {}", coord, floor.cells[coord].kind, to_xy(coord, w), floor.cells[coord].x, floor.cells[coord].y).as_str()); }
    match floor.cells[coord].kind {
      CellKind::Demand => {
        out.push(coord);

        let (x, y) = to_xy(coord, w);
        let coord2 =
          if x == 0 {
            to_coord(w,x + 1, y)
          } else if y == 0 {
            to_coord(w, x, y + 1)
          } else if x == w - 1 {
            to_coord(w, x - 1, y)
          } else if y == h - 1 {
            to_coord(w, x, y - 1)
          } else {
            panic!("a demand must live on the edge of the floor");
          };

        floor.cells[coord].marked = true;
        floor.cells[coord2].marked = true;
        if options.trace_priority_step { super::log(format!("- Adding {} as the cell that is connected to a Demand at {}", coord2, coord).as_str()); }
        demand_connects.push(coord2);
      }
      CellKind::Empty => {
        floor.cells[coord].marked = true;
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
    super::log(format!("- after step 1 {:?}", out).as_str());
  }

  // super::log(format!("- iteratively follow the trail").as_str());
  let mut stepped = 1;
  let mut found_something = true;
  let mut some_left = true;
  let w = floor.width + 2;
  let h = floor.height + 2;
  while found_something && some_left {
    stepped += 1;
    // super::log(format!("  - loop").as_str());
    found_something = false;

    // Only walk inside cells. Assume there's always an up/right/down/left neighbor cell.
    for coord in w..fsum-w {
      if coord % w == 0 || coord % w == w - 1 { continue; } // Skip edge cells (none/supply/demand)

      if options.trace_priority_step && !floor.cells[coord].marked { super::log(format!(" - kind {} {:?} {:?} {} {}", coord, floor.cells[coord].kind, to_xy(coord, w), floor.cells[coord].x, floor.cells[coord].y).as_str()); }
      if
      should_be_marked_for_cell_sorting(
        options,
        &floor.cells[coord],
        &floor.cells[to_coord_up(coord, w)],
        &floor.cells[to_coord_right(coord, w)],
        &floor.cells[to_coord_down(coord, w)],
        &floor.cells[to_coord_left(coord, w)],
      )
      {
        if options.trace_priority_step { super::log(format!("    - adding {:?}", to_xy(coord, w)).as_str()); }
        floor.cells[coord].marked = true;
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
  for coord in 0..fsum {
    let (x, y) = to_xy(coord, w);
    if !floor.cells[coord].marked && floor.cells[coord].kind != CellKind::Empty {
      if options.trace_priority_step{ super::log(format!("  - adding {} {:?}", coord, (x, y)).as_str()); }
      out.push(coord);
      floor.cells[coord].marked = true; // Kinda pointless
    }
  }

  return out;
}

fn progress(ticks: u64, since: u64, range: u64) -> f64 {
  return (ticks - since) as f64 / (range as f64);
}

fn b2b_belts_connected_outbound_to_inbound(factory: &mut Factory, coord: usize, dir: SegmentDirection, ocoord: usize, odir: SegmentDirection) -> bool {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Belt, "ports_outbound_to_inbound; this cell should be asserted to be a belt: {:?}", factory.floor.cells[coord].kind);
  assert_eq!(factory.floor.cells[ocoord].kind, CellKind::Belt, "ports_outbound_to_inbound; this cell should be asserted to be a belt: {:?}", factory.floor.cells[ocoord].kind);

  // Check if the cell at coord is connected to the cell at ocoord from outbound to inbound.

  return factory.floor.cells[coord].segments[dir as usize].port == Port::Outbound && factory.floor.cells[ocoord].segments[odir as usize].port == Port::Inbound;
}
fn b2b_outbound_segment_ready_to_send(factory: &mut Factory, coord: usize, dir: SegmentDirection) -> bool {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Belt, "outbound_segment_ready_to_send; this cell should be asserted to be a belt: {:?}", factory.floor.cells[coord].kind);

  let cell = &factory.floor.cells[coord];
  let segment: &Segment = &cell.segments[dir as usize];

  // Check if the segment at coord is not empty and is allocated and on/over 100% progress

  return cell.kind == CellKind::Belt && segment.allocated && segment.part.kind != PartKind::None && (factory.ticks - segment.at) >= cell.speed;
}
fn b2b_inbound_segment_ready_to_receive(factory: &mut Factory, ocoord: usize, odir: SegmentDirection) -> bool {
  assert_eq!(factory.floor.cells[ocoord].kind, CellKind::Belt, "inbound_segment_ready_to_receive; this cell should be asserted to be a belt: {:?}", factory.floor.cells[ocoord].kind);

  let cell = &factory.floor.cells[ocoord];
  let segment: &Segment = &cell.segments[odir as usize];

  // Check if the segment at coord is not empty and is allocated and on/over 100% progress

  return segment.part.kind == PartKind::None || segment.allocated;
}
fn b2b_move_part(options: &mut Options, _state: &mut State, factory: &mut Factory, coord: usize, dir: SegmentDirection, ocoord: usize, odir: SegmentDirection) {
  if options.print_moves || options.print_moves_belt { println!("({}) b2b_move_part(options, state, {}:{:?}, {}:{:?})", factory.ticks, coord, dir, ocoord, odir); }

  factory.floor.cells[ocoord].segments[odir as usize].part = factory.floor.cells[coord].segments[dir as usize].part.clone();
  factory.floor.cells[ocoord].segments[odir as usize].at = factory.ticks;
  factory.floor.cells[ocoord].segments[odir as usize].from = dir;
  factory.floor.cells[ocoord].segments[odir as usize].allocated = false;
  factory.floor.cells[ocoord].segments[odir as usize].claimed = false;

  factory.floor.cells[coord].segments[dir as usize].part = part_none();
  factory.floor.cells[coord].segments[dir as usize].at = 0;
  factory.floor.cells[coord].segments[dir as usize].allocated = false;
  // factory.floor.cells[coord].segments[dir as usize].claimed = false;
}
fn b2b_outbound_segment_ready_to_allocate(factory: &mut Factory, coord: usize, dir: SegmentDirection) -> bool {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Belt, "outbound_segment_ready_to_send; this cell should be asserted to be a belt: {:?}", factory.floor.cells[coord].kind);

  let cell = &factory.floor.cells[coord];
  let segment: &Segment = &cell.segments[dir as usize];

  // Check if the segment at coord is outbound, not empty, is allocated and on/over 100% progress

  return segment.port == Port::Outbound && !segment.allocated && segment.part.kind != PartKind::None && (factory.ticks - segment.at) >= (cell.speed / 2);
}
fn b2b_inbound_segment_ready_to_claim(factory: &mut Factory, ocoord: usize, odir: SegmentDirection) -> bool {
  assert_eq!(factory.floor.cells[ocoord].kind, CellKind::Belt, "inbound_segment_ready_to_receive; this cell should be asserted to be a belt: {:?}", factory.floor.cells[ocoord].kind);

  let cell = &factory.floor.cells[ocoord];
  let segment: &Segment = &cell.segments[odir as usize];

  // Check if the segment at coord is inbound, and empty or allocated, and not yet claimed

  return segment.port == Port::Inbound && !segment.claimed && (segment.part.kind == PartKind::None || segment.allocated);
}
fn b2b_allocate_part(options: &mut Options, _state: &mut State, factory: &mut Factory, coord: usize, dir: SegmentDirection, ocoord: usize, odir: SegmentDirection) {
  if options.print_choices || options.print_choices_belt { println!("({}) b2b_allocate_part({}:{:?}, {}:{:?})", factory.ticks, coord, dir, ocoord, odir); }

  factory.floor.cells[coord].segments[dir as usize].allocated = true;
  factory.floor.cells[ocoord].segments[odir as usize].claimed = true;
}
fn b2b_step(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize, dir: SegmentDirection, ocoord: usize, odir: SegmentDirection) {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Belt, "b2b_step(); this cell should be asserted to be a belt: {:?}", factory.floor.cells[coord].kind);
  // Note: ocoord may not be a belt

  if factory.floor.cells[ocoord].kind != CellKind::Belt {
    return;
  }
  // Ok coord is a belt :)

  // if coord == 10 {
  //   let ticks = factory.ticks;
  //   println!("({}) {}:{:?}~{}:{:?} ->  {}", ticks, coord, dir, ocoord, odir, b2b_belts_connected_outbound_to_inbound(factory, coord, dir, ocoord, odir));
  // }


  // When moving between belt cells, we only move from outbound segment to inbound segment

  if b2b_belts_connected_outbound_to_inbound(factory, coord, dir, ocoord, odir) {

    // if coord == 10 {
    //   let ticks = factory.ticks;
    //   println!("({}) {}:{:?}~{}:{:?} ->  {} {}", ticks, coord, dir, ocoord, odir, b2b_outbound_segment_ready_to_send(factory, coord, dir), b2b_outbound_segment_ready_to_allocate(factory, coord, dir));
    // }


    if b2b_outbound_segment_ready_to_send(factory, coord, dir) {
      // if coord == 10 {
      //   let ticks = factory.ticks;
      //   println!("({}) {}:{:?}~{}:{:?} ->  no no this was fine", ticks, coord, dir, ocoord, odir, );
      // }

      if b2b_inbound_segment_ready_to_receive(factory, ocoord, odir) {
        b2b_move_part(options, state, factory, coord, dir, ocoord, odir);
      }
    } else if b2b_outbound_segment_ready_to_allocate(factory, coord, dir) {
      // if coord == 10 {
      //   let ticks = factory.ticks;
      //   println!("({}) {}:{:?}~{}:{:?} ->  {} ({:?} {} {:?} {})", ticks, coord, dir, ocoord, odir, b2b_inbound_segment_ready_to_claim(factory, ocoord, odir),
      //
      //     factory.floor.cells[ocoord].segments[odir as usize].port,
      //     !!factory.floor.cells[ocoord].segments[odir as usize].claimed,
      //     factory.floor.cells[ocoord].segments[odir as usize].part.kind,
      //     !!factory.floor.cells[ocoord].segments[odir as usize].allocated,
      //   );
      // }
      if b2b_inbound_segment_ready_to_claim(factory, ocoord, odir) {
        // Okay. We should be able to allocate the part to this neighbor.
        // Note: this is not the actual move yet. But barring user interaction, it will move.
        b2b_allocate_part(options, state, factory, coord, dir, ocoord, odir);
      }
    }
  }
}

fn m2b_outbound_segment_ready_to_send(factory: &mut Factory, ocoord: usize, dir: SegmentDirection) -> bool {
  let cell = &factory.floor.cells[ocoord];
  let segment: &Segment = &cell.segments[dir as usize];

  // Check if the segment at coord is not empty and is allocated and on/over 100% progress

  return cell.kind == CellKind::Belt && segment.allocated && segment.part.kind != PartKind::None && (factory.ticks - segment.at) >= cell.speed;
}
fn m2b_move(options: &mut Options, _state: &mut State, factory: &mut Factory, coord: usize, ocoord: usize, odir: SegmentDirection) {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Machine, "m2b_move; this cell should be asserted to be a machine: {:?}", factory.floor.cells[ocoord].kind);
  assert_eq!(factory.floor.cells[ocoord].kind, CellKind::Belt, "m2b_move; this cell should be asserted to be a belt: {:?}", factory.floor.cells[ocoord].kind);
  assert_eq!(factory.floor.cells[ocoord].segments[odir as usize].port, Port::Inbound, "m2b_move; belt should be inbound (to receive the created part)");

  if options.print_moves || options.print_moves_belt { println!("({}) m2b_move(options, state, {}, {}:{:?})", factory.ticks, coord, ocoord, odir); }

  factory.floor.cells[ocoord].segments[odir as usize].part = factory.floor.cells[coord].machine_output_have.clone();
  factory.floor.cells[ocoord].segments[odir as usize].at = factory.ticks;
  factory.floor.cells[ocoord].segments[odir as usize].from = odir;
  factory.floor.cells[ocoord].segments[odir as usize].allocated = false;
  factory.floor.cells[ocoord].segments[odir as usize].claimed = false;

  factory.floor.cells[coord].machine_output_have = part_none();
}
fn m2b_inbound_segment_ready_to_receive(factory: &mut Factory, ocoord: usize, odir: SegmentDirection) -> bool {
  let cell = &factory.floor.cells[ocoord];
  let segment: &Segment = &cell.segments[odir as usize];

  // Check if the segment is in a belt, on an inbound segment, and empty.
  // We can ignore allocated, progress, and claimed in this case.

  return cell.kind == CellKind::Belt && cell.kind == CellKind::Belt && segment.port == Port::Inbound && segment.part.kind == PartKind::None;
}

fn b2m_can_move(factory: &mut Factory, coord: usize, ocoord: usize, odir: SegmentDirection) -> bool {
  if m2b_outbound_segment_ready_to_send(factory, ocoord, odir) {
    let kind = factory.floor.cells[ocoord].segments[odir as usize].part.kind;
    let machine = &factory.floor.cells[coord];
    return
      machine.machine_input_1_have.kind == PartKind::None ||
      machine.machine_input_2_have.kind == PartKind::None ||
      machine.machine_input_3_have.kind == PartKind::None
    ;
  }

  return false;
}
fn b2m_move(options: &mut Options, _state: &mut State, factory: &mut Factory, coord: usize, ocoord: usize, odir: SegmentDirection) {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Machine, "b2m_move; this cell should be asserted to be a machine: {:?}", factory.floor.cells[ocoord].kind);
  assert_eq!(factory.floor.cells[ocoord].kind, CellKind::Belt, "b2m_move; this cell should be asserted to be a belt: {:?}", factory.floor.cells[ocoord].kind);
  assert_eq!(factory.floor.cells[ocoord].segments[odir as usize].port, Port::Outbound, "b2m_move; belt should be outbound");

  let new_part = factory.floor.cells[ocoord].segments[odir as usize].part.clone();
  if options.print_moves || options.print_moves_belt { println!("({}) b2m_move(a: {:?}, from: {}:{:?}, into: {})", factory.ticks, new_part.kind, ocoord, odir, coord); }


  // Put it in the right input slot. One should be available as that check happened before this call.
  if factory.floor.cells[coord].machine_input_1_want.kind != factory.floor.cells[coord].machine_input_1_have.kind && factory.floor.cells[coord].machine_input_1_want.kind == new_part.kind {
    factory.floor.cells[coord].machine_input_1_have = new_part;
  } else if factory.floor.cells[coord].machine_input_2_want.kind != factory.floor.cells[coord].machine_input_2_have.kind && factory.floor.cells[coord].machine_input_2_want.kind == new_part.kind {
    factory.floor.cells[coord].machine_input_2_have = new_part;
  } else if factory.floor.cells[coord].machine_input_3_want.kind != factory.floor.cells[coord].machine_input_3_have.kind && factory.floor.cells[coord].machine_input_3_want.kind == new_part.kind {
    factory.floor.cells[coord].machine_input_3_have = new_part;
  }
  // Okay, put it in any and trash it
  else if factory.floor.cells[coord].machine_input_1_want.kind != factory.floor.cells[coord].machine_input_1_have.kind {
    factory.floor.cells[coord].machine_input_1_have = new_part;
  } else if factory.floor.cells[coord].machine_input_2_want.kind != factory.floor.cells[coord].machine_input_2_have.kind {
    factory.floor.cells[coord].machine_input_2_have = new_part;
  } else if factory.floor.cells[coord].machine_input_3_want.kind != factory.floor.cells[coord].machine_input_3_have.kind {
    factory.floor.cells[coord].machine_input_3_have = new_part;
  }
  else {
    panic!("there should have been at least one input empty that wanted this part...");
  }

  factory.floor.cells[ocoord].segments[odir as usize].part = part_none();
  factory.floor.cells[ocoord].segments[odir as usize].at = 0;
  factory.floor.cells[ocoord].segments[odir as usize].allocated = false;
  // factory.floor.cells[ocoord].segments[odir as usize].claimed = false;
}

fn b2m_outbound_segment_ready_to_allocate(factory: &mut Factory, coord: usize, dir: SegmentDirection) -> bool {
  let cell = &factory.floor.cells[coord];
  let segment: &Segment = &cell.segments[dir as usize];

  // Check if the segment at coord is outbound, not empty, is allocated and on/over 100% progress

  return cell.kind == CellKind::Belt && segment.port == Port::Outbound && !segment.allocated && segment.part.kind != PartKind::None && (factory.ticks - segment.at) >= (cell.speed / 2);
}
fn b2m_can_allocate(factory: &mut Factory, coord: usize, ocoord: usize, odir: SegmentDirection) -> bool {
  if b2m_outbound_segment_ready_to_allocate(factory, ocoord, odir) {
    let kind = factory.floor.cells[ocoord].segments[odir as usize].part.kind;
    let machine = &factory.floor.cells[coord];
    return
      machine.machine_input_1_have.kind == PartKind::None ||
      machine.machine_input_2_have.kind == PartKind::None ||
      machine.machine_input_3_have.kind == PartKind::None
    ;
  }

  return false;
}
fn b2m_allocate(options: &mut Options, _state: &mut State, factory: &mut Factory, coord: usize, ocoord: usize, odir: SegmentDirection) {
  if options.print_choices || options.print_choices_belt { println!("({}) b2m_allocate({}, {}:{:?})", factory.ticks, coord, ocoord, odir); }

  factory.floor.cells[ocoord].segments[odir as usize].allocated = true;

  if factory.floor.cells[coord].machine_input_1_want.kind == factory.floor.cells[ocoord].segments[odir as usize].part.kind && factory.floor.cells[coord].machine_input_1_have.kind == PartKind::None {
    factory.floor.cells[coord].machine_input_1_claimed = true;
  } else if factory.floor.cells[coord].machine_input_2_want.kind == factory.floor.cells[ocoord].segments[odir as usize].part.kind && factory.floor.cells[coord].machine_input_2_have.kind == PartKind::None {
    factory.floor.cells[coord].machine_input_2_claimed = true;
  } else if factory.floor.cells[coord].machine_input_3_want.kind == factory.floor.cells[ocoord].segments[odir as usize].part.kind && factory.floor.cells[coord].machine_input_3_have.kind == PartKind::None {
    factory.floor.cells[coord].machine_input_3_claimed = true;
  }
  // Will need to trash the part into whatever slot has space for it
  else if factory.floor.cells[coord].machine_input_1_have.kind == PartKind::None {
    factory.floor.cells[coord].machine_input_1_claimed = true;
  } else if factory.floor.cells[coord].machine_input_2_have.kind == PartKind::None {
    factory.floor.cells[coord].machine_input_2_claimed = true;
  } else if factory.floor.cells[coord].machine_input_3_have.kind == PartKind::None {
    factory.floor.cells[coord].machine_input_3_claimed = true;
  }
  else {
    panic!("One slot should be available at this point...");
  }
}

fn c2e_ready_to_send(factory: &mut Factory, coord: usize) -> bool {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Belt, "c2e_ready_to_send; this cell should be asserted to be a belt: {:?}", factory.floor.cells[coord].kind);

  let cell = &factory.floor.cells[coord];
  let segment: &Segment = &cell.segments[SegmentDirection::CENTER as usize];

  // Check if the center segment is allocated and on/over 100% progress

  return segment.allocated && (factory.ticks - segment.at) >= cell.speed;
}
fn c2e_ready_to_receive(factory: &mut Factory, coord: usize, dir: SegmentDirection) -> bool {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Belt, "c2e_ready_to_receive; this cell should be asserted to be a belt: {:?}", factory.floor.cells[coord].kind);

  let cell = &factory.floor.cells[coord];
  let segment: &Segment = &cell.segments[dir as usize];

  // Check if the segment at coord is not empty and is allocated and on/over 100% progress

  return segment.port == Port::Outbound && (segment.part.kind == PartKind::None || segment.allocated);
}
fn c2e_move_part(options: &mut Options, _state: &mut State, factory: &mut Factory, coord: usize, dir: SegmentDirection) {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Belt, "c2e_move_part; this cell should be asserted to be a belt: {:?}", factory.floor.cells[coord].kind);
  assert_eq!(factory.floor.cells[coord].segments[dir as usize].port, Port::Outbound, "c2e_move_part; the edge should be asserted to be outbound: {:?}", factory.floor.cells[coord].kind);

  if options.print_moves || options.print_moves_belt { println!("({}) c2e_move_part(options, state, {}:{:?})", factory.ticks, coord, dir); }

  factory.floor.cells[coord].segments[dir as usize].part = factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].part.clone();
  factory.floor.cells[coord].segments[dir as usize].at = factory.ticks;
  factory.floor.cells[coord].segments[dir as usize].from = dir;
  factory.floor.cells[coord].segments[dir as usize].allocated = false;
  factory.floor.cells[coord].segments[dir as usize].claimed = false;

  factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].part = part_none();
  factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].at = 0;
  factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].allocated = false;
  // factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].claimed = false;
}
fn c2e_ready_to_allocate(factory: &mut Factory, coord: usize) -> bool {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Belt, "c2e_ready_to_allocate; this cell should be asserted to be a belt: {:?}", factory.floor.cells[coord].kind);

  let cell = &factory.floor.cells[coord];
  let segment: &Segment = &cell.segments[SegmentDirection::CENTER as usize];

  // Check if the center segment is not empty, not allocated, and on/over 50% progress

  return !segment.allocated && segment.part.kind != PartKind::None && (factory.ticks - segment.at) >= (cell.speed / 2);
}
fn c2e_ready_to_claim(factory: &mut Factory, coord: usize, dir: SegmentDirection) -> bool {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Belt, "c2e_ready_to_claim; this cell should be asserted to be a belt: {:?}", factory.floor.cells[coord].kind);

  let cell = &factory.floor.cells[coord];
  let segment: &Segment = &cell.segments[dir as usize];

  // Check if the edge segment is outbound, and empty or allocated, and not yet claimed

  return segment.port == Port::Outbound && segment.port == Port::Outbound && !segment.claimed && (segment.part.kind == PartKind::None || segment.allocated);
}
fn c2e_allocate_part(options: &mut Options, _state: &mut State, factory: &mut Factory, coord: usize, dir: SegmentDirection) {
  if options.print_choices || options.print_choices_belt { println!("({}) c2e_allocate_part({}:{:?})", factory.ticks, coord, dir); }

  factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].allocated = true;
  factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].to = dir;
  factory.floor.cells[coord].segments[dir as usize].claimed = true;
}

fn e2c_ready_to_receive(factory: &mut Factory, coord: usize) -> bool {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Belt, "e2c_ready_to_receive; this cell should be asserted to be a belt: {:?}", factory.floor.cells[coord].kind);

  let cell = &factory.floor.cells[coord];
  let segment: &Segment = &cell.segments[SegmentDirection::CENTER as usize];

  // Check if the center segment at coord is empty

  return segment.part.kind == PartKind::None || segment.allocated
}
fn e2c_ready_to_send(factory: &mut Factory, coord: usize, dir: SegmentDirection) -> bool {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Belt, "e2c_ready_to_send; this cell should be asserted to be a belt: {:?}", factory.floor.cells[coord].kind);

  let cell = &factory.floor.cells[coord];
  let segment: &Segment = &cell.segments[dir as usize];

  assert!(!segment.allocated || segment.part.kind != PartKind::None);

  // Check if the segment is inbound, not empty, and is allocated and on/over 100% progress
  // (If the element is allocated then it must not be empty)

  return segment.port == Port::Inbound && segment.allocated && (factory.ticks - segment.at) >= cell.speed;
}
fn e2c_move_part(options: &mut Options, _state: &mut State, factory: &mut Factory, coord: usize, dir: SegmentDirection) {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Belt, "e2c_move_part; this cell should be asserted to be a belt: {:?}", factory.floor.cells[coord].kind);
  assert_eq!(factory.floor.cells[coord].segments[dir as usize].port, Port::Inbound, "e2c_move_part; the edge should be asserted to be inbound: {:?}", factory.floor.cells[coord].kind);
  assert_ne!(factory.floor.cells[coord].segments[dir as usize].part.kind, PartKind::None, "e2c_move_part; part not empty; dir={:?}", dir);
  assert_eq!(factory.floor.cells[coord].segments[dir as usize].allocated, true, "e2c_move_part; part allocated; dir={:?}", dir);
  assert_eq!(factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].claimed, true, "e2c_move_part; center claimed");

  if options.print_moves || options.print_moves_belt { println!("({}) e2c_move_part(options, state, {}:{:?})", factory.ticks, coord, dir); }

  factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].part = factory.floor.cells[coord].segments[dir as usize].part.clone();
  factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].at = factory.ticks;
  factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].from = dir;
  factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].allocated = false;
  factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].claimed = false;

  factory.floor.cells[coord].segments[dir as usize].part = part_none();
  factory.floor.cells[coord].segments[dir as usize].at = 0;
  factory.floor.cells[coord].segments[dir as usize].allocated = false;
  // factory.floor.cells[coord].segments[dir as usize].claimed = false;
}
fn e2c_ready_to_allocate(factory: &mut Factory, coord: usize, dir: SegmentDirection) -> bool {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Belt, "e2c_ready_to_allocate; this cell should be asserted to be a belt: {:?}", factory.floor.cells[coord].kind);

  let cell = &factory.floor.cells[coord];
  let segment: &Segment = &cell.segments[dir as usize];

  // Check if the edge segment is inbound, not empty, not allocated, and on/over 50% progress

  return segment.port == Port::Inbound && !segment.allocated && segment.part.kind != PartKind::None && (factory.ticks - segment.at) >= (cell.speed / 2);
}
fn e2c_ready_to_claim(factory: &mut Factory, coord: usize) -> bool {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Belt, "e2c_ready_to_claim; this cell should be asserted to be a belt: {:?}", factory.floor.cells[coord].kind);

  let cell = &factory.floor.cells[coord];
  let segment: &Segment = &cell.segments[SegmentDirection::CENTER as usize];

  // Check if the center segment at coord is empty or allocated, and not yet claimed

  return !segment.claimed && (segment.part.kind == PartKind::None || segment.allocated);
}
fn e2c_allocate_part(options: &mut Options, _state: &mut State, factory: &mut Factory, coord: usize, dir: SegmentDirection) {
  if options.print_choices || options.print_choices_belt { println!("({}) e2c_allocate_part({}, {:?})", factory.ticks, coord, dir); }

  factory.floor.cells[coord].segments[dir as usize].allocated = true;
  factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].claimed = true;
}

fn segment_part_needs_move(factory: &Factory, coord: usize, dir: SegmentDirection) -> bool {
  // Does this belt segment contain a part that is allocated and on/over 100% progress?

  let ticks = factory.ticks;
  let cell = &factory.floor.cells[coord];
  let segment = &cell.segments[dir as usize];

  assert_eq!(cell.kind, CellKind::Belt, "segment_part_needs_move; this cell should be asserted to be a belt: {:?}", cell.kind);

  // A Part on a segment needs to move if it exists, is over 100%, and is allocated
  true &&
    // The Part has been allocated to a neighbor
    segment.allocated &&
    // The part exists
    segment.part.kind != PartKind::None &&
    // The part is at the end of the segment
    (ticks - segment.at) >= cell.speed
}
fn is_segment_part_and_needs_move(factory: &Factory, coord: usize, dir: SegmentDirection) -> bool {
  return factory.floor.cells[coord].kind == CellKind::Belt && segment_part_needs_move(factory, coord, dir);
}

fn cell_can_receive_machine_part(factory: &Factory, coord: usize, dir: SegmentDirection) -> bool {
  // We can _move_ a part to a segment if the segment is empty

  let ticks = factory.ticks;
  let cell = &factory.floor.cells[coord];
  let segment = &cell.segments[dir as usize];

  true &&
    // Must be a belt
    cell.kind == CellKind::Belt &&
    // Target segment must be inport
    segment.port == Port::Inbound &&
    // Segment is empty (it has no part)
    segment.part.kind == PartKind::None
}

fn move_part_from_machine_to_belt(options: &mut Options, _state: &mut State, factory: &mut Factory, coord: usize, ocoord: usize, odir: SegmentDirection) {
  if options.print_moves || options.print_moves_machine { println!("({}) move_part_from_machine_to_belt(options, state, options, state, {}, {}, {:?})", factory.ticks, coord, ocoord, odir); }

  factory.floor.cells[ocoord].segments[odir as usize].part = factory.floor.cells[coord].machine_output_have.clone();
  factory.floor.cells[ocoord].segments[odir as usize].at = factory.ticks;
  factory.floor.cells[ocoord].segments[odir as usize].from = odir;
  factory.floor.cells[ocoord].segments[odir as usize].allocated = false;
  factory.floor.cells[ocoord].segments[odir as usize].claimed = false;
  factory.floor.cells[coord].machine_output_have = part_none();
}
fn move_part_from_belt_to_machine(options: &mut Options, _state: &mut State, factory: &mut Factory, coord: usize, ocoord: usize, dir: SegmentDirection) {
  if options.print_moves || options.print_moves_machine { println!("({}) move_part_from_belt_to_machine(options, state, options, state, {}, {}, {:?})", factory.ticks, coord, ocoord, dir); }

  factory.floor.cells[ocoord].machine_output_have = factory.floor.cells[coord].segments[dir as usize].part.clone();
  factory.floor.cells[coord].segments[dir as usize].part = part_none();
  factory.floor.cells[coord].segments[dir as usize].at = 0;
  factory.floor.cells[coord].segments[dir as usize].allocated = false;
  factory.floor.cells[coord].segments[dir as usize].claimed = false;
}
fn c2s_allocate_to_outbound_segment(options: &mut Options, _state: &mut State, factory: &mut Factory, coord: usize, ocoord: usize, dir: SegmentDirection, odir: SegmentDirection) {
  if options.print_choices || options.print_choices_belt { println!("({}) c2s_allocate_to_outbound_segment({}, {}, {:?}, {:?})", factory.ticks, coord, ocoord, dir, odir); }

  factory.floor.cells[coord].segments[dir as usize].allocated = true;
  factory.floor.cells[ocoord].segments[odir as usize].claimed = true;
}

pub fn can_move_part_from_supply_to_cell(factory: &Factory, ocoord: usize, odir: SegmentDirection) -> bool {
  // We can _move_ a part to a segment if the segment is empty

  let ticks = factory.ticks;
  let cell = &factory.floor.cells[ocoord];
  let segment = &cell.segments[odir as usize];

  true &&
    // Must be a belt
    cell.kind == CellKind::Belt &&
    // Target segment must be inport
    segment.port == Port::Inbound &&
    // Segment is empty (it has no part)
    segment.part.kind == PartKind::None
}
pub fn move_part_from_supply_to_segment(options: &mut Options, _state: &mut State, factory: &mut Factory, coord: usize, ocoord: usize, odir: SegmentDirection) {
  if options.print_moves || options.print_moves_supply { println!("({}) move_part_from_supply_to_segment({}, {}, {:?})", factory.ticks, coord, ocoord, odir); }

  factory.floor.cells[ocoord].segments[odir as usize].part = factory.floor.cells[coord].supply_part.clone();
  factory.floor.cells[ocoord].segments[odir as usize].at = factory.ticks;
  // factory.floor.cells[coord].segments[dir as usize].from = ; // irrelevant?
  factory.floor.cells[ocoord].segments[odir as usize].allocated = false;
  factory.floor.cells[ocoord].segments[odir as usize].claimed = false;
  factory.floor.cells[coord].supply_part = part_none();
  factory.floor.cells[coord].supply_part_at = 0;
  factory.floor.cells[coord].supply_last_part_out_at = factory.ticks;
}

fn b2d_outbound_segment_ready_to_send(factory: &mut Factory, ocoord: usize, odir: SegmentDirection) -> bool {
  if factory.floor.cells[ocoord].kind != CellKind::Demand {
    // Note: ocoord is the "other side from the demand" and must be a belt or else it wont work
    return false;
  }

  let cell = &factory.floor.cells[ocoord];
  let segment: &Segment = &cell.segments[odir as usize];

  // Check if the segment at coord is not empty and on/over 100% progress
  // Can ignore allocated in this case, not relevant for the demand (but has to be set anyways)

  return cell.kind == CellKind::Belt && segment.part.kind != PartKind::None && (factory.ticks - segment.at) >= cell.speed;
}
fn b2d_move_part(options: &mut Options, _state: &mut State, factory: &mut Factory, coord: usize, ocoord: usize, odir: SegmentDirection) {
  if options.print_moves || options.print_moves_belt { println!("({}) b2d_move_part(a: {:?}, from: {}:{:?}, to: {})", factory.ticks, factory.floor.cells[ocoord].segments[odir as usize].part.kind, ocoord, odir, ocoord); }

  factory.floor.cells[ocoord].segments[odir as usize].part = part_none();
  factory.floor.cells[ocoord].segments[odir as usize].at = 0;
  factory.floor.cells[ocoord].segments[odir as usize].allocated = false;
}
fn b2d_outbound_segment_ready_to_allocate(factory: &mut Factory, coord: usize, dir: SegmentDirection) -> bool {
  assert_eq!(factory.floor.cells[coord].kind, CellKind::Belt, "b2d_outbound_segment_ready_to_allocate; this cell should be asserted to be a belt: {:?}", factory.floor.cells[coord].kind);

  let cell = &factory.floor.cells[coord];
  let segment: &Segment = &cell.segments[dir as usize];

  // Check if the segment at coord is outbound, not empty, is allocated and on/over 100% progress

  return segment.port == Port::Outbound && !segment.allocated && segment.part.kind != PartKind::None && (factory.ticks - segment.at) >= (cell.speed / 2);
}
fn b2d_allocate_part(options: &mut Options, _state: &mut State, factory: &mut Factory, coord: usize, ocoord: usize, odir: SegmentDirection) {
  if options.print_moves || options.print_moves_belt { println!("({}) b2d_allocate_part(a: {:?}, from: {}:{:?}, to: {})", factory.ticks, factory.floor.cells[ocoord].segments[odir as usize].part.kind, ocoord, odir, ocoord); }

  factory.floor.cells[ocoord].segments[odir as usize].allocated = true;
}

pub fn tick_factory(options: &mut Options, state: &mut State, factory: &mut Factory) {
  factory.ticks += 1;
  let ticks = factory.ticks;

  let w = factory.floor.width + 2;
  let h = factory.floor.height + 2;

  for n in 0..factory.prio.len() {
    let coord = factory.prio[n];
    factory.floor.cells[coord].ticks += 1;

    match factory.floor.cells[coord].kind {
      CellKind::Empty => panic!("should not have empty cells in the prio list:: prio index: {}, coord: {}, cell: {:?}", n, coord, factory.floor.cells[coord]),
      CellKind::Belt => {
        // Each Belt Cell has one Center Segment and two to four Edge Segments
        // Edge Segments only gives parts to other, neighbouring, Edge Segments (out->in)
        // Center Segments give and take from Edge Segments, do the prioritization themselves

        if coord == 10 && ticks > 1000009 { panic!("exit2"); }

        let coord_u = to_coord_up(coord, w);
        let coord_r = to_coord_right(coord, w);
        let coord_d = to_coord_down(coord, w);
        let coord_l = to_coord_left(coord, w);

        // Start by ticking the OutPort Edge Segments.
        // Belts can not appear on the edge so it should not lead to array access oob
        // Only move from outbound segment to inbound segment.
        b2b_step(options, state,factory, coord, SegmentDirection::UP, coord_u, SegmentDirection::DOWN);
        b2b_step(options, state, factory, coord, SegmentDirection::RIGHT, coord_r, SegmentDirection::LEFT);
        b2b_step(options, state, factory, coord, SegmentDirection::DOWN, coord_d, SegmentDirection::UP);
        b2b_step(options, state, factory, coord, SegmentDirection::LEFT, coord_l, SegmentDirection::RIGHT);

        // Next do the Center Segment. First give parts or otherwise allocate them, then take parts.

        // if coord == 10 { println!("({}) ready to allocate c2e from {}? {} and {} {} {} {} ({} {} {} {}) ready to send? {} {} {}, ready to receive? {}",
        //   ticks,
        //   coord,
        //   c2e_ready_to_allocate(factory, coord),
        //   c2e_ready_to_claim(factory, coord, SegmentDirection::UP),
        //   c2e_ready_to_claim(factory, coord, SegmentDirection::RIGHT),
        //   c2e_ready_to_claim(factory, coord, SegmentDirection::DOWN),
        //   c2e_ready_to_claim(factory, coord, SegmentDirection::LEFT),
        //   factory.floor.cells[coord].segments[SegmentDirection::LEFT as usize].port == Port::Outbound,
        //   !!factory.floor.cells[coord].segments[SegmentDirection::LEFT as usize].claimed,
        //   factory.floor.cells[coord].segments[SegmentDirection::LEFT as usize].part.kind == PartKind::None,
        //   !!factory.floor.cells[coord].segments[SegmentDirection::LEFT as usize].allocated,
        //   c2e_ready_to_send(factory, coord),
        //   !!factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].allocated,
        //   (ticks - factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].at) > factory.floor.cells[coord].speed,
        //   c2e_ready_to_receive(factory, coord, port_dir_to_segment_dir(factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].to)),
        // )}

        // Get rid of the current part in case there is one and it is not yet allocated
        // If the part is not allocated yet, allocate it instead
        // If the part is allocated then it has a fixed port where it will go to
        if c2e_ready_to_send(factory, coord) {
          let to = factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].to;
          if c2e_ready_to_receive(factory, coord, to) {
            c2e_move_part(options, state, factory, coord, to);
          }
        } else if c2e_ready_to_allocate(factory, coord) {
          // Find an outbound edge segment that where we can allocate our part
          if c2e_ready_to_claim(factory, coord, SegmentDirection::UP) {
            c2e_allocate_part(options, state, factory, coord, SegmentDirection::UP);
          } else if c2e_ready_to_claim(factory, coord, SegmentDirection::RIGHT) {
            c2e_allocate_part(options, state, factory, coord, SegmentDirection::RIGHT);
          } else if c2e_ready_to_claim(factory, coord, SegmentDirection::DOWN) {
            c2e_allocate_part(options, state, factory, coord, SegmentDirection::DOWN);
          } else if c2e_ready_to_claim(factory, coord, SegmentDirection::LEFT) {
            c2e_allocate_part(options, state, factory, coord, SegmentDirection::LEFT);
          }
        }

        // if coord == 31 { println!("({}) from {}; {} {} {} {} ready to receive? {}, ready to send? {} ({} {} {}) {} {} {} {:?}",
        //   ticks,
        //   coord,
        //   coord_u,
        //   coord_r,
        //   coord_d,
        //   coord_l,
        //   e2c_ready_to_receive(factory, coord),
        //   e2c_ready_to_send(factory, coord, SegmentDirection::UP),
        //   factory.floor.cells[coord].segments[SegmentDirection::UP as usize].port == Port::Inbound,
        //   !!factory.floor.cells[coord].segments[SegmentDirection::UP as usize].allocated,
        //   (factory.ticks - factory.floor.cells[coord].segments[SegmentDirection::UP as usize].at) >= factory.floor.cells[coord].speed,
        //   e2c_ready_to_send(factory, coord, SegmentDirection::RIGHT),
        //   e2c_ready_to_send(factory, coord, SegmentDirection::DOWN),
        //   e2c_ready_to_send(factory, coord, SegmentDirection::LEFT),
        //   &factory.floor.cells[coord].segments[SegmentDirection::UP as usize]
        // )}
        // if coord == 10 && ticks > 10003 { panic!("exit1"); }

        // Acquire the next part, provided there is space
        if e2c_ready_to_receive(factory, coord) {
          if e2c_ready_to_send(factory, coord, SegmentDirection::UP) {
            e2c_move_part(options, state, factory, coord, SegmentDirection::UP);
          } else if e2c_ready_to_send(factory, coord, SegmentDirection::RIGHT) {
            e2c_move_part(options, state, factory, coord, SegmentDirection::RIGHT);
          } else if e2c_ready_to_send(factory, coord, SegmentDirection::DOWN) {
            e2c_move_part(options, state, factory, coord, SegmentDirection::DOWN);
          } else if e2c_ready_to_send(factory, coord, SegmentDirection::LEFT) {
            e2c_move_part(options, state, factory, coord, SegmentDirection::LEFT);
          }
        }

        // if coord == 10 { println!("({}) ready to claim? {} {} {} {}, ready to allocate? {} {} {} {}",
        //   ticks,
        //   e2c_ready_to_claim(factory, coord),
        //   !factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].claimed,
        //   factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].part.kind == PartKind::None,
        //   !!factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].allocated,
        //   e2c_ready_to_allocate(factory, coord, SegmentDirection::UP),
        //   e2c_ready_to_allocate(factory, coord, SegmentDirection::RIGHT),
        //   e2c_ready_to_allocate(factory, coord, SegmentDirection::DOWN),
        //   e2c_ready_to_allocate(factory, coord, SegmentDirection::LEFT)
        // )}
        // Find an Inbound edge segment that has a part ready to allocate to us
        if e2c_ready_to_claim(factory, coord) {
          if e2c_ready_to_allocate(factory, coord, SegmentDirection::UP) {
            e2c_allocate_part(options, state, factory, coord, SegmentDirection::UP);
          } else if e2c_ready_to_allocate(factory, coord, SegmentDirection::RIGHT) {
            e2c_allocate_part(options, state, factory, coord, SegmentDirection::RIGHT);
          } else if e2c_ready_to_allocate(factory, coord, SegmentDirection::DOWN) {
            e2c_allocate_part(options, state, factory, coord, SegmentDirection::DOWN);
          } else if e2c_ready_to_allocate(factory, coord, SegmentDirection::LEFT) {
            e2c_allocate_part(options, state, factory, coord, SegmentDirection::LEFT);
          }
        }
      }
      CellKind::Machine => {

        // - complete build in flight
        // - move output away
        // - try to start new build
        // - receive stuff
        // - try to start new build


        let coord_u = to_coord_up(coord, w);
        let coord_r = to_coord_right(coord, w);
        let coord_d = to_coord_down(coord, w);
        let coord_l = to_coord_left(coord, w);

        if factory.floor.cells[coord].machine_start_at > 0 && (factory.ticks - factory.floor.cells[coord].machine_start_at) > factory.floor.cells[coord].speed {
          // Finished constructing a part

          if options.print_machine_actions || options.print_moves_belt { println!("({}) machine {} finished creating part {:?}", factory.ticks, coord, factory.floor.cells[coord].machine_output_want.kind); }

          factory.floor.cells[coord].machine_start_at = 0;
          factory.floor.cells[coord].machine_output_have = factory.floor.cells[coord].machine_output_want.clone();
        }

        if factory.floor.cells[coord].machine_output_have.kind != PartKind::None {
          // There's a build output ready to move. Try to send it to a neighbor belt.
          if m2b_inbound_segment_ready_to_receive(factory, coord_u, SegmentDirection::DOWN) {
            m2b_move(options, state, factory, coord, coord_u, SegmentDirection::DOWN);
          } else if m2b_inbound_segment_ready_to_receive(factory, coord_r, SegmentDirection::LEFT) {
            m2b_move(options, state, factory, coord, coord_r, SegmentDirection::LEFT);
          } else if m2b_inbound_segment_ready_to_receive(factory, coord_d, SegmentDirection::UP) {
            m2b_move(options, state, factory, coord, coord_d, SegmentDirection::UP);
          } else if m2b_inbound_segment_ready_to_receive(factory, coord_l, SegmentDirection::RIGHT) {
            m2b_move(options, state, factory, coord, coord_l, SegmentDirection::RIGHT);
          }
        }

        // Start the machine if there are sufficient inputs and it's not running
        if
          factory.floor.cells[coord].machine_start_at == 0 &&
          factory.floor.cells[coord].machine_input_1_want.kind == factory.floor.cells[coord].machine_input_1_have.kind &&
          factory.floor.cells[coord].machine_input_2_want.kind == factory.floor.cells[coord].machine_input_2_have.kind &&
          factory.floor.cells[coord].machine_input_3_want.kind == factory.floor.cells[coord].machine_input_3_have.kind
        {
          if options.print_machine_actions || options.print_moves_belt { println!("({}) machine starting to create new part", factory.ticks); }

          factory.floor.cells[coord].machine_start_at = factory.ticks;
          factory.floor.cells[coord].machine_input_1_have = part_none();
          factory.floor.cells[coord].machine_input_2_have = part_none();
          factory.floor.cells[coord].machine_input_3_have = part_none();
        }

        // If there is space, try to acquire parts from neighbor belts
        // Machines only trash if there's an available spot to begin with. Otherwise ignore belts.
        if
          factory.floor.cells[coord].machine_input_1_want.kind != factory.floor.cells[coord].machine_input_1_have.kind ||
          factory.floor.cells[coord].machine_input_2_want.kind != factory.floor.cells[coord].machine_input_2_have.kind ||
          factory.floor.cells[coord].machine_input_3_want.kind != factory.floor.cells[coord].machine_input_3_have.kind
        {
          // Machine has at least one spot available. Try to receive parts from neighbor belt.
          // Machines do need to deal with the allocation/claim dance.
          if b2m_can_move(factory, coord, coord_u, SegmentDirection::DOWN) {
            b2m_move(options, state, factory, coord, coord_u, SegmentDirection::DOWN);
          } else if b2m_can_move(factory, coord, coord_r, SegmentDirection::LEFT) {
            b2m_move(options, state, factory, coord, coord_r, SegmentDirection::LEFT);
          } else if b2m_can_move(factory, coord, coord_d, SegmentDirection::UP) {
            b2m_move(options, state, factory, coord, coord_d, SegmentDirection::UP);
          } else if b2m_can_move(factory, coord, coord_l, SegmentDirection::RIGHT) {
            b2m_move(options, state, factory, coord, coord_l, SegmentDirection::RIGHT);
          }
          // If we moved none of them then try to allocate one
          else if b2m_can_allocate(factory, coord, coord_u, SegmentDirection::DOWN) {
            b2m_allocate(options, state, factory, coord, coord_u, SegmentDirection::DOWN);
          } else if b2m_can_allocate(factory, coord, coord_r, SegmentDirection::LEFT) {
            b2m_allocate(options, state, factory, coord, coord_r, SegmentDirection::LEFT);
          } else if b2m_can_allocate(factory, coord, coord_d, SegmentDirection::UP) {
            b2m_allocate(options, state, factory, coord, coord_d, SegmentDirection::UP);
          } else if b2m_can_allocate(factory, coord, coord_l, SegmentDirection::RIGHT) {
            b2m_allocate(options, state, factory, coord, coord_l, SegmentDirection::RIGHT);
          }
        }

        // Another machine start check, just in case the above move completed the inputs
        if
          factory.floor.cells[coord].machine_start_at == 0 &&
          factory.floor.cells[coord].machine_input_1_want.kind == factory.floor.cells[coord].machine_input_1_have.kind &&
          factory.floor.cells[coord].machine_input_2_want.kind == factory.floor.cells[coord].machine_input_2_have.kind &&
          factory.floor.cells[coord].machine_input_3_want.kind == factory.floor.cells[coord].machine_input_3_have.kind
        {
          if options.print_machine_actions || options.print_moves_belt { println!("({}) machine {} starting to create new part {:?}", factory.ticks, coord, factory.floor.cells[coord].machine_output_want.kind); }

          factory.floor.cells[coord].machine_start_at = factory.ticks;
          factory.floor.cells[coord].machine_input_1_have = part_none();
          factory.floor.cells[coord].machine_input_2_have = part_none();
          factory.floor.cells[coord].machine_input_3_have = part_none();
        }

        // Check if there's any part that needs trashing (slot not empty, not what it wanted)
        if factory.floor.cells[coord].machine_input_1_have.kind != PartKind::None && factory.floor.cells[coord].machine_input_1_want.kind != factory.floor.cells[coord].machine_input_1_have.kind {
          if options.print_machine_actions || options.print_moves_belt { println!("({}) machine {} trashed part in slot 1 ({:?})", factory.ticks, coord, factory.floor.cells[coord].machine_input_1_have.kind); }
          factory.floor.cells[coord].machine_input_1_have = part_none();
        }
        if factory.floor.cells[coord].machine_input_2_have.kind != PartKind::None && factory.floor.cells[coord].machine_input_2_want.kind != factory.floor.cells[coord].machine_input_2_have.kind {
          if options.print_machine_actions || options.print_moves_belt { println!("({}) machine {} trashed part in slot 1 ({:?})", factory.ticks, coord, factory.floor.cells[coord].machine_input_2_have.kind); }
          factory.floor.cells[coord].machine_input_2_have = part_none();
        }
        if factory.floor.cells[coord].machine_input_3_have.kind != PartKind::None && factory.floor.cells[coord].machine_input_3_want.kind != factory.floor.cells[coord].machine_input_3_have.kind {
          if options.print_machine_actions || options.print_moves_belt { println!("({}) machine {} trashed part in slot 1 ({:?})", factory.ticks, coord, factory.floor.cells[coord].machine_input_3_have.kind); }
          factory.floor.cells[coord].machine_input_3_have = part_none();
        }
      }
      CellKind::Supply => {
        tick_supply(options, state, factory, coord);
      }
      CellKind::Demand => {
        let (x, y) = to_xy(coord, w);
        if x == 0 {
          let ocoord = to_coord_right(coord, w);
          if b2d_outbound_segment_ready_to_send(factory, ocoord, SegmentDirection::LEFT) {
            b2d_move_part(options, state, factory, coord, ocoord, SegmentDirection::LEFT);
          }
        } else if y == 0 {
          let ocoord = to_coord_down(coord, w);
          if b2d_outbound_segment_ready_to_send(factory, ocoord, SegmentDirection::UP) {
            b2d_move_part(options, state, factory, coord, ocoord, SegmentDirection::UP);
          }
        } else if x == w-1 {
          let ocoord = to_coord_left(coord, w);
          if b2d_outbound_segment_ready_to_send(factory, ocoord, SegmentDirection::RIGHT) {
            b2d_move_part(options, state, factory, coord, ocoord, SegmentDirection::RIGHT);
          }
        } else if y == h-1 {
          let ocoord = to_coord_up(coord, w);
          if b2d_outbound_segment_ready_to_send(factory, ocoord, SegmentDirection::DOWN) {
            b2d_move_part(options, state, factory, coord, ocoord, SegmentDirection::DOWN);
          }
        } else {
          panic!("what edge?");
        }
      }
    }
  }

  update_factory_efficiency_stats(options, state, factory);
}

fn update_factory_efficiency_stats(options: &mut Options, _state: &mut State, factory: &mut Factory) {
  if (factory.ticks % options.print_stats_interval) == 0 {
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

pub fn add_price_delta(options: &mut Options, _state: &mut State, stats: &mut (VecDeque<(i32, u64)>, usize, i32, i32, i32), ticks: u64, delta: i32) {
  if options.print_price_deltas { println!("  - add_price_delta: {}, 1sec: {}, 10sec: {}", delta, stats.2 + (delta as i32), stats.3 + (delta as i32)); }
  stats.0.push_back((delta, ticks));
  stats.1 += 1; // new entry should not be older than 10k ticks ;)
  stats.2 += delta as i32;
  stats.3 += delta as i32;
}

pub fn serialize_cli_lines(factory: &Factory) -> Vec<String> {
  // s_ is side cells (demand/supply/empty), rest is inner cells (belt/machine/empty)
  // e_ is edge, c_ is center (belt segments)

  let e_u = | coord: usize | {
    // port up
    let cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => cell.belt.cli_out_box_u,
      CellKind::Belt => cell.belt.cli_out_box_u,
      CellKind::Machine => '*',
      CellKind::Supply => ' ',
      CellKind::Demand => ' ',
    }
  };
  let e_r= | coord: usize | {
    // port right
    let cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => cell.belt.cli_out_box_r,
      CellKind::Belt => cell.belt.cli_out_box_r,
      CellKind::Machine => '*',
      CellKind::Supply => ' ',
      CellKind::Demand => ' ',
    }
  };
  let e_d = | coord: usize | {
    // port down
    let cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => cell.belt.cli_out_box_d,
      CellKind::Belt => cell.belt.cli_out_box_d,
      CellKind::Machine => '*',
      CellKind::Supply => ' ',
      CellKind::Demand => ' ',
    }
  };
  let e_l = | coord: usize | {
    // port left
    let cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => cell.belt.cli_out_box_l,
      CellKind::Belt => cell.belt.cli_out_box_l,
      CellKind::Machine => '*',
      CellKind::Supply => ' ',
      CellKind::Demand => ' ',
    }
  };
  let c_lu = | coord: usize | {
    // segment left-up
    let cell: &Cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => ' ',
      CellKind::Machine => {
        // Show input 1 if there are 2 or more inputs required. Otherwise show nothing.
        if cell.machine_input_2_want.kind != PartKind::None {
          cell.machine_input_1_want.icon
        } else {
          ' '
        }
      },
      CellKind::Supply => ' ',
      CellKind::Demand => ' ',
    }
  };
  let c_u = | coord: usize | {
    // segment up
    let cell: &Cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => {
        // If there's a part here then paint that part instead of the belt segment track
        let part = &cell.segments[SegmentDirection::UP as usize].part;
        if part.kind == PartKind::None {
          cell.belt.cli_out_seg_u
        } else {
          part.icon
        }
      },
      CellKind::Machine => {
        // Show input 1 if there is one or three outputs. Otherwise show nothing.
        if cell.machine_input_2_want.kind == PartKind::None || cell.machine_input_3_want.kind != PartKind::None {
          cell.machine_input_1_want.icon
        } else {
          ' '
        }
      },
      CellKind::Supply => cell.supply_gives.icon,
      CellKind::Demand => ' ',
    }
  };
  let c_ru = | coord: usize | {
    // segment right up
    let cell: &Cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => ' ',
      CellKind::Machine => {
        // Show input 2 if there are 2 inputs required. Show input 3 if there are 3 inputs required.
        if cell.machine_input_3_want.kind != PartKind::None {
          cell.machine_input_3_want.icon
        } else if cell.machine_input_2_want.kind != PartKind::None {
          cell.machine_input_2_want.icon
        } else {
          ' '
        }
      },
      CellKind::Supply => ' ',
      CellKind::Demand => ' ',
    }
  };
  let c_r = | coord: usize | {
    // segment right
    let cell: &Cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => {
        // If there's a part here then paint that part instead of the belt segment track
        let part = &cell.segments[SegmentDirection::RIGHT as usize].part;
        if part.kind == PartKind::None {
          cell.belt.cli_out_seg_r
        } else {
          part.icon
        }
      },
      CellKind::Machine => {
        // Show a corner piece when there's two or three required inputs
        // If there are three inputs, show input 3, else show input 2
        // ┘ ┛
        if cell.machine_input_2_want.kind == PartKind::None {
          ' '
        } else if
          // if input3 wants an input, check if it has one
          (cell.machine_input_3_want.kind != PartKind::None && cell.machine_input_3_have.kind == PartKind::None) ||
          // otherwise, if input3 does not want an input, check if input2 has one
          (cell.machine_input_3_want.kind == PartKind::None && cell.machine_input_2_have.kind == PartKind::None)
        {
          '┘'
        } else {
          '┛'
        }
      }
      CellKind::Supply => ' ',
      CellKind::Demand => ' ',
    }
  };
  let c_dr = | coord: usize | {
    // segment down-right
    let cell: &Cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => ' ',
      CellKind::Machine => ' ',
      CellKind::Supply => ' ',
      CellKind::Demand => ' ',
    }
  };
  let c_d = | coord: usize | {
    // segment down
    let cell: &Cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => {
        // If there's a part here then paint that part instead of the belt segment track
        let part = &cell.segments[SegmentDirection::DOWN as usize].part;
        if part.kind == PartKind::None {
          cell.belt.cli_out_seg_d
        } else {
          part.icon
        }
      },
      CellKind::Machine => cell.machine_output_want.icon,
      CellKind::Supply => cell.supply_part.icon,
      CellKind::Demand => ' ',
    }
  };
  let c_dl = | coord: usize | {
    // segment down-left
    let cell: &Cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => ' ',
      CellKind::Machine => ' ',
      CellKind::Supply => ' ',
      CellKind::Demand => ' ',
    }
  };
  let c_l = | coord: usize | {
    // segment left
    let cell: &Cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => {
        // If there's a part here then paint that part instead of the belt segment track
        let part = &cell.segments[SegmentDirection::LEFT as usize].part;
        if part.kind == PartKind::None {
          cell.belt.cli_out_seg_l
        } else {
          part.icon
        }
      },
      CellKind::Machine => {
        // Show a corner piece when there's two or three required inputs
        // In all cases the actual value to show depends on input 1
        // └ ┗
        if cell.machine_input_2_want.kind == PartKind::None {
          ' '
        } else if cell.machine_input_1_have.kind == PartKind::None {
          '└'
        } else {
          '┗'
        }
      },
      CellKind::Supply => ' ',
      CellKind::Demand => ' ',
    }
  };
  let c_c = | coord: usize | {
    // segment cneter
    let cell: &Cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => {
        // If there's a part here then paint that part instead of the belt segment track
        let part = &cell.segments[SegmentDirection::CENTER as usize].part;
        if part.kind == PartKind::None {
          cell.belt.cli_out_seg_c
        } else {
          part.icon
        }
      },
      CellKind::Machine => {
        // │ ┃   ┼ ┽ ┾ ┿ ╀ ╁ ╂ ╃ ╄ ╅ ╆ ╇ ╈ ╉ ╊ ╋    ┬ ┭ ┮ ┯ ┰ ┱ ┲ ┳
        // This one is just annoying to do because it depends on how many inputs there are
        // and for each input whether it is available

        if cell.machine_input_3_want.kind != PartKind::None {
          // ┼ ┽ ┾ ┿ ╀ ╃ ╄ ╇ ╋
          if cell.machine_input_1_have.kind == PartKind::None {
            if cell.machine_input_2_have.kind == PartKind::None {
              if cell.machine_input_3_have.kind == PartKind::None {
                '┼'
              } else {
                '┾'
              }
            } else {
              if cell.machine_input_3_have.kind == PartKind::None {
                '╀'
              } else {
                '╄'
              }
            }
          } else {
            if cell.machine_input_2_have.kind == PartKind::None {
              if cell.machine_input_3_have.kind == PartKind::None {
                '┽'
              } else {
                '┿'
              }
            } else {
              if cell.machine_input_3_have.kind == PartKind::None {
                '╃'
              } else {
                '╋'
              }
            }
          }
        } else if cell.machine_input_2_want.kind != PartKind::None {
          // ┬ ┭ ┮ ┯ ┰ ┱ ┲ ┳
          if cell.machine_input_1_have.kind == PartKind::None {
            if cell.machine_input_2_have.kind == PartKind::None {
              '┬'
            } else {
              '┮'
            }
          } else {
            if cell.machine_input_2_have.kind == PartKind::None {
              '┭'
            } else {
              '┳'
            }
          }
        } else {
          // │ ┃
          if cell.machine_input_1_have.kind == PartKind::None {
            '│'
          } else {
            '┃'
          }
        }
      }
      CellKind::Supply => '|',
      CellKind::Demand => cell.demand_part.icon,
    }
  };

  let s_h = | coord: usize | {
    let cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => '?',
      CellKind::Machine => '?',
      CellKind::Supply => '─',
      CellKind::Demand => '─',
    }
  };
  let s_v = | coord: usize | {
    let cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => '?',
      CellKind::Machine => '?',
      CellKind::Supply => '│',
      CellKind::Demand => '│',
    }
  };
  let s_u = | coord: usize | {
    let cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => '?',
      CellKind::Machine => '?',
      CellKind::Supply => cell.belt.cli_out_box_u,
      CellKind::Demand => cell.belt.cli_out_box_u,
    }
  };
  let s_r = | coord: usize | {
    let cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => '?',
      CellKind::Machine => '?',
      CellKind::Supply => cell.belt.cli_out_box_r,
      CellKind::Demand => cell.belt.cli_out_box_r,
    }
  };
  let s_d = | coord: usize | {
    let cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => '?',
      CellKind::Machine => '?',
      CellKind::Supply => cell.belt.cli_out_box_d,
      CellKind::Demand => cell.belt.cli_out_box_d,
    }
  };
  let s_l = | coord: usize | {
    let cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => '?',
      CellKind::Machine => '?',
      CellKind::Supply => cell.belt.cli_out_box_l,
      CellKind::Demand => cell.belt.cli_out_box_l,
    }
  };

  let s_lu = | coord: usize | {
    let cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => '?',
      CellKind::Machine => '?',
      CellKind::Supply => cell.belt.cli_out_box_lu,
      CellKind::Demand => cell.belt.cli_out_box_lu,
    }
  };
  let s_ru = | coord: usize | {
    let cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => '?',
      CellKind::Machine => '?',
      CellKind::Supply => cell.belt.cli_out_box_ru,
      CellKind::Demand => cell.belt.cli_out_box_ru,
    }
  };
  let s_dl = | coord: usize | {
    let cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => '?',
      CellKind::Machine => '?',
      CellKind::Supply => cell.belt.cli_out_box_dl,
      CellKind::Demand => cell.belt.cli_out_box_dl,
    }
  };
  let s_dr = | coord: usize | {
    let cell = &factory.floor.cells[coord];
    match cell.kind {
      CellKind::Empty => ' ',
      CellKind::Belt => '?',
      CellKind::Machine => '?',
      CellKind::Supply => cell.belt.cli_out_box_dr,
      CellKind::Demand => cell.belt.cli_out_box_dr,
    }
  };


  // vec!(
  //   "┌─x─┐",                  e_u(0),
  //   "│   │",         c_lu(0), c_u(0), c_ru(0),
  //   "x   x", e_l(0), c_l(0),  c_c(0), c_r(0),  e_r(0),
  //   "│   │",         c_dl(0), c_d(0), c_dr(0),
  //   "└─x─┘",                  e_d(0),
  // );
  // vec!(
  //   "┌─{}─┐"     ,                     e_u( 0),
  //   "│{}{}{}│"   ,         c_lu( 0),   c_u( 0), c_ru( 0),
  //   "{}{}{}{}{}" , e_l( 0), c_l( 0),   c_c( 0),  c_r( 0),  e_r( 0),
  //   "│{}{}{}│"   ,         c_dl( 0),   c_d( 0), c_dr( 0),
  //   "└─{}─┘"     ,                     e_d( 0),
  // );

  // 7x7

  return vec!(
    format!("     {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}     "  ,                                                                 s_lu( 1),  s_h( 1), s_u( 1),  s_h( 1), s_ru( 1),     s_lu( 2), s_h( 2), s_u( 2),  s_h( 2), s_ru( 2),    s_lu( 3), s_h( 3),  s_u( 3), s_h( 3),  s_ru( 3),    s_lu( 4), s_h( 4),  s_u( 4),  s_h( 4), s_ru( 4),   s_lu( 5), s_h( 5), s_u( 5),  s_h( 5), s_ru( 5)),
    format!("     {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}     "  ,                                                                  s_v( 1),  ' ',     c_u( 1),  ' ',      s_v( 1),      s_v( 2), ' ',     c_u( 2),  ' ',      s_v( 2),     s_v( 3), ' ',      c_u( 3), ' ',       s_v( 3),     s_v( 4), ' ',      c_u( 4),  ' ',      s_v( 4),    s_v( 5), ' ',     c_u( 5),  ' ',      s_v( 5)),
    format!("     {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}     "  ,                                                                  s_l( 1),  c_l( 1), c_c( 1),  c_r( 1),  s_r( 1),      s_l( 2), c_l( 2), c_c( 2),  c_r( 2),  s_r( 2),     s_l( 3), c_l( 3),  c_c( 3), c_r( 3),   s_r( 3),     s_l( 4), c_l( 4),  c_c( 4),  c_r( 4),  s_r( 4),    s_l( 5), c_l( 5), c_c( 5),  c_r( 5),  s_r( 5)),
    format!("     {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}     "  ,                                                                  s_v( 1),   ' ',    c_d( 1),  ' ',      s_v( 1),      s_v( 2), ' ',     c_d( 2),  ' ',      s_v( 2),     s_v( 3),  ' ',     c_d( 3), ' ',       s_v( 3),     s_v( 4), ' ',      c_d( 4),  ' ',      s_v( 4),    s_v( 5), ' ',     c_d( 5),  ' ',      s_v( 5)),
    format!("     {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}     "  ,                                                                 s_dl( 1),  s_h( 1), s_d( 1),  s_h( 1), s_dr( 1),     s_dl( 2), s_h( 2), s_d( 2),  s_h( 2), s_dr( 2),    s_dl( 3), s_h( 3),  s_d( 3), s_h( 3),  s_dr( 3),    s_dl( 4), s_h( 4),  s_d( 4),  s_h( 4), s_dr( 4),   s_dl( 5), s_h( 5), s_d( 5),  s_h( 5), s_dr( 5)),
    format!("{}{}{}{}{}┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐{}{}{}{}{}"                        , s_lu( 7), s_h( 7), s_u( 7), s_h( 7), s_ru( 7),                          e_u( 8),                                            e_u( 9),                                            e_u(10),                                            e_u(11),                                          e_u(12),                      s_lu(13),  s_h(13),  s_u(13), s_h(13),   s_ru(13), ),
    format!("{}{}{}{}{}│{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}│{}{}{}{}{}"              , s_v( 7), ' ',      c_u( 7), ' ',      s_v( 7),                c_lu( 8), c_u( 8), c_ru( 8),                        c_lu( 9), c_u( 9), c_ru( 9),                       c_lu(10),  c_u(10), c_ru(10),                       c_lu(11),  c_u(11), c_ru(11),                      c_lu(12), c_u(12), c_ru(12),             s_v(13), ' ',       c_u(13), ' ',        s_v(13),      ),
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}"    , s_l( 7),  c_l( 7), c_c( 7), c_r( 7),  s_r( 7),       e_l( 8),  c_l( 8), c_c( 8),  c_r( 8),  e_r( 8),      e_l( 9), c_l( 9), c_c( 9),  c_r( 9),  e_r( 9),     e_l(10), c_l(10),  c_c(10), c_r(10),   e_r(10),     e_l(11), c_l(11),  c_c(11),  c_r(11),  e_r(11),    e_l(12), c_l(12), c_c(12),  c_r(12),  e_r(12),   s_l(13),  c_l(13),  c_c(13), c_r(13),    s_r(13),          ),
    format!("{}{}{}{}{}│{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}│{}{}{}{}{}"              , s_v( 7),  ' ',     c_d( 7), ' ',      s_v( 7),                c_dl( 8), c_d( 8), c_dr( 8),                        c_dl( 9), c_d( 9), c_dr( 9),                       c_dl(10),  c_d(10), c_dr(10),                       c_dl(11),  c_d(11), c_dr(11),                      c_dl(12), c_d(12), c_dr(12),             s_v(13),  ' ',      c_d(13), ' ',        s_v(13),      ),
    format!("{}{}{}{}{}└─{}─┘└─{}─┘└─{}─┘└─{}─┘└─{}─┘{}{}{}{}{}"                        , s_dl( 7), s_h( 7), s_d( 7), s_h( 7), s_dr( 7),                          e_d( 8),                                            e_d( 9),                                            e_d(10),                                            e_d(11),                                          e_d(12),                      s_dl(13),  s_h(13),  s_d(13), s_h(13),   s_dr(13), ),
    format!("{}{}{}{}{}┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐{}{}{}{}{}"                        , s_lu(14), s_h(14), s_u(14), s_h(14), s_ru(14),                          e_u(15),                                            e_u(16),                                            e_u(17),                                            e_u(18),                                          e_u(19),                      s_lu(20),  s_h(20),  s_u(20), s_h(20),   s_ru(20), ),
    format!("{}{}{}{}{}│{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}│{}{}{}{}{}"              , s_v(14), ' ',      c_u(14), ' ',      s_v(14),                c_lu(15), c_u(15), c_ru(15),                        c_lu(16), c_u(16), c_ru(16),                       c_lu(17),  c_u(17), c_ru(17),                       c_lu(18),  c_u(18), c_ru(18),                      c_lu(19), c_u(19), c_ru(19),             s_v(20), ' ',       c_u(20), ' ',        s_v(20),      ),
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}"    , s_l(14),  c_l(14), c_c(14), c_r(14),  s_r(14),       e_l(15),  c_l(15), c_c(15),  c_r(15),  e_r(15),      e_l(16), c_l(16), c_c(16),  c_r(16),  e_r(16),     e_l(17), c_l(17),  c_c(17), c_r(17),   e_r(17),     e_l(18), c_l(18),  c_c(18),  c_r(18),  e_r(18),    e_l(19), c_l(19), c_c(19),  c_r(19),  e_r(19),   s_l(20),  c_l(20),  c_c(20), c_r(20),    s_r(20),          ),
    format!("{}{}{}{}{}│{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}│{}{}{}{}{}"              , s_v(14),  ' ',     c_d(14), ' ',      s_v(14),                c_dl(15), c_d(15), c_dr(15),                        c_dl(16), c_d(16), c_dr(16),                       c_dl(17),  c_d(17), c_dr(17),                       c_dl(18),  c_d(18), c_dr(18),                      c_dl(19), c_d(19), c_dr(19),             s_v(20),  ' ',      c_d(20), ' ',        s_v(20),      ),
    format!("{}{}{}{}{}└─{}─┘└─{}─┘└─{}─┘└─{}─┘└─{}─┘{}{}{}{}{}"                        , s_dl(14), s_h(14), s_d(14), s_h(14), s_dr(14),                          e_d(15),                                            e_d(16),                                            e_d(17),                                            e_d(18),                                          e_d(19),                      s_dl(20),  s_h(20),  s_d(20), s_h(20),   s_dr(20), ),
    format!("{}{}{}{}{}┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐{}{}{}{}{}"                        , s_lu(21), s_h(21), s_u(21), s_h(21), s_ru(21),                          e_u(22),                                            e_u(23),                                            e_u(24),                                            e_u(25),                                          e_u(26),                      s_lu(27),  s_h(27),  s_u(27), s_h(27),   s_ru(27), ),
    format!("{}{}{}{}{}│{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}│{}{}{}{}{}"              , s_v(21), ' ',      c_u(21), ' ',      s_v(21),                c_lu(22), c_u(22), c_ru(22),                        c_lu(23), c_u(23), c_ru(23),                       c_lu(24),  c_u(24), c_ru(24),                       c_lu(25),  c_u(25), c_ru(25),                      c_lu(26), c_u(26), c_ru(26),             s_v(27), ' ',       c_u(27), ' ',        s_v(27),      ),
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}"    , s_l(21),  c_l(21), c_c(21), c_r(21),  s_r(21),       e_l(22),  c_l(22), c_c(22),  c_r(22),  e_r(22),      e_l(23), c_l(23), c_c(23),  c_r(23),   e_r(23),    e_l(24), c_l(24),  c_c(24), c_r(24),    e_r(24),    e_l(25), c_l(25),  c_c(25),  c_r(25),  e_r(25),    e_l(26), c_l(26), c_c(26),  c_r(26),  e_r(26),   s_l(27),  c_l(27),  c_c(27), c_r(27),    s_r(27),          ),
    format!("{}{}{}{}{}│{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}│{}{}{}{}{}"              , s_v(21),  ' ',     c_d(21), ' ',      s_v(21),                c_dl(22), c_d(22), c_dr(22),                        c_dl(23), c_d(23), c_dr(23),                       c_dl(24),  c_d(24), c_dr(24),                       c_dl(25),  c_d(25), c_dr(25),                      c_dl(26), c_d(26), c_dr(26),             s_v(27),  ' ',      c_d(27), ' ',        s_v(27),      ),
    format!("{}{}{}{}{}└─{}─┘└─{}─┘└─{}─┘└─{}─┘└─{}─┘{}{}{}{}{}"                        , s_dl(21), s_h(21), s_d(21), s_h(21), s_dr(21),                          e_d(22),                                            e_d(23),                                            e_d(24),                                            e_d(25),                                          e_d(26),                      s_dl(27),  s_h(27),  s_d(27), s_h(27),   s_dr(27), ),
    format!("{}{}{}{}{}┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐{}{}{}{}{}"                        , s_lu(28), s_h(28), s_u(28), s_h(28), s_ru(28),                          e_u(29),                                            e_u(30),                                            e_u(31),                                            e_u(32),                                          e_u(33),                      s_lu(34),  s_h(34),  s_u(34), s_h(34),   s_ru(34), ),
    format!("{}{}{}{}{}│{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}│{}{}{}{}{}"              , s_v(28), ' ',      c_u(28), ' ',      s_v(28),                c_lu(29), c_u(29), c_ru(29),                        c_lu(30), c_u(30), c_ru(30),                       c_lu(31),  c_u(31), c_ru(31),                       c_lu(32),  c_u(32), c_ru(32),                      c_lu(33), c_u(33), c_ru(33),             s_v(34), ' ',       c_u(34), ' ',        s_v(34),      ),
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}"    , s_l(28),  c_l(28), c_c(28), c_r(28),  s_r(28),       e_l(29),  c_l(29), c_c(29),  c_r(29),  e_r(29),      e_l(30), c_l(30), c_c(30),  c_r(30),   e_r(30),    e_l(31), c_l(31),  c_c(31), c_r(31),    e_r(31),    e_l(32), c_l(32),  c_c(32),  c_r(32),  e_r(32),    e_l(33), c_l(33), c_c(33),  c_r(33),  e_r(33),   s_l(34),  c_l(34),  c_c(34), c_r(34),    s_r(34),          ),
    format!("{}{}{}{}{}│{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}│{}{}{}{}{}"              , s_v(28),  ' ',     c_d(28), ' ',      s_v(28),                c_dl(29), c_d(29), c_dr(29),                        c_dl(30), c_d(30), c_dr(30),                       c_dl(31),  c_d(31), c_dr(31),                       c_dl(32),  c_d(32), c_dr(32),                      c_dl(33), c_d(33), c_dr(33),             s_v(34),  ' ',      c_d(34), ' ',        s_v(34),      ),
    format!("{}{}{}{}{}└─{}─┘└─{}─┘└─{}─┘└─{}─┘└─{}─┘{}{}{}{}{}"                        , s_dl(28), s_h(28), s_d(28), s_h(28), s_dr(28),                          e_d(29),                                            e_d(30),                                            e_d(31),                                            e_d(32),                                          e_d(33),                      s_dl(34),  s_h(34),  s_d(34), s_h(34),   s_dr(34), ),
    format!("{}{}{}{}{}┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐{}{}{}{}{}"                        , s_lu(35), s_h(35), s_u(35), s_h(35), s_ru(35),                          e_u(36),                                            e_u(37),                                            e_u(38),                                            e_u(39),                                          e_u(40),                      s_lu(41),  s_h(41),  s_u(41), s_h(41),   s_ru(41), ),
    format!("{}{}{}{}{}│{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}│{}{}{}{}{}"              , s_v(35), ' ',      c_u(35), ' ',      s_v(35),                c_lu(36), c_u(36), c_ru(36),                        c_lu(37), c_u(37), c_ru(37),                       c_lu(38),  c_u(38), c_ru(38),                       c_lu(39),  c_u(39), c_ru(39),                      c_lu(40), c_u(40), c_ru(40),             s_v(41), ' ',       c_u(41), ' ',        s_v(41),      ),
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}"    , s_l(35),  c_l(35), c_c(35), c_r(35),  s_r(35),       e_l(36),  c_l(36), c_c(36),  c_r(36),  e_r(36),      e_l(37), c_l(37), c_c(37),  c_r(37),   e_r(37),    e_l(38), c_l(38),  c_c(38), c_r(38),    e_r(38),    e_l(39), c_l(39),  c_c(39),  c_r(39),  e_r(39),    e_l(40), c_l(40), c_c(40),  c_r(40),  e_r(40),   s_l(41),  c_l(41),  c_c(41), c_r(41),    s_r(41),          ),
    format!("{}{}{}{}{}│{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}│{}{}{}{}{}"              , s_v(35),  ' ',     c_d(35), ' ',      s_v(35),                c_dl(36), c_d(36), c_dr(36),                        c_dl(37), c_d(37), c_dr(37),                       c_dl(38),  c_d(38), c_dr(38),                       c_dl(39),  c_d(39), c_dr(39),                      c_dl(40), c_d(40), c_dr(40),             s_v(41),  ' ',      c_d(41), ' ',        s_v(41),  ),
    format!("{}{}{}{}{}└─{}─┘└─{}─┘└─{}─┘└─{}─┘└─{}─┘{}{}{}{}{}"                        , s_dl(35), s_h(35), s_d(35), s_h(35),  s_dr(35),                         e_d(36),                                            e_d(37),                                            e_d(38),                                            e_d(39),                                          e_d(40),                      s_dl(41),  s_h(41),  s_d(41), s_h(41),   s_dr(41), ),
    format!("     {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}     "  ,                                                                 s_lu(43),  s_h(43), s_u(43),  s_h(43), s_ru(43),     s_lu(44), s_h(44), s_u(44),  s_h(44),  s_ru(44),   s_lu(45), s_h(45),  s_u(45),  s_h(45),  s_ru(45),   s_lu(46), s_h(46),  s_u(46),  s_h(46), s_ru(46),   s_lu(47), s_h(47), s_u(47), s_h(47),  s_ru(47)),
    format!("     {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}     "  ,                                                                  s_v(43), ' ',      c_u(43),  ' ',      s_v(43),      s_v(44), ' ',     c_u(44),  ' ',       s_v(44),    s_v(45), ' ',      c_u(45),  ' ',       s_v(45),    s_v(46), ' ',      c_u(46),  ' ',      s_v(46),    s_v(47), ' ',     c_u(47), ' ',       s_v(47)),
    format!("     {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}     "  ,                                                                  s_l(43),  c_l(43), c_c(43),  c_r(43),  s_r(43),      s_l(44), c_l(44), c_c(44),  c_r(44),   s_r(44),    s_l(45), c_l(45),  c_c(45),  c_r(45),   s_r(45),    s_l(46), c_l(46),  c_c(46),  c_r(46),  s_r(46),    s_l(47), c_l(47), c_c(47), c_r(47),   s_r(47)),
    format!("     {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}     "  ,                                                                  s_v(43),  ' ',     c_d(43),  ' ',      s_v(43),      s_v(44), ' ',     c_d(44),  ' ',       s_v(44),    s_v(45), ' ',      c_d(45),  ' ',       s_v(45),    s_v(46), ' ',      c_d(46),  ' ',      s_v(46),    s_v(47), ' ',     c_d(47), ' ',       s_v(47)),
    format!("     {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}     "  ,                                                                 s_dl(43),  s_h(43), s_d(43),  s_h(43), s_dr(43),     s_dl(44), s_h(44), s_d(44),  s_h(44),  s_dr(44),   s_dl(45), s_h(45),  s_d(45),  s_h(45),  s_dr(45),   s_dl(46), s_h(46),  s_d(46),  s_h(46), s_dr(46),   s_dl(47), s_h(47), s_d(47), s_h(47),  s_dr(47)),
  );
}

pub fn serialize_cli(factory: &Factory) -> String {
  let lines = serialize_cli_lines(factory);
  return lines.into_iter().collect::<Vec<String>>().join("\n");
}
