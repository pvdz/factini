use super::belt_codes::*;
use super::belt_frame::*;
use super::belt_meta::*;
use super::belt_type::*;
use super::belt_type::*;
use super::cell::*;
use super::config::*;
use super::demand::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::machine::*;
use super::options::*;
use super::part::*;
use super::port::*;
use super::port_auto::*;
use super::supply::*;
use super::state::*;
use super::utils::*;
use super::log;

// Clone but not Copy... I don't want to accidentally clone cells when I want to move them
#[derive(Debug, Clone)]
pub struct Belt {
  pub meta: BeltMeta,
  pub part: Part,
  pub part_from: Direction,
  pub part_to: Direction,
  pub part_to_tbd: bool, // If true, ignore part_to and consider it undetermined (usually because there is no available out)
  pub part_at: u64,
  pub part_progress: u64, // Usually factory.ticks - part_at is progress, but a part could be stuck at the center waiting for an outbound port to be available
  pub speed: u64,
  pub tick_price: i32, // Basically the continuous price you pay for having this belt on board, this applies to every tick so keep it low
  pub sprite_start_at: u64, // Begin of sprite animation for this belt
}

pub fn belt_none(config: &Config) -> Belt {
  return Belt {
    meta: BELT_NONE,
    part: part_none(config),
    part_from: Direction::Up,
    part_to: Direction::Up,
    part_to_tbd: true,
    part_at: 0,
    part_progress: 0,
    speed: 0,
    tick_price: 0,
    sprite_start_at: 0,
  };
}

pub fn belt_new(config: &Config, meta: BeltMeta) -> Belt {
  return Belt {
    meta,
    part: part_none(config),
    part_from: Direction::Up,
    part_to: Direction::Up,
    part_to_tbd: true,
    part_at: 0,
    part_progress: 0,
    speed: (ONE_SECOND as f64 * PART_OVER_BELT_SPEED_SEC) as u64,
    tick_price: 0,
    sprite_start_at: 0,
  };
}

