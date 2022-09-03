use crate::cli_serialize::*;
use crate::direction::*;
use crate::port::Port;
use super::belt::*;
use super::cell::*;
use super::factory::*;
use super::floor::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::state::*;

pub fn cli_main(options: &mut Options, state: &mut State) {

  let map = "\
    ...............s.\n\
    sb.111bbbbbbbb.b.\n\
    .b.111.......b.b.\n\
    .b.111bbbbb..bbb.\n\
    .b.b.b....bb...b.\n\
    .b.b.bbbb..b...b.\n\
    .b.b....b..b...b.\n\
    .bbb...222.b...b.\n\
    ...b...222.b..bb.\n\
    sbbb...222.b..b..\n\
    ........b..b..bbs\n\
    ..bbbbbbb..b.....\n\
    ..b.....b..bbbbbd\n\
    ..b.....b........\n\
    dbb..bbbb........\n\
    .....b...........\n\
    .....d...........\n\
    m1 = ws -> b\n\
    m2 = b -> g\n\
    s1 = w\n\
    s2 = w\n\
    s3 = s\n\
    s4 = s\n\
    d1 = s\n\
    d2 = w\n\
    d3 = g\n\
    d4 = g\n\
  ";
  let mut factory = create_factory(options, state, map.to_string(), vec!(PartKind::Sapphire, PartKind::WoodenStick));
  if options.print_initial_table {
    println!("prio: {:?}", factory.prio);
    print_floor_with_views(options, state, &mut factory);
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
  }

  //         "0 123456789012345 6"
  // (   )  "┌───────────────────┐"
  // (0  )  "│         s         │"
  // (   )  "│ ┌───────────────┐ │"
  // (17 )  "│ │ααα    ║       │ │"
  // (34 )  "│ │ααα════╩═════╗ │ │"
  // (51 )  "│ │ααα          ║ │ │"
  // (68 )  "│ │ ║           ║ │ │"
  // (85 )  "│ │ ╚═════╗     ║ │ │"
  // (102)  "│ │       ║     ║ │ │"
  // (119)  "│ │      βββ    ║ │ │"
  // (136)  "│ │      βββ    ║ │ │"
  // (153)  "│ │      βββ    ║ │ │"
  // (170)  "│ │       ║     ╚═│s│"
  // (187)  "│ │ ╔═════╣       │ │"
  // (204)  "│ │ ║     ║       │ │"
  // (221)  "│ │ ║     ║       │ │"
  // (238)  "│d│═╝  ╔══╝       │ │"
  // (255)  "│ │    ║          │ │"
  // (   )  "│ └───────────────┘ │"
  // (272)  "│      d            │"
  // (   )  "└───────────────────┘"
  //         "0 123456789012345 6"

  // println!("supply: {:?}", factory.floor[186]);

  // panic!("okay, gezien");
  //
  // let mut factory = create_factory(options, state, "".to_string());
  //
  // println!("->\n{}", serialize(options, state, &mut factory));

  // Do not record the cost of belt cells. assume them an ongoing 10k x belt cost cost/min modifier
  // Only record the non-belt costs, which happen far less frequently and mean the delta queue
  // will be less than 100 items. Probably slightly under 50, depending on how we tweak speeds.
  // Even 100 items seems well within acceptable ranges. We could even track 10s (1k items) which
  // might be useful to set consistency thresholds ("you need to maintain this efficiency for at
  // least 10s").

  loop {
    tick_factory(options, state, &config, &mut factory);

    if (factory.ticks % options.print_factory_interval) == 0 {
      println!("{:200}", ' ');
      println!("factory @ {} {:200}", factory.ticks, ' ');
      if factory.ticks % 10000 == 0 {
        print_floor_with_views(options, state, &mut factory);
      } else {
        print_floor_without_views(options, state, &mut factory);
      }
      print!("\n{:100}\n{:100}\x1b[{}A\n", ' ', ' ', 50);
    }
    // if factory.ticks == 200000 { break }
  }
}

