use std::collections::HashMap;
use std::collections::HashSet;

use super::part::*;
use super::utils::*;

pub struct Config {
  pub nodes: Vec<ConfigNode>,
  pub map: HashMap<String, usize>,
  pub sprite_cache_lookup: HashMap<String, usize>, // indexes into sprite_cache_canvas
  pub sprite_cache_canvas: Vec<web_sys::HtmlImageElement>,
}

#[derive(Debug)]
pub struct ConfigNode {
  pub kind: ConfigNodeKind,
  pub name: String,
  pub raw_name: String,
  pub parents: Vec<String>,
  pub requires: Vec<(u32, String)>,
  pub pattern: Vec<String>,
  pub icon: char,
  pub file: String,
  pub file_canvas_cache_index: usize,
  pub x: u64,
  pub y: u64,
  pub w: u64,
  pub h: u64,
}

#[derive(Debug)]
pub enum ConfigNodeKind {
  Part,
  Quest,
}

pub fn parse_fmd(config: String) -> Config {
  // Parse Fake MD config

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

  let mut nodes: Vec<ConfigNode> = vec!(ConfigNode {
    kind: ConfigNodeKind::Quest,
    name: "<placeholder>".to_string(),
    raw_name: "<raw placeholder>".to_string(),
    parents: vec!(),
    requires: vec!(),
    pattern: vec!(),
    icon: '?',
    file: "".to_string(),
    file_canvas_cache_index: 0,
    x: 0,
    y: 0,
    w: 0,
    h: 0
  });

  config.lines().for_each(
    |line| {
      let trimmed = line.trim();
      match trimmed.chars().nth(0) {
        Some('#') => {
          let rest = trimmed[1..].trim();
          let mut split = rest.split('_');
          let kind = split.next().or(Some("Quest")).unwrap().trim(); // first
          let name = split.next_back().or(Some("MissingName")).unwrap().trim(); // last
          let current_node = ConfigNode {
            kind:
            match kind {
              "Quest" => ConfigNodeKind::Quest,
              "Part" => ConfigNodeKind::Part,
              _ => panic!("Unsupported node kind. Node headers should be composed like Kind_Name and the kind can only be Quest or Part but was {:?}", kind),
            },
            name: name.to_string(),
            raw_name: rest.to_string(),
            parents: vec!(),
            requires: vec!(),
            pattern: vec!(),
            icon: '?',
            file: "".to_string(),
            file_canvas_cache_index: 0,
            x: 0,
            y: 0,
            w: 0,
            h: 0
          };
          nodes.push(current_node);
        }
        Some('-') => {
          let rest = trimmed[1..].trim();
          let mut split = rest.split(':');
          let label = split.next().or(Some("_")).unwrap().trim(); // first
          let value_raw = split.next_back().or(Some("")).unwrap().trim(); // last

          match label {
            "parents" => {
              let pairs = value_raw.split(',');
              for name in pairs {
                nodes.last_mut().unwrap().parents.push(name.trim().to_string());
              }
            }
            "requires" => {
              let pairs = value_raw.split(',');
              for pair in pairs {
                let mut split = pair.trim().split(' ');
                let count = split.next().or(Some("0")).unwrap().trim().parse::<u32>().or::<Result<u32, &str>>(Ok(0u32)).unwrap();
                let name = split.next_back().or(Some("MissingName")).unwrap().trim(); // last
                nodes.last_mut().unwrap().requires.push((count, name.to_string()));
              }
            }
            "pattern" => {
              let pairs = value_raw.split(' ').filter(|s| !s.is_empty());
              for name in pairs {
                nodes.last_mut().unwrap().parents.push(name.trim().to_string());
              }
            }
            "char" => {
              nodes.last_mut().unwrap().icon = value_raw.bytes().next().or(Some('?' as u8)).unwrap() as char;
            }
            "file" => {
              nodes.last_mut().unwrap().file = value_raw.trim().to_string();
            }
            "x" => {
              nodes.last_mut().unwrap().x = value_raw.parse::<u64>().or::<Result<u32, &str>>(Ok(0)).unwrap();
            }
            "y" => {
              nodes.last_mut().unwrap().y = value_raw.parse::<u64>().or::<Result<u32, &str>>(Ok(0)).unwrap();
            }
            "w" => {
              nodes.last_mut().unwrap().w = value_raw.parse::<u64>().or::<Result<u32, &str>>(Ok(0)).unwrap();
            }
            "h" => {
              nodes.last_mut().unwrap().h = value_raw.parse::<u64>().or::<Result<u32, &str>>(Ok(0)).unwrap();
            }
            _ => panic!("Unsupported node option. Node options must be one of a hard coded set but was {:?}", label),
          }
        }
        _ => {
          // comment
        }
      }
    }
  );

  // So now we have a serial list of nodes but we need to create a hierarchical tree from them
  // We create two models; one is a tree and the other a hashmap

  // Map name to index on config nodes
  let mut map = HashMap::new();
  // Create unique index for each unique sprite map url
  let mut sprite_cache_lookup = HashMap::new();

  nodes.iter_mut().enumerate().for_each(|(i, node)| {
    if i == 0 {
      return;
    }

    map.insert(node.name.clone(), i);
    // Add mapping to icon char, too
    if node.icon != '?' {
      map.insert(node.icon.to_string(), i);
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
      }
    }
  });

  // log(format!("parsed nodes: {:?}", &nodes[1..]));
  log(format!("parsed map: {:?}", map));

  return Config { nodes, map, sprite_cache_lookup, sprite_cache_canvas: vec!() };
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
