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
    ........s........\n\
    .mmm....b........\n\
    .mmmbbbbbbbbbbb..\n\
    .mmm..........b..\n\
    ..b...........b..\n\
    ..bbbbbbb.....b..\n\
    ........b.....b..\n\
    .......mmm....b..\n\
    .......mmm....b..\n\
    .......mmm....b..\n\
    ........b.....bbs\n\
    ..bbbbbbb........\n\
    ..b.....b........\n\
    ..b.....b........\n\
    dbb..bbbb........\n\
    .....b...........\n\
    .....d...........\n\
  ";
  let mut factory = create_factory(options, state, map.to_string());
  println!("->\n{}", serialize(options, state, &mut factory));
  println!("supply: {:?}", factory.floor[186]);
  println!("-> port up\n{}", serialize_cb(options, state, &mut factory, |cell: &Cell| match cell.direction_u {
    Port::Inbound => 'v',
    Port::Outbound => '^',
    Port::Unknown => '?',
    Port::None => if cell.kind == CellKind::Empty { ' ' } else { '.' },
  }));
  println!("-> port right\n{}", serialize_cb(options, state, &mut factory, |cell: &Cell| match cell.direction_r {
    Port::Inbound => '<',
    Port::Outbound => '>',
    Port::Unknown => '?',
    Port::None => if cell.kind == CellKind::Empty { ' ' } else { '.' },
  }));
  println!("-> port down\n{}", serialize_cb(options, state, &mut factory, |cell: &Cell| match cell.direction_d {
    Port::Inbound => '^',
    Port::Outbound => 'v',
    Port::Unknown => '?',
    Port::None => if cell.kind == CellKind::Empty { ' ' } else { '.' },
  }));
  println!("-> port left\n{}", serialize_cb(options, state, &mut factory, |cell: &Cell| match cell.direction_l {
    Port::Inbound => '>',
    Port::Outbound => '<',
    Port::Unknown => '?',
    Port::None => if cell.kind == CellKind::Empty { ' ' } else { '.' },
  }));
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
    tick_factory(options, state, &mut factory);

    if (factory.ticks % options.print_factory_interval) == 0 {
      println!("{:200}", ' ');
      println!("factory @ {} {:200}", factory.ticks, ' ');
      // println!("machine TL {:?} {:?} {:?} -> {:?}", factory.floor[8].machine.input_1_have.kind, factory.floor[8].machine.input_2_have.kind, factory.floor[8].machine.input_3_have.kind, factory.floor[8].machine.output_have.kind);
      println!("{}", serialize(options, state, &factory));
      // print!("\x1b[{}A\n", 60);
    }
  }
}

