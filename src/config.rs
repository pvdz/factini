use std::collections::HashMap;
use std::collections::HashSet;

use super::options::*;
use super::part::*;
use super::state::*;
use super::utils::*;

// These index directly to the config.nodes vec
pub const CONFIG_NODE_SUPPLY_UP: usize = 2;
pub const CONFIG_NODE_SUPPLY_RIGHT: usize = 3;
pub const CONFIG_NODE_SUPPLY_DOWN: usize = 4;
pub const CONFIG_NODE_SUPPLY_LEFT: usize = 5;
pub const CONFIG_NODE_DEMAND_UP: usize = 6;
pub const CONFIG_NODE_DEMAND_RIGHT: usize = 7;
pub const CONFIG_NODE_DEMAND_DOWN: usize = 8;
pub const CONFIG_NODE_DEMAND_LEFT: usize = 9;
pub const CONFIG_NODE_DOCK_UP: usize = 10;
pub const CONFIG_NODE_DOCK_RIGHT: usize = 11;
pub const CONFIG_NODE_DOCK_DOWN: usize = 12;
pub const CONFIG_NODE_DOCK_LEFT: usize = 13;
pub const CONFIG_NODE_MACHINE_1X1: usize = 13;
pub const CONFIG_NODE_MACHINE_2X2: usize = 14;
pub const CONFIG_NODE_MACHINE_3X3: usize = 15;

#[derive(Debug)]
pub struct Config {
  pub nodes: Vec<ConfigNode>,
  pub quest_nodes: Vec<usize>, // maps to nodes vec
  pub part_nodes: Vec<usize>, // maps to nodes vec
  pub node_name_to_index: HashMap<String, usize>,
  pub sprite_cache_lookup: HashMap<String, usize>, // indexes into sprite_cache_canvas
  pub sprite_cache_order: Vec<String>, // srcs by index.
  pub sprite_cache_canvas: Vec<web_sys::HtmlImageElement>,
}

#[derive(Debug)]
pub struct ConfigNode {
  pub index: usize, // own index in the config.nodes vec
  pub kind: ConfigNodeKind,
  pub name: String,
  pub raw_name: String,

  // Quest
  pub unlocks_after_by_name: Vec<String>, // Fully qualified name. Becomes available when these quests are finished.
  pub unlocks_after_by_index: Vec<usize>, // Becomes available when these quests are finished
  pub unlocks_todo_by_index: Vec<usize>, // Which quests still need to be unlocked before this one unlocks?
  pub starting_part_by_name: Vec<String>, // Fully qualified name. These parts are available when this quest becomes available
  pub starting_part_by_index: Vec<usize>, // These parts are available when this quest becomes available
  pub production_target_by_name: Vec<(u32, String)>, // Fully qualified name. count,name pairs, you need this to finish the quest
  pub production_target_by_index: Vec<(u32, usize)>, // count,index pairs, you need this to finish the quest

