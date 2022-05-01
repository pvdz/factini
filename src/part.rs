
#[derive(Clone, Debug)]
pub struct Part {
  pub kind: PartKind,
  pub icon: char,
}

#[derive(Clone, Copy, Debug)]
pub enum PartKind {
  None,
  Car,
  Doll,
  Fire,
  Flamingo,
}

pub fn part_none(x: usize, y: usize) -> Part {
  return Part {
    kind: PartKind::None,
    icon: ' ',
  }
}
