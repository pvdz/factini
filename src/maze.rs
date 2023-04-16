use super::belt::*;
use super::bouncer::*;
use super::cell::*;
use super::cli_serialize::*;
use super::cli_deserialize::*;
use super::config::*;
use super::demand::*;
use super::factory::*;
use super::floor::*;
use super::machine::*;
use super::options::*;
use super::part::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::quest_state::*;
use super::quote::*;
use super::state::*;
use super::supply::*;
use super::truck::*;
use super::utils::*;
use super::zone::*;
use super::log;

pub const MAZE_CELL_SIZE: f64 = 12.0;
pub const MAZE_WIDTH: f64 = 25.0 * MAZE_CELL_SIZE;
pub const MAZE_HEIGHT: f64 = 21.0 * MAZE_CELL_SIZE;
pub const MAZE_CELLS_W: usize = (MAZE_WIDTH / MAZE_CELL_SIZE) as usize;
pub const MAZE_CELLS_H: usize = (MAZE_HEIGHT / MAZE_CELL_SIZE) as usize;

const SPECIAL_DENSITY: f64 = 0.1; // 5% of cells are specials

fn dnow() -> u64 {
  js_sys::Date::now() as u64
}

const MAZE_EMPTY: u8 = 0;
const MAZE_ROCK: u8 = 1;
const MAZE_TREASURE: u8 = 2;

#[derive(Debug, Clone)]
pub struct MazeCell {
  pub state: u8,
  pub last_dir: u8,
  pub special: u8,

  pub up: bool,
  pub left: bool,

  pub has_up: bool,
  pub has_right: bool,
  pub has_down: bool,
  pub has_left: bool,

  pub up_index: usize,
  pub right_index: usize,
  pub down_index: usize,
  pub left_index: usize,
}

#[derive(Debug, Clone)]
pub struct MazeRunner {
  pub x: usize,
  pub y: usize,

  // tbd
  pub energy_max: u64, // max energy you began with
  pub energy_now: u64, // energy left currently
  pub speed: u64, // flat multiplier that influences tick interval
  pub power_max: u64, // how many hammers did you start with
  pub power_now: u64, // how many hammers do you have left (how many rocks can you break)
  pub volume_max: u64, // absolute number of gold pieces you can carry
  pub volume_now: u64, // absolute number of gold pieces you are carrying
}

