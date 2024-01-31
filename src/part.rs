use super::config::*;
use super::factory::*;
use super::options::*;
use super::utils::*;
use super::log;

#[derive(Clone, Debug)]
pub struct Part {
  pub kind: PartKind,
  pub icon: char,
}

// Note: This maps to config.nodes[index]. Just a visual alias to differentiate from usize (rust has no opaque types yet)
// #[derive(Clone, Copy, Debug, PartialEq)]
pub type PartKind = usize;

pub fn part_none(config: &Config) -> Part {
  return Part {
    kind: part_icon_to_kind(config, ' '),
    icon: ' ',
  }
}

pub fn part_c(config: &Config, icon: char) -> Part {
  return Part {
    kind: part_icon_to_kind(config, icon),
    icon,
  }
}

pub fn part_from_node(node: &ConfigNode) -> Part {
  return Part {
    kind: node.index,
    icon: node.icon,
  }
}

pub fn part_from_part_kind(config: &Config, part_kind: PartKind) -> Part {
  let node = &config.nodes[part_kind];
  return Part {
    kind: node.index,
    icon: node.icon,
  }
}

pub fn part_icon_to_kind(config: &Config, c: char) -> PartKind {
  // The value of a PartKind is really just the index on the nodes array of the config
  // TODO: figure out why the .or() is relevant. if you set it to 1 then it'll mess up (use debug tools to print parts)
  return *config.node_name_to_index.get(c.to_string().as_str()).or(Some(&0)).unwrap() as PartKind;
}

pub fn part_kind_to_icon(config: &Config, kind: PartKind) -> char {
  return config.nodes[kind].icon;
}

pub fn part_kind_to_exportable_icon(config: &Config, kind: PartKind) -> String {
  let icon = part_kind_to_icon(config, kind);
  if (icon >= 'a' && icon <= 'z') || (icon >= 'A' && icon <= 'Z') || (icon >= '0' && icon <= '9') { return format!("{}", icon); }
  return format!("&{}", icon as u8);
}

pub fn part_to_sprite_coord_from_config<'x>(config: &'x Config, options: &Options, part_kind: PartKind) -> (f64, f64, f64, f64, &'x web_sys::HtmlImageElement ) {
  assert!(part_kind < config.nodes.len(), "part kind should be a node index: {} < {}", part_kind, config.nodes.len());

  return config_get_sprite_details(config, options, part_kind, 0, 0);
}

pub fn part_kind_to_visible_woop_index(config: &Config, factory: &Factory, part_kind: PartKind) -> Option<usize> {
  for i in 0..factory.available_woops.len() {
    if factory.available_woops[i].0 == part_kind {
      return Some(i);
    }
  }

  log!("part_kind_to_visible_woop_index: could not find {} ({:?}) in {:?}", part_kind, config.nodes[part_kind].raw_name, factory.available_woops);

  return None;
}

pub fn part_kind_to_visible_atom_index(config: &Config, factory: &Factory, part_kind: PartKind) -> Option<usize> {
  for i in 0..factory.available_atoms.len() {
    if factory.available_atoms[i].0 == part_kind {
      return Some(i);
    }
  }

  log!("part_kind_to_visible_atom_index: could not find {} ({:?}) in {:?}", part_kind, config.nodes[part_kind].raw_name, factory.available_atoms);

  return None;
}

pub fn is_atom(config: &Config, part: PartKind) -> bool {
  return config.nodes[part].pattern_unique_kinds.len() == 0;
}

pub fn is_woop(config: &Config, part: PartKind) -> bool {
  return config.nodes[part].pattern_unique_kinds.len() > 0;
}
