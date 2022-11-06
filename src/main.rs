// RUST_BACKTRACE=1 cargo run
// wasm-pack build --target web

// Ugh. Stop hiding real bugs in mah code.
#![allow(unused_variables, unused_imports, dead_code)]

use std::collections::VecDeque;

#[cfg(not(target_arch = "wasm32"))]
pub mod _cli;

pub mod belt;
pub mod belt_type2;
pub mod bouncer;
pub mod cell;
pub mod cli_serialize;
pub mod cli_deserialize;
pub mod config;
pub mod craft;
pub mod demand;
pub mod direction;
pub mod factory;
pub mod floor;
pub mod init;
pub mod machine;
pub mod options;
pub mod part;
pub mod paste;
pub mod port;
pub mod port_auto;
pub mod prio;
pub mod quote;
pub mod state;
pub mod supply;
pub mod truck;
pub mod utils;
pub mod zone;

#[cfg(target_arch = "wasm32")]
pub mod _web;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
  println!("cli start");

  // std::env::set_var("RUST_BACKTRACE", "1");

  // Static state configuration (can still be changed by user)
  let mut options = options::create_options(1.0);

  // General app state
  let mut state = state::State {};

  _cli::cli_main(&mut options, &mut state);
}
