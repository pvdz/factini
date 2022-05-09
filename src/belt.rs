// We don't need to use all of them :)

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BeltDirection {
  In,
  Out,
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
}
// Keep in sync...
pub const BELT_TYPE_COUNT: usize = (BeltType::DRU_L as usize) + 1;

#[derive(Debug)]
pub struct BeltMeta {
  pub btype: BeltType,
  pub dbg: &'static str,
  pub src: &'static str,
  pub direction_u: BeltDirection,
  pub direction_r: BeltDirection,
  pub direction_d: BeltDirection,
  pub direction_l: BeltDirection,
  pub cli_u: char,
  pub cli_r: char,
  pub cli_d: char,
  pub cli_l: char,
  pub cli_c: char, // center
}

// ┌─┐
// │ │
// └─┘
pub const BELT_NONE: BeltMeta = BeltMeta {
  btype: BeltType::NONE,
  dbg: "BELT_NONE",
  src: "./img/none.png",
  direction_u: BeltDirection::None,
  direction_r: BeltDirection::None,
  direction_d: BeltDirection::None,
  direction_l: BeltDirection::None,
  cli_u: ' ',
  cli_r: ' ',
  cli_d: ' ',
  cli_l: ' ',
  cli_c: ' ',
};
// ┌v┐
// │ >
// └─┘
pub const BELT_U_R: BeltMeta = BeltMeta {
  btype: BeltType::U_R,
  dbg: "BELT_U_R",
  src: "./img/u_r.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::None,
  direction_l: BeltDirection::None,
  cli_u: '║',
  cli_r: '═',
  cli_d: ' ',
  cli_l: ' ',
  cli_c: '╚',
};
// ┌─┐
// │ <
// └v┘
pub const BELT_R_U: BeltMeta = BeltMeta {
  btype: BeltType::R_U,
  dbg: "BELT_R_U",
  src: "./img/r_u.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::None,
  direction_l: BeltDirection::None,
  cli_u: '║',
  cli_r: '═',
  cli_d: ' ',
  cli_l: ' ',
  cli_c: '╚',
};
// ┌─┐
// │ <
// └v┘
pub const BELT_R_D: BeltMeta = BeltMeta {
  btype: BeltType::R_D,
  dbg: "BELT_R_D",
  src: "./img/r_d.png",
  direction_u: BeltDirection::None,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::None,
  cli_u: ' ',
  cli_r: '═',
  cli_d: '║',
  cli_l: ' ',
  cli_c: '╔',
};
// ┌─┐
// │ >
// └^┘
pub const BELT_D_R: BeltMeta = BeltMeta {
  btype: BeltType::D_R,
  dbg: "BELT_D_R",
  src: "./img/d_r.png",
  direction_u: BeltDirection::None,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::None,
  cli_u: ' ',
  cli_r: '═',
  cli_d: '║',
  cli_l: ' ',
  cli_c: '╔',
};
// ┌─┐
// < │
// └^┘
pub const BELT_D_L: BeltMeta = BeltMeta {
  btype: BeltType::D_L,
  dbg: "BELT_D_L",
  src: "./img/d_l.png",
  direction_u: BeltDirection::None,
  direction_r: BeltDirection::None,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::Out,
  cli_u: ' ',
  cli_r: ' ',
  cli_d: '║',
  cli_l: '═',
  cli_c: '╗',
};
// ┌─┐
// > │
// └v┘
pub const BELT_L_D: BeltMeta = BeltMeta {
  btype: BeltType::L_D,
  dbg: "BELT_L_D",
  src: "./img/l_d.png",
  direction_u: BeltDirection::None,
  direction_r: BeltDirection::None,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::In,
  cli_u: ' ',
  cli_r: ' ',
  cli_d: '║',
  cli_l: '═',
  cli_c: '╗',
};
// ┌v┐
// < │
// └─┘
pub const BELT_L_U: BeltMeta = BeltMeta {
  btype: BeltType::L_U,
  dbg: "BELT_L_U",
  src: "./img/l_u.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::None,
  direction_d: BeltDirection::None,
  direction_l: BeltDirection::In,
  cli_u: '║',
  cli_r: ' ',
  cli_d: ' ',
  cli_l: '═',
  cli_c: '╝',
};
// ┌v┐
// < │
// └─┘
pub const BELT_U_L: BeltMeta = BeltMeta {
  btype: BeltType::U_L,
  dbg: "BELT_U_L",
  src: "./img/u_l.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::None,
  direction_d: BeltDirection::None,
  direction_l: BeltDirection::Out,
  cli_u: '║',
  cli_r: ' ',
  cli_d: ' ',
  cli_l: '═',
  cli_c: '╝',
};
// ┌v┐
// │ │
// └v┘
pub const BELT_U_D: BeltMeta = BeltMeta {
  btype: BeltType::U_D,
  dbg: "BELT_U_D",
  src: "./img/u_d.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::None,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::None,
  cli_u: '║',
  cli_r: ' ',
  cli_d: '║',
  cli_l: ' ',
  cli_c: '║',
};
// ┌^┐
// │ │
// └^┘
pub const BELT_D_U: BeltMeta = BeltMeta {
  btype: BeltType::D_U,
  dbg: "BELT_D_U",
  src: "./img/d_u.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::None,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::None,
  cli_u: '║',
  cli_r: ' ',
  cli_d: '║',
  cli_l: ' ',
  cli_c: '║',
};
// ┌─┐
// > >
// └─┘
pub const BELT_L_R: BeltMeta = BeltMeta {
  btype: BeltType::L_R,
  dbg: "BELT_L_R",
  src: "./img/l_r.png",
  direction_u: BeltDirection::None,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::None,
  direction_l: BeltDirection::In,
  cli_u: ' ',
  cli_r: '═',
  cli_d: ' ',
  cli_l: '═',
  cli_c: '═',
};
// ┌─┐
// < <
// └─┘
pub const BELT_R_L: BeltMeta = BeltMeta {
  btype: BeltType::R_L,
  dbg: "BELT_R_L",
  src: "./img/r_l.png",
  direction_u: BeltDirection::None,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::None,
  direction_l: BeltDirection::Out,
  cli_u: ' ',
  cli_r: '═',
  cli_d: ' ',
  cli_l: '═',
  cli_c: '═',
};
// ┌v┐
// < >
// └─┘
pub const BELT_U_LR: BeltMeta = BeltMeta {
  btype: BeltType::U_LR,
  dbg: "BELT_U_LR",
  src: "./img/u_lr.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::None,
  direction_l: BeltDirection::Out,
  cli_u: '║',
  cli_r: '═',
  cli_d: ' ',
  cli_l: '═',
  cli_c: '╩',
};
// ┌v┐
// < <
// └─┘
pub const BELT_RU_L: BeltMeta = BeltMeta {
  btype: BeltType::RU_L,
  dbg: "BELT_RU_L",
  src: "./img/ru_l.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::None,
  direction_l: BeltDirection::Out,
  cli_u: '║',
  cli_r: '═',
  cli_d: ' ',
  cli_l: '═',
  cli_c: '╩',
};
// ┌v┐
// > >
// └─┘
pub const BELT_LU_R: BeltMeta = BeltMeta {
  btype: BeltType::LU_R,
  dbg: "BELT_LU_R",
  src: "./img/lu_r.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::None,
  direction_l: BeltDirection::In,
  cli_u: '║',
  cli_r: '═',
  cli_d: ' ',
  cli_l: '═',
  cli_c: '╩',
};
// ┌^┐
// > >
// └─┘
pub const BELT_L_RU: BeltMeta = BeltMeta {
  btype: BeltType::L_RU,
  dbg: "BELT_L_RU",
  src: "./img/l_ru.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::None,
  direction_l: BeltDirection::In,
  cli_u: '║',
  cli_r: '═',
  cli_d: ' ',
  cli_l: '═',
  cli_c: '╩',
};
// ┌^┐
// > <
// └─┘
pub const BELT_LR_U: BeltMeta = BeltMeta {
  btype: BeltType::LR_U,
  dbg: "BELT_LR_U",
  src: "./img/lr_u.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::None,
  direction_l: BeltDirection::In,
  cli_u: '║',
  cli_r: '═',
  cli_d: ' ',
  cli_l: '═',
  cli_c: '╩',
};
// ┌^┐
// < <
// └─┘
pub const BELT_R_LU: BeltMeta = BeltMeta {
  btype: BeltType::R_LU,
  dbg: "BELT_R_LU",
  src: "./img/r_lu.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::None,
  direction_l: BeltDirection::Out,
  cli_u: '║',
  cli_r: '═',
  cli_d: ' ',
  cli_l: '═',
  cli_c: '╩',
};
// ┌^┐
// │ <
// └v┘
pub const BELT_R_DU: BeltMeta = BeltMeta {
  btype: BeltType::R_DU,
  dbg: "BELT_R_DU",
  src: "./img/r_du.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::None,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌v┐
// │ <
// └v┘
pub const BELT_RU_D: BeltMeta = BeltMeta {
  btype: BeltType::RU_D,
  dbg: "BELT_RU_D",
  src: "./img/ru_d.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::None,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌^┐
// │ <
// └^┘
pub const BELT_DR_U: BeltMeta = BeltMeta {
  btype: BeltType::DR_U,
  dbg: "BELT_DR_U",
  src: "./img/dr_u.png",
  direction_u: BeltDirection::None,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::Out,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌v┐
// │ >
// └^┘
pub const BELT_DU_R: BeltMeta = BeltMeta {
  btype: BeltType::DU_R,
  dbg: "BELT_DU_R",
  src: "./img/du_r.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::None,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌v┐
// │ >
// └v┘
pub const BELT_U_DR: BeltMeta = BeltMeta {
  btype: BeltType::U_DR,
  dbg: "BELT_U_DR",
  src: "./img/u_dr.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::None,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌^┐
// │ >
// └^┘
pub const BELT_D_RU: BeltMeta = BeltMeta {
  btype: BeltType::D_RU,
  dbg: "BELT_D_RU",
  src: "./img/d_ru.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::None,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌─┐
// < >
// └^┘
pub const BELT_D_LR: BeltMeta = BeltMeta {
  btype: BeltType::D_LR,
  dbg: "BELT_D_LR",
  src: "./img/d_lr.png",
  direction_u: BeltDirection::None,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::Out,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌─┐
// > >
// └^┘
pub const BELT_DL_R: BeltMeta = BeltMeta {
  btype: BeltType::DL_R,
  dbg: "BELT_DL_R",
  src: "./img/dl_r.png",
  direction_u: BeltDirection::None,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::In,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌─┐
// < <
// └^┘
pub const BELT_DR_L: BeltMeta = BeltMeta {
  btype: BeltType::DR_L,
  dbg: "BELT_DR_L",
  src: "./img/dr_l.png",
  direction_u: BeltDirection::None,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::Out,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌─┐
// > <
// └v┘
pub const BELT_LR_D: BeltMeta = BeltMeta {
  btype: BeltType::LR_D,
  dbg: "BELT_LR_D",
  src: "./img/dr_l.png",
  direction_u: BeltDirection::None,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::In,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌─┐
// > >
// └v┘
pub const BELT_L_DR: BeltMeta = BeltMeta {
  btype: BeltType::L_DR,
  dbg: "BELT_L_DR",
  src: "./img/dr_l.png",
  direction_u: BeltDirection::None,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::In,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌─┐
// < <
// └v┘
pub const BELT_R_DL: BeltMeta = BeltMeta {
  btype: BeltType::R_DL,
  dbg: "BELT_R_DL",
  src: "./img/r_dl.png",
  direction_u: BeltDirection::None,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::Out,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌^┐
// > │
// └v┘
pub const BELT_L_DU: BeltMeta = BeltMeta {
  btype: BeltType::L_DU,
  dbg: "BELT_L_DU",
  src: "./img/l_du.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::None,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::In,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌v┐
// > │
// └v┘
pub const BELT_LU_D: BeltMeta = BeltMeta {
  btype: BeltType::LU_D,
  dbg: "BELT_LU_D",
  src: "./img/lu_d.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::None,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::In,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌^┐
// > │
// └^┘
pub const BELT_DL_U: BeltMeta = BeltMeta {
  btype: BeltType::DL_U,
  dbg: "BELT_DL_U",
  src: "./img/dl_u.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::None,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::In,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌v┐
// < │
// └^┘
pub const BELT_DU_L: BeltMeta = BeltMeta {
  btype: BeltType::DU_L,
  dbg: "BELT_DU_L",
  src: "./img/du_l.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::None,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::Out,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌v┐
// < │
// └v┘
pub const BELT_U_DL: BeltMeta = BeltMeta {
  btype: BeltType::U_DL,
  dbg: "BELT_U_DL",
  src: "./img/u_dl.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::None,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::Out,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌^┐
// < │
// └^┘
pub const BELT_D_UL: BeltMeta = BeltMeta {
  btype: BeltType::D_UL,
  dbg: "BELT_D_UL",
  src: "./img/d_ul.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::None,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::Out,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌v┐
// < >
// └v┘
pub const BELT_U_DLR: BeltMeta = BeltMeta {
  btype: BeltType::U_DLR,
  dbg: "BELT_U_DLR",
  src: "./img/todo.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::Out,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌^┐
// < <
// └v┘
pub const BELT_R_DLU: BeltMeta = BeltMeta {
  btype: BeltType::R_DLU,
  dbg: "BELT_R_DLU",
  src: "./img/todo.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::Out,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌^┐
// < >
// └^┘
pub const BELT_D_LRU: BeltMeta = BeltMeta {
  btype: BeltType::D_LRU,
  dbg: "BELT_D_LRU",
  src: "./img/todo.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::Out,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌^┐
// > >
// └v┘
pub const BELT_L_DRU: BeltMeta = BeltMeta {
  btype: BeltType::L_DRU,
  dbg: "BELT_L_DRU",
  src: "./img/todo.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::In,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌v┐
// < <
// └v┘
pub const BELT_RU_DL: BeltMeta = BeltMeta {
  btype: BeltType::RU_DL,
  dbg: "BELT_RU_DL",
  src: "./img/todo.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::Out,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌v┐
// < >
// └^┘
pub const BELT_DU_LR: BeltMeta = BeltMeta {
  btype: BeltType::DU_LR,
  dbg: "BELT_DU_LR",
  src: "./img/todo.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::Out,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌v┐
// > >
// └v┘
pub const BELT_LU_DR: BeltMeta = BeltMeta {
  btype: BeltType::LU_DR,
  dbg: "BELT_LU_DR",
  src: "./img/todo.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::In,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌v┐
// > >
// └v┘
pub const BELT_LD_RU: BeltMeta = BeltMeta {
  btype: BeltType::LD_RU,
  dbg: "BELT_LD_RU",
  src: "./img/todo.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::In,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌^┐
// < <
// └^┘
pub const BELT_DR_LU: BeltMeta = BeltMeta {
  btype: BeltType::DR_LU,
  dbg: "BELT_DR_LU",
  src: "./img/todo.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::Out,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌^┐
// > <
// └v┘
pub const BELT_LR_DU: BeltMeta = BeltMeta {
  btype: BeltType::LR_DU,
  dbg: "BELT_LR_DU",
  src: "./img/todo.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::In,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌^┐
// > <
// └^┘
pub const BELT_DLR_U: BeltMeta = BeltMeta {
  btype: BeltType::DLR_U,
  dbg: "BELT_DLR_U",
  src: "./img/todo.png",
  direction_u: BeltDirection::Out,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::In,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌v┐
// > >
// └^┘
pub const BELT_DLU_R: BeltMeta = BeltMeta {
  btype: BeltType::DLU_R,
  dbg: "BELT_DLU_R",
  src: "./img/todo.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::Out,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::In,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌v┐
// > <
// └v┘
pub const BELT_RLU_D: BeltMeta = BeltMeta {
  btype: BeltType::RLU_D,
  dbg: "BELT_RLU_D",
  src: "./img/todo.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::Out,
  direction_l: BeltDirection::In,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};
// ┌v┐
// < <
// └^┘
pub const BELT_DRU_L: BeltMeta = BeltMeta {
  btype: BeltType::DRU_L,
  dbg: "BELT_DRU_L",
  src: "./img/todo.png",
  direction_u: BeltDirection::In,
  direction_r: BeltDirection::In,
  direction_d: BeltDirection::In,
  direction_l: BeltDirection::Out,
  cli_u: '?',
  cli_r: '?',
  cli_d: '?',
  cli_l: '?',
  cli_c: '?',
};


