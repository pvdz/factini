use std::borrow::Borrow;
use std::collections::VecDeque;
use std::convert::TryInto;

use super::belt::*;
use super::cell::*;
use super::config::*;
use super::demand::*;
use super::factory::*;
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

// Edge cells can only be demand/supply/empty, other cells can only be belt/machine/empty
pub fn floor_empty(config: &Config) -> [Cell; FLOOR_CELLS_WH] {
  log!("floor_empty()");


  // // error[E0658]: use of unstable library feature 'array_map'
  // let mut coord = 0;
  // return [(); FLOOR_CELLS_WH].map(|_| {
  //   let (x, y) = to_xy(coord);
  //   empty_cell(config, x, y)
  // });

  // https://stackoverflow.com/questions/67822062/fixed-array-initialization-without-implementing-copy-or-default-trait/67824946#67824946
  // :shrug: okay
  return (0..FLOOR_CELLS_WH)
    .map(|coord| {
      let (x, y) = to_xy(coord);
      empty_cell(config, x, y)
    })
    .collect::<Vec<Cell>>()
    .try_into() // runtime error if bad but this is fine. this does block --dev mode apparently (https://stackoverflow.com/questions/41710952/allocate-array-onto-heap-with-size-known-at-runtime)
    .unwrap();
}


