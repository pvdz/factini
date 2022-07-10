use super::belt::*;
use super::cell::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::machine::*;
use super::options::*;
use super::offer::*;
use super::part::*;
use super::port::*;
use super::state::*;
use super::utils::*;

fn serialize(options: &mut Options, state: &mut State, factory: &Factory, dump: bool) -> String {
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
        // if id < 24 {
        //   match greek_lo.char_indices().nth(id) {
        //     None => '!',
        //     Some((_, c)) => c,
        //   }
        // } else {
        ('0' as usize + id) as u8 as char
        // }
      },
      CellKind::Belt => {
        if !dump && factory.floor[coord].belt.part.kind != PartKind::None {
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
  let aa = serialize(options, state, factory, false);
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
  let aa = serialize(options, state, factory, false);
  let mut a = aa.split('\n');

  let mut out = vec!();

  let mut cor = 0; // Helps to account for lines where no real cells are printed
  out.push(format!("        \"0 123456789012345 6\""));
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
pub fn generate_floor_dump(options: &mut Options, state: &mut State, factory: &Factory) -> Vec<String> {
  // Send help. I'm sure this is wrong on multiple levels. But it works.
  let aa = serialize2(options, state, factory, true);
  let a = aa.split('\n');
  let mut out = vec!();
  for n in a {
    out.push(format!("{}", n));
  }
  return out;
}



pub fn serialize2(options: &mut Options, state: &mut State, factory: &Factory, dump: bool) -> String {
  // Create a string that we can parse again. This requires to be explicit about the port states.
  // While it would be super nice to have a condensed string, there's just too many variations.
  // There are four ports and each port can have one of four states (none, unknown, in, out) so
  // that makes for 4^4=256 different tile states just for signifying ports. That doesn't even cover
  // the machines. It's just infeasible and a bit of a parsing nightmare, although this is even
  // harder because now we have to parse a 3x3 but whatever.

  let mut out = vec!(
    vec!('d','=','1','7','x','1','7','\\','n','\\','\n'),
  );

  let mut line1: Vec<char> = vec!();
  let mut line2: Vec<char> = vec!();
  let mut line3: Vec<char> = vec!();
  let mut cell_params: Vec<char> = vec!();
  let mut machine_count: u8 = 0;
  let mut supply_count: u8 = 0;
  let mut demand_count: u8 = 0;

  // Generate the floor left to right, top to bottom, so iterate the floor linearly.
  for coord in 0..FLOOR_CELLS_WH {

    // if factory.floor[coord].kind == CellKind::Machine && factory.floor[coord].machine.main_coord == coord {
    //   log(format!("machine size: {} {}", factory.floor[coord].machine.cell_width, factory.floor[coord].machine.cell_height));
    // }

    let center_char = match factory.floor[coord].kind {
      CellKind::Empty => '.',
      CellKind::Supply => 's',
      CellKind::Demand => 'd',
      CellKind::Machine => {
        let greek_lo = "αβγδεζηθικλμνξοπρςτυφχψω";
        let greek_hi = "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ";
        let id = factory.floor[coord].machine.id;
        // if id < 24 {
        //   match greek_lo.char_indices().nth(id) {
        //     None => '!',
        //     Some((_, c)) => c,
        //   }
        // } else {
        ('0' as usize + id + 1) as u8 as char
        // }
      },
      CellKind::Belt => {
        if !dump && factory.floor[coord].belt.part.kind != PartKind::None {
          factory.floor[coord].belt.part.icon
        } else if factory.floor[coord].belt.meta.btype == BeltType::INVALID {
          match ( factory.floor[coord].port_u != Port::None, factory.floor[coord].port_r != Port::None, factory.floor[coord].port_d != Port::None, factory.floor[coord].port_l != Port::None ) {
            ( false, false, false, false ) => '!', // panic?
            ( false, true, false, false ) => '╺',
            ( false, false, true, false ) => '╻',
            ( false, false, false, true ) => '╸',
            ( false, true, true, false ) => '┏',
            ( false, true, false, true ) => '━',
            ( false, false, true, true ) => '┓',
            ( false, true, true, true ) => '┳',
            ( true, false, false, false ) => '╹',
            ( true, true, false, false ) => '┗',
            ( true, false, true, false ) => '┃',
            ( true, false, false, true ) => '┛',
            ( true, true, true, false ) => '┣',
            ( true, true, false,true ) => '╹',
            ( true, false, true, true ) => '┫',
            ( true, true, true, true ) => '╋',
          }
        } else {
          factory.floor[coord].belt.meta.cli_icon
        }
      },
    };

    let (x, y) = to_xy(coord);
    let is_m = factory.floor[coord].kind == CellKind::Machine;
    let main_coord = factory.floor[coord].machine.main_coord;
    let m_x = x - (main_coord % FLOOR_CELLS_W);
    let m_y = (coord / FLOOR_CELLS_W) - (main_coord / FLOOR_CELLS_W);
    let m_w = factory.floor[coord].machine.cell_width + 2; // TODO: fix once machine size is fixed
    let m_h = factory.floor[coord].machine.cell_height + 2; // TODO: fix once machine size is fixed
    let m_dl = main_coord + FLOOR_CELLS_W * (m_h - 1);
    let m_dr = m_dl + m_w - 1;

    if is_m && coord == main_coord {
      machine_count += 1;
      cell_params.push('m');
      cell_params.push((('0' as u8) + machine_count) as char);
      cell_params.push(' ');
      cell_params.push('=');
      cell_params.push(' ');
      cell_params.push(factory.floor[coord].machine.input_1_want.icon);
      cell_params.push(' ');
      cell_params.push(factory.floor[coord].machine.input_2_want.icon);
      cell_params.push(' ');
      cell_params.push(factory.floor[coord].machine.input_3_want.icon);
      cell_params.push(' ');
      cell_params.push('-');
      cell_params.push('>');
      cell_params.push(' ');
      cell_params.push(factory.floor[coord].machine.output_want.icon);
      cell_params.push(' ');
      cell_params.push('s');
      cell_params.push(':');
      for c in format!("{}", factory.floor[coord].machine.speed).as_bytes().iter() {
        cell_params.push(*c as char);
      }
      cell_params.push('\\');
      cell_params.push('n');
      cell_params.push('\\');
      cell_params.push('\n');
    }
    else if factory.floor[coord].kind == CellKind::Supply {
      supply_count += 1;
      cell_params.push('s');
      cell_params.push((('0' as u8) + supply_count) as char);
      cell_params.push(' ');
      cell_params.push('=');
      cell_params.push(' ');
      cell_params.push(factory.floor[coord].supply.gives.icon);
      cell_params.push(' ');
      cell_params.push('s');
      cell_params.push(':');
      for c in format!("{}", factory.floor[coord].machine.speed).as_bytes().iter() {
        cell_params.push(*c as char);
      }
      cell_params.push(' ');
      cell_params.push('c');
      cell_params.push(':');
      for c in format!("{}", factory.floor[coord].supply.cooldown).as_bytes().iter() {
        cell_params.push(*c as char);
      }
      cell_params.push('\\');
      cell_params.push('n');
      cell_params.push('\\');
      cell_params.push('\n');
    }
    else if factory.floor[coord].kind == CellKind::Demand {
      demand_count += 1;
      cell_params.push('d');
      cell_params.push((('0' as u8) + demand_count) as char);
      cell_params.push(' ');
      cell_params.push('=');
      cell_params.push(' ');
      cell_params.push(factory.floor[coord].demand.part.icon);
      cell_params.push('\\');
      cell_params.push('n');
      cell_params.push('\\');
      cell_params.push('\n');
    }

    line1.push(
      if coord == 0 { '┌' }
      else if x == 0 { '│' }
      else if coord < FLOOR_CELLS_W { '─' }
      else if coord == FLOOR_CELLS_WH-1 { '┘' }
      else if coord > FLOOR_CELLS_WH - FLOOR_CELLS_W && coord < FLOOR_CELLS_WH-1 { '─' }
      else if coord > 0 && coord != FLOOR_CELLS_W-1 && x == FLOOR_CELLS_W-1 { '│' }
      else if is_m && coord == main_coord { '┌' }
      else if is_m && coord < main_coord + FLOOR_CELLS_W { '─' }
      else if is_m && m_x == 0 { '│' }
      else { ' ' }
    );
    line1.push(match factory.floor[coord].port_u {
      Port::Inbound => 'v',
      Port::Outbound => '^',
      Port::None =>
        if coord < FLOOR_CELLS_W { '─' }
        else if coord > FLOOR_CELLS_WH - FLOOR_CELLS_W && coord < FLOOR_CELLS_WH-1 { '─' }
        else if is_m && coord < main_coord + FLOOR_CELLS_W { '─' }
        else { ' ' },
      Port::Unknown => '?', // panic?
    });
    line1.push(
      if coord == FLOOR_CELLS_W-1 { '┐' }
      else if x == FLOOR_CELLS_W-1 { '│' }
      else if coord < FLOOR_CELLS_W { '─' }
      else if coord == FLOOR_CELLS_WH-FLOOR_CELLS_W { '└' }
      else if coord > FLOOR_CELLS_WH - FLOOR_CELLS_W && coord < FLOOR_CELLS_WH-1 { '─' }
      else if coord > 0 && x == 0 { '│' }
      else if is_m && coord == main_coord + m_w - 1 { '┐' }
      else if is_m && coord < main_coord + FLOOR_CELLS_W { '─' }
      else if is_m && m_x == m_w - 1 { '│' }
      else { ' ' }
    );

    line2.push(match factory.floor[coord].port_l {
      Port::Inbound => '>',
      Port::Outbound => '<',
      Port::None =>
        if x == 0 { '│' }
        else if coord > 0 && coord != FLOOR_CELLS_W-1 && coord != FLOOR_CELLS_WH-1 && x == FLOOR_CELLS_W-1 { '│' }
        else if is_m && m_x == 0 { '│' }
        else { ' ' },
      Port::Unknown => '?',
    });
    line2.push(
      center_char
      // This would print the machine id in the middle. But honestly it just makes it hard to parse.
      // if is_m && (m_x != m_w/2 || m_y != m_h/2) { ' ' } // Print machine ID in "middle" of machine
      // else { center_char }
    );
    line2.push(match factory.floor[coord].port_r {
      Port::Inbound => '<',
      Port::Outbound => '>',
      Port::None =>
        if x == FLOOR_CELLS_W - 1 { '│' }
        else if coord > 0 && coord != FLOOR_CELLS_W-1 && coord != FLOOR_CELLS_WH-FLOOR_CELLS_W && x == 0 { '│' }
        else if is_m && m_x == m_w - 1 { '│' }
        else if (y == 0 || y == FLOOR_CELLS_H - 1) && factory.floor[coord].kind == CellKind::Supply { factory.floor[coord].supply.gives.icon }
        else if (y == 0 || y == FLOOR_CELLS_H - 1) && factory.floor[coord].kind == CellKind::Demand { factory.floor[coord].demand.part.icon }
        else { ' ' },
      Port::Unknown => '?',
    });

    line3.push(
      if coord == FLOOR_CELLS_W-1 { '┐' }
      else if coord == FLOOR_CELLS_WH - FLOOR_CELLS_W { '└' }
      else if coord >= FLOOR_CELLS_WH - FLOOR_CELLS_W { '─' }
      else if x == 0 { '│' }
      else if coord > 0 && coord < FLOOR_CELLS_W-1 { '─' }
      else if coord > 0 && coord != FLOOR_CELLS_WH-1 && x == FLOOR_CELLS_W-1 { '│' }
      else if is_m && coord == m_dl { '└' }
      else if is_m && coord >= m_dl { '─' }
      else if is_m && m_x == 0 { '│' }
      else { ' ' });
    line3.push(match factory.floor[coord].port_d {
      Port::Inbound => '^',
      Port::Outbound => 'v',
      Port::None =>
        if coord > 0 && coord < FLOOR_CELLS_W-1 { '─' }
        else if coord >= FLOOR_CELLS_WH - FLOOR_CELLS_W { '─' }
        else if (x == 0 || x == FLOOR_CELLS_W - 1) && factory.floor[coord].kind == CellKind::Supply { factory.floor[coord].supply.gives.icon }
        else if (x == 0 || x == FLOOR_CELLS_W - 1) && factory.floor[coord].kind == CellKind::Demand { factory.floor[coord].demand.part.icon }
        else if is_m && coord >= m_dl { '─' }
        else { ' ' },
      Port::Unknown => '?', // panic?
    });
    line3.push(
      if coord == 0 { '┌' }
      else if coord == FLOOR_CELLS_WH - 1 { '┘' }
      else if coord >= FLOOR_CELLS_WH - FLOOR_CELLS_W { '─' }
      else if x == FLOOR_CELLS_W - 1 { '│' }
      else if coord > 0 && coord < FLOOR_CELLS_W-1 { '─' }
      else if coord != FLOOR_CELLS_WH-FLOOR_CELLS_W && x == 0 { '│' }
      else if is_m && coord == m_dr { '┘' }
      else if is_m && coord >= m_dl { '─' }
      else if is_m && m_x == m_w - 1 { '│' }
      else { ' ' }
    );

    if x == FLOOR_CELLS_W - 1 {
      line1.push('\\');
      line1.push('n');
      line1.push('\\');
      line1.push('\n');
      line2.push('\\');
      line2.push('n');
      line2.push('\\');
      line2.push('\n');
      line3.push('\\');
      line3.push('n');
      line3.push('\\');
      line3.push('\n');

      out.push(line1.clone());
      out.push(line2.clone());
      out.push(line3.clone());

      line1 = vec!();
      line2 = vec!();
      line3 = vec!();
    }
  }

  out.push(cell_params.clone());

  for i in 0..factory.offers.len() {
    out.push(serialize_offer(&factory.offers[i]));
  }


  let flat: Vec<char> = out.into_iter().flatten().collect();
  return flat.iter().collect();
}

fn serialize_offer(offer: &Offer) -> Vec<char> {
  let mut s: Vec<char> = vec!();

  // TODO: encode modifiers too

  s.push('o');
  s.push(match offer.kind { CellKind::Machine => { 'm' }, CellKind::Supply => 's', CellKind::Demand => 'd', _ => panic!("offers are machines, suppliers, or demanders")});
  s.push(' ');
  s.push('=');
  s.push(' ');
  match offer.kind {
    CellKind::Machine => {
      s.push(offer.machine_input1);
      s.push(' ');
      s.push(offer.machine_input2);
      s.push(' ');
      s.push(offer.machine_input3);
      s.push(' ');
      s.push('-');
      s.push('>');
      s.push(' ');
      s.push(offer.machine_output);

      s.push(' ');
      s.push('s');
      s.push(':');
      for c in format!("{}", offer.speed).as_bytes().iter() {
        s.push(*c as char);
      }

      s.push(' ');
      s.push('d');
      s.push(':');
      for c in format!("{}", offer.cell_width).as_bytes().iter() {
        s.push(*c as char);
      }
      s.push('x');
      for c in format!("{}", offer.cell_height).as_bytes().iter() {
        s.push(*c as char);
      }
    },
    CellKind::Supply => {
      s.push(offer.supply_icon);

      s.push(' ');
      s.push('s');
      s.push(':');
      for c in format!("{}", offer.speed).as_bytes().iter() {
        s.push(*c as char);
      }

      s.push(' ');
      s.push('c');
      s.push(':');
      for c in format!("{}", offer.cooldown).as_bytes().iter() {
        s.push(*c as char);
      }
    }
    CellKind::Demand => {
      s.push(offer.demand_icon);
    }
    _ => panic!("offers are machines, suppliers, or demanders"),
  }
  s.push('\\');
  s.push('n');
  s.push('\\');
  s.push('\n');

  return s;
}