pub fn create_maze() -> Vec<MazeCell> {
  let mut z: usize = dnow() as usize;
  let mut n = 0;
  let mut vertices = [(); (MAZE_CELLS_W * MAZE_CELLS_H) as usize].map(|_| {
    z = xorshift(z);

    // ( up, left, visited_dead, last_dir )
    let v = MazeCell {
      // visited / finished
      state: 0u8,
      // last dir
      last_dir: 0u8,
      // special cell state / contents
      special: MAZE_EMPTY,
      // connected up and left
      up: z % 2 == 1,
      left: z % 2 == 1,
      // has neighbor on u/r/d/l side
      has_up: n >= MAZE_CELLS_W,
      has_right: n % MAZE_CELLS_W != MAZE_CELLS_W - 1,
      has_down: n < MAZE_CELLS_W * MAZE_CELLS_H - MAZE_CELLS_W,
      has_left: n % MAZE_CELLS_W != 0,
      // neighbor index (if there is one)
      up_index: (n - MAZE_CELLS_W) as usize,
      right_index: (n + 1) as usize,
      down_index: (n + MAZE_CELLS_W) as usize,
      left_index: (n - 1) as usize,
    };
    n += 1;
    return v;
  }).to_vec();

  // Reset all connections
  for v in vertices.iter_mut() {
    v.up = false;
    v.left = false;
  }

  // Given a grid where none of the cells are connected to each other
  // - start at some cell, push it on the stack
  // - while the stack is not empty
  //   - pick next cell from stack
  //   - if it has no more valid neighbors left to visit, skip to next on stack
  //   - pick a random valid orthogonal direction to an unvisited cell and connect it
  //   - push the index on the stack
  //   - push the neighbor on the stack


  let mut stack: Vec<usize> = vec!(0);
  vertices[0].state = 1; // "connected"
  while stack.len() > 0 {
    let index = stack.pop().unwrap();

    if vertices[index].state == 2 { panic!("should be prevented"); }
    if vertices[index].state != 1 { panic!("but how"); }

    z = xorshift(z);

    let mut options = vec!();

    let i = vertices[index].up_index;
    if vertices[index].has_up && vertices[i].state == 0 {
      options.push(0);
    }
    let i = vertices[index].right_index;
    if vertices[index].has_right && vertices[i].state == 0 {
      options.push(1);
    }
    let i = vertices[index].down_index;
    if vertices[index].has_down && vertices[i].state == 0 {
      options.push(2);
    }
    let i = vertices[index].left_index;
    if vertices[index].has_left && vertices[i].state == 0 {
      options.push(3);
    }

    if options.len() > 0 {
      let i = options[z as usize % options.len()];
      match i {
        0 => {
          let i = vertices[index].up_index;
          vertices[i].state = 1; // cell connected
          vertices[index].up = true; // connect up side of this cell
          stack.push(index);
          stack.push(i);
        }
        1 => {
          let i = vertices[index].right_index;
          vertices[i].state = 1; // cell connected
          vertices[i].left = true; // connect left side of right cell
          stack.push(index);
          stack.push(i);
        }
        2 => {
          let i = vertices[index].down_index;
          vertices[i].state = 1; // cell connected
          vertices[i].up = true; // connect up side of bottom cell
          stack.push(index);
          stack.push(i);
        }
        3 => {
          let i = vertices[index].left_index;
          vertices[i].state = 1; // cell connected
          vertices[index].left = true; // connect left side of this cell
          stack.push(index);
          stack.push(i);
        }
        _ => panic!("cant be"),
      }
    } else {
      vertices[index].state = 2;
    }
  }

  for v in vertices.iter_mut() {
    v.state = 0;
    v.last_dir = 0;
  }

  // Spread special states by dividing the linear space into N sections and picking one
  // of each of those sections to fill with the special.
  // Specials could be bonuses, points, or obstacles. TBD.

  let mut z: usize = dnow() as usize;

  let special_count = ((MAZE_CELLS_W * MAZE_CELLS_H) as f64 * SPECIAL_DENSITY) as usize;
  let section_size = ((vertices.len() as f64 / (special_count as f64)) as usize).max(1);
  log!("section_size={} special_count={} vertices.len()={}", section_size, special_count, vertices.len());
  for i in 0..special_count {
    z = xorshift(z);

    let what = match z % 10 {
      | 0
      | 1
      | 2
      | 3 => MAZE_ROCK,
      _ => MAZE_TREASURE,
    };

    z = xorshift(z);

    vertices[i * section_size + (z % section_size)].special = what;
  }

  return vertices;
}

pub fn create_maze_runner(x: usize, y: usize) -> MazeRunner {
  return MazeRunner {
    x,
    y,

    energy_max: 1000,
    energy_now: 800,
    speed: 5,
    power_max: 10,
    power_now: 3,
    volume_max: 10,
    volume_now: 3,
  }
}