  // Part
  pub pattern_by_index: Vec<PartKind>, // Machine pattern that generates this part (part_index)
  pub pattern_by_name: Vec<String>, // Machine pattern that generates this part (actual names)
  pub pattern_unique_icons: Vec<PartKind>, // String of unique non-empty part icons. We can use this to quickly find machines that have received these parts.
  pub icon: char, // Single (unique) character that also represents this part internally
  pub file: String, // Sprite image location
  pub file_canvas_cache_index: usize, // The canvas with the sprite image loaded
  // Coord of the part on the sprite
  pub x: f64,
  pub y: f64,
  pub w: f64,
  pub h: f64,
  // Mostly for debugging
  pub current_state: ConfigNodeState,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConfigNodeKind {
  Part,
  Quest,
  Supply,
  Demand,
  Dock,
  Machine,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConfigNodeState {
  Waiting, // Waiting for eligibility to be active

  // Quests
  LFG, // Ready to be added to be given to the user but not yet given to the user
  Active, // Currently given to the user, enabled to be finished
  Finished, // Already finished

  // Parts
  Available, // Part that can be used
}

pub fn config_get_available_parts(config: &Config) -> Vec<PartKind>{
  let mut parts = vec!(
    PARTKIND_TRASH // for testing/debugging?
  );
  config.part_nodes.iter().for_each(|&part_index| {
    if config.nodes[part_index].current_state == ConfigNodeState::Available {
      parts.push(part_index);
    }
  });
  return parts;
}

pub fn parse_fmd(options: &Options, config: String) -> Config {
  // Parse Fake MD config
  log(format!("parse_fmd(options.print_fmd_trace={})", options.print_fmd_trace));

  // This pseudo-md config file is a series of node definitions.
  // Names are case insensitive.
  // Each node starts with an md header (`# Abc_Def`) where the name is split between
  // a type and a name, separated by one underscore. (The underscore is basically to support
  // double-click-to-select the whole name, otherwise I would have picked a colon.)
  // The body of a node is zero or more list elements which are each key value pairs, separated
  // by a colon.
  // There are a few fields currently. For example: `parents` and `requires`.
  // - `parents`: a comma separated list of zero or more (fully qualified) parent node names.
  // - `requirements`: a comma separated list of zero or more, spacing separated counts and
  //   (fully qualified) node name pairs.
  // - `pattern`: input parts for the machine to generate this part. spacing separated. Underscore
  //   for empty spot. Single line. (Parsing multi-line pattern is just too much work.) Can use
  //   the char icon name instead of the full qualified name.
  // - `char`: CLI printing code. In some places, the char icon can be used instead of full name.
  // - `file`: source of the sprite image file
  // - `x`: offset of sprite for this part
  // - `y`: offset of sprite for this part
  // - `w`: width of the sprite for this part
  // - `h`: heigth of the sprite for this part
  // All fields are optional and default to an empty list, string, or 0.
  // All spacing is optional.
  // Lines that do not start with a `#` or `-` are ignored.
  // Only the space is considered for spacing, not tabs.
  // The idea is to create a tree/graph structure this way without too many syntactical limits.
  // Parsing can be simple on a high level, albeit is less efficient. Which should be fine.
  // The internal structure should be dumpable to a "dot" file, which can be consumed by open source
  // tooling to generate a proper graph.
  // Example:
  // # Quest_Foo
  // - parents: Quest_A, Quest_B
  // - requires: 10 Part_A, 200 Part_B

  let mut nodes: Vec<ConfigNode> = vec!(
    ConfigNode {
      index: PARTKIND_NONE, // 0
      kind: ConfigNodeKind::Part,
      name: "None".to_string(),
      raw_name: "Part_None".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      unlocks_todo_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern_by_index: vec!(),
      pattern_by_name: vec!(),
      pattern_unique_icons: vec!(),
      icon: ' ',
      file: "".to_string(),
      file_canvas_cache_index: 0,
      x: 0.0,
      y: 0.0,
      w: 0.0,
      h: 0.0,
      current_state: ConfigNodeState::Available,
    },
    ConfigNode {
      index: PARTKIND_TRASH, // 1
      kind: ConfigNodeKind::Part,
      name: "Trash".to_string(),
      raw_name: "Part_None".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      unlocks_todo_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      pattern_unique_icons: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern_by_index: vec!(),
      pattern_by_name: vec!(),
      icon: 't',
      file: "".to_string(),
      file_canvas_cache_index: 0,
      x: 0.0,
      y: 0.0,
      w: 0.0,
      h: 0.0,
      current_state: ConfigNodeState::Available,
    },
    ConfigNode {
      index: CONFIG_NODE_SUPPLY_UP,
      kind: ConfigNodeKind::Supply,
      name: "Supply".to_string(),
      raw_name: "Supply_Up".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      unlocks_todo_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      pattern_unique_icons: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern_by_index: vec!(),
      pattern_by_name: vec!(),
      icon: '?',
      file: "./img/supply.png".to_string(),
      file_canvas_cache_index: 0,
      x: 0.0,
      y: 0.0,
      w: 32.0,
      h: 32.0,
      current_state: ConfigNodeState::Available,
    },
    ConfigNode {
      index: CONFIG_NODE_SUPPLY_RIGHT,
      kind: ConfigNodeKind::Supply,
      name: "Supply".to_string(),
      raw_name: "Supply_Up".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      unlocks_todo_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      pattern_unique_icons: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern_by_index: vec!(),
      pattern_by_name: vec!(),
      icon: '?',
      file: "./img/supply.png".to_string(),
      file_canvas_cache_index: 0,
      x: 96.0,
      y: 0.0,
      w: 32.0,
      h: 32.0,
      current_state: ConfigNodeState::Available,
    },
    ConfigNode {
      index: CONFIG_NODE_SUPPLY_DOWN,
      kind: ConfigNodeKind::Supply,
      name: "Supply".to_string(),
      raw_name: "Supply_Up".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      unlocks_todo_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      pattern_unique_icons: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern_by_index: vec!(),
      pattern_by_name: vec!(),
      icon: '?',
      file: "./img/supply.png".to_string(),
      file_canvas_cache_index: 0,
      x: 64.0,
      y: 0.0,
      w: 32.0,
      h: 32.0,
      current_state: ConfigNodeState::Available,
    },
    ConfigNode {
      index: CONFIG_NODE_SUPPLY_LEFT,
      kind: ConfigNodeKind::Supply,
      name: "Supply".to_string(),
      raw_name: "Supply_Up".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      unlocks_todo_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      pattern_unique_icons: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern_by_index: vec!(),
      pattern_by_name: vec!(),
      icon: '?',
      file: "./img/supply.png".to_string(),
      file_canvas_cache_index: 0,
      x: 32.0,
      y: 0.0,
      w: 32.0,
      h: 32.0,
      current_state: ConfigNodeState::Available,
    },
    ConfigNode {
      index: CONFIG_NODE_DEMAND_UP,
      kind: ConfigNodeKind::Demand,
      name: "Demand".to_string(),
      raw_name: "Demand_Up".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      unlocks_todo_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      pattern_unique_icons: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern_by_index: vec!(),
      pattern_by_name: vec!(),
      icon: '?',
      file: "./img/demand.png".to_string(),
      file_canvas_cache_index: 0,
      x: 0.0,
      y: 0.0,
      w: 32.0,
      h: 32.0,
      current_state: ConfigNodeState::Available,
    },
    ConfigNode {
      index: CONFIG_NODE_DEMAND_RIGHT,
      kind: ConfigNodeKind::Demand,
      name: "Demand".to_string(),
      raw_name: "Demand_Right".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      unlocks_todo_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      pattern_unique_icons: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern_by_index: vec!(),
      pattern_by_name: vec!(),
      icon: '?',
      file: "./img/demand.png".to_string(),
      file_canvas_cache_index: 0,
      x: 96.0,
      y: 0.0,
      w: 32.0,
      h: 32.0,
      current_state: ConfigNodeState::Available,
    },
    ConfigNode {
      index: CONFIG_NODE_DEMAND_DOWN,
      kind: ConfigNodeKind::Demand,
      name: "Demand".to_string(),
      raw_name: "Demand_Down".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      unlocks_todo_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      pattern_unique_icons: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern_by_index: vec!(),
      pattern_by_name: vec!(),
      icon: '?',
      file: "./img/demand.png".to_string(),
      file_canvas_cache_index: 0,
      x: 64.0,
      y: 0.0,
      w: 32.0,
      h: 32.0,
      current_state: ConfigNodeState::Available,
    },
    ConfigNode {
      index: CONFIG_NODE_DEMAND_LEFT,
      kind: ConfigNodeKind::Demand,
      name: "Demand".to_string(),
      raw_name: "Demand_Left".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      unlocks_todo_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      pattern_unique_icons: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern_by_index: vec!(),
      pattern_by_name: vec!(),
      icon: '?',
      file: "./img/demand.png".to_string(),
      file_canvas_cache_index: 0,
      x: 32.0,
      y: 0.0,
      w: 32.0,
      h: 32.0,
      current_state: ConfigNodeState::Available,
    },
    ConfigNode {
      index: CONFIG_NODE_DOCK_UP,
      kind: ConfigNodeKind::Dock,
      name: "Dock".to_string(),
      raw_name: "Dock_Up".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      unlocks_todo_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      pattern_unique_icons: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern_by_index: vec!(),
      pattern_by_name: vec!(),
      icon: '?',
      file: "./img/dock.png".to_string(),
      file_canvas_cache_index: 0,
      x: 0.0,
      y: 0.0,
      w: 64.0,
      h: 64.0,
      current_state: ConfigNodeState::Available,
    },
    ConfigNode {
      index: CONFIG_NODE_DOCK_RIGHT,
      kind: ConfigNodeKind::Dock,
      name: "Dock".to_string(),
      raw_name: "Dock_Right".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      unlocks_todo_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      pattern_unique_icons: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern_by_index: vec!(),
      pattern_by_name: vec!(),
      icon: '?',
      file: "./img/dock.png".to_string(),
      file_canvas_cache_index: 0,
      x: 0.0,
      y: 0.0,
      w: 64.0,
      h: 64.0,
      current_state: ConfigNodeState::Available,
    },
    ConfigNode {
      index: CONFIG_NODE_DOCK_DOWN,
      kind: ConfigNodeKind::Dock,
      name: "Dock".to_string(),
      raw_name: "Dock_Down".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      unlocks_todo_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      pattern_unique_icons: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern_by_index: vec!(),
      pattern_by_name: vec!(),
      icon: '?',
      file: "./img/dock.png".to_string(),
      file_canvas_cache_index: 0,
      x: 0.0,
      y: 0.0,
      w: 64.0,
      h: 64.0,
      current_state: ConfigNodeState::Available,
    },
    ConfigNode {
      index: CONFIG_NODE_DOCK_LEFT,
      kind: ConfigNodeKind::Dock,
      name: "Dock".to_string(),
      raw_name: "Dock_Left".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      unlocks_todo_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      pattern_unique_icons: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern_by_index: vec!(),
      pattern_by_name: vec!(),
      icon: '?',
      file: "./img/dock.png".to_string(),
      file_canvas_cache_index: 0,
      x: 0.0,
      y: 0.0,
      w: 64.0,
      h: 64.0,
      current_state: ConfigNodeState::Available,
    },
    ConfigNode {
      index: CONFIG_NODE_MACHINE_1x1,
      kind: ConfigNodeKind::Machine,
      name: "Machine".to_string(),
      raw_name: "Machine_1x1".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      unlocks_todo_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      pattern_unique_icons: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern_by_index: vec!(),
      pattern_by_name: vec!(),
      icon: '?',
      file: "./img/machine_1_1.png".to_string(),
      file_canvas_cache_index: 0,
      // this is for the output part icon
      x: 5.0,
      y: 5.0,
      w: 5.0,
      h: 5.0,
      current_state: ConfigNodeState::Available,
    }
  ),
  ConfigNode {
    index: CONFIG_NODE_MACHINE_2x2,
    kind: ConfigNodeKind::Machine,
    name: "Machine".to_string(),
    raw_name: "Machine_1x1".to_string(),
    unlocks_after_by_name: vec!(),
    unlocks_after_by_index: vec!(),
    unlocks_todo_by_index: vec!(),
    starting_part_by_name: vec!(),
    starting_part_by_index: vec!(),
    pattern_unique_icons: vec!(),
    production_target_by_name: vec!(),
    production_target_by_index: vec!(),
    pattern_by_index: vec!(),
    pattern_by_name: vec!(),
    icon: '?',
    file: "./img/machine_2_2.png".to_string(),
    file_canvas_cache_index: 0,
    // this is for the output part icon
    x: 5.0,
    y: 5.0,
    w: 5.0,
    h: 5.0,
    current_state: ConfigNodeState::Available,
  }
  ),
  ConfigNode {
    index: CONFIG_NODE_MACHINE_3x3,
    kind: ConfigNodeKind::Machine,
    name: "Machine".to_string(),
    raw_name: "Machine_3x3".to_string(),
    unlocks_after_by_name: vec!(),
    unlocks_after_by_index: vec!(),
    unlocks_todo_by_index: vec!(),
    starting_part_by_name: vec!(),
    starting_part_by_index: vec!(),
    pattern_unique_icons: vec!(),
    production_target_by_name: vec!(),
    production_target_by_index: vec!(),
    pattern_by_index: vec!(),
    pattern_by_name: vec!(),
    icon: '?',
    file: "./img/machine_1_1.png".to_string(),
    file_canvas_cache_index: 0,
    // this is for the output part icon
    x: 0.0,
    y: 0.0,
    w: 30.0,
    h: 30.0,
    current_state: ConfigNodeState::Available,
  }
  );
  assert!(nodes.iter().enumerate().all(|(i, n)| n.index == i), "config.nodes should start with a set of nodes where each index is set correctly");
  // Indirect references to nodes. Can't share direct references so these index the nodes vec.
  let mut quest_nodes: Vec<usize> = vec!();
  let mut part_nodes: Vec<usize> = vec!(0, 1);

