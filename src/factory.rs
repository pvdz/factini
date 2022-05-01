use super::belt::*;
use super::cell::*;
use super::demand::*;
use super::floor::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::state::*;
use super::supply::*;

pub struct Factory {
  pub floor: Floor,
  pub suppliers: Vec<Supply>,
  pub demanders: Vec<Demand>,
}

pub fn create_factory(_options: &mut Options, _state: &mut State) -> Factory {
  let floor = test_floor();

  return Factory {
    floor,
    suppliers: vec!(
      Supply {x: 2, y: 0, ticks: 0, interval: 10000, part: part_none(0, 0), part_at: 0, last_part_out_at: 0, speed: 10000, stamp: Part { kind: PartKind::Flamingo, icon: 'f'}, part_price: -600},
      Supply {x: 4, y: 3, ticks: 0, interval: 10000, part: part_none(0, 0), part_at: 0, last_part_out_at: 0, speed: 10000, stamp: Part { kind: PartKind::Fire, icon: 'b'}, part_price: -800}
    ),
    demanders: vec!(
      Demand {x: 1, y: 4, part_kind: PartKind::Doll, part_price: 10000, trash_price: -1000},
    ),
  };
}

enum HasHole {
  None,
  Supply,
  Demand,
}

pub fn hole_in_the_wall(factory: &Factory, x: usize, y: usize, ifn: char, ifs: char, ifd: char) -> char {
  // My apologies for anyone doing perf ;)

  let suppliers = &factory.suppliers;
  let demanders = &factory.demanders;

  let mut has = HasHole::None;
  for supply in suppliers {
    if supply.x == x && supply.y == y {
      has = HasHole::Supply;
      break;
    }
  }

  if matches!(has, HasHole::None) {
    for demand in demanders {
      if demand.x == x && demand.y == y {
        has = HasHole::Demand;
        break;
      }
    }
  }

  return match has {
    HasHole::None => ifn,
    HasHole::Supply => ifs,
    HasHole::Demand => ifd,
  };
}
pub fn hu(factory: &Factory, x: usize) -> char {
  return hole_in_the_wall(factory, x, 0, '─', 'v', '^');
}
pub fn hr(factory: &Factory, y: usize) -> char {
  return hole_in_the_wall(factory, 4, y, '│', '<', '>');
}
pub fn hd(factory: &Factory, x: usize) -> char {
  return hole_in_the_wall(factory, x, 4, '─', '^', 'v');
}
pub fn hl(factory: &Factory, y: usize) -> char {
  return hole_in_the_wall(factory, 0, y, '│', '>', '<');
}

// belt entry/exits
fn br(factory: &Factory, x: usize, y: usize) -> char {
  match factory.floor.cells[x][y].belt.direction_r {
    BeltDirection::In => '<',
    BeltDirection::Out => '>',
    BeltDirection::None => '│',
  }
}
fn bl(factory: &Factory, x: usize, y: usize) -> char {
  match factory.floor.cells[x][y].belt.direction_l {
    BeltDirection::In => '>',
    BeltDirection::Out => '<',
    BeltDirection::None => '│',
  }
}
fn bu(factory: &Factory, x: usize, y: usize) -> char {
  match factory.floor.cells[x][y].belt.direction_u {
    BeltDirection::In => 'v',
    BeltDirection::Out => '^',
    BeltDirection::None => '─',
  }
}
fn bd(factory: &Factory, x: usize, y: usize) -> char {
  match factory.floor.cells[x][y].belt.direction_d {
    BeltDirection::In => '^',
    BeltDirection::Out => 'v',
    BeltDirection::None => '─',
  }
}

