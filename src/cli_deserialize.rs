use std::borrow::Borrow;
use std::collections::VecDeque;
use std::convert::TryInto;

use super::belt::*;
use super::cell::*;
use super::demand::*;
use super::factory::*;
use super::floor::*;
use super::direction::*;
use super::machine::*;
use super::offer::*;
use super::options::*;
use super::part::*;
use super::port::*;
use super::port_auto::*;
use super::state::*;
use super::supply::*;
use super::utils::*;

pub fn floor_from_str(str: String) -> ( [Cell; FLOOR_CELLS_WH], Vec<Offer> ) {
  if str.len() == 0 {
    return ( floor_empty(), vec!() );
  }

  // let ( floor, offers ) = str_to_floor(str);
  let ( floor, offers ) = str_to_floor2(str);
  return ( floor, offers );
}

fn str_to_floor2(str: String) -> ( [Cell; FLOOR_CELLS_WH], Vec<Offer> ) {
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

  log(format!("str_to_floor2:\n{}", str));

  let mut floor: [Cell; FLOOR_CELLS_WH] = floor_empty();
  let mut offers = vec!(); // TODO

  let hash: &char = &'#';
  let space: &u8 = &32u8;

  let mut lines = str.lines().collect::<Vec<&str>>();

  // Parse the first config line
  let mut lines = lines.iter_mut().map(|s| s.chars().peekable()).collect::<Vec<_>>();
  let mut lines = lines.iter_mut(); // hafta or the compiler complains

  let mut first_line = lines.next().unwrap(); // Bust if there's no input.
  log(format!("first First line: {:?}", first_line));
  loop {
    while first_line.peek().or(Some(&'#')).unwrap() == &' ' { first_line.next(); }
    // Keep skipping lines that start with comments and empty lines (only containing spaces)
    if first_line.peek().or(Some(&'#')).unwrap() != &'#' {
      break;
    }
    first_line = lines.next().unwrap(); // Bust if there's no more input.
  }
  log(format!("First line: {:?}", first_line));

  // We should now be at the start of the first non-comment line.
  // It is asserted to be the map header line.

  // The header line is currently only expecting one modifier: the floor dimensions

  let width = FLOOR_CELLS_W;
  let height = FLOOR_CELLS_H;
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
        log(format!("Map size: {} x {}", w, h));
      }
      _ => {}
    }
  }

  let mut machine_main_coords: [usize; 63] = [0; 63];

  // Expect as many cells as specified.
  for j in 0..height {

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

    for i in 0..width {
      // For each of the three lines, step them cell by cell, each cell consisting of 9 chars.
      // Characters are expected to be consecutive in sync across the three lines, even if they
      // had an inconsistent amount of leading spaces.

      let coord = to_coord(i, j);

      let _a = line1.next().or(Some('#')).unwrap();
      let b = line1.next().or(Some('#')).unwrap();
      let _c = line1.next().or(Some('#')).unwrap();
      let d = line2.next().or(Some('#')).unwrap();
      let e = line2.next().or(Some('#')).unwrap();
      let f = line2.next().or(Some('#')).unwrap();
      let _g = line3.next().or(Some('#')).unwrap();
      let h = line3.next().or(Some('#')).unwrap();
      let _i = line3.next().or(Some('#')).unwrap();

      // log(format!("{}x{}:\n   {} {} {}\n   {} {} {}\n   {} {} {}", i, j, _a, b, _c, d, e, f, _g, h, _i));

      let cell_kind = e;
      let port_u = b;
      let port_r = f;
      let port_d = h;
      let port_l = d;
      // Suppliers and demanders are at the edge. When they are left or right, the kind is printed
      // under the center. Otherwise the kind is printed to the right of the center.
      let supply_demand_kind = if i == 0 || i == width-1 { h } else { f };

      fn add_machine(floor: &mut [Cell; FLOOR_CELLS_WH], coord: usize, x: usize, y: usize, cell_kind: char, machine_main_coords: &mut [usize; 63], port_u: char, port_r: char, port_d: char, port_l: char) {
        // Auto layout will have to reconcile the individual machine parts into one machine
        // Any modifiers as well as the input and output parameters of this machine are
        // listed below the floor model. Expect them to be filled in later.
        let mn = (cell_kind as u8 - ('0' as u8)) as usize;
        let main_coord =
          if machine_main_coords[mn] == 0 {
            machine_main_coords[mn] = coord;
            coord
          } else {
            machine_main_coords[mn]
          };
        let mut cell = machine_any_cell(cell_kind as char, x, y, 1, 1, MachineKind::Unknown, vec!(), part_c(' '), 1, 1, 1);
        cell.port_u = match port_u as char { '^' => Port::Outbound, 'v' => Port::Inbound, '?' => Port::Unknown, ' ' => Port::None, '─' => Port::None, '│' => Port::None, _ => panic!("Port up indicators must be `^`, `v`, `?` or a space, this was `{}`", port_u)};
        cell.port_r = match port_r as char { '>' => Port::Outbound, '<' => Port::Inbound, '?' => Port::Unknown, ' ' => Port::None, '─' => Port::None, '│' => Port::None, _ => panic!("Port right indicators must be `<`, `>`, `?` or a space, this was `{}`", port_u)};
        cell.port_d = match port_d as char { 'v' => Port::Outbound, '^' => Port::Inbound, '?' => Port::Unknown, ' ' => Port::None, '─' => Port::None, '│' => Port::None, _ => panic!("Port down indicators must be `^`, `v`, `?` or a space, this was `{}`", port_u)};
        cell.port_l = match port_l as char { '<' => Port::Outbound, '>' => Port::Inbound, '?' => Port::Unknown, ' ' => Port::None, '─' => Port::None, '│' => Port::None, _ => panic!("Port left indicators must be `<`, `>`, `?` or a space, this was `{}`", port_u)};
        floor[coord] = cell;
      }

      match cell_kind as char {
        's' => {
          if is_middle(i, j) {
            add_machine(&mut floor, coord, i, j, cell_kind, &mut machine_main_coords, port_u, port_r, port_d, port_l);
          } else {
            let ( port_u, port_r, port_d, port_l ) =
              if j == 0 {
                ( Port::None, Port::None, Port::Outbound, Port::None )
              } else if i == width-1 {
                ( Port::None, Port::Outbound, Port::None, Port::None )
              } else if j == height-1 {
                ( Port::Outbound, Port::None, Port::None, Port::None )
              } else if i == 0 {
                ( Port::None, Port::None, Port::None, Port::Outbound )
              } else {
                panic!("Error parsing floor cell: Encountered an `s` inside the floor; this should be a Supply, which is bound to the edge");
              };
            // The speed and cooldown of the supply have to be added below the floor so use placeholder values for now; TODO: wire that up
            log(format!("Supply, gives `{}`", supply_demand_kind as char));
            let cell = supply_cell(i, j, part_c(supply_demand_kind as char), 1, 1, 1);
            floor[coord] = cell;
          }
        },

        'd' => {
          if is_middle(i, j) {
            add_machine(&mut floor, coord, i, j, cell_kind, &mut machine_main_coords, port_u, port_r, port_d, port_l);
          } else {
            let ( port_u, port_r, port_d, port_l ) =
            if j == 0 {
              ( Port::None, Port::None, Port::Outbound, Port::None )
            } else if i == width-1 {
              ( Port::None, Port::Outbound, Port::None, Port::None )
            } else if j == height-1 {
              ( Port::Outbound, Port::None, Port::None, Port::None )
            } else if i == 0 {
              ( Port::None, Port::None, Port::None, Port::Outbound )
            } else {
              panic!("Error parsing floor cell: Encountered an `d` inside the floor; this should be a Demand, which is bound to the edge");
            };
            log(format!("Demand, takes `{}`", supply_demand_kind as char));
            let cell = demand_cell(i, j, part_c(supply_demand_kind as char));
            floor[coord] = cell;
          }
        },

        // Empty cells are spaces or dots
        | ' '
        | '.'
        => {
          floor[coord] = empty_cell(i, j);
        }

        // Then there's a bunch of belt cells. These are either `b` or "table ascii art" chars
        // The actual char is not relevant here since we will auto-discover the meta of the cell
        // based on the port configuration, anyways.
        '%' => { // Joker, unspecified belt. (b is used by machine id)
          // Fix belt meta later in the auto layout step
          let mut cell = belt_cell(i, j, BELT_UNKNOWN);
          cell.port_u = Port::Unknown;
          cell.port_r = Port::Unknown;
          cell.port_d = Port::Unknown;
          cell.port_l = Port::Unknown;
          floor[coord] = cell;
        }
        | '╸' // double lines have no one-arm glyph :rolls-eys: so we use thick line instead
        | '╹'
        | '╺'
        | '╻'
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
          let mut cell = belt_cell(i, j, BELT_UNKNOWN);
          cell.port_u = match port_u as char { '^' => Port::Outbound, 'v' => Port::Inbound, '?' => Port::Unknown, ' ' => Port::None, '─' => Port::None, '│' => Port::None, _ => panic!("Port up indicators must be `^`, `v`, `?` or a space, this was `{}`", port_u)};
          cell.port_r = match port_r as char { '>' => Port::Outbound, '<' => Port::Inbound, '?' => Port::Unknown, ' ' => Port::None, '─' => Port::None, '│' => Port::None, _ => panic!("Port right indicators must be `<`, `>`, `?` or a space, this was `{}`", port_u)};
          cell.port_d = match port_d as char { 'v' => Port::Outbound, '^' => Port::Inbound, '?' => Port::Unknown, ' ' => Port::None, '─' => Port::None, '│' => Port::None, _ => panic!("Port down indicators must be `^`, `v`, `?` or a space, this was `{}`", port_u)};
          cell.port_l = match port_l as char { '<' => Port::Outbound, '>' => Port::Inbound, '?' => Port::Unknown, ' ' => Port::None, '─' => Port::None, '│' => Port::None, _ => panic!("Port left indicators must be `<`, `>`, `?` or a space, this was `{}`", port_u)};
          floor[coord] = cell;
        }

        _ => {
          if (cell_kind >= '1' && cell_kind <= '9') || (cell_kind >= 'a' && cell_kind <= 'z') || (cell_kind >= 'A' && cell_kind <= 'Z') {
            // Machine id is a single char 1-9a-zA-Z
            // Note: s and d are special cased between middle and edge cell above
            add_machine(&mut floor, coord, i, j, cell_kind, &mut machine_main_coords, port_u, port_r, port_d, port_l);
          } else {
            panic!("Error while parsing factory string: Encountered an unknown center cell char at {}x{}: `{}` ({})", i, j, cell_kind as char, cell_kind)
          }
        },
      }
    }
  }

  // Keep parsing config lines while skipping comments. These are optional and augment
  // things on the floor or populate the list of offers.
  loop {
    match lines.next() {
      None => break,
      Some(line) => {
        log(format!("Next line: {:?}", line));
        while line.peek().or(Some(&'#')).unwrap() == &' ' { line.next(); }
        // Keep skipping lines that start with comments and empty lines (only containing spaces)
        if line.peek().or(Some(&'#')).unwrap() != &'#' {
          let c = line.next().or(Some('#')).unwrap();
          match c {
            's' => {
              // s<n> = <p> [s:<d+>] [c:<d+>]
              // s1 = w s:100 c:100
              let mut nth = 0;
              let mut speed = 1;
              let mut cooldown = 1;
              let mut gives = 't';

              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              if c < '0' || c > '9' { panic!("Unexpected input while parsing supply augment: first character after `s` must be a digit indicating which supply it targets, found `{}`", c); }
              nth = (c as u8) - ('0' as u8);

              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              if c != '=' { panic!("Unexpected input while parsing supply augment: first character after `s{}` must be the `=` sign, found `{}`", nth, c); }

              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              if c < 'a' && c > 'z' { panic!("Unexpected input while parsing supply augment kind: input characters must be a-z, found `{}`", c); }
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
                    if c != ':' { panic!("Unexpected input while parsing supply augment speed modifier: first character after `s` must be a `:`, found `{}`", c); }

                    speed = 0;
                    let mut c = line.next().or(Some('#')).unwrap();
                    while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                    loop {
                      if c >= '0' && c <= '9' {
                        speed = (speed * 10) + ((c as u8) - ('0' as u8)) as u64; // This can lead to overflow fatal. :shrug:
                      } else if c == '#' || c == ' ' {
                        break;
                      } else {
                        panic!("Unexpected input while parsing supply augment speed modifier: speed value consists of digits, found `{}`", c);
                      }
                      c = line.next().or(Some('#')).unwrap();
                    }
                  }
                  'c' => {
                    // cooldown modifier
                    let mut c = line.next().or(Some('#')).unwrap();
                    while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                    if c != ':' { panic!("Unexpected input while parsing supply augment cooldown modifier: first character after `c` must be a `:`, found `{}`", c); }

                    cooldown = 0;
                    let mut c = line.next().or(Some('#')).unwrap();
                    while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                    loop {
                      if c >= '0' && c <= '9' {
                        cooldown = (cooldown * 10) + ((c as u8) - ('0' as u8)) as u64; // This can lead to overflow fatal. :shrug:
                      } else if c == '#' || c == ' ' {
                        break;
                      } else {
                        panic!("Unexpected input while parsing supply augment cooldown modifier: cooldown value consists of digits, found `{}`", c);
                      }
                      c = line.next().or(Some('#')).unwrap();
                    }
                  }
                  c => panic!("Unexpected input while parsing supply augment modifier: expecting `s`, `c`, '#', or EOL, found `{}`", c),
                }
              }

              let mut n = 1;
              // Find the nth supply. Not super optimal but at this scale no real issue.
              for coord in 0..FLOOR_CELLS_WH {
                if floor[coord].kind == CellKind::Supply {
                  if n == nth {
                    log(format!("Updating supply {} @{} with part {} and speed {} and cooldown {}", nth, coord, gives, speed, cooldown));
                    floor[coord].supply.speed = speed;
                    floor[coord].supply.cooldown = cooldown;
                    floor[coord].supply.gives = part_c(gives);
                    break;
                  }
                  n += 1;
                }
              }
            },
            'd' => {
              // d<n> = <p>
              // d1 = w
              let mut nth = 0;
              let mut takes = 't';

              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              if c < '0' || c > '9' { panic!("Unexpected input while parsing demand augment: first character after `s` must be a digit indicating which supply it targets, found `{}`", c); }
              nth = (c as u8) - ('0' as u8);

              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              if c != '=' { panic!("Unexpected input while parsing demand augment: first character after `s{}` must be the `=` sign, found `{}`", nth, c); }

              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              if c < 'a' && c > 'z' { panic!("Unexpected input while parsing supply augment kind: input characters must be a-z, found `{}`", c); }
              takes = c;

              let mut n = 1;
              // Find the nth supply. Not super optimal but at this scale no real issue.
              for coord in 0..FLOOR_CELLS_WH {
                if floor[coord].kind == CellKind::Demand {
                  if n == nth {
                    log(format!("Updating demand {} @{} with part {}", nth, coord, takes));
                    floor[coord].demand.part = part_c(takes);
                    break;
                  }
                  n += 1;
                }
              }
            },
            'm' => {
              // m<n> = <i>{0..w*h} -> <o> [s:<d+>]
              // m1 = a..b.c -> d s:100
              // Note: zero inputs are allowed. Dots are assumed to be "none". Parts will flow
              // in serial into the machine crafting pattern (left-right, top-bottom)
              let mut nth = 0;
              let mut speed = 1;
              let mut wants: Vec<Part> = vec!();
              let mut output = 't';

              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              if !((c >= '1' && c <= '9') || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) { panic!("Unexpected input while parsing machine augment: first character after `m` must be 1-9a-zA-Z, indicating which supply it targets, found `{}`", c); }
              nth = (c as u8) - ('0' as u8);

              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              if c != '=' { panic!("Unexpected input while parsing machine augment: first character after `m{}` must be the `=` sign, found `{}`", nth, c); }

              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              while c != '#' && c != '-' {
                if !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '.') { panic!("Unexpected input while parsing machine input: input characters must be a-zA-Z or dot, found `{}`", c); }
                // Convert the dot back to an empty part.
                wants.push(part_c(if c == '.' { ' ' } else { c }));

                c = line.next().or(Some('#')).unwrap();
                while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              }

              if c != '-' { panic!("Unexpected input while parsing machine augment: after input must follow an `->` arrow and then the output, found `{}`", c); }

              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              if c != '>' { panic!("Unexpected input while parsing machine augment: after input must follow an `->` arrow and then the output, found `{}`", c); }

              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
              if !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '.') { panic!("Unexpected input while parsing machine output: input characters must be a-zA-Z or dot, found `{}`", c); }
              output = c;

              loop {
                let mut c = line.next().or(Some('#')).unwrap();
                while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                match c {
                  '#' => break, // EOL or start of line comment
                  's' => {
                    // speed modifier
                    let mut c = line.next().or(Some('#')).unwrap();
                    while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                    if c != ':' { panic!("Unexpected input while parsing machine augment speed modifier: first character after `s` must be a `:`, found `{}`", c); }

                    speed = 0;
                    let mut c = line.next().or(Some('#')).unwrap();
                    while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                    loop {
                      if c >= '0' && c <= '9' {
                        speed = (speed * 10) + ((c as u8) - ('0' as u8)) as u64; // This can lead to overflow fatal. :shrug:
                      } else if c == '#' || c == ' ' {
                        break;
                      } else {
                        panic!("Unexpected input while parsing machine augment speed modifier: speed value consists of digits, found `{}`", c);
                      }
                      c = line.next().or(Some('#')).unwrap();
                    }
                  }
                  c => panic!("Unexpected input while parsing machine augment modifier: expecting `s`, '#', or EOL, found `{}`", c),
                }
              }

              let main_coord = machine_main_coords[nth as usize];
              if main_coord > 0 {
                let want_icons = wants.iter().map(|Part { icon, .. }| icon).collect::<Vec<&char>>();
                log(format!("Updating machine {} @{} with inputs {:?} and output {} at speed {}", nth, main_coord, want_icons, output, speed));

                // Note: auto discovery will have to make sure that wants.len and haves.len are equal and at least >= w*h
                floor[main_coord].machine.wants = wants;
                floor[main_coord].machine.output_want = part_c(output);
                floor[main_coord].machine.speed = speed;
              } else {
                log(format!("Machine {} was defined as having inputs {:?} and output {} at speed {} but its main_coord was not found", nth, wants, output, speed));
              }
            },
            'o' => {
              let mut c = line.next().or(Some('#')).unwrap();
              while c == ' ' { c = line.next().or(Some('#')).unwrap(); }

              // What kind of offer?
              match c {
                's' => {
                  // os = <p> [s:<d+>] [c:<d+>]
                  // os = w s:100 c:100
                  let mut speed = 1;
                  let mut cooldown = 1;
                  let mut gives = 't';

                  let mut c = line.next().or(Some('#')).unwrap();
                  while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                  if c != '=' { panic!("Unexpected input while parsing offer supply augment: first character after `os` must be the `=` sign, found `{}`", c); }

                  let mut c = line.next().or(Some('#')).unwrap();
                  while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                  if c < 'a' && c > 'z' { panic!("Unexpected input while parsing supply augment kind: input characters must be a-z, found `{}`", c); }
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
                        if c != ':' { panic!("Unexpected input while parsing supply augment speed modifier: first character after `s` must be a `:`, found `{}`", c); }

                        speed = 0;
                        let mut c = line.next().or(Some('#')).unwrap();
                        while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                        loop {
                          if c >= '0' && c <= '9' {
                            speed = (speed * 10) + ((c as u8) - ('0' as u8)) as u64; // This can lead to overflow fatal. :shrug:
                          } else if c == '#' || c == ' ' {
                            break;
                          } else {
                            panic!("Unexpected input while parsing supply augment speed modifier: speed value consists of digits, found `{}`", c);
                          }
                          c = line.next().or(Some('#')).unwrap();
                        }
                      }
                      'c' => {
                        // cooldown modifier
                        let mut c = line.next().or(Some('#')).unwrap();
                        while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                        if c != ':' { panic!("Unexpected input while parsing supply augment cooldown modifier: first character after `c` must be a `:`, found `{}`", c); }

                        cooldown = 0;
                        let mut c = line.next().or(Some('#')).unwrap();
                        while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                        loop {
                          if c >= '0' && c <= '9' {
                            cooldown = (cooldown * 10) + ((c as u8) - ('0' as u8)) as u64; // This can lead to overflow fatal. :shrug:
                          } else if c == '#' || c == ' ' {
                            break;
                          } else {
                            panic!("Unexpected input while parsing supply augment cooldown modifier: cooldown value consists of digits, found `{}`", c);
                          }
                          c = line.next().or(Some('#')).unwrap();
                        }
                      }
                      c => panic!("Unexpected input while parsing supply augment modifier: expecting `s`, `c`, '#', or EOL, found `{}`", c),
                    }
                  }

                  log(format!("- Creating Supply Offer that gives `{}`", gives));
                  offers.push(Offer {
                    kind: CellKind::Supply,
                    cell_width: 0,
                    cell_height: 0,
                    supply_icon: gives,
                    demand_icon: ' ',
                    wants: vec!(),
                    machine_output: ' ',
                    speed,
                    cooldown
                  });
                },
                'd' => {
                  // od = <p>
                  // od = w
                  let mut takes = 't';

                  let mut c = line.next().or(Some('#')).unwrap();
                  while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                  if c != '=' { panic!("Unexpected input while parsing offer demand augment: first character after `od` must be the `=` sign, found `{}`", c); }

                  let mut c = line.next().or(Some('#')).unwrap();
                  while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                  if c < 'a' && c > 'z' { panic!("Unexpected input while parsing demand augment kind: input characters must be a-z, found `{}`", c); }
                  takes = c;

                  log(format!("- Creating Demand Offer that wants `{}`", takes));
                  offers.push(Offer {
                    kind: CellKind::Demand,
                    cell_width: 0,
                    cell_height: 0,
                    supply_icon: ' ',
                    demand_icon: takes,
                    wants: vec!(),
                    machine_output: ' ',
                    speed: 1,
                    cooldown: 1,
                  });
                },
                'm' => {
                  // om = <i>[i][i] -> <o> [s:<d+>]
                  // om = abc -> d s:100
                  let mut speed = 1;
                  let mut wants = vec!();
                  let mut output = 't';
                  let mut width = 1;
                  let mut height = 1;

                  let mut c = line.next().or(Some('#')).unwrap();
                  while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                  if c != '=' { panic!("Unexpected input while parsing machine offer: first character after `om` must be the `=` sign, found `{}`", c); }

                  let mut c = line.next().or(Some('#')).unwrap();
                  while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                  while c != '#' && c != '-' {
                    if !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '.') { panic!("Unexpected input while parsing machine augment input2: input characters must be a-z, found `{}`", c); }
                    // Convert the dot back to an empty part.
                    wants.push(part_c(if c == '.' { ' ' } else { c }));

                    c = line.next().or(Some('#')).unwrap();
                    while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                  }

                  if c != '-' { panic!("Unexpected input while parsing machine offer: after input must follow an `->` arrow and then the output, found `{}`", c); }

                  let mut c = line.next().or(Some('#')).unwrap();
                  while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                  if c != '>' { panic!("Unexpected input while parsing machine offer: after input must follow an `->` arrow and then the output, found `{}`", c); }

                  let mut c = line.next().or(Some('#')).unwrap();
                  while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                  if c < 'a' && c > 'z' { panic!("Unexpected input while parsing machine offer output: output characters must be a-z, found `{}`", c); }
                  output = c;

                  loop {
                    let mut c = line.next().or(Some('#')).unwrap();
                    while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                    match c {
                      '#' => break, // EOL or start of line comment
                      's' => {
                        // speed modifier
                        let mut c = line.next().or(Some('#')).unwrap();
                        while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                        if c != ':' { panic!("Unexpected input while parsing machine offer speed modifier: first character after `s` must be a `:`, found `{}`", c); }

                        speed = 0;
                        let mut c = line.next().or(Some('#')).unwrap();
                        while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                        loop {
                          if c == '#' || c == ' ' {
                            break;
                          }
                          if c < '0' && c > '9' {
                            panic!("Unexpected input while parsing machine offer speed modifier: speed value consists of digits, found `{}`", c);
                          }
                          speed = (speed * 10) + ((c as u8) - ('0' as u8)) as u64; // This can lead to overflow fatal. :shrug:
                          c = line.next().or(Some('#')).unwrap();
                        }
                      }
                      'd' => {
                        // dimensions of the machine offer modifier
                        let mut c = line.next().or(Some('#')).unwrap();
                        while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                        if c != ':' { panic!("Unexpected input while parsing machine offer dimension modifier: first character after `s` must be a `:`, found `{}`", c); }

                        width = 0;
                        height = 0;
                        let mut c = line.next().or(Some('#')).unwrap();
                        while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                        loop {
                          if c == ' ' {
                            let mut c = line.next().or(Some('#')).unwrap();
                            while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                            if c == 'x' {
                              break;
                            }
                            panic!("Unexpected input while parsing machine offer dimension modifier: dimension value must be followed by `x`, found `{}`", c);
                          }
                          if c == 'x' {
                            break;
                          }
                          if c < '0' && c > '9' {
                            panic!("Unexpected input while parsing machine offer dimension modifier: dimension value consists of digits, found `{}`", c);
                          }
                          width = (width * 10) + ((c as u8) - ('0' as u8)) as u64; // This can lead to overflow fatal. :shrug:
                          c = line.next().or(Some('#')).unwrap();

                          if c >= '0' && c <= '9' {
                            width = (width * 10) + ((c as u8) - ('0' as u8)) as u64; // This can lead to overflow fatal. :shrug:
                          } else if c == 'x' {
                            break;
                          } else {
                            panic!("Unexpected input while parsing machine offer dimension modifier: the first set of digits should be followed by an `x`, found `{}`", c);
                          }
                          c = line.next().or(Some('#')).unwrap();
                        }
                        let mut c = line.next().or(Some('#')).unwrap();
                        while c == ' ' { c = line.next().or(Some('#')).unwrap(); }
                        loop {
                          if c >= '0' && c <= '9' {
                            height = (height * 10) + ((c as u8) - ('0' as u8)) as u64; // This can lead to overflow fatal. :shrug:
                          } else if c == ' ' || c == '#' {
                            break;
                          } else {
                            panic!("Unexpected input while parsing machine offer dimension modifier: the second set of digits should be followed a space or the end of the line, found `{}`", c);
                          }
                          c = line.next().or(Some('#')).unwrap();
                        }
                      }
                      c => panic!("Unexpected input while parsing machine offer modifier: expecting `s`, '#', or EOL, found `{}`", c),
                    }
                  }

                  let cw = (width * height) as usize;
                  for i in 0..cw {
                    if wants.len() < cw {
                      wants.push(part_none());
                    }
                  }

                  let output = wants_discover_output(&wants, width as usize, height as usize);

                  offers.push(Offer {
                    kind: CellKind::Machine,
                    cell_width: width as usize,
                    cell_height: height as usize,
                    supply_icon: ' ',
                    demand_icon: ' ',
                    wants,
                    machine_output: output.icon,
                    speed,
                    cooldown: 0
                  });
                },
                c => panic!("Unexpected input while parsing offer kind: expecting `s`, `d`, or `m`, found `{}`", c),
              }
            },
            _ => panic!("Unexpected input while parsing input augments: wanted start of augment line, found `{}`", c),
          }
        }
      }
    }
  }

  // Set the .ins and .outs of each cell cause otherwise nothing happens.
  auto_ins_outs_floor(&mut floor);

  return ( floor, offers );
}