  let mut seen_header = false;
  let mut current_node_index = 0;
  config.lines().for_each(
    |line| {
      let trimmed = line.trim();
      match trimmed.chars().nth(0) {
        Some('#') => {
          seen_header = true;

          if options.print_fmd_trace { log(format!("Next header. Previous was: {:?}", nodes[nodes.len()-1])); }
          let rest = trimmed[1..].trim();
          let mut split = rest.split('_');
          let kind = split.next().or(Some("Quest")).unwrap().trim(); // first
          let name = split.next_back().or(Some("MissingName")).unwrap().trim(); // last
          let icon = if rest == "Part_None" { ' ' } else { '?' };
          if options.print_fmd_trace { log(format!("- raw: `{}`, kind: `{}`, name: `{}`", rest, kind, name)); }
          let node_index: usize =
            match rest {
              "Part_None" => PARTKIND_NONE,
              "Part_Trash" => PARTKIND_TRASH,
              "Supply_Up" => CONFIG_NODE_SUPPLY_UP,
              "Supply_Right" => CONFIG_NODE_SUPPLY_RIGHT,
              "Supply_Down" => CONFIG_NODE_SUPPLY_DOWN,
              "Supply_Left" => CONFIG_NODE_SUPPLY_LEFT,
              "Demand_Up" => CONFIG_NODE_DEMAND_UP,
              "Demand_Right" => CONFIG_NODE_DEMAND_RIGHT,
              "Demand_Down" => CONFIG_NODE_DEMAND_DOWN,
              "Demand_Left" => CONFIG_NODE_DEMAND_LEFT,
              "Dock_Up" => CONFIG_NODE_DOCK_UP,
              "Dock_Right" => CONFIG_NODE_DOCK_RIGHT,
              "Dock_Down" => CONFIG_NODE_DOCK_DOWN,
              "Dock_Left" => CONFIG_NODE_DOCK_LEFT,
              "Machine_3x3" => CONFIG_NODE_MACHINE_3x3,
              _ => nodes.len(),
            };
          let current_node = ConfigNode {
            index: node_index,
            kind:
              match kind {
                "Quest" => ConfigNodeKind::Quest,
                "Part" => ConfigNodeKind::Part,
                "Demand" => ConfigNodeKind::Demand,
                "Supply" => ConfigNodeKind::Supply,
                "Dock" => ConfigNodeKind::Dock,
                "Machine" => ConfigNodeKind::Dock,
                _ => panic!("Unsupported node kind. Node headers should be composed like Kind_Name and the kind can only be Quest, Part, Supply, Demand, Machine, or Dock. But it was {:?} (`{}`)", kind, rest),
              },
            name: name.to_string(),
            raw_name: rest.to_string(),
            unlocks_after_by_name: vec!(),
            unlocks_after_by_index: vec!(),
            unlocks_todo_by_index: vec!(),
            starting_part_by_name: vec!(),
            starting_part_by_index: vec!(),
            production_target_by_name: vec!(),
            production_target_by_index: vec!(),
            pattern_by_index: vec!(),
            pattern_by_name: vec!(),
            pattern_unique_icons: vec!(),
            icon,
            file: "".to_string(),
            file_canvas_cache_index: 0,
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            current_state: ConfigNodeState::Waiting,
          };
          if node_index == nodes.len() {
            nodes.push(current_node);
          } else {
            // none, trash, supply, demand, dock
            nodes[node_index] = current_node;
          }
          current_node_index = node_index;
          match kind {
            "Quest" => quest_nodes.push(node_index),
            "Part" => part_nodes.push(node_index),
            "Supply" => {}
            "Demand" => {}
            "Dock" => {}
            "Machine" => {}
            _ => panic!("Unsupported node kind. Node headers should be composed like Kind_Name and the kind can only be Quest, Part, Supply, Demand, Machine, or Dock. But it was {:?}", kind),
          }
        }
        Some('-') => {
          if !seen_header {
            // Could ignore this with a warning ...
            panic!("Invalid config; found line starting with `-` before seeing a line starting with `#`");
          }

          let rest = trimmed[1..].trim();
          let mut split = rest.split(':');
          let label = split.next().or(Some("_")).unwrap().trim(); // first
          let value_raw = split.next_back().or(Some("")).unwrap().trim(); // last

          match label {
            "after" => {
              // This should be a list of zero or more quests that are required to unlock this quest
              let pairs = value_raw.split(',');
              for name_untrimmed in pairs {
                let name = name_untrimmed.trim();
                if name != "" {
                  nodes[current_node_index].unlocks_after_by_name.push(name.trim().to_string());
                }
              }
            }
            "parts" => {
              // This is a list of zero or more parts that unlock when this quest unlocks
              // So it's not _after_ this quest completes, but parts that unlock when starting this quest
              let pairs = value_raw.split(',');
              for name_untrimmed in pairs {
                let name = name_untrimmed.trim();
                if name != "" {
                  nodes[current_node_index].starting_part_by_name.push(name.to_string());
                }
              }
            }
            "targets" => {
              // One or more pairs of counts and parts, the requirements to finish this quest.
              let pairs = value_raw.split(',');
              for pair_untrimmed in pairs {
                let pair = pair_untrimmed.trim();
                if pair != "" {
                  let split = pair.trim().split(' ').collect::<Vec<&str>>();
                  let count_str = split[0].trim();
                  // The count can end with an `x`, strip it
                  let count_str2 =
                    if count_str.ends_with('x') {
                      count_str[..count_str.len()-1].trim()
                    } else {
                      count_str
                    };
                  let count = count_str2.parse::<u32>().or::<Result<u32, &str>>(Ok(0u32)).unwrap();
                  if count == 0 {
                    panic!("Config error: count for {} is zero or otherwise invalid. String found is `{}` (config value = `{}`, after x: `{}`)", nodes[nodes.len() - 1].raw_name, count_str, pair_untrimmed, count_str2);
                  }
                  let name = split[split.len() - 1]; // Ignore multiple spaces between, :shrug:
                  let name = if name == "" { "MissingName" } else { name };
                  log(format!("Parsing counts: `{}` into `{:?}` -> `{}` and `{}`", pair, split, count, name));
                  nodes[current_node_index].production_target_by_name.push((count, name.to_string()));
                }
              }
            }
            "pattern" => {
              // Optional pattern for creating this part in a machine
              // If there's no pattern then the part can be used as a supplier. Otherwise it cannot.
              let pairs = value_raw.split(' ').filter(|s| !s.is_empty());
              for name_untrimmed in pairs {
                let name = name_untrimmed.trim();
                if name != "" {
                  nodes[current_node_index].pattern_by_name.push(name.trim().to_string());
                }
              }
            }
            "char" => {
              // The icon
              nodes[current_node_index].icon = value_raw.bytes().next().or(Some('?' as u8)).unwrap() as char;
            }
            "file" => {
              // The sprite file
              nodes[current_node_index].file = value_raw.trim().to_string();
            }
            | "part_x"
            | "x"
            => {
              // x coord in the sprite file where this sprite begins
              nodes[current_node_index].x = value_raw.parse::<f64>().or::<Result<u32, &str>>(Ok(0.0)).unwrap();
            }
            | "part_y"
            | "y"
            => {
              // y coord in the sprite file where this sprite begins
              nodes[current_node_index].y = value_raw.parse::<f64>().or::<Result<u32, &str>>(Ok(0.0)).unwrap();
            }
            | "part_w"
            | "w"
            => {
              // width in the sprite file of this sprite
              nodes[current_node_index].w = value_raw.parse::<f64>().or::<Result<u32, &str>>(Ok(0.0)).unwrap();
            }
            | "part_h"
            | "h"
            => {
              // height in the sprite file of this sprite
              nodes[current_node_index].h = value_raw.parse::<f64>().or::<Result<u32, &str>>(Ok(0.0)).unwrap();
            }
            "state" => {
              match value_raw {
                "active" => nodes[current_node_index].current_state = ConfigNodeState::Active,
                "finished" => nodes[current_node_index].current_state = ConfigNodeState::Finished,
                "waiting" => nodes[current_node_index].current_state = ConfigNodeState::Waiting,
                  _ => panic!("Only valid states are valid; Expecting one if 'active', 'finished', or 'waiting', got: {}", value_raw),
              }
            }
            _ => panic!("Unsupported node option. Node options must be one of a hard coded set but was `{:?}`", label),
          }
        }
        _ => {
          // comment
        }
      }
    }
  );
  if options.print_fmd_trace { log(format!("Last node was: {:?}", nodes[nodes.len()-1])); }

