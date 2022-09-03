use std::collections::HashMap;
use std::collections::HashSet;

use super::options::*;
use super::part::*;
use super::state::*;
use super::utils::*;

#[derive(Debug)]
pub struct Config {
  pub nodes: Vec<ConfigNode>,
  pub quest_nodes: Vec<usize>, // maps to nodes vec
  pub part_nodes: Vec<usize>, // maps to nodes vec
  pub map: HashMap<String, usize>,
  pub sprite_cache_lookup: HashMap<String, usize>, // indexes into sprite_cache_canvas
  pub sprite_cache_order: Vec<String>, // srcs by index.
  pub sprite_cache_canvas: Vec<web_sys::HtmlImageElement>,
}

#[derive(Debug)]
pub struct ConfigNode {
  pub kind: ConfigNodeKind,
  pub name: String,
  pub raw_name: String,

  // Quest
  pub unlocks_after_by_name: Vec<String>, // Fully qualified name. Becomes available when these quests are finished.
  pub unlocks_after_by_index: Vec<usize>, // Becomes available when these quests are finished
  pub starting_part_by_name: Vec<String>, // Fully qualified name. These parts are available when this quest becomes available
  pub starting_part_by_index: Vec<usize>, // These parts are available when this quest becomes available
  pub production_target_by_name: Vec<(u32, String)>, // Fully qualified name. count,name pairs, you need this to finish the quest
  pub production_target_by_index: Vec<(u32, usize)>, // count,index pairs, you need this to finish the quest

  // Part
  pub pattern: Vec<String>, // Machine pattern that generates this part
  pub icon: char, // Single (unique) character that also represents this part internally
  pub file: String, // Sprite image location
  pub file_canvas_cache_index: usize, // The canvas with the sprite image loaded
  // Coord of the part on the sprite
  pub x: u64,
  pub y: u64,
  pub w: u64,
  pub h: u64,
  // Mostly for debugging
  pub initial_state: ConfigNodeState,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConfigNodeKind {
  Part,
  Quest,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConfigNodeState {
  Active, // Currently enabled to be finished
  Finished, // Already finished
  Waiting, // Waiting for eligibility to be active
  Available, // Part that can be used
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
      kind: ConfigNodeKind::Part,
      name: "None".to_string(),
      raw_name: "Part_None".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern: vec!(),
      icon: ' ',
      file: "".to_string(),
      file_canvas_cache_index: 0,
      x: 0,
      y: 0,
      w: 0,
      h: 0,
      initial_state: ConfigNodeState::Available,
    },
    ConfigNode {
      kind: ConfigNodeKind::Part,
      name: "Trash".to_string(),
      raw_name: "Part_None".to_string(),
      unlocks_after_by_name: vec!(),
      unlocks_after_by_index: vec!(),
      starting_part_by_name: vec!(),
      starting_part_by_index: vec!(),
      production_target_by_name: vec!(),
      production_target_by_index: vec!(),
      pattern: vec!(),
      icon: 't',
      file: "".to_string(),
      file_canvas_cache_index: 0,
      x: 0,
      y: 0,
      w: 0,
      h: 0,
      initial_state: ConfigNodeState::Available,
    },
  );
  // Indirect references to nodes. Can't share direct references so these index the nodes vec.
  let mut quest_nodes: Vec<usize> = vec!();
  let mut part_nodes: Vec<usize> = vec!(0, 1);

