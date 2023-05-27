// An offer represents a draggable part, atoms have no patterns and are the offers below
// while woops are the offers with a pattern and are to the right..
// "Woop" was chosen because I couldn't think of a reasonable unique short name for it.

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
use super::quote::*;
use super::state::*;
use super::truck::*;
use super::utils::*;
use super::zone::*;
use super::log;

pub fn get_atom_xy(index: usize) -> (f64, f64 ) {
  let x = UI_ATOMS_OFFSET_X + (index as f64 % UI_ATOMS_PER_ROW).floor() * UI_WOTOM_WIDTH_PLUS_MARGIN;
  let y = UI_ATOMS_OFFSET_Y + (index as f64 / UI_ATOMS_PER_ROW).floor() * UI_WOTOM_HEIGHT_PLUS_MARGIN;

  return ( x, y );
}

pub fn get_woop_xy(index: usize) -> (f64, f64 ) {
  let x = UI_WOOPS_OFFSET_X + (index as f64 % UI_WOOPS_PER_ROW).floor() * UI_WOTOM_WIDTH_PLUS_MARGIN;
  let y = UI_WOOPS_OFFSET_Y + (index as f64 / UI_WOOPS_PER_ROW).floor() * UI_WOTOM_HEIGHT_PLUS_MARGIN;

  return ( x, y );
}