  // So now we have a serial list of nodes but we need to create a hierarchical tree from them
  // We create two models; one is a tree and the other a hashmap

  // Map (fully qualified) name to index on config nodes
  let mut node_name_to_index = HashMap::new();
  // Create unique index for each unique sprite map url
  let mut sprite_cache_lookup = HashMap::new();
  let mut sprite_cache_order = vec!();

  // Generate lookup table name -> node_index
  nodes.iter_mut().enumerate().for_each(|(i, node)| {
    node_name_to_index.insert(node.raw_name.clone(), i);
    // Add mapping to icon char, too
    if node.icon != '?' {
      node_name_to_index.insert(node.icon.to_string(), i);
    }
  });


  if options.print_fmd_trace { log(format!("+ create part pattern_by_index tables")); }
  nodes.iter_mut().for_each(|node| {
    node.pattern_by_index = node.pattern_by_name.iter().map(|name| {
      let mut t = name.as_str().clone();
      if t == "." || t == "_"{
        // In patterns we use . or _ to represent nothing, which translates to the empty/none part
        t = " ";
      }
      return *node_name_to_index.get(t).unwrap_or_else(| | panic!("pattern_by_name to index: what happened here: unlock name=`{}` of names=`{:?}`", name, node_name_to_index.keys()))
    }).collect::<Vec<PartKind>>();

    // If the pattern was defined with empty nodes then clear it
    if node.pattern_by_index.iter().all(|&part_index| part_index == PARTKIND_NONE) {
      node.pattern_by_index = vec!();
      node.pattern_by_name = vec!();
    }
  });

