use std::collections::HashMap;
use std::collections::HashSet;

use wasm_bindgen::{JsValue};
use js_sys;

use super::belt::*;
use super::belt_codes::*;
use super::belt_frame::*;
use super::belt_meta::*;
use super::belt_type::*;
use super::belt_type::*;
use super::machine::*;
use super::options::*;
use super::part::*;
use super::sprite_config::*;
use super::sprite_frame::*;
use super::state::*;
use super::utils::*;
use super::log;

// These index directly to the config.nodes vec and BELT_CODES (for belts)
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
pub const CONFIG_NODE_BELT_NONE: usize = 17;
pub const CONFIG_NODE_BELT_UNKNOWN: usize = 18;
pub const CONFIG_NODE_BELT_INVALID: usize = 19;
pub const CONFIG_NODE_BELT_L_: usize = 20;
pub const CONFIG_NODE_BELT__L: usize = 21;
pub const CONFIG_NODE_BELT___L: usize = 22;
pub const CONFIG_NODE_BELT_D_: usize = 23;
pub const CONFIG_NODE_BELT_DL_: usize = 24;
pub const CONFIG_NODE_BELT_D_L: usize = 25;
pub const CONFIG_NODE_BELT_D__L: usize = 26;
pub const CONFIG_NODE_BELT__D: usize = 27;
pub const CONFIG_NODE_BELT_L_D: usize = 28;
pub const CONFIG_NODE_BELT__DL: usize = 29;
pub const CONFIG_NODE_BELT__D_L: usize = 30;
pub const CONFIG_NODE_BELT___D: usize = 31;
pub const CONFIG_NODE_BELT_L__D: usize = 32;
pub const CONFIG_NODE_BELT__L_D: usize = 33;
pub const CONFIG_NODE_BELT___DL: usize = 34;
pub const CONFIG_NODE_BELT_R_: usize = 35;
pub const CONFIG_NODE_BELT_LR_: usize = 36;
pub const CONFIG_NODE_BELT_R_L: usize = 37;
pub const CONFIG_NODE_BELT_R__L: usize = 38;
pub const CONFIG_NODE_BELT_DR_: usize = 39;
pub const CONFIG_NODE_BELT_DLR_: usize = 40;
pub const CONFIG_NODE_BELT_DR_L: usize = 41;
pub const CONFIG_NODE_BELT_DR__L: usize = 42;
pub const CONFIG_NODE_BELT_R_D: usize = 43;
pub const CONFIG_NODE_BELT_LR_D: usize = 44;
pub const CONFIG_NODE_BELT_R_DL: usize = 45;
pub const CONFIG_NODE_BELT_R_D_L: usize = 46;
pub const CONFIG_NODE_BELT_R__D: usize = 47;
pub const CONFIG_NODE_BELT_LR__D: usize = 48;
pub const CONFIG_NODE_BELT_R_L_D: usize = 49;
pub const CONFIG_NODE_BELT_R__DL: usize = 50;
pub const CONFIG_NODE_BELT__R: usize = 51;
pub const CONFIG_NODE_BELT_L_R: usize = 52;
pub const CONFIG_NODE_BELT__LR: usize = 53;
pub const CONFIG_NODE_BELT__R_L: usize = 54;
pub const CONFIG_NODE_BELT_D_R: usize = 55;
pub const CONFIG_NODE_BELT_DL_R: usize = 56;
pub const CONFIG_NODE_BELT_D_LR: usize = 57;
pub const CONFIG_NODE_BELT_D_R_L: usize = 58;
pub const CONFIG_NODE_BELT__DR: usize = 59;
pub const CONFIG_NODE_BELT_L_DR: usize = 60;
pub const CONFIG_NODE_BELT__DLR: usize = 61;
pub const CONFIG_NODE_BELT__DR_L: usize = 62;
pub const CONFIG_NODE_BELT__R_D: usize = 63;
pub const CONFIG_NODE_BELT_L_R_D: usize = 64;
pub const CONFIG_NODE_BELT__LR_D: usize = 65;
pub const CONFIG_NODE_BELT__R_DL: usize = 66;
pub const CONFIG_NODE_BELT___R: usize = 67;
pub const CONFIG_NODE_BELT_L__R: usize = 68;
pub const CONFIG_NODE_BELT__L_R: usize = 69;
pub const CONFIG_NODE_BELT___LR: usize = 70;
pub const CONFIG_NODE_BELT_D__R: usize = 71;
pub const CONFIG_NODE_BELT_DL__R: usize = 72;
pub const CONFIG_NODE_BELT_D_L_R: usize = 73;
pub const CONFIG_NODE_BELT_D__LR: usize = 74;
pub const CONFIG_NODE_BELT__D_R: usize = 75;
pub const CONFIG_NODE_BELT_L_D_R: usize = 76;
pub const CONFIG_NODE_BELT__DL_R: usize = 77;
pub const CONFIG_NODE_BELT__D_LR: usize = 78;
pub const CONFIG_NODE_BELT___DR: usize = 79;
pub const CONFIG_NODE_BELT_L__DR: usize = 80;
pub const CONFIG_NODE_BELT__L_DR: usize = 81;
pub const CONFIG_NODE_BELT___DLR: usize = 82;
pub const CONFIG_NODE_BELT_U_: usize = 83;
pub const CONFIG_NODE_BELT_LU_: usize = 84;
pub const CONFIG_NODE_BELT_U_L: usize = 85;
pub const CONFIG_NODE_BELT_U__L: usize = 86;
pub const CONFIG_NODE_BELT_DU_: usize = 87;
pub const CONFIG_NODE_BELT_DLU_: usize = 88;
pub const CONFIG_NODE_BELT_DU_L: usize = 89;
pub const CONFIG_NODE_BELT_DU__L: usize = 90;
pub const CONFIG_NODE_BELT_U_D: usize = 91;
pub const CONFIG_NODE_BELT_LU_D: usize = 92;
pub const CONFIG_NODE_BELT_U_DL: usize = 93;
pub const CONFIG_NODE_BELT_U_D_L: usize = 94;
pub const CONFIG_NODE_BELT_U__D: usize = 95;
pub const CONFIG_NODE_BELT_LU__D: usize = 96;
pub const CONFIG_NODE_BELT_U_L_D: usize = 97;
pub const CONFIG_NODE_BELT_U__DL: usize = 98;
pub const CONFIG_NODE_BELT_RU_: usize = 99;
pub const CONFIG_NODE_BELT_LRU_: usize = 100;
pub const CONFIG_NODE_BELT_RU_L: usize = 101;
pub const CONFIG_NODE_BELT_RU__L: usize = 102;
pub const CONFIG_NODE_BELT_DRU_: usize = 103;
pub const CONFIG_NODE_BELT_DLRU_: usize = 104;
pub const CONFIG_NODE_BELT_DRU_L: usize = 105;
pub const CONFIG_NODE_BELT_DRU__L: usize = 106;
pub const CONFIG_NODE_BELT_RU_D: usize = 107;
pub const CONFIG_NODE_BELT_LRU_D: usize = 108;
pub const CONFIG_NODE_BELT_RU_DL: usize = 109;
pub const CONFIG_NODE_BELT_RU_D_L: usize = 110;
pub const CONFIG_NODE_BELT_RU__D: usize = 111;
pub const CONFIG_NODE_BELT_LRU__D: usize = 112;
pub const CONFIG_NODE_BELT_RU_L_D: usize = 113;
pub const CONFIG_NODE_BELT_RU__DL: usize = 114;
pub const CONFIG_NODE_BELT_U_R: usize = 115;
pub const CONFIG_NODE_BELT_LU_R: usize = 116;
pub const CONFIG_NODE_BELT_U_LR: usize = 117;
pub const CONFIG_NODE_BELT_U_R_L: usize = 118;
pub const CONFIG_NODE_BELT_DU_R: usize = 119;
pub const CONFIG_NODE_BELT_DLU_R: usize = 120;
pub const CONFIG_NODE_BELT_DU_LR: usize = 121;
pub const CONFIG_NODE_BELT_DU_R_L: usize = 122;
pub const CONFIG_NODE_BELT_U_DR: usize = 123;
pub const CONFIG_NODE_BELT_LU_DR: usize = 124;
pub const CONFIG_NODE_BELT_U_DLR: usize = 125;
pub const CONFIG_NODE_BELT_U_DR_L: usize = 126;
pub const CONFIG_NODE_BELT_U_R_D: usize = 127;
pub const CONFIG_NODE_BELT_LU_R_D: usize = 128;
pub const CONFIG_NODE_BELT_U_LR_D: usize = 129;
pub const CONFIG_NODE_BELT_U_R_DL: usize = 130;
pub const CONFIG_NODE_BELT_U__R: usize = 131;
pub const CONFIG_NODE_BELT_LU__R: usize = 132;
pub const CONFIG_NODE_BELT_U_L_R: usize = 133;
pub const CONFIG_NODE_BELT_U__LR: usize = 134;
pub const CONFIG_NODE_BELT_DU__R: usize = 135;
pub const CONFIG_NODE_BELT_DLU__R: usize = 136;
pub const CONFIG_NODE_BELT_DU_L_R: usize = 137;
pub const CONFIG_NODE_BELT_DU__LR: usize = 138;
pub const CONFIG_NODE_BELT_U_D_R: usize = 139;
pub const CONFIG_NODE_BELT_LU_D_R: usize = 140;
pub const CONFIG_NODE_BELT_U_DL_R: usize = 141;
pub const CONFIG_NODE_BELT_U_D_LR: usize = 142;
pub const CONFIG_NODE_BELT_U__DR: usize = 143;
pub const CONFIG_NODE_BELT_LU__DR: usize = 144;
pub const CONFIG_NODE_BELT_U_L_DR: usize = 145;
pub const CONFIG_NODE_BELT_U__DLR: usize = 146;
pub const CONFIG_NODE_BELT__U: usize = 147;
pub const CONFIG_NODE_BELT_L_U: usize = 148;
pub const CONFIG_NODE_BELT__LU: usize = 149;
pub const CONFIG_NODE_BELT__U_L: usize = 150;
pub const CONFIG_NODE_BELT_D_U: usize = 151;
pub const CONFIG_NODE_BELT_DL_U: usize = 152;
pub const CONFIG_NODE_BELT_D_LU: usize = 153;
pub const CONFIG_NODE_BELT_D_U_L: usize = 154;
pub const CONFIG_NODE_BELT__DU: usize = 155;
pub const CONFIG_NODE_BELT_L_DU: usize = 156;
pub const CONFIG_NODE_BELT__DLU: usize = 157;
pub const CONFIG_NODE_BELT__DU_L: usize = 158;
pub const CONFIG_NODE_BELT__U_D: usize = 159;
pub const CONFIG_NODE_BELT_L_U_D: usize = 160;
pub const CONFIG_NODE_BELT__LU_D: usize = 161;
pub const CONFIG_NODE_BELT__U_DL: usize = 162;
pub const CONFIG_NODE_BELT_R_U: usize = 163;
pub const CONFIG_NODE_BELT_LR_U: usize = 164;
pub const CONFIG_NODE_BELT_R_LU: usize = 165;
pub const CONFIG_NODE_BELT_R_U_L: usize = 166;
pub const CONFIG_NODE_BELT_DR_U: usize = 167;
pub const CONFIG_NODE_BELT_DLR_U: usize = 168;
pub const CONFIG_NODE_BELT_DR_LU: usize = 169;
pub const CONFIG_NODE_BELT_DR_U_L: usize = 170;
pub const CONFIG_NODE_BELT_R_DU: usize = 171;
pub const CONFIG_NODE_BELT_LR_DU: usize = 172;
pub const CONFIG_NODE_BELT_R_DLU: usize = 173;
pub const CONFIG_NODE_BELT_R_DU_L: usize = 174;
pub const CONFIG_NODE_BELT_R_U_D: usize = 175;
pub const CONFIG_NODE_BELT_LR_U_D: usize = 176;
pub const CONFIG_NODE_BELT_R_LU_D: usize = 177;
pub const CONFIG_NODE_BELT_R_U_DL: usize = 178;
pub const CONFIG_NODE_BELT__RU: usize = 179;
pub const CONFIG_NODE_BELT_L_RU: usize = 180;
pub const CONFIG_NODE_BELT__LRU: usize = 181;
pub const CONFIG_NODE_BELT__RU_L: usize = 182;
pub const CONFIG_NODE_BELT_D_RU: usize = 183;
pub const CONFIG_NODE_BELT_DL_RU: usize = 184;
pub const CONFIG_NODE_BELT_D_LRU: usize = 185;
pub const CONFIG_NODE_BELT_D_RU_L: usize = 186;
pub const CONFIG_NODE_BELT__DRU: usize = 187;
pub const CONFIG_NODE_BELT_L_DRU: usize = 188;
pub const CONFIG_NODE_BELT__DLRU: usize = 189;
pub const CONFIG_NODE_BELT__DRU_L: usize = 190;
pub const CONFIG_NODE_BELT__RU_D: usize = 191;
pub const CONFIG_NODE_BELT_L_RU_D: usize = 192;
pub const CONFIG_NODE_BELT__LRU_D: usize = 193;
pub const CONFIG_NODE_BELT__RU_DL: usize = 194;
pub const CONFIG_NODE_BELT__U_R: usize = 195;
pub const CONFIG_NODE_BELT_L_U_R: usize = 196;
pub const CONFIG_NODE_BELT__LU_R: usize = 197;
pub const CONFIG_NODE_BELT__U_LR: usize = 198;
pub const CONFIG_NODE_BELT_D_U_R: usize = 199;
pub const CONFIG_NODE_BELT_DL_U_R: usize = 200;
pub const CONFIG_NODE_BELT_D_LU_R: usize = 201;
pub const CONFIG_NODE_BELT_D_U_LR: usize = 202;
pub const CONFIG_NODE_BELT__DU_R: usize = 203;
pub const CONFIG_NODE_BELT_L_DU_R: usize = 204;
pub const CONFIG_NODE_BELT__DLU_R: usize = 205;
pub const CONFIG_NODE_BELT__DU_LR: usize = 206;
pub const CONFIG_NODE_BELT__U_DR: usize = 207;
pub const CONFIG_NODE_BELT_L_U_DR: usize = 208;
pub const CONFIG_NODE_BELT__LU_DR: usize = 209;
pub const CONFIG_NODE_BELT__U_DLR: usize = 210;
pub const CONFIG_NODE_BELT___U: usize = 211;
pub const CONFIG_NODE_BELT_L__U: usize = 212;
pub const CONFIG_NODE_BELT__L_U: usize = 213;
pub const CONFIG_NODE_BELT___LU: usize = 214;
pub const CONFIG_NODE_BELT_D__U: usize = 215;
pub const CONFIG_NODE_BELT_DL__U: usize = 216;
pub const CONFIG_NODE_BELT_D_L_U: usize = 217;
pub const CONFIG_NODE_BELT_D__LU: usize = 218;
pub const CONFIG_NODE_BELT__D_U: usize = 219;
pub const CONFIG_NODE_BELT_L_D_U: usize = 220;
pub const CONFIG_NODE_BELT__DL_U: usize = 221;
pub const CONFIG_NODE_BELT__D_LU: usize = 222;
pub const CONFIG_NODE_BELT___DU: usize = 223;
pub const CONFIG_NODE_BELT_L__DU: usize = 224;
pub const CONFIG_NODE_BELT__L_DU: usize = 225;
pub const CONFIG_NODE_BELT___DLU: usize = 226;
pub const CONFIG_NODE_BELT_R__U: usize = 227;
pub const CONFIG_NODE_BELT_LR__U: usize = 228;
pub const CONFIG_NODE_BELT_R_L_U: usize = 229;
pub const CONFIG_NODE_BELT_R__LU: usize = 230;
pub const CONFIG_NODE_BELT_DR__U: usize = 231;
pub const CONFIG_NODE_BELT_DLR__U: usize = 232;
pub const CONFIG_NODE_BELT_DR_L_U: usize = 233;
pub const CONFIG_NODE_BELT_DR__LU: usize = 234;
pub const CONFIG_NODE_BELT_R_D_U: usize = 235;
pub const CONFIG_NODE_BELT_LR_D_U: usize = 236;
pub const CONFIG_NODE_BELT_R_DL_U: usize = 237;
pub const CONFIG_NODE_BELT_R_D_LU: usize = 238;
pub const CONFIG_NODE_BELT_R__DU: usize = 239;
pub const CONFIG_NODE_BELT_LR__DU: usize = 240;
pub const CONFIG_NODE_BELT_R_L_DU: usize = 241;
pub const CONFIG_NODE_BELT_R__DLU: usize = 242;
pub const CONFIG_NODE_BELT__R_U: usize = 243;
pub const CONFIG_NODE_BELT_L_R_U: usize = 244;
pub const CONFIG_NODE_BELT__LR_U: usize = 245;
pub const CONFIG_NODE_BELT__R_LU: usize = 246;
pub const CONFIG_NODE_BELT_D_R_U: usize = 247;
pub const CONFIG_NODE_BELT_DL_R_U: usize = 248;
pub const CONFIG_NODE_BELT_D_LR_U: usize = 249;
pub const CONFIG_NODE_BELT_D_R_LU: usize = 250;
pub const CONFIG_NODE_BELT__DR_U: usize = 251;
pub const CONFIG_NODE_BELT_L_DR_U: usize = 252;
pub const CONFIG_NODE_BELT__DLR_U: usize = 253;
pub const CONFIG_NODE_BELT__DR_LU: usize = 254;
pub const CONFIG_NODE_BELT__R_DU: usize = 255;
pub const CONFIG_NODE_BELT_L_R_DU: usize = 256;
pub const CONFIG_NODE_BELT__LR_DU: usize = 257;
pub const CONFIG_NODE_BELT__R_DLU: usize = 258;
pub const CONFIG_NODE_BELT___RU: usize = 259;
pub const CONFIG_NODE_BELT_L__RU: usize = 260;
pub const CONFIG_NODE_BELT__L_RU: usize = 261;
pub const CONFIG_NODE_BELT___LRU: usize = 262;
pub const CONFIG_NODE_BELT_D__RU: usize = 263;
pub const CONFIG_NODE_BELT_DL__RU: usize = 264;
pub const CONFIG_NODE_BELT_D_L_RU: usize = 265;
pub const CONFIG_NODE_BELT_D__LRU: usize = 266;
pub const CONFIG_NODE_BELT__D_RU: usize = 267;
pub const CONFIG_NODE_BELT_L_D_RU: usize = 268;
pub const CONFIG_NODE_BELT__DL_RU: usize = 269;
pub const CONFIG_NODE_BELT__D_LRU: usize = 270;
pub const CONFIG_NODE_BELT___DRU: usize = 271;
pub const CONFIG_NODE_BELT_L__DRU: usize = 272;
pub const CONFIG_NODE_BELT__L_DRU: usize = 273;
pub const CONFIG_NODE_BELT___DLRU: usize = 274;

