#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
  Up = 0,
  Right = 1,
  Down = 2,
  Left = 3,
}

pub fn direction_reverse(dir: Direction) -> Direction {

  // ((dir as u8 + 2) % 4) as Direction

  match dir {
    Direction::Up => Direction::Down,
    Direction::Right => Direction::Left,
    Direction::Down => Direction::Up,
    Direction::Left => Direction::Right,
  }
}
