use std::borrow::Borrow;
use std::convert::TryInto;

use super::belt::*;
use super::cell::*;
use super::demand::*;
use super::factory::*;
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

// Edge cells can only be demand/supply/empty, other cells can only be belt/machine/empty
pub fn floor_empty() -> [Cell; FLOOR_CELLS_WH] {
  // https://stackoverflow.com/questions/67822062/fixed-array-initialization-without-implementing-copy-or-default-trait/67824946#67824946
  // :shrug: okay
  return (0..FLOOR_CELLS_WH)
    .map(|coord| {
      let (x, y) = to_xy(coord);
      empty_cell(x, y)
    })
    .collect::<Vec<Cell>>()
    .try_into() // runtime error if bad but this is fine
    .unwrap();
}

pub fn floor_from_str(str: String) -> ( [Cell; FLOOR_CELLS_WH], Vec<Offer> ) {
  if str.len() == 0 {
    return ( floor_empty(), vec!() );
  }

  let ( floor, offers ) = str_to_floor(str);
  return ( floor, offers );
}

fn str_to_floor(str: String) -> ( [Cell; FLOOR_CELLS_WH], Vec<Offer> ) {
  // Given a string in a grid format, generate a floor
  // String floor must have equal with/height as hardcoded size, for now (11x11).
  // Empty cell is space or dot, s = supply, d = demand, b = belt, m = machine. Auto-layout should fix it.
  // Example string (inc newlines, ex one space indent after comment start):
  // ```
  // ........s........
  // .111....b........
  // .111bbbbbbbbbbb..
  // .111..........b..
  // ..b...........b..
  // ..bbbbbbb.....b..
  // ........b.....b..
  // .......222....b..
  // .......222....b..
  // .......222....b..
  // ........b.....bbs
  // ..bbbbbbb........
  // ..b.....b........
  // ..b.....b........
  // dbb..bbbb........
  // .....b...........
  // .....d...........
  // m1 = ws -> b s:50
  // m2 = b -> g
  // s1 = w s:100
  // s2 = s c:50
  // d1 = g
  // d2 = g
  // os = g
  // ```
  // Forming something like this:
  // ┌────────────────────┐
  // │         s1         │
  // │ ┌────────────────┐ │
  // │ │111    ║        │ │
  // │ │111════╩═════╗  │ │
  // │ │111          ║  │ │
  // │ │ ║           ║  │ │
  // │ │ ╚═════╗     ║  │ │
  // │ │       ║     ║  │ │
  // │ │      222    ║  │ │
  // │ │      222    ║  │ │
  // │ │      222    ║  │ │
  // │ │       ║     ║  │ │
  // │ │ ╔═════╣     ╚═s│s│
  // │ │ ║     ║        │2│
  // │ │ ║     ║        │ │
  // │d│═╝  ╔══╝        │ │
  // │1│    ║           │ │
  // │ └────────────────┘ │
  // │      d2            │
  // └────────────────────┘
  // Offers: supply that gives 'g' with default speed and delay

  let mut len = 0;
  for c in str.bytes() {
    if (c as char) != '\n' {
      len += 1;
    }
  }

  // if len != FLOOR_CELLS_WH {
  //   panic!("Error: input string (ignoring newlines) must be exactly {}x{}={} chars, but had {} chars", FLOOR_CELLS_W, FLOOR_CELLS_H, FLOOR_CELLS_WH, len);
  // }

  let lines = str.lines().collect::<Vec<&str>>();

  println!("Importing string map:\n```\n{}\n```", str);

  let defs = &lines[FLOOR_CELLS_H..lines.len()];
  println!("defs: {:?}", defs);

  let mut offers: Vec<Offer> = vec!();
  for line in lines[FLOOR_CELLS_H..lines.len()].iter().map(|s| s.bytes()).collect::<Vec<_>>().iter_mut() {
    if line.next().or(Some('-' as u8)).unwrap() as char == 'o' {
      // os = x         Offer of a supply that gives x
      // od = x         Offer of a demand that takes x
      // om = ab -> c    Offer of a machine that takes a and b and generates c
      // suppliers and machines can have c: and s: modifiers after them, setting cooldown and speed

      match line.next().or(Some('-' as u8)).unwrap() as char {
        's' => {
          let mut speed: u64 = 100;
          let mut cooldown: u64 = 50;
          let kind_icon = loop {
            // Skip spaces and equal signs. Stop if it hits EOL.
            let d = line.next().or(Some('?' as u8)).unwrap() as char;
            if d != ' ' && d != '=' && d != '-' {
              break d;
            }
          };
          // Now look for s:xxx and c:xxx
          loop {
            // Skip spaces. Stop if it hits EOL.
            let modifier = line.next().or(Some('?' as u8)).unwrap() as char;
            if modifier != ' ' {
              if modifier == 's' && (line.next().or(Some('?' as u8)).unwrap() as char) == ':' {
                // parse the value for speed
                speed = 0;

                loop {
                  // Skip spaces and equal signs. Stop if it hits EOL.
                  let d = line.next().or(Some('?' as u8)).unwrap() as char;
                  if d >= '0' && d <= '9' {
                    speed = speed * 10 + (d as u64 - '0' as u64);
                  } else {
                    break;
                  }
                }
                if speed == 0 {
                  // This means the value was zero or more zeroes. Set it to one, shrug.
                  speed = 1;
                }
              }
              else if modifier == 'c' && (line.next().or(Some('?' as u8)).unwrap() as char) == ':' {
                // parse the value for speed
                cooldown = 0;

                loop {
                  // Skip spaces and equal signs. Stop if it hits EOL.
                  let d = line.next().or(Some('?' as u8)).unwrap() as char;
                  if d >= '0' && d <= '9' {
                    cooldown = cooldown * 10 + (d as u64 - '0' as u64);
                  } else {
                    break;
                  }
                }
                if cooldown == 0 {
                  // This means the value was zero or more zeroes. Set it to one, shrug.
                  cooldown = 1;
                }
              }
              else {
                // Keep parsing while we find s: and c: modifiers
                break;
              }
            }
          };

          // The store kind should be in kind_icon, or it is `-` for EOL
          let offer = Offer {
            kind: CellKind::Supply,
            supply_icon: kind_icon,
            demand_icon: ' ',
            machine_input1: ' ',
            machine_input2: ' ',
            machine_input3: ' ',
            machine_output: ' ',
            speed,
            cooldown,
          };
          // log(format!("Parsed an offer: {:?}", offer));
          offers.push(offer);
        }
        'd' => {
          let kind_icon = loop {
            // either until it hits EOL or it hits a space or an eq sign
            let d = line.next().or(Some('?' as u8)).unwrap() as char;
            if d != ' ' && d != '=' && d != '-' {
              break d;
            }
          };
          // The store kind should be in kind_icon, or it is `-` for EOL
          let offer = Offer {
            kind: CellKind::Demand,
            supply_icon: ' ',
            demand_icon: kind_icon,
            machine_input1: ' ',
            machine_input2: ' ',
            machine_input3: ' ',
            machine_output: ' ',
            speed: 1,
            cooldown: 1,
          };
          // log(format!("Parsed an offer: {:?}", offer));
          offers.push(offer);
        }
        'm' => {
          let mut speed: u64 = 100;
          // we can ignore the = but we have to parse at least 3 characters for inputs, or up
          // to the dash. After that one more for the output. Ignore the rest.
          let input1_icon = loop {
            // either until it hits EOL or it hits a space or an eq sign
            let d = line.next().or(Some('?' as u8)).unwrap() as char;
            if d != ' ' && d != '=' {
              if d == '-' {
                break ' ';
              }
              break d;
            }
          };
          let input2_icon = if input1_icon == ' ' { ' ' } else {
            loop {
              // either until it hits EOL or it hits a space or an eq sign
              let d = line.next().or(Some('?' as u8)).unwrap() as char;
              if d != ' ' {
                if d == '-' {
                  break ' ';
                }
                break d;
              }
            }
          };
          let input3_icon = if input2_icon == ' ' { ' ' } else {
            loop {
              // either until it hits EOL or it hits a space or an eq sign
              let d = line.next().or(Some('?' as u8)).unwrap() as char;
              if d != ' ' {
                if d == '-' {
                  break ' ';
                }
                break d;
              }
            }
          };
          let output_icon = loop {
            // either until it hits EOL or it hits a space or an eq sign
            let d = line.next().or(Some('?' as u8)).unwrap() as char;
            if d != ' ' && d != '-' && d != '>' {
              break d;
            }
          };
          loop {
            // Skip spaces. Stop if it hits EOL.
            let modifier = line.next().or(Some('?' as u8)).unwrap() as char;
            if modifier != ' ' {
              if modifier == 's' && (line.next().or(Some('?' as u8)).unwrap() as char) == ':' {
                // parse the value for speed
                speed = 0;

                loop {
                  // Skip spaces and equal signs. Stop if it hits EOL.
                  let d = line.next().or(Some('?' as u8)).unwrap() as char;
                  if d >= '0' && d <= '9' {
                    speed = speed * 10 + (d as u64 - '0' as u64);
                  } else {
                    break;
                  }
                }
                if speed == 0 {
                  // This means the value was zero or more zeroes. Set it to one, shrug.
                  speed = 1;
                }
              }
              else {
                // Keep parsing while we find s:
                break;
              }
            }
          };
          // The store kind should be in kind_icon, or it is `-` for EOL
          let offer = Offer {
            kind: CellKind::Machine,
            supply_icon: ' ',
            demand_icon: ' ',
            machine_input1: input1_icon,
            machine_input2: input2_icon,
            machine_input3: input3_icon,
            machine_output: output_icon,
            speed,
            cooldown: 1,
          };
          // log(format!("Parsed an offer: {:?}", offer));
          offers.push(offer);
        }
        _ => ()
      }
    }
  }

  let mut suppliers: u8 = '0' as u8;
  let mut demanders: u8 = '0' as u8;

  let floor: [Cell; FLOOR_CELLS_WH] = lines[0..FLOOR_CELLS_H].iter().map(|s| s.bytes()).flatten().enumerate().map(|(coord, c)| {
    let (x, y) = to_xy(coord);

    return match c as char {
      | ' '
      | '.'
      => empty_cell(x, y),
      | '1'
      | '2'
      | '3'
      | '4'
      | '5'
      | '6'
      | '7'
      | '8'
      | '9'
      => {
        // Search through the defs for the machine at this index and get its input/output spec
        // `m1 = wg -> s`
        let cell = defs.iter().find_map(|s| {
          let mut b = s.bytes();
          if b.next().or(Some('!' as u8)).unwrap() == 'm' as u8 && b.next().or(Some('!' as u8)).unwrap() == c as u8 {
            // Found the machine def. Parse the input/output spec
            let mut in1 = b.next().or(Some('x' as u8)).unwrap() as char;
            while in1 == ' ' || in1 == '=' {
              in1 = b.next().or(Some('x' as u8)).unwrap() as char;
            }
            let mut in2 = b.next().or(Some('y' as u8)).unwrap() as char;
            while in2 == ' ' {
              in2 = b.next().or(Some('y' as u8)).unwrap() as char;
            }
            let mut in3 = if in2 == '-' { '-' } else { b.next().or(Some('z' as u8)).unwrap() as char };
            while in3 == ' ' {
              in3 = b.next().or(Some('z' as u8)).unwrap() as char;
            }

            let mut out = b.next().or(Some('w' as u8)).unwrap() as char;
            while out == ' ' || out == '-' || out == '>' {
              out = b.next().or(Some('w' as u8)).unwrap() as char;
            }

            // Now look for s:xxx
            let mut speed: u64 = 100;
            loop {
              // Skip spaces. Stop if it hits EOL.
              let modifier = b.next().or(Some('?' as u8)).unwrap() as char;
              if modifier != ' ' {
                if modifier == 's' && (b.next().or(Some('?' as u8)).unwrap() as char) == ':' {
                  // parse the value for speed
                  speed = 0;

                  loop {
                    // Skip spaces and equal signs. Stop if it hits EOL.
                    let d = b.next().or(Some('?' as u8)).unwrap() as char;
                    if d >= '0' && d <= '9' {
                      speed = speed * 10 + (d as u64 - '0' as u64);
                    } else {
                      break;
                    }
                  }
                  if speed == 0 {
                    // This means the value was zero or more zeroes. Set it to one, shrug.
                    speed = 1;
                  }
                }
                else {
                  // Keep parsing while we find s:
                  break;
                }
              }
            };

            println!("Creating machine id={} with inputs({} {} {}) and output({})", c, in1, in2, in3, out);

            let cell = machine_cell(x, y, MachineKind::Unknown, part_c(in1), if in2 == '-' { part_none() } else { part_c(in2) }, if in3 == '-' { part_none() } else { part_c(in3) }, part_c(out), speed, 1, 1);
            return Some(cell);
          }

          // This wasn't the target machine definition
          return None;
        })
          .or(Some(machine_cell(x, y, MachineKind::Unknown, part_none(), part_none(), part_none(), part_none(), 888, 1, 1)))
          .unwrap(); // Always returns a some due to the .or()

        return cell;
      },
      'b' => belt_cell(x, y, BELT_UNKNOWN),
      's' => {
        suppliers += 1;

        // Search through the defs for the machine at this index and get its input/output spec
        let cell = defs.iter().find_map(|s| {
          let mut b = s.bytes();
          if b.next().or(Some('!' as u8)).unwrap() == 's' as u8 && b.next().or(Some('!' as u8)).unwrap() == suppliers {
            // Found the supply def. Should have the kind of part that it gives.
            let mut gives = b.next().or(Some('x' as u8)).unwrap() as char;
            while gives == ' ' || gives == '=' {
              gives = b.next().or(Some('x' as u8)).unwrap() as char;
            }

            // Now look for s:xxx and c:xxx
            let mut speed: u64 = 100;
            let mut cooldown: u64 = 50;
            loop {
              // Skip spaces. Stop if it hits EOL.
              let modifier = b.next().or(Some('?' as u8)).unwrap() as char;
              if modifier != ' ' {
                if modifier == 's' && (b.next().or(Some('?' as u8)).unwrap() as char) == ':' {
                  // parse the value for speed
                  speed = 0;

                  loop {
                    // Skip spaces and equal signs. Stop if it hits EOL.
                    let d = b.next().or(Some('?' as u8)).unwrap() as char;
                    if d >= '0' && d <= '9' {
                      speed = speed * 10 + (d as u64 - '0' as u64);
                    } else {
                      break;
                    }
                  }
                  if speed == 0 {
                    // This means the value was zero or more zeroes. Set it to one, shrug.
                    speed = 1;
                  }
                }
                else if modifier == 'c' && (b.next().or(Some('?' as u8)).unwrap() as char) == ':' {
                  // parse the value for speed
                  cooldown = 0;

                  loop {
                    // Skip spaces and equal signs. Stop if it hits EOL.
                    let d = b.next().or(Some('?' as u8)).unwrap() as char;
                    if d >= '0' && d <= '9' {
                      cooldown = cooldown * 10 + (d as u64 - '0' as u64);
                    } else {
                      break;
                    }
                  }
                  if cooldown == 0 {
                    // This means the value was zero or more zeroes. Set it to one, shrug.
                    cooldown = 1;
                  }
                }
                else {
                  // Keep parsing while we find s: and c: modifiers
                  break;
                }
              }
            };

            println!("Creating supplier {}, id={} which gives ({})", suppliers, c, gives);

            let cell = supply_cell(x, y, part_c(gives), speed, cooldown, 1);
            return Some(cell);
          }

          // This wasn't the target machine definition
          return None;
        })
          .or(Some(supply_cell(x, y, part_none(), 0, 1, 1)))
          .unwrap(); // Always returns a some due to the .or()

        return cell;
      },
      'd' => {
        demanders += 1;

        // Search through the defs for the machine at this index and get its input/output spec
        let cell = defs.iter().find_map(|s| {
          let mut b = s.bytes();
          if b.next().or(Some('!' as u8)).unwrap() == 'd' as u8 && b.next().or(Some('!' as u8)).unwrap() == demanders {
            // Found the demand def. Should have the kind of part that it wants.
            let mut wants = b.next().or(Some('x' as u8)).unwrap() as char;
            while wants == ' ' || wants == '=' {
              wants = b.next().or(Some('x' as u8)).unwrap() as char;
            }

            println!("Creating demander {}, id={} which gives ({})", demanders, c, wants);

            let cell = demand_cell(x, y, part_c(wants));
            return Some(cell);
          }

          // This wasn't the target machine definition
          return None;
        })
          .or(Some(demand_cell(x, y, part_none())))
          .unwrap(); // Always returns a some due to the .or()

        return cell;


      },
      _ => panic!("no can do, only supported chars are: space, dot, m, b, d, and s, but got: `{}`", c as char),
    };
  })
    .collect::<Vec<Cell>>()
    .try_into() // runtime error if bad but this is fine
    .unwrap();

  return ( floor, offers );
}

