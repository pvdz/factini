use super::belt::*;
use super::belt_codes::*;
use crate::belt_type::*;
use super::cell::*;
use super::config::*;
use super::demand::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::machine::*;
use super::options::*;
use super::part::*;
use super::port::*;
use super::port_auto::*;
use super::supply::*;
use super::state::*;
use super::utils::*;

// Clone but not Copy... I don't want to accidentally clone cells when I want to move them
#[derive(Debug, Clone)]
pub struct BeltMeta {
  pub btype: BeltType, // BELT_FROM_TO
  pub dbg: &'static str,
  pub src: &'static str, // tile image
  // TBD if I want to keep this
  pub port_u: Port,
  pub port_r: Port,
  pub port_d: Port,
  pub port_l: Port,
  // simplify cli output painting
  pub cli_icon: char,
}


// // This is immutable static information about each kind of belt. Universal facts.
// #[derive(Debug, Clone)]
// pub struct BeltMeta {
//   pub btype: BeltType, // BELT_FROM_TO
//   pub sprite_id: usize, // 1:1 with SPRITE_CODES
//   pub dbg: &'static str,
//   // Ports usually determines belt type
//   pub port_u: Port,
//   pub port_r: Port,
//   pub port_d: Port,
//   pub port_l: Port,
//   // Used for import/export of maps.
//   pub cli_icon: char,
// }

const fn prebelt(u: Port, r: Port, d: Port, l: Port, cli_icon: char, src: &'static str) -> BeltMeta {
  let bt = belt_type_from_ports(u, r, d, l);
  return prebelt_meta(u, r, d, l, cli_icon, bt, src);
}
const fn prebelt_meta(u: Port, r: Port, d: Port, l: Port, cli_icon: char, belt_type: BeltType, src: &'static str) -> BeltMeta {
  return BeltMeta {
    btype: belt_type,
    // sprite_id: belt_type_from_ports(u, r, d, l) as usize,
    src,
    dbg: belt_code_from_ports(u, r, d, l),
    port_u: u,
    port_r: r,
    port_d: d,
    port_l: l,
    cli_icon,
  };
}

