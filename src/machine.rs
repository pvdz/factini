use crate::direction::Direction;
use super::belt::*;
use super::cell::*;
use super::floor::*;
use super::factory::*;
use super::options::*;
use super::part::*;
use super::port::*;
use super::port_auto::*;
use super::state::*;
use super::utils::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MachineKind {
  None,
  Unknown, // Not yet defined
  Main,
  SubBuilding, // Extra part but not the main building
}

// Clone but not Copy... I don't want to accidentally clone cells when I want to move them
#[derive(Debug, Clone)]
pub struct Machine {
  pub kind: MachineKind,
  pub main_coord: usize, // If this is a sub building, what is the coord of the main machine cell?
  pub coords: Vec<usize>, // First element is main coord. List of coords part of this machine
  pub cell_width: usize, // Number of cells this factory spans
  pub cell_height: usize,
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
  pub produced: u64,
  pub trash_price: i32, // Price you pay when a machine has to discard an invalid part
  pub trashed: u64,
}

pub const fn machine_none(main_coord: usize) -> Machine {
  return Machine {
    kind: MachineKind::None,
    main_coord,
    coords: vec!(),
    cell_width: 0,
    cell_height: 0,
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
    produced: 0,
    trash_price: 0,
    trashed: 0,
  };
}

pub fn machine_new(kind: MachineKind, cell_width: usize, cell_height: usize, id: usize, main_coord: usize, input1: Part, input2: Part, input3: Part, output: Part, speed: u64) -> Machine {
  return Machine {
    kind,
    main_coord,
    coords: vec!(),
    cell_width,
    cell_height,
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

    speed,
    production_price: 0,
    produced: 0,
    trash_price: 0,
    trashed: 0,
  };
}

