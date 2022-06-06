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
use super::utils::*;

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

pub fn print_floor_with_views(options: &mut Options, state: &mut State, factory: &Factory) {
  let lines = generate_floor_with_views(options, state, factory);
  for line in lines {
    log(format!("{}", line));
  }
}

pub fn generate_floor_with_views(options: &mut Options, state: &mut State, factory: &Factory) -> Vec<String>{
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

  let mut out = vec!();

  let mut cor = 0; // Helps to account for lines where no real cells are printed
  out.push(format!("        \"0 123456789012345 6\"                port_u                     port_r                     port_d                     port_l                       belt_from                   belt_to"));
  //         (   )  "┌───────────────────┐"       "┌───────────────────┐"    "┌───────────────────┐"    "┌───────────────────┐"    "┌───────────────────┐"        "┌───────────────────┐"    "┌───────────────────┐"
  for i in 0..FLOOR_CELLS_H+4 {
    if i != 0 && i != 1 && i != 3 && i != FLOOR_CELLS_H+2 { cor += 1 }
    let argh = format!("{}", cor*FLOOR_CELLS_W);
    let ok = if i != 0 && i != 2 && i != FLOOR_CELLS_H+1 && i != FLOOR_CELLS_H+3 { argh.as_str() } else { "" };
    out.push(format!("({:3})  {:?}       {:?}    {:?}    {:?}    {:?}        {:?}    {:?}", ok, a.next().expect(""), b.next().expect(""), c.next().expect(""), d.next().expect(""), e.next().expect(""), f.next().expect(""), g.next().expect("")));
  }
  //         (   )  "└───────────────────┘"       "└───────────────────┘"    "└───────────────────┘"    "└───────────────────┘"    "└───────────────────┘"        "└───────────────────┘"    "└───────────────────┘"
  out.push(format!("        \"0 123456789012345 6\""));

  return out;
}

pub fn print_floor_without_views(options: &mut Options, state: &mut State, factory: &Factory) {
  let lines = generate_floor_without_views(options, state, factory);
  for line in lines {
    log(format!("{}", line));
  }
}
pub fn generate_floor_without_views(options: &mut Options, state: &mut State, factory: &Factory) -> Vec<String> {
  let aa = serialize(options, state, factory);
  let mut a = aa.split('\n');

  let mut out = vec!();

  let mut cor = 0; // Helps to account for lines where no real cells are printed
  out.push(format!("        \"0 123456789012345 6\"                port_u                     port_r                     port_d                     port_l                       belt_from                   belt_to"));
  //         (   )  "┌───────────────────┐"       "┌───────────────────┐"    "┌───────────────────┐"    "┌───────────────────┐"    "┌───────────────────┐"        "┌───────────────────┐"    "┌───────────────────┐"
  for i in 0..FLOOR_CELLS_H+4 {
    if i != 0 && i != 1 && i != 3 && i != FLOOR_CELLS_H+2 { cor += 1 }
    let argh = format!("{}", cor*FLOOR_CELLS_W);
    let ok = if i != 0 && i != 2 && i != FLOOR_CELLS_H+1 && i != FLOOR_CELLS_H+3 { argh.as_str() } else { "" };
    out.push(format!("({:3})  {:?}", ok, a.next().expect("")));
  }
  //         (   )  "└───────────────────┘"       "└───────────────────┘"    "└───────────────────┘"    "└───────────────────┘"    "└───────────────────┘"        "└───────────────────┘"    "└───────────────────┘"
  out.push(format!("        \"0 123456789012345 6\""));

  return out;
}
