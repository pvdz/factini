use std::borrow::Borrow;
use std::convert::TryInto;

use super::belt::*;
use super::cell::*;
use super::demand::*;
use super::factory::*;
use super::direction::*;
use super::machine::*;
use super::options::*;
use super::part::*;
use super::port::*;
use super::state::*;
use super::supply::*;

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

pub fn floor_from_str(str: String) -> [Cell; FLOOR_CELLS_WH] {
  if str.len() == 0 {
    return floor_empty();
  }

  let mut floor = str_to_floor(str);
  return floor;
}

fn str_to_floor(str: String) -> [Cell; FLOOR_CELLS_WH] {
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
  // m1 = ws -> b
  // m2 = b -> g
  // s1 = w
  // s2 = s
  // d1 = g
  // d2 = g
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

  let mut len = 0;
  for c in str.bytes() {
    if (c as char) != '\n' {
      len += 1;
    }
  }

  // if len != FLOOR_CELLS_WH {
  //   panic!("Error: input string (ignoring newlines) must be exactly {}x{}={} chars, but had {} chars", FLOOR_CELLS_W, FLOOR_CELLS_H, FLOOR_CELLS_WH, len);
  // }

  let lines = str.split('\n').collect::<Vec<&str>>();

  println!("Importing string map:\n```\n{}\n```", str);

  let defs = &lines[FLOOR_CELLS_H..lines.len()]
  //   .map(|s| {
  //   let bytes = s.bytes();
  //   match bytes.next() {
  //     'm' => {
  //       // Machine lines are in the form of `m[1..9] ?=? ?[a-z]?[a-z]?[a-z]? ? -?>? ? [a-z]`
  //       // While the `=` and `->` and spacing is optional, the "parser" may fall off the rails if you mess up too badly.
  //
  //       // 1..9
  //       let n = bytes.next();
  //
  //       // a-z or -
  //       let mut i_1 = bytes.next();
  //       while i_1 == ' ' || i_1 == '=' {
  //         i_1 = bytes.next();
  //       }
  //       // a-z or -
  //       let mut i_2 = if i_1 == '-' { '-' } else { bytes.next() };
  //       while i_2 == ' ' || i_2 == '=' {
  //         i_2 = bytes.next();
  //       }
  //       // a-z or -
  //       let mut i_3 = if i_2 == '-' { '-' } else { bytes.next() };
  //       while i_3 == ' ' || i_3 == '=' {
  //         i_3 = bytes.next();
  //       }
  //
  //       // a-z
  //       let mut o = bytes.next();
  //       while o == ' ' || o == '-' || o == '>' {
  //         o = bytes.next();
  //       }
  //
  //       // Should now have up to three inputs and one output
  //
  //
  //     }
  //     'd' => {
  //       // 1..9
  //       let n = bytes.next();
  //
  //       // a-z
  //       let mut part = bytes.next();
  //       while part == ' ' || part == '=' || part == '-' || part == '>' {
  //         part = bytes.next();
  //       }
  //     }
  //     's' => {
  //       // 1..9
  //       let n = bytes.next();
  //
  //       // a-z
  //       let mut part = bytes.next();
  //       while part == ' ' || part == '=' || part == '-' || part == '>' {
  //         part = bytes.next();
  //       }
  //     }
  //     _ => panic!("Legend lines should start with m (machine), d (demand), or s (supply)").
  //   }
  // })
  ;
  println!("defs: {:?}", defs);

  let mut suppliers: u8 = '0' as u8;
  let mut demanders: u8 = '0' as u8;

  return lines[0..FLOOR_CELLS_H].iter().map(|s| s.bytes()).flatten().enumerate().map(|(coord, c)| {
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

            println!("Creating machine id={} with inputs({} {} {}) and output({})", c, in1, in2, in3, out);

            let cell = machine_cell(x, y, MachineKind::Unknown, part_c(in1), if in2 == '-' { part_none() } else { part_c(in2) }, if in3 == '-' { part_none() } else { part_c(in3) }, part_c(out), 1, 1);
            return Some(cell);
          }

          // This wasn't the target machine definition
          return None;
        })
          .or(Some(machine_cell(x, y, MachineKind::Unknown, part_none(), part_none(), part_none(), part_none(), 1, 1)))
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

            println!("Creating supplier {}, id={} which gives ({})", suppliers, c, gives);

            let cell = supply_cell(x, y, part_c(gives), 1, 1, 1);
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
}

pub fn auto_layout(factory: &mut Factory) {
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

  keep_auto_porting(factory);
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

pub fn is_floor(x: usize, y: usize, w: usize, h: usize) -> bool {
  return x != 0 && y != 0 && x != w - 1 && y != h - 1;
}

pub fn is_edge(x: usize, y: usize, w: usize, h: usize) -> bool {
  return x == 0 || y == 0 || x == w - 1 || y == h - 1;
}

pub fn to_floor_xy(coord: usize) -> (usize, usize, bool) {
  // Return 0,0 if coord is oob (so for edge cells)

  let x = coord % FLOOR_CELLS_W;
  let y = coord / FLOOR_CELLS_W; // int division

  return (x, y, is_floor(x, y, FLOOR_CELLS_W, FLOOR_CELLS_H));
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
