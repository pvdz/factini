use super::config::*;
use super::utils::*;

#[derive(Clone, Debug)]
pub struct Part {
  pub kind: PartKind,
  pub icon: char,
}

// Note: 0 is hardcoded to None, 1 is hardcoded to Trash. The rest is user defined.
// #[derive(Clone, Copy, Debug, PartialEq)]
pub type PartKind = usize;

pub const PARTKIND_NONE: PartKind = 0;
pub const PARTKIND_TRASH: PartKind = 1;

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

pub fn part_from_part_index(config: &Config, part_index: usize) -> Part {
  let node = &config.nodes[part_index];
  return Part {
    kind: node.index,
    icon: node.icon,
  }
}

// h e -> W
// i p j -> D
// D W -> C
// n -> Q
// k Q -> l
// W l -> o
// C o -> K

// blue-dirt + empty-potion -> blue-potion
// rope + paper -> white-book
// white-book + blue-potion -> blue-book
// white-dirt -> white-ingot
// wood + white-ingot -> shield
// blue-potion + shield -> blue-shield
// blue-book + blue-shield -> book-shield

// "achievements" to finish. like 100 bottles of blue in one day
// achievements unlock new achievements with new craftables
// should we show craftables in a grid to the right, like the unlock tree
// could use this to show the craft config to have a machine create a certain thing
// (should we just show that inside the machine UI, anyways?
// Offers become machines and special buildings etc

pub fn part_icon_to_kind(config: &Config, c: char) -> PartKind {
  // The value of a PartKind is really just the index on the nodes array of the config
  // TODO: figure out why the .or() is relevant. if you set it to 1 then it'll mess up (use debug tools to print parts)
  return *config.node_name_to_index.get(c.to_string().as_str()).or(Some(&0)).unwrap() as PartKind;
}
pub fn part_kind_to_icon(config: &Config, kind: PartKind) -> char {
  return config.nodes[kind].icon;
}
pub fn part_to_sprite_coord_from_config(config: &Config, kind: PartKind) -> ( f64, f64, f64, f64, &web_sys::HtmlImageElement ) {
  assert!((kind as usize) < config.nodes.len(), "part kind should be a node index: {} < {}", kind, config.nodes.len());
  let node = &config.nodes[kind];
  return ( node.x as f64, node.y as f64, node.w as f64, node.h as f64, &config.sprite_cache_canvas[node.file_canvas_cache_index] );
}
