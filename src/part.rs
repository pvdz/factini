
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
