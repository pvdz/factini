// RUST_BACKTRACE=1 cargo run
// wasm-pack build --target web

// Ugh. Stop hiding real bugs in mah code.
#![allow(unused_variables, unused_imports, dead_code)]

use std::collections::VecDeque;

#[cfg(not(target_arch = "wasm32"))]
pub mod _cli;

pub mod atom;
pub mod auto;
pub mod belt;
pub mod belt_codes;
pub mod belt_frame;
pub mod belt_meta;
pub mod belt_type;
pub mod bouncer;
pub mod canvas;
pub mod cell;
pub mod cli_serialize;
pub mod cli_deserialize;
pub mod config;
pub mod demand;
pub mod direction;
pub mod factory;
pub mod floor;
pub mod init;
pub mod machine;
pub mod maze;
pub mod options;
pub mod offer;
pub mod part;
pub mod paste;
pub mod port;
pub mod port_auto;
pub mod prio;
pub mod quest_state;
pub mod quick_save;
pub mod quest;
pub mod sprite_config;
pub mod sprite_frame;
pub mod state;
pub mod story;
pub mod supply;
pub mod truck;
pub mod utils;
pub mod woop;
pub mod zone;

#[cfg(target_arch = "wasm32")]
pub mod _web;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
  println!("cli start");

  // std::env::set_var("RUST_BACKTRACE", "1");

  // Static state configuration (can still be changed by user)
  let mut options = options::create_options(1.0, 1.0);

  // General app state
  let mut state = state::State {};

  _cli::cli_main(&mut options, &mut state);
}