fn serialize(options: &mut Options, state: &mut State, factory: &Factory) -> String {
  let mut out = vec!();

  // Top line
  out.push('┌');
  for coord in 0..FLOOR_CELLS_W+2 {
    out.push('─');
  }
  out.push('┐');

  for coord in 0..FLOOR_CELLS_WH {
    let p = coord % FLOOR_CELLS_W;
    if p == 0 {
      out.push('\n');
    }

    if coord == FLOOR_CELLS_W {
      // Second row. Insert an extra row that is the top-inner border.
      out.push('│');
      out.push(' ');
      out.push('┌');
      for _ in 0..FLOOR_CELLS_W-2 {
        out.push('─');
      }
      out.push('┐');
      out.push(' ');
      out.push('│');
      out.push('\n');
    } else if coord == FLOOR_CELLS_WH - FLOOR_CELLS_W {
      // Last row from the bottom. Insert extra row that is the bottom-inner border.

      // Second row. Insert an extra row that is the top-inner border.
      out.push('│');
      out.push(' ');
      out.push('└');
      for _ in 0..FLOOR_CELLS_W-2 {
        out.push('─');
      }
      out.push('┘');
      out.push(' ');
      out.push('│');
      out.push('\n');
    }

    if p == 0 {
      // Left most (edge) cells
      out.push('│'); // left/right outer edge
    } else if p == 1 {
      // Inner edge to the left
      if coord > FLOOR_CELLS_W && coord < FLOOR_CELLS_WH - FLOOR_CELLS_W {
        out.push('│');
      } else {
        out.push(' ');
      }
    }

    // These are the actual board elements
    out.push(match factory.floor[coord].kind {
      CellKind::Empty => ' ',
      CellKind::Supply => 's',
      CellKind::Demand => 'd',
      CellKind::Machine => {
        let greek_lo = "αβγδεζηθικλμνξοπρςτυφχψω";
        let greek_hi = "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ";
        let id = factory.floor[coord].machine.id;
        if id < 24 {
          match greek_lo.char_indices().nth(id) {
            None => '!',
            Some((_, c)) => c,
          }
        } else {
          (65 + id) as u8 as char
        }
      },
      CellKind::Belt => factory.floor[coord].belt.meta.cli_icon,
    });

    if p == FLOOR_CELLS_W - 2 {
      // Inner edge to the left
      if coord > FLOOR_CELLS_W && coord < FLOOR_CELLS_WH - FLOOR_CELLS_W {
        out.push('│');
      } else {
        out.push(' ');
      }
    } else if p == FLOOR_CELLS_W - 1 {
      // Right most (edge) cells
      out.push('│'); // left/right outer edge
    }
  }

  // Bottom line
  out.push('\n');
  out.push('└');
  for coord in 0..FLOOR_CELLS_W+2 {
    out.push('─');
  }
  out.push('┘');


  return out.iter().collect();
}

fn serialize_cb(options: &mut Options, state: &mut State, factory: &Factory, cb: fn(&Cell) -> char) -> String {
  let mut out = vec!();

  // Top line
  out.push('┌');
  for coord in 0..FLOOR_CELLS_W+2 {
    out.push('─');
  }
  out.push('┐');

  for coord in 0..FLOOR_CELLS_WH {
    let p = coord % FLOOR_CELLS_W;
    if p == 0 {
      out.push('\n');
    }

    if coord == FLOOR_CELLS_W {
      // Second row. Insert an extra row that is the top-inner border.
      out.push('│');
      out.push(' ');
      out.push('┌');
      for _ in 0..FLOOR_CELLS_W-2 {
        out.push('─');
      }
      out.push('┐');
      out.push(' ');
      out.push('│');
      out.push('\n');
    } else if coord == FLOOR_CELLS_WH - FLOOR_CELLS_W {
      // Last row from the bottom. Insert extra row that is the bottom-inner border.

      // Second row. Insert an extra row that is the top-inner border.
      out.push('│');
      out.push(' ');
      out.push('└');
      for _ in 0..FLOOR_CELLS_W-2 {
        out.push('─');
      }
      out.push('┘');
      out.push(' ');
      out.push('│');
      out.push('\n');
    }

    if p == 0 {
      // Left most (edge) cells
      out.push('│'); // left/right outer edge
    } else if p == 1 {
      // Inner edge to the left
      if coord > FLOOR_CELLS_W && coord < FLOOR_CELLS_WH - FLOOR_CELLS_W {
        out.push('│');
      } else {
        out.push(' ');
      }
    }

    // These are the actual board elements
    out.push(cb(&factory.floor[coord]));

    if p == FLOOR_CELLS_W - 2 {
      // Inner edge to the left
      if coord > FLOOR_CELLS_W && coord < FLOOR_CELLS_WH - FLOOR_CELLS_W {
        out.push('│');
      } else {
        out.push(' ');
      }
    } else if p == FLOOR_CELLS_W - 1 {
      // Right most (edge) cells
      out.push('│'); // left/right outer edge
    }
  }

  // Bottom line
  out.push('\n');
  out.push('└');
  for coord in 0..FLOOR_CELLS_W+2 {
    out.push('─');
  }
  out.push('┘');


  return out.iter().collect();
}
