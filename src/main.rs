// RUST_BACKTRACE=1 cargo run
// wasm-pack build --target web

// Ugh. Stop hiding real bugs in mah code.
#![allow(unused_variables, unused_imports, dead_code)]

use std::collections::VecDeque;

pub mod belt;
pub mod cell;
pub mod demand;
pub mod factory;
pub mod floor;
pub mod machine;
pub mod options;
pub mod part;
pub mod state;
pub mod supply;

#[cfg(target_arch = "wasm32")]
pub mod _web;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
  println!("start");

  // Static state configuration (can still be changed by user)
  let mut options = options::create_options();

  // General app state
  let mut state = state::State {};

  let mut factory = factory::create_factory(&mut options, &mut state);

  // Do not record the cost of belt cells. assume them an ongoing 10k x belt cost cost/min modifier
  // Only record the non-belt costs, which happen far less frequently and mean the delta queue
  // will be less than 100 items. Probably slightly under 50, depending on how we tweak speeds.
  // Even 100 items seems well within acceptable ranges. We could even track 10s (1k items) which
  // might be useful to set consistency thresholds ("you need to maintain this efficiency for at
  // least 10s").

  while factory.ticks < (120 * options::ONE_SECOND) {
    factory::tick_factory(&mut options, &mut state, &mut factory);

    if (factory.ticks % options.print_factory_interval) == 0 {
      println!("{:200}", ' ');
      println!("{:200}", ' ');
      println!("{}", factory::serialize_cli(&factory));
      // print!("\x1b[{}A\n", 60);
    }
  }

}
