use super::belt::*;
use super::cell::*;
use super::config::*;
use crate::direction::*;
use super::floor::*;
use super::factory::*;
use super::options::*;
use super::part::*;
use super::port::*;
use super::port_auto::*;
use super::state::*;
use super::utils::*;
use super::log;

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
  pub id: char,

  // Required input for this machine. Can be none. Can require up to one element per cell that the
  // machine occupies (arbitrary limit). Unused input slots are set to none.
  pub wants: Vec<Part>,
  pub haves: Vec<Part>,

  pub output_want: Part,
  pub start_at: u64,

  // Speed at which the machine produces output once all inputs are received
  pub speed: u64,

  pub production_price: i32, // Price you pay when a machine generates an output
  pub produced: u64,
  pub trash_price: i32, // Price you pay when a machine has to discard an invalid part
  pub trashed: u64,

  // The last 9 unique parts that this factory received, and the tick when this was _added_ to
  // this list last time. The craft menu will show the last 9 parts that a machine received.
  // Some effort is taken to attempt to keep the elements in place as much as possible.
  // The idea is that we remember the last 9 received parts and when we receive another one
  // that is not in this list, we replace the entry with the oldest timestamp with the new one.
  pub last_received: Vec< ( Part, u64 ) >,
  pub last_received_parts: Vec<PartKind>,
}

pub fn machine_none(config: &Config, main_coord: usize) -> Machine {
  return Machine {
    kind: MachineKind::None,
    main_coord,
    coords: vec!(),
    cell_width: 0,
    cell_height: 0,
    id: '!',

    wants: vec!(),
    haves: vec!(),

    start_at: 0,

    output_want: part_none(config),

    speed: 0,
    production_price: 0,
    produced: 0,
    trash_price: 0,
    trashed: 0,

    last_received: vec!(),
    last_received_parts: vec!(),
  };
}

pub fn machine_new(options: &Options, state: &State, config: &Config, kind: MachineKind, cell_width: usize, cell_height: usize, id: char, main_coord: usize, in_wants: Vec<Part>, output: Part, speed: u64) -> Machine {
  // Note: this is also called for each machine sub cell once
  let mut wants = in_wants.clone();
  let mut haves = vec!();

  let cw = cell_width * cell_height;
  for i in 0..cw {
    if wants.len() < cw {
      wants.push(part_none(config));
    }
    haves.push(part_none(config));
  }

  // log!("machine_new(), wants: {:?}", wants.iter().map(|p| p.kind));
  let output = machine_discover_output_wants(options, state, config, &wants, main_coord);
  // log!("  - machine_new(), yields: {:?}", config.nodes[output].kind);

  assert_eq!(wants.len(), haves.len(), "machines should start with same len wants as haves");

  return Machine {
    kind,
    main_coord,
    coords: vec!(),
    cell_width,
    cell_height,
    id,

    wants,
    haves,

    start_at: 0,

    output_want: part_from_part_kind(config, output),

    speed,
    production_price: 0,
    produced: 0,
    trash_price: 0,
    trashed: 0,

    last_received: vec!(),
    last_received_parts: vec!(),
  };
}

