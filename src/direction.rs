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

pub fn get_from_dir_between_xy(x0: usize, y0: usize, x1: usize, y1: usize) -> Direction {
  let dx = x1 as i8 - x0 as i8;
  let dy = y1 as i8 - y0 as i8;
  return match (dx, dy) {
    (  0,  -1 ) => Direction::Down,
    (  1,   0 ) => Direction::Left,
    (  0,   1 ) => Direction::Up,
    ( -1,   0 ) => Direction::Right,

    _ => panic!("what combination is this? {} {}", dx, dy),
  };
}
