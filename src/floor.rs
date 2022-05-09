use super::belt::*;
use super::cell::*;
use super::machine::*;
use super::options::*;
use super::part::*;
use super::state::*;

pub struct Floor {
  pub width: usize,
  pub height: usize,
  pub cells: Vec<Vec<Cell>>,
}

pub fn empty_floor() -> Floor {
  return Floor {
    width: 5,
    height: 5,
    cells: vec![
      vec![empty_cell(0, 0), empty_cell(1, 0), empty_cell(2, 0), empty_cell(3, 0), empty_cell(4, 0)],
      vec![empty_cell(0, 1), empty_cell(1, 1), empty_cell(2, 1), empty_cell(3, 1), empty_cell(4, 1)],
      vec![empty_cell(0, 2), empty_cell(1, 2), empty_cell(2, 2), empty_cell(3, 2), empty_cell(4, 2)],
      vec![empty_cell(0, 3), empty_cell(1, 3), empty_cell(2, 3), empty_cell(3, 3), empty_cell(4, 3)],
      vec![empty_cell(0, 4), empty_cell(1, 4), empty_cell(2, 4), empty_cell(3, 4), empty_cell(4, 4)],
    ]
  };
}



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

// https://en.wikipedia.org/wiki/Box-drawing_character
//
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

// ┌───┬┬───┬┬─v─┬┬───┬┬───┐
// │xxx││   ││ ║ ││   ││   │
// │xxx<<═══<<═╩═<<═══<<═╗ │
// │xxx││   ││   ││   ││ ║ │
// ├─v─┘└───┘└───┘└───┘└─^─┤
// ├─v─┐┌───┐┌───┐┌───┐┌─^─┤
// │ ║ ││   ││   ││   ││ ║ │
// │ ╚═>>═══>>═╗ ││   ││ ║ │
// │   ││   ││ ║ ││   ││ ║ │
// ├───┘└───┘└─v─┘└───┘└─^─┤
// ├───┐┌───┐┌─v─┐┌───┐┌─^─┤
// │   ││   ││yyy││   ││ ║ │
// │   ││   ││yyy││   ││ ║ │
// │   ││   ││yyy││   ││ ║ │
// ├───┘└───┘└─v─┘└───┘└─^─┤
// ├───┐┌───┐┌─v─┐┌───┐┌─^─┤
// │   ││   ││ ║ ││   ││ ║ │
// │   ││   ││ ║ ││   ││ ╚═<
// │   ││   ││ ║ ││   ││   │
// ├───┘└───┘└─v─┘└───┘└───┤
// ├───┐┌───┐┌─v─┐┌───┐┌───┤
// │   ││   ││ ║ ││   ││   │
// │   ││ ╔═<<═╝ ││   ││   │
// │   ││ ║ ││   ││   ││   │
// └───┴┴─v─┴┴───┴┴───┴┴───┘


pub fn test_floor() -> Floor {
  let machine1 = machine_cell(2, 2, Machine::Composer, Part { kind: PartKind::BlueWand, icon: 'b'}, part_none(2, 2), part_none(2, 2), Part { kind: PartKind::GoldenBlueWand, icon: 'g'}, -15, -3);
  let machine2 = machine_cell(2, 2, Machine::Smasher, Part { kind: PartKind::WoodenStick, icon: 'w'}, Part { kind: PartKind::Sapphire, icon: 's'}, part_none(2, 2), Part { kind: PartKind::BlueWand, icon: 'd'}, -25, -10);
  return Floor {
    width: 5,
    height: 5,
    cells: vec![
      // Note: enter transposed (!)
      vec![machine2, belt_cell(0, 1, BELT_U_R), empty_cell(0, 2), empty_cell(0, 3), empty_cell(0, 4)],
      vec![belt_cell(1, 0, BELT_R_L),  belt_cell(1, 1, BELT_L_R),  empty_cell(1, 2), empty_cell(1, 3), belt_cell(1, 4, BELT_R_D)],
      vec![belt_cell(2, 0, BELT_RU_L), belt_cell(2, 1, BELT_L_D),  machine1, belt_cell(2, 3, BELT_U_D),  belt_cell(2, 4, BELT_U_L)],
      vec![belt_cell(3, 0, BELT_R_L),  empty_cell(3, 1), empty_cell(3, 2), empty_cell(3, 3), empty_cell(3, 4)],
      vec![belt_cell(4, 0, BELT_D_L),  belt_cell(4, 1, BELT_D_U), belt_cell(4, 2, BELT_D_U), belt_cell(4, 3, BELT_R_U), empty_cell(4, 4)],
    ]
  };
}
