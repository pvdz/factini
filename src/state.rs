use std::collections::VecDeque;

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

pub const UNDO_STACK_SIZE: usize = 100;

pub struct State {
  pub paused: bool,
  pub reset_next_frame: bool,
  pub mouse_mode_erasing: bool,
  pub mouse_mode_selecting: bool,
  pub selected_area_copy: Vec<Vec<Cell>>,
  pub test: bool,
  pub bouncers: VecDeque<Bouncer>,
  pub trucks: Vec<Truck>,
  pub finished_quotes: Vec<usize>,
  pub lasers: Vec<Laser>,

  pub snapshot_stack: [String; UNDO_STACK_SIZE],
  pub load_snapshot_next_frame: bool,
  pub snapshot_pointer: usize,
  pub snapshot_undo_pointer: usize,
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

  pub cell_x: f64,
  pub cell_y: f64,
  pub cell_coord: usize,

  pub cell_rel_x: f64,
  pub cell_rel_y: f64,

  pub is_down: bool, // Set if pointer is currently down. Unset when the pointer is released.
  pub was_down: bool, // Set if current frame handled a pointer down. Should unset after the frame.
  pub is_dragging: bool,
  pub was_dragging: bool,
  pub is_drag_start: bool,
  pub is_up: bool, // Set if current frame handled a mouse up. It may be unset, unlike was_up.
  pub was_up: bool, // Set if current frame handled a mouse up. Should not be unset.

  pub offer_hover: bool,
  pub offer_hover_offer_index: usize, // Only relevant when offer_hover = true
  pub offer_down: bool,
  pub offer_down_offer_index: usize, // Kept until the next up, used for dragging
  pub offer_selected: bool,
  pub offer_selected_index: usize, // Offer index, not part index
  pub dragging_offer: bool,
  pub over_machine_button: bool, // Is the mouse currently over the machine button?
  pub down_machine_button: bool,
  pub dragging_machine: bool,

  pub craft_over_any: bool, // Was the over anywhere inside the craft circle? Prevemts actions underneath it.
  pub craft_over_ci: CraftInteractable,
  pub craft_over_ci_wx: f64,
  pub craft_over_ci_wy: f64,
  pub craft_over_ci_ww: f64,
  pub craft_over_ci_wh: f64,
  pub craft_over_ci_icon: char,
  pub craft_over_ci_index: u8,
  pub craft_over_ci_part_kind: PartKind,
  pub craft_down_any: bool, // Was the down anywhere inside the craft circle? Prevemts actions underneath it.
  pub craft_down_ci: CraftInteractable,
  pub craft_down_ci_wx: f64,
  pub craft_down_ci_wy: f64,
  pub craft_down_ci_ww: f64,
  pub craft_down_ci_wh: f64,
  pub craft_down_ci_icon: char,
  pub craft_down_ci_part_kind: PartKind,
  pub craft_down_ci_index: u8,
  pub craft_up_any: bool, // Was the up anywhere inside the craft circle? Prevemts actions underneath it.
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

  pub last_up_canvas_x: f64,
  pub last_up_canvas_y: f64,
  pub last_up_world_x: f64,
  pub last_up_world_y: f64,
}

#[derive(Debug)]
pub struct Laser {
  pub coord: usize,
  pub quote_pos: usize,
  pub ttl: u32,
  pub color: String,
}
