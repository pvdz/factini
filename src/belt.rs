// We don't need to use all of them :)

#[derive(Clone, Copy, Debug)]
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

#[derive(Debug)]
pub struct BeltMeta {
  pub dbg: &'static str,
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
  dbg: "BELT_NONE",
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
  dbg: "BELT_U_R",
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
  dbg: "BELT_R_U",
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
  dbg: "BELT_R_D",
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
  dbg: "BELT_D_R",
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
  dbg: "BELT_D_L",
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
  dbg: "BELT_L_D",
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
  dbg: "BELT_L_U",
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
  dbg: "BELT_U_L",
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
  dbg: "BELT_U_D",
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
  dbg: "BELT_D_U",
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
  dbg: "BELT_L_R",
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
  dbg: "BELT_R_L",
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
  dbg: "BELT_U_LR",
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
  dbg: "BELT_RU_L",
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
  dbg: "BELT_LU_R",
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
  dbg: "BELT_L_RU",
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
  dbg: "BELT_LR_U",
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
  dbg: "BELT_R_LU",
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
  dbg: "BELT_R_DU",
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
  dbg: "BELT_RU_D",
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
  dbg: "BELT_DR_U",
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
  dbg: "BELT_DU_R",
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
  dbg: "BELT_U_DR",
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
  dbg: "BELT_D_RU",
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
  dbg: "BELT_D_LR",
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
  dbg: "BELT_DL_R",
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
  dbg: "BELT_DR_L",
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
  dbg: "BELT_LR_D",
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
  dbg: "BELT_L_DR",
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
  dbg: "BELT_R_DL",
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
  dbg: "BELT_L_DU",
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
  dbg: "BELT_LU_D",
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
  dbg: "BELT_DL_U",
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
  dbg: "BELT_DU_L",
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
  dbg: "BELT_U_DL",
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
  dbg: "BELT_D_UL",
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
  dbg: "BELT_U_DLR",
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
  dbg: "BELT_R_DLU",
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
  dbg: "BELT_D_LRU",
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
  dbg: "BELT_L_DRU",
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
  dbg: "BELT_RU_DL",
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
  dbg: "BELT_DU_LR",
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
  dbg: "BELT_LU_DR",
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
  dbg: "BELT_LD_RU",
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
  dbg: "BELT_DR_LU",
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
  dbg: "BELT_LR_DU",
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
  dbg: "BELT_DLR_U",
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
  dbg: "BELT_DLU_R",
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
  dbg: "BELT_RLU_D",
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
  dbg: "BELT_DRU_L",
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


