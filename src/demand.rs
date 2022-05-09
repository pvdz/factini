use super::part::*;

pub struct Demand {
  pub x: usize,
  pub y: usize,
  pub part_kind: PartKind, // The part kind that this demander is waiting for
  pub part_price: i32, // Amount of money you receive when supplying the proper part
  pub trash_price: i32, // Penalty you pay for giving the wrong part
}
