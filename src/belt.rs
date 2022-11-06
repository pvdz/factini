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

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BeltType {
  NONE = 0,
  U_R = 1,
  R_U = 2,
  R_D = 3,
  D_R = 4,
  D_L = 5,
  L_D = 6,
  L_U = 7,
  U_L = 8,
  U_D = 9,
  D_U = 10,
  L_R = 11,
  R_L = 12,
  U_LR = 13,
  RU_L = 14,
  LU_R = 15,
  L_RU = 16,
  LR_U = 17,
  R_LU = 18,
  R_DU = 19,
  RU_D = 20,
  DR_U = 21,
  DU_R = 22,
  U_DR = 23,
  D_RU = 24,
  D_LR = 25,
  DL_R = 26,
  DR_L = 27,
  LR_D = 28,
  L_DR = 29,
  R_DL = 30,
  L_DU = 31,
  LU_D = 32,
  DL_U = 33,
  DU_L = 34,
  U_DL = 35,
  D_LU = 36,
  U_DLR = 37,
  R_DLU = 38,
  D_LRU = 39,
  L_DRU = 40,
  RU_DL = 41,
  DU_LR = 42,
  LU_DR = 43,
  DL_RU = 44,
  DR_LU = 45,
  LR_DU = 46,
  DLR_U = 47,
  DLU_R = 48,
  LRU_D = 49,
  DRU_L = 50,
  RU = 51,
  DR = 52,
  DL = 53,
  LU = 54,
  DU = 55,
  LR = 56,
  LRU = 57,
  DRU = 58,
  DLR = 59,
  DLU = 60,
  DLRU = 61,

  UNKNOWN = 62,
  INVALID = 63, // Keep last item
}
// Keep in sync...
pub const BELT_TYPE_COUNT: usize = (BeltType::INVALID as usize) + 1;

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
}

pub fn belt_auto_layout(up: CellKind, right: CellKind, down: CellKind, left: CellKind) -> BeltMeta {
  // log(format!("belt_auto_layout({:?}, {:?}, {:?}, {:?})", up, right, down, left));
  return match
    (up,             right,          down,           left)
  {
    // Straight
    (CellKind::Empty, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Empty, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand) => BELT_LR,
    (CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Empty, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Empty) => BELT_DU,

    // Corner
    (CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Empty, CellKind::Empty) => BELT_RU,
    (CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Empty, CellKind::Empty, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand) => BELT_LU,
    (CellKind::Empty, CellKind::Empty, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand) => BELT_DL,
    (CellKind::Empty, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Empty) => BELT_DR,

    // T
    (CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Empty, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand) => BELT_LRU,
    (CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Empty, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand) => BELT_DLU,
    (CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Empty) => BELT_DRU,
    (CellKind::Empty, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand) => BELT_DLR,

    // +
    (CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand, CellKind::Belt|CellKind::Machine|CellKind::Supply|CellKind::Demand) => BELT_DLRU,
    _ => BELT_UNKNOWN,
  };
}


// pub fn port_config_to_belt(up: Port, right: Port, down: Port, left: Port) -> BeltMeta {
//   match   (up,          right,          down,           left) {
//     (Port::Inbound,  Port::Inbound,  Port::Inbound,  Port::Inbound) =>   CELL_BELT_INVALID,
//     (Port::Inbound,  Port::Inbound,  Port::Inbound,  Port::Outbound) =>  CELL_BELT_DRU_L,
//     (Port::Inbound,  Port::Inbound,  Port::Inbound,  Port::None) =>      CELL_BELT_INVALID,
//     (Port::Inbound,  Port::Inbound,  Port::Outbound, Port::Inbound) =>   CELL_BELT_LRU_D,
//     (Port::Inbound,  Port::Inbound,  Port::Outbound, Port::Outbound) =>  CELL_BELT_RU_DL,
//     (Port::Inbound,  Port::Inbound,  Port::Outbound, Port::None) =>      CELL_BELT_RU_D,
//     (Port::Inbound,  Port::Inbound,  Port::None,     Port::Inbound) =>   CELL_BELT_INVALID,
//     (Port::Inbound,  Port::Inbound,  Port::None,     Port::Outbound) =>  CELL_BELT_RU_L,
//     (Port::Inbound,  Port::Inbound,  Port::None,     Port::None) =>      CELL_BELT_INVALID,
//     (Port::Inbound,  Port::Outbound, Port::Inbound,  Port::Inbound) =>   CELL_BELT_DLU_R,
//     (Port::Inbound,  Port::Outbound, Port::Inbound,  Port::Outbound) =>  CELL_BELT_DU_LR,
//     (Port::Inbound,  Port::Outbound, Port::Inbound,  Port::None) =>      CELL_BELT_DU_R,
//     (Port::Inbound,  Port::Outbound, Port::Outbound, Port::Inbound) =>   CELL_BELT_LU_DR,
//     (Port::Inbound,  Port::Outbound, Port::Outbound, Port::Outbound) =>  CELL_BELT_U_DLR,
//     (Port::Inbound,  Port::Outbound, Port::Outbound, Port::None) =>      CELL_BELT_U_DR,
//     (Port::Inbound,  Port::Outbound, Port::None,     Port::Inbound) =>   CELL_BELT_LU_R,
//     (Port::Inbound,  Port::Outbound, Port::None,     Port::Outbound) =>  CELL_BELT_U_LR,
//     (Port::Inbound,  Port::Outbound, Port::None,     Port::None) =>      CELL_BELT_U_R,
//     (Port::Inbound,  Port::None,     Port::Inbound,  Port::Inbound) =>   CELL_BELT_INVALID,
//     (Port::Inbound,  Port::None,     Port::Inbound,  Port::Outbound) =>  CELL_BELT_DU_L,
//     (Port::Inbound,  Port::None,     Port::Inbound,  Port::None) =>      CELL_BELT_INVALID,
//     (Port::Inbound,  Port::None,     Port::Outbound, Port::Inbound) =>   CELL_BELT_LU_D,
//     (Port::Inbound,  Port::None,     Port::Outbound, Port::Outbound) =>  CELL_BELT_U_DL,
//     (Port::Inbound,  Port::None,     Port::Outbound, Port::None) =>      CELL_BELT_U_D,
//     (Port::Inbound,  Port::None,     Port::None,     Port::Inbound) =>   CELL_BELT_INVALID,
//     (Port::Inbound,  Port::None,     Port::None,     Port::Outbound) =>  CELL_BELT_U_L,
//     (Port::Inbound,  Port::None,     Port::None,     Port::None) =>      CELL_BELT_INVALID,
//     (Port::Outbound, Port::Inbound,  Port::Inbound,  Port::Inbound) =>   CELL_BELT_DLR_U,
//     (Port::Outbound, Port::Inbound,  Port::Inbound,  Port::Outbound) =>  CELL_BELT_DR_LU,
//     (Port::Outbound, Port::Inbound,  Port::Inbound,  Port::None) =>      CELL_BELT_DR_U,
//     (Port::Outbound, Port::Inbound,  Port::Outbound, Port::Inbound) =>   CELL_BELT_LR_DU,
//     (Port::Outbound, Port::Inbound,  Port::Outbound, Port::Outbound) =>  CELL_BELT_R_DLU,
//     (Port::Outbound, Port::Inbound,  Port::Outbound, Port::None) =>      CELL_BELT_R_DU,
//     (Port::Outbound, Port::Inbound,  Port::None,     Port::Inbound) =>   CELL_BELT_LR_U,
//     (Port::Outbound, Port::Inbound,  Port::None,     Port::Outbound) =>  CELL_BELT_R_LU,
//     (Port::Outbound, Port::Inbound,  Port::None,     Port::None) =>      CELL_BELT_R_U,
//     (Port::Outbound, Port::Outbound, Port::Inbound,  Port::Inbound) =>   CELL_BELT_DL_RU,
//     (Port::Outbound, Port::Outbound, Port::Inbound,  Port::Outbound) =>  CELL_BELT_D_LRU,
//     (Port::Outbound, Port::Outbound, Port::Inbound,  Port::None) =>      CELL_BELT_D_RU,
//     (Port::Outbound, Port::Outbound, Port::Outbound, Port::Inbound) =>   CELL_BELT_L_DRU,
//     (Port::Outbound, Port::Outbound, Port::Outbound, Port::Outbound) =>  CELL_BELT_INVALID,
//     (Port::Outbound, Port::Outbound, Port::Outbound, Port::None) =>      CELL_BELT_INVALID,
//     (Port::Outbound, Port::Outbound, Port::None,     Port::Inbound) =>   CELL_BELT_L_RU,
//     (Port::Outbound, Port::Outbound, Port::None,     Port::Outbound) =>  CELL_BELT_INVALID,
//     (Port::Outbound, Port::Outbound, Port::None,     Port::None) =>      CELL_BELT_INVALID,
//     (Port::Outbound, Port::None,     Port::Inbound,  Port::Inbound) =>   CELL_BELT_DL_U,
//     (Port::Outbound, Port::None,     Port::Inbound,  Port::Outbound) =>  CELL_BELT_D_LU,
//     (Port::Outbound, Port::None,     Port::Inbound,  Port::None) =>      CELL_BELT_D_U,
//     (Port::Outbound, Port::None,     Port::Outbound, Port::Inbound) =>   CELL_BELT_L_DU,
//     (Port::Outbound, Port::None,     Port::Outbound, Port::Outbound) =>  CELL_BELT_INVALID,
//     (Port::Outbound, Port::None,     Port::Outbound, Port::None) =>      CELL_BELT_INVALID,
//     (Port::Outbound, Port::None,     Port::None,     Port::Inbound) =>   CELL_BELT_L_U,
//     (Port::Outbound, Port::None,     Port::None,     Port::Outbound) =>  CELL_BELT_INVALID,
//     (Port::Outbound, Port::None,     Port::None,     Port::None) =>      CELL_BELT_INVALID,
//     (Port::None,     Port::Inbound,  Port::Inbound,  Port::Inbound) =>   CELL_BELT_INVALID,
//     (Port::None,     Port::Inbound,  Port::Inbound,  Port::Outbound) =>  CELL_BELT_DR_L,
//     (Port::None,     Port::Inbound,  Port::Inbound,  Port::None) =>      CELL_BELT_INVALID,
//     (Port::None,     Port::Inbound,  Port::Outbound, Port::Inbound) =>   CELL_BELT_LR_D,
//     (Port::None,     Port::Inbound,  Port::Outbound, Port::Outbound) =>  CELL_BELT_R_DL,
//     (Port::None,     Port::Inbound,  Port::Outbound, Port::None) =>      CELL_BELT_R_D,
//     (Port::None,     Port::Inbound,  Port::None,     Port::Inbound) =>   CELL_BELT_INVALID,
//     (Port::None,     Port::Inbound,  Port::None,     Port::Outbound) =>  CELL_BELT_R_L,
//     (Port::None,     Port::Inbound,  Port::None,     Port::None) =>      CELL_BELT_INVALID,
//     (Port::None,     Port::Outbound, Port::Inbound,  Port::Inbound) =>   CELL_BELT_DL_R,
//     (Port::None,     Port::Outbound, Port::Inbound,  Port::Outbound) =>  CELL_BELT_D_LR,
//     (Port::None,     Port::Outbound, Port::Inbound,  Port::None) =>      CELL_BELT_D_R,
//     (Port::None,     Port::Outbound, Port::Outbound, Port::Inbound) =>   CELL_BELT_L_DR,
//     (Port::None,     Port::Outbound, Port::Outbound, Port::Outbound) =>  CELL_BELT_INVALID,
//     (Port::None,     Port::Outbound, Port::Outbound, Port::None) =>      CELL_BELT_INVALID,
//     (Port::None,     Port::Outbound, Port::None,     Port::Inbound) =>   CELL_BELT_L_R,
//     (Port::None,     Port::Outbound, Port::None,     Port::Outbound) =>  CELL_BELT_INVALID,
//     (Port::None,     Port::Outbound, Port::None,     Port::None) =>      CELL_BELT_INVALID,
//     (Port::None,     Port::None,     Port::Inbound,  Port::Inbound) =>   CELL_BELT_INVALID,
//     (Port::None,     Port::None,     Port::Inbound,  Port::Outbound) =>  CELL_BELT_D_L,
//     (Port::None,     Port::None,     Port::Inbound,  Port::None) =>      CELL_BELT_INVALID,
//     (Port::None,     Port::None,     Port::Outbound, Port::Inbound) =>   CELL_BELT_L_D,
//     (Port::None,     Port::None,     Port::Outbound, Port::Outbound) =>  CELL_BELT_INVALID,
//     (Port::None,     Port::None,     Port::Outbound, Port::None) =>      CELL_BELT_INVALID,
//     (Port::None,     Port::None,     Port::None,     Port::Inbound) =>   CELL_BELT_INVALID,
//     (Port::None,     Port::None,     Port::None,     Port::Outbound) =>  CELL_BELT_INVALID,
//     (Port::None,     Port::None,     Port::None,     Port::None) =>      CELL_BELT_INVALID,
//   }
// }