pub fn auto_layout(options: &mut Options, state: &mut State, factory: &mut Factory) {
  let mut machines = 0;
  for coord in 0..FLOOR_CELLS_WH {
    match factory.floor[coord].kind {
      CellKind::Empty => {}
      CellKind::Belt => {
        let up = get_cell_kind_at(factory, factory.floor[coord].coord_u);
        let right = get_cell_kind_at(factory, factory.floor[coord].coord_r);
        let down = get_cell_kind_at(factory, factory.floor[coord].coord_d);
        let left = get_cell_kind_at(factory, factory.floor[coord].coord_l);

        factory.floor[coord].belt = belt_new(belt_auto_layout(up, right, down, left));
      }
      CellKind::Machine => {
        if factory.floor[coord].machine.kind == MachineKind::Unknown {
          // This will be the main machine cell.
          // Any neighboring machine cells will be converted to be sub cells of this machine
          // Recursively collect them and mark them now so this loop skips these sub cells
          factory.floor[coord].machine.kind = MachineKind::Main;
          factory.floor[coord].machine.main_coord = coord;
          factory.floor[coord].machine.id = machines;
          factory.floor[coord].machine.coords = vec!(coord); // First element is always main coord
          auto_layout_tag_machine(factory, factory.floor[coord].coord_u, coord, machines);
          auto_layout_tag_machine(factory, factory.floor[coord].coord_r, coord, machines);
          auto_layout_tag_machine(factory, factory.floor[coord].coord_d, coord, machines);
          auto_layout_tag_machine(factory, factory.floor[coord].coord_l, coord, machines);

          machines += 1;
          // Since order does not _really_ matter, it's easier to debug when the subs are in
          // grid order of appearance so just sort them incrementally
          factory.floor[coord].machine.coords.sort();
          println!("Machine {} @{} has these parts: {:?}", factory.floor[coord].machine.id, coord, factory.floor[coord].machine.coords);
        }
      }
      CellKind::Supply => {}
      CellKind::Demand => {}
    }
  }

  keep_auto_porting(options, state, factory);
}
fn auto_layout_tag_machine(factory: &mut Factory, sub_coord: Option<usize>, main_coord: usize, machine_id: usize) {
  // Base "end" case for recursion: cell is not an unknown machine
  match sub_coord {
    None => {} // Noop
    Some(ocoord) => {
      if factory.floor[ocoord].kind == CellKind::Machine {
        if factory.floor[ocoord].machine.kind == MachineKind::Unknown {
          factory.floor[ocoord].machine.kind = MachineKind::SubBuilding;
          factory.floor[ocoord].machine.main_coord = main_coord;
          factory.floor[ocoord].machine.id = machine_id;
          factory.floor[main_coord].machine.coords.push(ocoord);
          println!("Tagged @{} as part of machine {}", ocoord, machine_id);
          // Find all neighbors. Lazily include the one you just came from. It'll be ignored.
          auto_layout_tag_machine(factory, factory.floor[ocoord].coord_u, main_coord, machine_id);
          auto_layout_tag_machine(factory, factory.floor[ocoord].coord_r, main_coord, machine_id);
          auto_layout_tag_machine(factory, factory.floor[ocoord].coord_d, main_coord, machine_id);
          auto_layout_tag_machine(factory, factory.floor[ocoord].coord_l, main_coord, machine_id);
        } else {
          // Since we're expanding from a single machine cell, we should be finding and tagging all neighboring machine cells in the same way. As such, I don't think it should be possible to have a machine of a different kind here.
          // (This may be different in the future, I don't think it's a hard long term requirement, but right now it is)
          // (One example where this won't be true anymore is when placing machines adjacent to each other. But in that case there's less auto-layout? Or maybe it's just possible in that case. Dunno yet.)
          assert_eq!(factory.floor[ocoord].machine.main_coord, main_coord, "if neighbor is not an unknown machine then it has to be part of the same machine");
        }
      }
    }
  }
}

