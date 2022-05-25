// We don't need to use all of them :)

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Port {
  Inbound,
  Outbound,
  None,
}

#[derive(Clone, Copy, Debug)]
pub enum Line {
  Topper = 0,
  Middle = 1,
  Bottom = 2,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum BeltType {
  NONE = 0,
  U_R = 1,
  R_U = 2,
  R_D = 3,
  D_R = 4,
  D_L = 5,
  L_D = 6,
  L_U = 7,
  U_L = 8,
  U_D = 9,
  D_U = 10,
  L_R = 11,
  R_L = 12,
  U_LR = 13,
  RU_L = 14,
  LU_R = 15,
  L_RU = 16,
  LR_U = 17,
  R_LU = 18,
  R_DU = 19,
  RU_D = 20,
  DR_U = 21,
  DU_R = 22,
  U_DR = 23,
  D_RU = 24,
  D_LR = 25,
  DL_R = 26,
  DR_L = 27,
  LR_D = 28,
  L_DR = 29,
  R_DL = 30,
  L_DU = 31,
  LU_D = 32,
  DL_U = 33,
  DU_L = 34,
  U_DL = 35,
  D_UL = 36,
  U_DLR = 37,
  R_DLU = 38,
  D_LRU = 39,
  L_DRU = 40,
  RU_DL = 41,
  DU_LR = 42,
  LU_DR = 43,
  LD_RU = 44,
  DR_LU = 45,
  LR_DU = 46,
  DLR_U = 47,
  DLU_R = 48,
  RLU_D = 49,
  DRU_L = 50,
  INVALID = 51,
}
// Keep in sync...
pub const CELL_BELT_TYPE_COUNT: usize = (BeltType::INVALID as usize) + 1;

pub fn port_config_to_belt(up: Port, right: Port, down: Port, left: Port) -> BeltMeta {
  match   (up,          right,          down,           left) {
    (Port::Inbound,  Port::Inbound,  Port::Inbound,  Port::Inbound) =>   CELL_BELT_INVALID,
    (Port::Inbound,  Port::Inbound,  Port::Inbound,  Port::Outbound) =>  CELL_BELT_DRU_L,
    (Port::Inbound,  Port::Inbound,  Port::Inbound,  Port::None) =>      CELL_BELT_INVALID,
    (Port::Inbound,  Port::Inbound,  Port::Outbound, Port::Inbound) =>   CELL_BELT_LRU_D,
    (Port::Inbound,  Port::Inbound,  Port::Outbound, Port::Outbound) =>  CELL_BELT_RU_DL,
    (Port::Inbound,  Port::Inbound,  Port::Outbound, Port::None) =>      CELL_BELT_RU_D,
    (Port::Inbound,  Port::Inbound,  Port::None,     Port::Inbound) =>   CELL_BELT_INVALID,
    (Port::Inbound,  Port::Inbound,  Port::None,     Port::Outbound) =>  CELL_BELT_RU_L,
    (Port::Inbound,  Port::Inbound,  Port::None,     Port::None) =>      CELL_BELT_INVALID,
    (Port::Inbound,  Port::Outbound, Port::Inbound,  Port::Inbound) =>   CELL_BELT_DLU_R,
    (Port::Inbound,  Port::Outbound, Port::Inbound,  Port::Outbound) =>  CELL_BELT_DU_LR,
    (Port::Inbound,  Port::Outbound, Port::Inbound,  Port::None) =>      CELL_BELT_DU_R,
    (Port::Inbound,  Port::Outbound, Port::Outbound, Port::Inbound) =>   CELL_BELT_LU_DR,
    (Port::Inbound,  Port::Outbound, Port::Outbound, Port::Outbound) =>  CELL_BELT_U_DLR,
    (Port::Inbound,  Port::Outbound, Port::Outbound, Port::None) =>      CELL_BELT_U_DR,
    (Port::Inbound,  Port::Outbound, Port::None,     Port::Inbound) =>   CELL_BELT_LU_R,
    (Port::Inbound,  Port::Outbound, Port::None,     Port::Outbound) =>  CELL_BELT_U_LR,
    (Port::Inbound,  Port::Outbound, Port::None,     Port::None) =>      CELL_BELT_U_R,
    (Port::Inbound,  Port::None,     Port::Inbound,  Port::Inbound) =>   CELL_BELT_INVALID,
    (Port::Inbound,  Port::None,     Port::Inbound,  Port::Outbound) =>  CELL_BELT_DU_L,
    (Port::Inbound,  Port::None,     Port::Inbound,  Port::None) =>      CELL_BELT_INVALID,
    (Port::Inbound,  Port::None,     Port::Outbound, Port::Inbound) =>   CELL_BELT_LU_D,
    (Port::Inbound,  Port::None,     Port::Outbound, Port::Outbound) =>  CELL_BELT_U_DL,
    (Port::Inbound,  Port::None,     Port::Outbound, Port::None) =>      CELL_BELT_U_D,
    (Port::Inbound,  Port::None,     Port::None,     Port::Inbound) =>   CELL_BELT_INVALID,
    (Port::Inbound,  Port::None,     Port::None,     Port::Outbound) =>  CELL_BELT_U_L,
    (Port::Inbound,  Port::None,     Port::None,     Port::None) =>      CELL_BELT_INVALID,
    (Port::Outbound, Port::Inbound,  Port::Inbound,  Port::Inbound) =>   CELL_BELT_DLR_U,
    (Port::Outbound, Port::Inbound,  Port::Inbound,  Port::Outbound) =>  CELL_BELT_DR_LU,
    (Port::Outbound, Port::Inbound,  Port::Inbound,  Port::None) =>      CELL_BELT_DR_U,
    (Port::Outbound, Port::Inbound,  Port::Outbound, Port::Inbound) =>   CELL_BELT_LR_DU,
    (Port::Outbound, Port::Inbound,  Port::Outbound, Port::Outbound) =>  CELL_BELT_R_DLU,
    (Port::Outbound, Port::Inbound,  Port::Outbound, Port::None) =>      CELL_BELT_R_DU,
    (Port::Outbound, Port::Inbound,  Port::None,     Port::Inbound) =>   CELL_BELT_LR_U,
    (Port::Outbound, Port::Inbound,  Port::None,     Port::Outbound) =>  CELL_BELT_R_LU,
    (Port::Outbound, Port::Inbound,  Port::None,     Port::None) =>      CELL_BELT_R_U,
    (Port::Outbound, Port::Outbound, Port::Inbound,  Port::Inbound) =>   CELL_BELT_DL_RU,
    (Port::Outbound, Port::Outbound, Port::Inbound,  Port::Outbound) =>  CELL_BELT_D_LRU,
    (Port::Outbound, Port::Outbound, Port::Inbound,  Port::None) =>      CELL_BELT_D_RU,
    (Port::Outbound, Port::Outbound, Port::Outbound, Port::Inbound) =>   CELL_BELT_L_DRU,
    (Port::Outbound, Port::Outbound, Port::Outbound, Port::Outbound) =>  CELL_BELT_INVALID,
    (Port::Outbound, Port::Outbound, Port::Outbound, Port::None) =>      CELL_BELT_INVALID,
    (Port::Outbound, Port::Outbound, Port::None,     Port::Inbound) =>   CELL_BELT_L_RU,
    (Port::Outbound, Port::Outbound, Port::None,     Port::Outbound) =>  CELL_BELT_INVALID,
    (Port::Outbound, Port::Outbound, Port::None,     Port::None) =>      CELL_BELT_INVALID,
    (Port::Outbound, Port::None,     Port::Inbound,  Port::Inbound) =>   CELL_BELT_DL_U,
    (Port::Outbound, Port::None,     Port::Inbound,  Port::Outbound) =>  CELL_BELT_D_LU,
    (Port::Outbound, Port::None,     Port::Inbound,  Port::None) =>      CELL_BELT_D_U,
    (Port::Outbound, Port::None,     Port::Outbound, Port::Inbound) =>   CELL_BELT_L_DU,
    (Port::Outbound, Port::None,     Port::Outbound, Port::Outbound) =>  CELL_BELT_INVALID,
    (Port::Outbound, Port::None,     Port::Outbound, Port::None) =>      CELL_BELT_INVALID,
    (Port::Outbound, Port::None,     Port::None,     Port::Inbound) =>   CELL_BELT_L_U,
    (Port::Outbound, Port::None,     Port::None,     Port::Outbound) =>  CELL_BELT_INVALID,
    (Port::Outbound, Port::None,     Port::None,     Port::None) =>      CELL_BELT_INVALID,
    (Port::None,     Port::Inbound,  Port::Inbound,  Port::Inbound) =>   CELL_BELT_INVALID,
    (Port::None,     Port::Inbound,  Port::Inbound,  Port::Outbound) =>  CELL_BELT_DR_L,
    (Port::None,     Port::Inbound,  Port::Inbound,  Port::None) =>      CELL_BELT_INVALID,
    (Port::None,     Port::Inbound,  Port::Outbound, Port::Inbound) =>   CELL_BELT_LR_D,
    (Port::None,     Port::Inbound,  Port::Outbound, Port::Outbound) =>  CELL_BELT_R_DL,
    (Port::None,     Port::Inbound,  Port::Outbound, Port::None) =>      CELL_BELT_R_D,
    (Port::None,     Port::Inbound,  Port::None,     Port::Inbound) =>   CELL_BELT_INVALID,
    (Port::None,     Port::Inbound,  Port::None,     Port::Outbound) =>  CELL_BELT_R_L,
    (Port::None,     Port::Inbound,  Port::None,     Port::None) =>      CELL_BELT_INVALID,
    (Port::None,     Port::Outbound, Port::Inbound,  Port::Inbound) =>   CELL_BELT_DL_R,
    (Port::None,     Port::Outbound, Port::Inbound,  Port::Outbound) =>  CELL_BELT_D_LR,
    (Port::None,     Port::Outbound, Port::Inbound,  Port::None) =>      CELL_BELT_D_R,
    (Port::None,     Port::Outbound, Port::Outbound, Port::Inbound) =>   CELL_BELT_L_DR,
    (Port::None,     Port::Outbound, Port::Outbound, Port::Outbound) =>  CELL_BELT_INVALID,
    (Port::None,     Port::Outbound, Port::Outbound, Port::None) =>      CELL_BELT_INVALID,
    (Port::None,     Port::Outbound, Port::None,     Port::Inbound) =>   CELL_BELT_L_R,
    (Port::None,     Port::Outbound, Port::None,     Port::Outbound) =>  CELL_BELT_INVALID,
    (Port::None,     Port::Outbound, Port::None,     Port::None) =>      CELL_BELT_INVALID,
    (Port::None,     Port::None,     Port::Inbound,  Port::Inbound) =>   CELL_BELT_INVALID,
    (Port::None,     Port::None,     Port::Inbound,  Port::Outbound) =>  CELL_BELT_D_L,
    (Port::None,     Port::None,     Port::Inbound,  Port::None) =>      CELL_BELT_INVALID,
    (Port::None,     Port::None,     Port::Outbound, Port::Inbound) =>   CELL_BELT_L_D,
    (Port::None,     Port::None,     Port::Outbound, Port::Outbound) =>  CELL_BELT_INVALID,
    (Port::None,     Port::None,     Port::Outbound, Port::None) =>      CELL_BELT_INVALID,
    (Port::None,     Port::None,     Port::None,     Port::Inbound) =>   CELL_BELT_INVALID,
    (Port::None,     Port::None,     Port::None,     Port::Outbound) =>  CELL_BELT_INVALID,
    (Port::None,     Port::None,     Port::None,     Port::None) =>      CELL_BELT_INVALID,
  }
}

#[derive(Debug)]
pub struct BeltMeta {
  pub btype: BeltType,
  pub dbg: &'static str,
  pub src: &'static str,
  pub direction_u: Port,
  pub direction_r: Port,
  pub direction_d: Port,
  pub direction_l: Port,
  // simplify cli output painting
  pub cli_out_seg_u: char,
  pub cli_out_seg_r: char,
  pub cli_out_seg_d: char,
  pub cli_out_seg_l: char,
  pub cli_out_seg_c: char, // center
  pub cli_out_box_lu: char,
  pub cli_out_box_u: char,
  pub cli_out_box_ru: char,
  pub cli_out_box_l: char,
  pub cli_out_box_r: char,
  pub cli_out_box_dl: char,
  pub cli_out_box_d: char,
  pub cli_out_box_dr: char,
}

// https://en.wikipedia.org/wiki/Box-drawing_character

const BOX_ARROW_U: char = '^';
const BOX_ARROW_R: char = '>';
const BOX_ARROW_D: char = 'v';
const BOX_ARROW_L: char = '<';
const BOX_LU: char = '┌';
const BOX_U: char =  '─';
const BOX_RU: char = '┐';
const BOX_L: char =  '│';
const BOX_R: char =  '│';
const BOX_DL: char = '└';
const BOX_D: char =  '─';
const BLX_DR: char = '┘';
const BOX_EQ_V: char = '║';
const BOX_EQ_H: char = '═';
// const BOX_SEG_U: char = '│';
// const BOX_SEG_R: char = '─';
// const BOX_SEG_D: char = '│';
// const BOX_SEG_L: char = '─';
// const BOX_SEG_C_DL: char = '┐';
// const BOX_SEG_C_DLR: char = '┬';
// const BOX_SEG_C_DLRU: char = '┼';
// const BOX_SEG_C_DLU: char = '┤';
// const BOX_SEG_C_DRU: char = '├';
// const BOX_SEG_C_DR: char = '┌';
// const BOX_SEG_C_DU: char = '│';
// const BOX_SEG_C_LR: char = '─';
// const BOX_SEG_C_LRU: char = '┴';
// const BOX_SEG_C_LU: char = '┘';
// const BOX_SEG_C_RU: char = '└';

// ┌─┐
// │ │
// └─┘
pub const CELL_BELT_NONE: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "BELT_NONE",
  src: "./img/none.png",
  direction_u: Port::None,
  direction_r: Port::None,
  direction_d: Port::None,
  direction_l: Port::None,
  cli_out_seg_u: ' ',
  cli_out_seg_r: ' ',
  cli_out_seg_d: ' ',
  cli_out_seg_l: ' ',
  cli_out_seg_c: ' ',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// │ │
// └─┘
pub const CELL_BELT_INVALID: BeltMeta = BeltMeta {
  btype: BeltType::INVALID,
  dbg: "BELT_INVALID",
  src: "./img/invalid.png",
  direction_u: Port::None,
  direction_r: Port::None,
  direction_d: Port::None,
  direction_l: Port::None,
  cli_out_seg_u: '!',
  cli_out_seg_r: '!',
  cli_out_seg_d: '!',
  cli_out_seg_l: '!',
  cli_out_seg_c: '!',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌║┐
// ═ ═
// └║┘
pub const CELL_MACHINE: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_MACHINE",
  src: "./img/todo.png",
  direction_u: Port::None,
  direction_r: Port::None,
  direction_d: Port::None,
  direction_l: Port::None,
  cli_out_seg_u: ' ',
  cli_out_seg_r: ' ',
  cli_out_seg_d: ' ',
  cli_out_seg_l: ' ',
  cli_out_seg_c: ' ',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_EQ_V,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_EQ_H,
  cli_out_box_r: BOX_EQ_H,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_EQ_V,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// │ │
// └v┘
pub const CELL_SUPPLY_U: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_SUPPLY_U",
  src: "./img/todo.png",
  direction_u: Port::None,
  direction_r: Port::None,
  direction_d: Port::Outbound,
  direction_l: Port::None,
  cli_out_seg_u: ' ',
  cli_out_seg_r: ' ',
  cli_out_seg_d: ' ',
  cli_out_seg_l: ' ',
  cli_out_seg_c: ' ',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// < │
// └─┘
pub const CELL_SUPPLY_R: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_SUPPLY_R",
  src: "./img/todo.png",
  direction_u: Port::None,
  direction_r: Port::None,
  direction_d: Port::None,
  direction_l: Port::Outbound,
  cli_out_seg_u: ' ',
  cli_out_seg_r: ' ',
  cli_out_seg_d: ' ',
  cli_out_seg_l: ' ',
  cli_out_seg_c: ' ',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// │ │
// └─┘
pub const CELL_SUPPLY_D: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_SUPPLY_D",
  src: "./img/todo.png",
  direction_u: Port::Outbound,
  direction_r: Port::None,
  direction_d: Port::None,
  direction_l: Port::None,
  cli_out_seg_u: ' ',
  cli_out_seg_r: ' ',
  cli_out_seg_d: ' ',
  cli_out_seg_l: ' ',
  cli_out_seg_c: ' ',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// │ >
// └─┘
pub const CELL_SUPPLY_L: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_SUPPLY_L",
  src: "./img/todo.png",
  direction_u: Port::None,
  direction_r: Port::Outbound,
  direction_d: Port::None,
  direction_l: Port::None,
  cli_out_seg_u: ' ',
  cli_out_seg_r: ' ',
  cli_out_seg_d: ' ',
  cli_out_seg_l: ' ',
  cli_out_seg_c: ' ',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// │ │
// └^┘
pub const CELL_DEMAND_U: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_DEMAND_U",
  src: "./img/todo.png",
  direction_u: Port::Inbound,
  direction_r: Port::None,
  direction_d: Port::None,
  direction_l: Port::None,
  cli_out_seg_u: ' ',
  cli_out_seg_r: ' ',
  cli_out_seg_d: ' ',
  cli_out_seg_l: ' ',
  cli_out_seg_c: ' ',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// > │ 
// └─┘
pub const CELL_DEMAND_R: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_DEMAND_R",
  src: "./img/todo.png",
  direction_u: Port::None,
  direction_r: Port::Inbound,
  direction_d: Port::None,
  direction_l: Port::None,
  cli_out_seg_u: ' ',
  cli_out_seg_r: ' ',
  cli_out_seg_d: ' ',
  cli_out_seg_l: ' ',
  cli_out_seg_c: ' ',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// │ │
// └─┘
pub const CELL_DEMAND_D: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_DEMAND_D",
  src: "./img/todo.png",
  direction_u: Port::None,
  direction_r: Port::None,
  direction_d: Port::Inbound,
  direction_l: Port::None,
  cli_out_seg_u: ' ',
  cli_out_seg_r: ' ',
  cli_out_seg_d: ' ',
  cli_out_seg_l: ' ',
  cli_out_seg_c: ' ',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// │ < 
// └─┘
pub const CELL_DEMAND_L: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "CELL_DEMAND_L",
  src: "./img/todo.png",
  direction_u: Port::None,
  direction_r: Port::None,
  direction_d: Port::None,
  direction_l: Port::Inbound,
  cli_out_seg_u: ' ',
  cli_out_seg_r: ' ',
  cli_out_seg_d: ' ',
  cli_out_seg_l: ' ',
  cli_out_seg_c: ' ',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// │ >
// └─┘
pub const CELL_BELT_U_R: BeltMeta = BeltMeta {
  btype: BeltType::U_R,
  dbg: "BELT_U_R",
  src: "./img/u_r.png",
  direction_u: Port::Inbound,
  direction_r: Port::Outbound,
  direction_d: Port::None,
  direction_l: Port::None,
  cli_out_seg_u: '║',
  cli_out_seg_r: '═',
  cli_out_seg_d: ' ',
  cli_out_seg_l: ' ',
  cli_out_seg_c: '╚',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// │ <
// └─┘
pub const CELL_BELT_R_U: BeltMeta = BeltMeta {
  btype: BeltType::R_U,
  dbg: "BELT_R_U",
  src: "./img/r_u.png",
  direction_u: Port::Outbound,
  direction_r: Port::Inbound,
  direction_d: Port::None,
  direction_l: Port::None,
  cli_out_seg_u: '║',
  cli_out_seg_r: '═',
  cli_out_seg_d: ' ',
  cli_out_seg_l: ' ',
  cli_out_seg_c: '╚',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// │ <
// └v┘
pub const CELL_BELT_R_D: BeltMeta = BeltMeta {
  btype: BeltType::R_D,
  dbg: "BELT_R_D",
  src: "./img/r_d.png",
  direction_u: Port::None,
  direction_r: Port::Inbound,
  direction_d: Port::Outbound,
  direction_l: Port::None,
  cli_out_seg_u: ' ',
  cli_out_seg_r: '═',
  cli_out_seg_d: '║',
  cli_out_seg_l: ' ',
  cli_out_seg_c: '╔',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// │ >
// └^┘
pub const CELL_BELT_D_R: BeltMeta = BeltMeta {
  btype: BeltType::D_R,
  dbg: "BELT_D_R",
  src: "./img/d_r.png",
  direction_u: Port::None,
  direction_r: Port::Outbound,
  direction_d: Port::Inbound,
  direction_l: Port::None,
  cli_out_seg_u: ' ',
  cli_out_seg_r: '═',
  cli_out_seg_d: '║',
  cli_out_seg_l: ' ',
  cli_out_seg_c: '╔',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// < │
// └^┘
pub const CELL_BELT_D_L: BeltMeta = BeltMeta {
  btype: BeltType::D_L,
  dbg: "BELT_D_L",
  src: "./img/d_l.png",
  direction_u: Port::None,
  direction_r: Port::None,
  direction_d: Port::Inbound,
  direction_l: Port::Outbound,
  cli_out_seg_u: ' ',
  cli_out_seg_r: ' ',
  cli_out_seg_d: '║',
  cli_out_seg_l: '═',
  cli_out_seg_c: '╗',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// > │
// └v┘
pub const CELL_BELT_L_D: BeltMeta = BeltMeta {
  btype: BeltType::L_D,
  dbg: "BELT_L_D",
  src: "./img/l_d.png",
  direction_u: Port::None,
  direction_r: Port::None,
  direction_d: Port::Outbound,
  direction_l: Port::Inbound,
  cli_out_seg_u: ' ',
  cli_out_seg_r: ' ',
  cli_out_seg_d: '║',
  cli_out_seg_l: '═',
  cli_out_seg_c: '╗',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// < │
// └─┘
pub const CELL_BELT_L_U: BeltMeta = BeltMeta {
  btype: BeltType::L_U,
  dbg: "BELT_L_U",
  src: "./img/l_u.png",
  direction_u: Port::Outbound,
  direction_r: Port::None,
  direction_d: Port::None,
  direction_l: Port::Inbound,
  cli_out_seg_u: '║',
  cli_out_seg_r: ' ',
  cli_out_seg_d: ' ',
  cli_out_seg_l: '═',
  cli_out_seg_c: '╝',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// < │
// └─┘
pub const CELL_BELT_U_L: BeltMeta = BeltMeta {
  btype: BeltType::U_L,
  dbg: "BELT_U_L",
  src: "./img/u_l.png",
  direction_u: Port::Inbound,
  direction_r: Port::None,
  direction_d: Port::None,
  direction_l: Port::Outbound,
  cli_out_seg_u: '║',
  cli_out_seg_r: ' ',
  cli_out_seg_d: ' ',
  cli_out_seg_l: '═',
  cli_out_seg_c: '╝',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// │ │
// └v┘
pub const CELL_BELT_U_D: BeltMeta = BeltMeta {
  btype: BeltType::U_D,
  dbg: "BELT_U_D",
  src: "./img/u_d.png",
  direction_u: Port::Inbound,
  direction_r: Port::None,
  direction_d: Port::Outbound,
  direction_l: Port::None,
  cli_out_seg_u: '║',
  cli_out_seg_r: ' ',
  cli_out_seg_d: '║',
  cli_out_seg_l: ' ',
  cli_out_seg_c: '║',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// │ │
// └^┘
pub const CELL_BELT_D_U: BeltMeta = BeltMeta {
  btype: BeltType::D_U,
  dbg: "BELT_D_U",
  src: "./img/d_u.png",
  direction_u: Port::Outbound,
  direction_r: Port::None,
  direction_d: Port::Inbound,
  direction_l: Port::None,
  cli_out_seg_u: '║',
  cli_out_seg_r: ' ',
  cli_out_seg_d: '║',
  cli_out_seg_l: ' ',
  cli_out_seg_c: '║',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// > >
// └─┘
pub const CELL_BELT_L_R: BeltMeta = BeltMeta {
  btype: BeltType::L_R,
  dbg: "BELT_L_R",
  src: "./img/l_r.png",
  direction_u: Port::None,
  direction_r: Port::Outbound,
  direction_d: Port::None,
  direction_l: Port::Inbound,
  cli_out_seg_u: ' ',
  cli_out_seg_r: '═',
  cli_out_seg_d: ' ',
  cli_out_seg_l: '═',
  cli_out_seg_c: '═',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// < <
// └─┘
pub const CELL_BELT_R_L: BeltMeta = BeltMeta {
  btype: BeltType::R_L,
  dbg: "BELT_R_L",
  src: "./img/r_l.png",
  direction_u: Port::None,
  direction_r: Port::Inbound,
  direction_d: Port::None,
  direction_l: Port::Outbound,
  cli_out_seg_u: ' ',
  cli_out_seg_r: '═',
  cli_out_seg_d: ' ',
  cli_out_seg_l: '═',
  cli_out_seg_c: '═',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// < >
// └─┘
pub const CELL_BELT_U_LR: BeltMeta = BeltMeta {
  btype: BeltType::U_LR,
  dbg: "BELT_U_LR",
  src: "./img/u_lr.png",
  direction_u: Port::Inbound,
  direction_r: Port::Outbound,
  direction_d: Port::None,
  direction_l: Port::Outbound,
  cli_out_seg_u: '║',
  cli_out_seg_r: '═',
  cli_out_seg_d: ' ',
  cli_out_seg_l: '═',
  cli_out_seg_c: '╩',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// < <
// └─┘
pub const CELL_BELT_RU_L: BeltMeta = BeltMeta {
  btype: BeltType::RU_L,
  dbg: "BELT_RU_L",
  src: "./img/ru_l.png",
  direction_u: Port::Inbound,
  direction_r: Port::Inbound,
  direction_d: Port::None,
  direction_l: Port::Outbound,
  cli_out_seg_u: '║',
  cli_out_seg_r: '═',
  cli_out_seg_d: ' ',
  cli_out_seg_l: '═',
  cli_out_seg_c: '╩',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// > >
// └─┘
pub const CELL_BELT_LU_R: BeltMeta = BeltMeta {
  btype: BeltType::LU_R,
  dbg: "BELT_LU_R",
  src: "./img/lu_r.png",
  direction_u: Port::Inbound,
  direction_r: Port::Outbound,
  direction_d: Port::None,
  direction_l: Port::Inbound,
  cli_out_seg_u: '║',
  cli_out_seg_r: '═',
  cli_out_seg_d: ' ',
  cli_out_seg_l: '═',
  cli_out_seg_c: '╩',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// > >
// └─┘
pub const CELL_BELT_L_RU: BeltMeta = BeltMeta {
  btype: BeltType::L_RU,
  dbg: "BELT_L_RU",
  src: "./img/l_ru.png",
  direction_u: Port::Outbound,
  direction_r: Port::Outbound,
  direction_d: Port::None,
  direction_l: Port::Inbound,
  cli_out_seg_u: '║',
  cli_out_seg_r: '═',
  cli_out_seg_d: ' ',
  cli_out_seg_l: '═',
  cli_out_seg_c: '╩',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// > <
// └─┘
pub const CELL_BELT_LR_U: BeltMeta = BeltMeta {
  btype: BeltType::LR_U,
  dbg: "BELT_LR_U",
  src: "./img/lr_u.png",
  direction_u: Port::Outbound,
  direction_r: Port::Inbound,
  direction_d: Port::None,
  direction_l: Port::Inbound,
  cli_out_seg_u: '║',
  cli_out_seg_r: '═',
  cli_out_seg_d: ' ',
  cli_out_seg_l: '═',
  cli_out_seg_c: '╩',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// < <
// └─┘
pub const CELL_BELT_R_LU: BeltMeta = BeltMeta {
  btype: BeltType::R_LU,
  dbg: "BELT_R_LU",
  src: "./img/r_lu.png",
  direction_u: Port::Outbound,
  direction_r: Port::Inbound,
  direction_d: Port::None,
  direction_l: Port::Outbound,
  cli_out_seg_u: '║',
  cli_out_seg_r: '═',
  cli_out_seg_d: ' ',
  cli_out_seg_l: '═',
  cli_out_seg_c: '╩',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_D,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// │ <
// └v┘
pub const CELL_BELT_R_DU: BeltMeta = BeltMeta {
  btype: BeltType::R_DU,
  dbg: "BELT_R_DU",
  src: "./img/r_du.png",
  direction_u: Port::Outbound,
  direction_r: Port::Inbound,
  direction_d: Port::Outbound,
  direction_l: Port::None,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// │ <
// └v┘
pub const CELL_BELT_RU_D: BeltMeta = BeltMeta {
  btype: BeltType::RU_D,
  dbg: "BELT_RU_D",
  src: "./img/ru_d.png",
  direction_u: Port::Inbound,
  direction_r: Port::Inbound,
  direction_d: Port::Outbound,
  direction_l: Port::None,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// │ <
// └^┘
pub const CELL_BELT_DR_U: BeltMeta = BeltMeta {
  btype: BeltType::DR_U,
  dbg: "BELT_DR_U",
  src: "./img/dr_u.png",
  direction_u: Port::None,
  direction_r: Port::Inbound,
  direction_d: Port::Inbound,
  direction_l: Port::Outbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// │ >
// └^┘
pub const CELL_BELT_DU_R: BeltMeta = BeltMeta {
  btype: BeltType::DU_R,
  dbg: "BELT_DU_R",
  src: "./img/du_r.png",
  direction_u: Port::Inbound,
  direction_r: Port::Outbound,
  direction_d: Port::Inbound,
  direction_l: Port::None,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// │ >
// └v┘
pub const CELL_BELT_U_DR: BeltMeta = BeltMeta {
  btype: BeltType::U_DR,
  dbg: "BELT_U_DR",
  src: "./img/u_dr.png",
  direction_u: Port::Inbound,
  direction_r: Port::Outbound,
  direction_d: Port::Outbound,
  direction_l: Port::None,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// │ >
// └^┘
pub const CELL_BELT_D_RU: BeltMeta = BeltMeta {
  btype: BeltType::D_RU,
  dbg: "BELT_D_RU",
  src: "./img/d_ru.png",
  direction_u: Port::Outbound,
  direction_r: Port::Outbound,
  direction_d: Port::Inbound,
  direction_l: Port::None,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_L,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// < >
// └^┘
pub const CELL_BELT_D_LR: BeltMeta = BeltMeta {
  btype: BeltType::D_LR,
  dbg: "BELT_D_LR",
  src: "./img/d_lr.png",
  direction_u: Port::None,
  direction_r: Port::Outbound,
  direction_d: Port::Inbound,
  direction_l: Port::Outbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// > >
// └^┘
pub const CELL_BELT_DL_R: BeltMeta = BeltMeta {
  btype: BeltType::DL_R,
  dbg: "BELT_DL_R",
  src: "./img/dl_r.png",
  direction_u: Port::None,
  direction_r: Port::Outbound,
  direction_d: Port::Inbound,
  direction_l: Port::Inbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// < <
// └^┘
pub const CELL_BELT_DR_L: BeltMeta = BeltMeta {
  btype: BeltType::DR_L,
  dbg: "BELT_DR_L",
  src: "./img/dr_l.png",
  direction_u: Port::None,
  direction_r: Port::Inbound,
  direction_d: Port::Inbound,
  direction_l: Port::Outbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// > <
// └v┘
pub const CELL_BELT_LR_D: BeltMeta = BeltMeta {
  btype: BeltType::LR_D,
  dbg: "BELT_LR_D",
  src: "./img/dr_l.png",
  direction_u: Port::None,
  direction_r: Port::Inbound,
  direction_d: Port::Outbound,
  direction_l: Port::Inbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// > >
// └v┘
pub const CELL_BELT_L_DR: BeltMeta = BeltMeta {
  btype: BeltType::L_DR,
  dbg: "BELT_L_DR",
  src: "./img/dr_l.png",
  direction_u: Port::None,
  direction_r: Port::Outbound,
  direction_d: Port::Outbound,
  direction_l: Port::Inbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌─┐
// < <
// └v┘
pub const CELL_BELT_R_DL: BeltMeta = BeltMeta {
  btype: BeltType::R_DL,
  dbg: "BELT_R_DL",
  src: "./img/r_dl.png",
  direction_u: Port::None,
  direction_r: Port::Inbound,
  direction_d: Port::Outbound,
  direction_l: Port::Outbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// > │
// └v┘
pub const CELL_BELT_L_DU: BeltMeta = BeltMeta {
  btype: BeltType::L_DU,
  dbg: "BELT_L_DU",
  src: "./img/l_du.png",
  direction_u: Port::Outbound,
  direction_r: Port::None,
  direction_d: Port::Outbound,
  direction_l: Port::Inbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// > │
// └v┘
pub const CELL_BELT_LU_D: BeltMeta = BeltMeta {
  btype: BeltType::LU_D,
  dbg: "BELT_LU_D",
  src: "./img/lu_d.png",
  direction_u: Port::Inbound,
  direction_r: Port::None,
  direction_d: Port::Outbound,
  direction_l: Port::Inbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// > │
// └^┘
pub const CELL_BELT_DL_U: BeltMeta = BeltMeta {
  btype: BeltType::DL_U,
  dbg: "BELT_DL_U",
  src: "./img/dl_u.png",
  direction_u: Port::Outbound,
  direction_r: Port::None,
  direction_d: Port::Inbound,
  direction_l: Port::Inbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// < │
// └^┘
pub const CELL_BELT_DU_L: BeltMeta = BeltMeta {
  btype: BeltType::DU_L,
  dbg: "BELT_DU_L",
  src: "./img/du_l.png",
  direction_u: Port::Inbound,
  direction_r: Port::None,
  direction_d: Port::Inbound,
  direction_l: Port::Outbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// < │
// └v┘
pub const CELL_BELT_U_DL: BeltMeta = BeltMeta {
  btype: BeltType::U_DL,
  dbg: "BELT_U_DL",
  src: "./img/u_dl.png",
  direction_u: Port::Inbound,
  direction_r: Port::None,
  direction_d: Port::Outbound,
  direction_l: Port::Outbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// < │
// └^┘
pub const CELL_BELT_D_LU: BeltMeta = BeltMeta {
  btype: BeltType::D_UL,
  dbg: "BELT_D_UL",
  src: "./img/d_ul.png",
  direction_u: Port::Outbound,
  direction_r: Port::None,
  direction_d: Port::Inbound,
  direction_l: Port::Outbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// < >
// └v┘
pub const CELL_BELT_U_DLR: BeltMeta = BeltMeta {
  btype: BeltType::U_DLR,
  dbg: "BELT_U_DLR",
  src: "./img/todo.png",
  direction_u: Port::Inbound,
  direction_r: Port::Outbound,
  direction_d: Port::Outbound,
  direction_l: Port::Outbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_ARROW_D,
  cli_out_box_u: BOX_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// < <
// └v┘
pub const CELL_BELT_R_DLU: BeltMeta = BeltMeta {
  btype: BeltType::R_DLU,
  dbg: "BELT_R_DLU",
  src: "./img/todo.png",
  direction_u: Port::Outbound,
  direction_r: Port::Inbound,
  direction_d: Port::Outbound,
  direction_l: Port::Outbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// < >
// └^┘
pub const CELL_BELT_D_LRU: BeltMeta = BeltMeta {
  btype: BeltType::D_LRU,
  dbg: "BELT_D_LRU",
  src: "./img/todo.png",
  direction_u: Port::Outbound,
  direction_r: Port::Outbound,
  direction_d: Port::Inbound,
  direction_l: Port::Outbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// > >
// └v┘
pub const CELL_BELT_L_DRU: BeltMeta = BeltMeta {
  btype: BeltType::L_DRU,
  dbg: "BELT_L_DRU",
  src: "./img/todo.png",
  direction_u: Port::Outbound,
  direction_r: Port::Outbound,
  direction_d: Port::Outbound,
  direction_l: Port::Inbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// < <
// └v┘
pub const CELL_BELT_RU_DL: BeltMeta = BeltMeta {
  btype: BeltType::RU_DL,
  dbg: "BELT_RU_DL",
  src: "./img/todo.png",
  direction_u: Port::Inbound,
  direction_r: Port::Inbound,
  direction_d: Port::Outbound,
  direction_l: Port::Outbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// < >
// └^┘
pub const CELL_BELT_DU_LR: BeltMeta = BeltMeta {
  btype: BeltType::DU_LR,
  dbg: "BELT_DU_LR",
  src: "./img/todo.png",
  direction_u: Port::Inbound,
  direction_r: Port::Outbound,
  direction_d: Port::Inbound,
  direction_l: Port::Outbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// > >
// └v┘
pub const CELL_BELT_LU_DR: BeltMeta = BeltMeta {
  btype: BeltType::LU_DR,
  dbg: "BELT_LU_DR",
  src: "./img/todo.png",
  direction_u: Port::Inbound,
  direction_r: Port::Outbound,
  direction_d: Port::Outbound,
  direction_l: Port::Inbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// > >
// └v┘
pub const CELL_BELT_DL_RU: BeltMeta = BeltMeta {
  btype: BeltType::LD_RU,
  dbg: "BELT_LD_RU",
  src: "./img/todo.png",
  direction_u: Port::Outbound,
  direction_r: Port::Outbound,
  direction_d: Port::Inbound,
  direction_l: Port::Inbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// < <
// └^┘
pub const CELL_BELT_DR_LU: BeltMeta = BeltMeta {
  btype: BeltType::DR_LU,
  dbg: "BELT_DR_LU",
  src: "./img/todo.png",
  direction_u: Port::Outbound,
  direction_r: Port::Inbound,
  direction_d: Port::Inbound,
  direction_l: Port::Outbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// > <
// └v┘
pub const CELL_BELT_LR_DU: BeltMeta = BeltMeta {
  btype: BeltType::LR_DU,
  dbg: "BELT_LR_DU",
  src: "./img/todo.png",
  direction_u: Port::Outbound,
  direction_r: Port::Inbound,
  direction_d: Port::Outbound,
  direction_l: Port::Inbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌^┐
// > <
// └^┘
pub const CELL_BELT_DLR_U: BeltMeta = BeltMeta {
  btype: BeltType::DLR_U,
  dbg: "BELT_DLR_U",
  src: "./img/todo.png",
  direction_u: Port::Outbound,
  direction_r: Port::Inbound,
  direction_d: Port::Inbound,
  direction_l: Port::Inbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_U,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// > >
// └^┘
pub const CELL_BELT_DLU_R: BeltMeta = BeltMeta {
  btype: BeltType::DLU_R,
  dbg: "BELT_DLU_R",
  src: "./img/todo.png",
  direction_u: Port::Inbound,
  direction_r: Port::Outbound,
  direction_d: Port::Inbound,
  direction_l: Port::Inbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_ARROW_R,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// > <
// └v┘
pub const CELL_BELT_LRU_D: BeltMeta = BeltMeta {
  btype: BeltType::RLU_D,
  dbg: "BELT_RLU_D",
  src: "./img/todo.png",
  direction_u: Port::Inbound,
  direction_r: Port::Inbound,
  direction_d: Port::Outbound,
  direction_l: Port::Inbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_R,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_D,
  cli_out_box_dr: BLX_DR,
};
// ┌v┐
// < <
// └^┘
pub const CELL_BELT_DRU_L: BeltMeta = BeltMeta {
  btype: BeltType::DRU_L,
  dbg: "BELT_DRU_L",
  src: "./img/todo.png",
  direction_u: Port::Inbound,
  direction_r: Port::Inbound,
  direction_d: Port::Inbound,
  direction_l: Port::Outbound,
  cli_out_seg_u: '?',
  cli_out_seg_r: '?',
  cli_out_seg_d: '?',
  cli_out_seg_l: '?',
  cli_out_seg_c: '?',
  cli_out_box_lu: BOX_LU,
  cli_out_box_u: BOX_ARROW_D,
  cli_out_box_ru: BOX_RU,
  cli_out_box_l: BOX_ARROW_L,
  cli_out_box_r: BOX_ARROW_L,
  cli_out_box_dl: BOX_DL,
  cli_out_box_d: BOX_ARROW_U,
  cli_out_box_dr: BLX_DR,
};