// Clone but not Copy... I don't want to accidentally clone cells when I want to move them
#[derive(Debug, Clone)]
pub struct BeltMeta {
  pub btype: BeltType, // BELT_FROM_TO
  pub dbg: &'static str,
  pub src: &'static str, // tile image
  // TBD if I want to keep this
  pub port_u: Port,
  pub port_r: Port,
  pub port_d: Port,
  pub port_l: Port,
  // simplify cli output painting
  pub cli_icon: char,
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
    tick_price: 0
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
    speed: ONE_SECOND / 5,
    tick_price: 0
  };
}

fn tick_belt_take_from_belt(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, curr_coord: usize, curr_dir: Direction, from_coord: usize, from_dir: Direction) -> bool {
  // Take from neighbor belt if and only if the part is heading this way and at at least 100%

  // if curr_coord == 52 { log(format!("tick_belt_one_inbound_dir {:?}", curr_dir)); }

  if factory.floor[from_coord].belt.part.kind == PARTKIND_NONE {
    // if curr_coord == 52 {
    //   log(format!("        - no part, bailing"));
    // }
    // Nothing to take here.
    return false;
  }

  if factory.floor[from_coord].belt.part_to != from_dir {
    // if curr_coord == 52 {
    //   log(format!("        - part not going here, bailing"));
    // }
    // Part is not moving into the same direction as from which we are looking right now, bail.
    return false;
  }

  if factory.floor[from_coord].belt.part_progress < factory.floor[from_coord].belt.speed {
    // if curr_coord == 52 {
    //   log(format!("        - part not at 100%, bailing, {} < {}", factory.floor[from_coord].belt.part_progress, factory.floor[from_coord].belt.speed));
    // }
    // Did not complete traversing the cell yet
    return false;
  }

  // if curr_coord == 52 {
  //   log(format!("        - ok"));
  // }

  // Okay, ready to move that part
  if options.print_moves || options.print_moves_belt { log(format!("({}) Moved {:?} from belt @{} to belt @{}", factory.ticks, factory.floor[from_coord].belt.part, from_coord, curr_coord)); }
  belt_receive_part(factory, curr_coord, curr_dir, factory.floor[from_coord].belt.part.clone());
  belt_receive_part(factory, from_coord, from_dir, part_none(config));

  return true;
}
fn tick_belt_take_from_supply(options: &mut Options, state: &mut State, factory: &mut Factory, belt_coord: usize, supply_coord: usize, belt_dir: Direction) -> bool {
  // Check if the belt is empty
  // Check if the supply has a part ready to move out
  // If so, move it to this belt
  assert_eq!(factory.floor[belt_coord].belt.part.kind, PARTKIND_NONE, "belt is empty or this should not be called");

  if factory.floor[supply_coord].supply.part_created_at > 0 {
    if factory.floor[supply_coord].supply.part_tbd {
      if (factory.floor[supply_coord].supply.part_progress as f64) / (factory.floor[supply_coord].supply.speed.max(1) as f64) >= 0.5 {
        factory.floor[supply_coord].supply.part_tbd = false;
        return true;
      }
    }
    else {
      if factory.floor[supply_coord].supply.part_progress >= factory.floor[supply_coord].supply.speed {
        if options.print_moves || options.print_moves_supply { log(format!("({}) Supply {:?} from @{} to belt @{}", factory.ticks, factory.floor[supply_coord].supply.gives.kind, supply_coord, belt_coord)); }
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
  if factory.floor[belt_coord].belt.part.kind != PARTKIND_NONE {
    if factory.floor[belt_coord].belt.part_to == belt_dir_towards_demand {
      if factory.floor[belt_coord].belt.part_at > 0 && factory.floor[belt_coord].belt.part_progress >= factory.floor[belt_coord].belt.speed {
        if options.print_moves || options.print_moves_demand { log(format!("({}) Demand takes {:?} at @{} from belt @{}. belt.part_at={:?}, belt_dir={:?}", factory.ticks, factory.floor[belt_coord].belt.part.kind, demand_coord, belt_coord, factory.floor[belt_coord].belt.part_to, belt_dir_towards_demand)); }
        demand_receive_part(options, state, config, factory, demand_coord, belt_coord);
        belt_receive_part(factory, belt_coord, Direction::Up, part_none(config));
        return true;
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
        // log(format!("TODO: empty cells should not be part of .outs vector"));
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
  // if curr_coord == 52 { log(format!("tick_belt_one_inbound_dir {:?} {:?}", curr_dir, factory.floor[from_coord].kind)); }

  match factory.floor[from_coord].kind {
    CellKind::Empty => {
      // panic!("empty cells should not be part of .ins vector")
      if !state.test {
        log(format!("TODO: empty cells should not be part of .ins vector"));
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
  if factory.floor[curr_coord].belt.part.kind != PARTKIND_NONE {
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
  if factory.floor[curr_coord].belt.part.kind == PARTKIND_NONE {
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
  if kind != PARTKIND_NONE {
    belt_determine_part_target_port(factory, curr_coord);
  }
}

pub fn belt_determine_part_target_port(factory: &mut Factory, curr_coord: usize) {
  assert!(factory.floor[curr_coord].kind == CellKind::Belt && factory.floor[curr_coord].belt.part.kind != PARTKIND_NONE && factory.floor[curr_coord].belt.part_to_tbd, "should be a belt where the part is not yet determined");
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
        (factory.floor[a_neighbor_coord].kind == CellKind::Belt && factory.floor[a_neighbor_coord].belt.part.kind == PARTKIND_NONE);
    if available {
      factory.floor[curr_coord].belt.part_to = a_out_dir;
      factory.floor[curr_coord].belt.part_to_tbd = false;
      back_of_the_line(&mut factory.floor[curr_coord].outs, i);
      break;
    }
  }
}

// https://en.wikipedia.org/wiki/Box-drawing_character

fn boxx(up: Port, right: Port, down: Port, left: Port) -> char {
  // Each port as four possible states (none, in, out, unknown) and there are four ports so there
  // are 4^4=256 options. Luckily ascii delivers.
  // empty: none
  // in: thin
  // out: thick
  // unknown: double, except when there is no double for that case, then use the T with double
  match (up, right, down, left) {
    (Port::None, Port::None, Port::None, Port::None) => ' ',
    (Port::None, Port::None, Port::None, Port::Inbound) => '╴',
    (Port::None, Port::None, Port::None, Port::Outbound) => '╸',
    (Port::None, Port::None, Port::None, Port::Unknown) => '╡', // There is no "just-left" double char
    (Port::None, Port::None, Port::Inbound, Port::None) => '╷',
    (Port::None, Port::None, Port::Inbound, Port::Inbound) => '┐',
    (Port::None, Port::None, Port::Inbound, Port::Outbound) => '┑',
    (Port::None, Port::None, Port::Inbound, Port::Unknown) => '╕',
    (Port::None, Port::None, Port::Outbound, Port::None) => ' ',
    (Port::None, Port::None, Port::Outbound, Port::Inbound) => ' ',
    (Port::None, Port::None, Port::Outbound, Port::Outbound) => ' ',
    (Port::None, Port::None, Port::Outbound, Port::Unknown) => ' ',
    (Port::None, Port::None, Port::Unknown, Port::None) => ' ',
    (Port::None, Port::None, Port::Unknown, Port::Inbound) => ' ',
    (Port::None, Port::None, Port::Unknown, Port::Outbound) => ' ',
    (Port::None, Port::None, Port::Unknown, Port::Unknown) => ' ',
    (Port::None, Port::Inbound, Port::None, Port::None) => ' ',
    (Port::None, Port::Inbound, Port::None, Port::Inbound) => ' ',
    (Port::None, Port::Inbound, Port::None, Port::Outbound) => ' ',
    (Port::None, Port::Inbound, Port::None, Port::Unknown) => ' ',
    (Port::None, Port::Inbound, Port::Inbound, Port::None) => ' ',
    (Port::None, Port::Inbound, Port::Inbound, Port::Inbound) => ' ',
    (Port::None, Port::Inbound, Port::Inbound, Port::Outbound) => ' ',
    (Port::None, Port::Inbound, Port::Inbound, Port::Unknown) => ' ',
    (Port::None, Port::Inbound, Port::Outbound, Port::None) => ' ',
    (Port::None, Port::Inbound, Port::Outbound, Port::Inbound) => ' ',
    (Port::None, Port::Inbound, Port::Outbound, Port::Outbound) => ' ',
    (Port::None, Port::Inbound, Port::Outbound, Port::Unknown) => ' ',
    (Port::None, Port::Inbound, Port::Unknown, Port::None) => ' ',
    (Port::None, Port::Inbound, Port::Unknown, Port::Inbound) => ' ',
    (Port::None, Port::Inbound, Port::Unknown, Port::Outbound) => ' ',
    (Port::None, Port::Inbound, Port::Unknown, Port::Unknown) => ' ',
    (Port::None, Port::Outbound, Port::None, Port::None) => ' ',
    (Port::None, Port::Outbound, Port::None, Port::Inbound) => ' ',
    (Port::None, Port::Outbound, Port::None, Port::Outbound) => ' ',
    (Port::None, Port::Outbound, Port::None, Port::Unknown) => ' ',
    (Port::None, Port::Outbound, Port::Inbound, Port::None) => ' ',
    (Port::None, Port::Outbound, Port::Inbound, Port::Inbound) => ' ',
    (Port::None, Port::Outbound, Port::Inbound, Port::Outbound) => ' ',
    (Port::None, Port::Outbound, Port::Inbound, Port::Unknown) => ' ',
    (Port::None, Port::Outbound, Port::Outbound, Port::None) => ' ',
    (Port::None, Port::Outbound, Port::Outbound, Port::Inbound) => ' ',
    (Port::None, Port::Outbound, Port::Outbound, Port::Outbound) => ' ',
    (Port::None, Port::Outbound, Port::Outbound, Port::Unknown) => ' ',
    (Port::None, Port::Outbound, Port::Unknown, Port::None) => ' ',
    (Port::None, Port::Outbound, Port::Unknown, Port::Inbound) => ' ',
    (Port::None, Port::Outbound, Port::Unknown, Port::Outbound) => ' ',
    (Port::None, Port::Outbound, Port::Unknown, Port::Unknown) => ' ',
    (Port::None, Port::Unknown, Port::None, Port::None) => ' ',
    (Port::None, Port::Unknown, Port::None, Port::Inbound) => ' ',
    (Port::None, Port::Unknown, Port::None, Port::Outbound) => ' ',
    (Port::None, Port::Unknown, Port::None, Port::Unknown) => ' ',
    (Port::None, Port::Unknown, Port::Inbound, Port::None) => ' ',
    (Port::None, Port::Unknown, Port::Inbound, Port::Inbound) => ' ',
    (Port::None, Port::Unknown, Port::Inbound, Port::Outbound) => ' ',
    (Port::None, Port::Unknown, Port::Inbound, Port::Unknown) => ' ',
    (Port::None, Port::Unknown, Port::Outbound, Port::None) => ' ',
    (Port::None, Port::Unknown, Port::Outbound, Port::Inbound) => ' ',
    (Port::None, Port::Unknown, Port::Outbound, Port::Outbound) => ' ',
    (Port::None, Port::Unknown, Port::Outbound, Port::Unknown) => ' ',
    (Port::None, Port::Unknown, Port::Unknown, Port::None) => ' ',
    (Port::None, Port::Unknown, Port::Unknown, Port::Inbound) => ' ',
    (Port::None, Port::Unknown, Port::Unknown, Port::Outbound) => ' ',
    (Port::None, Port::Unknown, Port::Unknown, Port::Unknown) => ' ',
    (Port::Inbound, Port::None, Port::None, Port::None) => ' ',
    (Port::Inbound, Port::None, Port::None, Port::Inbound) => ' ',
    (Port::Inbound, Port::None, Port::None, Port::Outbound) => ' ',
    (Port::Inbound, Port::None, Port::None, Port::Unknown) => ' ',
    (Port::Inbound, Port::None, Port::Inbound, Port::None) => ' ',
    (Port::Inbound, Port::None, Port::Inbound, Port::Inbound) => ' ',
    (Port::Inbound, Port::None, Port::Inbound, Port::Outbound) => ' ',
    (Port::Inbound, Port::None, Port::Inbound, Port::Unknown) => ' ',
    (Port::Inbound, Port::None, Port::Outbound, Port::None) => ' ',
    (Port::Inbound, Port::None, Port::Outbound, Port::Inbound) => ' ',
    (Port::Inbound, Port::None, Port::Outbound, Port::Outbound) => ' ',
    (Port::Inbound, Port::None, Port::Outbound, Port::Unknown) => ' ',
    (Port::Inbound, Port::None, Port::Unknown, Port::None) => ' ',
    (Port::Inbound, Port::None, Port::Unknown, Port::Inbound) => ' ',
    (Port::Inbound, Port::None, Port::Unknown, Port::Outbound) => ' ',
    (Port::Inbound, Port::None, Port::Unknown, Port::Unknown) => ' ',
    (Port::Inbound, Port::Inbound, Port::None, Port::None) => ' ',
    (Port::Inbound, Port::Inbound, Port::None, Port::Inbound) => ' ',
    (Port::Inbound, Port::Inbound, Port::None, Port::Outbound) => ' ',
    (Port::Inbound, Port::Inbound, Port::None, Port::Unknown) => ' ',
    (Port::Inbound, Port::Inbound, Port::Inbound, Port::None) => ' ',
    (Port::Inbound, Port::Inbound, Port::Inbound, Port::Inbound) => ' ',
    (Port::Inbound, Port::Inbound, Port::Inbound, Port::Outbound) => ' ',
    (Port::Inbound, Port::Inbound, Port::Inbound, Port::Unknown) => ' ',
    (Port::Inbound, Port::Inbound, Port::Outbound, Port::None) => ' ',
    (Port::Inbound, Port::Inbound, Port::Outbound, Port::Inbound) => ' ',
    (Port::Inbound, Port::Inbound, Port::Outbound, Port::Outbound) => ' ',
    (Port::Inbound, Port::Inbound, Port::Outbound, Port::Unknown) => ' ',
    (Port::Inbound, Port::Inbound, Port::Unknown, Port::None) => ' ',
    (Port::Inbound, Port::Inbound, Port::Unknown, Port::Inbound) => ' ',
    (Port::Inbound, Port::Inbound, Port::Unknown, Port::Outbound) => ' ',
    (Port::Inbound, Port::Inbound, Port::Unknown, Port::Unknown) => ' ',
    (Port::Inbound, Port::Outbound, Port::None, Port::None) => ' ',
    (Port::Inbound, Port::Outbound, Port::None, Port::Inbound) => ' ',
    (Port::Inbound, Port::Outbound, Port::None, Port::Outbound) => ' ',
    (Port::Inbound, Port::Outbound, Port::None, Port::Unknown) => ' ',
    (Port::Inbound, Port::Outbound, Port::Inbound, Port::None) => ' ',
    (Port::Inbound, Port::Outbound, Port::Inbound, Port::Inbound) => ' ',
    (Port::Inbound, Port::Outbound, Port::Inbound, Port::Outbound) => ' ',
    (Port::Inbound, Port::Outbound, Port::Inbound, Port::Unknown) => ' ',
    (Port::Inbound, Port::Outbound, Port::Outbound, Port::None) => ' ',
    (Port::Inbound, Port::Outbound, Port::Outbound, Port::Inbound) => ' ',
    (Port::Inbound, Port::Outbound, Port::Outbound, Port::Outbound) => ' ',
    (Port::Inbound, Port::Outbound, Port::Outbound, Port::Unknown) => ' ',
    (Port::Inbound, Port::Outbound, Port::Unknown, Port::None) => ' ',
    (Port::Inbound, Port::Outbound, Port::Unknown, Port::Inbound) => ' ',
    (Port::Inbound, Port::Outbound, Port::Unknown, Port::Outbound) => ' ',
    (Port::Inbound, Port::Outbound, Port::Unknown, Port::Unknown) => ' ',
    (Port::Inbound, Port::Unknown, Port::None, Port::None) => ' ',
    (Port::Inbound, Port::Unknown, Port::None, Port::Inbound) => ' ',
    (Port::Inbound, Port::Unknown, Port::None, Port::Outbound) => ' ',
    (Port::Inbound, Port::Unknown, Port::None, Port::Unknown) => ' ',
    (Port::Inbound, Port::Unknown, Port::Inbound, Port::None) => ' ',
    (Port::Inbound, Port::Unknown, Port::Inbound, Port::Inbound) => ' ',
    (Port::Inbound, Port::Unknown, Port::Inbound, Port::Outbound) => ' ',
    (Port::Inbound, Port::Unknown, Port::Inbound, Port::Unknown) => ' ',
    (Port::Inbound, Port::Unknown, Port::Outbound, Port::None) => ' ',
    (Port::Inbound, Port::Unknown, Port::Outbound, Port::Inbound) => ' ',
    (Port::Inbound, Port::Unknown, Port::Outbound, Port::Outbound) => ' ',
    (Port::Inbound, Port::Unknown, Port::Outbound, Port::Unknown) => ' ',
    (Port::Inbound, Port::Unknown, Port::Unknown, Port::None) => ' ',
    (Port::Inbound, Port::Unknown, Port::Unknown, Port::Inbound) => ' ',
    (Port::Inbound, Port::Unknown, Port::Unknown, Port::Outbound) => ' ',
    (Port::Inbound, Port::Unknown, Port::Unknown, Port::Unknown) => ' ',
    (Port::Outbound, Port::None, Port::None, Port::None) => ' ',
    (Port::Outbound, Port::None, Port::None, Port::Inbound) => ' ',
    (Port::Outbound, Port::None, Port::None, Port::Outbound) => ' ',
    (Port::Outbound, Port::None, Port::None, Port::Unknown) => ' ',
    (Port::Outbound, Port::None, Port::Inbound, Port::None) => ' ',
    (Port::Outbound, Port::None, Port::Inbound, Port::Inbound) => ' ',
    (Port::Outbound, Port::None, Port::Inbound, Port::Outbound) => ' ',
    (Port::Outbound, Port::None, Port::Inbound, Port::Unknown) => ' ',
    (Port::Outbound, Port::None, Port::Outbound, Port::None) => ' ',
    (Port::Outbound, Port::None, Port::Outbound, Port::Inbound) => ' ',
    (Port::Outbound, Port::None, Port::Outbound, Port::Outbound) => ' ',
    (Port::Outbound, Port::None, Port::Outbound, Port::Unknown) => ' ',
    (Port::Outbound, Port::None, Port::Unknown, Port::None) => ' ',
    (Port::Outbound, Port::None, Port::Unknown, Port::Inbound) => ' ',
    (Port::Outbound, Port::None, Port::Unknown, Port::Outbound) => ' ',
    (Port::Outbound, Port::None, Port::Unknown, Port::Unknown) => ' ',
    (Port::Outbound, Port::Inbound, Port::None, Port::None) => ' ',
    (Port::Outbound, Port::Inbound, Port::None, Port::Inbound) => ' ',
    (Port::Outbound, Port::Inbound, Port::None, Port::Outbound) => ' ',
    (Port::Outbound, Port::Inbound, Port::None, Port::Unknown) => ' ',
    (Port::Outbound, Port::Inbound, Port::Inbound, Port::None) => ' ',
    (Port::Outbound, Port::Inbound, Port::Inbound, Port::Inbound) => ' ',
    (Port::Outbound, Port::Inbound, Port::Inbound, Port::Outbound) => ' ',
    (Port::Outbound, Port::Inbound, Port::Inbound, Port::Unknown) => ' ',
    (Port::Outbound, Port::Inbound, Port::Outbound, Port::None) => ' ',
    (Port::Outbound, Port::Inbound, Port::Outbound, Port::Inbound) => ' ',
    (Port::Outbound, Port::Inbound, Port::Outbound, Port::Outbound) => ' ',
    (Port::Outbound, Port::Inbound, Port::Outbound, Port::Unknown) => ' ',
    (Port::Outbound, Port::Inbound, Port::Unknown, Port::None) => ' ',
    (Port::Outbound, Port::Inbound, Port::Unknown, Port::Inbound) => ' ',
    (Port::Outbound, Port::Inbound, Port::Unknown, Port::Outbound) => ' ',
    (Port::Outbound, Port::Inbound, Port::Unknown, Port::Unknown) => ' ',
    (Port::Outbound, Port::Outbound, Port::None, Port::None) => ' ',
    (Port::Outbound, Port::Outbound, Port::None, Port::Inbound) => ' ',
    (Port::Outbound, Port::Outbound, Port::None, Port::Outbound) => ' ',
    (Port::Outbound, Port::Outbound, Port::None, Port::Unknown) => ' ',
    (Port::Outbound, Port::Outbound, Port::Inbound, Port::None) => ' ',
    (Port::Outbound, Port::Outbound, Port::Inbound, Port::Inbound) => ' ',
    (Port::Outbound, Port::Outbound, Port::Inbound, Port::Outbound) => ' ',
    (Port::Outbound, Port::Outbound, Port::Inbound, Port::Unknown) => ' ',
    (Port::Outbound, Port::Outbound, Port::Outbound, Port::None) => ' ',
    (Port::Outbound, Port::Outbound, Port::Outbound, Port::Inbound) => ' ',
    (Port::Outbound, Port::Outbound, Port::Outbound, Port::Outbound) => ' ',
    (Port::Outbound, Port::Outbound, Port::Outbound, Port::Unknown) => ' ',
    (Port::Outbound, Port::Outbound, Port::Unknown, Port::None) => ' ',
    (Port::Outbound, Port::Outbound, Port::Unknown, Port::Inbound) => ' ',
    (Port::Outbound, Port::Outbound, Port::Unknown, Port::Outbound) => ' ',
    (Port::Outbound, Port::Outbound, Port::Unknown, Port::Unknown) => ' ',
    (Port::Outbound, Port::Unknown, Port::None, Port::None) => ' ',
    (Port::Outbound, Port::Unknown, Port::None, Port::Inbound) => ' ',
    (Port::Outbound, Port::Unknown, Port::None, Port::Outbound) => ' ',
    (Port::Outbound, Port::Unknown, Port::None, Port::Unknown) => ' ',
    (Port::Outbound, Port::Unknown, Port::Inbound, Port::None) => ' ',
    (Port::Outbound, Port::Unknown, Port::Inbound, Port::Inbound) => ' ',
    (Port::Outbound, Port::Unknown, Port::Inbound, Port::Outbound) => ' ',
    (Port::Outbound, Port::Unknown, Port::Inbound, Port::Unknown) => ' ',
    (Port::Outbound, Port::Unknown, Port::Outbound, Port::None) => ' ',
    (Port::Outbound, Port::Unknown, Port::Outbound, Port::Inbound) => ' ',
    (Port::Outbound, Port::Unknown, Port::Outbound, Port::Outbound) => ' ',
    (Port::Outbound, Port::Unknown, Port::Outbound, Port::Unknown) => ' ',
    (Port::Outbound, Port::Unknown, Port::Unknown, Port::None) => ' ',
    (Port::Outbound, Port::Unknown, Port::Unknown, Port::Inbound) => ' ',
    (Port::Outbound, Port::Unknown, Port::Unknown, Port::Outbound) => ' ',
    (Port::Outbound, Port::Unknown, Port::Unknown, Port::Unknown) => ' ',
    (Port::Unknown, Port::None, Port::None, Port::None) => ' ',
    (Port::Unknown, Port::None, Port::None, Port::Inbound) => ' ',
    (Port::Unknown, Port::None, Port::None, Port::Outbound) => ' ',
    (Port::Unknown, Port::None, Port::None, Port::Unknown) => ' ',
    (Port::Unknown, Port::None, Port::Inbound, Port::None) => ' ',
    (Port::Unknown, Port::None, Port::Inbound, Port::Inbound) => ' ',
    (Port::Unknown, Port::None, Port::Inbound, Port::Outbound) => ' ',
    (Port::Unknown, Port::None, Port::Inbound, Port::Unknown) => ' ',
    (Port::Unknown, Port::None, Port::Outbound, Port::None) => ' ',
    (Port::Unknown, Port::None, Port::Outbound, Port::Inbound) => ' ',
    (Port::Unknown, Port::None, Port::Outbound, Port::Outbound) => ' ',
    (Port::Unknown, Port::None, Port::Outbound, Port::Unknown) => ' ',
    (Port::Unknown, Port::None, Port::Unknown, Port::None) => ' ',
    (Port::Unknown, Port::None, Port::Unknown, Port::Inbound) => ' ',
    (Port::Unknown, Port::None, Port::Unknown, Port::Outbound) => ' ',
    (Port::Unknown, Port::None, Port::Unknown, Port::Unknown) => ' ',
    (Port::Unknown, Port::Inbound, Port::None, Port::None) => ' ',
    (Port::Unknown, Port::Inbound, Port::None, Port::Inbound) => ' ',
    (Port::Unknown, Port::Inbound, Port::None, Port::Outbound) => ' ',
    (Port::Unknown, Port::Inbound, Port::None, Port::Unknown) => ' ',
    (Port::Unknown, Port::Inbound, Port::Inbound, Port::None) => ' ',
    (Port::Unknown, Port::Inbound, Port::Inbound, Port::Inbound) => ' ',
    (Port::Unknown, Port::Inbound, Port::Inbound, Port::Outbound) => ' ',
    (Port::Unknown, Port::Inbound, Port::Inbound, Port::Unknown) => ' ',
    (Port::Unknown, Port::Inbound, Port::Outbound, Port::None) => ' ',
    (Port::Unknown, Port::Inbound, Port::Outbound, Port::Inbound) => ' ',
    (Port::Unknown, Port::Inbound, Port::Outbound, Port::Outbound) => ' ',
    (Port::Unknown, Port::Inbound, Port::Outbound, Port::Unknown) => ' ',
    (Port::Unknown, Port::Inbound, Port::Unknown, Port::None) => ' ',
    (Port::Unknown, Port::Inbound, Port::Unknown, Port::Inbound) => ' ',
    (Port::Unknown, Port::Inbound, Port::Unknown, Port::Outbound) => ' ',
    (Port::Unknown, Port::Inbound, Port::Unknown, Port::Unknown) => ' ',
    (Port::Unknown, Port::Outbound, Port::None, Port::None) => ' ',
    (Port::Unknown, Port::Outbound, Port::None, Port::Inbound) => ' ',
    (Port::Unknown, Port::Outbound, Port::None, Port::Outbound) => ' ',
    (Port::Unknown, Port::Outbound, Port::None, Port::Unknown) => ' ',
    (Port::Unknown, Port::Outbound, Port::Inbound, Port::None) => ' ',
    (Port::Unknown, Port::Outbound, Port::Inbound, Port::Inbound) => ' ',
    (Port::Unknown, Port::Outbound, Port::Inbound, Port::Outbound) => ' ',
    (Port::Unknown, Port::Outbound, Port::Inbound, Port::Unknown) => ' ',
    (Port::Unknown, Port::Outbound, Port::Outbound, Port::None) => ' ',
    (Port::Unknown, Port::Outbound, Port::Outbound, Port::Inbound) => ' ',
    (Port::Unknown, Port::Outbound, Port::Outbound, Port::Outbound) => ' ',
    (Port::Unknown, Port::Outbound, Port::Outbound, Port::Unknown) => ' ',
    (Port::Unknown, Port::Outbound, Port::Unknown, Port::None) => ' ',
    (Port::Unknown, Port::Outbound, Port::Unknown, Port::Inbound) => ' ',
    (Port::Unknown, Port::Outbound, Port::Unknown, Port::Outbound) => ' ',
    (Port::Unknown, Port::Outbound, Port::Unknown, Port::Unknown) => ' ',
    (Port::Unknown, Port::Unknown, Port::None, Port::None) => ' ',
    (Port::Unknown, Port::Unknown, Port::None, Port::Inbound) => ' ',
    (Port::Unknown, Port::Unknown, Port::None, Port::Outbound) => ' ',
    (Port::Unknown, Port::Unknown, Port::None, Port::Unknown) => ' ',
    (Port::Unknown, Port::Unknown, Port::Inbound, Port::None) => ' ',
    (Port::Unknown, Port::Unknown, Port::Inbound, Port::Inbound) => ' ',
    (Port::Unknown, Port::Unknown, Port::Inbound, Port::Outbound) => ' ',
    (Port::Unknown, Port::Unknown, Port::Inbound, Port::Unknown) => ' ',
    (Port::Unknown, Port::Unknown, Port::Outbound, Port::None) => ' ',
    (Port::Unknown, Port::Unknown, Port::Outbound, Port::Inbound) => ' ',
    (Port::Unknown, Port::Unknown, Port::Outbound, Port::Outbound) => ' ',
    (Port::Unknown, Port::Unknown, Port::Outbound, Port::Unknown) => ' ',
    (Port::Unknown, Port::Unknown, Port::Unknown, Port::None) => ' ',
    (Port::Unknown, Port::Unknown, Port::Unknown, Port::Inbound) => ' ',
    (Port::Unknown, Port::Unknown, Port::Unknown, Port::Outbound) => ' ',
    (Port::Unknown, Port::Unknown, Port::Unknown, Port::Unknown) => ' ',
  }
}

pub fn belt_type_to_belt_meta(belt_type: BeltType) -> BeltMeta {
  match belt_type {
    BeltType::RU => BELT_RU,
    BeltType::DR => BELT_DR,
    BeltType::DL => BELT_DL,
    BeltType::LU => BELT_LU,
    BeltType::DU => BELT_DU,
    BeltType::LR => BELT_LR,
    BeltType::LRU => BELT_LRU,
    BeltType::DRU => BELT_DRU,
    BeltType::DLR => BELT_DLR,
    BeltType::DLU => BELT_DLU,
    BeltType::DLRU => BELT_DLRU,
    BeltType::UNKNOWN => BELT_UNKNOWN,
    BeltType::INVALID => BELT_INVALID,
    _ => panic!("Only use this for unguided types or code support for the other ones ^^ {:?}", belt_type),
  }
}

pub fn add_one_ports_to_cell(factory: &Factory, coord: usize, dir: Direction) -> BeltType {
  // Given a coord and two dirs return a belt type that has _a_ port in all the directions of:
  // - the given dirs
  // - the non-none ports of the current cell
  // - any dir where the neighbor is a belt (if flag is not set)

  match (
    dir == Direction::Up || factory.floor[coord].port_u != Port::None,
    dir == Direction::Right || factory.floor[coord].port_r != Port::None,
    dir == Direction::Down || factory.floor[coord].port_d != Port::None,
    dir == Direction::Left || factory.floor[coord].port_l != Port::None,
  ) {
    (true, false, false, false) => BeltType::INVALID, // TODO
    (true, true, false, false) => BeltType::RU,
    (true, false, true, false) => BeltType::DU,
    (true, false, false, true) => BeltType::LU,
    (true, true, true, false) => BeltType::DRU,
    (true, true, false, true) => BeltType::LRU,
    (true, false, true, true) => BeltType::DLU,
    (true, true, true, true) => BeltType::DLRU,
    (false, false, false, false) => BeltType::INVALID, // TODO
    (false, true, false, false) => BeltType::INVALID, // TODO
    (false, false, true, false) => BeltType::INVALID, // TODO
    (false, false, false, true) => BeltType::INVALID, // TODO
    (false, true, true, false) => BeltType::DR,
    (false, true, false, true) => BeltType::LR,
    (false, false, true, true) => BeltType::DL,
    (false, true, true, true) => BeltType::DLR,
  }
}

pub fn add_two_ports_to_cell(factory: &Factory, coord: usize, dir1: Direction, dir2: Direction) -> BeltType {
  // Given a coord and two dirs return a belt type that has _a_ port in all the directions of:
  // - the given dirs
  // - the non-none ports of the current cell
  // - any dir where the neighbor is a belt (if flag is not set)

  match (
    dir1 == Direction::Up || dir2 == Direction::Up || factory.floor[coord].port_u != Port::None, // || (!ignore_neighbors && factory.floor[coord].coord_u != None && factory.floor[factory.floor[coord].coord_u.unwrap()].kind == CellKind::Belt),
    dir1 == Direction::Right || dir2 == Direction::Right || factory.floor[coord].port_r != Port::None, // || (!ignore_neighbors && factory.floor[coord].coord_r != None && factory.floor[factory.floor[coord].coord_r.unwrap()].kind == CellKind::Belt),
    dir1 == Direction::Down || dir2 == Direction::Down || factory.floor[coord].port_d != Port::None, // || (!ignore_neighbors && factory.floor[coord].coord_d != None && factory.floor[factory.floor[coord].coord_d.unwrap()].kind == CellKind::Belt),
    dir1 == Direction::Left || dir2 == Direction::Left || factory.floor[coord].port_l != Port::None, // || (!ignore_neighbors && factory.floor[coord].coord_l != None && factory.floor[factory.floor[coord].coord_l.unwrap()].kind == CellKind::Belt),
  ) {
    (true, false, false, false) => BeltType::INVALID, // TODO
    (true, true, false, false) => BeltType::RU,
    (true, false, true, false) => BeltType::DU,
    (true, false, false, true) => BeltType::LU,
    (true, true, true, false) => BeltType::DRU,
    (true, true, false, true) => BeltType::LRU,
    (true, false, true, true) => BeltType::DLU,
    (true, true, true, true) => BeltType::DLRU,
    (false, false, false, false) => BeltType::INVALID, // TODO
    (false, true, false, false) => BeltType::INVALID, // TODO
    (false, false, true, false) => BeltType::INVALID, // TODO
    (false, false, false, true) => BeltType::INVALID, // TODO
    (false, true, true, false) => BeltType::DR,
    (false, true, false, true) => BeltType::LR,
    (false, false, true, true) => BeltType::DL,
    (false, true, true, true) => BeltType::DLR,
  }
}
pub fn get_belt_type_for_cell_ports(factory: &Factory, coord: usize) -> BeltType {
  match (
    factory.floor[coord].port_u != Port::None,
    factory.floor[coord].port_r != Port::None,
    factory.floor[coord].port_d != Port::None,
    factory.floor[coord].port_l != Port::None,
  ) {
    (true, false, false, false) => BeltType::INVALID, // TODO
    (true, true, false, false) => BeltType::RU,
    (true, false, true, false) => BeltType::DU,
    (true, false, false, true) => BeltType::LU,
    (true, true, true, false) => BeltType::DRU,
    (true, true, false, true) => BeltType::LRU,
    (true, false, true, true) => BeltType::DLU,
    (true, true, true, true) => BeltType::DLRU,
    (false, false, false, false) => BeltType::INVALID, // TODO
    (false, true, false, false) => BeltType::INVALID, // TODO
    (false, false, true, false) => BeltType::INVALID, // TODO
    (false, false, false, true) => BeltType::INVALID, // TODO
    (false, true, true, false) => BeltType::DR,
    (false, true, false, true) => BeltType::LR,
    (false, false, true, true) => BeltType::DL,
    (false, true, true, true) => BeltType::DLR,
  }
}

pub fn belt_discover_ins_and_outs(factory: &mut Factory, coord: usize) {
  // log(format!("belt_discover_ins_and_outs({}) {:?} {:?} {:?} {:?}", coord, factory.floor[coord].port_u, factory.floor[coord].port_r, factory.floor[coord].port_d, factory.floor[coord].port_l));

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

pub fn connect_to_neighbor_dead_end_belts(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, coord: usize) {
  // log(format!("connect_to_neighbor_dead_end_belts({})", coord));

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

const BOX_ARROW_U: char = '^';
const BOX_ARROW_R: char = '>';
const BOX_ARROW_D: char = 'v';
const BOX_ARROW_L: char = '<';
const BOX_LU: char = '┌';
const BOX_U: char =  '─';
const BOX_RU: char = '┐';
const BOX_L: char =  '│';
const BOX_R: char =  '│';
const BOX_DL: char = '└';
const BOX_D: char =  '─';
const BLX_DR: char = '┘';
const BOX_EQ_V: char = '║';
const BOX_EQ_H: char = '═';
// const BOX_SEG_U: char = '│';
// const BOX_SEG_R: char = '─';
// const BOX_SEG_D: char = '│';
// const BOX_SEG_L: char = '─';
// const BOX_SEG_C_DL: char = '┐';
// const BOX_SEG_C_DLR: char = '┬';
// const BOX_SEG_C_DLRU: char = '┼';
// const BOX_SEG_C_DLU: char = '┤';
// const BOX_SEG_C_DRU: char = '├';
// const BOX_SEG_C_DR: char = '┌';
// const BOX_SEG_C_DU: char = '│';
// const BOX_SEG_C_LR: char = '─';
// const BOX_SEG_C_LRU: char = '┴';
// const BOX_SEG_C_LU: char = '┘';
// const BOX_SEG_C_RU: char = '└';

// ┌─┐
// │ │
// └─┘
pub const BELT_NONE: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "BELT_NONE",
  src: "./img/none.png",
  port_u: Port::None,
  port_r: Port::None,
  port_d: Port::None,
  port_l: Port::None,
  cli_icon: ' ',
};

// o
// ┌─┐
// │ │
// └─┘
pub const BELT_INVALID: BeltMeta = BeltMeta {
  btype: BeltType::INVALID,
  dbg: "BELT_INVALID",
  src: "./img/invalid.png",
  port_u: Port::None,
  port_r: Port::None,
  port_d: Port::None,
  port_l: Port::None,
  cli_icon: '!',
};
// ┌║┐
// ═ ═
// └║┘
pub const BELT_UNKNOWN: BeltMeta = BeltMeta {
  btype: BeltType::INVALID,
  dbg: "BELT_UNKNOWN",
  src: "./img/invalid.png",
  port_u: Port::Unknown,
  port_r: Port::Unknown,
  port_d: Port::Unknown,
  port_l: Port::Unknown,
  cli_icon: '?',
};
// ┌║┐
// ═ ═
// └║┘
pub const MACHINE: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_MACHINE",
  src: "./img/todo.png",
  port_u: Port::None,
  port_r: Port::None,
  port_d: Port::None,
  port_l: Port::None,
  cli_icon: 'm',
};
// ┌─┐
// │s│
// └v┘
pub const SUPPLY_U: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_SUPPLY_U",
  src: "./img/todo.png",
  port_u: Port::None,
  port_r: Port::None,
  port_d: Port::Outbound,
  port_l: Port::None,
  cli_icon: 's',
};
// ┌─┐
// <s│
// └─┘
pub const SUPPLY_R: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_SUPPLY_R",
  src: "./img/todo.png",
  port_u: Port::None,
  port_r: Port::None,
  port_d: Port::None,
  port_l: Port::Outbound,
  cli_icon: 's',
};
// ┌^┐
// │s│
// └─┘
pub const SUPPLY_D: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_SUPPLY_D",
  src: "./img/todo.png",
  port_u: Port::Outbound,
  port_r: Port::None,
  port_d: Port::None,
  port_l: Port::None,
  cli_icon: 's',
};
// ┌─┐
// │s>
// └─┘
pub const SUPPLY_L: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_SUPPLY_L",
  src: "./img/todo.png",
  port_u: Port::None,
  port_r: Port::Outbound,
  port_d: Port::None,
  port_l: Port::None,
  cli_icon: 's',
};
// ┌─┐
// │d│
// └^┘
pub const DEMAND_U: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_DEMAND_U",
  src: "./img/todo.png",
  port_u: Port::Inbound,
  port_r: Port::None,
  port_d: Port::None,
  port_l: Port::None,
  cli_icon: 'd',
};
// ┌─┐
// >d│
// └─┘
pub const DEMAND_R: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_DEMAND_R",
  src: "./img/todo.png",
  port_u: Port::None,
  port_r: Port::Inbound,
  port_d: Port::None,
  port_l: Port::None,
  cli_icon: 'd',
};
// ┌v┐
// │d│
// └─┘
pub const DEMAND_D: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_DEMAND_D",
  src: "./img/todo.png",
  port_u: Port::None,
  port_r: Port::None,
  port_d: Port::Inbound,
  port_l: Port::None,
  cli_icon: 'd',
};
// ┌─┐
// │d<
// └─┘
pub const DEMAND_L: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_DEMAND_L",
  src: "./img/todo.png",
  port_u: Port::None,
  port_r: Port::None,
  port_d: Port::None,
  port_l: Port::Inbound,
  cli_icon: 'd',
};
// ┌?┐
// │ ?
// └─┘
pub const BELT_RU: BeltMeta = BeltMeta {
  btype: BeltType::RU,
  dbg: "BELT_RU",
  src: "./img/ru.png",
  port_u: Port::Unknown,
  port_r: Port::Unknown,
  port_d: Port::None,
  port_l: Port::None,
  cli_icon: '╚',
};
// // ┌v┐
// // │ >
// // └─┘
// pub const BELT_U_R: BeltMeta = BeltMeta {
//   btype: BeltType::U_R,
//   dbg: "BELT_U_R",
//   src: "./img/u_r.png",
//   port_u: Port::Inbound,
//   port_r: Port::Outbound,
//   port_d: Port::None,
//   port_l: Port::None,
//   cli_icon: '╚',
// };
// // ┌^┐
// // │ <
// // └─┘
// pub const BELT_R_U: BeltMeta = BeltMeta {
//   btype: BeltType::R_U,
//   dbg: "BELT_R_U",
//   src: "./img/r_u.png",
//   port_u: Port::Outbound,
//   port_r: Port::Inbound,
//   port_d: Port::None,
//   port_l: Port::None,
//   cli_icon: '╚',
// };
// ┌─┐
// │ ?
// └?┘
pub const BELT_DR: BeltMeta = BeltMeta {
  btype: BeltType::DR,
  dbg: "BELT_DR",
  src: "./img/dr.png",
  port_u: Port::None,
  port_r: Port::Unknown,
  port_d: Port::Unknown,
  port_l: Port::None,
  cli_icon: '╔',
};
// // ┌─┐
// // │ <
// // └v┘
// pub const BELT_R_D: BeltMeta = BeltMeta {
//   btype: BeltType::R_D,
//   dbg: "BELT_R_D",
//   src: "./img/r_d.png",
//   port_u: Port::None,
//   port_r: Port::Inbound,
//   port_d: Port::Outbound,
//   port_l: Port::None,
//   cli_icon: '╔',
// };
// // ┌─┐
// // │ >
// // └^┘
// pub const BELT_D_R: BeltMeta = BeltMeta {
//   btype: BeltType::D_R,
//   dbg: "BELT_D_R",
//   src: "./img/d_r.png",
//   port_u: Port::None,
//   port_r: Port::Outbound,
//   port_d: Port::Inbound,
//   port_l: Port::None,
//   cli_icon: '╔',
// };
// ┌─┐
// ? │
// └?┘
pub const BELT_DL: BeltMeta = BeltMeta {
  btype: BeltType::DL,
  dbg: "BELT_DL",
  src: "./img/dl.png",
  port_u: Port::None,
  port_r: Port::None,
  port_d: Port::Unknown,
  port_l: Port::Unknown,
  cli_icon: '╗',
};
// // ┌─┐
// // < │
// // └^┘
// pub const BELT_D_L: BeltMeta = BeltMeta {
//   btype: BeltType::D_L,
//   dbg: "BELT_D_L",
//   src: "./img/d_l.png",
//   port_u: Port::None,
//   port_r: Port::None,
//   port_d: Port::Inbound,
//   port_l: Port::Outbound,
//   cli_icon: '╗',
// };
// // ┌─┐
// // > │
// // └v┘
// pub const BELT_L_D: BeltMeta = BeltMeta {
//   btype: BeltType::L_D,
//   dbg: "BELT_L_D",
//   src: "./img/l_d.png",
//   port_u: Port::None,
//   port_r: Port::None,
//   port_d: Port::Outbound,
//   port_l: Port::Inbound,
//   cli_icon: '╗',
// };
// ┌?┐
// ? │
// └─┘
pub const BELT_LU: BeltMeta = BeltMeta {
  btype: BeltType::LU,
  dbg: "BELT_LU",
  src: "./img/lu.png",
  port_u: Port::Unknown,
  port_r: Port::None,
  port_d: Port::None,
  port_l: Port::Unknown,
  cli_icon: '╝',
};
// // ┌v┐
// // < │
// // └─┘
// pub const BELT_L_U: BeltMeta = BeltMeta {
//   btype: BeltType::L_U,
//   dbg: "BELT_L_U",
//   src: "./img/l_u.png",
//   port_u: Port::Outbound,
//   port_r: Port::None,
//   port_d: Port::None,
//   port_l: Port::Inbound,
//   cli_icon: '╝',
// };
// // ┌v┐
// // < │
// // └─┘
// pub const BELT_U_L: BeltMeta = BeltMeta {
//   btype: BeltType::U_L,
//   dbg: "BELT_U_L",
//   src: "./img/u_l.png",
//   port_u: Port::Inbound,
//   port_r: Port::None,
//   port_d: Port::None,
//   port_l: Port::Outbound,
//   cli_icon: '╝',
// };
// ┌?┐
// │ │
// └?┘
pub const BELT_DU: BeltMeta = BeltMeta {
  btype: BeltType::DU,
  dbg: "BELT_DU",
  src: "./img/du.png",
  port_u: Port::Unknown,
  port_r: Port::None,
  port_d: Port::Unknown,
  port_l: Port::None,
  cli_icon: '║',
};
// // ┌v┐
// // │ │
// // └v┘
// pub const BELT_U_D: BeltMeta = BeltMeta {
//   btype: BeltType::U_D,
//   dbg: "BELT_U_D",
//   src: "./img/u_d.png",
//   port_u: Port::Inbound,
//   port_r: Port::None,
//   port_d: Port::Outbound,
//   port_l: Port::None,
//   cli_icon: '║',
// };
// // ┌^┐
// // │ │
// // └^┘
// pub const BELT_D_U: BeltMeta = BeltMeta {
//   btype: BeltType::D_U,
//   dbg: "BELT_D_U",
//   src: "./img/d_u.png",
//   port_u: Port::Outbound,
//   port_r: Port::None,
//   port_d: Port::Inbound,
//   port_l: Port::None,
//   cli_icon: '║',
// };
// ┌─┐
// ? ?
// └─┘
pub const BELT_LR: BeltMeta = BeltMeta {
  btype: BeltType::LR,
  dbg: "BELT_LR",
  src: "./img/lr.png",
  port_u: Port::None,
  port_r: Port::Unknown,
  port_d: Port::None,
  port_l: Port::Unknown,
  cli_icon: '═',
};
// // ┌─┐
// // > >
// // └─┘
// pub const BELT_L_R: BeltMeta = BeltMeta {
//   btype: BeltType::L_R,
//   dbg: "BELT_L_R",
//   src: "./img/l_r.png",
//   port_u: Port::None,
//   port_r: Port::Outbound,
//   port_d: Port::None,
//   port_l: Port::Inbound,
//   cli_icon: '═',
// };
// // ┌─┐
// // < <
// // └─┘
// pub const BELT_R_L: BeltMeta = BeltMeta {
//   btype: BeltType::R_L,
//   dbg: "BELT_R_L",
//   src: "./img/r_l.png",
//   port_u: Port::None,
//   port_r: Port::Inbound,
//   port_d: Port::None,
//   port_l: Port::Outbound,
//   cli_icon: '═',
// };
// ┌?┐
// ? ?
// └─┘
pub const BELT_LRU: BeltMeta = BeltMeta {
  btype: BeltType::LRU,
  dbg: "BELT_LRU",
  src: "./img/lru.png",
  port_u: Port::Unknown,
  port_r: Port::Unknown,
  port_d: Port::None,
  port_l: Port::Unknown,
  cli_icon: '╩',
};
// // ┌v┐
// // < >
// // └─┘
// pub const BELT_U_LR: BeltMeta = BeltMeta {
//   btype: BeltType::U_LR,
//   dbg: "BELT_U_LR",
//   src: "./img/u_lr.png",
//   port_u: Port::Inbound,
//   port_r: Port::Outbound,
//   port_d: Port::None,
//   port_l: Port::Outbound,
//   cli_icon: '╩',
// };
// // ┌v┐
// // < <
// // └─┘
// pub const BELT_RU_L: BeltMeta = BeltMeta {
//   btype: BeltType::RU_L,
//   dbg: "BELT_RU_L",
//   src: "./img/ru_l.png",
//   port_u: Port::Inbound,
//   port_r: Port::Inbound,
//   port_d: Port::None,
//   port_l: Port::Outbound,
//   cli_icon: '╩',
// };
// // ┌v┐
// // > >
// // └─┘
// pub const BELT_LU_R: BeltMeta = BeltMeta {
//   btype: BeltType::LU_R,
//   dbg: "BELT_LU_R",
//   src: "./img/lu_r.png",
//   port_u: Port::Inbound,
//   port_r: Port::Outbound,
//   port_d: Port::None,
//   port_l: Port::Inbound,
//   cli_icon: '╩',
// };
// // ┌^┐
// // > >
// // └─┘
// pub const BELT_L_RU: BeltMeta = BeltMeta {
//   btype: BeltType::L_RU,
//   dbg: "BELT_L_RU",
//   src: "./img/l_ru.png",
//   port_u: Port::Outbound,
//   port_r: Port::Outbound,
//   port_d: Port::None,
//   port_l: Port::Inbound,
//   cli_icon: '╩',
// };
// // ┌^┐
// // > <
// // └─┘
// pub const BELT_LR_U: BeltMeta = BeltMeta {
//   btype: BeltType::LR_U,
//   dbg: "BELT_LR_U",
//   src: "./img/lr_u.png",
//   port_u: Port::Outbound,
//   port_r: Port::Inbound,
//   port_d: Port::None,
//   port_l: Port::Inbound,
//   cli_icon: '╩',
// };
// // ┌^┐
// // < <
// // └─┘
// pub const BELT_R_LU: BeltMeta = BeltMeta {
//   btype: BeltType::R_LU,
//   dbg: "BELT_R_LU",
//   src: "./img/r_lu.png",
//   port_u: Port::Outbound,
//   port_r: Port::Inbound,
//   port_d: Port::None,
//   port_l: Port::Outbound,
//   cli_icon: '╩',
// };
// ┌?┐
// │ ?
// └?┘
pub const BELT_DRU: BeltMeta = BeltMeta {
  btype: BeltType::DRU,
  dbg: "BELT_DRU",
  src: "./img/dru.png",
  port_u: Port::Unknown,
  port_r: Port::Unknown,
  port_d: Port::Unknown,
  port_l: Port::None,
  cli_icon: '╠',
};
// // ┌^┐
// // │ <
// // └v┘
// pub const BELT_R_DU: BeltMeta = BeltMeta {
//   btype: BeltType::R_DU,
//   dbg: "BELT_R_DU",
//   src: "./img/r_du.png",
//   port_u: Port::Outbound,
//   port_r: Port::Inbound,
//   port_d: Port::Outbound,
//   port_l: Port::None,
//   cli_icon: '╠',
// };
// // ┌v┐
// // │ <
// // └v┘
// pub const BELT_RU_D: BeltMeta = BeltMeta {
//   btype: BeltType::RU_D,
//   dbg: "BELT_RU_D",
//   src: "./img/ru_d.png",
//   port_u: Port::Inbound,
//   port_r: Port::Inbound,
//   port_d: Port::Outbound,
//   port_l: Port::None,
//   cli_icon: '╠',
// };
// // ┌^┐
// // │ <
// // └^┘
// pub const BELT_DR_U: BeltMeta = BeltMeta {
//   btype: BeltType::DR_U,
//   dbg: "BELT_DR_U",
//   src: "./img/dr_u.png",
//   port_u: Port::None,
//   port_r: Port::Inbound,
//   port_d: Port::Inbound,
//   port_l: Port::Outbound,
//   cli_icon: '╠',
// };
// // ┌v┐
// // │ >
// // └^┘
// pub const BELT_DU_R: BeltMeta = BeltMeta {
//   btype: BeltType::DU_R,
//   dbg: "BELT_DU_R",
//   src: "./img/du_r.png",
//   port_u: Port::Inbound,
//   port_r: Port::Outbound,
//   port_d: Port::Inbound,
//   port_l: Port::None,
//   cli_icon: '╠',
// };
// // ┌v┐
// // │ >
// // └v┘
// pub const BELT_U_DR: BeltMeta = BeltMeta {
//   btype: BeltType::U_DR,
//   dbg: "BELT_U_DR",
//   src: "./img/u_dr.png",
//   port_u: Port::Inbound,
//   port_r: Port::Outbound,
//   port_d: Port::Outbound,
//   port_l: Port::None,
//   cli_icon: '╠',
// };
// // ┌^┐
// // │ >
// // └^┘
// pub const BELT_D_RU: BeltMeta = BeltMeta {
//   btype: BeltType::D_RU,
//   dbg: "BELT_D_RU",
//   src: "./img/d_ru.png",
//   port_u: Port::Outbound,
//   port_r: Port::Outbound,
//   port_d: Port::Inbound,
//   port_l: Port::None,
//   cli_icon: '╠',
// };
// ┌─┐
// ? ?
// └?┘
pub const BELT_DLR: BeltMeta = BeltMeta {
  btype: BeltType::DLR,
  dbg: "BELT_DLR",
  src: "./img/dlr.png",
  port_u: Port::None,
  port_r: Port::Unknown,
  port_d: Port::Unknown,
  port_l: Port::Unknown,
  cli_icon: '╦',
};
// // ┌─┐
// // < >
// // └^┘
// pub const BELT_D_LR: BeltMeta = BeltMeta {
//   btype: BeltType::D_LR,
//   dbg: "BELT_D_LR",
//   src: "./img/d_lr.png",
//   port_u: Port::None,
//   port_r: Port::Outbound,
//   port_d: Port::Inbound,
//   port_l: Port::Outbound,
//   cli_icon: '╦',
// };
// // ┌─┐
// // > >
// // └^┘
// pub const BELT_DL_R: BeltMeta = BeltMeta {
//   btype: BeltType::DL_R,
//   dbg: "BELT_DL_R",
//   src: "./img/dl_r.png",
//   port_u: Port::None,
//   port_r: Port::Outbound,
//   port_d: Port::Inbound,
//   port_l: Port::Inbound,
//   cli_icon: '╦',
// };
// // ┌─┐
// // < <
// // └^┘
// pub const BELT_DR_L: BeltMeta = BeltMeta {
//   btype: BeltType::DR_L,
//   dbg: "BELT_DR_L",
//   src: "./img/dr_l.png",
//   port_u: Port::None,
//   port_r: Port::Inbound,
//   port_d: Port::Inbound,
//   port_l: Port::Outbound,
//   cli_icon: '╦',
// };
// // ┌─┐
// // > <
// // └v┘
// pub const BELT_LR_D: BeltMeta = BeltMeta {
//   btype: BeltType::LR_D,
//   dbg: "BELT_LR_D",
//   src: "./img/dr_l.png",
//   port_u: Port::None,
//   port_r: Port::Inbound,
//   port_d: Port::Outbound,
//   port_l: Port::Inbound,
//   cli_icon: '╦',
// };
// // ┌─┐
// // > >
// // └v┘
// pub const BELT_L_DR: BeltMeta = BeltMeta {
//   btype: BeltType::L_DR,
//   dbg: "BELT_L_DR",
//   src: "./img/dr_l.png",
//   port_u: Port::None,
//   port_r: Port::Outbound,
//   port_d: Port::Outbound,
//   port_l: Port::Inbound,
//   cli_icon: '╦',
// };
// // ┌─┐
// // < <
// // └v┘
// pub const BELT_R_DL: BeltMeta = BeltMeta {
//   btype: BeltType::R_DL,
//   dbg: "BELT_R_DL",
//   src: "./img/r_dl.png",
//   port_u: Port::None,
//   port_r: Port::Inbound,
//   port_d: Port::Outbound,
//   port_l: Port::Outbound,
//   cli_icon: '╦',
// };
// ┌?┐
// ? │
// └?┘
pub const BELT_DLU: BeltMeta = BeltMeta {
  btype: BeltType::DLU,
  dbg: "BELT_DLU",
  src: "./img/dlu.png",
  port_u: Port::Unknown,
  port_r: Port::None,
  port_d: Port::Unknown,
  port_l: Port::Unknown,
  cli_icon: '╣',
};
// // ┌^┐
// // > │
// // └v┘
// pub const BELT_L_DU: BeltMeta = BeltMeta {
//   btype: BeltType::L_DU,
//   dbg: "BELT_L_DU",
//   src: "./img/l_du.png",
//   port_u: Port::Outbound,
//   port_r: Port::None,
//   port_d: Port::Outbound,
//   port_l: Port::Inbound,
//   cli_icon: '╣',
// };
// // ┌v┐
// // > │
// // └v┘
// pub const BELT_LU_D: BeltMeta = BeltMeta {
//   btype: BeltType::LU_D,
//   dbg: "BELT_LU_D",
//   src: "./img/lu_d.png",
//   port_u: Port::Inbound,
//   port_r: Port::None,
//   port_d: Port::Outbound,
//   port_l: Port::Inbound,
//   cli_icon: '╣',
// };
// // ┌^┐
// // > │
// // └^┘
// pub const BELT_DL_U: BeltMeta = BeltMeta {
//   btype: BeltType::DL_U,
//   dbg: "BELT_DL_U",
//   src: "./img/dl_u.png",
//   port_u: Port::Outbound,
//   port_r: Port::None,
//   port_d: Port::Inbound,
//   port_l: Port::Inbound,
//   cli_icon: '╣',
// };
// // ┌v┐
// // < │
// // └^┘
// pub const BELT_DU_L: BeltMeta = BeltMeta {
//   btype: BeltType::DU_L,
//   dbg: "BELT_DU_L",
//   src: "./img/du_l.png",
//   port_u: Port::Inbound,
//   port_r: Port::None,
//   port_d: Port::Inbound,
//   port_l: Port::Outbound,
//   cli_icon: '╣',
// };
// // ┌v┐
// // < │
// // └v┘
// pub const BELT_U_DL: BeltMeta = BeltMeta {
//   btype: BeltType::U_DL,
//   dbg: "BELT_U_DL",
//   src: "./img/u_dl.png",
//   port_u: Port::Inbound,
//   port_r: Port::None,
//   port_d: Port::Outbound,
//   port_l: Port::Outbound,
//   cli_icon: '╣',
// };
// // ┌^┐
// // < │
// // └^┘
// pub const BELT_D_LU: BeltMeta = BeltMeta {
//   btype: BeltType::D_UL,
//   dbg: "BELT_D_UL",
//   src: "./img/d_ul.png",
//   port_u: Port::Outbound,
//   port_r: Port::None,
//   port_d: Port::Inbound,
//   port_l: Port::Outbound,
//   cli_icon: '╣',
// };
// ┌?┐
// ? ?
// └?┘
pub const BELT_DLRU: BeltMeta = BeltMeta {
  btype: BeltType::DLRU,
  dbg: "BELT_DLRU",
  src: "./img/dlru.png",
  port_u: Port::Unknown,
  port_r: Port::Unknown,
  port_d: Port::Unknown,
  port_l: Port::Unknown,
  cli_icon: '╬',
};
// // ┌v┐
// // < >
// // └v┘
// pub const BELT_U_DLR: BeltMeta = BeltMeta {
//   btype: BeltType::U_DLR,
//   dbg: "BELT_U_DLR",
//   src: "./img/dlru.png",
//   port_u: Port::Inbound,
//   port_r: Port::Outbound,
//   port_d: Port::Outbound,
//   port_l: Port::Outbound,
//   cli_icon: '╬',
// };
// // ┌^┐
// // < <
// // └v┘
// pub const BELT_R_DLU: BeltMeta = BeltMeta {
//   btype: BeltType::R_DLU,
//   dbg: "BELT_R_DLU",
//   src: "./img/todo.png",
//   port_u: Port::Outbound,
//   port_r: Port::Inbound,
//   port_d: Port::Outbound,
//   port_l: Port::Outbound,
//   cli_icon: '╬',
// };
// // ┌^┐
// // < >
// // └^┘
// pub const BELT_D_LRU: BeltMeta = BeltMeta {
//   btype: BeltType::D_LRU,
//   dbg: "BELT_D_LRU",
//   src: "./img/todo.png",
//   port_u: Port::Outbound,
//   port_r: Port::Outbound,
//   port_d: Port::Inbound,
//   port_l: Port::Outbound,
//   cli_icon: '╬',
// };
// // ┌^┐
// // > >
// // └v┘
// pub const BELT_L_DRU: BeltMeta = BeltMeta {
//   btype: BeltType::L_DRU,
//   dbg: "BELT_L_DRU",
//   src: "./img/todo.png",
//   port_u: Port::Outbound,
//   port_r: Port::Outbound,
//   port_d: Port::Outbound,
//   port_l: Port::Inbound,
//   cli_icon: '╬',
// };
// // ┌v┐
// // < <
// // └v┘
// pub const BELT_RU_DL: BeltMeta = BeltMeta {
//   btype: BeltType::RU_DL,
//   dbg: "BELT_RU_DL",
//   src: "./img/todo.png",
//   port_u: Port::Inbound,
//   port_r: Port::Inbound,
//   port_d: Port::Outbound,
//   port_l: Port::Outbound,
//   cli_icon: '╬',
// };
// // ┌v┐
// // < >
// // └^┘
// pub const BELT_DU_LR: BeltMeta = BeltMeta {
//   btype: BeltType::DU_LR,
//   dbg: "BELT_DU_LR",
//   src: "./img/todo.png",
//   port_u: Port::Inbound,
//   port_r: Port::Outbound,
//   port_d: Port::Inbound,
//   port_l: Port::Outbound,
//   cli_icon: '╬',
// };
// // ┌v┐
// // > >
// // └v┘
// pub const BELT_LU_DR: BeltMeta = BeltMeta {
//   btype: BeltType::LU_DR,
//   dbg: "BELT_LU_DR",
//   src: "./img/todo.png",
//   port_u: Port::Inbound,
//   port_r: Port::Outbound,
//   port_d: Port::Outbound,
//   port_l: Port::Inbound,
//   cli_icon: '╬',
// };
// // ┌v┐
// // > >
// // └v┘
// pub const BELT_DL_RU: BeltMeta = BeltMeta {
//   btype: BeltType::LD_RU,
//   dbg: "BELT_LD_RU",
//   src: "./img/todo.png",
//   port_u: Port::Outbound,
//   port_r: Port::Outbound,
//   port_d: Port::Inbound,
//   port_l: Port::Inbound,
//   cli_icon: '╬',
// };
// // ┌^┐
// // < <
// // └^┘
// pub const BELT_DR_LU: BeltMeta = BeltMeta {
//   btype: BeltType::DR_LU,
//   dbg: "BELT_DR_LU",
//   src: "./img/todo.png",
//   port_u: Port::Outbound,
//   port_r: Port::Inbound,
//   port_d: Port::Inbound,
//   port_l: Port::Outbound,
//   cli_icon: '╬',
// };
// // ┌^┐
// // > <
// // └v┘
// pub const BELT_LR_DU: BeltMeta = BeltMeta {
//   btype: BeltType::LR_DU,
//   dbg: "BELT_LR_DU",
//   src: "./img/todo.png",
//   port_u: Port::Outbound,
//   port_r: Port::Inbound,
//   port_d: Port::Outbound,
//   port_l: Port::Inbound,
//   cli_icon: '╬',
// };
// // ┌^┐
// // > <
// // └^┘
// pub const BELT_DLR_U: BeltMeta = BeltMeta {
//   btype: BeltType::DLR_U,
//   dbg: "BELT_DLR_U",
//   src: "./img/todo.png",
//   port_u: Port::Outbound,
//   port_r: Port::Inbound,
//   port_d: Port::Inbound,
//   port_l: Port::Inbound,
//   cli_icon: '╬',
// };
// // ┌v┐
// // > >
// // └^┘
// pub const BELT_DLU_R: BeltMeta = BeltMeta {
//   btype: BeltType::DLU_R,
//   dbg: "BELT_DLU_R",
//   src: "./img/todo.png",
//   port_u: Port::Inbound,
//   port_r: Port::Outbound,
//   port_d: Port::Inbound,
//   port_l: Port::Inbound,
//   cli_icon: '╬',
// };
// // ┌v┐
// // > <
// // └v┘
// pub const BELT_LRU_D: BeltMeta = BeltMeta {
//   btype: BeltType::RLU_D,
//   dbg: "BELT_RLU_D",
//   src: "./img/todo.png",
//   port_u: Port::Inbound,
//   port_r: Port::Inbound,
//   port_d: Port::Outbound,
//   port_l: Port::Inbound,
//   cli_icon: '╬',
// };
// // ┌v┐
// // < <
// // └^┘
// pub const BELT_DRU_L: BeltMeta = BeltMeta {
//   btype: BeltType::DRU_L,
//   dbg: "BELT_DRU_L",
//   src: "./img/todo.png",
//   port_u: Port::Inbound,
//   port_r: Port::Inbound,
//   port_d: Port::Inbound,
//   port_l: Port::Outbound,
//   cli_icon: '╬',
// };


