use super::belt::*;
use super::cell::*;
use super::floor::*;
use super::factory::*;
use super::options::*;
use super::part::*;
use super::state::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MachineKind {
  None,
  Unknown, // Not yet defined
  Main,
  Smasher,
  SubBuilding, // Extra part but not the main building
}

#[derive(Debug)]
pub struct Machine {
  pub kind: MachineKind,
  pub main_coord: usize, // If this is a sub building, what is the coord of the main machine cell?
  pub coords: Vec<usize>, // First element is main coord. List of coords part of this machine
  pub id: usize,

  // Required input for this machine. Can be none. Can require up to three things.
  // There should be no gap, meaning if there are two inputs, then 3 should always be the none part
  // An input that is empty but reserved can not unblock another segment
  pub input_1_want: Part,
  pub input_1_have: Part,
  pub input_2_want: Part,
  pub input_2_have: Part,
  pub input_3_want: Part,
  pub input_3_have: Part,

  // Output is mandatory and should not be None (or maybe? free trash anyone?)
  pub start_at: u64,
  pub output_want: Part,
  pub output_have: Part, // Will sit at factory for as long as there is no out with space

  // Speed at which the machine produces output once all inputs are received
  pub speed: u64,

  pub production_price: i32, // Price you pay when a machine generates an output
  pub trash_price: i32, // Price you pay when a machine has to discard an invalid part
}

pub const fn machine_none(main_coord: usize) -> Machine {
  return Machine {
    kind: MachineKind::None,
    main_coord,
    coords: vec!(),
    id: 999,

    input_1_want: part_none(),
    input_1_have: part_none(),
    input_2_want: part_none(),
    input_2_have: part_none(),
    input_3_want: part_none(),
    input_3_have: part_none(),

    start_at: 0,

    output_want: part_none(),
    output_have: part_none(),

    speed: 0,
    production_price: 0,
    trash_price: 0,
  };
}

pub fn machine_new(kind: MachineKind, id: usize, main_coord: usize, input1: Part, input2: Part, input3: Part, output: Part) -> Machine {
  return Machine {
    kind,
    main_coord,
    coords: vec!(),
    id,

    input_1_want: input1,
    input_1_have: part_none(),
    input_2_want: input2,
    input_2_have: part_none(),
    input_3_want: input3,
    input_3_have: part_none(),

    start_at: 0,

    output_want: output,
    output_have: part_none(),

    speed: 0,
    production_price: 0,
    trash_price: 0,
  };
}

pub fn tick_machine(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) {
}