  for i in 0..nodes.len() {
    // Get all unique required parts, convert them to their icon, order them, create a string
    // If we do the same for the machines then we can do string comparisons.
    let mut icons = nodes[i].pattern_by_index.iter().filter(|&&part_index| part_index != PARTKIND_NONE).map(|&x|x).collect::<Vec<usize>>();
    icons.sort_unstable();
    icons.dedup();
    nodes[i].pattern_unique_icons = icons;
  }

  if options.print_fmd_trace { log(format!("+ create quest unlocks_after_by_index and starting_part_by_index pointers")); }
  quest_nodes.iter().for_each(|&node_index| {
    if options.print_fmd_trace { log(format!("++ quest node index = {}, name = {}, unlocks after = `{:?}`", node_index, nodes[node_index].name, nodes[node_index].unlocks_after_by_name)); }

    let mut indices: Vec<usize> = vec!();
    nodes[node_index].unlocks_after_by_name.iter().for_each(|name| {
      indices.push(*node_name_to_index.get(name.as_str().clone()).unwrap_or_else(| | panic!("parent_quest_name to index: what happened here: unlock name=`{}` of names=`{:?}`", name, node_name_to_index.keys())));
    });
    nodes[node_index].unlocks_after_by_index = indices.clone();
    nodes[node_index].unlocks_todo_by_index = indices; // This one depletes as quests are finished. When the vec is empty, this quest becomes available.

    let mut indices: Vec<usize> = vec!();
    nodes[node_index].starting_part_by_name.iter().for_each(|name| {
      indices.push(*node_name_to_index.get(name.as_str().clone()).unwrap_or_else(| | panic!("starting_part_name to index: what happened here: part name=`{} of names=`{:?}`", name, node_name_to_index.keys())));
    });
    nodes[node_index].starting_part_by_index = indices;

    let mut indices: Vec<(u32, usize)> = vec!();
    nodes[node_index].production_target_by_name.iter().for_each(|(count, name)| {
      let index = *node_name_to_index.get(name.as_str().clone()).unwrap_or_else(| | panic!("production_target_name to index: what happened here: unlock name=`{} of names=`{:?}`", name, node_name_to_index.keys()));
      indices.push((*count, index));
    });
    nodes[node_index].production_target_by_index = indices;

  });