pub fn auto_layout(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory) {
  log!("auto_layout(options.print_auto_layout_debug={})", options.print_auto_layout_debug);
  let mut machines = 0;
  for coord in 0..FLOOR_CELLS_WH {
    match factory.floor[coord].kind {
      CellKind::Empty => {}
      CellKind::Belt => {
        let up = if factory.floor[coord].port_u == Port::None { CellKind::Empty } else { get_cell_kind_at(factory, factory.floor[coord].coord_u) };
        let right = if factory.floor[coord].port_r == Port::None { CellKind::Empty } else { get_cell_kind_at(factory, factory.floor[coord].coord_r) };
        let down = if factory.floor[coord].port_d == Port::None { CellKind::Empty } else { get_cell_kind_at(factory, factory.floor[coord].coord_d) };
        let left = if factory.floor[coord].port_l == Port::None { CellKind::Empty } else { get_cell_kind_at(factory, factory.floor[coord].coord_l) };
 
        factory.floor[coord].belt = belt_new(config, cell_neighbors_to_auto_belt_meta(up, right, down, left));
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

          if options.print_auto_layout_debug { log!("======"); }
          if options.print_auto_layout_debug { log!("- Discovering machine..."); }
          // Find biggest machine rectangle area wise, with x,y in the top-left corner
          for n in y .. y + FLOOR_CELLS_H - y {
            max_height = n - y;
            for m in x .. x + max_width {
              let c = to_coord(m, n);
              if options.print_auto_layout_debug { log!("  - {}x{}; is machine? {:?}; if so, what kind? {:?}", m, n, factory.floor[c].kind, factory.floor[c].machine.kind); }
              if factory.floor[c].kind != CellKind::Machine || factory.floor[c].machine.kind != MachineKind::Unknown {
                if options.print_auto_layout_debug { log!("    - end of machine row, max width {}, max height {}, first col? {}, area is {}, biggest is {}", max_width, max_height, m==x, max_width * max_height, biggest_area_size); }

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
          if options.print_auto_layout_debug { log!("  - Last area: {} ({} by {})", max_width * max_height, max_width, max_height); }
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

          if options.print_auto_layout_debug { log!("  - Final biggest area: {} at {} x {}, assigning machine number {}, with id `{}`", biggest_area_size, biggest_area_width, biggest_area_height, machines, id); }
          if options.print_auto_layout_debug { log!("======"); }

          // Now collect all cells in this grid and assign them to be the same machine
          factory.floor[main_coord].machine.coords.clear();
          for dx in 0..biggest_area_width {
            for dy in 0..biggest_area_height {
              let ocoord = to_coord(x + dx, y + dy);
              factory.floor[ocoord].machine.kind = MachineKind::SubBuilding;
              factory.floor[ocoord].machine.main_coord = main_coord;

              factory.floor[ocoord].machine.id = (('0' as u8) + machines) as char;
              factory.floor[ocoord].machine.coords.clear();

              factory.floor[ocoord].machine.cell_width = biggest_area_width;
              factory.floor[ocoord].machine.cell_height = biggest_area_height;

              factory.floor[main_coord].machine.coords.push(ocoord);
            }
          }
          factory.floor[main_coord].machine.kind = MachineKind::Main;
          factory.floor[main_coord].machine.coords.sort(); // Makes debugging easier

          // Now that we know the size of the machine, make sure that the wants and haves match
          let cw = biggest_area_width * biggest_area_height;
          for i in 0..cw {
            if factory.floor[main_coord].machine.wants.len() <= i {
              factory.floor[main_coord].machine.wants.push(part_none(config));
            }
            if factory.floor[main_coord].machine.haves.len() <= i {
              factory.floor[main_coord].machine.haves.push(part_none(config));
            }
          }

          if options.print_auto_layout_debug { log!("Machine {} @{} has these cells: {:?}", factory.floor[main_coord].machine.id, main_coord, factory.floor[main_coord].machine.coords); }
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

pub fn is_floor(x: f64, y: f64) -> bool {
  // Note: usize is
  return x >= 0.0 && x < FLOOR_CELLS_W as f64 && y >= 0.0 && y < FLOOR_CELLS_H as f64;
}

pub fn is_middle(x: f64, y: f64) -> bool {
  return x >= 1.0 && y >= 1.0 && x < FLOOR_CELLS_W as f64 - 1.0 && y < FLOOR_CELLS_H as f64 - 1.0;
}

pub fn is_edge(x: f64, y: f64) -> bool {
  return (x >= 0.0 && x < 1.0) || (y >= 0.0 && y < 1.0) || (x >= FLOOR_CELLS_W as f64 - 1.0 && x < FLOOR_CELLS_W as f64) || (y >= FLOOR_CELLS_H as f64 - 1.0 && y < FLOOR_CELLS_H as f64);
}

pub fn is_edge_not_corner(x: f64, y: f64) -> bool {
  // We have to do a bounds check on the other side regardless
  if (x >= 0.0 && x < 1.0) || (x >= (FLOOR_CELLS_W - 1) as f64 && x < FLOOR_CELLS_W as f64) {
    return y > 0.0 && y < (FLOOR_CELLS_H - 1) as f64;
  }

  if (y >= 0.0 && y < 1.0) || (y >= (FLOOR_CELLS_H - 1) as f64 && y < FLOOR_CELLS_H as f64) {
    return x > 0.0 && x < (FLOOR_CELLS_W - 1) as f64;
  }

  return false;
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
//   log!("floor_create_cell_at_partial @{} - @{} -> {} {}, ports: {} and {}", coord1, coord2, dx, dy, serialize_ports(factory, coord1), serialize_ports(factory, coord2));
//   assert!(dx == 0 || dy == 0, "cell should neighbor previous cell so one axis should not change {} {} - {} {}", x1, y1, x2, y2);
//   if dy < 0 {
//     // x1,y1 is above x2,y2 (because y2>y1)
//     if factory.floor[coord1].kind == CellKind::Belt {
//       if factory.floor[coord1].port_d == Port::None {
//         factory.floor[coord1].port_d = Port::Unknown;
//         fix_belt_meta(options, state, config, factory, coord1);
//       }
//       factory.floor[coord2].port_u = Port::Unknown;
//     }
//   } else if dx > 0 {
//     // x2,y2 is right of x1,p1
//     if factory.floor[coord1].kind == CellKind::Belt {
//       if factory.floor[coord1].port_l == Port::None {
//         factory.floor[coord1].port_l = Port::Unknown;
//         fix_belt_meta(options, state, config, factory, coord1);
//       }
//       factory.floor[coord2].port_r = Port::Unknown;
//     }
//   } else if dy > 0 {
//     // x2,y2 is under x1,y1
//     if factory.floor[coord1].kind == CellKind::Belt {
//       if factory.floor[coord1].port_u == Port::None {
//         factory.floor[coord1].port_u = Port::Unknown;
//         fix_belt_meta(options, state, config, factory, coord1);
//       }
//       factory.floor[coord2].port_d = Port::Unknown;
//     }
//   } else if dx < 0 {
//     // x2,y2 is left of x1,y1
//     if factory.floor[coord1].kind == CellKind::Belt {
//       if factory.floor[coord1].port_r == Port::None {
//         factory.floor[coord1].port_r = Port::Unknown;
//         fix_belt_meta(options, state, config, factory, coord1);
//       }
//       factory.floor[coord2].port_l = Port::Unknown;
//     }
//   }
//   log!("  - after  @{} - @{} -> {} {}, ports: {} and {}", coord1, coord2, dx, dy, serialize_ports(factory, coord1), serialize_ports(factory, coord2));
//
//   fix_belt_meta(options, state, config, factory, coord2);
// }
pub fn floor_delete_cell_at_partial(options: &Options, state: &State, config: &Config, factory: &mut Factory, coord: usize) {
  // Note: partial because factory prio must to be updated too, elsewhere (!)
  // Running auto-porting may uncover new tracks but should not be required to run
  // Can be used for any cell

  if factory.floor[coord].kind == CellKind::Machine {
    let main_coord = factory.floor[coord].machine.main_coord;
    log!("Dropping entire factory, {} cells", factory.floor[main_coord].machine.coords.len());
    // Special case: we have to remove the entire machine, not just this cell
    // For every part of it we have to remove all ports relating to it.
    for index in 0..factory.floor[main_coord].machine.coords.len() {
      let coord = factory.floor[main_coord].machine.coords[index];
      if coord != main_coord {
        floor_delete_cell_at_partial_sub(options, state, config, factory, coord);
      }
    }
    // Do main coord last since we indirectly reference it while removing the other subs
    floor_delete_cell_at_partial_sub(options, state, config, factory, main_coord);
    log!("-- dropped");
  } else {
    floor_delete_cell_at_partial_sub(options, state, config, factory, coord);
  }

  factory.changed = true;
}
pub fn floor_delete_cell_at_partial_sub(options: &Options, state: &State, config: &Config, factory: &mut Factory, coord: usize) {
  // For all connected cells
  // - delete port towards this cell
  // - update belt meta to reflect new cell meta
  // - update ins and outs to remove this port if it's in there
  // Replace this cell with fresh empty cell

  if factory.floor[coord].port_u != Port::None {
    if let Some(ocoord) = factory.floor[coord].coord_u {
      port_disconnect_cell(options, state, config, factory, ocoord, Direction::Down);
      fix_belt_meta(options, state, config, factory, ocoord);
    }
  }

  if factory.floor[coord].port_r != Port::None {
    if let Some(ocoord) = factory.floor[coord].coord_r {
      port_disconnect_cell(options, state, config, factory, ocoord, Direction::Left);
      fix_belt_meta(options, state, config, factory, ocoord);
    }
  }

  if factory.floor[coord].port_d != Port::None {
    if let Some(ocoord) = factory.floor[coord].coord_d {
      port_disconnect_cell(options, state, config, factory, ocoord, Direction::Up);
      fix_belt_meta(options, state, config, factory, ocoord);
    }
  }

  if factory.floor[coord].port_l != Port::None {
    if let Some(ocoord) = factory.floor[coord].coord_l {
      port_disconnect_cell(options, state, config, factory, ocoord, Direction::Right);
      fix_belt_meta(options, state, config, factory, ocoord);
    }
  }

  factory.floor[coord] = empty_cell(config, factory.floor[coord].x, factory.floor[coord].y);
}


pub fn fix_ins_and_outs_for_all_belts_and_machines(factory: &mut Factory) {
  log!("fix_ins_and_outs_for_all_belts_and_machines()");

  // Fix ins and outs. Especially necessary for machines in some cases.
  for coord in 0..FLOOR_CELLS_WH {
    match factory.floor[coord].kind {
      CellKind::Demand => {}
      CellKind::Empty => {}
      CellKind::Belt => {
        // Not sure we actually need this. I think this loop is mainly to patch up machines.
        belt_discover_ins_and_outs(factory, coord);
      }
      CellKind::Machine => {
        if coord == factory.floor[coord].machine.main_coord {
          machine_discover_ins_and_outs(factory, coord);
        }
      }
      CellKind::Supply => {}
    }
  }
}