pub fn tick_machine(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, main_coord: usize) {
  // Finish building by giving the wanted part to a neighbor outbound belt
  // Accept from the input ports
  // Start building

  // Finished
  if factory.floor[main_coord].machine.start_at > 0 && factory.ticks - factory.floor[main_coord].machine.start_at >= factory.floor[main_coord].machine.speed {
    // log!("part {} is finished", factory.floor[main_coord].machine.output_want.icon);
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
        if factory.floor[to_coord].kind == CellKind::Belt && factory.floor[to_coord].belt.part.kind == CONFIG_NODE_PART_NONE {
          // The neighbor is a belt that is empty
          if options.print_moves || options.print_moves_machine {
            log!("({}) Machine @{} (sub @{}) finished part {:?}! Moving to belt @{}", factory.ticks, main_coord, sub_coord, factory.floor[main_coord].machine.output_want.kind, to_coord);
          }

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

  // Check receive/create state
  let mut accepts_nothing = true;
  let mut waiting_for_input = false;
  for i in 0..factory.floor[main_coord].machine.haves.len() {
    // If a certain input does not exist, it will be none.
    // If a certain input is none, then have should always be none too and it will auto-satisfy
    // Otherwise, it will satisfy if the have is not none, but rather the part that is wanted.
    let want = factory.floor[main_coord].machine.wants[i].kind;
    if want != CONFIG_NODE_PART_NONE {
      accepts_nothing = false;
    }
    if want != factory.floor[main_coord].machine.haves[i].kind {
      waiting_for_input = true;
      accepts_nothing = false;
      break;
    }
  }

  let mut trashed_anything = false;

  // Receive
  // It should only trash the input if it's actually still waiting for something so check that first
  if waiting_for_input || accepts_nothing {
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

          if belt_part != CONFIG_NODE_PART_NONE && !factory.floor[from_coord].belt.part_to_tbd && factory.floor[from_coord].belt.part_to == main_neighbor_in_dir && factory.floor[from_coord].belt.part_progress >= factory.floor[from_coord].belt.speed {
            // Check whether it fits in any input slot. If so, put it there. Otherwise trash it unless
            // all slots are full (only trash input parts while actually waiting for more input).

            assert_eq!(factory.floor[main_coord].machine.wants.len(), factory.floor[main_coord].machine.haves.len(), "machines should start with same len wants as haves");
            let mut trash = true;
            if !accepts_nothing {
              for i in 0..factory.floor[main_coord].machine.wants.len() {
                // There must be space
                let have = factory.floor[main_coord].machine.haves[i].kind;
                if have == CONFIG_NODE_PART_NONE {
                  // Accept if trash (joker part) or if it matches the required part
                  let want = factory.floor[main_coord].machine.wants[i].kind;
                  if want != CONFIG_NODE_PART_NONE {
                    let have_eq_wants = belt_part == factory.floor[main_coord].machine.wants[i].kind && belt_part != have;
                    let trash_joker = options.dbg_trash_is_joker && belt_part == CONFIG_NODE_PART_TRASH;
                    if have_eq_wants || trash_joker {
                      if options.print_moves || options.print_moves_machine {
                        log!("({}) Machine @{} (sub @{}) accepting part {:?} as input {} from belt @{}, had {:?}", factory.ticks, main_coord, sub_coord, belt_part, i, from_coord, have);
                      }
                      machine_receive_part(factory, main_coord, i, part_from_part_kind(config, want));
                      belt_receive_part(factory, from_coord, incoming_dir, part_none(config));
                      trash = false;
                      break;
                    }
                  }
                }
              }
            }

            if trash {
              if options.print_moves || options.print_moves_machine {
                log!("({}) Machine @{} (sub @{}) trashing part {:?} from belt @{}", factory.ticks, main_coord, sub_coord, belt_part, from_coord);
              }
              let part = factory.floor[from_coord].belt.part.clone();
              machine_update_oldest_list(factory, main_coord, &part);
              belt_receive_part(factory, from_coord, incoming_dir, part_none(config));
              factory.floor[main_coord].machine.trashed += 1;
              trashed_anything = true;
            }
          }
        }
      }
    }
  }

  // Produce
  if (!accepts_nothing || (trashed_anything && options.dbg_machine_produce_trash)) && factory.floor[main_coord].machine.start_at == 0 {
    let mut ready = true;
    for i in 0..factory.floor[main_coord].machine.haves.len() {
      // If a certain input does not exist, it will be none.
      // If a certain input is none, then have should always be none too and it will auto-satisfy
      // Otherwise, it will satisfy if the have is not none, but rather the part that is wanted.
      if factory.floor[main_coord].machine.wants[i].kind != factory.floor[main_coord].machine.haves[i].kind {
        ready = false;
        break;
      }
    }

    if ready {
      // Ready to produce a new part
      if options.print_moves || options.print_moves_machine { log!("({}) Machine @{} started to create new part", factory.ticks, main_coord); }
      for i in 0..factory.floor[main_coord].machine.haves.len() {
        factory.floor[main_coord].machine.haves[i] = part_none(config);
      }
      factory.floor[main_coord].machine.start_at = factory.ticks;
    }
  }
}

fn machine_receive_part(factory: &mut Factory, main_coord: usize, have_index: usize, part: Part) {
  machine_update_oldest_list(factory, main_coord, &part);

  factory.floor[main_coord].machine.haves[have_index] = part;
}
fn machine_update_oldest_list(factory: &mut Factory, main_coord: usize, part: &Part) {
  assert_ne!(part.kind, CONFIG_NODE_PART_NONE, "machine_update_oldest_list: should not receive NONE parts here...");
  assert_ne!(part.kind, 0, "machine_update_oldest_list: should not receive NONE parts here...");

  // Update the last_received, if necessary
  let mut oldest_index = 0;
  let mut oldest_ticks = factory.ticks;
  let len = factory.floor[main_coord].machine.last_received.len();
  for i in 0..len {
    if part.kind == factory.floor[main_coord].machine.last_received[i].0.kind {
      // This part was already in the recent list so stop this step
      return;
    }
    if oldest_ticks >= factory.floor[main_coord].machine.last_received[i].1 {
      oldest_index = i;
      oldest_ticks = factory.floor[main_coord].machine.last_received[i].1;
    }
  }

  if len < 9 {
    factory.floor[main_coord].machine.last_received.push( ( part.clone(), factory.ticks ) );
    factory.floor[main_coord].machine.last_received_parts.push(part.kind);
  } else {
    factory.floor[main_coord].machine.last_received[oldest_index] = ( part.clone(), factory.ticks );
    factory.floor[main_coord].machine.last_received_parts[oldest_index] = part.kind;
  }
}