// ┌─┐
// │ │
// └─┘
pub const BELT_NONE: BeltMeta = prebelt_meta(Port::None, Port::None, Port::None, Port::None, ' ', BeltType::NONE,"./img/belt/belt_none.png");
pub const BELT_UNKNOWN: BeltMeta = prebelt_meta(Port::Unknown, Port::Unknown, Port::Unknown, Port::Unknown, '?', BeltType::UNKNOWN, "./img/belt/belt_unknown.png");
pub const BELT_INVALID: BeltMeta = prebelt_meta(Port::Unknown, Port::Unknown, Port::Unknown, Port::Unknown, '!', BeltType::INVALID, "./img/belt/belt_invalid.png");
pub const BELT_L_: BeltMeta = prebelt(Port::None, Port::None, Port::None, Port::Inbound, '╴', "./img/belt/l__cb.png");
pub const BELT__L: BeltMeta = prebelt(Port::None, Port::None, Port::None, Port::Outbound, '╴', "./img/belt/_l_cb.png");
pub const BELT___L: BeltMeta = prebelt(Port::None, Port::None, Port::None, Port::Unknown, '╴', "./img/belt/__l_cb.png");
pub const BELT_D_: BeltMeta = prebelt(Port::None, Port::None, Port::Inbound, Port::None, '╷', "./img/belt/d__cb.png");
pub const BELT_DL_: BeltMeta = prebelt(Port::None, Port::None, Port::Inbound, Port::Inbound, '╗', "./img/belt/dl__cb.png");
pub const BELT_D_L: BeltMeta = prebelt(Port::None, Port::None, Port::Inbound, Port::Outbound, '╗', "./img/belt/d_l_cb.png");
pub const BELT_D__L: BeltMeta = prebelt(Port::None, Port::None, Port::Inbound, Port::Unknown, '╗', "./img/belt/d__l_cb.png");
pub const BELT__D: BeltMeta = prebelt(Port::None, Port::None, Port::Outbound, Port::None, '╷', "./img/belt/_d_cb.png");
pub const BELT_L_D: BeltMeta = prebelt(Port::None, Port::None, Port::Outbound, Port::Inbound, '╗', "./img/belt/l_d_cb.png");
pub const BELT__DL: BeltMeta = prebelt(Port::None, Port::None, Port::Outbound, Port::Outbound, '╗', "./img/belt/_dl_cb.png");
pub const BELT__D_L: BeltMeta = prebelt(Port::None, Port::None, Port::Outbound, Port::Unknown, '╗', "./img/belt/_d_l_cb.png");
pub const BELT___D: BeltMeta = prebelt(Port::None, Port::None, Port::Unknown, Port::None, '╷', "./img/belt/__d_cb.png");
pub const BELT_L__D: BeltMeta = prebelt(Port::None, Port::None, Port::Unknown, Port::Inbound, '╗', "./img/belt/l__d_cb.png");
pub const BELT__L_D: BeltMeta = prebelt(Port::None, Port::None, Port::Unknown, Port::Outbound, '╗', "./img/belt/_l_d_cb.png");
pub const BELT___DL: BeltMeta = prebelt(Port::None, Port::None, Port::Unknown, Port::Unknown, '╗', "./img/belt/__dl_cb.png");
pub const BELT_R_: BeltMeta = prebelt(Port::None, Port::Inbound, Port::None, Port::None, '╶', "./img/belt/r__cb.png");
pub const BELT_LR_: BeltMeta = prebelt(Port::None, Port::Inbound, Port::None, Port::Inbound, '═', "./img/belt/lr__cb.png");
pub const BELT_R_L: BeltMeta = prebelt(Port::None, Port::Inbound, Port::None, Port::Outbound, '═', "./img/belt/r_l_cb.png");
pub const BELT_R__L: BeltMeta = prebelt(Port::None, Port::Inbound, Port::None, Port::Unknown, '═', "./img/belt/r__l_cb.png");
pub const BELT_DR_: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Inbound, Port::None, '╔', "./img/belt/dr__cb.png");
pub const BELT_DLR_: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Inbound, Port::Inbound, '╩', "./img/belt/dlr__cb.png");
pub const BELT_DR_L: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Inbound, Port::Outbound, '╩', "./img/belt/dr_l_cb.png");
pub const BELT_DR__L: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Inbound, Port::Unknown, '╩', "./img/belt/dr__l_cb.png");
pub const BELT_R_D: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Outbound, Port::None, '╔', "./img/belt/r_d_cb.png");
pub const BELT_LR_D: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Outbound, Port::Inbound, '╩', "./img/belt/lr_d_cb.png");
pub const BELT_R_DL: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Outbound, Port::Outbound, '╩', "./img/belt/r_dl_cb.png");
pub const BELT_R_D_L: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Outbound, Port::Unknown, '╩', "./img/belt/r_d_l_cb.png");
pub const BELT_R__D: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Unknown, Port::None, '╔', "./img/belt/r__d_cb.png");
pub const BELT_LR__D: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Unknown, Port::Inbound, '╩', "./img/belt/lr__d_cb.png");
pub const BELT_R_L_D: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Unknown, Port::Outbound, '╩', "./img/belt/r_l_d_cb.png");
pub const BELT_R__DL: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Unknown, Port::None, '╩', "./img/belt/r__dl_cb.png");
pub const BELT__R: BeltMeta = prebelt(Port::None, Port::Outbound, Port::None, Port::None, '╶', "./img/belt/_r_cb.png");
pub const BELT_L_R: BeltMeta = prebelt(Port::None, Port::Outbound, Port::None, Port::Inbound, '═', "./img/belt/l_r_cb.png");
pub const BELT__LR: BeltMeta = prebelt(Port::None, Port::Outbound, Port::None, Port::Outbound, '═', "./img/belt/_lr_cb.png");
pub const BELT__R_L: BeltMeta = prebelt(Port::None, Port::Outbound, Port::None, Port::Unknown, '═', "./img/belt/_r_l_cb.png");
pub const BELT_D_R: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Inbound, Port::None, '╔', "./img/belt/d_r_cb.png");
pub const BELT_DL_R: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Inbound, Port::Inbound, '╩', "./img/belt/dl_r_cb.png");
pub const BELT_D_LR: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Inbound, Port::Outbound, '╩', "./img/belt/d_lr_cb.png");
pub const BELT_D_R_L: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Inbound, Port::Unknown, '╩', "./img/belt/d_r_l_cb.png");
pub const BELT__DR: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Outbound, Port::None, '╔', "./img/belt/_dr_cb.png");
pub const BELT_L_DR: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Outbound, Port::Inbound, '╩', "./img/belt/l_dr_cb.png");
pub const BELT__DLR: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Outbound, Port::Outbound, '╩', "./img/belt/_dlr_cb.png");
pub const BELT__DR_L: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Outbound, Port::Unknown, '╩', "./img/belt/_dr_l_cb.png");
pub const BELT__R_D: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Unknown, Port::None, '╔', "./img/belt/_r_d_cb.png");
pub const BELT_L_R_D: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Unknown, Port::Inbound, '╩', "./img/belt/l_r_d_cb.png");
pub const BELT__LR_D: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Unknown, Port::Outbound, '╩', "./img/belt/_lr_d_cb.png");
pub const BELT__R_DL: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Unknown, Port::Unknown, '╩', "./img/belt/_r_dl_cb.png");
pub const BELT___R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::None, Port::None, '╶', "./img/belt/__r_cb.png");
pub const BELT_L__R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::None, Port::Inbound, '═', "./img/belt/l__r_cb.png");
pub const BELT__L_R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::None, Port::Outbound, '═', "./img/belt/_l_r_cb.png");
pub const BELT___LR: BeltMeta = prebelt(Port::None, Port::Unknown, Port::None, Port::Unknown, '═', "./img/belt/__lr_cb.png");
pub const BELT_D__R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Inbound, Port::None, '╔', "./img/belt/d__r_cb.png");
pub const BELT_DL__R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Inbound, Port::Inbound, '╩', "./img/belt/dl__r_cb.png");
pub const BELT_D_L_R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Inbound, Port::Outbound, '╩', "./img/belt/d_l_r_cb.png");
pub const BELT_D__LR: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Inbound, Port::Unknown, '╩', "./img/belt/d__lr_cb.png");
pub const BELT__D_R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Outbound, Port::None, '╔', "./img/belt/_d_r_cb.png");
pub const BELT_L_D_R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Outbound, Port::Inbound, '╩', "./img/belt/l_d_r_cb.png");
pub const BELT__DL_R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Outbound, Port::Outbound, '╩', "./img/belt/_dl_r_cb.png");
pub const BELT__D_LR: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Outbound, Port::Unknown, '╩', "./img/belt/_d_lr_cb.png");
pub const BELT___DR: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Unknown, Port::None, '╔', "./img/belt/__dr_cb.png");
pub const BELT_L__DR: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Unknown, Port::Inbound, '╩', "./img/belt/l__dr_cb.png");
pub const BELT__L_DR: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Unknown, Port::Outbound, '╩', "./img/belt/_l_dr_cb.png");
pub const BELT___DLR: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Unknown, Port::Unknown, '╩', "./img/belt/__dlr_cb.png");
pub const BELT_U_: BeltMeta = prebelt(Port::Inbound, Port::None, Port::None, Port::None, '╵', "./img/belt/u__cb.png");
pub const BELT_LU_: BeltMeta = prebelt(Port::Inbound, Port::None, Port::None, Port::Inbound, '╝', "./img/belt/lu__cb.png");
pub const BELT_U_L: BeltMeta = prebelt(Port::Inbound, Port::None, Port::None, Port::Outbound, '╝', "./img/belt/u_l_cb.png");
pub const BELT_U__L: BeltMeta = prebelt(Port::Inbound, Port::None, Port::None, Port::Unknown, '╝', "./img/belt/u__l_cb.png");
pub const BELT_DU_: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Inbound, Port::None, '║', "./img/belt/du__cb.png");
pub const BELT_DLU_: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Inbound, Port::Inbound, '╣', "./img/belt/dlu__cb.png");
pub const BELT_DU_L: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Inbound, Port::Outbound, '╣', "./img/belt/du_l_cb.png");
pub const BELT_DU__L: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Inbound, Port::Unknown, '╣', "./img/belt/du__l_cb.png");
pub const BELT_U_D: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Outbound, Port::None, '║', "./img/belt/u_d_cb.png");
pub const BELT_LU_D: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Outbound, Port::Inbound, '╣', "./img/belt/lu_d_cb.png");
pub const BELT_U_DL: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Outbound, Port::Outbound, '╣', "./img/belt/u_dl_cb.png");
pub const BELT_U_D_L: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Outbound, Port::Unknown, '╣', "./img/belt/u_d_l_cb.png");
pub const BELT_U__D: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Unknown, Port::None, '║', "./img/belt/u__d_cb.png");
pub const BELT_LU__D: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Unknown, Port::Inbound, '╣', "./img/belt/lu__d_cb.png");
pub const BELT_U_L_D: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Unknown, Port::Outbound, '╣', "./img/belt/u_l_d_cb.png");
pub const BELT_U__DL: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Unknown, Port::Unknown, '╣', "./img/belt/u__dl_cb.png");
pub const BELT_RU_: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::None, Port::None, '╚', "./img/belt/ru__cb.png");
pub const BELT_LRU_: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::None, Port::Inbound, '╩', "./img/belt/lru__cb.png");
pub const BELT_RU_L: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::None, Port::Outbound, '╩', "./img/belt/ru_l_cb.png");
pub const BELT_RU__L: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::None, Port::Unknown, '╩', "./img/belt/ru__l_cb.png");
pub const BELT_DRU_: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Inbound, Port::None, '╠', "./img/belt/dru__cb.png");
pub const BELT_DLRU_: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Inbound, Port::Inbound, '╬', "./img/belt/dlru__cb.png");
pub const BELT_DRU_L: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Inbound, Port::Outbound, '╬', "./img/belt/dru_l_cb.png");
pub const BELT_DRU__L: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Inbound, Port::Unknown, '╬', "./img/belt/dru__l_cb.png");
pub const BELT_RU_D: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Outbound, Port::None, '╠', "./img/belt/ru_d_cb.png");
pub const BELT_LRU_D: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Outbound, Port::Inbound, '╬', "./img/belt/lru_d_cb.png");
pub const BELT_RU_DL: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Outbound, Port::Outbound, '╬', "./img/belt/ru_dl_cb.png");
pub const BELT_RU_D_L: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Outbound, Port::Unknown, '╬', "./img/belt/ru_d_l_cb.png");
pub const BELT_RU__D: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Unknown, Port::None, '╠', "./img/belt/ru__d_cb.png");
pub const BELT_LRU__D: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Unknown, Port::Inbound, '╬', "./img/belt/lru__d_cb.png");
pub const BELT_RU_L_D: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Unknown, Port::Outbound, '╬', "./img/belt/ru_l_d_cb.png");
pub const BELT_RU__DL: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Unknown, Port::Unknown, '╬', "./img/belt/ru__dl_cb.png");
pub const BELT_U_R: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::None, Port::None, '╚', "./img/belt/u_r_cb.png");
pub const BELT_LU_R: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::None, Port::Inbound, '╩', "./img/belt/lu_r_cb.png");
pub const BELT_U_LR: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::None, Port::Outbound, '╩', "./img/belt/u_lr_cb.png");
pub const BELT_U_R_L: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::None, Port::Unknown, '╩', "./img/belt/u_r_l_cb.png");
pub const BELT_DU_R: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Inbound, Port::None, '╠', "./img/belt/du_r_cb.png");
pub const BELT_DLU_R: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Inbound, Port::Inbound, '╬', "./img/belt/dlu_r_cb.png");
pub const BELT_DU_LR: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Inbound, Port::Outbound, '╬', "./img/belt/du_lr_cb.png");
pub const BELT_DU_R_L: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Inbound, Port::Unknown, '╬', "./img/belt/du_r_l_cb.png");
pub const BELT_U_DR: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Outbound, Port::None, '╠', "./img/belt/u_dr_cb.png");
pub const BELT_LU_DR: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Outbound, Port::Inbound, '╬', "./img/belt/lu_dr_cb.png");
pub const BELT_U_DLR: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Outbound, Port::Outbound, '╬', "./img/belt/u_dlr_cb.png");
pub const BELT_U_DR_L: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Outbound, Port::Unknown, '╬', "./img/belt/u_dr_l_cb.png");
pub const BELT_U_R_D: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Unknown, Port::None, '╠', "./img/belt/u_r_d_cb.png");
pub const BELT_LU_R_D: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Unknown, Port::Inbound, '╬', "./img/belt/lu_r_d_cb.png");
pub const BELT_U_LR_D: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Unknown, Port::Outbound, '╬', "./img/belt/u_lr_d_cb.png");
pub const BELT_U_R_DL: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Unknown, Port::Unknown, '╬', "./img/belt/u_r_dl_cb.png");
pub const BELT_U__R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::None, Port::None, '╚', "./img/belt/u__r_cb.png");
pub const BELT_LU__R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::None, Port::Inbound, '╩', "./img/belt/lu__r_cb.png");
pub const BELT_U_L_R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::None, Port::Outbound, '╩', "./img/belt/u_l_r_cb.png");
pub const BELT_U__LR: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::None, Port::Unknown, '╩', "./img/belt/u__lr_cb.png");
pub const BELT_DU__R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Inbound, Port::None, '╠', "./img/belt/du__r_cb.png");
pub const BELT_DLU__R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Inbound, Port::Inbound, '╬', "./img/belt/dlu__r_cb.png");
pub const BELT_DU_L_R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Inbound, Port::Outbound, '╬', "./img/belt/du_l_r_cb.png");
pub const BELT_DU__LR: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Inbound, Port::Unknown, '╬', "./img/belt/du__lr_cb.png");
pub const BELT_U_D_R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Outbound, Port::None, '╠', "./img/belt/u_d_r_cb.png");
pub const BELT_LU_D_R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Outbound, Port::Inbound, '╬', "./img/belt/lu_d_r_cb.png");
pub const BELT_U_DL_R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Outbound, Port::Outbound, '╬', "./img/belt/u_dl_r_cb.png");
pub const BELT_U_D_LR: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Outbound, Port::Unknown, '╬', "./img/belt/u_d_lr_cb.png");
pub const BELT_U__DR: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Unknown, Port::None, '╠', "./img/belt/u__dr_cb.png");
pub const BELT_LU__DR: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Unknown, Port::Inbound, '╬', "./img/belt/lu__dr_cb.png");
pub const BELT_U_L_DR: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Unknown, Port::Outbound, '╬', "./img/belt/u_l_dr_cb.png");
pub const BELT_U__DLR: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Unknown, Port::Unknown, '╬', "./img/belt/u__dlr_cb.png");
pub const BELT__U: BeltMeta = prebelt(Port::Outbound, Port::None, Port::None, Port::None, '╵', "./img/belt/_u_cb.png");
pub const BELT_L_U: BeltMeta = prebelt(Port::Outbound, Port::None, Port::None, Port::Inbound, '╝', "./img/belt/l_u_cb.png");
pub const BELT__LU: BeltMeta = prebelt(Port::Outbound, Port::None, Port::None, Port::Outbound, '╝', "./img/belt/_lu_cb.png");
pub const BELT__U_L: BeltMeta = prebelt(Port::Outbound, Port::None, Port::None, Port::Unknown, '╝', "./img/belt/_u_l_cb.png");
pub const BELT_D_U: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Inbound, Port::None, '║', "./img/belt/d_u_cb.png");
pub const BELT_DL_U: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Inbound, Port::Inbound, '╣', "./img/belt/dl_u_cb.png");
pub const BELT_D_LU: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Inbound, Port::Outbound, '╣', "./img/belt/d_lu_cb.png");
pub const BELT_D_U_L: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Inbound, Port::Unknown, '╣', "./img/belt/d_u_l_cb.png");
pub const BELT__DU: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Outbound, Port::None, '║', "./img/belt/_du_cb.png");
pub const BELT_L_DU: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Outbound, Port::Inbound, '╣', "./img/belt/l_du_cb.png");
pub const BELT__DLU: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Outbound, Port::Outbound, '╣', "./img/belt/_dlu_cb.png");
pub const BELT__DU_L: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Outbound, Port::Unknown, '╣', "./img/belt/_du_l_cb.png");
pub const BELT__U_D: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Unknown, Port::None, '║', "./img/belt/_u_d_cb.png");
pub const BELT_L_U_D: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Unknown, Port::Inbound, '╣', "./img/belt/l_u_d_cb.png");
pub const BELT__LU_D: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Unknown, Port::Outbound, '╣', "./img/belt/_lu_d_cb.png");
pub const BELT__U_DL: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Unknown, Port::Unknown, '╣', "./img/belt/_u_dl_cb.png");
pub const BELT_R_U: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::None, Port::None, '╚', "./img/belt/r_u_cb.png");
pub const BELT_LR_U: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::None, Port::Inbound, '╩', "./img/belt/lr_u_cb.png");
pub const BELT_R_LU: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::None, Port::Outbound, '╩', "./img/belt/r_lu_cb.png");
pub const BELT_R_U_L: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::None, Port::Unknown, '╩', "./img/belt/r_u_l_cb.png");
pub const BELT_DR_U: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Inbound, Port::None, '╠', "./img/belt/dr_u_cb.png");
pub const BELT_DLR_U: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Inbound, Port::Inbound, '╬', "./img/belt/dlr_u_cb.png");
pub const BELT_DR_LU: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Inbound, Port::Outbound, '╬', "./img/belt/dr_lu_cb.png");
pub const BELT_DR_U_L: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Inbound, Port::Unknown, '╬', "./img/belt/dr_u_l_cb.png");
pub const BELT_R_DU: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Outbound, Port::None, '╠', "./img/belt/r_du_cb.png");
pub const BELT_LR_DU: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Outbound, Port::Inbound, '╬', "./img/belt/lr_du_cb.png");
pub const BELT_R_DLU: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Outbound, Port::Outbound, '╬', "./img/belt/r_dlu_cb.png");
pub const BELT_R_DU_L: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Outbound, Port::Unknown, '╬', "./img/belt/r_du_l_cb.png");
pub const BELT_R_U_D: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Unknown, Port::None, '╠', "./img/belt/r_u_d_cb.png");
pub const BELT_LR_U_D: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Unknown, Port::Inbound, '╬', "./img/belt/lr_u_d_cb.png");
pub const BELT_R_LU_D: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Unknown, Port::Outbound, '╬', "./img/belt/r_lu_d_cb.png");
pub const BELT_R_U_DL: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Unknown, Port::Unknown, '╬', "./img/belt/r_u_dl_cb.png");
pub const BELT__RU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::None, Port::None, '╚', "./img/belt/_ru_cb.png");
pub const BELT_L_RU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::None, Port::Inbound, '╩', "./img/belt/l_ru_cb.png");
pub const BELT__LRU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::None, Port::Outbound, '╩', "./img/belt/_lru_cb.png");
pub const BELT__RU_L: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::None, Port::Unknown, '╩', "./img/belt/_ru_l_cb.png");
pub const BELT_D_RU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Inbound, Port::None, '╠', "./img/belt/d_ru_cb.png");
pub const BELT_DL_RU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Inbound, Port::Inbound, '╬', "./img/belt/dl_ru_cb.png");
pub const BELT_D_LRU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Inbound, Port::Outbound, '╬', "./img/belt/d_lru_cb.png");
pub const BELT_D_RU_L: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Inbound, Port::Unknown, '╬', "./img/belt/d_ru_l_cb.png");
pub const BELT__DRU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Outbound, Port::Inbound, '╠', "./img/belt/_dru_cb.png");
pub const BELT_L_DRU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Outbound, Port::Inbound, '╬', "./img/belt/l_dru_cb.png");
pub const BELT__DLRU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Outbound, Port::Outbound, '╬', "./img/belt/_dlru_cb.png");
pub const BELT__DRU_L: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Outbound, Port::Unknown, '╬', "./img/belt/_dru_l_cb.png");
pub const BELT__RU_D: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Unknown, Port::None, '╠', "./img/belt/_ru_d_cb.png");
pub const BELT_L_RU_D: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Unknown, Port::Inbound, '╬', "./img/belt/l_ru_d_cb.png");
pub const BELT__LRU_D: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Unknown, Port::Outbound, '╬', "./img/belt/_lru_d_cb.png");
pub const BELT__RU_DL: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Unknown, Port::Unknown, '╬', "./img/belt/_ru_dl_cb.png");
pub const BELT__U_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::None, Port::None, '╚', "./img/belt/_u_r_cb.png");
pub const BELT_L_U_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::None, Port::Inbound, '╩', "./img/belt/l_u_r_cb.png");
pub const BELT__LU_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::None, Port::Outbound, '╩', "./img/belt/_lu_r_cb.png");
pub const BELT__U_LR: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::None, Port::Unknown, '╩', "./img/belt/_u_lr_cb.png");
pub const BELT_D_U_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Inbound, Port::None, '╠', "./img/belt/d_u_r_cb.png");
pub const BELT_DL_U_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Inbound, Port::Inbound, '╬', "./img/belt/dl_u_r_cb.png");
pub const BELT_D_LU_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Inbound, Port::Outbound, '╬', "./img/belt/d_lu_r_cb.png");
pub const BELT_D_U_LR: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Inbound, Port::Unknown, '╬', "./img/belt/d_u_lr_cb.png");
pub const BELT__DU_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Outbound, Port::Inbound, '╠', "./img/belt/_du_r_cb.png");
pub const BELT_L_DU_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Outbound, Port::Inbound, '╬', "./img/belt/l_du_r_cb.png");
pub const BELT__DLU_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Outbound, Port::Outbound, '╬', "./img/belt/_dlu_r_cb.png");
pub const BELT__DU_LR: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Outbound, Port::Unknown, '╬', "./img/belt/_du_lr_cb.png");
pub const BELT__U_DR: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Unknown, Port::None, '╠', "./img/belt/_u_dr_cb.png");
pub const BELT_L_U_DR: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Unknown, Port::Inbound, '╬', "./img/belt/l_u_dr_cb.png");
pub const BELT__LU_DR: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Unknown, Port::Outbound, '╬', "./img/belt/_lu_dr_cb.png");
pub const BELT__U_DLR: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Unknown, Port::Unknown, '╬', "./img/belt/_u_dlr_cb.png");
pub const BELT___U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::None, Port::None, '╵', "./img/belt/__u_cb.png");
pub const BELT_L__U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::None, Port::Inbound, '╝', "./img/belt/l__u_cb.png");
pub const BELT__L_U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::None, Port::Outbound, '╝', "./img/belt/_l_u_cb.png");
pub const BELT___LU: BeltMeta = prebelt(Port::Unknown, Port::None, Port::None, Port::Unknown, '╝', "./img/belt/__lu_cb.png");
pub const BELT_D__U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Inbound, Port::None, '║', "./img/belt/d__u_cb.png");
pub const BELT_DL__U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Inbound, Port::Inbound, '╣', "./img/belt/dl__u_cb.png");
pub const BELT_D_L_U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Inbound, Port::Outbound, '╣', "./img/belt/d_l_u_cb.png");
pub const BELT_D__LU: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Inbound, Port::Unknown, '╣', "./img/belt/d__lu_cb.png");
pub const BELT__D_U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Outbound, Port::None, '║', "./img/belt/_d_u_cb.png");
pub const BELT_L_D_U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Outbound, Port::Inbound, '╣', "./img/belt/l_d_u_cb.png");
pub const BELT__DL_U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Outbound, Port::Outbound, '╣', "./img/belt/_dl_u_cb.png");
pub const BELT__D_LU: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Outbound, Port::Unknown, '╣', "./img/belt/_d_lu_cb.png");
pub const BELT___DU: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Unknown, Port::None, '║', "./img/belt/__du_cb.png");
pub const BELT_L__DU: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Unknown, Port::Inbound, '╣', "./img/belt/l__du_cb.png");
pub const BELT__L_DU: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Unknown, Port::Outbound, '╣', "./img/belt/_l_du_cb.png");
pub const BELT___DLU: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Unknown, Port::Unknown, '╣', "./img/belt/__dlu_cb.png");
pub const BELT_R__U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::None, Port::None, '╚', "./img/belt/r__u_cb.png");
pub const BELT_LR__U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::None, Port::Inbound, '╩', "./img/belt/lr__u_cb.png");
pub const BELT_R_L_U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::None, Port::Outbound, '╩', "./img/belt/r_l_u_cb.png");
pub const BELT_R__LU: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::None, Port::Unknown, '╩', "./img/belt/r__lu_cb.png");
pub const BELT_DR__U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Inbound, Port::None, '╠', "./img/belt/dr__u_cb.png");
pub const BELT_DLR__U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Inbound, Port::Inbound, '╬', "./img/belt/dlr__u_cb.png");
pub const BELT_DR_L_U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Inbound, Port::Outbound, '╬', "./img/belt/dr_l_u_cb.png");
pub const BELT_DR__LU: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Inbound, Port::Unknown, '╬', "./img/belt/dr__lu_cb.png");
pub const BELT_R_D_U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Outbound, Port::None, '╠', "./img/belt/r_d_u_cb.png");
pub const BELT_LR_D_U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Outbound, Port::Inbound, '╬', "./img/belt/lr_d_u_cb.png");
pub const BELT_R_DL_U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Outbound, Port::Outbound, '╬', "./img/belt/r_dl_u_cb.png");
pub const BELT_R_D_LU: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Outbound, Port::Unknown, '╬', "./img/belt/r_d_lu_cb.png");
pub const BELT_R__DU: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Unknown, Port::None, '╠', "./img/belt/r__du_cb.png");
pub const BELT_LR__DU: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Unknown, Port::Inbound, '╬', "./img/belt/lr__du_cb.png");
pub const BELT_R_L_DU: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Unknown, Port::Outbound, '╬', "./img/belt/r_l_du_cb.png");
pub const BELT_R__DLU: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Unknown, Port::Unknown, '╬', "./img/belt/r__dlu_cb.png");
pub const BELT__R_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::None, Port::None, '╚', "./img/belt/_r_u_cb.png");
pub const BELT_L_R_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::None, Port::Inbound, '╩', "./img/belt/l_r_u_cb.png");
pub const BELT__LR_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::None, Port::Outbound, '╩', "./img/belt/_lr_u_cb.png");
pub const BELT__R_LU: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::None, Port::Unknown, '╩', "./img/belt/_r_lu_cb.png");
pub const BELT_D_R_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Inbound, Port::None, '╠', "./img/belt/d_r_u_cb.png");
pub const BELT_DL_R_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Inbound, Port::Inbound, '╬', "./img/belt/dl_r_u_cb.png");
pub const BELT_D_LR_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Inbound, Port::Outbound, '╬', "./img/belt/d_lr_u_cb.png");
pub const BELT_D_R_LU: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Inbound, Port::Unknown, '╬', "./img/belt/d_r_lu_cb.png");
pub const BELT__DR_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Outbound, Port::None, '╠', "./img/belt/_dr_u_cb.png");
pub const BELT_L_DR_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Outbound, Port::Inbound, '╬', "./img/belt/l_dr_u_cb.png");
pub const BELT__DLR_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Outbound, Port::Outbound, '╬', "./img/belt/_dlr_u_cb.png");
pub const BELT__DR_LU: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Outbound, Port::Unknown, '╬', "./img/belt/_dr_lu_cb.png");
pub const BELT__R_DU: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Unknown, Port::None, '╠', "./img/belt/_r_du_cb.png");
pub const BELT_L_R_DU: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Unknown, Port::Inbound, '╬', "./img/belt/l_r_du_cb.png");
pub const BELT__LR_DU: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Unknown, Port::Outbound, '╬', "./img/belt/_lr_du_cb.png");
pub const BELT__R_DLU: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Unknown, Port::Unknown, '╬', "./img/belt/_r_dlu_cb.png");
pub const BELT___RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::None, Port::None, '╚', "./img/belt/__ru_cb.png");
pub const BELT_L__RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::None, Port::Inbound, '╩', "./img/belt/l__ru_cb.png");
pub const BELT__L_RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Outbound, Port::None, '╩', "./img/belt/_l_ru_cb.png");
pub const BELT___LRU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::None, Port::Unknown, '╩', "./img/belt/__lru_cb.png");
pub const BELT_D__RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Inbound, Port::None, '╠', "./img/belt/d__ru_cb.png");
pub const BELT_DL__RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Inbound, Port::Inbound, '╬', "./img/belt/dl__ru_cb.png");
pub const BELT_D_L_RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Inbound, Port::Outbound, '╬', "./img/belt/d_l_ru_cb.png");
pub const BELT_D__LRU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Inbound, Port::Unknown, '╬', "./img/belt/d__lru_cb.png");
pub const BELT__D_RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Outbound, Port::None, '╠', "./img/belt/_d_ru_cb.png");
pub const BELT_L_D_RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Outbound, Port::Inbound, '╬', "./img/belt/l_d_ru_cb.png");
pub const BELT__DL_RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Outbound, Port::Outbound, '╬', "./img/belt/_dl_ru_cb.png");
pub const BELT__D_LRU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Outbound, Port::Unknown, '╬', "./img/belt/_d_lru_cb.png");
pub const BELT___DRU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Unknown, Port::None, '╠', "./img/belt/__dru_cb.png");
pub const BELT_L__DRU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Unknown, Port::Inbound, '╬', "./img/belt/l__dru_cb.png");
pub const BELT__L_DRU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Unknown, Port::Outbound, '╬', "./img/belt/_l_dru_cb.png");
pub const BELT___DLRU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Unknown, Port::Unknown, '╬', "./img/belt/__dlru_cb.png");