fn tick_belt_take_from_belt(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, curr_coord: usize, curr_dir: Direction, from_coord: usize, from_dir: Direction) -> bool {
  // Take from neighbor belt if and only if the part is heading this way and at at least 100%

  // if curr_coord == 52 { log!("tick_belt_one_inbound_dir {:?}", curr_dir); }

  if factory.floor[from_coord].belt.part.kind == CONFIG_NODE_PART_NONE {
    // if curr_coord == 52 {
    //   log!("        - no part, bailing");
    // }
    // Nothing to take here.
    return false;
  }

  if factory.floor[from_coord].belt.part_to != from_dir {
    // if curr_coord == 52 {
    //   log!("        - part not going here, bailing");
    // }
    // Part is not moving into the same direction as from which we are looking right now, bail.
    return false;
  }

  if factory.floor[from_coord].belt.part_progress < factory.floor[from_coord].belt.speed {
    // if curr_coord == 52 {
    //   log!("        - part not at 100%, bailing, {} < {}", factory.floor[from_coord].belt.part_progress, factory.floor[from_coord].belt.speed);
    // }
    // Did not complete traversing the cell yet
    return false;
  }

  // if curr_coord == 52 {
  //   log!("        - ok");
  // }

  // Okay, ready to move that part
  if options.trace_all_moves || options.trace_moves_belt { log!("({}) Moved {:?} from belt @{} to belt @{}", factory.ticks, factory.floor[from_coord].belt.part, from_coord, curr_coord); }
  belt_receive_part(factory, curr_coord, curr_dir, factory.floor[from_coord].belt.part.clone());
  belt_receive_part(factory, from_coord, from_dir, part_none(config));

  return true;
}
fn tick_belt_take_from_supply(options: &mut Options, state: &mut State, factory: &mut Factory, belt_coord: usize, supply_coord: usize, belt_dir: Direction) -> bool {
  // Check if the belt is empty
  // Check if the supply has a part ready to move out
  // If so, move it to this belt
  assert_eq!(factory.floor[belt_coord].belt.part.kind, CONFIG_NODE_PART_NONE, "belt is empty or this should not be called");

  if factory.floor[supply_coord].supply.part_created_at > 0 {
    if factory.floor[supply_coord].supply.part_tbd {
      if (factory.floor[supply_coord].supply.part_progress as f64) / (factory.floor[supply_coord].supply.speed.max(1) as f64) >= 0.5 {
        factory.floor[supply_coord].supply.part_tbd = false;
        return true;
      }
    }
    else {
      if factory.floor[supply_coord].supply.part_progress >= factory.floor[supply_coord].supply.speed {
        if options.trace_all_moves || options.trace_moves_supply { log!("({}) Supply {:?} from @{} to belt @{}", factory.ticks, factory.floor[supply_coord].supply.gives.kind, supply_coord, belt_coord); }
        belt_receive_part(factory, belt_coord, factory.floor[supply_coord].supply.neighbor_incoming_dir, factory.floor[supply_coord].supply.gives.clone());
        supply_clear_part(factory, supply_coord);
        return true;
      }
    }
  }

  return false;
}
fn tick_belt_give_to_demand(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, belt_coord: usize, belt_dir_towards_demand: Direction, demand_coord: usize, demand_dir_coming_from_belt: Direction) -> bool {
  // Check if belt has part
  // Check if belt part is ready to move out
  // Check if belt part is going into this direction
  // If so, move to demand
  if factory.floor[belt_coord].belt.part.kind != CONFIG_NODE_PART_NONE {
    if factory.floor[belt_coord].belt.part_to == belt_dir_towards_demand {
      if factory.floor[belt_coord].belt.part_at > 0 && factory.floor[belt_coord].belt.part_progress >= factory.floor[belt_coord].belt.speed {
        if demand_ready(options, state, config, factory, demand_coord) {
          if options.trace_all_moves || options.trace_moves_demand { log!("({}) Demand takes {:?} at @{} from belt @{}. belt.part_at={:?}, belt_dir={:?}", factory.ticks, factory.floor[belt_coord].belt.part.kind, demand_coord, belt_coord, factory.floor[belt_coord].belt.part_to, belt_dir_towards_demand); }
          demand_receive_part(options, state, config, factory, demand_coord, belt_coord);
          belt_receive_part(factory, belt_coord, Direction::Up, part_none(config));
          return true;
        }
      }
    }
  }
  return false;
}

fn tick_belt_one_outbound_dir(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, curr_coord: usize, curr_dir_towards_neighbor: Direction, to_coord: usize, to_dir_coming_from_curr: Direction) -> bool {
  match factory.floor[to_coord].kind {
    CellKind::Empty => {
      // if !state.test {
        panic!("empty cells should not be part of .outs vector")
        // log!("TODO: empty cells should not be part of .outs vector");
        // state.test = true;
      // }
      // return false;
    },
    CellKind::Belt => {
      // noop
      return false;
    }
    CellKind::Machine => {
      // Machine will do the taking, so skip here.
      // (A belt connected to a machine and a demand will perform very bad)
      return false; // Did not take here
    }
    CellKind::Supply => {
      panic!("Supply can not be outbound");
      // tick_belt_take_from_supply(options, state, factory, curr_coord, to_coord, curr_dir)
    }
    CellKind::Demand => {
      return tick_belt_give_to_demand(options, state, config, factory, curr_coord, curr_dir_towards_neighbor, to_coord, to_dir_coming_from_curr)
    }
  };
}

