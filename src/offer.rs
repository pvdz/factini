use super::belt::*;
use super::belt_type::*;
use super::bouncer::*;
use super::cell::*;
use super::canvas::*;
use super::cli_serialize::*;
use super::config::*;
use super::craft::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::init::*;
use super::options::*;
use super::machine::*;
use super::maze::*;
use super::part::*;
use super::paste::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::quest_state::*;
use super::quick_save::*;
use super::quote::*;
use super::state::*;
use super::truck::*;
use super::utils::*;
use super::zone::*;
use super::log;

pub fn get_offer_xy(index: usize) -> (f64, f64 ) {
  let x = UI_OFFERS_OFFSET_X + (index as f64 % UI_OFFERS_PER_ROW).floor() * UI_OFFER_WIDTH_PLUS_MARGIN;
  let y = UI_OFFERS_OFFSET_Y + (index as f64 / UI_OFFERS_PER_ROW).floor() * UI_OFFER_HEIGHT_PLUS_MARGIN;

  return ( x, y );
}
