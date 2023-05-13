use std::collections::VecDeque;
use js_sys::{Array};

use super::belt::*;
use super::bouncer::*;
use super::cell::*;
use super::cli_serialize::*;
use crate::craft::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::init::*;
use super::machine::*;
use super::options::*;
use super::part::*;
use super::paste::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::truck::*;
use super::utils::*;
use super::zone::*;
use super::log;

pub const UNDO_STACK_SIZE: usize = 100;

pub struct State {
  pub pregame: bool, // Showing main screen or loading screen?
  pub paused: bool,
  pub active_story_index: usize, // Story quest/parts that are currently used
  pub mouse_mode_mirrored: bool, // Note: all this really does is flip the lmb and rmb actions but we need this toggle for touch-only mode
  pub event_type_swapped: bool, // Treat a mouse event like a touch event and a touch event like a mouse event? (Mostly for debugging)
  pub mouse_mode_selecting: bool,
  pub selected_area_copy: Vec<Vec<Cell>>,
  pub test: bool,
  pub lasers: Vec<Laser>,
  pub manual_open: bool,
  pub snapshot_stack: [String; UNDO_STACK_SIZE],
  pub snapshot_pointer: usize,
  pub snapshot_undo_pointer: usize,
  pub examples: Vec<String>,
  pub example_pointer: usize,

  pub reset_next_frame: bool,
  pub load_snapshot_next_frame: bool,
  pub load_example_next_frame: bool,
}

#[derive(Debug)]
pub struct CellSelection {
  pub on: bool,
  pub x: f64,
  pub y: f64,
  pub coord: usize,
  pub area: bool,
  pub x2: f64,
  pub y2: f64,
}

#[derive(Debug)]
pub struct MouseState {
  pub canvas_x: f64,
  pub canvas_y: f64,

  pub world_x: f64,
  pub world_y: f64,
  pub moved_since_start: bool, // The worldxy will be zero until the mouse moves at app start.

  pub cell_x: f64, // unfloored
  pub cell_y: f64, // unfloored
  pub cell_x_floored: f64, // floored
  pub cell_y_floored: f64, // floored

  pub last_cell_x: f64,
  pub last_cell_y: f64,

  pub over_zone: Zone,
  pub down_zone: Zone,
  pub up_zone: Zone,

  pub last_down_event_type: EventSourceType, // MOUSE or TOUCH

  pub is_down: bool, // Set if pointer is currently down. Unset when the pointer is released.
  pub was_down: bool, // Set if current frame handled a pointer down. Should unset after the frame.
  pub is_dragging: bool,
  pub was_dragging: bool,
  pub is_drag_start: bool,
  pub is_up: bool, // Set if current frame handled a mouse up. It may be unset, unlike was_up.
  pub was_up: bool, // Set if current frame handled a mouse up. Should not be unset.

  // TODO: change the hover/down state/location to an enum rather than individual down states for each part of the UI

  pub over_floor_area: bool,
  pub over_floor_not_corner: bool, // Over the floor but not any of the corner cells
  pub down_floor_area: bool,
  pub down_floor_not_corner: bool,

  pub over_quest: bool,
  pub over_quest_visible_index: usize, // Only if over_quote
  pub down_quest: bool,
  pub down_quest_visible_index: usize, // Only if down_quest
  pub up_quote: bool,
  pub up_quest_visible_index: usize, // Only if up_quote

  pub over_menu_row: MenuRow,
  pub over_menu_button: MenuButton,
  pub down_menu_row: MenuRow,
  pub down_menu_button: MenuButton,
  pub up_menu_row: MenuRow,
  pub up_menu_button: MenuButton,

  pub help_hover: bool,
  pub help_down: bool,

  pub offer_hover: bool,
  pub offer_hover_offer_index: usize, // Only relevant when offer_hover = true
  pub offer_down: bool,
  pub offer_down_offer_index: usize, // Kept until the next up, used for dragging
  pub offer_selected: bool,
  pub offer_selected_index: usize, // Offer index, not part index
  pub dragging_offer: bool,
  pub dragging_machine1x2: bool,
  pub dragging_machine2x1: bool,
  pub dragging_machine2x2: bool,
  pub dragging_machine3x3: bool,

