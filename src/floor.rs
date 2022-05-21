use super::belt::*;
use super::cell::*;
use super::machine::*;
use super::options::*;
use super::part::*;
use super::state::*;

pub struct Floor {
  pub width: usize,
  pub height: usize,
  pub sum: usize, // width * height (excluding the borders!)
  pub fsum: usize, // sum, including borders
  pub cells: [Cell; 49], // left to right, top to bottom
}

// The corners are never used
// The top/bottom/left/right most rows/cols are only used by demanders and suppliers
const EMPTY_FLOOR: Floor = Floor {
  width: 5, // actual cells
  height: 5,
  sum: 25,
  fsum: 49,
  cells: [
    empty_cell(0, 0), empty_cell(1, 0), empty_cell(2, 0), empty_cell(3, 0), empty_cell(4, 0), empty_cell(5, 0), empty_cell(6, 0),
    empty_cell(0, 1), empty_cell(1, 1), empty_cell(2, 1), empty_cell(3, 1), empty_cell(4, 1), empty_cell(5, 1), empty_cell(6, 1),
    empty_cell(0, 2), empty_cell(1, 2), empty_cell(2, 2), empty_cell(3, 2), empty_cell(4, 2), empty_cell(5, 2), empty_cell(6, 2),
    empty_cell(0, 3), empty_cell(1, 3), empty_cell(2, 3), empty_cell(3, 3), empty_cell(4, 3), empty_cell(5, 3), empty_cell(6, 3),
    empty_cell(0, 4), empty_cell(1, 4), empty_cell(2, 4), empty_cell(3, 4), empty_cell(4, 4), empty_cell(5, 4), empty_cell(6, 4),
    empty_cell(0, 5), empty_cell(1, 5), empty_cell(2, 5), empty_cell(3, 5), empty_cell(4, 5), empty_cell(5, 5), empty_cell(6, 5),
    empty_cell(0, 6), empty_cell(1, 6), empty_cell(2, 6), empty_cell(3, 6), empty_cell(4, 6), empty_cell(5, 6), empty_cell(6, 6),
  ],
};

// https://en.wikipedia.org/wiki/Box-drawing_character

// ┌─────────┐
// │# # # # #│
// │# # # # #│
// │# # # # #│
// │# # # # #│
// │# # # # #│
// └─────────┘

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
//                  ┌───┐                                          0     1    2  ┌───┐  4    5    6
// 0                │a A│                0      6                                │ 3 │
//       ┌────────────v────────────┐                                  ┌────────────v────────────┐
//       │┌───┐┌───┐┌─v─┐┌───┐┌───┐│                                  │┌───┐┌───┐┌─v─┐┌───┐┌───┐│
//       ││xxx││   ││ ║ ││   ││   ││                                  ││   ││   ││   ││   ││   ││
// 1     ││xxx<<═══<<═╩═<<═══<<═╗ ││     1                         7  ││ 8 << 9 << 10<< 11<< 12││ 13
//       ││xxx││   ││10 ││   ││ ║ ││            13                    ││   ││   ││   ││   ││   ││
//       │└─v─┘└───┘└───┘└───┘└─^─┘│                                  │└─v─┘└───┘└───┘└───┘└─^─┘│
//       │┌─v─┐┌───┐┌───┐┌───┐┌─^─┐│                                  │┌─v─┐┌───┐┌───┐┌───┐┌─^─┐│
//       ││ ║ ││   ││   ││   ││ ║ ││                                  ││   ││   ││   ││   ││   ││
// 2     ││ ╚═>>═══>>═╗ ││   ││ ║ ││     2                        14  ││ 15>> 16>> 17││ 18││ 19││ 20
//       ││15 ││   ││ ║ ││   ││ ║ ││            20                    ││   ││   ││   ││   ││   ││
//       │└───┘└───┘└─v─┘└───┘└─^─┘│                                  │└───┘└───┘└─v─┘└───┘└─^─┘│
//       │┌───┐┌───┐┌─v─┐┌───┐┌─^─┐│                                  │┌───┐┌───┐┌─v─┐┌───┐┌─^─┐│
//       ││   ││   ││yyy││   ││ ║ ││                                  ││   ││   ││   ││   ││   ││
// 3     ││   ││   ││yyy││   ││ ║ ││     3                        21  ││ 22││ 23││ 24││ 25││ 26││ 27
//       ││   ││   ││yyy││   ││ ║ ││            27                    ││   ││   ││   ││   ││   ││
//       │└───┘└───┘└─v─┘└───┘└─^─┘│                                  │└───┘└───┘└─v─┘└───┘└─^─┘│
//       │┌───┐┌───┐┌─v─┐┌───┐┌─^─┐│┌──┐                              │┌───┐┌───┐┌─v─┐┌───┐┌─^─┐│┌──┐
//       ││   ││   ││ ║ ││   ││ ║ │││a │                              ││   ││   ││   ││   ││   │││  │
// 4     ││   ││   ││3║1││   ││ ╚═<<│  │ 4                        28  ││ 29││ 30││ 31││ 32││ 33<<│34│
//       ││   ││   ││ ║ ││   ││   │││A │        34                    ││   ││   ││   ││   ││   │││  │
//       │└───┘└───┘└─v─┘└───┘└───┘│└──┘                              │└───┘└───┘└─v─┘└───┘└───┘│└──┘
//       │┌───┐┌───┐┌─v─┐┌───┐┌───┐│                                  │┌───┐┌───┐┌─v─┐┌───┐┌───┐│
//       ││   ││   ││ ║ ││   ││   ││                                  ││   ││   ││   ││   ││   ││
// 5     ││   ││ ╔═<<═╝ ││   ││   ││     5      41                35  ││ 36││ 37<< 38││ 39││ 40││ 41
//       ││   ││ ║ ││   ││   ││   ││                                  ││   ││   ││   ││   ││   ││
//       │└───┘└─v─┘└───┘└───┘└───┘│                                  │└───┘└─v─┘└───┘└───┘└───┘│
//       └───────v─────────────────┘                                  └───────v─────────────────┘
// 6           │ a │                     6      48                42     43 │ 44│  45   46   47   48
//             └───┘                                                        └───┘
//    0     1    2    3    4    5     6

