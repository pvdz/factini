use super::part::*;
use super::cell::*;

#[derive(Debug)]
pub struct Offer {
  pub kind: CellKind,
  pub cell_width: usize,
  pub cell_height: usize,
  pub supply_icon: char,
  pub demand_icon: char,
  pub wants: Vec<Part>,
  pub machine_output: char,
  pub speed: u64,
  pub cooldown: u64,
}
