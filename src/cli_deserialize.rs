use std::borrow::Borrow;
use std::collections::VecDeque;
use std::convert::TryInto;

use super::belt::*;
use super::belt_meta::*;
use crate::belt_type::*;
use super::cell::*;
use super::config::*;
use super::demand::*;
use super::factory::*;
use super::floor::*;
use super::direction::*;
use super::machine::*;
use super::options::*;
use super::part::*;
use super::port::*;
use super::port_auto::*;
use super::state::*;
use super::supply::*;
use super::utils::*;
use super::log;

pub fn floor_from_str(options: &Options, state: &mut State, config: &Config, str: &String) -> ( [Cell; FLOOR_CELLS_WH], Vec<char> ) {
  if str.trim().len() == 0 {
    return (floor_empty(config), vec!());
  }

  return str_to_floor2(options, state, config, str);
}

fn str_to_floor2(options: &Options, state: &mut State, config: &Config, str: &String) -> ([Cell; FLOOR_CELLS_WH], Vec<char>) {
  // Given a string in a grid format, generate a floor
  // The string starts with at least one line of config.
  // - For now the only modifier are the dimension of the hardcoded 11x11
  // - Everything after the # is supposed to be a non-parsed comment
  // - (Trimmed) Lines that start with # are ignored as line comments
  // Next follows a grid of that size, each cell being 3x3 characters, each line trimmed.
  // Cells composition:
  // - The center determines the kind
  //   - Empty: space or dot
  //   - Belt: b or any table ascii art
  //   - Supply: s with the given part below or to the right
  //   - Demand: d with the given part below or to the right
  //   - Machine: single digits to connect all cells of the same machine
  // - The up/right/down/left char determine ports
  // - Rest is ignored. Can be whatever.
  // Auto-layout is applied afterwards.

  // d:11x11
  // # Generated 2022-07-08T22:00:01
  // ┌─────────────────────────────────────────────────┐
  // │.  .  .  .  .  .  .  .  .  .  .  .  .  .  .  sw .│
  // │ ┌───────────────────────────────────────────v─┐ │
  // │ │      ┌───────┐                            v │ │
  // │s>>╗  . │       <<═<<═<<═<<═<<═<<═<<═<<╗  .  ║ │.│
  // │w│ v    │       │                      ^     v │ │
  // │ │ v    │       │                      ^     v │ │
  // │.│ ║  . │   0   │ .  .  .  .  .  .  .  ║  .  ║ │.│
  // │ │ v    │       │                      ^     v │ │
  // │ │ v    │       │                      ^     v │ │
  // │.│ ║  . │       >>═>>═>>═>>═>>╗  .  .  ╚<<═<<╣ │.│
  // │ │ v    └^─────v┘             v              ^ │ │
  // │ │ v     ^     v              v              ^ │ │
  // │.│ ║  .  ║  .  ║  .  .  .  .  ╚>>╗  .  .  .  ║ │.│
  // │ │ v     ^     v                 v           ^ │ │
  // │ │ v     ^     v                 v           ^ │ │
  // │.│ ║  .  ║  .  ╚>>═>>═>>╗  .  .  ║  .  .  .  ║ │.│
  // │ │ v     ^              v        v           ^ │ │
  // │ │ v     ^              v        v           ^ │ │
  // │.│ ║  .  ║  .  .  .  .  ║  .  .  ║  .  .  .  ║ │.│
  // │ │ v     ^              v        v           ^ │ │
  // │ │ v     ^          ┌───v───┐    v           ^ │ │
  // │.│ ╚>>═>>╣  .  .  . │       │ .  ║  .  .  .  ║ │.│
  // │ │       ^          │       │    v           ^ │ │
  // │ │       ^          │       │    v           ^ │ │
  // │.│ .  .  ║  .  .  . │   1   │ .  ║  .  .  ╔>>╝ │.│
  // │ │       ^          │       │    v        ^    │ │
  // │ │       ^          │       │    v        ^    │ │
  // │s>>═>>═>>╝  .  .  . │       │ .  ║  .  .  ║  . │.│
  // │s│                  └───v───┘    v        ^    │ │
  // │ │                      v        v        ^    │ │
  // │.│ .  .  .  .  .  .  .  ║  .  .  ║  .  .  ╚<<═<<s│
  // │ │                      v        v             │s│
  // │ │                      v        v             │ │
  // │.│ .  ╔<<═<<═<<═<<═<<═<<╣  .  .  ║  .  .  .  . │.│
  // │ │    v                 v        v             │ │
  // │ │    v                 v        v             │ │
  // │.│ .  ║  .  .  .  .  .  ║  .  .  ╚>>═>>═>>═>>═>>d│
  // │ │    v                 v                      │s│
  // │ │    v                 v                      │ │
  // │.│ .  ║  .  .  .  .  .  ║  .  .  .  .  .  .  . │.│
  // │ │    v                 v                      │ │
  // │ │    v                 v                      │ │
  // │d<<═<<╝  .  .  ╔<<═<<═<<╝  .  .  .  .  .  .  . │.│
  // │w│             v                               │ │
  // │ │             v                               │ │
  // │.│ .  .  .  .  ║  .  .  .  .  .  .  .  .  .  . │.│
  // │ │             v                               │ │
  // │ └─────────────v───────────────────────────────┘ │
  // │.  .  .  .  .  dg .  .  .  .  .  .  .  .  .  .  .│
  // └─────────────────────────────────────────────────┘
  // m1 = w . . . s   -> b
  // m2 = b           -> g
  // os = w s:10 c:5
  // os = s s:10 c:5
  // od = g
  // om = s . . w   -> b s:0  d:3x2
  // om = b         -> g s:10 d:1x1
  // om = b         -> g s:0  d:3x3
  // om = b         -> g s:0  d:4x4
  // $ abc

  log!("str_to_floor2(options.trace_map_parsing={})", options.trace_map_parsing);
  if options.trace_map_parsing { log!("{}", str); }

  let mut floor: [Cell; FLOOR_CELLS_WH] = floor_empty(config);
  // Populate the unlocked icons by at least the ones that unlock by default
  let mut unlocked_part_icons: Vec<char> = config_get_initial_unlocks(options, state, config);

  let hash: &char = &'#';
  let space: &u8 = &32u8;

  let mut lines = str.lines().collect::<Vec<&str>>();

  // Parse the first config line
  let mut lines = lines.iter_mut().map(|s| s.chars().peekable()).collect::<Vec<_>>();
  let mut lines = lines.iter_mut(); // hafta or the compiler complains

  let mut first_line = lines.next().unwrap(); // Bust if there's no input.
  if options.trace_map_parsing { log!("first First line: {:?}", first_line); }
  loop {
    while first_line.peek().or(Some(&'#')).unwrap() == &' ' { first_line.next(); }
    // Keep skipping lines that start with comments and empty lines (only containing spaces)
    if first_line.peek().or(Some(&'#')).unwrap() != &'#' {
      break;
    }
    first_line = lines.next().unwrap(); // Bust if there's no more input.
  }
  if options.trace_map_parsing { log!("First line after comments: {:?}", first_line); }

  // We should now be at the start of the first non-comment line.
  // It is asserted to be the map header line.

  // The header line is currently only expecting one modifier: the floor dimensions

  loop {
    match first_line.next().or(Some('#')).unwrap() {
      '#' => {
        // EOL
        break;
      }
      '\n' => {
        // EOL
        break;
      }
      'd' => {
        // Expecting `d=axb` where a and b are consecutive digits
        if first_line.next().or(Some('#')).unwrap() != '=' { panic!("Error parsing `d` modifier in header: The `d` should be followed by `=` but was not."); }

        let mut w: usize = 0;
        loop {
          let d = first_line.next().or(Some('#')).unwrap() as u8;
          if d >= ('0' as u8) && d <= ('9' as u8) {
            w = (w * 10) + (d - ('0' as u8)) as usize;
          } else if d == ('x' as u8) {
            break;
          } else {
            panic!("Error parsing `d` modifier in header: Expected an `x` after the width, found `{}`", d as char);
          }
        }
        let mut h: usize = 0;
        loop {
          let d = *first_line.peek().or(Some(&'#')).unwrap() as u8;
          if d >= ('0' as u8) && d <= ('9' as u8) {
            h = (h * 10) + (d - ('0' as u8)) as usize;
            first_line.next();
          } else if d == ('#' as u8) || d == ('\n' as u8) {
            break;
          } else {
            panic!("Error parsing `d` modifier in header: Expected an `x` after the width, found `{}`", d as char);
          }
        }

        if w == 0 || h == 0 {
          panic!("Error parsing `d` modifier in header: At least one dimension was parsed as zero: {} x {}", w, h);
        }
        if w != FLOOR_CELLS_W {
          panic!("Error parsing `d` modifier in header: The width must match the currently hardcoded width of {}, but it was {}", FLOOR_CELLS_W, w);
        }
        if h != FLOOR_CELLS_H {
          panic!("Error parsing `d` modifier in header: The height must match the currently hardcoded height of {}, but it was {}", FLOOR_CELLS_H, h);
        }

        // Okay, we now have a proper width and height and it matches the current hardcoded values. Move along.
        if options.trace_map_parsing { log!("Map size: {} x {}", w, h); }
      }
      _ => {}
    }
  }

  fn add_machine(options: &Options, state: &mut State, config: &Config, floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize, x: usize, y: usize, machine_id: char, machine_meta_data: &mut Vec<(usize, usize, char, u64, Vec<PartKind>)>, port_u: char, port_r: char, port_d: char, port_l: char) {
    // Auto layout will have to reconcile the individual machine parts into one machine
    // Any modifiers as well as the input and output parameters of this machine are
    // listed below the floor model. Expect them to be filled in later.
    let mn = (machine_id as u8 - ('0' as u8)) as usize;

    let mut index = machine_meta_data.len();
    machine_meta_data.iter_mut().enumerate().any(|(i, obj)| {
      if obj.2 != machine_id {
        return false;
      }

      obj.1 = coord; // Coords are walked left-to-right top-to-bottom so the next coord is always higher than the current.
      index = i;
      return true;
    });

    if index == machine_meta_data.len() {
      machine_meta_data.push((coord, coord, machine_id, 1, vec!()));
    }

    let mut cell = machine_any_cell(options, state, config, machine_id as char, x, y, 1, 1, MachineKind::Unknown, vec!(), part_c(config, ' '), 1, 1, 1);
    cell.port_u = match port_u as char { '^' => Port::Outbound, 'v' => Port::Inbound, '?' => Port::Unknown, ' ' => Port::None, '─' => Port::None, '│' => Port::None, _ => panic!("Port up indicators must be `^`, `v`, `?` or a space, this was `{}`", port_u)};
    cell.port_r = match port_r as char { '>' => Port::Outbound, '<' => Port::Inbound, '?' => Port::Unknown, ' ' => Port::None, '─' => Port::None, '│' => Port::None, _ => panic!("Port right indicators must be `<`, `>`, `?` or a space, this was `{}`", port_u)};
    cell.port_d = match port_d as char { 'v' => Port::Outbound, '^' => Port::Inbound, '?' => Port::Unknown, ' ' => Port::None, '─' => Port::None, '│' => Port::None, _ => panic!("Port down indicators must be `^`, `v`, `?` or a space, this was `{}`", port_u)};
    cell.port_l = match port_l as char { '<' => Port::Outbound, '>' => Port::Inbound, '?' => Port::Unknown, ' ' => Port::None, '─' => Port::None, '│' => Port::None, _ => panic!("Port left indicators must be `<`, `>`, `?` or a space, this was `{}`", port_u)};
    floor[coord] = cell;

    if options.trace_map_parsing { log!("This was a machine [{}]: cell @{}, main_coord @{}, with id {}", index, coord, machine_meta_data[index].2, machine_id); }
  }

  let mut machine_meta_data: Vec<(
    usize, // main_coord (coord of the top-left-most cell). unused machine if zero.
    usize, // max_coord (coord of the bottom-right-most cell). unused machine if zero.
    char, // id
    u64, // Speed modifier, default=1
    Vec<PartKind>, // pattern (for a machine up to 5x5)
  )> = vec!();

  // Expect as many cells as specified.
  for j in 0..FLOOR_CELLS_H {

    // Now get the next three lines, skip leading spaces and skip empty lines or ones starting with a hash

    let mut line1 = lines.next().unwrap();
    loop {
      while line1.peek().or(Some(&'#')).unwrap() == &' ' { line1.next(); }
      // Keep skipping lines that start with comments and empty lines (only containing spaces)
      if line1.peek().or(Some(&'#')).unwrap() != &'#' {
        break;
      }
      line1 = lines.next().unwrap(); // Bust if there's no more input.
    }
    let mut line2 = lines.next().unwrap();
    loop {
      while line2.peek().or(Some(&'#')).unwrap() == &' ' { line2.next(); }
      // Keep skipping lines that start with comments and empty lines (only containing spaces)
      if line2.peek().or(Some(&'#')).unwrap() != &'#' {
        break;
      }
      line2 = lines.next().unwrap(); // Bust if there's no more input.
    }
    let mut line3 = lines.next().unwrap();
    loop {
      while line3.peek().or(Some(&'#')).unwrap() == &' ' { line3.next(); }
      // Keep skipping lines that start with comments and empty lines (only containing spaces)
      if line3.peek().or(Some(&'#')).unwrap() != &'#' {
        break;
      }
      line3 = lines.next().unwrap(); // Bust if there's no more input.
    }

    if options.trace_map_parsing {
      log!("three lines:");
      log!("1: {:?}", line1);
      log!("2: {:?}", line2);
      log!("3: {:?}", line3);
    }

    for i in 0..FLOOR_CELLS_W {
      // For each of the three lines, step them cell by cell, each cell consisting of 9 chars.
      // Characters are expected to be consecutive in sync across the three lines, even if they
      // had an inconsistent amount of leading spaces.

      let coord = to_coord(i, j);

      // (We don't care about the outer corner characters)
      let _a = line1.next().or(Some('#')).unwrap();
      let b = line1.next().or(Some('#')).unwrap();
      let _c = line1.next().or(Some('#')).unwrap();
      let d = line2.next().or(Some('#')).unwrap();
      let e = line2.next().or(Some('#')).unwrap();
      let f = line2.next().or(Some('#')).unwrap();
      let _g = line3.next().or(Some('#')).unwrap();
      let h = line3.next().or(Some('#')).unwrap();
      let _i = line3.next().or(Some('#')).unwrap();

      if options.trace_map_parsing { log!("{}x{}:\n   {} {} {}\n   {} {} {}\n   {} {} {}", i, j, _a, b, _c, d, e, f, _g, h, _i); }

      let cell_kind = e;
      let port_u = b;
      let port_r = f;
      let port_d = h;
      let port_l = d;

      match cell_kind as char {
        's' => {
          if is_middle(i as f64, j as f64) {
            add_machine(options, state, config, &mut floor, coord, i, j, cell_kind, &mut machine_meta_data, port_u, port_r, port_d, port_l);
          } else {
            let ( port_u, port_r, port_d, port_l ) =
              if j == 0 { ( Port::None, Port::None, Port::Outbound, Port::None ) }
              else if i == FLOOR_CELLS_W-1 { ( Port::None, Port::Outbound, Port::None, Port::None ) }
              else if j == FLOOR_CELLS_H-1 { ( Port::Outbound, Port::None, Port::None, Port::None ) }
              else if i == 0 { ( Port::None, Port::None, Port::None, Port::Outbound ) }
              else {
                panic!("Error parsing floor cell: Encountered an `s` inside the floor; this should be a Supply, which is bound to the edge");
              };
            // The speed and cooldown of the supply have to be added below the floor so use placeholder values for now; TODO: wire that up
            if options.trace_map_parsing { log!("Supply"); }
            let cell = supply_cell(config, i, j, part_c(config, 't'), 1, 1, 1);
            floor[coord] = cell;
            if options.trace_map_parsing { log!("This was a supplier"); }
          }
        },

        'd' => {
          if is_middle(i as f64, j as f64) {
            add_machine(options, state, config, &mut floor, coord, i, j, cell_kind, &mut machine_meta_data, port_u, port_r, port_d, port_l);
          } else {
            let ( port_u, port_r, port_d, port_l ) =
            if j == 0 {
              ( Port::None, Port::None, Port::Outbound, Port::None )
            } else if i == FLOOR_CELLS_W-1 {
              ( Port::None, Port::Outbound, Port::None, Port::None )
            } else if j == FLOOR_CELLS_H-1 {
              ( Port::Outbound, Port::None, Port::None, Port::None )
            } else if i == 0 {
              ( Port::None, Port::None, Port::None, Port::Outbound )
            } else {
              panic!("Error parsing floor cell: Encountered an `d` inside the floor; this should be a Demand, which is bound to the edge");
            };
            if options.trace_map_parsing { log!("Demand"); }
            let cell = demand_cell(config, i, j, options.default_demand_speed, options.default_demand_cooldown);
            floor[coord] = cell;
            if options.trace_map_parsing { log!("This was a demander"); }
          }
        },

        // Empty cells are spaces or dots
        | ' '
        | '.'
        => {
          floor[coord] = empty_cell(config, i, j);
          if options.trace_map_parsing { log!("This was an empty cell"); }
        }

        // Then there's a bunch of belt cells. These are either `b` or "table ascii art" chars
        // The actual char is not relevant here since we will auto-discover the meta of the cell
        // based on the port configuration, anyways.

        // Joker, unspecified belt. Ports are ignored. (b is used by machine id)
        '%' => {
          // Fix belt meta later in the auto layout step
          let mut cell = belt_cell(config, i, j, BELT_UNKNOWN);
          cell.port_u = Port::Unknown;
          cell.port_r = Port::Unknown;
          cell.port_d = Port::Unknown;
          cell.port_l = Port::Unknown;
          floor[coord] = cell;
          if options.trace_map_parsing { log!("This was a BELT_UNKNOWN"); }
        }

        // Explicit tiles use box art but are exclusively defined by the ports
        | '╹' // double lines have no one-arm glyph :rolls-eys: so we use thick line instead
        | '╺'
        | '╻'
        | '╸'
        | '╷'
        | '╴'
        | '╵'
        | '╶'
        | '╚'
        | '╔'
        | '╗'
        | '╝'
        | '║'
        | '═'
        | '╩'
        | '╠'
        | '╦'
        | '╣'
        | '╬'
        => {
          // Fix belt meta later in the auto layout step

          let belt_type = belt_type_from_ports(
            match port_u as char { '^' => Port::Outbound, 'v' => Port::Inbound, '?' => Port::Unknown, ' ' => Port::None, '─' => Port::None, '│' => Port::None, _ => panic!("Port up indicators must be `^`, `v`, `?` or a space, this was `{}`", port_u)},
            match port_r as char { '>' => Port::Outbound, '<' => Port::Inbound, '?' => Port::Unknown, ' ' => Port::None, '─' => Port::None, '│' => Port::None, _ => panic!("Port right indicators must be `<`, `>`, `?` or a space, this was `{}`", port_u)},
            match port_d as char { 'v' => Port::Outbound, '^' => Port::Inbound, '?' => Port::Unknown, ' ' => Port::None, '─' => Port::None, '│' => Port::None, _ => panic!("Port down indicators must be `^`, `v`, `?` or a space, this was `{}`", port_u)},
            match port_l as char { '<' => Port::Outbound, '>' => Port::Inbound, '?' => Port::Unknown, ' ' => Port::None, '─' => Port::None, '│' => Port::None, _ => panic!("Port left indicators must be `<`, `>`, `?` or a space, this was `{}`", port_u)},
          );
          let belt_meta = belt_type_to_belt_meta(belt_type);
          let cell = belt_cell(config, i, j, belt_meta);
          floor[coord] = cell;
          if options.trace_map_parsing { log!("This was a specific belt: {:?}", belt_type); }
        }

        _ => {
          if (cell_kind >= '1' && cell_kind <= '9') || (cell_kind >= 'a' && cell_kind <= 'z') || (cell_kind >= 'A' && cell_kind <= 'Z') {
            // Machine id is a single char 1-9a-zA-Z
            // Note: s and d are special cased between middle and edge cell above
            add_machine(options, state, config, &mut floor, coord, i, j, cell_kind, &mut machine_meta_data, port_u, port_r, port_d, port_l);
          } else {
            panic!("Error while parsing factory string: Encountered an unknown center cell char at {}x{}: `{}` ({})", i, j, cell_kind as char, cell_kind)
          }
        },
      }
    }
  }

  // Keep parsing config lines while skipping comments. These are optional and augment
  // things on the floor that don't really fit inside the schematic cleanly
  let mut line_no = 0;
  loop {
    line_no += 1;
    match lines.next() {
      None => break,
      Some(line) => {
        if options.trace_map_parsing { log!("Next line({}): {}", line_no, line.clone().collect::<String>()); }
        while line.peek().or(Some(&'#')).unwrap() == &' ' { line.next(); }
        // Keep skipping lines that start with comments and empty lines (only containing spaces)
        if line.peek().or(Some(&'#')).unwrap() != &'#' {
          let c = line.next().or(Some('#')).unwrap();
          match c {
            's' => {
              // s<n> = <p> [s:<d+>] [c:<d+>]
              // s1 = w s:100 c:100
              let nth;
              let mut speed = 1;
              let mut cooldown = 1;
              let gives;

              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              nth = alnum_to_n(c);

              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              if c != '=' { panic!("Unexpected input on line {} while parsing supply augment: first character after `s{}` must be the `=` sign, found `{}`", line_no, n_to_alnum(nth), c); }

              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              if !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c as u8 > 127) { panic!("Unexpected input on line {} while parsing supply augment kind: input characters must be a-zA-Z or non-ascii, found `{}`", line_no, c); }
              gives = c;

              loop {
                let mut c = line.next().or(Some('#')).unwrap();
                while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                match c {
                  '#' => break, // EOL or start of line comment
                  's' => {
                    // speed modifier
                    let mut c = line.next().or(Some('#')).unwrap();
                    while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                    if c != ':' { panic!("Unexpected input on line {} while parsing supply augment speed modifier: first character after `s` must be a `:`, found `{}`", line_no, c); }

                    speed = 0;
                    let mut c = line.next().or(Some('#')).unwrap();
                    while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                    loop {
                      if c >= '0' && c <= '9' {
                        speed = (speed * 10) + ((c as u8) - ('0' as u8)) as u64; // This can lead to overflow fatal. :shrug:
                      } else if c == '#' || c == ' ' {
                        break;
                      } else {
                        panic!("Unexpected input on line {} while parsing supply augment speed modifier: speed value consists of digits, found `{}`", line_no, c);
                      }
                      c = line.next().or(Some('#')).unwrap();
                    }
                  }
                  'c' => {
                    // cooldown modifier
                    let mut c = line.next().or(Some('#')).unwrap();
                    while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                    if c != ':' { panic!("Unexpected input on line {} while parsing supply augment cooldown modifier: first character after `c` must be a `:`, found `{}`", line_no, c); }

                    cooldown = 0;
                    let mut c = line.next().or(Some('#')).unwrap();
                    while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                    loop {
                      if c >= '0' && c <= '9' {
                        cooldown = (cooldown * 10) + ((c as u8) - ('0' as u8)) as u64; // This can lead to overflow fatal. :shrug:
                      } else if c == '#' || c == ' ' {
                        break;
                      } else {
                        panic!("Unexpected input on line {} while parsing supply augment cooldown modifier: cooldown value consists of digits, found `{}`", line_no, c);
                      }
                      c = line.next().or(Some('#')).unwrap();
                    }
                  }
                  c => panic!("Unexpected input on line {} while parsing supply augment modifier: expecting `s`, `c`, '#', or EOL, found `{}`", line_no, c),
                }
              }

              let mut n = 1;
              // Find the nth supply. Not super optimal but at this scale no real issue.
              for coord in 0..FLOOR_CELLS_WH {
                if floor[coord].kind == CellKind::Supply {
                  if n == nth {
                    if options.trace_map_parsing { log!("Updating supply {} @{} with part {} and speed {} and cooldown {}", nth, coord, gives, speed, cooldown); }
                    floor[coord].supply.speed = speed;
                    floor[coord].supply.cooldown = cooldown;
                    floor[coord].supply.gives = part_c(config, gives);
                    break;
                  }
                  n += 1;
                }
              }
            },
            'd' => {
              // Demand
              // d<n> [s:<n>] [c:<n>]
              // d1

              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              let nth = alnum_to_n(c);

              let mut speed = 1;
              let mut cooldown = 1;

              loop {
                let mut c = line.next().or(Some('#')).unwrap();
                while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                match c {
                  '#' => break, // EOL or start of line comment
                  's' => {
                    // speed modifier
                    let mut c = line.next().or(Some('#')).unwrap();
                    while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                    if c != ':' { panic!("Unexpected input on line {} while parsing demand augment speed modifier: first character after `s` must be a `:`, found `{}`", line_no, c); }

                    speed = 0;
                    let mut c = line.next().or(Some('#')).unwrap();
                    while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                    loop {
                      if c >= '0' && c <= '9' {
                        speed = (speed * 10) + ((c as u8) - ('0' as u8)) as u64; // This can lead to overflow fatal. :shrug:
                      } else if c == '#' || c == ' ' {
                        break;
                      } else {
                        panic!("Unexpected input on line {} while parsing demand augment speed modifier: speed value consists of digits, found `{}`", line_no, c);
                      }
                      c = line.next().or(Some('#')).unwrap();
                    }
                  }
                  'c' => {
                    // cooldown modifier
                    let mut c = line.next().or(Some('#')).unwrap();
                    while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                    if c != ':' { panic!("Unexpected input on line {} while parsing demand augment cooldown modifier: first character after `c` must be a `:`, found `{}`", line_no, c); }

                    cooldown = 0;
                    let mut c = line.next().or(Some('#')).unwrap();
                    while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                    loop {
                      if c >= '0' && c <= '9' {
                        cooldown = (cooldown * 10) + ((c as u8) - ('0' as u8)) as u64; // This can lead to overflow fatal. :shrug:
                      } else if c == '#' || c == ' ' {
                        break;
                      } else {
                        panic!("Unexpected input on line {} while parsing demand augment cooldown modifier: cooldown value consists of digits, found `{}`", line_no, c);
                      }
                      c = line.next().or(Some('#')).unwrap();
                    }
                  }
                  c => panic!("Unexpected input on line {} while parsing demand augment modifier: expecting `s`, `c`, '#', or EOL, found `{}`", line_no, c),
                }
              }

              let mut n = 1;
              // Find the nth demand. Not super optimal but at this scale no real issue.
              for coord in 0..FLOOR_CELLS_WH {
                if floor[coord].kind == CellKind::Demand {
                  if n == nth {
                    if options.trace_map_parsing { log!("Updating demand {} @{} with speed {} and cooldown {}", nth, coord, speed, cooldown); }
                    floor[coord].demand.speed = speed;
                    floor[coord].demand.cooldown = cooldown;
                    break;
                  }
                  n += 1;
                }
              }
            },
            'm' => {
              // m<n> = <i>{0..w*h} -> <o> [s:<d+>]
              // m1 = a..b.c -> d s:100
              // Note: zero inputs (empty pattern) are allowed. Dots are assumed to be "none". Parts will flow
              // in serial into the machine crafting pattern (left-right, top-bottom)
              let mut speed = 1;
              let mut wants: Vec<String> = vec!();

              // Parser:
              //    m3 = pear apple -> orange s:100 # end
              //     ^

              // Parse the machine ID, following the `m`
              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              if !((c >= '0' && c <= '9') || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) { panic!("Unexpected input on line {} while parsing machine augment: first character after `m` must be 1-9a-zA-Z, indicating which supply it targets, found `{}`", line_no, c); }
              // log!("machine id={}", c);

              // Find the machine meta
              let machine_id = c;
              let mut machine_index = machine_meta_data.len();
              machine_meta_data.iter().enumerate().any(|(i, (main_coord, max_coord, id, speed, pattern))| {
                if *id != machine_id {
                  return false;
                }
                machine_index = i;
                return true;
              });
              if machine_index == machine_meta_data.len() {
                // Do we need to fatal this? Should we? Could just ignore it, but .. not sure if we should.
                panic!("Parsed a machine modifier with id={} but did not encounter a machine cell with that id", machine_id);
              }
              // log!("machine index={}/{}", machine_index + 1, machine_meta_data.len());

              // Parser:
              //    m3 = pear apple -> orange s:100 # end
              //      ^

              // Skip pas the `=` sign
              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              if c != '=' { panic!("Unexpected input on line {} while parsing machine augment: first character after `m{}` must be the `=` sign, found `{}`", line_no, machine_id, c); }

              // Parser:
              //    m3 = pear apple -> orange s:100 # end
              //       ^
              c = line.next().or(Some('#')).unwrap();
              // Parser:
              //    m3 = pear apple -> orange s:100 # end
              //        ^

              // Parse the current input pattern, separated by spaces, until EOL, dash, or colon
              // The colon is a special case for the modifier `s:100`
              let words: Vec<String> = vec!();
              while c != '#' && c != '-' && c != ':' {
                // Parser:
                //    m3 = pear apple -> orange s:100 # end
                //        ^    ^     ^
                // Collect letters until spacing/EOL/colon/dash. They can form words or icons.
                let mut letters: Vec<char> = vec!();
                while c == ' ' || c == '.' { c = line.next().or(Some('#')).unwrap(); }
                // Parser:
                //    m3 = pear apple -> orange s:100 # end
                //         ^    ^     ^
                //    m3 = s:100 # end
                //         ^
                while c != '#' && c != '-' && c != ':' && c != '.' && c != ' ' {
                  if !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c as u8 > 127) { panic!("Unexpected input on line {} while parsing machine input: input characters must be a-zA-Z or dot or non-ascii, found `{}`", line_no, c); }
                  letters.push(c);
                  c = line.next().or(Some('#')).unwrap();
                }

                if letters.len() > 0 {
                  // Convert to a string. Deal with icon vs raw_name later
                  // log!("collected one input: {:?}", letters);
                  wants.push(letters.iter().collect::<String>());
                }
              }
              // Parser:
              //    m3 = pear apple -> orange s:100 # end
              //                    ^
              //    m3 = pear apple s:100 # end
              //                     ^

              // If the next char is a colon then the previous letter was not a part but a modifier
              // This would be for `m1 = a.b.c s:100`, without an arrow
              let mut modifier_edge_case = c == ':';
              if modifier_edge_case {
                // Pop the last word from the pattern. It should be "s" (or any other supported modifier).
                if wants.len() == 0 || wants.pop().unwrap() != "s".to_string() {
                  panic!("Unexpected colon (`:`) on line {} while parsing machine config line. Expecting a pattern, arrow, output, and then maybe a modifier. But modifiers should start with a letter, like `s:100`", line_no);
                }
              }

              // Parser:
              //    m3 = pear apple -> orange s:100 # end
              //                    ^
              //    m3 = pear apple s:100 # end
              //                     ^

              // Try to parse an arrow `->`
              if c == '-' {
                c = line.next().or(Some('#')).unwrap();
                while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                if c != '>' { panic!("Unexpected input on line {} while parsing machine augment: after input pattern an `->` arrow must follow and then the output, found `{}`", line_no, c); }

                // Skip past spacing after arrow
                c = line.next().or(Some('#')).unwrap();
                while c == ' ' { c = line.next().or(Some('#')).unwrap(); }

                // Skip past a word; the output for this machine. We ignore the value.
                while c != '#' && c != ' ' {
                  if !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c as u8 > 127) { panic!("Unexpected input on line {} while parsing machine output: input characters must be a-zA-Z or non-ascii, found `{}`", line_no, c); }
                  c = line.next().or(Some('#')).unwrap();
                }
                // We discard the output because we detect it based on the pattern, anyways.
              }

              // Now past the pattern, arrow, and output
              // We may not have parsed anything and only have seen a `s:` so far. Or nothing at all.

              // Parser:
              //    m3 = pear apple -> orange s:100 # end
              //                             ^
              //    m3 = pear apple s:100 # end
              //                     ^

              // Skip spaces
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }

              // Parser:
              //    m3 = pear apple -> orange s:100 # end
              //                              ^
              //    m3 = pear apple s:100 # end
              //                     ^

              // If there is a modifier, it would have to appear now
              if !modifier_edge_case {
                if c == 's' {
                  c = line.next().or(Some('#')).unwrap();
                  while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                  if c == ':' {
                    modifier_edge_case = true;
                  } else {
                    log!("map:");
                    log!("{}", str);
                    log!("line:");
                    log!("{:?}", line);
                    panic!("Unexpected input on line {} while parsing machine tail with modifier: found s but no colon, found `{}`", line_no, c);
                  }
                } else if c != '#' {
                  panic!("Unexpected input on line {} while parsing machine tail: expected `s` or `#`, found `{}`", line_no, c);
                }
              }

              // Parser:
              //    m3 = pear apple -> orange s:100 # end
              //                               ^
              //    m3 = pear apple s:100 # end
              //                     ^

              if modifier_edge_case {
                assert_eq!(c, ':', "pointer must now be at the colon");
                c = line.next().or(Some('#')).unwrap();
                while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                // Now start parsing digits. Only valid input is digits or spacing/eol.
                speed = 0;
                loop {
                  if c >= '0' && c <= '9' {
                    speed = (speed * 10) + ((c as u8) - ('0' as u8)) as u64; // This can lead to overflow fatal. :shrug:
                  } else if c == '#' || c == ' ' {
                    break;
                  } else {
                    panic!("Unexpected input on line {} while parsing machine augment speed modifier: speed value consists of digits, found `{}`", line_no, c);
                  }
                  c = line.next().or(Some('#')).unwrap();
                }
              }

              // Skip more spacing
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }

              // Must now be reaching the end of the line
              if c != '#' {
                panic!("Unexpected input on line {} after parsing line: expected # or EOL, found `{}`", line_no, c);
              }

              machine_meta_data[machine_index].3 = speed;
              machine_meta_data[machine_index].4 = wants.iter().map(|x| {
                let index = config.node_name_to_index.get(x).unwrap_or_else(| | {
                  log!("map:");
                  log!("{}", str);
                  log!("line:");
                  log!("{:?}", line);

                  log!("config.node_name_to_index: {:?}", config.node_name_to_index);
                  log!("looking for: {}", x);

                  panic!("pattern_by_name to index: what happened here: unlock name=`{}`", x)
                });
                return *index;
              }).collect::<Vec<PartKind>>();
            },
            '$' => {
              if options.trace_map_parsing { log!("Parsing $ unlocked parts list:"); }
              // Unlocked parts icons.
              // Expect a-zA-Z or >127. Spaces are skipped. Stops at EOL, EOF, or #
              loop {

                let c = line.next().or(Some('#')).unwrap();
                if options.trace_map_parsing { log!("unlocked part icon: `{}` : ascii {}", c, c as u32); }
                if c == '#' { break; }
                if c != ' ' {
                  if !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c as u8 > 127) { panic!("Unexpected input on line {} while parsing available parts ($): input characters must be a-zA-Z or non-ascii, found `{}` ({})", line_no, c, c as u8); }
                  if !unlocked_part_icons.contains(&c) { unlocked_part_icons.push(c); }
                }
              }
            }
            _ => panic!("Unexpected input on line {} while parsing input augments: wanted start of augment line, found `{}`", line_no, c),
          }
        }
      }
    }
  }

  if options.trace_map_parsing { log!("Machines after config phase: {:?}", machine_meta_data); }

  for ( main_coord, max_coord, id, speed, input_pattern ) in machine_meta_data.iter() {
    let (x1, y1) = to_xy(*main_coord);
    let (x2, y2) = to_xy(*max_coord);

    let w = x2 - x1 + 1;
    let h = y2 - y1 + 1;

    if options.trace_map_parsing { log!("Initializing machine @{}x@{} ({}): {}x{} with speed {}. The wants before normalization are: {:?}", main_coord, max_coord, id, w, h, speed, input_pattern); }
    assert!(*main_coord > 0, "each element should now represent a machine. before it was possible to be zero here.");

    let normalized_wants = machine_normalize_wants(&input_pattern);
    if options.trace_map_parsing { log!("The wants after normalization are: {:?}", normalized_wants); }

    let mut coords: Vec<usize> = vec!();
    for x in x1..=x2 {
      for y in y1..=y2 {
        coords.push(to_coord(x, y));
      }
    }

    if options.trace_map_parsing { log!("coords: {:?}", coords); }

    // Mark the machine grid of cells with the proper main_coord. This will cause the actual machine to take the proper shape.
    for x in x1..=x2 {
      for y in y1..=y2 {
        let coord = to_coord(x, y);
        floor[coord].kind = CellKind::Machine;
        floor[coord].machine.kind = if x == x1 && y == y1 { MachineKind::Main } else { MachineKind::SubBuilding };
        floor[coord].machine.main_coord = *main_coord;
        floor[coord].machine.id = *id;
        floor[coord].machine.cell_width = w;
        floor[coord].machine.cell_height = h;
        floor[coord].machine.coords = coords.clone();
      }
    }

    // Ensure that .wants and .haves are exactly as big as they cover cells.
    floor[*main_coord].machine.wants = coords.iter().enumerate().map(|(i, _)| part_from_part_kind(config, *input_pattern.get(i).unwrap_or(&CONFIG_NODE_PART_NONE))).collect::<Vec<Part>>();
    floor[*main_coord].machine.haves = coords.iter().map(|_| part_from_part_kind(config, CONFIG_NODE_PART_NONE)).collect::<Vec<Part>>();
    floor[*main_coord].machine.speed = *speed;

    let output_want = machine_discover_output_floor(options, state, config, &mut floor, *main_coord);
    floor[*main_coord].machine.output_want = part_from_part_kind(config, output_want);
  }

  // Set the .ins and .outs of each cell cause otherwise nothing happens.
  auto_ins_outs_floor(options, state, config, &mut floor);

  if options.trace_map_parsing { log!("-- end of str_to_floor2()"); }

  return (floor, unlocked_part_icons);
}

fn n_to_alnum(n: u8) -> char {
  return
    if n <= 9 { ('0' as u8 + n) as char }
    else if n <= 35 { ('a' as u8 + (n -10)) as char }
    else { ('A' as u8 + (n -36)) as char };
}

fn alnum_to_n(c: char) -> u8 {
  return
    if c >= '0' && c <= '9' {
      (c as u8) - ('0' as u8)
    } else if c >= 'a' && c <= 'z' {
      (c as u8) - ('a' as u8) + 10
    } else if c >= 'A' && c <= 'Z' {
      (c as u8) - ('A' as u8) + 36
    } else {
      panic!("Unexpected input while parsing index augment: first character after `s` or `d` must be a 0-9a-zA-Z indicating which supply/demand it targets, found `{}`", c);
    };
}