pub fn get_edge_neighbor(x: usize, y: usize, coord: usize) -> (usize, Direction, Direction) {
  // Returns: neighbor coord, supply outgoing dir / demand incoming dir, neighbor incoming/outgoing dir
  return
    if y == 0 { ( to_coord_down(coord), Direction::Down, Direction::Up ) }
    else if x == FLOOR_CELLS_W - 1 { ( to_coord_left(coord), Direction::Left, Direction::Right ) }
    else if y == FLOOR_CELLS_H - 1 { ( to_coord_up(coord), Direction::Up, Direction::Down ) }
    else if x == 0 { ( to_coord_right(coord), Direction::Right, Direction::Left ) }
    else { panic!("get_edge_neighbor({}, {}, {}): coord should live on an edge", x, y, coord); };
}

// https://en.wikipedia.org/wiki/Box-drawing_character

// ┌─────────┐    ┌─────────┐    ┌─────────┐
// │# # # # #│    │? ? ? ? #│    │/ - - \ #│
// │# # # # #│    d? # # ? ?│    d/ # # \ \│
// │# # # # #│ >  │# # # # ?│ >  │# # # # |│
// │# # # # #│    │# # # # ?│    │# # # # |│
// │# # # # #│    │# # # # ?│    │# # # # |│
// └─────────┘    └────────s┘    └────────s┘

