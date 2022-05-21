use super::belt::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::segment::*;
use super::state::*;

#[derive(Debug)]
pub struct Cell {
  pub kind: CellKind,
  pub belt: BeltMeta, // If the kind is not CellKind::Belt then this is Belt::none
  pub machine: Machine, // If the kind is not CellKind::Machine then this is Machine::none
  pub ticks: u64, // Number of ticks this tile has existed. Used for progress.
  pub x: usize,
  pub y: usize,
  pub belt_tick_price: i32, // Basically the continuous price you pay for having this belt on board, this applies to every tick so keep it low
  pub machine_production_price: i32, // Price you pay when a machine generates an output
  pub machine_trash_price: i32, // Price you pay when a machine has to discard an invalid part

  pub marked: bool,

  // Belt: speed per part on one segment (so 3 to get from in to out)
  // Machine: speed at which the machine produces output once all inputs are received
  pub speed: u64,

  // These offsets make sure the center gives to and takes from belts in alternating order
  pub offset_segment: u8,
  pub offset_center: u8,

  // A belt can have up to 5 segments; they form a cross in a 3x3 grid inside a cell (u, r, d, l, c)
  // Machine and empty cells have segments with none-parts.
  pub segments: [Segment; 5],

  // Required input for this machine. Can be none. Can require up to three things.
  // There should be no gap, meaning if there are two inputs, then 3 should always be the none part
  // An input that is empty but reserved can not unblock another segment
  pub machine_input_1_want: Part,
  pub machine_input_1_have: Part,
  pub machine_input_1_claimed: bool,
  pub machine_input_2_want: Part,
  pub machine_input_2_have: Part,
  pub machine_input_2_claimed: bool,
  pub machine_input_3_want: Part,
  pub machine_input_3_have: Part,
  pub machine_input_3_claimed: bool,
  // Output is mandatory and should not be None (or maybe? free trash anyone?)
  pub machine_start_at: u64,
  pub machine_output_want: Part,
  pub machine_output_have: Part, // Will sit at factory for as long as there is no out with space

  pub demand_part: Part, // The part that this demander is waiting for
  pub demand_part_price: i32, // Amount of money you receive when supplying the proper part
  pub demand_trash_price: i32, // Penalty you pay for giving the wrong part

  pub supply_part: Part, // Current part that's moving out
  pub supply_part_at: u64, // Last time part was generated
  pub supply_last_part_out_at: u64, // Last time a part left this supply
  pub supply_interval: u64, // Generate new part every this many ticks
  pub supply_gives: Part, // The example Part that this supply will generate
  pub supply_part_price: i32, // Cost when dispensing one part
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellKind {
  Empty,
  Belt,
  Machine,
  Supply,
  Demand,
}

// Ports connect cells. Each cell has their own side of a connection. A port is incoming, outgoing
// or none. A port orientation is up, right, down, or left. Probably will rename this to PortDir.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellPort {
  Up,
  Right,
  Down,
  Left,
}

pub const fn empty_cell(x: usize, y: usize) -> Cell {
  Cell {
    kind: CellKind::Empty,
    belt: CELL_BELT_NONE,
    machine: Machine::None,
    ticks: 0,
    x,
    y,
    belt_tick_price: 0,
    machine_production_price: 0,
    machine_trash_price: 0,
    marked: false,
    speed: 0,
    offset_segment: 0,
    offset_center: 0,
    segments: [
      segment_none(SegmentDirection::UP),
      segment_none(SegmentDirection::RIGHT),
      segment_none(SegmentDirection::DOWN),
      segment_none(SegmentDirection::LEFT),
      segment_none(SegmentDirection::CENTER),
    ],
    machine_input_1_want: part_none(),
    machine_input_1_have: part_none(),
    machine_input_1_claimed: false,
    machine_input_2_want: part_none(),
    machine_input_2_have: part_none(),
    machine_input_2_claimed: false,
    machine_input_3_want: part_none(),
    machine_input_3_have: part_none(),
    machine_input_3_claimed: false,
    machine_start_at: 0,
    machine_output_want: part_none(),
    machine_output_have: part_none(),
    demand_part: part_none(),
    demand_part_price: 0,
    demand_trash_price: 0,
    supply_part: part_none(),
    supply_part_at: 0,
    supply_last_part_out_at: 0,
    supply_interval: 0,
    supply_gives: part_none(),
    supply_part_price: 0,
  }
}

pub fn belt_cell(x: usize, y: usize, belt: BeltMeta) -> Cell {
  let segments = [
    segment_create(SegmentDirection::UP, belt.direction_u),
    segment_create(SegmentDirection::RIGHT, belt.direction_r),
    segment_create(SegmentDirection::DOWN, belt.direction_d),
    segment_create(SegmentDirection::LEFT, belt.direction_l),
    segment_some(SegmentDirection::CENTER, Port::None),
  ];

  Cell {
    kind: CellKind::Belt,
    belt,
    machine: Machine::None,
    ticks: 0,
    x,
    y,
    belt_tick_price: 1,
    machine_production_price: 0,
    machine_trash_price: 0,
    marked: false,
    speed: 10000,
    offset_segment: 0,
    offset_center: 0,
    segments,
    machine_input_1_want: part_none(),
    machine_input_1_have: part_none(),
    machine_input_1_claimed: false,
    machine_input_2_want: part_none(),
    machine_input_2_have: part_none(),
    machine_input_2_claimed: false,
    machine_input_3_want: part_none(),
    machine_input_3_have: part_none(),
    machine_input_3_claimed: false,
    machine_start_at: 0,
    machine_output_want: part_none(),
    machine_output_have: part_none(),
    demand_part: part_none(),
    demand_part_price: 0,
    demand_trash_price: 0,
    supply_part: part_none(),
    supply_part_at: 0,
    supply_last_part_out_at: 0,
    supply_interval: 0,
    supply_gives: part_none(),
    supply_part_price: 0,
  }
}