pub fn machine_discover_ins_and_outs(factory: &mut Factory, main_coord: usize) {
  machine_discover_ins_and_outs_floor(&mut factory.floor, main_coord);
}
pub fn machine_discover_ins_and_outs_floor(floor: &mut [Cell; FLOOR_CELLS_WH], main_coord: usize) {
  assert_eq!(floor[main_coord].kind, CellKind::Machine, "cell should be a machine");
  assert_eq!(floor[main_coord].machine.main_coord, main_coord, "func should receive the main coord since thats where the ins and outs are collected");

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

pub fn machine_normalize_wants(wants: &Vec<PartKind>) -> Vec<PartKind> {
  let mut wants = wants.iter()
    .map(|&p| p)
    .filter(|&part_part| part_part != CONFIG_NODE_PART_NONE)
    .collect::<Vec<PartKind>>();
  wants.sort_unstable();
  return wants;
}

pub fn machine_change_want_kind(options: &Options, state: &State, config: &Config, factory: &mut Factory, main_coord: usize, index: usize, kind: PartKind) {
  factory.floor[main_coord].machine.wants[index] = part_from_part_kind(config, kind);

  let new_out = machine_discover_output_unmut(options, state, config, factory, main_coord, index, kind);
  log!("machine_change_want() -> {:?}", new_out);
  factory.floor[main_coord].machine.output_want = part_from_part_kind(config, new_out);
  factory.changed = true;
}
pub fn machine_discover_output_unmut(options: &Options, state: &State, config: &Config, factory: &Factory, main_coord: usize, index: usize, kind: PartKind) -> PartKind {
  log!("machine_discover_output_unmut()");
  // Work around slicing muts. why this does work is beyond me rn.
  return machine_discover_output_floor(options, state, config, &factory.floor, main_coord);
}
pub fn machine_discover_output_floor(options: &Options, state: &State, config: &Config, floor: &[Cell; FLOOR_CELLS_WH], main_coord: usize) -> PartKind {
  // Given a set of wants, determine what the output should be
  // Things to consider;
  // - input parts (sorted list without nones)
  // - factory type
  // - unlock tree
  // - level limitations / specials (?)

  // Probably only a subset of these? with expansion options
  return machine_discover_output_wants(options, state, config, &floor[main_coord].machine.wants, main_coord);
}
pub fn machine_discover_output_wants(options: &Options, state: &State, config: &Config, wants: &Vec<Part>, main_coord: usize) -> PartKind {
  // log!("machine_discover_output_wants({}): {:?}", main_coord, wants);
  let mut ordered_icons = wants.iter().map(|part| part.icon).collect::<Vec<char>>();
  ordered_icons.sort();
  let pattern_str_untrimmed = ordered_icons.iter().collect::<String>().to_string();
  let pattern_str = str::replace(pattern_str_untrimmed.trim(), " ", "");
  if pattern_str == "" {
    // log!("  Machine has no inputs so it has no output");
    return CONFIG_NODE_PART_NONE;
  }
  let target_kind = *config.node_pattern_to_index.get(pattern_str.as_str()).or(Some(&CONFIG_NODE_PART_NONE)).unwrap();
  // log!("  Looking in node_pattern_to_index for: `{}` --> resulting target kind: {} -> {:?}", pattern_str, target_kind, config.nodes[target_kind]);
  assert!(config.nodes[target_kind].kind == ConfigNodeKind::Part, "the pattern should resolve to a part node...");
  return target_kind;
}

pub fn machine_size_to_asset_index(width: usize, height: usize) -> usize {
  return match ( width, height ) {
    ( 1, 1 ) => CONFIG_NODE_ASSET_MACHINE1,
    ( 2, 2 ) => CONFIG_NODE_ASSET_MACHINE2,
    ( 3, 3 ) => CONFIG_NODE_ASSET_MACHINE_1_1,
    ( 3, 4 ) => CONFIG_NODE_ASSET_MACHINE3,
    ( 4, 4 ) => CONFIG_NODE_ASSET_MACHINE4,
    ( 2, 1 ) => CONFIG_NODE_ASSET_MACHINE_2_1,
    ( 4, 2 ) => CONFIG_NODE_ASSET_MACHINE_2_1,
    ( 3, 2 ) => CONFIG_NODE_ASSET_MACHINE_3_2,
    _ => CONFIG_NODE_ASSET_MACHINE_1_1,
  };
}

/**
 * Put the coordinates of various machine indicators at various machine cell sizes.
 * The coords are absolute and relative to the offset of the machine. Use CELL_W/CELL_H.
 */
pub struct MachineUIConfig {
  // Coords are relative to offset of machine, units in cell size

  pub missing_input_x: f64,
  pub missing_input_y: f64,

  pub missing_output_x: f64,
  pub missing_output_y: f64,

  pub missing_purpose_x: f64,
  pub missing_purpose_y: f64,

  pub wee_woo_x: f64,
  pub wee_woo_y: f64,

  pub part_x: f64,
  pub part_y: f64,
}

pub const MACHINE_1X1_UI: MachineUIConfig = MachineUIConfig {
  missing_input_x: 0.0,
  missing_input_y: 0.0,

  missing_output_x: 0.0,
  missing_output_y: 0.0,

  missing_purpose_x: 0.0,
  missing_purpose_y: 0.0,

  wee_woo_x: 0.0,
  wee_woo_y: 0.0,

  part_x: 0.0,
  part_y: 0.0,
};

pub const MACHINE_1X2_UI: MachineUIConfig = MachineUIConfig {
  missing_input_x: 0.0,
  missing_input_y: 0.0 + CELL_H,

  missing_output_x: 0.0,
  missing_output_y: 0.0 + CELL_H,

  missing_purpose_x: 0.0,
  missing_purpose_y: 0.0 * CELL_H / 2.0,

  wee_woo_x: 0.0,
  wee_woo_y: 5.0,

  part_x: 0.0,
  part_y: 0.0,
};

pub const MACHINE_2X1_UI: MachineUIConfig = MachineUIConfig {
  missing_input_x: 0.0,
  missing_input_y: 0.0,

  missing_output_x: 0.0 + CELL_W,
  missing_output_y: 0.0,

  missing_purpose_x: 0.0 + CELL_W / 2.0,
  missing_purpose_y: 0.0,

  wee_woo_x: 0.0 + CELL_W,
  wee_woo_y: 5.0,

  part_x: 0.0,
  part_y: 0.0,
};

pub const MACHINE_2X2_UI: MachineUIConfig = MachineUIConfig {
  missing_input_x: 0.0,
  missing_input_y: 0.0 + CELL_H,

  missing_output_x: 0.0 + CELL_W,
  missing_output_y: 0.0 + CELL_H,

  missing_purpose_x: 0.0 + CELL_W / 2.0,
  missing_purpose_y: 0.0 + 0.75 * CELL_H,

  wee_woo_x: 0.0 + CELL_W,
  wee_woo_y: 5.0,

  part_x: 0.0,
  part_y: 0.0,
};

pub const MACHINE_3X3_UI: MachineUIConfig = MachineUIConfig {
  missing_input_x: 0.0,
  missing_input_y: 0.0 + CELL_H,

  missing_output_x: 0.0 + CELL_W + CELL_W,
  missing_output_y: 0.0 + CELL_H,

  missing_purpose_x: 0.0 + CELL_W,
  missing_purpose_y: 0.0 + CELL_H,

  wee_woo_x: 0.0 + CELL_W + CELL_W,
  wee_woo_y: 0.0 + CELL_W + CELL_W,

  part_x: 0.0,
  part_y: 0.0,
};

pub fn get_machine_ui_config(w: usize, h: usize) -> MachineUIConfig {
  if w == 1 && h == 2 {
    return MACHINE_1X2_UI;
  }
  if w == 2 && h == 1 {
    return MACHINE_2X1_UI;
  }
  if w == 2 && h == 2 {
    return MACHINE_2X2_UI;
  }
  if w == 3 && h == 3 {
    return MACHINE_3X3_UI;
  }
  return MACHINE_1X1_UI;
}

pub fn machine_add_to_factory(options: &Options, state: &State, config: &Config, factory: &mut Factory, cx: usize, cy: usize, machine_cell_width: usize, machine_cell_height: usize) {
  let ccoord = to_coord(cx, cy);

  // Get all machines and then get the first unused ID. First we round up all the existing
  // machine ids into a vector and then we iterate through the vector incrementally until
  // an ID is not used. This is O(n^2) but realistically worst case O(63^2) and good luck.

  let mut ids = vec!();
  for coord in 0..FLOOR_CELLS_WH {
    if factory.floor[coord].kind == CellKind::Machine && factory.floor[coord].machine.main_coord == coord {
      ids.push(factory.floor[coord].machine.id);
    }
  }

  // Now iterate through all valid IDs, that is: 0-9a-zA-Z. I guess bail if we exhaust that.
  // TODO: gracefully handle too many machines
  let mut found = '!';
  // Note: machine ids offset at 1 (because m0 is just too confusing for comfort)
  for id in 1..62 {
    let c =
      if id >= 36 {
        (('A' as u8) + (id - 36)) as char // A-Z
      } else if id > 9 {
        (('a' as u8) + (id - 10)) as char // a-z
      } else {
        (('0' as u8) + id) as char // 1-9
      };
    if !ids.contains(&c) {
      found = c;
      break;
    }
  }
  if found == '!' {
    panic!("Unable to find a fresh ID. Either there are too many machines on the floor or there is a bug with reclaiming them. Or d: something else.");
  }

  // Fill the rest with sub machine cells
  for i in 0..machine_cell_width {
    for j in 0..machine_cell_height {
      let x = cx + i;
      let y = cy + j;
      let coord = to_coord(x, y);

      // Meh. But we want to remember this state for checks below.
      let ( mut port_u, mut port_r, mut port_d, mut port_l, coord_u, coord_r, coord_d, coord_l ) = match factory.floor[coord] {
        super::cell::Cell { port_u, port_r, port_d, port_l, coord_u, coord_r, coord_d, coord_l, .. } => ( port_u, port_r, port_d, port_l, coord_u, coord_r, coord_d, coord_l )
      };

      // If the neighbor was a machine then reset the port
      if let Some(c) = coord_u { if factory.floor[c].kind == CellKind::Machine { port_u = Port::None; } };
      if let Some(c) = coord_r { if factory.floor[c].kind == CellKind::Machine { port_r = Port::None; } };
      if let Some(c) = coord_d { if factory.floor[c].kind == CellKind::Machine { port_d = Port::None; } };
      if let Some(c) = coord_l { if factory.floor[c].kind == CellKind::Machine { port_l = Port::None; } };

      // Make sure to drop machines properly. Belts are 1x1 so no problem. Empty are fine.
      if factory.floor[coord].kind == CellKind::Machine {
        floor_delete_cell_at_partial(options, state, config, factory, coord);
      }

      if i == 0 && j == 0 {
        // Top-left cell is the main_coord here
        factory.floor[coord] = machine_main_cell(
          options,
          state,
          config,
          found,
          x, y,
          machine_cell_width, machine_cell_height,
          vec!(), // Could fill with trash but no need I guess
          part_c(config, 't'),
          2000,
          1, 1
        );
      } else {
        factory.floor[coord] = machine_sub_cell(options, state, config, found, x, y, ccoord, machine_cell_width, machine_cell_height);
      }
      factory.floor[ccoord].machine.coords.push(coord);

      factory.floor[coord].port_u = if j == 0 { port_u } else { Port::None };
      factory.floor[coord].port_r = if i == machine_cell_width - 1 { port_r } else { Port::None };
      factory.floor[coord].port_d = if j == machine_cell_height - 1 { port_d } else { Port::None };
      factory.floor[coord].port_l = if i == 0 { port_l } else { Port::None };
    }
  }

  log!("Attaching machine to neighbor dead ending belts");
  for i in 0..factory.floor[ccoord].machine.coords.len() {
    let coord = factory.floor[ccoord].machine.coords[i];
    connect_to_neighbor_dead_end_belts(options, state, config, factory, coord);
  }

  machine_discover_ins_and_outs(factory, ccoord);

  factory.machines.push(ccoord);

  factory.changed = true;
}

pub fn machine_set_target_part_kind(options: &Options, state: &State, config: &Config, factory: &mut Factory, main_coord: usize, target_part_kind: PartKind) {
  for want_index in 0..factory.floor[main_coord].machine.cell_width * factory.floor[main_coord].machine.cell_height {
    let part_kind = config.nodes[target_part_kind].pattern_by_index.get(want_index).unwrap_or(&CONFIG_NODE_PART_NONE);
    machine_change_want_kind(options, state, config, factory, main_coord, want_index, *part_kind);
    // Make sure the haves are cleared as well
    factory.floor[main_coord].machine.haves[want_index] = part_none(config);
  }
}