pub fn test_floor_one_supply() -> Floor {
  let club = Part { kind: PartKind::WoodenStick, icon: 'w'};
  let gem = Part { kind: PartKind::Sapphire, icon: 's'};
  let wand = Part { kind: PartKind::BlueWand, icon: 'b'};
  let gwand = Part { kind: PartKind::GoldenBlueWand, icon: 'g'};

  let supply_top = supply_cell(3, 0, CELL_SUPPLY_U, club.clone(), 10000, 10000, -600);
  let supply_right = supply_cell(6, 4, CELL_SUPPLY_R, gem.clone(), 10000, 10000, -800);
  let demand_down = demand_cell(2, 6, CELL_DEMAND_D, gwand.clone(), 10000, 12000, -900);

  let machine1 = machine_cell(2, 2, Machine::Composer, wand.clone(), part_none(), part_none(), gwand.clone(), -15, -3);
  let machine2 = machine_cell(2, 2, Machine::Smasher, club.clone(), gem.clone(), part_none(), wand.clone(), -25, -10);

  let width: usize = 5;
  let height: usize = 5;

  return Floor {
    width,
    height,
    sum: width * height,
    fsum: (width+2) * (height+2),
    cells: [
      empty_cell(0, 0), empty_cell(1, 0), empty_cell(2, 0), supply_top, empty_cell(4, 0), empty_cell(5, 0), empty_cell(6, 0),
      empty_cell(0, 1), machine2, belt_cell(2, 1, CELL_BELT_R_L), belt_cell(3, 1, CELL_BELT_RU_L), belt_cell(4, 1, CELL_BELT_R_L), belt_cell(5, 1, CELL_BELT_D_L), empty_cell(6, 1),
      empty_cell(0, 2), belt_cell(1, 2, CELL_BELT_U_R), belt_cell(2, 2, CELL_BELT_L_R), belt_cell(3, 2, CELL_BELT_L_D), empty_cell(4, 2), belt_cell(5, 2, CELL_BELT_D_U), empty_cell(6, 2),
      empty_cell(0, 3), empty_cell(1, 3), empty_cell(2, 3), machine1, empty_cell(4, 3), belt_cell(5, 3, CELL_BELT_D_U), empty_cell(6, 3),
      empty_cell(0, 4), empty_cell(1, 4), empty_cell(2, 4), belt_cell(3, 4, CELL_BELT_U_D), empty_cell(4, 4), belt_cell(5, 4, CELL_BELT_R_U), empty_cell(6, 4),
      empty_cell(0, 5), empty_cell(1, 5), belt_cell(2, 5, CELL_BELT_R_D), belt_cell(3, 5, CELL_BELT_U_L), empty_cell(4, 5), empty_cell(5, 5), empty_cell(6, 5),
      empty_cell(0, 6), empty_cell(1, 6), demand_down, empty_cell(3, 6), empty_cell(4, 6), empty_cell(5, 6), empty_cell(6, 6),
    ]
  };
}
pub fn test_floor() -> Floor {
  let club = Part { kind: PartKind::WoodenStick, icon: 'w'};
  let gem = Part { kind: PartKind::Sapphire, icon: 's'};
  let wand = Part { kind: PartKind::BlueWand, icon: 'b'};
  let gwand = Part { kind: PartKind::GoldenBlueWand, icon: 'g'};

  let supply_top = supply_cell(3, 0, CELL_SUPPLY_U, club.clone(), 10000, 10000, -600);
  let supply_right = supply_cell(6, 4, CELL_SUPPLY_R, gem.clone(), 10000, 10000, -800);
  let demand_down = demand_cell(2, 6, CELL_DEMAND_D, gwand.clone(), 10000, 12000, -900);

  let machine1 = machine_cell(2, 2, Machine::Composer, wand.clone(), part_none(), part_none(), gwand.clone(), -15, -3);
  let machine2 = machine_cell(2, 2, Machine::Smasher, club.clone(), gem.clone(), part_none(), wand.clone(), -25, -10);

  let width: usize = 5;
  let height: usize = 5;

  return Floor {
    width,
    height,
    sum: width * height,
    fsum: (width+2) * (height+2),
    cells: [
      empty_cell(0, 0), empty_cell(1, 0), empty_cell(2, 0), supply_top, empty_cell(4, 0), empty_cell(5, 0), empty_cell(6, 0),
      empty_cell(0, 1), machine2, belt_cell(2, 1, CELL_BELT_R_L), belt_cell(3, 1, CELL_BELT_RU_L), belt_cell(4, 1, CELL_BELT_R_L), belt_cell(5, 1, CELL_BELT_D_L), empty_cell(6, 1),
      empty_cell(0, 2), belt_cell(1, 2, CELL_BELT_U_R), belt_cell(2, 2, CELL_BELT_L_R), belt_cell(3, 2, CELL_BELT_L_D), empty_cell(4, 2), belt_cell(5, 2, CELL_BELT_D_U), empty_cell(6, 2),
      empty_cell(0, 3), empty_cell(1, 3), empty_cell(2, 3), machine1, empty_cell(4, 3), belt_cell(5, 3, CELL_BELT_D_U), empty_cell(6, 3),
      empty_cell(0, 4), empty_cell(1, 4), empty_cell(2, 4), belt_cell(3, 4, CELL_BELT_U_D), empty_cell(4, 4), belt_cell(5, 4, CELL_BELT_R_U), supply_right,
      empty_cell(0, 5), empty_cell(1, 5), belt_cell(2, 5, CELL_BELT_R_D), belt_cell(3, 5, CELL_BELT_U_L), empty_cell(4, 5), empty_cell(5, 5), empty_cell(6, 5),
      empty_cell(0, 6), empty_cell(1, 6), demand_down, empty_cell(3, 6), empty_cell(4, 6), empty_cell(5, 6), empty_cell(6, 6),
    ]
  };
}

