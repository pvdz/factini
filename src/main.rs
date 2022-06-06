// RUST_BACKTRACE=1 cargo run
// wasm-pack build --target web

// Ugh. Stop hiding real bugs in mah code.
#![allow(unused_variables, unused_imports, dead_code)]

use std::collections::VecDeque;

#[cfg(not(target_arch = "wasm32"))]
pub mod _cli;

pub mod belt;
pub mod cell;
pub mod cli_serialize;
pub mod demand;
pub mod direction;
pub mod factory;
pub mod floor;
pub mod machine;
pub mod options;
pub mod part;
pub mod port;
pub mod prio;
pub mod state;
pub mod supply;
pub mod utils;

#[cfg(target_arch = "wasm32")]
pub mod _web;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
  println!("cli start");

  // Static state configuration (can still be changed by user)
  let mut options = options::create_options(1.0);

  // General app state
  let mut state = state::State {};

  _cli::cli_main(&mut options, &mut state);
}
