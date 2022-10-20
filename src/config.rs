use std::collections::HashMap;
use std::collections::HashSet;

use wasm_bindgen::{JsValue} ;

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
pub const CONFIG_NODE_MACHINE_1X1: usize = 14;
pub const CONFIG_NODE_MACHINE_2X2: usize = 15;
pub const CONFIG_NODE_MACHINE_3X3: usize = 16;
pub const CONFIG_NODE_BELT_D_U: usize = 17;
pub const CONFIG_NODE_BELT_U_D: usize = 18;
pub const CONFIG_NODE_BELT_DU: usize = 19;
pub const CONFIG_NODE_BELT_L_R: usize = 20;
pub const CONFIG_NODE_BELT_R_L: usize = 21;
pub const CONFIG_NODE_BELT_LR: usize = 22;
pub const CONFIG_NODE_BELT_L_U: usize = 23;
pub const CONFIG_NODE_BELT_U_L: usize = 24;
pub const CONFIG_NODE_BELT_LU: usize = 25;
pub const CONFIG_NODE_BELT_R_U: usize = 26;
pub const CONFIG_NODE_BELT_U_R: usize = 27;
pub const CONFIG_NODE_BELT_RU: usize = 28;
pub const CONFIG_NODE_BELT_D_R: usize = 29;
pub const CONFIG_NODE_BELT_R_D: usize = 30;
pub const CONFIG_NODE_BELT_DR: usize = 31;
pub const CONFIG_NODE_BELT_D_L: usize = 32;
pub const CONFIG_NODE_BELT_L_D: usize = 33;
pub const CONFIG_NODE_BELT_DL: usize = 34;
pub const CONFIG_NODE_BELT_DU_R: usize = 35;
pub const CONFIG_NODE_BELT_DR_U: usize = 36;
pub const CONFIG_NODE_BELT_D_RU: usize = 37;
pub const CONFIG_NODE_BELT_RU_D: usize = 38;
pub const CONFIG_NODE_BELT_R_DU: usize = 39;
pub const CONFIG_NODE_BELT_U_DR: usize = 40;
pub const CONFIG_NODE_BELT_DRU: usize = 41;
pub const CONFIG_NODE_BELT_LU_R: usize = 42;
pub const CONFIG_NODE_BELT_LR_U: usize = 43;
pub const CONFIG_NODE_BELT_L_RU: usize = 44;
pub const CONFIG_NODE_BELT_RU_L: usize = 45;
pub const CONFIG_NODE_BELT_R_LU: usize = 46;
pub const CONFIG_NODE_BELT_U_LR: usize = 47;
pub const CONFIG_NODE_BELT_LRU: usize = 48;
pub const CONFIG_NODE_BELT_DL_R: usize = 49;
pub const CONFIG_NODE_BELT_DR_L: usize = 50;
pub const CONFIG_NODE_BELT_D_LR: usize = 51;
pub const CONFIG_NODE_BELT_LR_D: usize = 52;
pub const CONFIG_NODE_BELT_R_DL: usize = 53;
pub const CONFIG_NODE_BELT_L_DR: usize = 54;
pub const CONFIG_NODE_BELT_DLR: usize = 55;
pub const CONFIG_NODE_BELT_DL_U: usize = 56;
pub const CONFIG_NODE_BELT_DU_L: usize = 57;
pub const CONFIG_NODE_BELT_D_LU: usize = 58;
pub const CONFIG_NODE_BELT_LU_D: usize = 59;
pub const CONFIG_NODE_BELT_U_DL: usize = 60;
pub const CONFIG_NODE_BELT_L_DU: usize = 61;
pub const CONFIG_NODE_BELT_DLU: usize = 62;
pub const CONFIG_NODE_BELT_DLR_U: usize = 63;
pub const CONFIG_NODE_BELT_DLU_R: usize = 64;
pub const CONFIG_NODE_BELT_DRU_L: usize = 65;
pub const CONFIG_NODE_BELT_LRU_D: usize = 66;
pub const CONFIG_NODE_BELT_DL_RU: usize = 67;
pub const CONFIG_NODE_BELT_DR_LU: usize = 68;
pub const CONFIG_NODE_BELT_DU_LR: usize = 69;
pub const CONFIG_NODE_BELT_LR_DU: usize = 70;
pub const CONFIG_NODE_BELT_LU_DR: usize = 71;
pub const CONFIG_NODE_BELT_RU_DL: usize = 72;
pub const CONFIG_NODE_BELT_D_LRU: usize = 73;
pub const CONFIG_NODE_BELT_L_DRU: usize = 74;
pub const CONFIG_NODE_BELT_R_DLU: usize = 75;
pub const CONFIG_NODE_BELT_U_DLR: usize = 76;
pub const CONFIG_NODE_BELT_DLRU: usize = 77;
pub const CONFIG_NODE_BELT_NONE: usize = 78;
pub const CONFIG_NODE_BELT_UNKNOWN: usize = 79;
pub const CONFIG_NODE_BELT_INVALID: usize = 80;

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
  pub pattern_by_icon: Vec<char>, // Machine pattern that generates this part
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
  Belt,
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