  let mut seen_header = false;
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
          let current_node = ConfigNode {
            kind:
              match kind {
                "Quest" => ConfigNodeKind::Quest,
                "Part" => ConfigNodeKind::Part,
                _ => panic!("Unsupported node kind. Node headers should be composed like Kind_Name and the kind can only be Quest or Part but was {:?}", kind),
              },
            name: name.to_string(),
            raw_name: rest.to_string(),
            unlocks_after_by_name: vec!(),
            unlocks_after_by_index: vec!(),
            starting_part_by_name: vec!(),
            starting_part_by_index: vec!(),
            production_target_by_name: vec!(),
            production_target_by_index: vec!(),
            pattern: vec!(),
            icon,
            file: "".to_string(),
            file_canvas_cache_index: 0,
            x: 0,
            y: 0,
            w: 0,
            h: 0,
            initial_state: ConfigNodeState::Waiting,
          };
          let node_index: usize =
            if rest == "Part_None" {
              // PART_NONE = 0
              nodes[0] = current_node;
              0
            } else if rest == "Part_Trash" {
              // PART_TRASH = 1
              nodes[1] = current_node;
              1
            } else {
              nodes.push(current_node);
              nodes.len() - 1
            };
          match kind {
            "Quest" => quest_nodes.push(node_index),
            "Part" => part_nodes.push(node_index),
            _ => panic!("Unsupported node kind. Node headers should be composed like Kind_Name and the kind can only be Quest or Part but was {:?}", kind),
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
                  nodes.last_mut().unwrap().unlocks_after_by_name.push(name.trim().to_string());
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
                  nodes.last_mut().unwrap().starting_part_by_name.push(name.to_string());
                }
              }
            }
            "targets" => {
              // One or more pairs of counts and parts, the requirements to finish this quest.
              let pairs = value_raw.split(',');
              for pair_untrimmed in pairs {
                let pair = pair_untrimmed.trim();
                if pair != "" {
                  let mut split = pair.trim().split(' ');
                  let count = split.next().or(Some("0")).unwrap().trim().parse::<u32>().or::<Result<u32, &str>>(Ok(0u32)).unwrap();
                  let name = split.next_back().or(Some("MissingName")).unwrap().trim(); // last
                  nodes.last_mut().unwrap().production_target_by_name.push((count, name.to_string()));
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
                  nodes.last_mut().unwrap().pattern.push(name.trim().to_string());
                }
              }
            }
            "char" => {
              // The icon
              nodes.last_mut().unwrap().icon = value_raw.bytes().next().or(Some('?' as u8)).unwrap() as char;
            }
            "file" => {
              // The sprite file
              nodes.last_mut().unwrap().file = value_raw.trim().to_string();
            }
            "x" => {
              // x coord in the sprite file where this sprite begins
              nodes.last_mut().unwrap().x = value_raw.parse::<u64>().or::<Result<u32, &str>>(Ok(0)).unwrap();
            }
            "y" => {
              // y coord in the sprite file where this sprite begins
              nodes.last_mut().unwrap().y = value_raw.parse::<u64>().or::<Result<u32, &str>>(Ok(0)).unwrap();
            }
            "w" => {
              // width in the sprite file of this sprite
              nodes.last_mut().unwrap().w = value_raw.parse::<u64>().or::<Result<u32, &str>>(Ok(0)).unwrap();
            }
            "h" => {
              // height in the sprite file of this sprite
              nodes.last_mut().unwrap().h = value_raw.parse::<u64>().or::<Result<u32, &str>>(Ok(0)).unwrap();
            }
            "state" => {
              match value_raw {
                "active" => nodes.last_mut().unwrap().initial_state = ConfigNodeState::Active,
                "finished" => nodes.last_mut().unwrap().initial_state = ConfigNodeState::Finished,
                "waiting" => nodes.last_mut().unwrap().initial_state = ConfigNodeState::Waiting,
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

  if options.print_fmd_trace { log(format!("+ create quest unlocks_after_by_index and starting_part_by_index pointers")); }
  quest_nodes.iter().for_each(|&node_index| {
    if options.print_fmd_trace { log(format!("++ quest node index = {}, name = {}, unlocks after = `{:?}`", node_index, nodes[node_index].name, nodes[node_index].unlocks_after_by_name)); }

    let mut indices: Vec<usize> = vec!();
    nodes[node_index].unlocks_after_by_name.iter().for_each(|name| {
      indices.push(*node_name_to_index.get(name.as_str().clone()).unwrap_or_else(| | panic!("what happened here: unlock name=`{} of names=`{:?}`", name, node_name_to_index.keys())));
    });
    nodes[node_index].unlocks_after_by_index = indices;

    let mut indices: Vec<usize> = vec!();
    nodes[node_index].starting_part_by_name.iter().for_each(|name| {
      indices.push(*node_name_to_index.get(name.as_str().clone()).unwrap_or_else(| | panic!("what happened here: part name=`{} of names=`{:?}`", name, node_name_to_index.keys())));
    });
    nodes[node_index].starting_part_by_index = indices;
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
      if nodes[quest_index].initial_state == ConfigNodeState::Waiting {
        if nodes[quest_index].unlocks_after_by_index.iter().all(|&other_index| nodes[other_index].initial_state == ConfigNodeState::Finished) {
          if options.print_fmd_trace { log(format!("+++ Quest `{}` is available because `{:?}` are all finished", nodes[quest_index].name, nodes[quest_index].unlocks_after_by_name)); }
          nodes[quest_index].initial_state = ConfigNodeState::Available;
          changed = true;
        }
      }
    }
  });

  if options.print_fmd_trace { log(format!("+ initialize the part node states")); }
  quest_nodes.iter().for_each(|&quest_index| {
    // Clone the list of numbers because otherwise it moves. So be it.
    if options.print_fmd_trace { log(format!("++ Quest Part {} is {:?} and would enable parts {:?} ({:?})", nodes[quest_index].name, nodes[quest_index].initial_state, nodes[quest_index].starting_part_by_name, nodes[quest_index].starting_part_by_index)); }
    if nodes[quest_index].initial_state != ConfigNodeState::Waiting {
      nodes[quest_index].starting_part_by_index.clone().iter().for_each(|&part_index| {
        if options.print_fmd_trace { log(format!("+++ Part {} is available because Quest {} is available", nodes[part_index].name, nodes[quest_index].name)); }
        nodes[part_index].initial_state = ConfigNodeState::Available;
      });
    }
  });

  if options.print_fmd_trace { log(format!("Available Quests and Parts from the start:")); }
  nodes.iter().for_each(|node| {
    if node.initial_state != ConfigNodeState::Waiting {
      match node.kind {
        ConfigNodeKind::Part => log(format!("- Part {} will be {:?} from the start", node.raw_name, node.initial_state)),
        ConfigNodeKind::Quest => log(format!("- Quest {} will be {:?} from the start", node.raw_name, node.initial_state)),
      }
    }
  });

  // log(format!("parsed nodes: {:?}", &nodes[1..]));
  if options.print_fmd_trace { log(format!("parsed map: {:?}", node_name_to_index)); }

  return Config { nodes, quest_nodes, part_nodes, map: node_name_to_index, sprite_cache_lookup, sprite_cache_order, sprite_cache_canvas: vec!() };
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
