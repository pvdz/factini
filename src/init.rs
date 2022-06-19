use super::belt::*;
use super::cell::*;
use super::cli_serialize::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::offer::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::state::*;
use super::utils::*;

pub fn init() -> ( Options, State, Factory ) {

  // Static state configuration (can still be changed by user)
  let mut options = create_options(1.0);

  // General app state
  let mut state = State {
    paused: false,
    reset_next_frame: false,
  };

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
    m1 = ws -> b s:10\n\
    m2 = b -> g s:10\n\
    s1 = w s:10 c:5\n\
    s2 = w s:10 c:5\n\
    s3 = s s:10 c:5\n\
    s4 = s s:10 c:5\n\
    d1 = s\n\
    d2 = w\n\
    d3 = g\n\
    d4 = g\n\
    os = w s:10 c:5\n\
    os = s s:10 c:5\n\
    od = g\n\
    om = sw -> b s:10\n\
    om = b -> g s:10\n\
  ";

  let factory = create_factory(&mut options, &mut state, map.to_string());

  return ( options, state, factory );
}
