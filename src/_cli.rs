use crate::direction::Direction;
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
    .111....b........\n\
    .111bbbbbbbbbbb..\n\
    .111..........b..\n\
    ..b...........b..\n\
    ..bbbbbbb.....b..\n\
    ........b.....b..\n\
    .......222....b..\n\
    .......222....b..\n\
    .......222....b..\n\
    ........b.....bbs\n\
    ..bbbbbbb........\n\
    ..b.....b........\n\
    ..b.....b........\n\
    dbb..bbbb........\n\
    .....b...........\n\
    .....d...........\n\
    m1 = ws -> b\n\
    m2 = b -> g\n\
    s1 = w\n\
    s2 = s\n\
    d1 = g\n\
    d2 = g\n\
  ";
  let mut factory = create_factory(options, state, map.to_string());
  println!("prio: {:?}", factory.prio);
  print_floor_with_views(options, state, &mut factory);


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
    tick_factory(options, state, &mut factory);

    if (factory.ticks % options.print_factory_interval) == 0 {
      println!("{:200}", ' ');
      println!("factory @ {} {:200}", factory.ticks, ' ');
      // println!("machine TL {:?} {:?} {:?} -> {:?}", factory.floor[8].machine.input_1_have.kind, factory.floor[8].machine.input_2_have.kind, factory.floor[8].machine.input_3_have.kind, factory.floor[8].machine.output_have.kind);
      if factory.ticks % 10000 == 0 {
        print_floor_with_views(options, state, &mut factory);
      } else {
        print_floor_without_views(options, state, &mut factory);
      }
      // print!("\n{:100}\n{:100}\x1b[{}A\n", ' ', ' ', 50);
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
      CellKind::Belt => {
        if factory.floor[coord].belt.part.kind != PartKind::None {
          factory.floor[coord].belt.part.icon
        } else {
          factory.floor[coord].belt.meta.cli_icon
        }
      },
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

fn print_floor_with_views(options: &mut Options, state: &mut State, factory: &Factory) {

  let aa = serialize(options, state, factory);
  let mut a = aa.split('\n');

  let bb = serialize_cb(options, state, factory, |cell: &Cell| match cell.port_u {
    Port::Inbound => 'v',
    Port::Outbound => '^',
    Port::Unknown => '?',
    Port::None => if cell.kind == CellKind::Empty { ' ' } else { '.' },
  });
  let mut b = bb.split('\n');

  let cc = serialize_cb(options, state, factory, |cell: &Cell| match cell.port_r {
    Port::Inbound => '<',
    Port::Outbound => '>',
    Port::Unknown => '?',
    Port::None => if cell.kind == CellKind::Empty { ' ' } else { '.' },
  });
  let mut c = cc.split('\n');

  let dd = serialize_cb(options, state, factory, |cell: &Cell| match cell.port_d {
    Port::Inbound => '^',
    Port::Outbound => 'v',
    Port::Unknown => '?',
    Port::None => if cell.kind == CellKind::Empty { ' ' } else { '.' },
  });
  let mut d = dd.split('\n');

  let ee = serialize_cb(options, state, factory, |cell: &Cell| match cell.port_l {
    Port::Inbound => '>',
    Port::Outbound => '<',
    Port::Unknown => '?',
    Port::None => if cell.kind == CellKind::Empty { ' ' } else { '.' },
  });
  let mut e = ee.split('\n');


  let ff = serialize_cb(options, state, factory, |cell: &Cell| if cell.kind != CellKind::Belt { ' ' } else if cell.belt.part.kind == PartKind::None { '-' } else { match cell.belt.part_from {
    Direction::Up => 'u',
    Direction::Right => 'r',
    Direction::Down => 'd',
    Direction::Left => 'l',
  }});
  let mut f = ff.split('\n');

  let gg = serialize_cb(options, state, factory, |cell: &Cell| if cell.kind != CellKind::Belt { ' ' } else if cell.belt.part.kind == PartKind::None { '-' } else { match cell.belt.part_to {
    Direction::Up => 'u',
    Direction::Right => 'r',
    Direction::Down => 'd',
    Direction::Left => 'l',
  }});
  let mut g = gg.split('\n');



  let mut cor = 0; // Helps to account for lines where no real cells are printed
  println!("        \"0 123456789012345 6\"                port_u                     port_r                     port_d                     port_l                       belt_from                   belt_to");
  //         (   )  "┌───────────────────┐"       "┌───────────────────┐"    "┌───────────────────┐"    "┌───────────────────┐"    "┌───────────────────┐"        "┌───────────────────┐"    "┌───────────────────┐"
  for i in 0..FLOOR_CELLS_H+4 {
    if i != 0 && i != 1 && i != 3 && i != FLOOR_CELLS_H+2 { cor += 1 }
    let argh = format!("{}", cor*FLOOR_CELLS_W);
    let ok = if i != 0 && i != 2 && i != FLOOR_CELLS_H+1 && i != FLOOR_CELLS_H+3 { argh.as_str() } else { "" };
    println!("({:3})  {:?}       {:?}    {:?}    {:?}    {:?}        {:?}    {:?}", ok, a.next().expect(""), b.next().expect(""), c.next().expect(""), d.next().expect(""), e.next().expect(""), f.next().expect(""), g.next().expect(""));
  }
  //         (   )  "└───────────────────┘"       "└───────────────────┘"    "└───────────────────┘"    "└───────────────────┘"    "└───────────────────┘"        "└───────────────────┘"    "└───────────────────┘"
  println!("        \"0 123456789012345 6\"");
}

fn print_floor_without_views(options: &mut Options, state: &mut State, factory: &Factory) {

  let aa = serialize(options, state, factory);
  let mut a = aa.split('\n');

  let mut cor = 0; // Helps to account for lines where no real cells are printed
  println!("        \"0 123456789012345 6\"                port_u                     port_r                     port_d                     port_l                       belt_from                   belt_to");
  //         (   )  "┌───────────────────┐"       "┌───────────────────┐"    "┌───────────────────┐"    "┌───────────────────┐"    "┌───────────────────┐"        "┌───────────────────┐"    "┌───────────────────┐"
  for i in 0..FLOOR_CELLS_H+4 {
    if i != 0 && i != 1 && i != 3 && i != FLOOR_CELLS_H+2 { cor += 1 }
    let argh = format!("{}", cor*FLOOR_CELLS_W);
    let ok = if i != 0 && i != 2 && i != FLOOR_CELLS_H+1 && i != FLOOR_CELLS_H+3 { argh.as_str() } else { "" };
    println!("({:3})  {:?}", ok, a.next().expect(""));
  }
  //         (   )  "└───────────────────┘"       "└───────────────────┘"    "└───────────────────┘"    "└───────────────────┘"    "└───────────────────┘"        "└───────────────────┘"    "└───────────────────┘"
  println!("        \"0 123456789012345 6\"");
}