  if options.print_fmd_trace { log(format!("+ prepare unique sprite map pointers")); }
  nodes.iter_mut().enumerate().for_each(|(i, node)| {
    if i == 0 || node.name == "None" {
      // Do not add a sprite map for the None part; we should never be painting it.
      return;
    }

    // Create sprite image index pointers. There will be an array with a canvas loaded with
    // that image and it will sit in a vector at the position of that index.
    let f = node.file.as_str();
    match sprite_cache_lookup.get(f) {
      Some(&index) => {
        node.file_canvas_cache_index = index;
      },
      None => {
        let index = sprite_cache_lookup.len();
        let file = node.file.clone();
        node.file_canvas_cache_index = index;
        sprite_cache_lookup.insert(file.clone(), index);
        sprite_cache_order.push(file);
      }
    }
  });

  if options.print_fmd_trace { log(format!("+ initialize the quest node states")); }
  quest_nodes.iter().for_each(|&quest_index| {
    if options.print_fmd_trace { log(format!("++ state loop")); }
    // If the config specified an initial state then just roll with that
    let mut changed = true;
    while changed {
      if options.print_fmd_trace { log(format!("+++ state inner loop")); }
      // Repeat the process until there's no further changes. This loop is guaranteed to halt.
      changed = false;
      if nodes[quest_index].current_state == ConfigNodeState::Waiting {
        if nodes[quest_index].unlocks_after_by_index.iter().all(|&other_index| nodes[other_index].current_state == ConfigNodeState::Finished) {
          if options.print_fmd_trace { log(format!("+++ Quest `{}` is available because `{:?}` are all finished", nodes[quest_index].name, nodes[quest_index].unlocks_after_by_name)); }
          nodes[quest_index].current_state = ConfigNodeState::Available;
          changed = true;
        }
      }
    }
  });

