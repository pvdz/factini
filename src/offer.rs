use super::cell::*;

#[derive(Debug)]
pub struct Offer {
  pub kind: CellKind,
  pub cell_width: usize,
  pub cell_height: usize,
  pub supply_icon: char,
  pub demand_icon: char,
  pub machine_input1: char,
  pub machine_input2: char,
  pub machine_input3: char,
  pub machine_output: char,
  pub speed: u64,
  pub cooldown: u64,
}
