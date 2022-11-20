use super::belt_codes::*;
use super::belt_meta::*;
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
use super::log;

// Meant for animated belts

#[derive(Debug, Clone)]
pub struct BeltFrame {
  pub src: &'static str,
  pub sx: f64,
  pub sy: f64,
  pub sw: f64,
  pub sh: f64,
}
