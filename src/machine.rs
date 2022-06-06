use crate::direction::Direction;
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

  pub output_want: Part,
  pub start_at: u64,
  // pub output_have: Part, // Will sit at factory for as long as there is no out with space

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
    // output_have: part_none(),

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
    // output_have: part_none(),

    speed: 0,
    production_price: 0,
    trash_price: 0,
  };
}

pub fn tick_machine(options: &mut Options, state: &mut State, factory: &mut Factory, main_coord: usize) {
  // Finish building by giving the wanted part to a neighbor outbound belt
  // Accept from the input ports
  // Start building

  if factory.floor[main_coord].machine.start_at > 0 && factory.ticks - factory.floor[main_coord].machine.start_at >= factory.floor[main_coord].machine.speed {
    // Finished! Find an available outlet.
    let mut handed_off = false;
    let outlen = factory.floor[main_coord].outs.len();
    for index in 0..outlen {
      let rotated_index = ((index as u64 + factory.floor[main_coord].outrot) % (outlen as u64)) as usize;
      let (sub_dir, sub_coord, _, _ ) = factory.floor[main_coord].outs[rotated_index];
      let (to_coord, to_dir) = match sub_dir {
        Direction::Up => ( factory.floor[sub_coord].coord_u, Direction::Down ),
        Direction::Right => ( factory.floor[sub_coord].coord_r, Direction::Left ),
        Direction::Down => ( factory.floor[sub_coord].coord_d, Direction::Up ),
        Direction::Left => ( factory.floor[sub_coord].coord_l, Direction::Right ),
      };
      if let Some(to_coord) = to_coord {
        if factory.floor[to_coord].kind == CellKind::Belt && factory.floor[to_coord].belt.part.kind == PartKind::None {
          // The neighbor is a belt that is empty
          if options.print_moves || options.print_moves_machine { super::log(format!("({}) Machine @{} (sub @{}) finished part {:?}! Moving to belt @{}", factory.ticks, main_coord, sub_coord, factory.floor[main_coord].machine.output_want.kind, to_coord).as_str()); }

          belt_receive_part(factory, to_coord, to_dir, factory.floor[main_coord].machine.output_want.clone());
          factory.floor[main_coord].machine.start_at = 0;
          factory.floor[main_coord].outrot = ((rotated_index + 1) % outlen) as u64;
          handed_off = true;
          break;
        }
      }
    }
    if !handed_off {
      println!("machine has a part but is unable to unload it...");
    }
  }

  // Find the input connected to a belt with matching part as any of the inputs that await one
  for index in 0..factory.floor[main_coord].ins.len() {
    let (sub_dir, sub_coord, _main_neighbor_coord, _main_neighbor_in_dir) = factory.floor[main_coord].ins[index];
    let (from_coord, incoming_dir ) = match sub_dir {
      Direction::Up => ( factory.floor[sub_coord].coord_u, Direction::Down ),
      Direction::Right => ( factory.floor[sub_coord].coord_r, Direction::Left ),
      Direction::Down => ( factory.floor[sub_coord].coord_d, Direction::Up ),
      Direction::Left => ( factory.floor[sub_coord].coord_l, Direction::Right ),
    };

    // Can't we assume ocoord exists here (use unwrap instead)?
    if let Some(from_coord) = from_coord {
      // TODO: can we assume the other side is a belt?
      if factory.floor[from_coord].kind == CellKind::Belt {
        let belt_part = factory.floor[from_coord].belt.part.kind;
        if belt_part != PartKind::None && factory.ticks - factory.floor[from_coord].belt.part_at >= factory.floor[from_coord].belt.speed {
          if belt_part == factory.floor[main_coord].machine.input_1_want.kind && belt_part != factory.floor[main_coord].machine.input_1_have.kind && factory.floor[main_coord].machine.input_1_have.kind == PartKind::None {
            if options.print_moves || options.print_moves_machine { super::log(format!("({}) Machine @{} (sub @{}) accepting part {:?} as input1 from belt @{}, had {:?}", factory.ticks, main_coord, sub_coord, belt_part, from_coord, factory.floor[main_coord].machine.input_1_have).as_str()); }
            factory.floor[main_coord].machine.input_1_have = factory.floor[from_coord].belt.part.clone();
            belt_receive_part(factory, from_coord, incoming_dir, part_none());
          } else if belt_part == factory.floor[main_coord].machine.input_2_want.kind && belt_part != factory.floor[main_coord].machine.input_2_have.kind && factory.floor[main_coord].machine.input_2_have.kind == PartKind::None {
            if options.print_moves || options.print_moves_machine { super::log(format!("({}) Machine @{} (sub @{}) accepting part {:?} as input2 from belt @{}, had {:?}", factory.ticks, main_coord, sub_coord, belt_part, from_coord, factory.floor[main_coord].machine.input_2_have).as_str()); }
            factory.floor[main_coord].machine.input_2_have = factory.floor[from_coord].belt.part.clone();
            belt_receive_part(factory, from_coord, incoming_dir, part_none());
          } else if belt_part == factory.floor[main_coord].machine.input_3_want.kind && belt_part != factory.floor[main_coord].machine.input_3_have.kind && factory.floor[main_coord].machine.input_3_have.kind == PartKind::None {
            if options.print_moves || options.print_moves_machine { super::log(format!("({}) Machine @{} (sub @{}) accepting part {:?} as input3 from belt @{}, had {:?}", factory.ticks, main_coord, sub_coord, belt_part, from_coord, factory.floor[main_coord].machine.input_3_have).as_str()); }
            factory.floor[main_coord].machine.input_3_have = factory.floor[from_coord].belt.part.clone();
            belt_receive_part(factory, from_coord, incoming_dir, part_none());
          } else {
            // Trash it? TODO: machine with multiple inputs should perhaps only trash if none of the inputs were acceptable?
            if options.print_moves || options.print_moves_machine { super::log(format!("({}) Machine @{} (sub @{}) trashing part {:?} from belt @{}; wants: {:?} {:?} {:?}, has: {:?} {:?} {:?}", factory.ticks, main_coord, sub_coord, belt_part, from_coord,
              factory.floor[main_coord].machine.input_1_want.kind,
              factory.floor[main_coord].machine.input_2_want.kind,
              factory.floor[main_coord].machine.input_3_want.kind,
              factory.floor[main_coord].machine.input_1_have.kind,
              factory.floor[main_coord].machine.input_2_have.kind,
              factory.floor[main_coord].machine.input_3_have.kind,
            ).as_str()); }
            belt_receive_part(factory, from_coord, incoming_dir, part_none());
          }
        }
      }
    }
  }
  //
  // println!("({}) machine @{} ready? {:?} == {:?}, {:?} == {:?}, {:?} == {:?}",
  //   factory.ticks,
  //   main_coord,
  //     factory.floor[main_coord].machine.input_1_want.kind, factory.floor[main_coord].machine.input_1_have.kind,
  //     factory.floor[main_coord].machine.input_2_want.kind, factory.floor[main_coord].machine.input_2_have.kind,
  //     factory.floor[main_coord].machine.input_3_want.kind, factory.floor[main_coord].machine.input_3_have.kind
  // );

  if
    factory.floor[main_coord].machine.input_1_want.kind == factory.floor[main_coord].machine.input_1_have.kind &&
    factory.floor[main_coord].machine.input_2_want.kind == factory.floor[main_coord].machine.input_2_have.kind &&
    factory.floor[main_coord].machine.input_3_want.kind == factory.floor[main_coord].machine.input_3_have.kind
  {
    // Ready to produce a new part
    if options.print_moves || options.print_moves_machine { super::log(format!("({}) Machine @{} started to create new part", factory.ticks, main_coord).as_str()); }
    factory.floor[main_coord].machine.input_1_have = part_none();
    factory.floor[main_coord].machine.input_2_have = part_none();
    factory.floor[main_coord].machine.input_3_have = part_none();
    factory.floor[main_coord].machine.start_at = factory.ticks;
  }

}