// cell segments
fn clu(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => ' ',
    CellKind::Machine => {
      // Print the first input part if there is more than one input
      if !matches!(cell.machine_input_2_want.kind, PartKind::None) {
        cell.machine_input_1_want.icon
      } else {
        ' '
      }
    }
  }
}
fn cu(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => {
      if !matches!(cell.segment_u_part.kind, PartKind::None) {
        cell.segment_u_part.icon
      } else {
        cell.belt.cli_u
      }
    }
    CellKind::Machine => {
      // Print the first input part if there is one input
      // Print the second input part if there are three inputs
      // Print nothing if there are two inputs
      if !matches!(cell.machine_input_3_want.kind, PartKind::None) {
        cell.machine_input_2_want.icon
      } else if matches!(cell.machine_input_2_want.kind, PartKind::None) {
        cell.machine_input_1_want.icon
      } else {
        ' '
      }
    }
  }
}
fn cru(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => ' ',
    CellKind::Machine => {
      // Print the third input part if there are three inputs
      // Print the second input part if there are two inputs
      // Print nothing if there is one input
      if !matches!(cell.machine_input_3_want.kind, PartKind::None) {
        cell.machine_input_3_want.icon
      } else if !matches!(cell.machine_input_2_want.kind, PartKind::None) {
        cell.machine_input_2_want.icon
      } else {
        ' '
      }
    }
  }
}
fn cl(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => {
      if !matches!(cell.segment_l_part.kind, PartKind::None) {
        cell.segment_l_part.icon
      } else {
        cell.belt.cli_l
      }
    }
    CellKind::Machine => {
      // Print corner if there are two or three inputs. Otherwise print nothing
      if !matches!(cell.machine_input_2_want.kind, PartKind::None) {
        if !matches!(cell.machine_input_2_have.kind, PartKind::None) {
          '┗'
        } else {
          '└'
        }
      } else {
        ' '
      }
    }
  }
}
fn cc(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => {
      if !matches!(cell.segment_c_part.kind, PartKind::None) {
        cell.segment_c_part.icon
      } else {
        cell.belt.cli_c
      }
    }
    CellKind::Machine => {
      // Print straight line if one input
      // Print T down if two inputs
      // Print cross if three inputs
      if !matches!(cell.machine_input_3_want.kind, PartKind::None) {
        if !matches!(cell.machine_input_1_have.kind, PartKind::None) && !matches!(cell.machine_input_2_have.kind, PartKind::None) && !matches!(cell.machine_input_3_have.kind, PartKind::None) {
          '╋'
        } else if !matches!(cell.machine_input_1_have.kind, PartKind::None) && !matches!(cell.machine_input_2_have.kind, PartKind::None) {
          '╃'
        } else if !matches!(cell.machine_input_1_have.kind, PartKind::None) && !matches!(cell.machine_input_3_have.kind, PartKind::None) {
          '┿'
        } else if !matches!(cell.machine_input_2_have.kind, PartKind::None) && !matches!(cell.machine_input_3_have.kind, PartKind::None) {
          '╄'
        } else if !matches!(cell.machine_input_1_have.kind, PartKind::None) {
          '┽'
        } else if !matches!(cell.machine_input_2_have.kind, PartKind::None) {
          '╀'
        } else if !matches!(cell.machine_input_3_have.kind, PartKind::None) {
          '┾'
        } else {
          '┼'
        }
      } else if !matches!(cell.machine_input_2_want.kind, PartKind::None) {
        if !matches!(cell.machine_input_1_have.kind, PartKind::None) && !matches!(cell.machine_input_2_have.kind, PartKind::None) {
          '┳'
        } else if !matches!(cell.machine_input_1_have.kind, PartKind::None) {
          '┭'
        } else if !matches!(cell.machine_input_2_have.kind, PartKind::None) {
          '┮'
        } else {
          '┬'
        }
      } else {
        if !matches!(cell.machine_input_1_have.kind, PartKind::None) {
          '┃'
        } else {
          '│'
        }
      }
    }
  }
}
fn cr(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => {
      if !matches!(cell.segment_r_part.kind, PartKind::None) {
        cell.segment_r_part.icon
      } else {
        cell.belt.cli_r
      }
    }
    CellKind::Machine => {
      // Print corner if there are two or three inputs. Otherwise print nothing
      if !matches!(cell.machine_input_2_want.kind, PartKind::None) {
        if (!matches!(cell.machine_input_3_want.kind, PartKind::None) && !matches!(cell.machine_input_3_have.kind, PartKind::None)) || (matches!(cell.machine_input_3_want.kind, PartKind::None) && !matches!(cell.machine_input_2_have.kind, PartKind::None)) {
          '┛'
        } else {
          '┘'
        }
      } else {
        ' '
      }
    }
  }
}
fn cdl(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => ' ',
    CellKind::Machine => {
      ' '
    }
  }
}
fn cd(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => {
      if !matches!(cell.segment_d_part.kind, PartKind::None) {
        cell.segment_d_part.icon
      } else {
        cell.belt.cli_d
      }
    }
    CellKind::Machine => {
      cell.machine_output_want.icon
    }
  }
}
fn cdr(factory: &Factory, x: usize, y: usize) -> char {
  let cell = &factory.floor.cells[x][y];
  match cell.kind {
    CellKind::Empty => ' ',
    CellKind::Belt => ' ',
    CellKind::Machine => {
      ' '
    }
  }
}


