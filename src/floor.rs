use std::borrow::Borrow;
use std::collections::VecDeque;
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

pub fn floor_from_str1(str: String) -> ( [Cell; FLOOR_CELLS_WH], Vec<Offer> ) {
  if str.len() == 0 {
    return ( floor_empty(), vec!() );
  }

  // let ( floor, offers ) = str_to_floor(str);
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
  // om = rgb -> a d:3x2
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
  // Offers:
  // - supply that gives 'g' with default speed and delay
  // - machine that takes r g and b and generates an a, 3 cells wide by 2 cells high


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
  // m1 = w s   -> b
  // m2 = b     -> g
  // os = w s:10 c:5
  // os = s s:10 c:5
  // od = g
  // om = s w   -> b s:0 d:3x2
  // om = b     -> g s:10 d:1x1
  // om = b     -> g s:0 d:3x3
  // om = b     -> g s:0 d:4x4


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
      // os = x          Offer of a supply that gives x
      // od = x          Offer of a demand that takes x
      // om = ab -> c    Offer of a machine that takes a and b and generates c
      // suppliers and machines can have a s: modifier, setting speed
      // suppliers can have a c: modifier, setting cooldown
      // machines can have a d: modifier, setting cell width/height of the machine

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
            cell_width: 1,
            cell_height: 1,
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
            cell_width: 1,
            cell_height: 1,
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
          let mut cell_width: usize = 1;
          let mut cell_height: usize = 1;
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
              else if modifier == 'd' && (line.next().or(Some('?' as u8)).unwrap() as char) == ':' {
                // parse the width and height
                speed = 0;

                loop {
                  // Skip non-digits, stop at EOL.
                  let d = line.next().or(Some('?' as u8)).unwrap() as char;
                  if d >= '0' && d <= '9' {
                    cell_width = d as usize - '0' as usize;
                    if cell_width == 0 {
                      cell_width = 1;
                    }
                    break;
                  } else if d == '?' {
                    break; // EOL
                  }
                }

                loop {
                  // Skip non-digits, stop at EOL.
                  let d = line.next().or(Some('?' as u8)).unwrap() as char;
                  if d >= '0' && d <= '9' {
                    cell_height = d as usize - '0' as usize;
                    if cell_height == 0 {
                      cell_height = 1;
                    }
                    break;
                  } else if d == '?' {
                    break; // EOL
                  }
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
            cell_width,
            cell_height,
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

            let cell = machine_any_cell(c as char, x, y, 1, 1, MachineKind::Unknown, part_c(in1), if in2 == '-' { part_none() } else { part_c(in2) }, if in3 == '-' { part_none() } else { part_c(in3) }, part_c(out), speed, 1, 1);
            return Some(cell);
          }

          // This wasn't the target machine definition
          return None;
        })
          .or(Some(machine_any_cell(c as char, x, y, 1, 1, MachineKind::Unknown, part_none(), part_none(), part_none(), part_none(), 888, 1, 1)))
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
  log(format!("auto_layout()"));
  let mut machines = 0;
  for coord in 0..FLOOR_CELLS_WH {
    match factory.floor[coord].kind {
      CellKind::Empty => {}
      CellKind::Belt => {
        let up = if factory.floor[coord].port_u == Port::None { CellKind::Empty } else { get_cell_kind_at(factory, factory.floor[coord].coord_u) };
        let right = if factory.floor[coord].port_r == Port::None { CellKind::Empty } else { get_cell_kind_at(factory, factory.floor[coord].coord_r) };
        let down = if factory.floor[coord].port_d == Port::None { CellKind::Empty } else { get_cell_kind_at(factory, factory.floor[coord].coord_d) };
        let left = if factory.floor[coord].port_l == Port::None { CellKind::Empty } else { get_cell_kind_at(factory, factory.floor[coord].coord_l) };

        factory.floor[coord].belt = belt_new(belt_auto_layout(up, right, down, left));
      }
      CellKind::Machine => {
        if factory.floor[coord].machine.kind == MachineKind::Unknown {
          machines += 1; // offset 1
          let main_coord = coord;
          // This will be the main machine cell.
          // Any neighboring machine cells will be converted to be sub cells of this machine
          // Recursively collect them and mark them now so this loop skips these sub cells
          // This process is greedy for any valid machine, meaning it will read create the biggest
          // rectangle, but without backtracking (in case that matters).
          // As such, this unprocessed machine should be the top-left most cell of any machine and
          // we only have to check the rectangle moving to the right and downward. Then verify that
          // all cells inside are also unknowns. Then mark them to their new owner.

          let ( x, y ) = to_xy(main_coord);
          let mut max_width = FLOOR_CELLS_W - x;
          let mut max_height = FLOOR_CELLS_H - y;

          let mut biggest_area_size = 0;
          let mut biggest_area_width = 0;
          let mut biggest_area_height = 0;

          log(format!("======"));
          log(format!("- Discovering machine..."));
          // Find biggest machine rectangle area wise, with x,y in the top-left corner
          for n in y .. y + FLOOR_CELLS_H - y {
            max_height = n - y;
            for m in x .. x + max_width {
              let c = to_coord(m, n);
              log(format!("  - {}x{}; is machine? {:?}; if so, what kind? {:?}", m, n, factory.floor[c].kind, factory.floor[c].machine.kind));
              if factory.floor[c].kind != CellKind::Machine || factory.floor[c].machine.kind != MachineKind::Unknown {
                log(format!("    - end of machine row, max width {}, max height {}, first col? {}, area is {}, biggest is {}", max_width, max_height, m==x, max_width * max_height, biggest_area_size));

                if max_width * max_height > biggest_area_size {
                  biggest_area_size = max_width * max_height;
                  biggest_area_width = max_width;
                  biggest_area_height = max_height;
                }

                max_width = m - x;
                break;
              }
            }
            if max_width == 0 {
              break;
            }
          }
          log(format!("  - Last area: {} ({} by {})", max_width * max_height, max_width, max_height));
          if max_width * max_height > biggest_area_size {
            biggest_area_size = max_width * max_height;
            biggest_area_width = max_width;
            biggest_area_height = max_height;
          }

          let id =
            if machines >= 36 {
              (('A' as u8) + (machines - 36)) as char // A-Z
            } else if machines > 9 {
              (('a' as u8) + (machines - 10)) as char // a-z
            } else {
              (('0' as u8) + machines) as char // 1-9
            };

          log(format!("  - Final biggest area: {} at {} x {}, assigning machine number {}, with id `{}`", biggest_area_size, biggest_area_width, biggest_area_height, machines, id));
          log(format!("======"));

          // Now collect all cells in this grid and assign them to be the same machine
          factory.floor[main_coord].machine.coords.clear();
          for dx in 0..biggest_area_width {
            for dy in 0..biggest_area_height {
              let ocoord = to_coord(x + dx, y + dy);
              factory.floor[ocoord].machine.kind = MachineKind::SubBuilding;
              factory.floor[ocoord].machine.main_coord = main_coord;

              factory.floor[ocoord].machine.id = (('0' as u8) + machines) as char;
              factory.floor[ocoord].machine.coords.clear();
              factory.floor[main_coord].machine.coords.push(ocoord);
            }
          }
          factory.floor[main_coord].machine.kind = MachineKind::Main;
          factory.floor[main_coord].machine.cell_width = biggest_area_width;
          factory.floor[main_coord].machine.cell_height = biggest_area_height;
          factory.floor[main_coord].machine.coords.sort(); // Makes debugging easier

          log(format!("Machine {} @{} has these cells: {:?}", factory.floor[main_coord].machine.id, main_coord, factory.floor[main_coord].machine.coords));
        }
      }
      CellKind::Supply => {}
      CellKind::Demand => {}
    }
  }

  keep_auto_porting(options, state, factory);
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

pub fn is_floor(x: usize, y: usize) -> bool {
  // Note: usize is
  return x <= FLOOR_CELLS_W - 1 && y <= FLOOR_CELLS_H - 1;
}

pub fn is_middle(x: usize, y: usize) -> bool {
  return x > 0 && y > 0 && x < FLOOR_CELLS_W - 1 && y < FLOOR_CELLS_H - 1;
}

pub fn is_edge(x: usize, y: usize) -> bool {
  return x == 0 || y == 0 || x == FLOOR_CELLS_W - 1 || y == FLOOR_CELLS_H - 1;
}

pub fn is_edge_not_corner(x: usize, y: usize) -> bool {
  return (x == 0 || x == FLOOR_CELLS_W - 1) != (y == 0 || y == FLOOR_CELLS_H - 1);
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