pub fn parse_fmd(print_fmd_trace: bool, config: String) -> Config {
  // Parse Fake MD config
  log(format!("parse_fmd(print_fmd_trace={})", print_fmd_trace));

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

  let mut nodes: Vec<ConfigNode> = get_system_nodes();
  nodes.iter().enumerate().for_each(|(i, n)| assert!(n.index == i, "config.nodes should start with a set of nodes where each index is set correctly, index {} was not {}", n.index, i));
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

          if print_fmd_trace { log(format!("Next header. Previous was: {:?}", nodes[nodes.len()-1])); }
          let rest = trimmed[1..].trim();
          let mut split = rest.split('_');
          let kind = split.next().or(Some("Quest")).unwrap().trim(); // first
          let name = split.next_back().or(Some("MissingName")).unwrap().trim(); // last
          let icon = if rest == "Part_None" { ' ' } else { '?' };
          if print_fmd_trace { log(format!("- raw: `{}`, kind: `{}`, name: `{}`", rest, kind, name)); }
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
              "Machine_3x3" => CONFIG_NODE_MACHINE_3X3,
              "Belt_D_U" => CONFIG_NODE_BELT_D_U,
              "Belt_U_D" => CONFIG_NODE_BELT_U_D,
              "Belt_DU" => CONFIG_NODE_BELT_DU,
              "Belt_L_R" => CONFIG_NODE_BELT_L_R,
              "Belt_R_L" => CONFIG_NODE_BELT_R_L,
              "Belt_LR" => CONFIG_NODE_BELT_LR,
              "Belt_L_U" => CONFIG_NODE_BELT_L_U,
              "Belt_U_L" => CONFIG_NODE_BELT_U_L,
              "Belt_LU" => CONFIG_NODE_BELT_LU,
              "Belt_R_U" => CONFIG_NODE_BELT_R_U,
              "Belt_U_R" => CONFIG_NODE_BELT_U_R,
              "Belt_RU" => CONFIG_NODE_BELT_RU,
              "Belt_D_R" => CONFIG_NODE_BELT_D_R,
              "Belt_R_D" => CONFIG_NODE_BELT_R_D,
              "Belt_DR" => CONFIG_NODE_BELT_DR,
              "Belt_D_L" => CONFIG_NODE_BELT_D_L,
              "Belt_L_D" => CONFIG_NODE_BELT_L_D,
              "Belt_DL" => CONFIG_NODE_BELT_DL,
              "Belt_DU_R" => CONFIG_NODE_BELT_DU_R,
              "Belt_DR_U" => CONFIG_NODE_BELT_DR_U,
              "Belt_D_RU" => CONFIG_NODE_BELT_D_RU,
              "Belt_RU_D" => CONFIG_NODE_BELT_RU_D,
              "Belt_R_DU" => CONFIG_NODE_BELT_R_DU,
              "Belt_U_DR" => CONFIG_NODE_BELT_U_DR,
              "Belt_DRU" => CONFIG_NODE_BELT_DRU,
              "Belt_LU_R" => CONFIG_NODE_BELT_LU_R,
              "Belt_LR_U" => CONFIG_NODE_BELT_LR_U,
              "Belt_L_RU" => CONFIG_NODE_BELT_L_RU,
              "Belt_RU_L" => CONFIG_NODE_BELT_RU_L,
              "Belt_R_LU" => CONFIG_NODE_BELT_R_LU,
              "Belt_U_LR" => CONFIG_NODE_BELT_U_LR,
              "Belt_LRU" => CONFIG_NODE_BELT_LRU,
              "Belt_DL_R" => CONFIG_NODE_BELT_DL_R,
              "Belt_DR_L" => CONFIG_NODE_BELT_DR_L,
              "Belt_D_LR" => CONFIG_NODE_BELT_D_LR,
              "Belt_LR_D" => CONFIG_NODE_BELT_LR_D,
              "Belt_R_DL" => CONFIG_NODE_BELT_R_DL,
              "Belt_L_DR" => CONFIG_NODE_BELT_L_DR,
              "Belt_DLR" => CONFIG_NODE_BELT_DLR,
              "Belt_DL_U" => CONFIG_NODE_BELT_DL_U,
              "Belt_DU_L" => CONFIG_NODE_BELT_DU_L,
              "Belt_D_LU" => CONFIG_NODE_BELT_D_LU,
              "Belt_LU_D" => CONFIG_NODE_BELT_LU_D,
              "Belt_U_DL" => CONFIG_NODE_BELT_U_DL,
              "Belt_L_DU" => CONFIG_NODE_BELT_L_DU,
              "Belt_DLU" => CONFIG_NODE_BELT_DLU,
              "Belt_DLR_U" => CONFIG_NODE_BELT_DLR_U,
              "Belt_DLU_R" => CONFIG_NODE_BELT_DLU_R,
              "Belt_DRU_L" => CONFIG_NODE_BELT_DRU_L,
              "Belt_LRU_D" => CONFIG_NODE_BELT_LRU_D,
              "Belt_DL_RU" => CONFIG_NODE_BELT_DL_RU,
              "Belt_DR_LU" => CONFIG_NODE_BELT_DR_LU,
              "Belt_DU_LR" => CONFIG_NODE_BELT_DU_LR,
              "Belt_LR_DU" => CONFIG_NODE_BELT_LR_DU,
              "Belt_LU_DR" => CONFIG_NODE_BELT_LU_DR,
              "Belt_RU_DL" => CONFIG_NODE_BELT_RU_DL,
              "Belt_D_LRU" => CONFIG_NODE_BELT_D_LRU,
              "Belt_L_DRU" => CONFIG_NODE_BELT_L_DRU,
              "Belt_R_DLU" => CONFIG_NODE_BELT_R_DLU,
              "Belt_U_DLR" => CONFIG_NODE_BELT_U_DLR,
              "Belt_DLRU" => CONFIG_NODE_BELT_DLRU,
              "Belt_None" => CONFIG_NODE_BELT_NONE,
              "Belt_Unknown" => CONFIG_NODE_BELT_UNKNOWN,
              "Belt_Invalid" => CONFIG_NODE_BELT_INVALID,
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
                "Belt" => ConfigNodeKind::Belt,
                _ => panic!("Unsupported node kind. Node headers should be composed like Kind_Name and the kind can only be Quest, Part, Supply, Demand, Machine, Belt, or Dock. But it was {:?} (`{}`)", kind, rest),
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
            pattern_by_icon: vec!(),
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
            "Belt" => {},
            _ => panic!("Unsupported node kind. Node headers should be composed like Kind_Name and the kind can only be Quest, Part, Supply, Demand, Machine, Belt, or Dock. But it was {:?}", kind),
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
              for s in pairs {
                let pairs = s.split(' ');
                for name_untrimmed in pairs {
                  let name = name_untrimmed.trim();
                  if name != "" {
                    nodes[current_node_index].unlocks_after_by_name.push(name.trim().to_string());
                  }
                }
              }
            }
            "parts" => {
              // This is a list of zero or more parts that unlock when this quest unlocks
              // So it's not _after_ this quest completes, but parts that unlock when starting this quest
              let pairs = value_raw.split(',');
              for s in pairs {
                let pairs = s.split(' ');
                for name_untrimmed in pairs {
                  let name = name_untrimmed.trim();
                  if name != "" {
                    nodes[current_node_index].starting_part_by_name.push(name.to_string());
                  }
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
              let pairs = value_raw.split(' ');
              for s in pairs {
                let pairs = s.split(',');
                for name_untrimmed in pairs {
                  let name = name_untrimmed.trim();
                  if name != "" {
                    nodes[current_node_index].pattern_by_name.push(name.trim().to_string());
                  }
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
  if print_fmd_trace { log(format!("Last node was: {:?}", nodes[nodes.len()-1])); }

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

  if print_fmd_trace { log(format!("+ create part pattern_by_index tables")); }
  for i in 0..nodes.len() {
    let node = &mut nodes[i];

    // First resolve the target index, regardless of whether the name or icon was used
    nodes[i].pattern_by_index = nodes[i].pattern_by_name.iter().map(|name| {
      let mut t = name.as_str().clone();
      if t == "." || t == "_"{
        // In patterns we use . or _ to represent nothing, which translates to the empty/none part
        t = " ";
      }
      return *node_name_to_index.get(t).unwrap_or_else(| | panic!("pattern_by_name to index: what happened here: unlock name=`{}` of names=`{:?}`", name, node_name_to_index.keys()))
    }).collect::<Vec<PartKind>>();

    // Fetch all the icons
    nodes[i].pattern_by_icon = nodes[i].pattern_by_index.iter().map(|&index| nodes[index].icon).collect::<Vec<char>>();

    // Normalize to name (could be icon)
    nodes[i].pattern_by_name = nodes[i].pattern_by_index.iter().map(|&index| nodes[index].name.clone()).collect::<Vec<String>>();

    // If the pattern was defined with empty nodes then clear it
    if nodes[i].pattern_by_index.iter().all(|&part_index| part_index == PARTKIND_NONE) {
      nodes[i].pattern_by_index = vec!();
      nodes[i].pattern_by_name = vec!();
      nodes[i].pattern_by_icon = vec!();
    }
  // });
  }

  for i in 0..nodes.len() {
    // Get all unique required parts, convert them to their icon, order them, create a string
    // If we do the same for the machines then we can do string comparisons.
    let mut icons = nodes[i].pattern_by_index.iter().filter(|&&part_index| part_index != PARTKIND_NONE).map(|&x|x).collect::<Vec<usize>>();
    icons.sort_unstable();
    icons.dedup();
    nodes[i].pattern_unique_icons = icons;
  }

  if print_fmd_trace { log(format!("+ create quest unlocks_after_by_index and starting_part_by_index pointers")); }
  quest_nodes.iter().for_each(|&node_index| {
    if print_fmd_trace { log(format!("++ quest node index = {}, name = {}, unlocks after = `{:?}`", node_index, nodes[node_index].name, nodes[node_index].unlocks_after_by_name)); }

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

  if print_fmd_trace { log(format!("+ prepare unique sprite map pointers")); }
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

  if print_fmd_trace { log(format!("+ initialize the quest node states")); }
  quest_nodes.iter().for_each(|&quest_index| {
    if print_fmd_trace { log(format!("++ state loop")); }
    // If the config specified an initial state then just roll with that
    let mut changed = true;
    while changed {
      if print_fmd_trace { log(format!("+++ state inner loop")); }
      // Repeat the process until there's no further changes. This loop is guaranteed to halt.
      changed = false;
      if nodes[quest_index].current_state == ConfigNodeState::Waiting {
        if nodes[quest_index].unlocks_after_by_index.iter().all(|&other_index| nodes[other_index].current_state == ConfigNodeState::Finished) {
          if print_fmd_trace { log(format!("+++ Quest `{}` is available because `{:?}` are all finished", nodes[quest_index].name, nodes[quest_index].unlocks_after_by_name)); }
          nodes[quest_index].current_state = ConfigNodeState::Available;
          changed = true;
        }
      }
    }
  });

  if print_fmd_trace { log(format!("+ initialize the part node states")); }
  quest_nodes.iter().for_each(|&quest_index| {
    // Clone the list of numbers because otherwise it moves. So be it.
    if print_fmd_trace { log(format!("++ Quest Part {} is {:?} and would enable parts {:?} ({:?})", nodes[quest_index].name, nodes[quest_index].current_state, nodes[quest_index].starting_part_by_name, nodes[quest_index].starting_part_by_index)); }
    if nodes[quest_index].current_state != ConfigNodeState::Waiting {
      nodes[quest_index].starting_part_by_index.clone().iter().for_each(|&part_index| {
        if print_fmd_trace { log(format!("+++ Part {} is available because Quest {} is available", nodes[part_index].name, nodes[quest_index].name)); }
        nodes[part_index].current_state = ConfigNodeState::Available;
      });
    }
  });

  if print_fmd_trace { log(format!("Available Quests and Parts from the start:")); }
  nodes.iter().for_each(|node| {
    if node.current_state != ConfigNodeState::Waiting {
      match node.kind {
        ConfigNodeKind::Part => log(format!("- Part {} will be {:?} from the start", node.raw_name, node.current_state)),
        ConfigNodeKind::Quest => log(format!("- Quest {} will be {:?} from the start", node.raw_name, node.current_state)),
        ConfigNodeKind::Demand => {}
        ConfigNodeKind::Supply => {}
        ConfigNodeKind::Dock => {}
        ConfigNodeKind::Machine => {}
        ConfigNodeKind::Belt => {}
      }
    }
  });

  // log(format!("parsed nodes: {:?}", &nodes[1..]));
  if print_fmd_trace { log(format!("parsed map: {:?}", node_name_to_index)); }

  return Config { nodes, quest_nodes, part_nodes, node_name_to_index, sprite_cache_lookup, sprite_cache_order, sprite_cache_canvas: vec!() };
}

fn get_system_nodes() -> Vec<ConfigNode> {
  // These are default nodes that you can still override like any other node in the config

  let v = vec!(
    config_node_part(PARTKIND_NONE, "None".to_string(), ' '),
    config_node_part(PARTKIND_TRASH, "Trash".to_string(), 't'),
    config_node_supply(CONFIG_NODE_SUPPLY_UP, "Up".to_string()),
    config_node_supply(CONFIG_NODE_SUPPLY_RIGHT, "Right".to_string()),
    config_node_supply(CONFIG_NODE_SUPPLY_DOWN, "Down".to_string()),
    config_node_supply(CONFIG_NODE_SUPPLY_LEFT, "Left".to_string()),
    config_node_demand(CONFIG_NODE_DEMAND_UP, "Up".to_string()),
    config_node_demand(CONFIG_NODE_DEMAND_RIGHT, "Right".to_string()),
    config_node_demand(CONFIG_NODE_DEMAND_DOWN, "Down".to_string()),
    config_node_demand(CONFIG_NODE_DEMAND_LEFT, "Left".to_string()),
    config_node_dock(CONFIG_NODE_DOCK_UP, "Up".to_string()),
    config_node_dock(CONFIG_NODE_DOCK_RIGHT, "Right".to_string()),
    config_node_dock(CONFIG_NODE_DOCK_DOWN, "Down".to_string()),
    config_node_dock(CONFIG_NODE_DOCK_LEFT, "Left".to_string()),
    config_node_machine(CONFIG_NODE_MACHINE_1X1, "1x1".to_string(), "./img/machine_1_1.png".to_string()),
    config_node_machine(CONFIG_NODE_MACHINE_2X2, "2x2".to_string(), "./img/machine_2_2.png".to_string()),
    config_node_machine(CONFIG_NODE_MACHINE_3X3, "3x3".to_string(), "./img/machine_1_1.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_D_U, "D_U".to_string(), "./img/d_u.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_U_D, "U_D".to_string(), "./img/u_d.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DU, "DU".to_string(), "./img/du.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_L_R, "L_R".to_string(), "./img/l_r.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_R_L, "R_L".to_string(), "./img/r_l.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_LR, "LR".to_string(), "./img/lr.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_L_U, "L_U".to_string(), "./img/l_u.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_U_L, "U_L".to_string(), "./img/u_l.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_LU, "LU".to_string(), "./img/lu.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_R_U, "R_U".to_string(), "./img/r_u.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_U_R, "U_R".to_string(), "./img/u_r.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_RU, "RU".to_string(), "./img/ru.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_D_R, "D_R".to_string(), "./img/d_r.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_R_D, "R_D".to_string(), "./img/r_d.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DR, "DR".to_string(), "./img/dr.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_D_L, "D_L".to_string(), "./img/d_l.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_L_D, "L_D".to_string(), "./img/l_d.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DL, "DL".to_string(), "./img/dl.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DU_R, "DU_R".to_string(), "./img/du_r.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DR_U, "DR_U".to_string(), "./img/dr_u.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_D_RU, "D_RU".to_string(), "./img/d_ru.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_RU_D, "RU_D".to_string(), "./img/ru_d.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_R_DU, "R_DU".to_string(), "./img/r_du.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_U_DR, "U_DR".to_string(), "./img/u_dr.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DRU, "DRU".to_string(), "./img/dru.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_LU_R, "LU_R".to_string(), "./img/lu_r.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_LR_U, "LR_U".to_string(), "./img/lr_u.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_L_RU, "L_RU".to_string(), "./img/l_ru.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_RU_L, "RU_L".to_string(), "./img/ru_l.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_R_LU, "R_LU".to_string(), "./img/r_lu.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_U_LR, "U_LR".to_string(), "./img/u_lr.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_LRU, "LRU".to_string(), "./img/lru.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DL_R, "DL_R".to_string(), "./img/dl_r.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DR_L, "DR_L".to_string(), "./img/dr_l.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_D_LR, "D_LR".to_string(), "./img/d_lr.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_LR_D, "LR_D".to_string(), "./img/lr_d.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_R_DL, "R_DL".to_string(), "./img/r_dl.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_L_DR, "L_DR".to_string(), "./img/l_dr.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DLR, "DLR".to_string(), "./img/dlr.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DL_U, "DL_U".to_string(), "./img/dl_u.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DU_L, "DU_L".to_string(), "./img/du_l.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_D_LU, "D_LU".to_string(), "./img/d_lu.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_LU_D, "LU_D".to_string(), "./img/lu_d.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_U_DL, "U_DL".to_string(), "./img/u_dl.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_L_DU, "L_DU".to_string(), "./img/l_du.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DLU, "DLU".to_string(), "./img/dlu.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DLR_U, "DLR_U".to_string(), "./img/dlr_u.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DLU_R, "DLU_R".to_string(), "./img/dlu_r.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DRU_L, "DRU_L".to_string(), "./img/dru_l.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_LRU_D, "LRU_D".to_string(), "./img/lru_d.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DL_RU, "DL_RU".to_string(), "./img/dl_ru.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DR_LU, "DR_LU".to_string(), "./img/dr_lu.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DU_LR, "DU_LR".to_string(), "./img/du_lr.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_LR_DU, "LR_DU".to_string(), "./img/lr_du.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_LU_DR, "LU_DR".to_string(), "./img/lu_dr.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_RU_DL, "RU_DL".to_string(), "./img/ru_dl.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_D_LRU, "D_LRU".to_string(), "./img/d_lru.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_L_DRU, "L_DRU".to_string(), "./img/l_dru.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_R_DLU, "R_DLU".to_string(), "./img/r_dlu.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_U_DLR, "U_DLR".to_string(), "./img/u_dlr.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_DLRU, "DLRU".to_string(), "./img/dlru.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_NONE, "NONE".to_string(), "./img/belt_none.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_UNKNOWN, "UNKNOWN".to_string(), "./img/belt_unknown.png".to_string()),
    config_node_belt(CONFIG_NODE_BELT_INVALID, "INVALID".to_string(), "./img/belt_invalid.png".to_string()),
  );

  v.iter().enumerate().for_each(|(i, node)| assert!(node.index == i, "system node indexes must match their global constant value; mismatch for index {}", i));

  return v;
}

fn config_node_part(index: PartKind, name: String, icon: char) -> ConfigNode {
  let raw_name = format!("Part_{}", name);
  return ConfigNode {
    index,
    kind: ConfigNodeKind::Part,
    name,
    raw_name,
    unlocks_after_by_name: vec!(),
    unlocks_after_by_index: vec!(),
    unlocks_todo_by_index: vec!(),
    starting_part_by_name: vec!(),
    starting_part_by_index: vec!(),
    production_target_by_name: vec!(),
    production_target_by_index: vec!(),
    pattern_by_index: vec!(),
    pattern_by_name: vec!(),
    pattern_by_icon: vec!(),
    pattern_unique_icons: vec!(),
    icon,
    file: "".to_string(),
    file_canvas_cache_index: 0,
    x: 0.0,
    y: 0.0,
    w: 0.0,
    h: 0.0,
    current_state: ConfigNodeState::Available,
  };
}
fn config_node_supply(index: PartKind, name: String) -> ConfigNode {
  let raw_name = format!("Supply_{}", name);
  return ConfigNode {
    index,
    kind: ConfigNodeKind::Supply,
    name,
    raw_name,
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
    pattern_by_icon: vec!(),
    icon: '?',
    file: "./img/supply.png".to_string(),
    file_canvas_cache_index: 0,
    x: 0.0,
    y: 0.0,
    w: 32.0,
    h: 32.0,
    current_state: ConfigNodeState::Available,
  };
}
fn config_node_demand(index: PartKind, name: String) -> ConfigNode {
  let raw_name = format!("Demand_{}", name);
  return ConfigNode {
    index,
    kind: ConfigNodeKind::Demand,
    name,
    raw_name,
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
    pattern_by_icon: vec!(),
    icon: '?',
    file: "./img/demand.png".to_string(),
    file_canvas_cache_index: 0,
    x: 0.0,
    y: 0.0,
    w: 32.0,
    h: 32.0,
    current_state: ConfigNodeState::Available,
  };
}
fn config_node_dock(index: PartKind, name: String) -> ConfigNode {
  let raw_name = format!("Dock_{}", name);
  return ConfigNode {
    index,
    kind: ConfigNodeKind::Dock,
    name,
    raw_name,
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
    pattern_by_icon: vec!(),
    icon: '?',
    file: "./img/dock.png".to_string(),
    file_canvas_cache_index: 0,
    x: 0.0,
    y: 0.0,
    w: 64.0,
    h: 64.0,
    current_state: ConfigNodeState::Available,
  };
}
fn config_node_machine(index: PartKind, name: String, file: String) -> ConfigNode {
  let raw_name = format!("Machine_{}", name);
  return ConfigNode {
    index,
    kind: ConfigNodeKind::Machine,
    name,
    raw_name,
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
    pattern_by_icon: vec!(),
    icon: '?',
    file,
    file_canvas_cache_index: 0,
    // This hints where on this machine tile the output part icon of this machine should be painted
    x: 5.0,
    y: 5.0,
    w: 5.0,
    h: 5.0,
    current_state: ConfigNodeState::Available,
  };
}
fn config_node_belt(index: PartKind, name: String, file: String) -> ConfigNode {
  let raw_name = format!("Belt_{}", name);
  return ConfigNode {
    index,
    kind: ConfigNodeKind::Machine,
    name,
    raw_name,
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
    pattern_by_icon: vec!(),
    icon: '?',
    file,
    file_canvas_cache_index: 0,
    // This hints where on this machine tile the output part icon of this machine should be painted
    x: 0.0,
    y: 0.0,
    w: 160.0,
    h: 160.0,
    current_state: ConfigNodeState::Available,
  };
}

pub fn config_get_sprite_details(config: &Config, kind: usize) -> ( f64, f64, f64, f64, &web_sys::HtmlImageElement ) {
  assert!(kind < config.nodes.len(), "kind should be a node index: {} < {}", kind, config.nodes.len());
  let node = &config.nodes[kind];
  return ( node.x as f64, node.y as f64, node.w as f64, node.h as f64, &config.sprite_cache_canvas[node.file_canvas_cache_index] );
}

// Convert a config node to jsvalue (sorta) so we can send it to the html for debug/editor
fn convert_vec_string_to_jsvalue(v: &Vec<String>) -> JsValue {
  return v.iter().cloned().map(|x| JsValue::from(x)).collect::<js_sys::Array>().into();
}
fn convert_vec_char_to_jsvalue(v: &Vec<char>) -> JsValue {
  return v.iter().cloned().map(|x| JsValue::from(format!("{}", x))).collect::<js_sys::Array>().into();
}
fn convert_vec_usize_to_jsvalue(v: &Vec<usize>) -> JsValue {
  return v.iter().cloned().map(|x| JsValue::from(x)).collect::<js_sys::Array>().into();
}
fn convert_string_to_pair(a: &str, b: &str) -> js_sys::Array {
  return convert_js_to_pair(a, JsValue::from(b));
}
fn convert_js_to_pair(a: &str, b: JsValue) -> js_sys::Array {
  return js_sys::Array::of2(&JsValue::from(a), &b);
}
fn config_node_to_jsvalue(node: &ConfigNode) -> JsValue {
  return vec!(
    convert_js_to_pair("index", JsValue::from(node.index)),
    convert_string_to_pair("kind", format!("{:?}", node.kind).as_str()),
    convert_string_to_pair("name", &node.name),
    convert_string_to_pair("raw_name", &node.raw_name),

    convert_js_to_pair("unlocks_after_by_name", convert_vec_string_to_jsvalue(&node.unlocks_after_by_name)),
    convert_js_to_pair("unlocks_after_by_index", convert_vec_usize_to_jsvalue(&node.unlocks_after_by_index)),
    convert_js_to_pair("unlocks_todo_by_index", convert_vec_usize_to_jsvalue(&node.unlocks_todo_by_index)),
    convert_js_to_pair("starting_part_by_name", convert_vec_string_to_jsvalue(&node.starting_part_by_name)),
    convert_js_to_pair("starting_part_by_index", convert_vec_usize_to_jsvalue(&node.starting_part_by_index)),
    vec!(
      JsValue::from("production_target_by_name"),
      node.production_target_by_name.iter().cloned().map(|(index, name)| vec!(JsValue::from(index), JsValue::from(name)).iter().collect::<js_sys::Array>()).collect::<js_sys::Array>().into()
    ).iter().collect::<js_sys::Array>(),
    vec!(
      JsValue::from("production_target_by_index"),
      node.production_target_by_index.iter().cloned().map(|(index, name)| vec!(JsValue::from(index), JsValue::from(name)).iter().collect::<js_sys::Array>()).collect::<js_sys::Array>().into()
    ).iter().collect::<js_sys::Array>(),

    convert_js_to_pair("pattern_by_index", convert_vec_usize_to_jsvalue(&node.pattern_by_index)),
    convert_js_to_pair("pattern_by_name", convert_vec_string_to_jsvalue(&node.pattern_by_name)),
    convert_js_to_pair("pattern_by_icon", convert_vec_char_to_jsvalue(&node.pattern_by_icon)),
    convert_js_to_pair("pattern_unique_icons", convert_vec_usize_to_jsvalue(&node.pattern_unique_icons)),
    convert_string_to_pair("icon", format!("{}", node.icon).as_str()),
    convert_string_to_pair("file", &node.file),
    convert_js_to_pair("file_canvas_cache_index", JsValue::from(node.file_canvas_cache_index)),
    convert_js_to_pair("x", JsValue::from(node.x)),
    convert_js_to_pair("y", JsValue::from(node.y)),
    convert_js_to_pair("w", JsValue::from(node.w)),
    convert_js_to_pair("h", JsValue::from(node.h)),
    convert_js_to_pair("current_state", JsValue::from(format!("{:?}", node.current_state))),
  ).iter().collect::<js_sys::Array>().into();
}
pub fn config_to_jsvalue(config: &Config) -> JsValue {
  return config.nodes.iter().map(|node| {
    let key: JsValue = node.raw_name.clone().into();
    let value = config_node_to_jsvalue(node);
    return vec!(key, value).iter().collect::<js_sys::Array>();
  }).collect::<js_sys::Array>().into();
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