pub fn tick_machine(options: &mut Options, state: &mut State, factory: &mut Factory, main_coord: usize) {
  // Finish building by giving the wanted part to a neighbor outbound belt
  // Accept from the input ports
  // Start building

  if factory.floor[main_coord].machine.start_at > 0 && factory.ticks - factory.floor[main_coord].machine.start_at >= factory.floor[main_coord].machine.speed {
    // log(format!("part {} is finished", factory.floor[main_coord].machine.output_want.icon));
    // Finished! Find an available outlet.
    let mut handed_off = false;
    let outlen = factory.floor[main_coord].outs.len();
    for index in 0..outlen {
      let (sub_dir, sub_coord, _, _ ) = factory.floor[main_coord].outs[index];
      let (to_coord, to_dir) = match sub_dir {
        Direction::Up => ( factory.floor[sub_coord].coord_u, Direction::Down ),
        Direction::Right => ( factory.floor[sub_coord].coord_r, Direction::Left ),
        Direction::Down => ( factory.floor[sub_coord].coord_d, Direction::Up ),
        Direction::Left => ( factory.floor[sub_coord].coord_l, Direction::Right ),
      };
      if let Some(to_coord) = to_coord {
        if factory.floor[to_coord].kind == CellKind::Belt && factory.floor[to_coord].belt.part.kind == PartKind::None {
          // The neighbor is a belt that is empty
          if options.print_moves || options.print_moves_machine { log(format!("({}) Machine @{} (sub @{}) finished part {:?}! Moving to belt @{}", factory.ticks, main_coord, sub_coord, factory.floor[main_coord].machine.output_want.kind, to_coord)); }

          belt_receive_part(factory, to_coord, to_dir, factory.floor[main_coord].machine.output_want.clone());
          factory.floor[main_coord].machine.start_at = 0;
          handed_off = true;
          factory.floor[main_coord].machine.produced += 1;
          back_of_the_line(&mut factory.floor[main_coord].outs, index);
          break;
        }
      }
    }
    if !handed_off {
      println!("machine has a part but is unable to unload it...");
    }
  }
  //
  // if main_coord == 20 {
  //   log(format!("machine @{}: {:?}>{:?} {:?}>{:?} {:?}>{:?}",
  //     main_coord,
  //     factory.floor[main_coord].machine.input_1_want.kind, factory.floor[main_coord].machine.input_1_have.kind,
  //     factory.floor[main_coord].machine.input_2_want.kind, factory.floor[main_coord].machine.input_2_have.kind,
  //     factory.floor[main_coord].machine.input_3_want.kind, factory.floor[main_coord].machine.input_3_have.kind,
  //   ));
  // }

  // It should only trash the input if it's actually still waiting for something so check that first
  if
    factory.floor[main_coord].machine.input_1_want.kind != factory.floor[main_coord].machine.input_1_have.kind ||
    factory.floor[main_coord].machine.input_2_want.kind != factory.floor[main_coord].machine.input_2_have.kind ||
    factory.floor[main_coord].machine.input_3_want.kind != factory.floor[main_coord].machine.input_3_have.kind
  {
    // Find the input connected to a belt with matching part as any of the inputs that await one
    for index in 0..factory.floor[main_coord].ins.len() {
      let (sub_dir, sub_coord, _main_neighbor_coord, main_neighbor_in_dir) = factory.floor[main_coord].ins[index];
      let (from_coord, incoming_dir ) = match sub_dir {
        Direction::Up => ( factory.floor[sub_coord].coord_u, Direction::Down ),
        Direction::Right => ( factory.floor[sub_coord].coord_r, Direction::Left ),
        Direction::Down => ( factory.floor[sub_coord].coord_d, Direction::Up ),
        Direction::Left => ( factory.floor[sub_coord].coord_l, Direction::Right ),
      };

      // Can't we assume ocoord exists here (use unwrap instead)?
      if let Some(from_coord) = from_coord {
        if factory.floor[from_coord].kind == CellKind::Belt {
          // Verify that there is a part, the part is at 100% progress, and that the part is determined to go towards the machine
          let belt_part = factory.floor[from_coord].belt.part.kind;
          if belt_part != PartKind::None && !factory.floor[from_coord].belt.part_to_tbd && factory.floor[from_coord].belt.part_to == main_neighbor_in_dir && factory.floor[from_coord].belt.part_progress >= factory.floor[from_coord].belt.speed {
            // Check whether it fits in any input slot. If so, put it there. Otherwise trash it unless all slots are full (only trash input parts while actually waiting for more input).
            if belt_part == factory.floor[main_coord].machine.input_1_want.kind && belt_part != factory.floor[main_coord].machine.input_1_have.kind && factory.floor[main_coord].machine.input_1_have.kind == PartKind::None {
              if options.print_moves || options.print_moves_machine {
                log(format!("({}) Machine @{} (sub @{}) accepting part {:?} as input1 from belt @{}, had {:?}", factory.ticks, main_coord, sub_coord, belt_part, from_coord, factory.floor[main_coord].machine.input_1_have));
              }
              factory.floor[main_coord].machine.input_1_have = factory.floor[from_coord].belt.part.clone();
              belt_receive_part(factory, from_coord, incoming_dir, part_none());
            } else if belt_part == factory.floor[main_coord].machine.input_2_want.kind && belt_part != factory.floor[main_coord].machine.input_2_have.kind && factory.floor[main_coord].machine.input_2_have.kind == PartKind::None {
              if options.print_moves || options.print_moves_machine {
                log(format!("({}) Machine @{} (sub @{}) accepting part {:?} as input2 from belt @{}, had {:?}", factory.ticks, main_coord, sub_coord, belt_part, from_coord, factory.floor[main_coord].machine.input_2_have));
              }
              factory.floor[main_coord].machine.input_2_have = factory.floor[from_coord].belt.part.clone();
              belt_receive_part(factory, from_coord, incoming_dir, part_none());
            } else if belt_part == factory.floor[main_coord].machine.input_3_want.kind && belt_part != factory.floor[main_coord].machine.input_3_have.kind && factory.floor[main_coord].machine.input_3_have.kind == PartKind::None {
              if options.print_moves || options.print_moves_machine {
                log(format!("({}) Machine @{} (sub @{}) accepting part {:?} as input3 from belt @{}, had {:?}", factory.ticks, main_coord, sub_coord, belt_part, from_coord, factory.floor[main_coord].machine.input_3_have));
              }
              factory.floor[main_coord].machine.input_3_have = factory.floor[from_coord].belt.part.clone();
              belt_receive_part(factory, from_coord, incoming_dir, part_none());
            } else {
              // Trash it? TODO: machine with multiple inputs should perhaps only trash if none of the inputs were acceptable?
              if options.print_moves || options.print_moves_machine { log(format!("({}) Machine @{} (sub @{}) trashing part {:?} from belt @{}; wants: {:?} {:?} {:?}, has: {:?} {:?} {:?}", factory.ticks, main_coord, sub_coord, belt_part, from_coord,
                factory.floor[main_coord].machine.input_1_want.kind,
                factory.floor[main_coord].machine.input_2_want.kind,
                factory.floor[main_coord].machine.input_3_want.kind,
                factory.floor[main_coord].machine.input_1_have.kind,
                factory.floor[main_coord].machine.input_2_have.kind,
                factory.floor[main_coord].machine.input_3_have.kind,
              )); }
              belt_receive_part(factory, from_coord, incoming_dir, part_none());
              factory.floor[main_coord].machine.trashed += 1;
            }
          }
        }
      }
    }
  }

  if factory.floor[main_coord].machine.start_at == 0 {
    if
      factory.floor[main_coord].machine.input_1_want.kind == factory.floor[main_coord].machine.input_1_have.kind &&
      factory.floor[main_coord].machine.input_2_want.kind == factory.floor[main_coord].machine.input_2_have.kind &&
      factory.floor[main_coord].machine.input_3_want.kind == factory.floor[main_coord].machine.input_3_have.kind
    {
      // Ready to produce a new part
      if options.print_moves || options.print_moves_machine { log(format!("({}) Machine @{} started to create new part", factory.ticks, main_coord)); }
      factory.floor[main_coord].machine.input_1_have = part_none();
      factory.floor[main_coord].machine.input_2_have = part_none();
      factory.floor[main_coord].machine.input_3_have = part_none();
      factory.floor[main_coord].machine.start_at = factory.ticks;
    }
  }
}