fn tick_belt_one_inbound_dir(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, curr_coord: usize, curr_dir: Direction, from_coord: usize, from_dir: Direction) -> bool {
  // if curr_coord == 52 { log!("tick_belt_one_inbound_dir {:?} {:?}", curr_dir, factory.floor[from_coord].kind); }

  match factory.floor[from_coord].kind {
    CellKind::Empty => {
      // panic!("empty cells should not be part of .ins vector")
      if !state.test {
        log!("TODO: empty cells should not be part of .ins vector");
        state.test = true;
      }
      return false;
    },
    CellKind::Belt => {
      return tick_belt_take_from_belt(options, state, config, factory, curr_coord, curr_dir, from_coord, from_dir);
    }
    CellKind::Machine => {
      // Do not take from machines. They deal with dispensing parts on their own.
      return false; // Did not take here
    }
    CellKind::Supply => {
      return tick_belt_take_from_supply(options, state, factory, curr_coord, from_coord, curr_dir);
    }
    CellKind::Demand => {
      panic!("Demanders cannot be connected to the inbound port of a belt");
    }
  };
}

pub fn tick_belt(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, curr_coord: usize) {
  // Belts request parts from incoming neighbor belts. This way they can switch multi-way
  // inbound ports properly. Machines do their own switching based on availability.
  // Demanders and suppliers only have one option but should let the belts do the choosing since
  // they may still need to switch their options.

  if !factory.floor[curr_coord].belt.part_to_tbd || (factory.floor[curr_coord].belt.part_progress as f64) / (factory.floor[curr_coord].belt.speed as f64) < 0.5 {
    // Either the part knows where to go to or it has not yet reached the center.
    // Move the progress forward.
    factory.floor[curr_coord].belt.part_progress = (factory.floor[curr_coord].belt.part_progress + 1).min(factory.floor[curr_coord].belt.speed);
  }

  // Try to find a Demand to take a part if one is ready.
  if factory.floor[curr_coord].belt.part.kind != CONFIG_NODE_PART_NONE {
    let outlen = factory.floor[curr_coord].outs.len();
    for index in 0..outlen {
      // Note: not rotating outs here because that already happened when assigning this part_to
      let (curr_dir, _curr_coord, to_coord, to_dir) = factory.floor[curr_coord].outs[index];
      assert_eq!(curr_coord, _curr_coord, "cell .outs cur__coord must match position in factory");
      // The next call will only give to Demand cells, the rest is noops and asserts.
      if tick_belt_one_outbound_dir(options, state, config, factory, curr_coord, curr_dir, to_coord, to_dir) {
        break;
      }
    }
  }

  // Try to take a part from a neighbor belt or supply (but not machine)
  if factory.floor[curr_coord].belt.part.kind == CONFIG_NODE_PART_NONE {
    let inlen = factory.floor[curr_coord].ins.len();
    for index in 0..inlen {
      let (curr_dir, _curr_coord, from_coord, from_dir ) = factory.floor[curr_coord].ins[index];
      assert_eq!(curr_coord, _curr_coord);
      if tick_belt_one_inbound_dir(options, state, config, factory, curr_coord, curr_dir, from_coord, from_dir) {
        back_of_the_line(&mut factory.floor[curr_coord].ins, index);
        // Only take from one inbound port
        break;
      }
    }
  } else if factory.floor[curr_coord].belt.part_to_tbd {
    // This means the belt has a part but it was not yet able to determine to which
    // port the part goes to because there was no port available last tick.
    belt_determine_part_target_port(factory, curr_coord);
  }
}

pub fn belt_receive_part(factory: &mut Factory, curr_coord: usize, curr_dir: Direction, part: Part) {
  let kind = part.kind;

  // println!("({}) - belt @{} is receiving {:?} from {:?}", factory.ticks, coord, part, incoming_dir);
  factory.floor[curr_coord].belt.part = part;
  factory.floor[curr_coord].belt.part_at = factory.ticks;
  factory.floor[curr_coord].belt.part_progress = 0;
  factory.floor[curr_coord].belt.part_to_tbd = true;
  factory.floor[curr_coord].belt.part_from = curr_dir;

  // if curr_coord == 195 { println!("@195 received a {:?}", factory.floor[curr_coord].belt.part); }
  if kind != CONFIG_NODE_PART_NONE {
    belt_determine_part_target_port(factory, curr_coord);
  }
}