// ┌─────────┐    ┌──s──────┐    ┌──s──────┐
// │# # # # #│    │? ? # ? #│    │/ - ? \ #│
// │# # # # #│    d? # # ? ?│    d/ # # \ \│
// │# # # # #│ >  │# # # # ?│ >  │# # # # |│
// │# # # # #│    │# # # # ?│    │# # # # |│
// │# # # # #│    │# # # # ?│    │# # # # |│
// └─────────┘    └────────d┘    └────────s┘

// ┌──────────────┐
// │/\ /\ /\ /\ /\│
// │\/ \/ \/ \/ \/│
// │              │
// │/\ /\ /\ /\ /\│
// │\/ \/ \/ \/ \/│
// │              │
// │/\ /\ /\ /\ /\│
// │\/ \/ \/ \/ \/│
// │              │
// │/\ /\ /\ /\ /\│
// │\/ \/ \/ \/ \/│
// │              │
// │/\ /\ /\ /\ /\│
// │\/ \/ \/ \/ \/│
// └──────────────┘

// ┌─────────v─────────┐
// │xxx│   │ ║ │   │   │
// │xxx│═══│═╩═│═══│═╗ │
// │xxx│   │   │   │ ║ │
// │───────────────────│
// │ ║ │   │   │   │ ║ │
// │ ╚═│═══│═╗ │   │ ╚═<
// │   │   │ ║ │   │   │
// │───────────────────│
// │   │   │yyy│   │   │
// │   │   │yyy│   │   │
// │   │   │yyy│   │   │
// │───────────────────│
// │   │   │ ║ │   │   │
// │   │   │ ║ │   │   │
// │   │   │ ║ │   │   │
// │───────────────────│
// │   │   │ ║ │   │   │
// │   │ ╔═│═╝ │   │   │
// │   │ ║ │   │   │   │
// └─────v─────────────┘