pub fn machine_discover_ins_and_outs(factory: &mut Factory, main_coord: usize) {
  machine_discover_ins_and_outs_floor(&mut factory.floor, main_coord);
}
pub fn machine_discover_ins_and_outs_floor(floor: &mut [Cell; FLOOR_CELLS_WH], main_coord: usize) {
  floor[main_coord].ins.clear();
  floor[main_coord].outs.clear();

  for index in 0..floor[main_coord].machine.coords.len() {
    let coord = floor[main_coord].machine.coords[index];
    match floor[coord].port_u {
      Port::Inbound => floor[main_coord].ins.push(( Direction::Up, coord, to_coord_up(coord), Direction::Down )),
      Port::Outbound => floor[main_coord].outs.push(( Direction::Up, coord, to_coord_up(coord), Direction::Down )),
      Port::None => {}
      Port::Unknown => {}
    };
    match floor[coord].port_r {
      Port::Inbound => floor[main_coord].ins.push(( Direction::Right, coord, to_coord_right(coord), Direction::Left )),
      Port::Outbound => floor[main_coord].outs.push(( Direction::Right, coord, to_coord_right(coord), Direction::Left )),
      Port::None => {}
      Port::Unknown => {}
    };
    match floor[coord].port_d {
      Port::Inbound => floor[main_coord].ins.push(( Direction::Down, coord, to_coord_down(coord), Direction::Up )),
      Port::Outbound => floor[main_coord].outs.push(( Direction::Down, coord, to_coord_down(coord), Direction::Up )),
      Port::None => {}
      Port::Unknown => {}
    };
    match floor[coord].port_l {
      Port::Inbound => floor[main_coord].ins.push(( Direction::Left, coord, to_coord_left(coord), Direction::Right )),
      Port::Outbound => floor[main_coord].outs.push(( Direction::Left, coord, to_coord_left(coord), Direction::Right )),
      Port::None => {}
      Port::Unknown => {}
    };
  }
}
