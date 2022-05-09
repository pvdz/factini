use super::belt::*;
use super::options::*;
use super::machine::*;
use super::part::*;
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

  // Each belt has five potential segments in a 3x3 grid; one to each cardinal side and the center
  // (In the future there may also be the double corner case, but same difference.)
  // Each segment can hold one part and has to track it and when it started moving it
  pub segment_u_part: Part,
  pub segment_u_at: u64,
  pub segment_r_part: Part,
  pub segment_r_at: u64,
  pub segment_d_part: Part,
  pub segment_d_at: u64,
  pub segment_l_part: Part,
  pub segment_l_at: u64,

  // And the center piece, which is special because it may have to merge or divide
  pub segment_c_part: Part,
  pub segment_c_at: u64,
  // The center segment has to track where its part came from and where it'll be going to
  pub segment_c_from: CellPort, // This is basically tracked to paint it properly
  pub segment_c_to: CellPort,
  // A part on the center segment is stuck at 50% while all valid target segments are full
  pub segment_c_blocked: bool,

  // Required input for this machine. Can be none. Can require up to three things.
  // There should be no gap, meaning if there are two inputs, then 3 should always be the none part
  pub machine_input_1_want: Part,
  pub machine_input_1_have: Part,
  pub machine_input_2_want: Part,
  pub machine_input_2_have: Part,
  pub machine_input_3_want: Part,
  pub machine_input_3_have: Part,
  // Output is mandatory and should not be None (or maybe? free trash anyone?)
  pub machine_start_at: u64,
  pub machine_output_want: Part,
  pub machine_output_have: Part, // Will sit at factory for as long as there is no out with space
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellKind {
  Empty,
  Belt,
  Machine,
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

pub fn empty_cell(x: usize, y: usize) -> Cell {
  Cell {
    kind: CellKind::Empty,
    belt: BELT_NONE,
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
    segment_u_part: part_none(x, y),
    segment_u_at: 0,
    segment_r_part: part_none(x, y),
    segment_r_at: 0,
    segment_d_part: part_none(x, y),
    segment_d_at: 0,
    segment_l_part: part_none(x, y),
    segment_l_at: 0,
    segment_c_part: part_none(x, y),
    segment_c_at: 0,
    segment_c_from: CellPort::Up,
    segment_c_to: CellPort::Up,
    segment_c_blocked: true,
    machine_input_1_want: part_none(x, y),
    machine_input_1_have: part_none(x, y),
    machine_input_2_want: part_none(x, y),
    machine_input_2_have: part_none(x, y),
    machine_input_3_want: part_none(x, y),
    machine_input_3_have: part_none(x, y),
    machine_start_at: 0,
    machine_output_want: part_none(x, y),
    machine_output_have: part_none(x, y),
  }
}

pub fn belt_cell(x: usize, y: usize, belt: BeltMeta) -> Cell {
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
    segment_u_part: part_none(x, y),
    segment_u_at: 0,
    segment_r_part: part_none(x, y),
    segment_r_at: 0,
    segment_d_part: part_none(x, y),
    segment_d_at: 0,
    segment_l_part: part_none(x, y),
    segment_l_at: 0,
    segment_c_part: part_none(x, y),
    segment_c_at: 0,
    segment_c_from: CellPort::Up,
    segment_c_to: CellPort::Up,
    segment_c_blocked: true,
    machine_input_1_want: part_none(x, y),
    machine_input_1_have: part_none(x, y),
    machine_input_2_want: part_none(x, y),
    machine_input_2_have: part_none(x, y),
    machine_input_3_want: part_none(x, y),
    machine_input_3_have: part_none(x, y),
    machine_start_at: 0,
    machine_output_want: part_none(x, y),
    machine_output_have: part_none(x, y),
  }
}

pub fn machine_cell(x: usize, y: usize, machine: Machine, input1: Part, input2: Part, input3: Part, output: Part, machine_production_price: i32, machine_trash_price: i32) -> Cell {
  Cell {
    kind: CellKind::Machine,
    belt: BELT_NONE,
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
    segment_u_part: part_none(x, y),
    segment_u_at: 0,
    segment_r_part: part_none(x, y),
    segment_r_at: 0,
    segment_d_part: part_none(x, y),
    segment_d_at: 0,
    segment_l_part: part_none(x, y),
    segment_l_at: 0,
    segment_c_part: part_none(x, y),
    segment_c_at: 0,
    segment_c_from: CellPort::Up,
    segment_c_to: CellPort::Up,
    segment_c_blocked: true,
    machine_input_1_want: input1,
    machine_input_1_have: part_none(x, y),
    machine_input_2_want: input2,
    machine_input_2_have: part_none(x, y),
    machine_input_3_want: input3,
    machine_input_3_have: part_none(x, y),
    machine_start_at: 0,
    machine_output_want: output,
    machine_output_have: part_none(x, y),
  }
}