//    0     1    2    3    4    5     6
//                  ┌───┐
// 0                │a A│                0      6
//       ┌────────────v────────────┐
//       │┌───┐┌───┐┌─v─┐┌───┐┌───┐│
//       ││xxx││   ││ ║ ││   ││   ││
// 1     ││xxx<<═══<<═╩═<<═══<<═╗ ││     1
//       ││xxx││   ││10 ││   ││ ║ ││            13
//       │└─v─┘└───┘└───┘└───┘└─^─┘│
//       │┌─v─┐┌───┐┌───┐┌───┐┌─^─┐│
//       ││ ║ ││   ││   ││   ││ ║ ││
// 2     ││ ╚═>>═══>>═╗ ││   ││ ║ ││     2
//       ││15 ││   ││ ║ ││   ││ ║ ││            20
//       │└───┘└───┘└─v─┘└───┘└─^─┘│
//       │┌───┐┌───┐┌─v─┐┌───┐┌─^─┐│
//       ││   ││   ││yyy││   ││ ║ ││
// 3     ││   ││   ││yyy││   ││ ║ ││     3
//       ││   ││   ││yyy││   ││ ║ ││            27
//       │└───┘└───┘└─v─┘└───┘└─^─┘│
//       │┌───┐┌───┐┌─v─┐┌───┐┌─^─┐│┌──┐
//       ││   ││   ││ ║ ││   ││ ║ │││a │
// 4     ││   ││   ││3║1││   ││ ╚═<<│  │ 4
//       ││   ││   ││ ║ ││   ││   │││A │        34
//       │└───┘└───┘└─v─┘└───┘└───┘│└──┘
//       │┌───┐┌───┐┌─v─┐┌───┐┌───┐│
//       ││   ││   ││ ║ ││   ││   ││
// 5     ││   ││ ╔═<<═╝ ││   ││   ││     5      41
//       ││   ││ ║ ││   ││   ││   ││
//       │└───┘└─v─┘└───┘└───┘└───┘│
//       └───────v─────────────────┘
// 6           │ a │                     6      48
//             └───┘
//    0     1    2    3    4    5     6