pub fn machine_cell(x: usize, y: usize, machine: Machine, input1: Part, input2: Part, input3: Part, output: Part, machine_production_price: i32, machine_trash_price: i32) -> Cell {
  Cell {
    kind: CellKind::Machine,
    belt: CELL_MACHINE,
    machine,
    ticks: 0,
    x,
    y,
    belt_tick_price: 0,
    machine_production_price,
    machine_trash_price,
    marked: false,
    speed: 10000,
    offset_segment: 0,
    offset_center: 0,
    // Not sure if and how these are used for machine cells. TBD
    segments: [
      segment_none(SegmentDirection::UP),
      segment_none(SegmentDirection::RIGHT),
      segment_none(SegmentDirection::DOWN),
      segment_none(SegmentDirection::LEFT),
      segment_none(SegmentDirection::CENTER),
    ],
    machine_input_1_want: input1,
    machine_input_1_have: part_none(),
    machine_input_1_claimed: false,
    machine_input_2_want: input2,
    machine_input_2_have: part_none(),
    machine_input_2_claimed: false,
    machine_input_3_want: input3,
    machine_input_3_have: part_none(),
    machine_input_3_claimed: false,
    machine_start_at: 0,
    machine_output_want: output,
    machine_output_have: part_none(),
    demand_part: part_none(),
    demand_part_price: 0,
    demand_trash_price: 0,
    supply_part: part_none(),
    supply_part_at: 0,
    supply_last_part_out_at: 0,
    supply_interval: 0,
    supply_gives: part_none(),
    supply_part_price: 0,
  }
}

pub fn supply_cell(x: usize, y: usize, belt: BeltMeta, part: Part, speed: u64, interval: u64, price: i32) -> Cell {
  let segments = [
    segment_none(SegmentDirection::UP),
    segment_none(SegmentDirection::RIGHT),
    segment_none(SegmentDirection::DOWN),
    segment_none(SegmentDirection::LEFT),
    segment_none(SegmentDirection::CENTER),
  ];

  Cell {
    kind: CellKind::Supply,
    belt,
    machine: Machine::None,
    ticks: 0,
    x,
    y,
    belt_tick_price: 0,
    machine_production_price: 0,
    machine_trash_price: 0,
    marked: false,
    speed,
    offset_segment: 0,
    offset_center: 0,
    segments,
    machine_input_1_want: part_none(),
    machine_input_1_have: part_none(),
    machine_input_1_claimed: false,
    machine_input_2_want: part_none(),
    machine_input_2_have: part_none(),
    machine_input_2_claimed: false,
    machine_input_3_want: part_none(),
    machine_input_3_have: part_none(),
    machine_input_3_claimed: false,
    machine_start_at: 0,
    machine_output_want: part_none(),
    machine_output_have: part_none(),
    demand_part: part_none(),
    demand_part_price: 0,
    demand_trash_price: 0,
    supply_part: part_none(),
    supply_part_at: 0,
    supply_last_part_out_at: 0,
    supply_interval: interval,
    supply_gives: part,
    supply_part_price: price,
  }
}

pub fn demand_cell(x: usize, y: usize, belt: BeltMeta, part: Part, speed: u64, part_price: i32, trash_price: i32) -> Cell {
  let segments = [
    segment_none(SegmentDirection::UP),
    segment_none(SegmentDirection::RIGHT),
    segment_none(SegmentDirection::DOWN),
    segment_none(SegmentDirection::LEFT),
    segment_none(SegmentDirection::CENTER),
  ];

  Cell {
    kind: CellKind::Demand,
    belt,
    machine: Machine::None,
    ticks: 0,
    x,
    y,
    belt_tick_price: 0,
    machine_production_price: 0,
    machine_trash_price: 0,
    marked: false,
    speed,
    offset_segment: 0,
    offset_center: 0,
    segments,
    machine_input_1_want: part_none(),
    machine_input_1_have: part_none(),
    machine_input_1_claimed: false,
    machine_input_2_want: part_none(),
    machine_input_2_have: part_none(),
    machine_input_2_claimed: false,
    machine_input_3_want: part_none(),
    machine_input_3_have: part_none(),
    machine_input_3_claimed: false,
    machine_start_at: 0,
    machine_output_want: part_none(),
    machine_output_have: part_none(),
    demand_part: part,
    demand_part_price: part_price,
    demand_trash_price: trash_price,
    supply_part: part_none(),
    supply_part_at: 0,
    supply_last_part_out_at: 0,
    supply_interval: 0,
    supply_gives: part_none(),
    supply_part_price: 0,
  }
}