pub fn is_floor(x: usize, y: usize, w: usize, h: usize) -> bool {
  return x != 0 && y != 0 && x != w - 1 && y != h - 1;
}

pub fn is_edge(x: usize, y: usize, w: usize, h: usize) -> bool {
  return x == 0 || y == 0 || x == w - 1 || y == h - 1;
}

pub fn to_floor_xy(floor: &Floor, index: usize) -> (usize, usize, bool) {
  // Return 0,0 if index is not in the floor (so for edge cells)

  let w = floor.width;
  let h = floor.height;
  let x = index % w;
  let y = index / w; // int division

  return (x, y, is_floor(x, y, w, h));
}

pub fn to_edge_xy(floor: &Floor, index: usize) -> (usize, usize, bool) {
  let w = floor.width;
  let h = floor.height;
  let x = index % w;
  let y = index / w; // int division

  return (x, y, is_edge(x, y, w, h));
}

pub fn to_xy_f(floor: &Floor, index: usize) -> (usize, usize) {
  to_xy(index, floor.width)
}

pub fn to_xy(coord: usize, w: usize) -> (usize, usize) {
  // Return 0,0 if index is not in the floor (so for edge cells)

  let x = coord % w;
  let y = coord / w; // int division

  return (x, y);
}

pub fn to_coord_f(floor: &Floor, x: usize, y: usize) -> usize {
  to_coord(floor.width, x, y)
}

pub fn to_coord(w: usize, x: usize, y: usize) -> usize {
  return w * y + x;
}

pub fn to_coord_up(coord: usize, w: usize) -> usize {
  return coord - w;
}
pub fn to_coord_right(coord: usize, _w: usize) -> usize {
  return coord + 1;
}
pub fn to_coord_down(coord: usize, w: usize) -> usize {
  return coord + w;
}
pub fn to_coord_left(coord: usize, _w: usize) -> usize {
  return coord - 1;
}