pub fn belt_determine_part_target_port(factory: &mut Factory, curr_coord: usize) {
  assert!(factory.floor[curr_coord].kind == CellKind::Belt && factory.floor[curr_coord].belt.part.kind != CONFIG_NODE_PART_NONE && factory.floor[curr_coord].belt.part_to_tbd, "should be a belt where the part is not yet determined");
  // Note: this may leave belt.part_to_tbd as true!

  // Where it goes to depends on a few things;
  // - is the belt one way? (No split, no crossing, no merge)
  // - yes: just go
  // - no:
  //   - are the neighbors of all but one outgoing port taken?
  //     - yes: move to the free one
  //     - no, all taken: freeze part
  //     - no:
  //       - pick one in rotating order

  let outlen = factory.floor[curr_coord].outs.len();

  for i in 0..outlen {
    let ( a_out_dir, a_coord, a_neighbor_coord, a_neighbor_in_dir ) = factory.floor[curr_coord].outs[i];
    let available =
      factory.floor[a_neighbor_coord].kind == CellKind::Demand ||
        factory.floor[a_neighbor_coord].kind == CellKind::Machine || // TODO: if the machine has no space then it should not be considered available here?
        (factory.floor[a_neighbor_coord].kind == CellKind::Belt && factory.floor[a_neighbor_coord].belt.part.kind == CONFIG_NODE_PART_NONE);
    if available {
      factory.floor[curr_coord].belt.part_to = a_out_dir;
      factory.floor[curr_coord].belt.part_to_tbd = false;
      back_of_the_line(&mut factory.floor[curr_coord].outs, i);
      break;
    }
  }
}

pub fn add_unknown_port_to_cell(factory: &Factory, coord: usize, dir: Direction) -> BeltType {
  // Given a coord and two dirs return a belt type that has _a_ port in all the directions of:
  // - the given dirs
  // - the non-none ports of the current cell
  // - any dir where the neighbor is a belt (if flag is not set)

  let Cell { port_u, port_r, port_d, port_l, .. } = factory.floor[coord];

  let port_u = if port_u != Port::None { port_u } else if dir == Direction::Up { Port::Unknown } else { Port::None };
  let port_r = if port_r != Port::None { port_r } else if dir == Direction::Right { Port::Unknown } else { Port::None };
  let port_d = if port_d != Port::None { port_d } else if dir == Direction::Down { Port::Unknown } else { Port::None };
  let port_l = if port_l != Port::None { port_l } else if dir == Direction::Left { Port::Unknown } else { Port::None };

  return belt_type_from_ports(port_u, port_r, port_d, port_l);
}

pub fn add_two_ports_to_cell(factory: &Factory, coord: usize, dir1: Direction, dir2: Direction) -> BeltType {
  // Given a coord and two dirs return a belt type that has _a_ port in all the directions of:
  // - the given dirs
  // - the non-none ports of the current cell
  // - any dir where the neighbor is a belt (if flag is not set)

  let Cell { port_u, port_r, port_d, port_l, .. } = factory.floor[coord];

  let port_u = if port_u != Port::None { port_u } else if dir1 == Direction::Up || dir2 == Direction::Up { Port::Unknown } else { Port::None };
  let port_r = if port_r != Port::None { port_r } else if dir1 == Direction::Right || dir2 == Direction::Right { Port::Unknown } else { Port::None };
  let port_d = if port_d != Port::None { port_d } else if dir1 == Direction::Down || dir2 == Direction::Down { Port::Unknown } else { Port::None };
  let port_l = if port_l != Port::None { port_l } else if dir1 == Direction::Left || dir2 == Direction::Left { Port::Unknown } else { Port::None };

  return belt_type_from_ports(port_u, port_r, port_d, port_l);
}

