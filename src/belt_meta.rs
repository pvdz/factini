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
  pub port_u: Port,
  pub port_r: Port,
  pub port_d: Port,
  pub port_l: Port,
  // simplify cli output painting
  pub cli_icon: char,
}

const fn prebelt(u: Port, r: Port, d: Port, l: Port, cli_icon: char) -> BeltMeta {
  let bt = belt_type_from_ports(u, r, d, l);
  return prebelt_meta(u, r, d, l, cli_icon, bt);
}
const fn prebelt_meta(u: Port, r: Port, d: Port, l: Port, cli_icon: char, belt_type: BeltType) -> BeltMeta {
  let code = belt_code_from_ports(u, r, d, l);
  return BeltMeta {
    btype: belt_type,
    src: "./img/belt/belt_unknown.png",
    dbg: code,
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
pub const BELT_NONE: BeltMeta = prebelt_meta(Port::None, Port::None, Port::None, Port::None, ' ', BeltType::NONE);
pub const BELT_UNKNOWN: BeltMeta = prebelt_meta(Port::Unknown, Port::Unknown, Port::Unknown, Port::Unknown, '?', BeltType::UNKNOWN);
pub const BELT_INVALID: BeltMeta = prebelt_meta(Port::Unknown, Port::Unknown, Port::Unknown, Port::Unknown, '!', BeltType::INVALID);
pub const BELT_L_: BeltMeta = prebelt(Port::None, Port::None, Port::None, Port::Inbound, '╴');
pub const BELT__L: BeltMeta = prebelt(Port::None, Port::None, Port::None, Port::Outbound, '╴');
pub const BELT___L: BeltMeta = prebelt(Port::None, Port::None, Port::None, Port::Unknown, '╴');
pub const BELT_D_: BeltMeta = prebelt(Port::None, Port::None, Port::Inbound, Port::None, '╷');
pub const BELT_DL_: BeltMeta = prebelt(Port::None, Port::None, Port::Inbound, Port::Inbound, '╗');
pub const BELT_D_L: BeltMeta = prebelt(Port::None, Port::None, Port::Inbound, Port::Outbound, '╗');
pub const BELT_D__L: BeltMeta = prebelt(Port::None, Port::None, Port::Inbound, Port::Unknown, '╗');
pub const BELT__D: BeltMeta = prebelt(Port::None, Port::None, Port::Outbound, Port::None, '╷');
pub const BELT_L_D: BeltMeta = prebelt(Port::None, Port::None, Port::Outbound, Port::Inbound, '╗');
pub const BELT__DL: BeltMeta = prebelt(Port::None, Port::None, Port::Outbound, Port::Outbound, '╗');
pub const BELT__D_L: BeltMeta = prebelt(Port::None, Port::None, Port::Outbound, Port::Unknown, '╗');
pub const BELT___D: BeltMeta = prebelt(Port::None, Port::None, Port::Unknown, Port::None, '╷');
pub const BELT_L__D: BeltMeta = prebelt(Port::None, Port::None, Port::Unknown, Port::Inbound, '╗');
pub const BELT__L_D: BeltMeta = prebelt(Port::None, Port::None, Port::Unknown, Port::Outbound, '╗');
pub const BELT___DL: BeltMeta = prebelt(Port::None, Port::None, Port::Unknown, Port::Unknown, '╗');
pub const BELT_R_: BeltMeta = prebelt(Port::None, Port::Inbound, Port::None, Port::None, '╶');
pub const BELT_LR_: BeltMeta = prebelt(Port::None, Port::Inbound, Port::None, Port::Inbound, '═');
pub const BELT_R_L: BeltMeta = prebelt(Port::None, Port::Inbound, Port::None, Port::Outbound, '═');
pub const BELT_R__L: BeltMeta = prebelt(Port::None, Port::Inbound, Port::None, Port::Unknown, '═');
pub const BELT_DR_: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Inbound, Port::None, '╔');
pub const BELT_DLR_: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Inbound, Port::Inbound, '╦');
pub const BELT_DR_L: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Inbound, Port::Outbound, '╦');
pub const BELT_DR__L: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Inbound, Port::Unknown, '╦');
pub const BELT_R_D: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Outbound, Port::None, '╔');
pub const BELT_LR_D: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Outbound, Port::Inbound, '╦');
pub const BELT_R_DL: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Outbound, Port::Outbound, '╦');
pub const BELT_R_D_L: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Outbound, Port::Unknown, '╦');
pub const BELT_R__D: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Unknown, Port::None, '╔');
pub const BELT_LR__D: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Unknown, Port::Inbound, '╦');
pub const BELT_R_L_D: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Unknown, Port::Outbound, '╦');
pub const BELT_R__DL: BeltMeta = prebelt(Port::None, Port::Inbound, Port::Unknown, Port::None, '╦');
pub const BELT__R: BeltMeta = prebelt(Port::None, Port::Outbound, Port::None, Port::None, '╶');
pub const BELT_L_R: BeltMeta = prebelt(Port::None, Port::Outbound, Port::None, Port::Inbound, '═');
pub const BELT__LR: BeltMeta = prebelt(Port::None, Port::Outbound, Port::None, Port::Outbound, '═');
pub const BELT__R_L: BeltMeta = prebelt(Port::None, Port::Outbound, Port::None, Port::Unknown, '═');
pub const BELT_D_R: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Inbound, Port::None, '╔');
pub const BELT_DL_R: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Inbound, Port::Inbound, '╦');
pub const BELT_D_LR: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Inbound, Port::Outbound, '╦');
pub const BELT_D_R_L: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Inbound, Port::Unknown, '╦');
pub const BELT__DR: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Outbound, Port::None, '╔');
pub const BELT_L_DR: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Outbound, Port::Inbound, '╦');
pub const BELT__DLR: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Outbound, Port::Outbound, '╦');
pub const BELT__DR_L: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Outbound, Port::Unknown, '╦');
pub const BELT__R_D: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Unknown, Port::None, '╔');
pub const BELT_L_R_D: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Unknown, Port::Inbound, '╦');
pub const BELT__LR_D: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Unknown, Port::Outbound, '╦');
pub const BELT__R_DL: BeltMeta = prebelt(Port::None, Port::Outbound, Port::Unknown, Port::Unknown, '╦');
pub const BELT___R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::None, Port::None, '╶');
pub const BELT_L__R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::None, Port::Inbound, '═');
pub const BELT__L_R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::None, Port::Outbound, '═');
pub const BELT___LR: BeltMeta = prebelt(Port::None, Port::Unknown, Port::None, Port::Unknown, '═');
pub const BELT_D__R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Inbound, Port::None, '╔');
pub const BELT_DL__R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Inbound, Port::Inbound, '╦');
pub const BELT_D_L_R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Inbound, Port::Outbound, '╦');
pub const BELT_D__LR: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Inbound, Port::Unknown, '╦');
pub const BELT__D_R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Outbound, Port::None, '╔');
pub const BELT_L_D_R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Outbound, Port::Inbound, '╦');
pub const BELT__DL_R: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Outbound, Port::Outbound, '╦');
pub const BELT__D_LR: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Outbound, Port::Unknown, '╦');
pub const BELT___DR: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Unknown, Port::None, '╔');
pub const BELT_L__DR: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Unknown, Port::Inbound, '╦');
pub const BELT__L_DR: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Unknown, Port::Outbound, '╦');
pub const BELT___DLR: BeltMeta = prebelt(Port::None, Port::Unknown, Port::Unknown, Port::Unknown, '╦');
pub const BELT_U_: BeltMeta = prebelt(Port::Inbound, Port::None, Port::None, Port::None, '╵');
pub const BELT_LU_: BeltMeta = prebelt(Port::Inbound, Port::None, Port::None, Port::Inbound, '╝');
pub const BELT_U_L: BeltMeta = prebelt(Port::Inbound, Port::None, Port::None, Port::Outbound, '╝');
pub const BELT_U__L: BeltMeta = prebelt(Port::Inbound, Port::None, Port::None, Port::Unknown, '╝');
pub const BELT_DU_: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Inbound, Port::None, '║');
pub const BELT_DLU_: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Inbound, Port::Inbound, '╣');
pub const BELT_DU_L: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Inbound, Port::Outbound, '╣');
pub const BELT_DU__L: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Inbound, Port::Unknown, '╣');
pub const BELT_U_D: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Outbound, Port::None, '║');
pub const BELT_LU_D: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Outbound, Port::Inbound, '╣');
pub const BELT_U_DL: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Outbound, Port::Outbound, '╣');
pub const BELT_U_D_L: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Outbound, Port::Unknown, '╣');
pub const BELT_U__D: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Unknown, Port::None, '║');
pub const BELT_LU__D: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Unknown, Port::Inbound, '╣');
pub const BELT_U_L_D: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Unknown, Port::Outbound, '╣');
pub const BELT_U__DL: BeltMeta = prebelt(Port::Inbound, Port::None, Port::Unknown, Port::Unknown, '╣');
pub const BELT_RU_: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::None, Port::None, '╚');
pub const BELT_LRU_: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::None, Port::Inbound, '╩');
pub const BELT_RU_L: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::None, Port::Outbound, '╩');
pub const BELT_RU__L: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::None, Port::Unknown, '╩');
pub const BELT_DRU_: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Inbound, Port::None, '╠');
pub const BELT_DLRU_: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Inbound, Port::Inbound, '╬');
pub const BELT_DRU_L: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Inbound, Port::Outbound, '╬');
pub const BELT_DRU__L: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Inbound, Port::Unknown, '╬');
pub const BELT_RU_D: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Outbound, Port::None, '╠');
pub const BELT_LRU_D: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Outbound, Port::Inbound, '╬');
pub const BELT_RU_DL: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Outbound, Port::Outbound, '╬');
pub const BELT_RU_D_L: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Outbound, Port::Unknown, '╬');
pub const BELT_RU__D: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Unknown, Port::None, '╠');
pub const BELT_LRU__D: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Unknown, Port::Inbound, '╬');
pub const BELT_RU_L_D: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Unknown, Port::Outbound, '╬');
pub const BELT_RU__DL: BeltMeta = prebelt(Port::Inbound, Port::Inbound, Port::Unknown, Port::Unknown, '╬');
pub const BELT_U_R: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::None, Port::None, '╚');
pub const BELT_LU_R: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::None, Port::Inbound, '╩');
pub const BELT_U_LR: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::None, Port::Outbound, '╩');
pub const BELT_U_R_L: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::None, Port::Unknown, '╩');
pub const BELT_DU_R: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Inbound, Port::None, '╠');
pub const BELT_DLU_R: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Inbound, Port::Inbound, '╬');
pub const BELT_DU_LR: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Inbound, Port::Outbound, '╬');
pub const BELT_DU_R_L: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Inbound, Port::Unknown, '╬');
pub const BELT_U_DR: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Outbound, Port::None, '╠');
pub const BELT_LU_DR: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Outbound, Port::Inbound, '╬');
pub const BELT_U_DLR: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Outbound, Port::Outbound, '╬');
pub const BELT_U_DR_L: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Outbound, Port::Unknown, '╬');
pub const BELT_U_R_D: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Unknown, Port::None, '╠');
pub const BELT_LU_R_D: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Unknown, Port::Inbound, '╬');
pub const BELT_U_LR_D: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Unknown, Port::Outbound, '╬');
pub const BELT_U_R_DL: BeltMeta = prebelt(Port::Inbound, Port::Outbound, Port::Unknown, Port::Unknown, '╬');
pub const BELT_U__R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::None, Port::None, '╚');
pub const BELT_LU__R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::None, Port::Inbound, '╩');
pub const BELT_U_L_R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::None, Port::Outbound, '╩');
pub const BELT_U__LR: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::None, Port::Unknown, '╩');
pub const BELT_DU__R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Inbound, Port::None, '╠');
pub const BELT_DLU__R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Inbound, Port::Inbound, '╬');
pub const BELT_DU_L_R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Inbound, Port::Outbound, '╬');
pub const BELT_DU__LR: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Inbound, Port::Unknown, '╬');
pub const BELT_U_D_R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Outbound, Port::None, '╠');
pub const BELT_LU_D_R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Outbound, Port::Inbound, '╬');
pub const BELT_U_DL_R: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Outbound, Port::Outbound, '╬');
pub const BELT_U_D_LR: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Outbound, Port::Unknown, '╬');
pub const BELT_U__DR: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Unknown, Port::None, '╠');
pub const BELT_LU__DR: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Unknown, Port::Inbound, '╬');
pub const BELT_U_L_DR: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Unknown, Port::Outbound, '╬');
pub const BELT_U__DLR: BeltMeta = prebelt(Port::Inbound, Port::Unknown, Port::Unknown, Port::Unknown, '╬');
pub const BELT__U: BeltMeta = prebelt(Port::Outbound, Port::None, Port::None, Port::None, '╵');
pub const BELT_L_U: BeltMeta = prebelt(Port::Outbound, Port::None, Port::None, Port::Inbound, '╝');
pub const BELT__LU: BeltMeta = prebelt(Port::Outbound, Port::None, Port::None, Port::Outbound, '╝');
pub const BELT__U_L: BeltMeta = prebelt(Port::Outbound, Port::None, Port::None, Port::Unknown, '╝');
pub const BELT_D_U: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Inbound, Port::None, '║');
pub const BELT_DL_U: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Inbound, Port::Inbound, '╣');
pub const BELT_D_LU: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Inbound, Port::Outbound, '╣');
pub const BELT_D_U_L: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Inbound, Port::Unknown, '╣');
pub const BELT__DU: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Outbound, Port::None, '║');
pub const BELT_L_DU: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Outbound, Port::Inbound, '╣');
pub const BELT__DLU: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Outbound, Port::Outbound, '╣');
pub const BELT__DU_L: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Outbound, Port::Unknown, '╣');
pub const BELT__U_D: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Unknown, Port::None, '║');
pub const BELT_L_U_D: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Unknown, Port::Inbound, '╣');
pub const BELT__LU_D: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Unknown, Port::Outbound, '╣');
pub const BELT__U_DL: BeltMeta = prebelt(Port::Outbound, Port::None, Port::Unknown, Port::Unknown, '╣');
pub const BELT_R_U: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::None, Port::None, '╚');
pub const BELT_LR_U: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::None, Port::Inbound, '╩');
pub const BELT_R_LU: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::None, Port::Outbound, '╩');
pub const BELT_R_U_L: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::None, Port::Unknown, '╩');
pub const BELT_DR_U: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Inbound, Port::None, '╠');
pub const BELT_DLR_U: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Inbound, Port::Inbound, '╬');
pub const BELT_DR_LU: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Inbound, Port::Outbound, '╬');
pub const BELT_DR_U_L: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Inbound, Port::Unknown, '╬');
pub const BELT_R_DU: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Outbound, Port::None, '╠');
pub const BELT_LR_DU: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Outbound, Port::Inbound, '╬');
pub const BELT_R_DLU: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Outbound, Port::Outbound, '╬');
pub const BELT_R_DU_L: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Outbound, Port::Unknown, '╬');
pub const BELT_R_U_D: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Unknown, Port::None, '╠');
pub const BELT_LR_U_D: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Unknown, Port::Inbound, '╬');
pub const BELT_R_LU_D: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Unknown, Port::Outbound, '╬');
pub const BELT_R_U_DL: BeltMeta = prebelt(Port::Outbound, Port::Inbound, Port::Unknown, Port::Unknown, '╬');
pub const BELT__RU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::None, Port::None, '╚');
pub const BELT_L_RU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::None, Port::Inbound, '╩');
pub const BELT__LRU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::None, Port::Outbound, '╩');
pub const BELT__RU_L: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::None, Port::Unknown, '╩');
pub const BELT_D_RU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Inbound, Port::None, '╠');
pub const BELT_DL_RU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Inbound, Port::Inbound, '╬');
pub const BELT_D_LRU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Inbound, Port::Outbound, '╬');
pub const BELT_D_RU_L: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Inbound, Port::Unknown, '╬');
pub const BELT__DRU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Outbound, Port::None, '╠');
pub const BELT_L_DRU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Outbound, Port::Inbound, '╬');
pub const BELT__DLRU: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Outbound, Port::Outbound, '╬');
pub const BELT__DRU_L: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Outbound, Port::Unknown, '╬');
pub const BELT__RU_D: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Unknown, Port::None, '╠');
pub const BELT_L_RU_D: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Unknown, Port::Inbound, '╬');
pub const BELT__LRU_D: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Unknown, Port::Outbound, '╬');
pub const BELT__RU_DL: BeltMeta = prebelt(Port::Outbound, Port::Outbound, Port::Unknown, Port::Unknown, '╬');
pub const BELT__U_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::None, Port::None, '╚');
pub const BELT_L_U_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::None, Port::Inbound, '╩');
pub const BELT__LU_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::None, Port::Outbound, '╩');
pub const BELT__U_LR: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::None, Port::Unknown, '╩');
pub const BELT_D_U_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Inbound, Port::None, '╠');
pub const BELT_DL_U_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Inbound, Port::Inbound, '╬');
pub const BELT_D_LU_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Inbound, Port::Outbound, '╬');
pub const BELT_D_U_LR: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Inbound, Port::Unknown, '╬');
pub const BELT__DU_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Outbound, Port::Inbound, '╠');
pub const BELT_L_DU_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Outbound, Port::Inbound, '╬');
pub const BELT__DLU_R: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Outbound, Port::Outbound, '╬');
pub const BELT__DU_LR: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Outbound, Port::Unknown, '╬');
pub const BELT__U_DR: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Unknown, Port::None, '╠');
pub const BELT_L_U_DR: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Unknown, Port::Inbound, '╬');
pub const BELT__LU_DR: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Unknown, Port::Outbound, '╬');
pub const BELT__U_DLR: BeltMeta = prebelt(Port::Outbound, Port::Unknown, Port::Unknown, Port::Unknown, '╬');
pub const BELT___U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::None, Port::None, '╵');
pub const BELT_L__U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::None, Port::Inbound, '╝');
pub const BELT__L_U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::None, Port::Outbound, '╝');
pub const BELT___LU: BeltMeta = prebelt(Port::Unknown, Port::None, Port::None, Port::Unknown, '╝');
pub const BELT_D__U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Inbound, Port::None, '║');
pub const BELT_DL__U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Inbound, Port::Inbound, '╣');
pub const BELT_D_L_U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Inbound, Port::Outbound, '╣');
pub const BELT_D__LU: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Inbound, Port::Unknown, '╣');
pub const BELT__D_U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Outbound, Port::None, '║');
pub const BELT_L_D_U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Outbound, Port::Inbound, '╣');
pub const BELT__DL_U: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Outbound, Port::Outbound, '╣');
pub const BELT__D_LU: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Outbound, Port::Unknown, '╣');
pub const BELT___DU: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Unknown, Port::None, '║');
pub const BELT_L__DU: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Unknown, Port::Inbound, '╣');
pub const BELT__L_DU: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Unknown, Port::Outbound, '╣');
pub const BELT___DLU: BeltMeta = prebelt(Port::Unknown, Port::None, Port::Unknown, Port::Unknown, '╣');
pub const BELT_R__U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::None, Port::None, '╚');
pub const BELT_LR__U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::None, Port::Inbound, '╩');
pub const BELT_R_L_U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::None, Port::Outbound, '╩');
pub const BELT_R__LU: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::None, Port::Unknown, '╩');
pub const BELT_DR__U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Inbound, Port::None, '╠');
pub const BELT_DLR__U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Inbound, Port::Inbound, '╬');
pub const BELT_DR_L_U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Inbound, Port::Outbound, '╬');
pub const BELT_DR__LU: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Inbound, Port::Unknown, '╬');
pub const BELT_R_D_U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Outbound, Port::None, '╠');
pub const BELT_LR_D_U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Outbound, Port::Inbound, '╬');
pub const BELT_R_DL_U: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Outbound, Port::Outbound, '╬');
pub const BELT_R_D_LU: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Outbound, Port::Unknown, '╬');
pub const BELT_R__DU: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Unknown, Port::None, '╠');
pub const BELT_LR__DU: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Unknown, Port::Inbound, '╬');
pub const BELT_R_L_DU: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Unknown, Port::Outbound, '╬');
pub const BELT_R__DLU: BeltMeta = prebelt(Port::Unknown, Port::Inbound, Port::Unknown, Port::Unknown, '╬');
pub const BELT__R_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::None, Port::None, '╚');
pub const BELT_L_R_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::None, Port::Inbound, '╩');
pub const BELT__LR_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::None, Port::Outbound, '╩');
pub const BELT__R_LU: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::None, Port::Unknown, '╩');
pub const BELT_D_R_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Inbound, Port::None, '╠');
pub const BELT_DL_R_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Inbound, Port::Inbound, '╬');
pub const BELT_D_LR_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Inbound, Port::Outbound, '╬');
pub const BELT_D_R_LU: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Inbound, Port::Unknown, '╬');
pub const BELT__DR_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Outbound, Port::None, '╠');
pub const BELT_L_DR_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Outbound, Port::Inbound, '╬');
pub const BELT__DLR_U: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Outbound, Port::Outbound, '╬');
pub const BELT__DR_LU: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Outbound, Port::Unknown, '╬');
pub const BELT__R_DU: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Unknown, Port::None, '╠');
pub const BELT_L_R_DU: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Unknown, Port::Inbound, '╬');
pub const BELT__LR_DU: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Unknown, Port::Outbound, '╬');
pub const BELT__R_DLU: BeltMeta = prebelt(Port::Unknown, Port::Outbound, Port::Unknown, Port::Unknown, '╬');
pub const BELT___RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::None, Port::None, '╚');
pub const BELT_L__RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::None, Port::Inbound, '╩');
pub const BELT__L_RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Outbound, Port::None, '╩');
pub const BELT___LRU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::None, Port::Unknown, '╩');
pub const BELT_D__RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Inbound, Port::None, '╠');
pub const BELT_DL__RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Inbound, Port::Inbound, '╬');
pub const BELT_D_L_RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Inbound, Port::Outbound, '╬');
pub const BELT_D__LRU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Inbound, Port::Unknown, '╬');
pub const BELT__D_RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Outbound, Port::None, '╠');
pub const BELT_L_D_RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Outbound, Port::Inbound, '╬');
pub const BELT__DL_RU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Outbound, Port::Outbound, '╬');
pub const BELT__D_LRU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Outbound, Port::Unknown, '╬');
pub const BELT___DRU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Unknown, Port::None, '╠');
pub const BELT_L__DRU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Unknown, Port::Inbound, '╬');
pub const BELT__L_DRU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Unknown, Port::Outbound, '╬');
pub const BELT___DLRU: BeltMeta = prebelt(Port::Unknown, Port::Unknown, Port::Unknown, Port::Unknown, '╬');

