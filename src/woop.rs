// A woop is a part(ial) on the right side of the screen.
// They are always composed of other parts (woops or atoms).

use super::atom::*;
use super::auto::*;
use super::belt::*;
use super::belt_type::*;
use super::bouncer::*;
use super::cell::*;
use super::canvas::*;
use super::cli_serialize::*;
use super::config::*;
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
use super::quest::*;
use super::state::*;
use super::truck::*;
use super::utils::*;
use super::zone::*;

pub fn woop_is_visible(factory: &Factory, woop_down_woop_index: usize) -> bool {
  return factory.available_woops[woop_down_woop_index].1;
}

pub fn get_woop_xy(index: usize) -> (f64, f64 ) {
  let x = UI_WOOPS_OFFSET_X + (index as f64 % UI_WOOPS_PER_ROW).floor() * UI_WOTOM_WIDTH_PLUS_MARGIN;
  let y = UI_WOOPS_OFFSET_Y + (index as f64 / UI_WOOPS_PER_ROW).floor() * UI_WOTOM_HEIGHT_PLUS_MARGIN;

  return ( x, y );
}