  if options.print_fmd_trace { log(format!("+ initialize the part node states")); }
  quest_nodes.iter().for_each(|&quest_index| {
    // Clone the list of numbers because otherwise it moves. So be it.
    if options.print_fmd_trace { log(format!("++ Quest Part {} is {:?} and would enable parts {:?} ({:?})", nodes[quest_index].name, nodes[quest_index].current_state, nodes[quest_index].starting_part_by_name, nodes[quest_index].starting_part_by_index)); }
    if nodes[quest_index].current_state != ConfigNodeState::Waiting {
      nodes[quest_index].starting_part_by_index.clone().iter().for_each(|&part_index| {
        if options.print_fmd_trace { log(format!("+++ Part {} is available because Quest {} is available", nodes[part_index].name, nodes[quest_index].name)); }
        nodes[part_index].current_state = ConfigNodeState::Available;
      });
    }
  });

  if options.print_fmd_trace { log(format!("Available Quests and Parts from the start:")); }
  nodes.iter().for_each(|node| {
    if node.current_state != ConfigNodeState::Waiting {
      match node.kind {
        ConfigNodeKind::Part => log(format!("- Part {} will be {:?} from the start", node.raw_name, node.current_state)),
        ConfigNodeKind::Quest => log(format!("- Quest {} will be {:?} from the start", node.raw_name, node.current_state)),
        ConfigNodeKind::Demand => {}
        ConfigNodeKind::Supply => {}
        ConfigNodeKind::Dock => {}
        ConfigNodeKind::Machine => {}
      }
    }
  });

