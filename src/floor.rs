use std::borrow::Borrow;
use std::convert::TryInto;

use super::belt::*;
use super::cell::*;
use super::demand::*;
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
  auto_layout(&mut floor);
  return floor;
}

fn str_to_floor(str: String) -> [Cell; FLOOR_CELLS_WH] {
  // Given a string in a grid format, generate a floor
  // String floor must have equal with/height as hardcoded size, for now (11x11).
  // Empty cell is space or dot, s = supply, d = demand, b = belt, m = machine. Auto-layout should fix it.
  // Example string (inc newlines, ex one space indent after comment start):
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
  // Forming something like this:
  // ┌────────────────────┐
  // │         s          │
  // │ ┌────────────────┐ │
  // │ │mmm    ║        │ │
  // │ │mmm════╩═════╗  │ │
  // │ │mmm          ║  │ │
  // │ │ ║           ║  │ │
  // │ │ ╚═════╗     ║  │ │
  // │ │       ║     ║  │ │
  // │ │      mmm    ║  │ │
  // │ │      mmm    ║  │ │
  // │ │      mmm    ║  │ │
  // │ │       ║     ║  │ │
  // │ │ ╔═════╣     ╚═s│s│
  // │ │ ║     ║        │ │
  // │ │ ║     ║        │ │
  // │d│═╝  ╔══╝        │ │
  // │ │    ║           │ │
  // │ └────────────────┘ │
  // │      d             │
  // └────────────────────┘

  let mut len = 0;
  for c in str.bytes() {
    if (c as char) != '\n' {
      len += 1;
    }
  }

  if len != FLOOR_CELLS_WH {
    panic!("Error: input string (ignoring newlines) must be exactly {}x{}={} chars, but had {} chars", FLOOR_CELLS_W, FLOOR_CELLS_H, FLOOR_CELLS_WH, len);
  }

  return str.split('\n').map(|s| s.bytes()).flatten().enumerate().map(|(coord, c)| {
    let (x, y) = to_xy(coord);

    return match c as char {
      | ' '
      | '.'
      => empty_cell(x, y),
      'b' => belt_cell(x, y, BELT_INVALID),
      'm' => machine_cell(x, y, MachineKind::Unknown, part_none(), part_none(), part_none(), part_none(), 1, 1),
      's' => supply_cell(x, y, part_none(), 0, 1, 1),
      'd' => demand_cell(x, y, part_none()),
      _ => panic!("no can do, only supported chars are: space, dot, m, b, d, and s, but got: `{}`", c as char),
    };
  })
    .collect::<Vec<Cell>>()
    .try_into() // runtime error if bad but this is fine
    .unwrap();
}
fn auto_layout(floor: &mut [Cell; FLOOR_CELLS_WH]) {
  let mut machines = 0;
  for coord in 0..FLOOR_CELLS_WH {
    match floor[coord].kind {
      CellKind::Empty => {}
      CellKind::Belt => {
        let up = get_kind_at(floor, floor[coord].coord_u);
        let right = get_kind_at(floor, floor[coord].coord_r);
        let down = get_kind_at(floor, floor[coord].coord_d);
        let left = get_kind_at(floor, floor[coord].coord_l);

        floor[coord].belt = belt_new(belt_auto_layout(up, right, down, left));
      }
      CellKind::Machine => {
        if floor[coord].machine.kind == MachineKind::Unknown {
          // This will be the main machine cell.
          // Any neighboring machine cells will be converted to be sub cells of this machine
          // Recursively collect them and mark them now so this loop skips these sub cells
          floor[coord].machine.kind = MachineKind::Main;
          floor[coord].machine.main_coord = coord;
          floor[coord].machine.id = machines;
          floor[coord].machine.coords = vec!(coord); // First element is always main coord
          auto_layout_tag_machine(floor, floor[coord].coord_u, coord, machines);
          auto_layout_tag_machine(floor, floor[coord].coord_r, coord, machines);
          auto_layout_tag_machine(floor, floor[coord].coord_d, coord, machines);
          auto_layout_tag_machine(floor, floor[coord].coord_l, coord, machines);

          machines += 1;
          println!("Machine {} @{} has these parts: {:?}", floor[coord].machine.id, coord, floor[coord].machine.coords);
        }
      }
      CellKind::Supply => {}
      CellKind::Demand => {}
    }
  }

  // Start at demands, mark connected belts
  // From connected belts, mark any other connected belt if it is connected to only one unmarked
  // belt. If it is connected to a machine or belt with no unmarked neighbors, then it is looping.
  // From connected machines, do the same
  // When all paths are exhausted collect all machines and belts which are connected to at least
  // one unmarked belt. Mark those and repeat.
  // When there is no more

  let mut attempt = 1; // start at 1 because this value gets used -1, too, and it's a u32.
  while auto_port(floor, attempt) {
    attempt += 1;
  }
}
fn auto_layout_tag_machine(floor: &mut [Cell; FLOOR_CELLS_WH], sub_coord: Option<usize>, main_coord: usize, machine_id: usize) {
  // Base "end" case for recursion: cell is not an unknown machine
  match sub_coord {
    None => {} // Noop
    Some(ocoord) => {
      if floor[ocoord].kind == CellKind::Machine {
        if floor[ocoord].machine.kind == MachineKind::Unknown {
          floor[ocoord].machine.kind = MachineKind::SubBuilding;
          floor[ocoord].machine.main_coord = main_coord;
          floor[ocoord].machine.id = machine_id;
          floor[main_coord].machine.coords.push(ocoord);
          println!("Tagged @{} as part of machine {}", ocoord, machine_id);
          // Find all neighbors. Lazily include the one you just came from. It'll be ignored.
          auto_layout_tag_machine(floor, floor[ocoord].coord_u, main_coord, machine_id);
          auto_layout_tag_machine(floor, floor[ocoord].coord_r, main_coord, machine_id);
          auto_layout_tag_machine(floor, floor[ocoord].coord_d, main_coord, machine_id);
          auto_layout_tag_machine(floor, floor[ocoord].coord_l, main_coord, machine_id);
        } else {
          // Since we're expanding from a single machine cell, we should be finding and tagging all neighboring machine cells in the same way. As such, I don't think it should be possible to have a machine of a different kind here.
          // (This may be different in the future, I don't think it's a hard long term requirement, but right now it is)
          // (One example where this won't be true anymore is when placing machines adjacent to each other. But in that case there's less auto-layout? Or maybe it's just possible in that case. Dunno yet.)
          assert_eq!(floor[ocoord].machine.main_coord, main_coord, "if neighbor is not an unknown machine then it has to be part of the same machine");
        }
      }
    }
  }
}
fn get_kind_at(floor: &mut [Cell; FLOOR_CELLS_WH], coord: Option<usize>) -> CellKind {
  return match coord {
    None => CellKind::Empty,
    Some(coord) => floor[coord].kind,
  };
}

pub fn get_edge_neighbor(x: usize, y: usize, coord: usize) -> usize {
  return
    if y == 0 { to_coord_down(coord) }
    else if x == FLOOR_CELLS_W - 1 { to_coord_left(coord) }
    else if y == FLOOR_CELLS_H - 1 { to_coord_up(coord) }
    else if x == 0 { to_coord_right(coord) }
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

pub fn to_xy_f(index: usize) -> (usize, usize) {
  to_xy(index)
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
