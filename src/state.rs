use std::collections::VecDeque;
use js_sys::{Array};

use super::belt::*;
use super::bouncer::*;
use super::cell::*;
use super::cli_serialize::*;
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
  // Track the visual pixel width/height of the canvas for screen-to-world mouse conversion
  // This size can change in fullscreen mode. The canvas is auto-scaled when it happens.
  // Note that this does not affect the canvas pixels for painting. That's always fixed.
  pub canvas_css_x: f64, // Not fullscreen: 0, otherwise fullscreen api implicitly adds padding
  pub canvas_css_y: f64, // Not fullscreen: 0, otherwise fullscreen api implicitly adds padding
  pub canvas_css_width: f64,
  pub canvas_css_height: f64,
  // How many pixels do we actually paint on the canvas.
  pub canvas_pixel_width: f64,
  pub canvas_pixel_height: f64,

  pub pregame: bool, // Showing main screen or loading screen?
  pub paused: bool,
  // Maps to config.stories (not config.nodes!). Defaults to 0. Use `- active` in any one story to activate it. Rejects for multiple occurrences (to prevent accidental issues)
  // 0=system. dont default to 0.
  pub active_story_index: usize,
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
  pub request_fullscreen: bool,

  pub reset_next_frame: bool,
  pub load_snapshot_next_frame: bool,
  pub load_example_next_frame: bool,
  pub load_copy_hint_kind: LoadCopyHint,
  pub load_copy_hint_since: u64,
  pub load_paste_next_frame: bool,
  pub load_paste_hint_kind: LoadPasteHint, // In case of problems, what do we show to the user?
  pub load_paste_hint_since: u64, // Last ticks timestamp of when the hint changed
  pub paste_to_load: String,
  pub hint_msg_since: u64, // Last ticks timestamp of when the hint changed
  pub hint_msg_text: String,

  pub showing_debug_bottom: bool, // Allows us to toggle the debug part with little overhead
  pub ui_unlock_progress: u8, // Used to track how much of the UI to unlock
  pub ui_speed_menu_anim_progress: u64, // Animation progress. Ticks left, relative to options.save_menu_animation_time
  pub ui_save_menu_anim_progress: u64, // Animation progress. Ticks left, relative to options.speed_menu_animation_time
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LoadPasteHint {
  None,
  Empty,
  Invalid,
  Success,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LoadCopyHint {
  None,
  Success,
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

  pub over_floor_zone: bool, // Anywhere in zone (even outside of floor)
  pub over_floor_not_corner: bool, // Over the floor but not any of the corner cells
  pub down_floor_area: bool,
  pub down_floor_not_corner: bool,

  pub over_quest: bool,
  pub over_quest_visible_index: usize, // Only if over_quest
  pub down_quest: bool,
  pub down_quest_visible_index: usize, // Only if down_quest
  pub up_quest: bool,
  pub up_quest_visible_index: usize, // Only if up_quest

  pub over_menu_button: MenuButton,
  pub down_menu_button: MenuButton,
  pub up_menu_button: MenuButton,

  pub help_hover: bool,
  pub help_down: bool,

  // woop: previously named "offer". But I don't have a proper short word for this. So it's woop now.
  pub woop_hover: bool,
  pub woop_hover_woop_index: usize, // Only relevant when woop_hover = true
  pub woop_down: bool,
  pub woop_down_woop_index: usize, // Kept until the next up, used for dragging
  pub woop_selected: bool,
  pub woop_selected_index: usize, // woop index, not part index

  pub atom_hover: bool,
  pub atom_hover_atom_index: usize, // Only relevant when atom_hover = true
  pub atom_down: bool,
  pub atom_down_atom_index: usize, // Kept until the next up, used for dragging
  pub atom_selected: bool,
  pub atom_selected_index: usize, // Atom index, not part index
  pub dragging_atom: bool,

  pub is_dragging_machine: bool,
  pub dragging_machine_w: u8,
  pub dragging_machine_h: u8,
  pub dragging_machine_part: PartKind, // The part that the machine being dragged will construct

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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuButton {
  None,
  SpeedMin,
  SpeedHalf,
  SpeedPlayPause,
  SpeedDouble,
  SpeedPlus,
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
  UndoButton,
  FullScreenButton,
  RedoButton,
  ClearButton,
  PaintToggleButton,
  AutoBuildButton,
  CopyFactory,
  PasteFactory,
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
  if options.trace_story_changes { log!("active_story_index state_create {}", active_story_index); }

  return State {
    canvas_css_x: 0.0,
    canvas_css_y: 0.0,
    canvas_css_width: CANVAS_CSS_INITIAL_WIDTH,
    canvas_css_height: CANVAS_CSS_INITIAL_HEIGHT,
    canvas_pixel_width: CANVAS_PIXEL_INITIAL_WIDTH,
    canvas_pixel_height: CANVAS_PIXEL_INITIAL_HEIGHT,
    pregame: true,
    paused: false,
    active_story_index,
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
    load_paste_next_frame: false,
    load_copy_hint_kind: LoadCopyHint::None,
    load_copy_hint_since: 0,
    load_paste_hint_kind: LoadPasteHint::None,
    load_paste_hint_since: 0,
    paste_to_load: "".to_string(),
    hint_msg_since: 0,
    hint_msg_text: "".to_string(),
    examples: vec!(),
    example_pointer: 0,
    request_fullscreen: false,
    showing_debug_bottom: true,
    ui_unlock_progress: 0,
    ui_speed_menu_anim_progress: 0,
    ui_save_menu_anim_progress: 0,
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

pub fn state_set_ui_unlock_progress(options: &mut Options, state: &mut State, ui_unlock_progress: u8) {
  state.ui_unlock_progress = ui_unlock_progress;
  if ui_unlock_progress == 1 {
    state.ui_speed_menu_anim_progress = options.speed_menu_animation_time;
    state.ui_unlock_progress += 1; // This makes it so that the animation does not restart when loading at this unlock progress step
  }
  if ui_unlock_progress >= 2 { options.enable_speed_menu = true; }
  if ui_unlock_progress == 3 {
    state.ui_save_menu_anim_progress = options.save_menu_animation_time;
    state.ui_unlock_progress += 1; // This makes it so that the animation does not restart when loading at this unlock progress step
  }
  if ui_unlock_progress >= 4 { options.enable_quick_save_menu = true; }
  if ui_unlock_progress >= 5 { options.enable_maze_roundway_and_collection = true; }
  if ui_unlock_progress >= 6 { options.enable_maze_full = true; }
}