  // log(format!("parsed nodes: {:?}", &nodes[1..]));
  if options.print_fmd_trace { log(format!("parsed map: {:?}", node_name_to_index)); }

  return Config { nodes, quest_nodes, part_nodes, node_name_to_index, sprite_cache_lookup, sprite_cache_order, sprite_cache_canvas: vec!() };
}

pub const EXAMPLE_CONFIG: &str = "
  # Part_DirtWhite

  - char: d
  - file: ./img/roguelikeitems.png
  - x: 59
  - y: 50
  - w: 25
  - h: 25

  # Part_Ingots

  - char: d
  - file: ./img/roguelikeitems.png
  - x: 59
  - y: 50
  - w: 25
  - h: 25

  # Quest_LetsGo

  - requires: 10 Part_DirtWhite, 200 Part_Ingots

  # Part_IngotWhite

  - parents: Quest_LetsGo
  - char: d
  - file: ./img/roguelikeitems.png
  - x: 59
  - y: 50
  - w: 25
  - h: 25

  # Part_EmptyBottle

  - parents: Quest_LetsGo
  - char: d
  - file: ./img/roguelikeitems.png
  - x: 59
  - y: 50
  - w: 25
  - h: 25

  # Part_Rope

  - parents: Quest_LetsGo
  - char: d
  - file: ./img/roguelikeitems.png
  - x: 59
  - y: 50
  - w: 25
  - h: 25

  # Quest_IngotsForLife

  - parents: Quest_LetsGo
  - requires: 100 Part_IngotWhite

  # Part_Wood

  - parents: Quest_IngotsForLife
  - char: d
  - file: ./img/roguelikeitems.png
  - x: 59
  - y: 50
  - w: 25
  - h: 25

  # Part_ShieldWood

  - parents: Quest_IngotsForLife
  - char: d
  - file: ./img/roguelikeitems.png
  - x: 59
  - y: 50
  - w: 25
  - h: 25

  # Part_DirtBlue

  - parents: Quest_IngotsForLife
  - pattern: Part_Wood Part_Wood Part_Wood Part_Wood _ Part_Wood Part_Wood Part_Wood Part_Wood
  - char: d
  - file: ./img/roguelikeitems.png
  - x: 59
  - y: 50
  - w: 25
  - h: 25

  # Part_Paper

  - parents: Quest_IngotsForLife
  - char: d
  - file: ./img/roguelikeitems.png
  - x: 59
  - y: 50
  - w: 25
  - h: 25
  ";
