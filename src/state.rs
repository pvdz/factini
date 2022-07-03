use super::cell::*;

pub struct State {
  pub paused: bool,
  pub reset_next_frame: bool,
  pub mouse_mode_erasing: bool,
  pub mouse_mode_selecting: bool,
  pub selected_area_copy: Vec<Vec<Cell>>,
  pub test: bool,
}