pub fn tick_maze(options: &Options, state: &State, config: &Config, factory: &mut Factory) {
  if factory.ticks % 150 == 0 {
    let x = (GRID_X2 + MAZE_WIDTH / 2.0).floor() + 0.5;
    let y = (GRID_Y1 + FLOOR_HEIGHT - MAZE_WIDTH).floor() + 0.5;

    if factory.maze_runner.energy_now > 0 {
      factory.maze_runner.energy_now -= 1;
    }
    if factory.maze_runner.energy_now == 0 {
      return;
    }

    // Move. Then increment visit count of current coord.
    let index = (factory.maze_runner.y as f64 * (MAZE_CELLS_W as f64) + factory.maze_runner.x as f64) as usize;
    {
      if factory.maze[index].state < 255 {
        let has_power = factory.maze_runner.power_now > 0;

        let can_up = factory.maze[index].has_up && factory.maze[index].up && factory.maze[factory.maze[index].up_index].state < 255 && (has_power || factory.maze[factory.maze[index].up_index].special != MAZE_ROCK);
        let can_right = factory.maze[index].has_right && factory.maze[factory.maze[index].right_index].left && factory.maze[factory.maze[index].right_index].state < 255 && (has_power || factory.maze[factory.maze[index].right_index].special != MAZE_ROCK);
        let can_down = factory.maze[index].has_down && factory.maze[factory.maze[index].down_index].up && factory.maze[factory.maze[index].down_index].state < 255 && (has_power || factory.maze[factory.maze[index].down_index].special != MAZE_ROCK);
        let can_left = factory.maze[index].has_left && factory.maze[index].left && factory.maze[factory.maze[index].left_index].state < 255 && (has_power || factory.maze[factory.maze[index].left_index].special != MAZE_ROCK);

        // log!("maze runner @ {}: can go up: {}, right: {}, down: {}, left: {}", index, can_up, can_right, can_down, can_left);

        let mut options = vec!();
        let mut min = 255;
        let mut sum = 0;
        if can_up {
          sum += 1;
          if factory.maze[factory.maze[index].up_index].state <= min {
            if factory.maze[factory.maze[index].up_index].state < min {
              options = vec!(0);
              min = factory.maze[factory.maze[index].up_index].state;
            } else {
              options.push(0);
            }
          }
        }
        if can_right {
          sum += 1;
          if factory.maze[factory.maze[index].right_index].state <= min {
            if factory.maze[factory.maze[index].right_index].state < min {
              options = vec!(1);
              min = factory.maze[factory.maze[index].right_index].state;
            } else {
              options.push(1);
            }
          }
        }
        if can_down {
          sum += 1;
          if factory.maze[factory.maze[index].down_index].state <= min {
            if factory.maze[factory.maze[index].down_index].state < min {
              options = vec!(2);
              min = factory.maze[factory.maze[index].down_index].state;
            } else {
              options.push(2);
            }
          }
        }
        if can_left {
          sum += 1;
          if factory.maze[factory.maze[index].left_index].state <= min {
            if factory.maze[factory.maze[index].left_index].state < min {
              options = vec!(3);
              // min = factory.maze[factory.maze[index].left_index].state;
            } else {
              options.push(3);
            }
          }
        }

        if sum == 0 {
          // log!("stuck!!");
          factory.maze[index].state = 255;
        } else {
          if sum <= 1 {
            // log!("maze runner - marking {} as dead end", index);
            // Dead end. Go back if possible. Otherwise we don't move.
            factory.maze[index].state = 255;
          } else {
            if factory.maze[index].state < 255 {
              factory.maze[index].state += 1;
            }
          }

          let offset = options[(factory.ticks as usize) % options.len()];
          if offset == 0 {
            factory.maze_runner.y -= 1;
          } else if offset == 1 {
            factory.maze_runner.x += 1;
          } else if offset == 2 {
            factory.maze_runner.y += 1;
          } else if offset == 3 {
            factory.maze_runner.x -= 1;
          } else {
            panic!("only has 0-3");
          }

          let mut new_index = factory.maze_runner.y * MAZE_CELLS_W + factory.maze_runner.x;
          match factory.maze[new_index].special {
            // MAZE_EMPTY
            0 => {}
            // MAZE_ROCK
            1 => {
              if factory.maze_runner.power_now == 0 { panic!("should check above if power left"); }
              factory.maze_runner.power_now -= 1;
              factory.maze[new_index].special = MAZE_EMPTY;
            }
            // MAZE_TREASURE
            2 => {
              if factory.maze_runner.volume_now < factory.maze_runner.volume_max {
                factory.maze_runner.volume_now += 1;
                factory.maze[new_index].special = MAZE_EMPTY;
              }
            }
            n => panic!("not yet supported...: {}", n),
          }
        }
      }
    }
  }
}