// ```
//  .......s.......
// .mmm....b........
// .mmmbbbbbbbbbbb..
// .mmm..........b..
// ..b...........b..
// ..bbbbbbb.....b..
// ........b.....b..
// .......mmm....b..
// .......mmm....b..
// .......mmm....b..
// ........b.....bbs
// ..bbbbbbb........
// ..b.....b........
// ..b.....b........
// dbb..bbbb........
// .....b...........
//  ....d..........
// ```


// ┌─────────────────┐
// │        s        │
// │ mmm    ║        │
// │ mmm════╩═════╗  │
// │ mmm          ║  │
// │  ║           ║  │
// │  ╚═════╗     ║  │
// │        ║     ║  │
// │       mmm    ║  │
// │       mmm    ║  │
// │       mmm    ║  │
// │        ║     ║  │
// │  ╔═════╣     ╚═s│
// │  ║     ║        │
// │  ║     ║        │
// │d═╝  ╔══╝        │
// │     ║           │
// │     d           │
// └─────────────────┘


// 11x11=121
// ┌───────────────────────────────────────────────────────┐
// │     ┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐     │
// │     │   ││   ││   ││   ││   ││   ││   ││   ││   │     │
// │     │   ││   ││   ││   ││   ││   ││   ││   ││   │     │
// │     │   ││   ││   ││   ││   ││   ││   ││   ││   │     │
// │     └───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘     │
// │┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐│
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// │└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘│
// │┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐│
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// │└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘│
// │┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐│
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// │└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘│
// │┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐│
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// │└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘│
// │┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐│
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// │└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘│
// │┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐│
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// │└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘│
// │┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐│
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// │└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘│
// │┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐│
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// │└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘│
// │┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐│
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││   ││
// │└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘│
// │     ┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐┌───┐     │
// │     │   ││   ││   ││   ││   ││   ││   ││   ││   │     │
// │     │   ││   ││   ││   ││   ││   ││   ││   ││   │     │
// │     │   ││   ││   ││   ││   ││   ││   ││   ││   │     │
// │     └───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘└───┘     │
// └───────────────────────────────────────────────────────┘