  pub craft_over_ci: CraftInteractable,
  pub craft_over_ci_wx: f64,
  pub craft_over_ci_wy: f64,
  pub craft_over_ci_ww: f64,
  pub craft_over_ci_wh: f64,
  pub craft_over_ci_icon: char,
  pub craft_over_ci_index: u8,
  pub craft_over_ci_part_kind: PartKind,
  pub craft_down_ci: CraftInteractable,
  pub craft_down_ci_wx: f64,
  pub craft_down_ci_wy: f64,
  pub craft_down_ci_ww: f64,
  pub craft_down_ci_wh: f64,
  pub craft_down_ci_icon: char,
  pub craft_down_ci_part_kind: PartKind,
  pub craft_down_ci_index: u8,
  pub craft_up_ci: CraftInteractable,
  pub craft_up_ci_wx: f64,
  pub craft_up_ci_wy: f64,
  pub craft_up_ci_ww: f64,
  pub craft_up_ci_wh: f64,
  pub craft_up_ci_icon: char,
  pub craft_up_ci_part_kind: PartKind,
  pub craft_up_ci_index: u8,
  pub craft_dragging_ci: bool, // in this case craft_down_ci_c can tell you what's being dragged

  // https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons
  // bitwise field; 1=left, 2=right, 3=left|right, 4=middle, etc
  // (8 and 16 supposedly browser back/forward button but ehhhh)
  // On a phone/tablet this is not used of course
  pub last_down_button: u16,

  pub last_down_canvas_x: f64,
  pub last_down_canvas_y: f64,
  pub last_down_world_x: f64,
  pub last_down_world_y: f64,
  pub last_down_cell_x: f64,
  pub last_down_cell_y: f64,
  pub last_down_cell_x_floored: f64,
  pub last_down_cell_y_floored: f64,

  pub last_up_canvas_x: f64,
  pub last_up_canvas_y: f64,
  pub last_up_world_x: f64,
  pub last_up_world_y: f64,
  pub last_up_cell_x: f64, // Can be negative (oob), is not floored
  pub last_up_cell_y: f64, // Can be negative (oob), is not floored

  pub over_save_map: bool,
  pub over_save_map_index: usize,
  pub down_save_map: bool,
  pub down_save_map_index: usize,
  pub up_save_map: bool,
  pub up_save_map_index: usize,

  pub over_undo: bool,
  pub down_undo: bool,
  pub up_undo: bool,
  pub over_trash: bool,
  pub down_trash: bool,
  pub up_trash: bool,
  pub over_redo: bool,
  pub down_redo: bool,
  pub up_redo: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuButton {
  None,
  Row1ButtonMin,
  Row1ButtonHalf,
  Row1ButtonPlay,
  Row1Button2x,
  Row1ButtonPlus,
  Row2Button0,
  Row2Button1,
  Row2Button2,
  Row2Button3,
  Row2Button4,
  Row2Button5,
  Row2Button6,
  Row3Button0,
  Row3Button1,
  Row3Button2,
  Row3Button3,
  Row3Button4,
  Row3Button5,
  Row3Button6,
  PaintToggleButton, // Left of the big factory button
  Machine1x2Button,
  Machine2x1Button,
  Machine2x2Button,
  Machine3x3Button,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuRow {
  None,
  First,
  Second,
  Third,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
  Add,
  Remove
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventSourceType {
  Mouse,
  Touch,
  Unknown, // I think there's like tap, pointer, pen, whatever?
}

#[derive(Debug)]
pub struct Laser {
  pub coord: usize,
  pub visible_quest_index: usize,
  pub ttl: u32,
  pub color: String,
}

pub fn state_create(options: &Options, active_story_index: usize) -> State {
  return State {
    pregame: true,
    paused: false,
    active_story_index, // 0=system. dont default to 0.
    reset_next_frame: false,
    mouse_mode_mirrored: false,
    event_type_swapped: options.initial_event_type_swapped,
    mouse_mode_selecting: false,
    selected_area_copy: vec!(),
    test: false,
    lasers: vec!(),
    manual_open: false,
    snapshot_stack: [(); 100].map(|_| "".to_string()),
    load_snapshot_next_frame: false, // TODO: could do this for init too...?
    snapshot_pointer: 0,
    snapshot_undo_pointer: 0,
    load_example_next_frame: false,
    examples: vec!(),
    example_pointer: 0,
  };
}

pub fn state_add_examples(examples: Array, state: &mut State) {
  let mut result: Vec<String> = vec!();
  for maybe_str in examples.iter() {
    result.push(maybe_str.as_string().unwrap_or_else(| | panic!("Unable to parse element as string. Expecting an array of strings")));
  }
  state.examples = result;
}

pub fn mouse_button_to_action(state: &State, mouse_state: &MouseState) -> Action {
  let left_button = if state.mouse_mode_mirrored { 2 } else { 1 };
  return if mouse_state.last_down_button == left_button { Action::Add } else { Action::Remove }
}