#[derive(Debug)]
pub struct Config {
  pub nodes: Vec<ConfigNode>,
  pub quest_nodes: Vec<usize>, // maps to nodes vec
  pub part_nodes: Vec<PartKind>, // maps to nodes vec
  pub node_name_to_index: HashMap<String, PartKind>,
  pub node_pattern_to_index: HashMap<String, PartKind>,
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
  pub pattern_by_name: Vec<String>, // Actual names. Used while parsing. Should only be used for debugging afterwards
  pub pattern_by_icon: Vec<char>, // Char icons. Should only be used for debugging
  pub pattern: String, // pattern_by_icon as a string cached (or "prerendered")
  pub pattern_unique_kinds: Vec<PartKind>, // Unique non-empty part kinds. We can use this to quickly find machines that have received these parts.
  pub icon: char, // Single (unique) character that also represents this part internally

  pub sprite_config: SpriteConfig,

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
  log!("parse_fmd(print_fmd_trace={})", print_fmd_trace);

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
  // Indirect references to nodes. Can't share direct references so these index the nodes vec.
  let mut quest_nodes: Vec<usize> = vec!();
  let mut part_nodes: Vec<usize> = vec!(0, 1);

  let mut first_frame = true;
  let mut seen_header = false;
  let mut current_node_index = 0;
  config.lines().for_each(
    |line| {
      let trimmed = line.trim();
      match trimmed.chars().nth(0) {
        Some('#') => {
          seen_header = true;
          first_frame = true;

          if print_fmd_trace { log!("Next header. Previous was: {:?}", nodes[nodes.len()-1]); }
          let rest = trimmed[1..].trim();
          let mut split = rest.split('_');
          let kind = split.next().or(Some("UnknownPrefix")).unwrap().trim(); // first
          let name = split.collect::<Vec<&str>>();
          let name = name.join("_");
          let name = name.trim();
          // let mut name = split.next_back().or(Some("MissingName")).unwrap().trim(); // last
          let icon = if rest == "Part_None" { ' ' } else { '?' };
          let node_index: usize = config_full_node_name_to_target_index(rest, kind, nodes.len());
          if print_fmd_trace { log!("- raw: `{}`, kind: `{}`, name: `{}`, index: {}", rest, kind, name, node_index); }
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
            pattern: "".to_string(),
            pattern_unique_kinds: vec!(),
            icon,
            sprite_config: SpriteConfig {
              pause_between: 0,
              frame_offset: 0,
              initial_delay: 0,
              looping: false,
              frames: vec![SpriteFrame {
                file: "".to_string(),
                name: "untitled frame".to_string(),
                file_canvas_cache_index: 0,
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.0
              }]
            },
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
                  if print_fmd_trace { log!("Parsing counts: `{}` into `{:?}` -> `{}` and `{}`", pair, split, count, name); }
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
              let last = nodes[current_node_index].sprite_config.frames.len() - 1;
              nodes[current_node_index].sprite_config.frames[last].file = value_raw.trim().to_string();
            }
            | "part_x"
            | "x"
            => {
              // x coord in the sprite file where this sprite begins
              let last = nodes[current_node_index].sprite_config.frames.len() - 1;
              nodes[current_node_index].sprite_config.frames[last].x = value_raw.parse::<f64>().or::<Result<u32, &str>>(Ok(0.0)).unwrap();
            }
            | "part_y"
            | "y"
            => {
              // y coord in the sprite file where this sprite begins
              let last = nodes[current_node_index].sprite_config.frames.len() - 1;
              nodes[current_node_index].sprite_config.frames[last].y = value_raw.parse::<f64>().or::<Result<u32, &str>>(Ok(0.0)).unwrap();
            }
            | "part_w"
            | "w"
            => {
              // width in the sprite file of this sprite
              let last = nodes[current_node_index].sprite_config.frames.len() - 1;
              nodes[current_node_index].sprite_config.frames[last].w = value_raw.parse::<f64>().or::<Result<u32, &str>>(Ok(0.0)).unwrap();
            }
            | "part_h"
            | "h"
            => {
              // height in the sprite file of this sprite
              let last = nodes[current_node_index].sprite_config.frames.len() - 1;
              nodes[current_node_index].sprite_config.frames[last].h = value_raw.parse::<f64>().or::<Result<u32, &str>>(Ok(0.0)).unwrap();
            }
            "state" => {
              match value_raw {
                "active" => nodes[current_node_index].current_state = ConfigNodeState::Active,
                "finished" => nodes[current_node_index].current_state = ConfigNodeState::Finished,
                "waiting" => nodes[current_node_index].current_state = ConfigNodeState::Waiting,
                  _ => panic!("Only valid states are valid; Expecting one if 'active', 'finished', or 'waiting', got: {}", value_raw),
              }
            }
            "frame" => {
              if first_frame {
                // Ignore. Keep the only frame that's already in this sprite and overwrite parts of it.
                first_frame = false;
              } else {
                // Clone the last frame and push it as a new frame.
                let last = nodes[current_node_index].sprite_config.frames.len() - 1;
                let c = nodes[current_node_index].sprite_config.frames[last].clone();
                nodes[current_node_index].sprite_config.frames.push(c);
              }
              let last = nodes[current_node_index].sprite_config.frames.len() - 1;
              nodes[current_node_index].sprite_config.frames[last].name = value_raw.to_string();
            }
            "frame_offset" => {
              nodes[current_node_index].sprite_config.frame_offset = value_raw.parse::<u64>().or::<Result<u32, &str>>(Ok(0)).unwrap();
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
  if print_fmd_trace { log!("Last node was: {:?}", nodes[nodes.len()-1]); }

  // So now we have a serial list of nodes but we need to create a hierarchical tree from them
  // We create two models; one is a tree and the other a hashmap

  // Map (fully qualified) name to index on config nodes
  let mut node_name_to_index = HashMap::new();
  // Map for getting fast pattern lookups. The icons for each part of the pattern (sorted, not none) are serialized into a string and checked against this hashmap. Faster-simpler than anything else.
  let mut node_pattern_to_index = HashMap::new();
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

  if print_fmd_trace { log!("+ create part pattern_by_index tables"); }
  for i in 0..nodes.len() {
    let node = &mut nodes[i];

    // First resolve the target index, regardless of whether the name or icon was used
    let pattern_by_index =
      nodes[i].pattern_by_name.iter()
        .map(|name| {
          let mut t = name.as_str().clone();
          if t == "." || t == "_"{
            // In patterns we use . or _ to represent nothing, which translates to the empty/none part
            t = " ";
          }
          return *node_name_to_index.get(t).unwrap_or_else(| | panic!("pattern_by_name to index: what happened here: unlock name=`{}` of names=`{:?}`", name, node_name_to_index.keys()))
        })
        .collect::<Vec<PartKind>>();
    let pattern_by_index = machine_normalize_wants(&pattern_by_index);

    // Clean up the pattern by name (should only be used for parsing and debugging)
    nodes[i].pattern_by_name = pattern_by_index.iter().map(|&kind| nodes[kind].name.clone()).collect::<Vec<String>>();
    // Should only be used for debugging and serialization
    nodes[i].pattern_by_icon = pattern_by_index.iter().map(|&index| nodes[index].icon).collect::<Vec<char>>();
    nodes[i].pattern_by_icon.sort();

    // Sort by char to normalize it
    let pattern_str = nodes[i].pattern_by_icon.iter().collect::<String>();
    let pattern_str = str::replace(&pattern_str, " ", "");
    let pattern_str = str::replace(&pattern_str, ".", "");
    node_pattern_to_index.insert(pattern_str.clone(), i);
    nodes[i].pattern = pattern_str;

    // Get all unique required parts, convert them to their icon, order them, create a string
    // If we do the same for the machines then we can do string comparisons.
    let mut kinds = pattern_by_index.clone();
    kinds.dedup();
    nodes[i].pattern_unique_kinds = kinds;
    nodes[i].pattern_by_index = pattern_by_index;
  }

  if print_fmd_trace { log!("+ create quest unlocks_after_by_index and starting_part_by_index pointers"); }
  quest_nodes.iter().for_each(|&node_index| {
    if print_fmd_trace { log!("++ quest node index = {}, name = {}, unlocks after = `{:?}`", node_index, nodes[node_index].name, nodes[node_index].unlocks_after_by_name); }

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

  if print_fmd_trace {
    nodes.iter_mut().enumerate().for_each(|(i, node)| {
      log!("- node {} is {}", i, node.raw_name);
    });
  }

  if print_fmd_trace { log!("+ prepare unique sprite map pointers"); }
  nodes.iter_mut().enumerate().for_each(|(i, node)| {
    if i == 0 || node.name == "None" {
      // Do not add a sprite map for the None part; we should never be painting it.
      return;
    }

    // Create sprite image index pointers. There will be an array with a canvas loaded with
    // that image and it will sit in a vector at the position of that index.
    node.sprite_config.frames.iter_mut().for_each(|frame| {
      let f = frame.file.as_str();
      match sprite_cache_lookup.get(f) {
        Some(&index) => {
          frame.file_canvas_cache_index = index;
        },
        None => {
          let index = sprite_cache_lookup.len();
          let file = frame.file.clone();
          frame.file_canvas_cache_index = index;
          sprite_cache_lookup.insert(file.clone(), index);
          sprite_cache_order.push(file);
        }
      }
    });
  });

  if print_fmd_trace { log!("+ initialize the quest node states"); }
  quest_nodes.iter().for_each(|&quest_index| {
    if print_fmd_trace { log!("++ state loop"); }
    // If the config specified an initial state then just roll with that
    let mut changed = true;
    while changed {
      if print_fmd_trace { log!("+++ state inner loop"); }
      // Repeat the process until there's no further changes. This loop is guaranteed to halt.
      changed = false;
      if nodes[quest_index].current_state == ConfigNodeState::Waiting {
        if nodes[quest_index].unlocks_after_by_index.iter().all(|&other_index| nodes[other_index].current_state == ConfigNodeState::Finished) {
          if print_fmd_trace { log!("+++ Quest `{}` is available because `{:?}` are all finished", nodes[quest_index].name, nodes[quest_index].unlocks_after_by_name); }
          nodes[quest_index].current_state = ConfigNodeState::Available;
          changed = true;
        }
      }
    }
  });

  if print_fmd_trace { log!("+ initialize the part node states"); }
  quest_nodes.iter().for_each(|&quest_index| {
    // Clone the list of numbers because otherwise it moves. So be it.
    if print_fmd_trace { log!("++ Quest Part {} is {:?} and would enable parts {:?} ({:?})", nodes[quest_index].name, nodes[quest_index].current_state, nodes[quest_index].starting_part_by_name, nodes[quest_index].starting_part_by_index); }
    if nodes[quest_index].current_state != ConfigNodeState::Waiting {
      nodes[quest_index].starting_part_by_index.clone().iter().for_each(|&part_index| {
        if print_fmd_trace { log!("+++ Part {} is available because Quest {} is available", nodes[part_index].name, nodes[quest_index].name); }
        nodes[part_index].current_state = ConfigNodeState::Available;
      });
    }
  });

  if print_fmd_trace { log!("Available Quests and Parts from the start:"); }
  nodes.iter().for_each(|node| {
    if node.current_state != ConfigNodeState::Waiting {
      match node.kind {
        ConfigNodeKind::Part => log!("- Part {} will be {:?} from the start", node.raw_name, node.current_state),
        ConfigNodeKind::Quest => log!("- Quest {} will be {:?} from the start", node.raw_name, node.current_state),
        ConfigNodeKind::Demand => {}
        ConfigNodeKind::Supply => {}
        ConfigNodeKind::Dock => {}
        ConfigNodeKind::Machine => {}
        ConfigNodeKind::Belt => {}
      }
    }
  });

  // log!("parsed nodes: {:?}", &nodes[1..]);
  if print_fmd_trace { log!("parsed map: {:?}", node_name_to_index); }
  if print_fmd_trace { node_pattern_to_index.iter_mut().for_each(|(str, &mut kind)| log!("- node_pattern_to_index: {} = pattern({}) -> kind: {}", nodes[kind].raw_name, str, kind)); }

  return Config { nodes, quest_nodes, part_nodes, node_name_to_index, node_pattern_to_index, sprite_cache_lookup, sprite_cache_order, sprite_cache_canvas: vec!() };
}

fn config_full_node_name_to_target_index(name: &str, kind: &str, def_index: usize) -> usize {
  return match name {
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
    "Belt_None" => CONFIG_NODE_BELT_NONE,
    "Belt_Unknown" => CONFIG_NODE_BELT_UNKNOWN,
    "Belt_Invalid" => CONFIG_NODE_BELT_INVALID,
    "Belt_L_" => CONFIG_NODE_BELT_L_,
    "Belt__L" => CONFIG_NODE_BELT__L,
    "Belt___L" => CONFIG_NODE_BELT___L,
    "Belt_D_" => CONFIG_NODE_BELT_D_,
    "Belt_DL_" => CONFIG_NODE_BELT_DL_,
    "Belt_D_L" => CONFIG_NODE_BELT_D_L,
    "Belt_D__L" => CONFIG_NODE_BELT_D__L,
    "Belt__D" => CONFIG_NODE_BELT__D,
    "Belt_L_D" => CONFIG_NODE_BELT_L_D,
    "Belt__DL" => CONFIG_NODE_BELT__DL,
    "Belt__D_L" => CONFIG_NODE_BELT__D_L,
    "Belt___D" => CONFIG_NODE_BELT___D,
    "Belt_L__D" => CONFIG_NODE_BELT_L__D,
    "Belt__L_D" => CONFIG_NODE_BELT__L_D,
    "Belt___DL" => CONFIG_NODE_BELT___DL,
    "Belt_R_" => CONFIG_NODE_BELT_R_,
    "Belt_LR_" => CONFIG_NODE_BELT_LR_,
    "Belt_R_L" => CONFIG_NODE_BELT_R_L,
    "Belt_R__L" => CONFIG_NODE_BELT_R__L,
    "Belt_DR_" => CONFIG_NODE_BELT_DR_,
    "Belt_DLR_" => CONFIG_NODE_BELT_DLR_,
    "Belt_DR_L" => CONFIG_NODE_BELT_DR_L,
    "Belt_DR__L" => CONFIG_NODE_BELT_DR__L,
    "Belt_R_D" => CONFIG_NODE_BELT_R_D,
    "Belt_LR_D" => CONFIG_NODE_BELT_LR_D,
    "Belt_R_DL" => CONFIG_NODE_BELT_R_DL,
    "Belt_R_D_L" => CONFIG_NODE_BELT_R_D_L,
    "Belt_R__D" => CONFIG_NODE_BELT_R__D,
    "Belt_LR__D" => CONFIG_NODE_BELT_LR__D,
    "Belt_R_L_D" => CONFIG_NODE_BELT_R_L_D,
    "Belt_R__DL" => CONFIG_NODE_BELT_R__DL,
    "Belt__R" => CONFIG_NODE_BELT__R,
    "Belt_L_R" => CONFIG_NODE_BELT_L_R,
    "Belt__LR" => CONFIG_NODE_BELT__LR,
    "Belt__R_L" => CONFIG_NODE_BELT__R_L,
    "Belt_D_R" => CONFIG_NODE_BELT_D_R,
    "Belt_DL_R" => CONFIG_NODE_BELT_DL_R,
    "Belt_D_LR" => CONFIG_NODE_BELT_D_LR,
    "Belt_D_R_L" => CONFIG_NODE_BELT_D_R_L,
    "Belt__DR" => CONFIG_NODE_BELT__DR,
    "Belt_L_DR" => CONFIG_NODE_BELT_L_DR,
    "Belt__DLR" => CONFIG_NODE_BELT__DLR,
    "Belt__DR_L" => CONFIG_NODE_BELT__DR_L,
    "Belt__R_D" => CONFIG_NODE_BELT__R_D,
    "Belt_L_R_D" => CONFIG_NODE_BELT_L_R_D,
    "Belt__LR_D" => CONFIG_NODE_BELT__LR_D,
    "Belt__R_DL" => CONFIG_NODE_BELT__R_DL,
    "Belt___R" => CONFIG_NODE_BELT___R,
    "Belt_L__R" => CONFIG_NODE_BELT_L__R,
    "Belt__L_R" => CONFIG_NODE_BELT__L_R,
    "Belt___LR" => CONFIG_NODE_BELT___LR,
    "Belt_D__R" => CONFIG_NODE_BELT_D__R,
    "Belt_DL__R" => CONFIG_NODE_BELT_DL__R,
    "Belt_D_L_R" => CONFIG_NODE_BELT_D_L_R,
    "Belt_D__LR" => CONFIG_NODE_BELT_D__LR,
    "Belt__D_R" => CONFIG_NODE_BELT__D_R,
    "Belt_L_D_R" => CONFIG_NODE_BELT_L_D_R,
    "Belt__DL_R" => CONFIG_NODE_BELT__DL_R,
    "Belt__D_LR" => CONFIG_NODE_BELT__D_LR,
    "Belt___DR" => CONFIG_NODE_BELT___DR,
    "Belt_L__DR" => CONFIG_NODE_BELT_L__DR,
    "Belt__L_DR" => CONFIG_NODE_BELT__L_DR,
    "Belt___DLR" => CONFIG_NODE_BELT___DLR,
    "Belt_U_" => CONFIG_NODE_BELT_U_,
    "Belt_LU_" => CONFIG_NODE_BELT_LU_,
    "Belt_U_L" => CONFIG_NODE_BELT_U_L,
    "Belt_U__L" => CONFIG_NODE_BELT_U__L,
    "Belt_DU_" => CONFIG_NODE_BELT_DU_,
    "Belt_DLU_" => CONFIG_NODE_BELT_DLU_,
    "Belt_DU_L" => CONFIG_NODE_BELT_DU_L,
    "Belt_DU__L" => CONFIG_NODE_BELT_DU__L,
    "Belt_U_D" => CONFIG_NODE_BELT_U_D,
    "Belt_LU_D" => CONFIG_NODE_BELT_LU_D,
    "Belt_U_DL" => CONFIG_NODE_BELT_U_DL,
    "Belt_U_D_L" => CONFIG_NODE_BELT_U_D_L,
    "Belt_U__D" => CONFIG_NODE_BELT_U__D,
    "Belt_LU__D" => CONFIG_NODE_BELT_LU__D,
    "Belt_U_L_D" => CONFIG_NODE_BELT_U_L_D,
    "Belt_U__DL" => CONFIG_NODE_BELT_U__DL,
    "Belt_RU_" => CONFIG_NODE_BELT_RU_,
    "Belt_LRU_" => CONFIG_NODE_BELT_LRU_,
    "Belt_RU_L" => CONFIG_NODE_BELT_RU_L,
    "Belt_RU__L" => CONFIG_NODE_BELT_RU__L,
    "Belt_DRU_" => CONFIG_NODE_BELT_DRU_,
    "Belt_DLRU_" => CONFIG_NODE_BELT_DLRU_,
    "Belt_DRU_L" => CONFIG_NODE_BELT_DRU_L,
    "Belt_DRU__L" => CONFIG_NODE_BELT_DRU__L,
    "Belt_RU_D" => CONFIG_NODE_BELT_RU_D,
    "Belt_LRU_D" => CONFIG_NODE_BELT_LRU_D,
    "Belt_RU_DL" => CONFIG_NODE_BELT_RU_DL,
    "Belt_RU_D_L" => CONFIG_NODE_BELT_RU_D_L,
    "Belt_RU__D" => CONFIG_NODE_BELT_RU__D,
    "Belt_LRU__D" => CONFIG_NODE_BELT_LRU__D,
    "Belt_RU_L_D" => CONFIG_NODE_BELT_RU_L_D,
    "Belt_RU__DL" => CONFIG_NODE_BELT_RU__DL,
    "Belt_U_R" => CONFIG_NODE_BELT_U_R,
    "Belt_LU_R" => CONFIG_NODE_BELT_LU_R,
    "Belt_U_LR" => CONFIG_NODE_BELT_U_LR,
    "Belt_U_R_L" => CONFIG_NODE_BELT_U_R_L,
    "Belt_DU_R" => CONFIG_NODE_BELT_DU_R,
    "Belt_DLU_R" => CONFIG_NODE_BELT_DLU_R,
    "Belt_DU_LR" => CONFIG_NODE_BELT_DU_LR,
    "Belt_DU_R_L" => CONFIG_NODE_BELT_DU_R_L,
    "Belt_U_DR" => CONFIG_NODE_BELT_U_DR,
    "Belt_LU_DR" => CONFIG_NODE_BELT_LU_DR,
    "Belt_U_DLR" => CONFIG_NODE_BELT_U_DLR,
    "Belt_U_DR_L" => CONFIG_NODE_BELT_U_DR_L,
    "Belt_U_R_D" => CONFIG_NODE_BELT_U_R_D,
    "Belt_LU_R_D" => CONFIG_NODE_BELT_LU_R_D,
    "Belt_U_LR_D" => CONFIG_NODE_BELT_U_LR_D,
    "Belt_U_R_DL" => CONFIG_NODE_BELT_U_R_DL,
    "Belt_U__R" => CONFIG_NODE_BELT_U__R,
    "Belt_LU__R" => CONFIG_NODE_BELT_LU__R,
    "Belt_U_L_R" => CONFIG_NODE_BELT_U_L_R,
    "Belt_U__LR" => CONFIG_NODE_BELT_U__LR,
    "Belt_DU__R" => CONFIG_NODE_BELT_DU__R,
    "Belt_DLU__R" => CONFIG_NODE_BELT_DLU__R,
    "Belt_DU_L_R" => CONFIG_NODE_BELT_DU_L_R,
    "Belt_DU__LR" => CONFIG_NODE_BELT_DU__LR,
    "Belt_U_D_R" => CONFIG_NODE_BELT_U_D_R,
    "Belt_LU_D_R" => CONFIG_NODE_BELT_LU_D_R,
    "Belt_U_DL_R" => CONFIG_NODE_BELT_U_DL_R,
    "Belt_U_D_LR" => CONFIG_NODE_BELT_U_D_LR,
    "Belt_U__DR" => CONFIG_NODE_BELT_U__DR,
    "Belt_LU__DR" => CONFIG_NODE_BELT_LU__DR,
    "Belt_U_L_DR" => CONFIG_NODE_BELT_U_L_DR,
    "Belt_U__DLR" => CONFIG_NODE_BELT_U__DLR,
    "Belt__U" => CONFIG_NODE_BELT__U,
    "Belt_L_U" => CONFIG_NODE_BELT_L_U,
    "Belt__LU" => CONFIG_NODE_BELT__LU,
    "Belt__U_L" => CONFIG_NODE_BELT__U_L,
    "Belt_D_U" => CONFIG_NODE_BELT_D_U,
    "Belt_DL_U" => CONFIG_NODE_BELT_DL_U,
    "Belt_D_LU" => CONFIG_NODE_BELT_D_LU,
    "Belt_D_U_L" => CONFIG_NODE_BELT_D_U_L,
    "Belt__DU" => CONFIG_NODE_BELT__DU,
    "Belt_L_DU" => CONFIG_NODE_BELT_L_DU,
    "Belt__DLU" => CONFIG_NODE_BELT__DLU,
    "Belt__DU_L" => CONFIG_NODE_BELT__DU_L,
    "Belt__U_D" => CONFIG_NODE_BELT__U_D,
    "Belt_L_U_D" => CONFIG_NODE_BELT_L_U_D,
    "Belt__LU_D" => CONFIG_NODE_BELT__LU_D,
    "Belt__U_DL" => CONFIG_NODE_BELT__U_DL,
    "Belt_R_U" => CONFIG_NODE_BELT_R_U,
    "Belt_LR_U" => CONFIG_NODE_BELT_LR_U,
    "Belt_R_LU" => CONFIG_NODE_BELT_R_LU,
    "Belt_R_U_L" => CONFIG_NODE_BELT_R_U_L,
    "Belt_DR_U" => CONFIG_NODE_BELT_DR_U,
    "Belt_DLR_U" => CONFIG_NODE_BELT_DLR_U,
    "Belt_DR_LU" => CONFIG_NODE_BELT_DR_LU,
    "Belt_DR_U_L" => CONFIG_NODE_BELT_DR_U_L,
    "Belt_R_DU" => CONFIG_NODE_BELT_R_DU,
    "Belt_LR_DU" => CONFIG_NODE_BELT_LR_DU,
    "Belt_R_DLU" => CONFIG_NODE_BELT_R_DLU,
    "Belt_R_DU_L" => CONFIG_NODE_BELT_R_DU_L,
    "Belt_R_U_D" => CONFIG_NODE_BELT_R_U_D,
    "Belt_LR_U_D" => CONFIG_NODE_BELT_LR_U_D,
    "Belt_R_LU_D" => CONFIG_NODE_BELT_R_LU_D,
    "Belt_R_U_DL" => CONFIG_NODE_BELT_R_U_DL,
    "Belt__RU" => CONFIG_NODE_BELT__RU,
    "Belt_L_RU" => CONFIG_NODE_BELT_L_RU,
    "Belt__LRU" => CONFIG_NODE_BELT__LRU,
    "Belt__RU_L" => CONFIG_NODE_BELT__RU_L,
    "Belt_D_RU" => CONFIG_NODE_BELT_D_RU,
    "Belt_DL_RU" => CONFIG_NODE_BELT_DL_RU,
    "Belt_D_LRU" => CONFIG_NODE_BELT_D_LRU,
    "Belt_D_RU_L" => CONFIG_NODE_BELT_D_RU_L,
    "Belt__DRU" => CONFIG_NODE_BELT__DRU,
    "Belt_L_DRU" => CONFIG_NODE_BELT_L_DRU,
    "Belt__DLRU" => CONFIG_NODE_BELT__DLRU,
    "Belt__DRU_L" => CONFIG_NODE_BELT__DRU_L,
    "Belt__RU_D" => CONFIG_NODE_BELT__RU_D,
    "Belt_L_RU_D" => CONFIG_NODE_BELT_L_RU_D,
    "Belt__LRU_D" => CONFIG_NODE_BELT__LRU_D,
    "Belt__RU_DL" => CONFIG_NODE_BELT__RU_DL,
    "Belt__U_R" => CONFIG_NODE_BELT__U_R,
    "Belt_L_U_R" => CONFIG_NODE_BELT_L_U_R,
    "Belt__LU_R" => CONFIG_NODE_BELT__LU_R,
    "Belt__U_LR" => CONFIG_NODE_BELT__U_LR,
    "Belt_D_U_R" => CONFIG_NODE_BELT_D_U_R,
    "Belt_DL_U_R" => CONFIG_NODE_BELT_DL_U_R,
    "Belt_D_LU_R" => CONFIG_NODE_BELT_D_LU_R,
    "Belt_D_U_LR" => CONFIG_NODE_BELT_D_U_LR,
    "Belt__DU_R" => CONFIG_NODE_BELT__DU_R,
    "Belt_L_DU_R" => CONFIG_NODE_BELT_L_DU_R,
    "Belt__DLU_R" => CONFIG_NODE_BELT__DLU_R,
    "Belt__DU_LR" => CONFIG_NODE_BELT__DU_LR,
    "Belt__U_DR" => CONFIG_NODE_BELT__U_DR,
    "Belt_L_U_DR" => CONFIG_NODE_BELT_L_U_DR,
    "Belt__LU_DR" => CONFIG_NODE_BELT__LU_DR,
    "Belt__U_DLR" => CONFIG_NODE_BELT__U_DLR,
    "Belt___U" => CONFIG_NODE_BELT___U,
    "Belt_L__U" => CONFIG_NODE_BELT_L__U,
    "Belt__L_U" => CONFIG_NODE_BELT__L_U,
    "Belt___LU" => CONFIG_NODE_BELT___LU,
    "Belt_D__U" => CONFIG_NODE_BELT_D__U,
    "Belt_DL__U" => CONFIG_NODE_BELT_DL__U,
    "Belt_D_L_U" => CONFIG_NODE_BELT_D_L_U,
    "Belt_D__LU" => CONFIG_NODE_BELT_D__LU,
    "Belt__D_U" => CONFIG_NODE_BELT__D_U,
    "Belt_L_D_U" => CONFIG_NODE_BELT_L_D_U,
    "Belt__DL_U" => CONFIG_NODE_BELT__DL_U,
    "Belt__D_LU" => CONFIG_NODE_BELT__D_LU,
    "Belt___DU" => CONFIG_NODE_BELT___DU,
    "Belt_L__DU" => CONFIG_NODE_BELT_L__DU,
    "Belt__L_DU" => CONFIG_NODE_BELT__L_DU,
    "Belt___DLU" => CONFIG_NODE_BELT___DLU,
    "Belt_R__U" => CONFIG_NODE_BELT_R__U,
    "Belt_LR__U" => CONFIG_NODE_BELT_LR__U,
    "Belt_R_L_U" => CONFIG_NODE_BELT_R_L_U,
    "Belt_R__LU" => CONFIG_NODE_BELT_R__LU,
    "Belt_DR__U" => CONFIG_NODE_BELT_DR__U,
    "Belt_DLR__U" => CONFIG_NODE_BELT_DLR__U,
    "Belt_DR_L_U" => CONFIG_NODE_BELT_DR_L_U,
    "Belt_DR__LU" => CONFIG_NODE_BELT_DR__LU,
    "Belt_R_D_U" => CONFIG_NODE_BELT_R_D_U,
    "Belt_LR_D_U" => CONFIG_NODE_BELT_LR_D_U,
    "Belt_R_DL_U" => CONFIG_NODE_BELT_R_DL_U,
    "Belt_R_D_LU" => CONFIG_NODE_BELT_R_D_LU,
    "Belt_R__DU" => CONFIG_NODE_BELT_R__DU,
    "Belt_LR__DU" => CONFIG_NODE_BELT_LR__DU,
    "Belt_R_L_DU" => CONFIG_NODE_BELT_R_L_DU,
    "Belt_R__DLU" => CONFIG_NODE_BELT_R__DLU,
    "Belt__R_U" => CONFIG_NODE_BELT__R_U,
    "Belt_L_R_U" => CONFIG_NODE_BELT_L_R_U,
    "Belt__LR_U" => CONFIG_NODE_BELT__LR_U,
    "Belt__R_LU" => CONFIG_NODE_BELT__R_LU,
    "Belt_D_R_U" => CONFIG_NODE_BELT_D_R_U,
    "Belt_DL_R_U" => CONFIG_NODE_BELT_DL_R_U,
    "Belt_D_LR_U" => CONFIG_NODE_BELT_D_LR_U,
    "Belt_D_R_LU" => CONFIG_NODE_BELT_D_R_LU,
    "Belt__DR_U" => CONFIG_NODE_BELT__DR_U,
    "Belt_L_DR_U" => CONFIG_NODE_BELT_L_DR_U,
    "Belt__DLR_U" => CONFIG_NODE_BELT__DLR_U,
    "Belt__DR_LU" => CONFIG_NODE_BELT__DR_LU,
    "Belt__R_DU" => CONFIG_NODE_BELT__R_DU,
    "Belt_L_R_DU" => CONFIG_NODE_BELT_L_R_DU,
    "Belt__LR_DU" => CONFIG_NODE_BELT__LR_DU,
    "Belt__R_DLU" => CONFIG_NODE_BELT__R_DLU,
    "Belt___RU" => CONFIG_NODE_BELT___RU,
    "Belt_L__RU" => CONFIG_NODE_BELT_L__RU,
    "Belt__L_RU" => CONFIG_NODE_BELT__L_RU,
    "Belt___LRU" => CONFIG_NODE_BELT___LRU,
    "Belt_D__RU" => CONFIG_NODE_BELT_D__RU,
    "Belt_DL__RU" => CONFIG_NODE_BELT_DL__RU,
    "Belt_D_L_RU" => CONFIG_NODE_BELT_D_L_RU,
    "Belt_D__LRU" => CONFIG_NODE_BELT_D__LRU,
    "Belt__D_RU" => CONFIG_NODE_BELT__D_RU,
    "Belt_L_D_RU" => CONFIG_NODE_BELT_L_D_RU,
    "Belt__DL_RU" => CONFIG_NODE_BELT__DL_RU,
    "Belt__D_LRU" => CONFIG_NODE_BELT__D_LRU,
    "Belt___DRU" => CONFIG_NODE_BELT___DRU,
    "Belt_L__DRU" => CONFIG_NODE_BELT_L__DRU,
    "Belt__L_DRU" => CONFIG_NODE_BELT__L_DRU,
    "Belt___DLRU" => CONFIG_NODE_BELT___DLRU,
    _ => {
      if !name.starts_with("Part_") && !name.starts_with("Quest_") {
        log!("Warning: {} did not match a known node name and was not Quest or Part! assigning fresh index: {}", name, def_index);
      }
      if kind != "Part" && kind != "Quest" {
        panic!("Only expecting parts and quests to be of unknown node types. Detected kind as `{}` for `{}`", kind, name);
      }
      // If not known then return the next index (nodes.len())
      def_index
    },
  };
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
    config_node_machine(CONFIG_NODE_MACHINE_1X1, "1x1", "./img/machine_1_1.png"),
    config_node_machine(CONFIG_NODE_MACHINE_2X2, "2x2", "./img/machine_2_2.png"),
    config_node_machine(CONFIG_NODE_MACHINE_3X3, "3x3", "./img/machine_1_1.png"),
    config_node_belt(CONFIG_NODE_BELT_NONE, "NONE"),
    config_node_belt(CONFIG_NODE_BELT_UNKNOWN, "UNKNOWN"),
    config_node_belt(CONFIG_NODE_BELT_INVALID, "INVALID"),
    config_node_belt(CONFIG_NODE_BELT_L_, "L_"),
    config_node_belt(CONFIG_NODE_BELT__L, "_L"),
    config_node_belt(CONFIG_NODE_BELT___L, "__L"),
    config_node_belt(CONFIG_NODE_BELT_D_, "D_"),
    config_node_belt(CONFIG_NODE_BELT_DL_, "DL_"),
    config_node_belt(CONFIG_NODE_BELT_D_L, "D_L"),
    config_node_belt(CONFIG_NODE_BELT_D__L, "D__L"),
    config_node_belt(CONFIG_NODE_BELT__D, "_D"),
    config_node_belt(CONFIG_NODE_BELT_L_D, "L_D"),
    config_node_belt(CONFIG_NODE_BELT__DL, "_DL"),
    config_node_belt(CONFIG_NODE_BELT__D_L, "_D_L"),
    config_node_belt(CONFIG_NODE_BELT___D, "__D"),
    config_node_belt(CONFIG_NODE_BELT_L__D, "L__D"),
    config_node_belt(CONFIG_NODE_BELT__L_D, "_L_D"),
    config_node_belt(CONFIG_NODE_BELT___DL, "__DL"),
    config_node_belt(CONFIG_NODE_BELT_R_, "R_"),
    config_node_belt(CONFIG_NODE_BELT_LR_, "LR_"),
    config_node_belt(CONFIG_NODE_BELT_R_L, "R_L"),
    config_node_belt(CONFIG_NODE_BELT_R__L, "R__L"),
    config_node_belt(CONFIG_NODE_BELT_DR_, "DR_"),
    config_node_belt(CONFIG_NODE_BELT_DLR_, "DLR_"),
    config_node_belt(CONFIG_NODE_BELT_DR_L, "DR_L"),
    config_node_belt(CONFIG_NODE_BELT_DR__L, "DR__L"),
    config_node_belt(CONFIG_NODE_BELT_R_D, "R_D"),
    config_node_belt(CONFIG_NODE_BELT_LR_D, "LR_D"),
    config_node_belt(CONFIG_NODE_BELT_R_DL, "R_DL"),
    config_node_belt(CONFIG_NODE_BELT_R_D_L, "R_D_L"),
    config_node_belt(CONFIG_NODE_BELT_R__D, "R__D"),
    config_node_belt(CONFIG_NODE_BELT_LR__D, "LR__D"),
    config_node_belt(CONFIG_NODE_BELT_R_L_D, "R_L_D"),
    config_node_belt(CONFIG_NODE_BELT_R__DL, "R__DL"),
    config_node_belt(CONFIG_NODE_BELT__R, "_R"),
    config_node_belt(CONFIG_NODE_BELT_L_R, "L_R"),
    config_node_belt(CONFIG_NODE_BELT__LR, "_LR"),
    config_node_belt(CONFIG_NODE_BELT__R_L, "_R_L"),
    config_node_belt(CONFIG_NODE_BELT_D_R, "D_R"),
    config_node_belt(CONFIG_NODE_BELT_DL_R, "DL_R"),
    config_node_belt(CONFIG_NODE_BELT_D_LR, "D_LR"),
    config_node_belt(CONFIG_NODE_BELT_D_R_L, "D_R_L"),
    config_node_belt(CONFIG_NODE_BELT__DR, "_DR"),
    config_node_belt(CONFIG_NODE_BELT_L_DR, "L_DR"),
    config_node_belt(CONFIG_NODE_BELT__DLR, "_DLR"),
    config_node_belt(CONFIG_NODE_BELT__DR_L, "_DR_L"),
    config_node_belt(CONFIG_NODE_BELT__R_D, "_R_D"),
    config_node_belt(CONFIG_NODE_BELT_L_R_D, "L_R_D"),
    config_node_belt(CONFIG_NODE_BELT__LR_D, "_LR_D"),
    config_node_belt(CONFIG_NODE_BELT__R_DL, "_R_DL"),
    config_node_belt(CONFIG_NODE_BELT___R, "__R"),
    config_node_belt(CONFIG_NODE_BELT_L__R, "L__R"),
    config_node_belt(CONFIG_NODE_BELT__L_R, "_L_R"),
    config_node_belt(CONFIG_NODE_BELT___LR, "__LR"),
    config_node_belt(CONFIG_NODE_BELT_D__R, "D__R"),
    config_node_belt(CONFIG_NODE_BELT_DL__R, "DL__R"),
    config_node_belt(CONFIG_NODE_BELT_D_L_R, "D_L_R"),
    config_node_belt(CONFIG_NODE_BELT_D__LR, "D__LR"),
    config_node_belt(CONFIG_NODE_BELT__D_R, "_D_R"),
    config_node_belt(CONFIG_NODE_BELT_L_D_R, "L_D_R"),
    config_node_belt(CONFIG_NODE_BELT__DL_R, "_DL_R"),
    config_node_belt(CONFIG_NODE_BELT__D_LR, "_D_LR"),
    config_node_belt(CONFIG_NODE_BELT___DR, "__DR"),
    config_node_belt(CONFIG_NODE_BELT_L__DR, "L__DR"),
    config_node_belt(CONFIG_NODE_BELT__L_DR, "_L_DR"),
    config_node_belt(CONFIG_NODE_BELT___DLR, "__DLR"),
    config_node_belt(CONFIG_NODE_BELT_U_, "U_"),
    config_node_belt(CONFIG_NODE_BELT_LU_, "LU_"),
    config_node_belt(CONFIG_NODE_BELT_U_L, "U_L"),
    config_node_belt(CONFIG_NODE_BELT_U__L, "U__L"),
    config_node_belt(CONFIG_NODE_BELT_DU_, "DU_"),
    config_node_belt(CONFIG_NODE_BELT_DLU_, "DLU_"),
    config_node_belt(CONFIG_NODE_BELT_DU_L, "DU_L"),
    config_node_belt(CONFIG_NODE_BELT_DU__L, "DU__L"),
    config_node_belt(CONFIG_NODE_BELT_U_D, "U_D"),
    config_node_belt(CONFIG_NODE_BELT_LU_D, "LU_D"),
    config_node_belt(CONFIG_NODE_BELT_U_DL, "U_DL"),
    config_node_belt(CONFIG_NODE_BELT_U_D_L, "U_D_L"),
    config_node_belt(CONFIG_NODE_BELT_U__D, "U__D"),
    config_node_belt(CONFIG_NODE_BELT_LU__D, "LU__D"),
    config_node_belt(CONFIG_NODE_BELT_U_L_D, "U_L_D"),
    config_node_belt(CONFIG_NODE_BELT_U__DL, "U__DL"),
    config_node_belt(CONFIG_NODE_BELT_RU_, "RU_"),
    config_node_belt(CONFIG_NODE_BELT_LRU_, "LRU_"),
    config_node_belt(CONFIG_NODE_BELT_RU_L, "RU_L"),
    config_node_belt(CONFIG_NODE_BELT_RU__L, "RU__L"),
    config_node_belt(CONFIG_NODE_BELT_DRU_, "DRU_"),
    config_node_belt(CONFIG_NODE_BELT_DLRU_, "DLRU_"),
    config_node_belt(CONFIG_NODE_BELT_DRU_L, "DRU_L"),
    config_node_belt(CONFIG_NODE_BELT_DRU__L, "DRU__L"),
    config_node_belt(CONFIG_NODE_BELT_RU_D, "RU_D"),
    config_node_belt(CONFIG_NODE_BELT_LRU_D, "LRU_D"),
    config_node_belt(CONFIG_NODE_BELT_RU_DL, "RU_DL"),
    config_node_belt(CONFIG_NODE_BELT_RU_D_L, "RU_D_L"),
    config_node_belt(CONFIG_NODE_BELT_RU__D, "RU__D"),
    config_node_belt(CONFIG_NODE_BELT_LRU__D, "LRU__D"),
    config_node_belt(CONFIG_NODE_BELT_RU_L_D, "RU_L_D"),
    config_node_belt(CONFIG_NODE_BELT_RU__DL, "RU__DL"),
    config_node_belt(CONFIG_NODE_BELT_U_R, "U_R"),
    config_node_belt(CONFIG_NODE_BELT_LU_R, "LU_R"),
    config_node_belt(CONFIG_NODE_BELT_U_LR, "U_LR"),
    config_node_belt(CONFIG_NODE_BELT_U_R_L, "U_R_L"),
    config_node_belt(CONFIG_NODE_BELT_DU_R, "DU_R"),
    config_node_belt(CONFIG_NODE_BELT_DLU_R, "DLU_R"),
    config_node_belt(CONFIG_NODE_BELT_DU_LR, "DU_LR"),
    config_node_belt(CONFIG_NODE_BELT_DU_R_L, "DU_R_L"),
    config_node_belt(CONFIG_NODE_BELT_U_DR, "U_DR"),
    config_node_belt(CONFIG_NODE_BELT_LU_DR, "LU_DR"),
    config_node_belt(CONFIG_NODE_BELT_U_DLR, "U_DLR"),
    config_node_belt(CONFIG_NODE_BELT_U_DR_L, "U_DR_L"),
    config_node_belt(CONFIG_NODE_BELT_U_R_D, "U_R_D"),
    config_node_belt(CONFIG_NODE_BELT_LU_R_D, "LU_R_D"),
    config_node_belt(CONFIG_NODE_BELT_U_LR_D, "U_LR_D"),
    config_node_belt(CONFIG_NODE_BELT_U_R_DL, "U_R_DL"),
    config_node_belt(CONFIG_NODE_BELT_U__R, "U__R"),
    config_node_belt(CONFIG_NODE_BELT_LU__R, "LU__R"),
    config_node_belt(CONFIG_NODE_BELT_U_L_R, "U_L_R"),
    config_node_belt(CONFIG_NODE_BELT_U__LR, "U__LR"),
    config_node_belt(CONFIG_NODE_BELT_DU__R, "DU__R"),
    config_node_belt(CONFIG_NODE_BELT_DLU__R, "DLU__R"),
    config_node_belt(CONFIG_NODE_BELT_DU_L_R, "DU_L_R"),
    config_node_belt(CONFIG_NODE_BELT_DU__LR, "DU__LR"),
    config_node_belt(CONFIG_NODE_BELT_U_D_R, "U_D_R"),
    config_node_belt(CONFIG_NODE_BELT_LU_D_R, "LU_D_R"),
    config_node_belt(CONFIG_NODE_BELT_U_DL_R, "U_DL_R"),
    config_node_belt(CONFIG_NODE_BELT_U_D_LR, "U_D_LR"),
    config_node_belt(CONFIG_NODE_BELT_U__DR, "U__DR"),
    config_node_belt(CONFIG_NODE_BELT_LU__DR, "LU__DR"),
    config_node_belt(CONFIG_NODE_BELT_U_L_DR, "U_L_DR"),
    config_node_belt(CONFIG_NODE_BELT_U__DLR, "U__DLR"),
    config_node_belt(CONFIG_NODE_BELT__U, "_U"),
    config_node_belt(CONFIG_NODE_BELT_L_U, "L_U"),
    config_node_belt(CONFIG_NODE_BELT__LU, "_LU"),
    config_node_belt(CONFIG_NODE_BELT__U_L, "_U_L"),
    config_node_belt(CONFIG_NODE_BELT_D_U, "D_U"),
    config_node_belt(CONFIG_NODE_BELT_DL_U, "DL_U"),
    config_node_belt(CONFIG_NODE_BELT_D_LU, "D_LU"),
    config_node_belt(CONFIG_NODE_BELT_D_U_L, "D_U_L"),
    config_node_belt(CONFIG_NODE_BELT__DU, "_DU"),
    config_node_belt(CONFIG_NODE_BELT_L_DU, "L_DU"),
    config_node_belt(CONFIG_NODE_BELT__DLU, "_DLU"),
    config_node_belt(CONFIG_NODE_BELT__DU_L, "_DU_L"),
    config_node_belt(CONFIG_NODE_BELT__U_D, "_U_D"),
    config_node_belt(CONFIG_NODE_BELT_L_U_D, "L_U_D"),
    config_node_belt(CONFIG_NODE_BELT__LU_D, "_LU_D"),
    config_node_belt(CONFIG_NODE_BELT__U_DL, "_U_DL"),
    config_node_belt(CONFIG_NODE_BELT_R_U, "R_U"),
    config_node_belt(CONFIG_NODE_BELT_LR_U, "LR_U"),
    config_node_belt(CONFIG_NODE_BELT_R_LU, "R_LU"),
    config_node_belt(CONFIG_NODE_BELT_R_U_L, "R_U_L"),
    config_node_belt(CONFIG_NODE_BELT_DR_U, "DR_U"),
    config_node_belt(CONFIG_NODE_BELT_DLR_U, "DLR_U"),
    config_node_belt(CONFIG_NODE_BELT_DR_LU, "DR_LU"),
    config_node_belt(CONFIG_NODE_BELT_DR_U_L, "DR_U_L"),
    config_node_belt(CONFIG_NODE_BELT_R_DU, "R_DU"),
    config_node_belt(CONFIG_NODE_BELT_LR_DU, "LR_DU"),
    config_node_belt(CONFIG_NODE_BELT_R_DLU, "R_DLU"),
    config_node_belt(CONFIG_NODE_BELT_R_DU_L, "R_DU_L"),
    config_node_belt(CONFIG_NODE_BELT_R_U_D, "R_U_D"),
    config_node_belt(CONFIG_NODE_BELT_LR_U_D, "LR_U_D"),
    config_node_belt(CONFIG_NODE_BELT_R_LU_D, "R_LU_D"),
    config_node_belt(CONFIG_NODE_BELT_R_U_DL, "R_U_DL"),
    config_node_belt(CONFIG_NODE_BELT__RU, "_RU"),
    config_node_belt(CONFIG_NODE_BELT_L_RU, "L_RU"),
    config_node_belt(CONFIG_NODE_BELT__LRU, "_LRU"),
    config_node_belt(CONFIG_NODE_BELT__RU_L, "_RU_L"),
    config_node_belt(CONFIG_NODE_BELT_D_RU, "D_RU"),
    config_node_belt(CONFIG_NODE_BELT_DL_RU, "DL_RU"),
    config_node_belt(CONFIG_NODE_BELT_D_LRU, "D_LRU"),
    config_node_belt(CONFIG_NODE_BELT_D_RU_L, "D_RU_L"),
    config_node_belt(CONFIG_NODE_BELT__DRU, "_DRU"),
    config_node_belt(CONFIG_NODE_BELT_L_DRU, "L_DRU"),
    config_node_belt(CONFIG_NODE_BELT__DLRU, "_DLRU"),
    config_node_belt(CONFIG_NODE_BELT__DRU_L, "_DRU_L"),
    config_node_belt(CONFIG_NODE_BELT__RU_D, "_RU_D"),
    config_node_belt(CONFIG_NODE_BELT_L_RU_D, "L_RU_D"),
    config_node_belt(CONFIG_NODE_BELT__LRU_D, "_LRU_D"),
    config_node_belt(CONFIG_NODE_BELT__RU_DL, "_RU_DL"),
    config_node_belt(CONFIG_NODE_BELT__U_R, "_U_R"),
    config_node_belt(CONFIG_NODE_BELT_L_U_R, "L_U_R"),
    config_node_belt(CONFIG_NODE_BELT__LU_R, "_LU_R"),
    config_node_belt(CONFIG_NODE_BELT__U_LR, "_U_LR"),
    config_node_belt(CONFIG_NODE_BELT_D_U_R, "D_U_R"),
    config_node_belt(CONFIG_NODE_BELT_DL_U_R, "DL_U_R"),
    config_node_belt(CONFIG_NODE_BELT_D_LU_R, "D_LU_R"),
    config_node_belt(CONFIG_NODE_BELT_D_U_LR, "D_U_LR"),
    config_node_belt(CONFIG_NODE_BELT__DU_R, "_DU_R"),
    config_node_belt(CONFIG_NODE_BELT_L_DU_R, "L_DU_R"),
    config_node_belt(CONFIG_NODE_BELT__DLU_R, "_DLU_R"),
    config_node_belt(CONFIG_NODE_BELT__DU_LR, "_DU_LR"),
    config_node_belt(CONFIG_NODE_BELT__U_DR, "_U_DR"),
    config_node_belt(CONFIG_NODE_BELT_L_U_DR, "L_U_DR"),
    config_node_belt(CONFIG_NODE_BELT__LU_DR, "_LU_DR"),
    config_node_belt(CONFIG_NODE_BELT__U_DLR, "_U_DLR"),
    config_node_belt(CONFIG_NODE_BELT___U, "__U"),
    config_node_belt(CONFIG_NODE_BELT_L__U, "L__U"),
    config_node_belt(CONFIG_NODE_BELT__L_U, "_L_U"),
    config_node_belt(CONFIG_NODE_BELT___LU, "__LU"),
    config_node_belt(CONFIG_NODE_BELT_D__U, "D__U"),
    config_node_belt(CONFIG_NODE_BELT_DL__U, "DL__U"),
    config_node_belt(CONFIG_NODE_BELT_D_L_U, "D_L_U"),
    config_node_belt(CONFIG_NODE_BELT_D__LU, "D__LU"),
    config_node_belt(CONFIG_NODE_BELT__D_U, "_D_U"),
    config_node_belt(CONFIG_NODE_BELT_L_D_U, "L_D_U"),
    config_node_belt(CONFIG_NODE_BELT__DL_U, "_DL_U"),
    config_node_belt(CONFIG_NODE_BELT__D_LU, "_D_LU"),
    config_node_belt(CONFIG_NODE_BELT___DU, "__DU"),
    config_node_belt(CONFIG_NODE_BELT_L__DU, "L__DU"),
    config_node_belt(CONFIG_NODE_BELT__L_DU, "_L_DU"),
    config_node_belt(CONFIG_NODE_BELT___DLU, "__DLU"),
    config_node_belt(CONFIG_NODE_BELT_R__U, "R__U"),
    config_node_belt(CONFIG_NODE_BELT_LR__U, "LR__U"),
    config_node_belt(CONFIG_NODE_BELT_R_L_U, "R_L_U"),
    config_node_belt(CONFIG_NODE_BELT_R__LU, "R__LU"),
    config_node_belt(CONFIG_NODE_BELT_DR__U, "DR__U"),
    config_node_belt(CONFIG_NODE_BELT_DLR__U, "DLR__U"),
    config_node_belt(CONFIG_NODE_BELT_DR_L_U, "DR_L_U"),
    config_node_belt(CONFIG_NODE_BELT_DR__LU, "DR__LU"),
    config_node_belt(CONFIG_NODE_BELT_R_D_U, "R_D_U"),
    config_node_belt(CONFIG_NODE_BELT_LR_D_U, "LR_D_U"),
    config_node_belt(CONFIG_NODE_BELT_R_DL_U, "R_DL_U"),
    config_node_belt(CONFIG_NODE_BELT_R_D_LU, "R_D_LU"),
    config_node_belt(CONFIG_NODE_BELT_R__DU, "R__DU"),
    config_node_belt(CONFIG_NODE_BELT_LR__DU, "LR__DU"),
    config_node_belt(CONFIG_NODE_BELT_R_L_DU, "R_L_DU"),
    config_node_belt(CONFIG_NODE_BELT_R__DLU, "R__DLU"),
    config_node_belt(CONFIG_NODE_BELT__R_U, "_R_U"),
    config_node_belt(CONFIG_NODE_BELT_L_R_U, "L_R_U"),
    config_node_belt(CONFIG_NODE_BELT__LR_U, "_LR_U"),
    config_node_belt(CONFIG_NODE_BELT__R_LU, "_R_LU"),
    config_node_belt(CONFIG_NODE_BELT_D_R_U, "D_R_U"),
    config_node_belt(CONFIG_NODE_BELT_DL_R_U, "DL_R_U"),
    config_node_belt(CONFIG_NODE_BELT_D_LR_U, "D_LR_U"),
    config_node_belt(CONFIG_NODE_BELT_D_R_LU, "D_R_LU"),
    config_node_belt(CONFIG_NODE_BELT__DR_U, "_DR_U"),
    config_node_belt(CONFIG_NODE_BELT_L_DR_U, "L_DR_U"),
    config_node_belt(CONFIG_NODE_BELT__DLR_U, "_DLR_U"),
    config_node_belt(CONFIG_NODE_BELT__DR_LU, "_DR_LU"),
    config_node_belt(CONFIG_NODE_BELT__R_DU, "_R_DU"),
    config_node_belt(CONFIG_NODE_BELT_L_R_DU, "L_R_DU"),
    config_node_belt(CONFIG_NODE_BELT__LR_DU, "_LR_DU"),
    config_node_belt(CONFIG_NODE_BELT__R_DLU, "_R_DLU"),
    config_node_belt(CONFIG_NODE_BELT___RU, "__RU"),
    config_node_belt(CONFIG_NODE_BELT_L__RU, "L__RU"),
    config_node_belt(CONFIG_NODE_BELT__L_RU, "_L_RU"),
    config_node_belt(CONFIG_NODE_BELT___LRU, "__LRU"),
    config_node_belt(CONFIG_NODE_BELT_D__RU, "D__RU"),
    config_node_belt(CONFIG_NODE_BELT_DL__RU, "DL__RU"),
    config_node_belt(CONFIG_NODE_BELT_D_L_RU, "D_L_RU"),
    config_node_belt(CONFIG_NODE_BELT_D__LRU, "D__LRU"),
    config_node_belt(CONFIG_NODE_BELT__D_RU, "_D_RU"),
    config_node_belt(CONFIG_NODE_BELT_L_D_RU, "L_D_RU"),
    config_node_belt(CONFIG_NODE_BELT__DL_RU, "_DL_RU"),
    config_node_belt(CONFIG_NODE_BELT__D_LRU, "_D_LRU"),
    config_node_belt(CONFIG_NODE_BELT___DRU, "__DRU"),
    config_node_belt(CONFIG_NODE_BELT_L__DRU, "L__DRU"),
    config_node_belt(CONFIG_NODE_BELT__L_DRU, "_L_DRU"),
    config_node_belt(CONFIG_NODE_BELT___DLRU, "__DLRU"),
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
    pattern: "".to_string(),
    pattern_unique_kinds: vec!(),
    icon,

    sprite_config: SpriteConfig {
      pause_between: 10,
      frame_offset: 0,
      initial_delay: 10,
      looping: true,
      frames: vec!(
        SpriteFrame {
          file: "".to_string(),
          name: "do not use me; part".to_string(),
          file_canvas_cache_index: 0,
          x: 0.0,
          y: 0.0,
          w: 0.0,
          h: 0.0,
        }
      )
    },

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
    pattern_unique_kinds: vec!(),
    production_target_by_name: vec!(),
    production_target_by_index: vec!(),
    pattern_by_index: vec!(),
    pattern_by_name: vec!(),
    pattern_by_icon: vec!(),
    pattern: "".to_string(),
    icon: '?',

    sprite_config: SpriteConfig {
      pause_between: 10,
      frame_offset: 0,
      initial_delay: 10,
      looping: true,
      frames: vec!(
        SpriteFrame {
          file: "./img/supply.png".to_string(),
          name: "do not use me; supply".to_string(),
          file_canvas_cache_index: 0,
          x: 0.0,
          y: 0.0,
          w: 32.0,
          h: 32.0,
        }
      )
    },

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
    pattern_unique_kinds: vec!(),
    production_target_by_name: vec!(),
    production_target_by_index: vec!(),
    pattern_by_index: vec!(),
    pattern_by_name: vec!(),
    pattern_by_icon: vec!(),
    pattern: "".to_string(),
    icon: '?',

    sprite_config: SpriteConfig {
      pause_between: 10,
      frame_offset: 0,
      initial_delay: 10,
      looping: true,
      frames: vec!(
        SpriteFrame {
          file: "./img/demand.png".to_string(),
          name: "do not use me; demand".to_string(),
          file_canvas_cache_index: 0,
          x: 0.0,
          y: 0.0,
          w: 32.0,
          h: 32.0,
        }
      )
    },

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
    pattern_unique_kinds: vec!(),
    production_target_by_name: vec!(),
    production_target_by_index: vec!(),
    pattern_by_index: vec!(),
    pattern_by_name: vec!(),
    pattern_by_icon: vec!(),
    pattern: "".to_string(),
    icon: '?',

    sprite_config: SpriteConfig {
      pause_between: 10,
      frame_offset: 0,
      initial_delay: 10,
      looping: true,
      frames: vec!(
        SpriteFrame {
          file: "./img/dock.png".to_string(),
          name: "do not use me; dock".to_string(),
          file_canvas_cache_index: 0,
          x: 0.0,
          y: 0.0,
          w: 64.0,
          h: 64.0,
        }
      )
    },

    current_state: ConfigNodeState::Available,
  };
}
fn config_node_machine(index: PartKind, name: &str, file: &str) -> ConfigNode {
  let raw_name = format!("Machine_{}", name);
  return ConfigNode {
    index,
    kind: ConfigNodeKind::Machine,
    name: name.to_string(),
    raw_name,
    unlocks_after_by_name: vec!(),
    unlocks_after_by_index: vec!(),
    unlocks_todo_by_index: vec!(),
    starting_part_by_name: vec!(),
    starting_part_by_index: vec!(),
    pattern_unique_kinds: vec!(),
    production_target_by_name: vec!(),
    production_target_by_index: vec!(),
    pattern_by_index: vec!(),
    pattern_by_name: vec!(),
    pattern_by_icon: vec!(),
    pattern: "".to_string(),
    icon: '?',

    sprite_config: SpriteConfig {
      pause_between: 10,
      frame_offset: 0,
      initial_delay: 10,
      looping: true,
      frames: vec!(
        SpriteFrame {
          file: file.to_string(),
          name: "do not use me; machine".to_string(),
          file_canvas_cache_index: 0,
          x: 5.0,
          y: 5.0,
          w: 5.0,
          h: 5.0,
        }
      )
    },

    // This hints where on this machine tile the output part icon of this machine should be painted
    current_state: ConfigNodeState::Available,
  };
}
fn config_node_belt(index: PartKind, name: &str) -> ConfigNode {
  let raw_name = format!("Belt_{}", name);
  let belt_type = belt_name_to_belt_type(name);
  let belt_meta = belt_type_to_belt_meta(belt_type);
  return ConfigNode {
    index,
    kind: ConfigNodeKind::Machine,
    name: name.to_string(),
    raw_name,
    unlocks_after_by_name: vec!(),
    unlocks_after_by_index: vec!(),
    unlocks_todo_by_index: vec!(),
    starting_part_by_name: vec!(),
    starting_part_by_index: vec!(),
    pattern_unique_kinds: vec!(),
    production_target_by_name: vec!(),
    production_target_by_index: vec!(),
    pattern_by_index: vec!(),
    pattern_by_name: vec!(),
    pattern_by_icon: vec!(),
    pattern: "".to_string(),
    icon: '?',

    sprite_config: SpriteConfig {
      pause_between: 10,
      frame_offset: 0,
      initial_delay: 10,
      looping: true,
      frames: vec!(
        SpriteFrame {
          file: belt_meta.src.to_string(),
          name: "do not use me; belt".to_string(),
          file_canvas_cache_index: 0,
          x: 0.0,
          y: 0.0,
          w: 160.0,
          h: 160.0
        }
      )
    },

    current_state: ConfigNodeState::Available,
  };
}

pub fn config_get_sprite_details(config: &Config, config_index: usize, sprite_offset: u64, ticks: u64) -> (f64, f64, f64, f64, &web_sys::HtmlImageElement) {
  assert!(config_index < config.nodes.len(), "config_index should be a node index: {} < {}", config_index, config.nodes.len());
  let node = &config.nodes[config_index];
  let sprite_index = ((((ticks - sprite_offset) + (node.sprite_config.frame_offset * 62)) % ((node.sprite_config.frames.len() as u64) * 62)) / 62) as usize;
  let sprite = &node.sprite_config.frames[sprite_index];
  return ( sprite.x, sprite.y, sprite.w, sprite.h, &config.sprite_cache_canvas[sprite.file_canvas_cache_index] );
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
    convert_js_to_pair("pattern", JsValue::from(node.pattern.clone())),
    convert_js_to_pair("pattern_unique_kinds", convert_vec_usize_to_jsvalue(&node.pattern_unique_kinds)),
    convert_string_to_pair("icon", format!("{}", node.icon).as_str()),

    convert_js_to_pair(
      "sprite_config",
      node.sprite_config.frames.iter().map(|frame| {
        JsValue::from(vec!(
          convert_string_to_pair("file", frame.file.as_str()),
          convert_js_to_pair("file_canvas_cache_index", JsValue::from(frame.file_canvas_cache_index)),
          convert_js_to_pair("x", JsValue::from(frame.x)),
          convert_js_to_pair("y", JsValue::from(frame.y)),
          convert_js_to_pair("w", JsValue::from(frame.w)),
          convert_js_to_pair("h", JsValue::from(frame.h)),
          ).iter().collect::<js_sys::Array>())
      }).collect::<js_sys::Array>().into()
    ),

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

pub fn config_get_sprite_for_belt_type(config: &Config, belt_type: BeltType, sprite_offset: u64, ticks: u64) -> ( f64, f64, f64, f64, &web_sys::HtmlImageElement) {
  return config_get_sprite_details(config, belt_type as usize, sprite_offset, ticks);
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
