
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

pub fn part_none(x: usize, y: usize) -> Part {
  return Part {
    kind: PartKind::None,
    icon: ' ',
  }
}