// pub fn test_floor() -> Floor {
//   let club = Part { kind: PartKind::WoodenStick, icon: 'w'};
//   let gem = Part { kind: PartKind::Sapphire, icon: 's'};
//   let wand = Part { kind: PartKind::BlueWand, icon: 'b'};
//   let gwand = Part { kind: PartKind::GoldenBlueWand, icon: 'g'};
//
//   let supply_top = supply_cell(3, 0, club.clone(), 1000, 1000, 100);
//   let supply_right = supply_cell(6, 4,  gem.clone(), 1000, 1000, 100);
//
//   let demand_down = demand_cell(2, 6, gwand.clone());
//
//   let machine1 = machine_cell(2, 2, MachineKind::Composer, wand.clone(), part_none(), part_none(), gwand.clone(), -15, -3);
//   let machine2 = machine_cell(2, 2, MachineKind::Smasher, club.clone(), gem.clone(), part_none(), wand.clone(), -25, -10);
//
//   let width: usize = 5;
//   let height: usize = 5;
//
//   return Floor {
//     width,
//     height,
//     sum: width * height,
//     fsum: (width+2) * (height+2),
//     cells: [
//       empty_cell(0, 0), empty_cell(1, 0), empty_cell(2, 0), supply_top, empty_cell(4, 0), empty_cell(5, 0), empty_cell(6, 0),
//       empty_cell(0, 1), machine2, belt_cell(2, 1, CELL_BELT_R_L), belt_cell(3, 1, CELL_BELT_RU_L), belt_cell(4, 1, CELL_BELT_R_L), belt_cell(5, 1, CELL_BELT_D_L), empty_cell(6, 1),
//       empty_cell(0, 2), belt_cell(1, 2, CELL_BELT_U_R), belt_cell(2, 2, CELL_BELT_L_R), belt_cell(3, 2, CELL_BELT_L_D), empty_cell(4, 2), belt_cell(5, 2, CELL_BELT_D_U), empty_cell(6, 2),
//       empty_cell(0, 3), empty_cell(1, 3), empty_cell(2, 3), machine1, empty_cell(4, 3), belt_cell(5, 3, CELL_BELT_D_U), empty_cell(6, 3),
//       empty_cell(0, 4), empty_cell(1, 4), empty_cell(2, 4), belt_cell(3, 4, CELL_BELT_U_D), empty_cell(4, 4), belt_cell(5, 4, CELL_BELT_R_U), supply_right,
//       empty_cell(0, 5), empty_cell(1, 5), belt_cell(2, 5, CELL_BELT_R_D), belt_cell(3, 5, CELL_BELT_U_L), empty_cell(4, 5), empty_cell(5, 5), empty_cell(6, 5),
//       empty_cell(0, 6), empty_cell(1, 6), demand_down, empty_cell(3, 6), empty_cell(4, 6), empty_cell(5, 6), empty_cell(6, 6),
//     ]
//   };
// }

pub fn is_middle(x: usize, y: usize) -> bool {
  return x > 0 && y > 0 && x < FLOOR_CELLS_W - 1 && y < FLOOR_CELLS_H - 1;
}

pub fn is_edge(x: usize, y: usize) -> bool {
  return x == 0 || y == 0 || x == FLOOR_CELLS_W - 1 || y == FLOOR_CELLS_H - 1;
}

pub fn to_xy(coord: usize) -> (usize, usize) {
  // Return 0,0 if coord is oob (so for edge cells)

  let x = coord % FLOOR_CELLS_W;
  let y = coord / FLOOR_CELLS_W; // int division

  return (x, y);
}

pub fn to_coord(x: usize, y: usize) -> usize {
  return x + FLOOR_CELLS_W * y;
}

pub const fn to_coord_up(coord: usize) -> usize {
  return coord - FLOOR_CELLS_W;
}
pub const fn to_coord_right(coord: usize) -> usize {
  return coord + 1;
}
pub const fn to_coord_down(coord: usize) -> usize {
  return coord + FLOOR_CELLS_W;
}
pub const fn to_coord_left(coord: usize) -> usize {
  return coord - 1;
}

