use super::belt_codes::*;
use super::belt_frame::*;
use super::belt_meta::*;
// use super::belt_type::*;
use super::belt_type::*;
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

#[derive(Debug, Clone)]
pub struct BeltSprite {
  pub sprite_id: usize,
  pub meta: &'static BeltMeta,
  pub frames: Vec<BeltFrame>, // Must always have at least one element
  pub speed: usize, // How long do we show each frame?
}

pub type BeltSprites = [BeltSprite; 275];

// pub fn belt_sprites_default_all_unknown() -> BeltSprites {
//   let unknown = BeltSprite {
//     sprite_id: SPRITE_ID_UNKNOWN,
//     meta: &belt_type_to_belt_meta(belt_name_to_belt_type("UNKNOWN")),
//     frames: vec!(BeltFrame {
//       src: "./img/unknown_belt.png",
//       sx: 0.0,
//       sy: 0.0,
//       sw: 32.0,
//       sh: 32.0
//     }),
//     speed: 1,
//   };
//
//   return BELT_CODES.map(|_| unknown.clone());
// }