pub fn serialize_cli(factory: &Factory) -> String {
  let floor = &factory.floor;
  // floor.cells[0][0].belt
  let v: Vec<String> = vec!(
    format!("┌──{}────{}────{}────{}────{}──┐", hu(factory, 0), hu(factory, 1), hu(factory, 2), hu(factory, 3), hu(factory, 4)).to_string(),
    format!("│┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐│", bu(factory, 0, 0), bu(factory, 1, 0), bu(factory, 2, 0), bu(factory, 3, 0), bu(factory, 4, 0)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", clu(factory, 0, 0), cu(factory, 0, 0), cru(factory, 0, 0), clu(factory, 1, 0), cu(factory, 1, 0), cru(factory, 1, 0), clu(factory, 2, 0), cu(factory, 2, 0), cru(factory, 2, 0), clu(factory, 3, 0), cu(factory, 3, 0), cru(factory, 3, 0), clu(factory, 4, 0), cu(factory, 4, 0), cru(factory, 4, 0)).to_string(),
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", hl(factory, 0), bl(factory, 0, 0), cl(factory, 0, 0), cc(factory, 0, 0), cr(factory, 0, 0), br(factory, 0, 0), bl(factory, 1, 0), cl(factory, 1, 0), cc(factory, 1, 0), cr(factory, 1, 0), br(factory, 1, 0), bl(factory, 2, 0), cl(factory, 2, 0), cc(factory, 2, 0), cr(factory, 2, 0), br(factory, 2, 0), bl(factory, 3, 0), cl(factory, 3, 0), cc(factory, 3, 0), cr(factory, 3, 0), br(factory, 3, 0), bl(factory, 4, 0), cl(factory, 4, 0), cc(factory, 4, 0), cr(factory, 4, 0), br(factory, 4, 0), hr(factory, 0)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", cdl(factory, 0, 0), cd(factory, 0, 0), cdr(factory, 0, 0), cdl(factory, 1, 0), cd(factory, 1, 0), cdr(factory, 1, 0), cdl(factory, 2, 0), cd(factory, 2, 0), cdr(factory, 2, 0), cdl(factory, 3, 0), cd(factory, 3, 0), cdr(factory, 3, 0), cdl(factory, 4, 0), cd(factory, 4, 0), cdr(factory, 4, 0)).to_string(),
    format!("│└─{}─┘└─{}─┘└─{}─┘└─{}─┘└─{}─┘│", bd(factory, 0, 0), bd(factory, 1, 0), bd(factory, 2, 0), bd(factory, 3, 0), bd(factory, 4, 0)).to_string(),
    format!("│┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐│", bu(factory, 0, 1), bu(factory, 1, 1), bu(factory, 2, 1), bu(factory, 3, 1), bu(factory, 4, 1)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", clu(factory, 0, 1), cu(factory, 0, 1), cru(factory, 0, 1), clu(factory, 1, 1), cu(factory, 1, 1), cru(factory, 1, 1), clu(factory, 2, 1), cu(factory, 2, 1), cru(factory, 2, 1), clu(factory, 3, 1), cu(factory, 3, 1), cru(factory, 3, 1), clu(factory, 4, 1), cu(factory, 4, 1), cru(factory, 4, 1)).to_string(),
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", hl(factory, 1), bl(factory, 0, 1), cl(factory, 0, 1), cc(factory, 0, 1), cr(factory, 0, 1), br(factory, 0, 1), bl(factory, 1, 1), cl(factory, 1, 1), cc(factory, 1, 1), cr(factory, 1, 1), br(factory, 1, 1), bl(factory, 2, 1), cl(factory, 2, 1), cc(factory, 2, 1), cr(factory, 2, 1), br(factory, 2, 1), bl(factory, 3, 1), cl(factory, 3, 1), cc(factory, 3, 1), cr(factory, 3, 1), br(factory, 3, 1), bl(factory, 4, 1), cl(factory, 4, 1), cc(factory, 4, 1), cr(factory, 4, 1), br(factory, 4, 1), hr(factory, 1)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", cdl(factory, 0, 1), cd(factory, 0, 1), cdr(factory, 0, 1), cdl(factory, 1, 1), cd(factory, 1, 1), cdr(factory, 1, 1), cdl(factory, 2, 1), cd(factory, 2, 1), cdr(factory, 2, 1), cdl(factory, 3, 1), cd(factory, 3, 1), cdr(factory, 3, 1), cdl(factory, 4, 1), cd(factory, 4, 1), cdr(factory, 4, 1)).to_string(),
    format!("│└─{}─┘└─{}─┘└─{}─┘└─{}─┘└─{}─┘│", bd(factory, 0, 1), bd(factory, 1, 1), bd(factory, 2, 1), bd(factory, 3, 1), bd(factory, 4, 1)).to_string(),
    format!("│┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐│", bu(factory, 0, 2), bu(factory, 1, 2), bu(factory, 2, 2), bu(factory, 3, 2), bu(factory, 4, 2)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", clu(factory, 0, 2), cu(factory, 0, 2), cru(factory, 0, 2), clu(factory, 1, 2), cu(factory, 1, 2), cru(factory, 1, 2), clu(factory, 2, 2), cu(factory, 2, 2), cru(factory, 2, 2), clu(factory, 3, 2), cu(factory, 3, 2), cru(factory, 3, 2), clu(factory, 4, 2), cu(factory, 4, 2), cru(factory, 4, 2)).to_string(),
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", hl(factory, 2), bl(factory, 0, 2), cl(factory, 0, 2), cc(factory, 0, 2), cr(factory, 0, 2), br(factory, 0, 2), bl(factory, 1, 2), cl(factory, 1, 2), cc(factory, 1, 2), cr(factory, 1, 2), br(factory, 1, 2), bl(factory, 2, 2), cl(factory, 2, 2), cc(factory, 2, 2), cr(factory, 2, 2), br(factory, 2, 2), bl(factory, 3, 2), cl(factory, 3, 2), cc(factory, 3, 2), cr(factory, 3, 2), br(factory, 3, 2), bl(factory, 4, 2), cl(factory, 4, 2), cc(factory, 4, 2), cr(factory, 4, 2), br(factory, 4, 2), hr(factory, 2)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", cdl(factory, 0, 2), cd(factory, 0, 2), cdr(factory, 0, 2), cdl(factory, 1, 2), cd(factory, 1, 2), cdr(factory, 1, 2), cdl(factory, 2, 2), cd(factory, 2, 2), cdr(factory, 2, 2), cdl(factory, 3, 2), cd(factory, 3, 2), cdr(factory, 3, 2), cdl(factory, 4, 2), cd(factory, 4, 2), cdr(factory, 4, 2)).to_string(),
    format!("│└─{}─┘└─{}─┘└─{}─┘└─{}─┘└─{}─┘│", bd(factory, 0, 2), bd(factory, 1, 2), bd(factory, 2, 2), bd(factory, 3, 2), bd(factory, 4, 2)).to_string(),
    format!("│┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐│", bu(factory, 0, 3), bu(factory, 1, 3), bu(factory, 2, 3), bu(factory, 3, 3), bu(factory, 4, 3)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", clu(factory, 0, 3), cu(factory, 0, 3), cru(factory, 0, 3), clu(factory, 1, 3), cu(factory, 1, 3), cru(factory, 1, 3), clu(factory, 2, 3), cu(factory, 2, 3), cru(factory, 2, 3), clu(factory, 3, 3), cu(factory, 3, 3), cru(factory, 3, 3), clu(factory, 4, 3), cu(factory, 4, 3), cru(factory, 4, 3)).to_string(),
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", hl(factory, 3), bl(factory, 0, 3), cl(factory, 0, 3), cc(factory, 0, 3), cr(factory, 0, 3), br(factory, 0, 3), bl(factory, 1, 3), cl(factory, 1, 3), cc(factory, 1, 3), cr(factory, 1, 3), br(factory, 1, 3), bl(factory, 2, 3), cl(factory, 2, 3), cc(factory, 2, 3), cr(factory, 2, 3), br(factory, 2, 3), bl(factory, 3, 3), cl(factory, 3, 3), cc(factory, 3, 3), cr(factory, 3, 3), br(factory, 3, 3), bl(factory, 4, 3), cl(factory, 4, 3), cc(factory, 4, 3), cr(factory, 4, 3), br(factory, 4, 3), hr(factory, 3)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", cdl(factory, 0, 3), cd(factory, 0, 3), cdr(factory, 0, 3), cdl(factory, 1, 3), cd(factory, 1, 3), cdr(factory, 1, 3), cdl(factory, 2, 3), cd(factory, 2, 3), cdr(factory, 2, 3), cdl(factory, 3, 3), cd(factory, 3, 3), cdr(factory, 3, 3), cdl(factory, 4, 3), cd(factory, 4, 3), cdr(factory, 4, 3)).to_string(),
    format!("│└─{}─┘└─{}─┘└─{}─┘└─{}─┘└─{}─┘│", bd(factory, 0, 3), bd(factory, 1, 3), bd(factory, 2, 3), bd(factory, 3, 3), bd(factory, 4, 3)).to_string(),
    format!("│┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐┌─{}─┐│", bu(factory, 0, 4), bu(factory, 1, 4), bu(factory, 2, 4), bu(factory, 3, 4), bu(factory, 4, 4)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", clu(factory, 0, 4), cu(factory, 0, 4), cru(factory, 0, 4), clu(factory, 1, 4), cu(factory, 1, 4), cru(factory, 1, 4), clu(factory, 2, 4), cu(factory, 2, 4), cru(factory, 2, 4), clu(factory, 3, 4), cu(factory, 3, 4), cru(factory, 3, 4), clu(factory, 4, 4), cu(factory, 4, 4), cru(factory, 4, 4)).to_string(),
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", hl(factory, 4), bl(factory, 0, 4), cl(factory, 0, 4), cc(factory, 0, 4), cr(factory, 0, 4), br(factory, 0, 4), bl(factory, 1, 4), cl(factory, 1, 4), cc(factory, 1, 4), cr(factory, 1, 4), br(factory, 1, 4), bl(factory, 2, 4), cl(factory, 2, 4), cc(factory, 2, 4), cr(factory, 2, 4), br(factory, 2, 4), bl(factory, 3, 4), cl(factory, 3, 4), cc(factory, 3, 4), cr(factory, 3, 4), br(factory, 3, 4), bl(factory, 4, 4), cl(factory, 4, 4), cc(factory, 4, 4), cr(factory, 4, 4), br(factory, 4, 4), hr(factory, 4)).to_string(),
    format!("││{}{}{}││{}{}{}││{}{}{}││{}{}{}││{}{}{}││", cdl(factory, 0, 4), cd(factory, 0, 4), cdr(factory, 0, 4), cdl(factory, 1, 4), cd(factory, 1, 4), cdr(factory, 1, 4), cdl(factory, 2, 4), cd(factory, 2, 4), cdr(factory, 2, 4), cdl(factory, 3, 4), cd(factory, 3, 4), cdr(factory, 3, 4), cdl(factory, 4, 4), cd(factory, 4, 4), cdr(factory, 4, 4)).to_string(),
    format!("│└─{}─┘└─{}─┘└─{}─┘└─{}─┘└─{}─┘│", bd(factory, 0, 4), bd(factory, 1, 4), bd(factory, 2, 4), bd(factory, 3, 4), bd(factory, 4, 4)).to_string(),
    format!("└──{}────{}────{}────{}────{}──┘", hd(factory, 0), hd(factory, 1), hd(factory, 2), hd(factory, 3), hd(factory, 4)).to_string(),
  );

  return v.into_iter().collect::<Vec<String>>().join("\n");
}