pub fn belt_discover_ins_and_outs(factory: &mut Factory, coord: usize) {
  // log!("belt_discover_ins_and_outs(@{}) {:?} {:?} {:?} {:?}", coord, factory.floor[coord].port_u, factory.floor[coord].port_r, factory.floor[coord].port_d, factory.floor[coord].port_l);

  factory.floor[coord].ins.clear();
  factory.floor[coord].outs.clear();

  match factory.floor[coord].port_u {
    Port::Inbound => factory.floor[coord].ins.push(( Direction::Up, coord, to_coord_up(coord), Direction::Down )),
    Port::Outbound => factory.floor[coord].outs.push(( Direction::Up, coord, to_coord_up(coord), Direction::Down )),
    Port::None => {}
    Port::Unknown => {}
  };
  match factory.floor[coord].port_r {
    Port::Inbound => factory.floor[coord].ins.push(( Direction::Right, coord, to_coord_right(coord), Direction::Left )),
    Port::Outbound => factory.floor[coord].outs.push(( Direction::Right, coord, to_coord_right(coord), Direction::Left )),
    Port::None => {}
    Port::Unknown => {}
  };
  match factory.floor[coord].port_d {
    Port::Inbound => factory.floor[coord].ins.push(( Direction::Down, coord, to_coord_down(coord), Direction::Up )),
    Port::Outbound => factory.floor[coord].outs.push(( Direction::Down, coord, to_coord_down(coord), Direction::Up )),
    Port::None => {}
    Port::Unknown => {}
  };
  match factory.floor[coord].port_l {
    Port::Inbound => factory.floor[coord].ins.push(( Direction::Left, coord, to_coord_left(coord), Direction::Right )),
    Port::Outbound => factory.floor[coord].outs.push(( Direction::Left, coord, to_coord_left(coord), Direction::Right )),
    Port::None => {}
    Port::Unknown => {}
  };
}

pub fn connect_to_neighbor_dead_end_belts(options: &Options, state: &State, config: &Config, factory: &mut Factory, coord: usize) {
  // log!("connect_to_neighbor_dead_end_belts({})", coord);

  let from_machine = factory.floor[coord].kind == CellKind::Machine;
  // Connect if the neighbor belt is a dead end or has no ports at all
  // Ignores the case where the neighbor is a dead end that leads to this coord already
  // Set both ports to unknown and let auto-porting figure it out
  if let Some(ocoord) = factory.floor[coord].coord_u {
    if factory.floor[ocoord].kind == CellKind::Belt && port_count(factory, ocoord) <= 1 {
      factory.floor[coord].port_u = Port::Unknown;
      factory.floor[ocoord].port_d = Port::Unknown;
      fix_belt_meta(options, state, config, factory, ocoord);
      fix_belt_meta(options, state, config, factory, coord);
    }
  }
  if let Some(ocoord) = factory.floor[coord].coord_r {
    if factory.floor[ocoord].kind == CellKind::Belt && port_count(factory, ocoord) <= 1 {
      factory.floor[coord].port_r = Port::Unknown;
      factory.floor[ocoord].port_l = Port::Unknown;
      fix_belt_meta(options, state, config, factory, ocoord);
      fix_belt_meta(options, state, config, factory, coord);
    }
  }
  if let Some(ocoord) = factory.floor[coord].coord_d {
    if factory.floor[ocoord].kind == CellKind::Belt && port_count(factory, ocoord) <= 1 {
      factory.floor[coord].port_d = Port::Unknown;
      factory.floor[ocoord].port_u = Port::Unknown;
      fix_belt_meta(options, state, config, factory, ocoord);
      fix_belt_meta(options, state, config, factory, coord);
    }
  }
  if let Some(ocoord) = factory.floor[coord].coord_l {
    if factory.floor[ocoord].kind == CellKind::Belt && port_count(factory, ocoord) <= 1 {
      factory.floor[coord].port_l = Port::Unknown;
      factory.floor[ocoord].port_r = Port::Unknown;
      fix_belt_meta(options, state, config, factory, ocoord);
      fix_belt_meta(options, state, config, factory, coord);
    }
  }
}
