use super::part::*;

pub struct Supply {
  pub x: usize,
  pub y: usize,
  pub ticks: u64,
  pub part: Part, // Current part that's moved out (can be none for noop)
  pub part_at: u64, // Last time part was generated
  pub last_part_out_at: u64, // Last time a part left this supply
  pub speed: u64, // Number of ticks after which a new part is ready to move to neighboring cell
  pub interval: u64, // Generate new part every this many ticks
  pub stamp: Part, // The example Part that this supply will generate
  pub part_price: i32, // Cost when dispensing one part
}