// pub fn floor_create_cell_at_partial(options: &mut Options, state: &mut State, factory: &mut Factory, coord1: usize, x1: i8, y1: i8, coord2: usize, x2: i8, y2: i8) {
//   // Note: this still requires auto porting and creating a new prio
//   // Note: generating cell at x1,x2
//
//   // Must cast because I want to know about negatives
//   let dx = x1 - x2;
//   let dy = y1 - y2;
//   log(format!("floor_create_cell_at_partial @{} - @{} -> {} {}, ports: {} and {}", coord1, coord2, dx, dy, serialize_ports(factory, coord1), serialize_ports(factory, coord2)));
//   assert!(dx == 0 || dy == 0, "cell should neighbor previous cell so one axis should not change {} {} - {} {}", x1, y1, x2, y2);
//   if dy < 0 {
//     // x1,y1 is above x2,y2 (because y2>y1)
//     if factory.floor[coord1].kind == CellKind::Belt {
//       if factory.floor[coord1].port_d == Port::None {
//         factory.floor[coord1].port_d = Port::Unknown;
//         fix_belt_meta(factory, coord1);
//       }
//       factory.floor[coord2].port_u = Port::Unknown;
//     }
//   } else if dx > 0 {
//     // x2,y2 is right of x1,p1
//     if factory.floor[coord1].kind == CellKind::Belt {
//       if factory.floor[coord1].port_l == Port::None {
//         factory.floor[coord1].port_l = Port::Unknown;
//         fix_belt_meta(factory, coord1);
//       }
//       factory.floor[coord2].port_r = Port::Unknown;
//     }
//   } else if dy > 0 {
//     // x2,y2 is under x1,y1
//     if factory.floor[coord1].kind == CellKind::Belt {
//       if factory.floor[coord1].port_u == Port::None {
//         factory.floor[coord1].port_u = Port::Unknown;
//         fix_belt_meta(factory, coord1);
//       }
//       factory.floor[coord2].port_d = Port::Unknown;
//     }
//   } else if dx < 0 {
//     // x2,y2 is left of x1,y1
//     if factory.floor[coord1].kind == CellKind::Belt {
//       if factory.floor[coord1].port_r == Port::None {
//         factory.floor[coord1].port_r = Port::Unknown;
//         fix_belt_meta(factory, coord1);
//       }
//       factory.floor[coord2].port_l = Port::Unknown;
//     }
//   }
//   log(format!("  - after  @{} - @{} -> {} {}, ports: {} and {}", coord1, coord2, dx, dy, serialize_ports(factory, coord1), serialize_ports(factory, coord2)));
//
//   fix_belt_meta(factory, coord2);
// }
pub fn floor_delete_cell_at_partial(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) {
  // Note: partial because factory prio must to be updated too, elsewhere (!)
  // Running auto-porting may uncover new tracks but should not be required to run
  // Can be used for any cell

  if factory.floor[coord].kind == CellKind::Machine {
    let main_coord = factory.floor[coord].machine.main_coord;
    log(format!("Dropping entire factory, {} cells", factory.floor[main_coord].machine.coords.len()));
    // Special case: we have to remove the entire machine, not just this cell
    // For every part of it we have to remove all ports relating to it.
    for index in 0..factory.floor[main_coord].machine.coords.len() {
      let coord = factory.floor[main_coord].machine.coords[index];
      if coord != main_coord {
        floor_delete_cell_at_partial_sub(options, state, factory, coord);
      }
    }
    // Do main coord last since we indirectly reference it while removing the other subs
    floor_delete_cell_at_partial_sub(options, state, factory, main_coord);
    log(format!("-- dropped"));
  } else {
    return floor_delete_cell_at_partial_sub(options, state, factory, coord);
  }
}
pub fn floor_delete_cell_at_partial_sub(options: &mut Options, state: &mut State, factory: &mut Factory, coord: usize) {
  // For all connected cells
  // - delete port towards this cell
  // - update belt meta to reflect new cell meta
  // - update ins and outs to remove this port if it's in there
  // Replace this cell with fresh empty cell

  if factory.floor[coord].port_u != Port::None {
    if let Some(ocoord) = factory.floor[coord].coord_u {
      port_disconnect_cell(factory, ocoord, Direction::Down);
      fix_belt_meta(factory, ocoord);
    }
  }

  if factory.floor[coord].port_r != Port::None {
    if let Some(ocoord) = factory.floor[coord].coord_r {
      port_disconnect_cell(factory, ocoord, Direction::Left);
      fix_belt_meta(factory, ocoord);
    }
  }

  if factory.floor[coord].port_d != Port::None {
    if let Some(ocoord) = factory.floor[coord].coord_d {
      port_disconnect_cell(factory, ocoord, Direction::Up);
      fix_belt_meta(factory, ocoord);
    }
  }

  if factory.floor[coord].port_l != Port::None {
    if let Some(ocoord) = factory.floor[coord].coord_l {
      port_disconnect_cell(factory, ocoord, Direction::Right);
      fix_belt_meta(factory, ocoord);
    }
  }

  factory.floor[coord] = empty_cell(factory.floor[coord].x, factory.floor[coord].y);
}
