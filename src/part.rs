use super::utils::*;

#[derive(Clone, Debug)]
pub struct Part {
  pub kind: PartKind,
  pub icon: char,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PartKind {
  None,
  Sapphire,
  WoodenStick,
  BlueWand,
  GoldenBlueWand,
}

pub const fn part_none() -> Part {
  return Part {
    kind: PartKind::None,
    icon: ' ',
  }
}

pub const fn part_c(icon: char) -> Part {
  return Part {
    kind: match icon {
      'w' => PartKind::WoodenStick,
      's' => PartKind::Sapphire,
      'b' => PartKind::BlueWand,
      'g' => PartKind::GoldenBlueWand,
      ' ' => PartKind::None,
      _ => PartKind::None,
    },
    icon,
  }
}
