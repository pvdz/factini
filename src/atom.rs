// An atom is a part(ial) on the bottom side of the screen.
// They never require other parts to be created and go on the edge.

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
use super::offer::*;
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
use super::woop::*;
use super::zone::*;

pub fn atom_is_visible(factory: &Factory, woop_down_atom_index: usize) -> bool {
  return factory.available_atoms[woop_down_atom_index].1;
}
