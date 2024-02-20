use super::config::*;
use super::floor::*;
use super::factory::*;
use super::options::*;
use super::state::*;
use crate::utils::*;
use super::log;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Zone {
  None,
  TopLeft,
  Top,
  TopRight,
  Left,
  Middle,
  Right,
  BottomLeft,
  Bottom,
  BottomRight,
  BottomBottomLeft,
  BottomBottom,
  BottomBottomRight,
  Manual,
  Margin,
}

// Size of the floor determines dimensions of all other grid items and depends on cell size and cell count.
pub const GRID_TOP_HEIGHT: f64 = 65.0;
pub const GRID_RIGHT_WIDTH: f64 = 400.0;
pub const GRID_BOTTOM_HEIGHT: f64 = 150.0;
pub const GRID_BOTTOM_DEBUG_HEIGHT: f64 = 400.0;
pub const GRID_LEFT_WIDTH: f64 = 200.0;

pub const GRID_PADDING: f64 = 5.0; // Spacing of grid blocks from edge and between grid blocks

pub const FLOOR_PATH_SPACING: f64 = 20.0;

// The Floor is the main game area
// The floor_offset is where the actual floor starts (opposed to the floor zone)
pub const UI_FLOOR_OFFSET_X: f64 = GRID_X1 + FLOOR_PATH_SPACING;
pub const UI_FLOOR_OFFSET_Y: f64 = GRID_Y1 + FLOOR_PATH_SPACING;
// This is the size of the whole zone, including the encircling track
pub const UI_FLOOR_WIDTH: f64 = FLOOR_PATH_SPACING + FLOOR_WIDTH + FLOOR_PATH_SPACING;
pub const UI_FLOOR_HEIGHT: f64 = FLOOR_PATH_SPACING + FLOOR_HEIGHT + FLOOR_PATH_SPACING;

// Achievements on the left
pub const UI_QUESTS_OFFSET_X: f64 = GRID_X0;
pub const UI_QUESTS_OFFSET_Y: f64 = UI_FLOOR_OFFSET_Y;
pub const UI_QUESTS_WIDTH: f64 = GRID_LEFT_WIDTH;
pub const UI_QUESTS_HEIGHT: f64 = UI_FLOOR_HEIGHT;
pub const UI_QUEST_X: f64 = 15.0;
pub const UI_QUEST_Y: f64 = 0.0;
pub const UI_QUEST_WIDTH: f64 = GRID_LEFT_WIDTH - (2.0 * UI_QUEST_X);
pub const UI_QUEST_HEIGHT: f64 = CELL_H + 4.0;
pub const UI_QUEST_MARGIN: f64 = 5.0;
pub const QUEST_FADE_TIME: u64 = 4 * ONE_SECOND;

pub const UI_SMALL_BUTTON_WIDTH: f64 = 60.0;
pub const UI_SMALL_BUTTON_HEIGHT: f64 = 60.0;

// Undo, redo, clear, paint mode
pub const UI_UNREDO_WIDTH: f64 = UI_SMALL_BUTTON_WIDTH;
pub const UI_UNREDO_HEIGHT: f64 = UI_SMALL_BUTTON_HEIGHT;
pub const UI_UNREDO_MARGIN: f64 = 5.0;
pub const UI_UNREDO_OFFSET_X: f64 = GRID_X2 + 10.0 - (UI_UNREDO_WIDTH * 4.0 + UI_UNREDO_MARGIN * 3.0);
pub const UI_UNREDO_OFFSET_Y: f64 = GRID_Y0;
pub const UI_UNREDO_UNDO_OFFSET_X: f64 = UI_UNREDO_OFFSET_X;
pub const UI_UNREDO_UNDO_OFFSET_Y: f64 = UI_UNREDO_OFFSET_Y;
pub const UI_UNREDO_REDO_OFFSET_X: f64 = UI_UNREDO_OFFSET_X + 1.0 * (UI_UNREDO_WIDTH + UI_UNREDO_MARGIN);
pub const UI_UNREDO_REDO_OFFSET_Y: f64 = UI_UNREDO_OFFSET_Y;
pub const UI_UNREDO_CLEAR_OFFSET_X: f64 = UI_UNREDO_OFFSET_X + 2.0 * (UI_UNREDO_WIDTH + UI_UNREDO_MARGIN);
pub const UI_UNREDO_CLEAR_OFFSET_Y: f64 = UI_UNREDO_OFFSET_Y;
pub const UI_UNREDO_PAINT_TOGGLE_X: f64 = UI_UNREDO_OFFSET_X + 3.0 * (UI_UNREDO_WIDTH + UI_UNREDO_MARGIN);
pub const UI_UNREDO_PAINT_TOGGLE_Y: f64 = UI_UNREDO_OFFSET_Y;

pub const UI_TOP_OFFSET_X: f64 = GRID_X1;
pub const UI_TOP_OFFSET_Y: f64 = GRID_Y0;
pub const UI_TOP_WIDTH: f64 = UI_FLOOR_WIDTH;
pub const UI_TOP_HEIGHT: f64 = GRID_TOP_HEIGHT;

pub const UI_HELP_X: f64 = GRID_X0 + 40.0;
pub const UI_HELP_Y: f64 = GRID_Y0 + 8.0;
pub const UI_HELP_WIDTH: f64 = 50.0;
pub const UI_HELP_HEIGHT: f64 = 40.0;

pub const UI_SAVE_OFFSET_X: f64 = 5.0;
pub const UI_SAVE_OFFSET_Y: f64 = 8.0;
// Note: game is currently at 1000 x 800. The floor is 600x600, so 1:1 ratio
pub const UI_SAVE_THUMB_WIDTH: f64 = 90.0;
pub const UI_SAVE_THUMB_HEIGHT: f64 = UI_SMALL_BUTTON_HEIGHT;
pub const UI_SAVE_THUMB_IMG_WIDTH: f64 = UI_SAVE_THUMB_WIDTH * 0.66; // Leave room for the close button
pub const UI_SAVE_THUMB_IMG_HEIGHT: f64 = UI_SAVE_THUMB_HEIGHT;
pub const UI_SAVE_MARGIN: f64 = 7.0;
// Note: we have 3x2 save tiles
pub const UI_SAVE_THUMB_X1: f64 = UI_SAVE_OFFSET_X;
pub const UI_SAVE_THUMB_Y1: f64 = UI_SAVE_OFFSET_Y;
// Clipboard import/export
pub const UI_SAVE_CP_WIDTH: f64 = UI_SMALL_BUTTON_WIDTH;
pub const UI_SAVE_CP_HEIGHT: f64 = UI_SMALL_BUTTON_HEIGHT;
pub const UI_SAVE_COPY_X: f64 = GRID_X0 + UI_SAVE_OFFSET_X + 2.0*UI_SAVE_THUMB_WIDTH + 2.0*UI_SAVE_MARGIN;
pub const UI_SAVE_COPY_Y: f64 = GRID_Y2 + UI_SAVE_OFFSET_Y;
pub const UI_SAVE_PASTE_X: f64 = UI_SAVE_COPY_X;
pub const UI_SAVE_PASTE_Y: f64 = UI_SAVE_COPY_Y + UI_SAVE_THUMB_HEIGHT + UI_SAVE_MARGIN;

pub const UI_BOTTOM_OFFSET_X: f64 = GRID_X1 + 15.0;
pub const UI_BOTTOM_OFFSET_Y: f64 = GRID_Y2 + 10.0;

pub const BUTTON_SPEED_MIN_INDEX: usize = 0;
pub const BUTTON_SPEED_HALF_INDEX: usize = 1;
pub const BUTTON_SPEED_PLAY_PAUSE_INDEX: usize = 2;
pub const BUTTON_SPEED_DOUBLE_INDEX: usize = 3;
pub const BUTTON_SPEED_PLUS_INDEX: usize = 4;

pub const UI_SPEED_BUBBLE_OFFSET_X: f64 = UI_TOP_OFFSET_X + 5.0;
pub const UI_SPEED_BUBBLE_OFFSET_Y: f64 = UI_TOP_OFFSET_Y + 5.0;
pub const UI_SPEED_BUBBLE_RADIUS: f64 = 25.0; // half the diameter...
pub const UI_SPEED_BUBBLE_SPACING: f64 = 10.0;

pub const UI_MENU_BUTTONS_COUNT_WIDTH_MAX: f64 = 6.0; // Update after adding new button
pub const UI_MENU_BUTTONS_OFFSET_X: f64 = GRID_X2 + 20.0;
pub const UI_MENU_BUTTONS_OFFSET_Y: f64 = GRID_Y0 + 5.0;
pub const UI_MENU_BUTTONS_OFFSET_Y2: f64 = UI_MENU_BUTTONS_OFFSET_Y + 30.0;
pub const UI_MENU_BUTTONS_WIDTH: f64 = 50.0;
pub const UI_MENU_BUTTONS_HEIGHT: f64 = 20.0;
pub const UI_MENU_BUTTONS_SPACING: f64 = 10.0;
pub const UI_MENU_BUTTONS_WIDTH_MAX: f64 = UI_MENU_BUTTONS_COUNT_WIDTH_MAX * (UI_MENU_BUTTONS_WIDTH + UI_MENU_BUTTONS_SPACING);

pub const UI_LOGO_X: f64 = UI_MENU_BUTTONS_OFFSET_X + 50.0;
pub const UI_LOGO_Y: f64 = UI_MENU_BUTTONS_OFFSET_Y;
pub const UI_LOGO_W: f64 = 210.0;
pub const UI_LOGO_H: f64 = 75.0;
// Target the dot on the first i as a way to quickly progress UI unlocks
pub const UI_DEBUG_UNLOCK_X: f64 = UI_LOGO_X + UI_LOGO_W - 76.0;
pub const UI_DEBUG_UNLOCK_Y: f64 = UI_LOGO_Y + 2.0;
pub const UI_DEBUG_UNLOCK_W: f64 = 15.0;
pub const UI_DEBUG_UNLOCK_H: f64 = 15.0;
// Target the dot on the last i as secret clickable area
pub const UI_DEBUG_SECRET_X: f64 = UI_LOGO_X + UI_LOGO_W - 21.0;
pub const UI_DEBUG_SECRET_Y: f64 = UI_LOGO_Y + 2.0;
pub const UI_DEBUG_SECRET_W: f64 = 15.0;
pub const UI_DEBUG_SECRET_H: f64 = 15.0;

pub const UI_MENU_MACHINE_BUTTON_1X2_X: f64 = GRID_X1 + 400.0;
pub const UI_MENU_MACHINE_BUTTON_1X2_Y: f64 = UI_BOTTOM_OFFSET_Y + 45.0;
pub const UI_MENU_MACHINE_BUTTON_1X2_WIDTH: f64 = 23.0;
pub const UI_MENU_MACHINE_BUTTON_1X2_HEIGHT: f64 = 47.0;

pub const UI_MENU_MACHINE_BUTTON_2X1_X: f64 = GRID_X1 + 380.0;
pub const UI_MENU_MACHINE_BUTTON_2X1_Y: f64 = UI_BOTTOM_OFFSET_Y + 10.0;
pub const UI_MENU_MACHINE_BUTTON_2X1_WIDTH: f64 = 47.0;
pub const UI_MENU_MACHINE_BUTTON_2X1_HEIGHT: f64 = 23.0;

pub const UI_MENU_MACHINE_BUTTON_2X2_X: f64 = GRID_X1 + 440.0;
pub const UI_MENU_MACHINE_BUTTON_2X2_Y: f64 = UI_BOTTOM_OFFSET_Y + 10.0;
pub const UI_MENU_MACHINE_BUTTON_2X2_WIDTH: f64 = 47.0;
pub const UI_MENU_MACHINE_BUTTON_2X2_HEIGHT: f64 = 47.0;

pub const UI_MENU_MACHINE_BUTTON_3X3_X: f64 = GRID_X1 + 500.0;
pub const UI_MENU_MACHINE_BUTTON_3X3_Y: f64 = UI_BOTTOM_OFFSET_Y + 14.0;
pub const UI_MENU_MACHINE_BUTTON_3X3_WIDTH: f64 = 70.0;
pub const UI_MENU_MACHINE_BUTTON_3X3_HEIGHT: f64 = 70.0;

pub const UI_DEBUG_OFFSET_X: f64 = GRID_X0 + 5.0;
pub const UI_DEBUG_OFFSET_Y: f64 = GRID_Y3 + 10.0;
pub const UI_DEBUG_WIDTH: f64 = GRID_LEFT_WIDTH + GRID_PADDING + UI_FLOOR_WIDTH + GRID_PADDING + GRID_RIGHT_WIDTH;
pub const UI_DEBUG_HEIGHT: f64 = GRID_BOTTOM_DEBUG_HEIGHT;
// The app stats
pub const UI_DEBUG_APP_OFFSET_X: f64 = GRID_X2 + 5.0;
pub const UI_DEBUG_APP_OFFSET_Y: f64 = GRID_Y3 + 10.0;
pub const UI_DEBUG_APP_WIDTH: f64 = 240.0;
pub const UI_DEBUG_APP_LINE_H: f64 = 25.0;
pub const UI_DEBUG_APP_FONT_H: f64 = 16.0;
pub const UI_DEBUG_APP_SPACING: f64 = 6.0;
pub const UI_DEBUG_LINES: f64 = 12.0; // Update after adding more lines
// Debug for AutoBuild mode
pub const UI_DEBUG_AUTO_BUILD_OFFSET_X: f64 = GRID_X1 + 5.0;
pub const UI_DEBUG_AUTO_BUILD_OFFSET_Y: f64 = GRID_Y3 + 10.0;
pub const UI_DEBUG_AUTO_BUILD_WIDTH: f64 = 240.0;
pub const UI_DEBUG_AUTO_BUILD_LINE_H: f64 = 25.0;
pub const UI_DEBUG_AUTO_BUILD_FONT_H: f64 = 16.0;
pub const UI_DEBUG_AUTO_BUILD_SPACING: f64 = 6.0;
pub const UI_DEBUG_AUTO_BUILD_LINES: f64 = 6.0; // Update after adding more lines
// Selected cell/machine details
pub const UI_DEBUG_CELL_OFFSET_X: f64 = UI_DEBUG_OFFSET_X;
pub const UI_DEBUG_CELL_OFFSET_Y: f64 = UI_DEBUG_OFFSET_Y;
pub const UI_DEBUG_CELL_WIDTH: f64 = 300.0;
pub const UI_DEBUG_CELL_HEIGHT: f64 = 250.0;
pub const UI_DEBUG_CELL_MARGIN: f64 = 5.0;
pub const UI_DEBUG_CELL_FONT_HEIGHT: f64 = 16.0; // at 12px + bottom spacing

pub const UI_WOOPS_OFFSET_X: f64 = GRID_X2 + 10.0;
pub const UI_WOOPS_OFFSET_Y: f64 = UI_FLOOR_OFFSET_Y;
pub const UI_WOOPS_PER_ROW: f64 = 6.0;
pub const UI_WOOPS_WIDTH: f64 = UI_WOTOM_WIDTH + ((UI_WOOPS_PER_ROW - 1.0) * UI_WOTOM_WIDTH_PLUS_MARGIN);

// woop / atom
pub const UI_WOTOM_WIDTH: f64 = 50.0;
pub const UI_WOTOM_HEIGHT: f64 = 50.0;
pub const UI_WOTOM_WIDTH_PLUS_MARGIN: f64 = UI_WOTOM_WIDTH + 10.0;
pub const UI_WOTOM_HEIGHT_PLUS_MARGIN: f64 = UI_WOTOM_HEIGHT + 10.0;

pub const UI_WOOP_TOOLTIP_X: f64 = UI_WOOPS_OFFSET_X + UI_WOTOM_WIDTH * 1.5; // Start in the middle of the second woops from the left
pub const UI_WOOP_TOOLTIP_Y_HIGH: f64 = UI_WOOPS_OFFSET_Y + 7.0; // Where the tooltip starts when painted high. Start in the middle of the top woops
pub const UI_WOOP_TOOLTIP_Y_LOW: f64 = UI_WOOP_TOOLTIP_Y_HIGH + UI_WOOP_TOOLTIP_HEIGHT + 32.0; // Where the tooltip starts when painted low. Start below where the other tooltip would be painted
pub const UI_WOOP_TOOLTIP_WIDTH: f64 = 185.0;
pub const UI_WOOP_TOOLTIP_HEIGHT: f64 = 3.0 + (0.75 * CELL_H) + 5.0 + (0.75 * CELL_H) + 5.0 + (0.75 * CELL_H) + 3.0;

pub const UI_ATOMS_OFFSET_X: f64 = GRID_X1 + 70.0;
pub const UI_ATOMS_OFFSET_Y: f64 = GRID_Y2 + 10.0;
pub const UI_ATOMS_PER_ROW: f64 = 5.0;
pub const UI_ATOMS_WIDTH: f64 = UI_WOTOM_WIDTH + ((UI_ATOMS_PER_ROW - 1.0) * UI_WOTOM_WIDTH_PLUS_MARGIN);

pub const UI_AUTO_BUILD_W: f64 = 60.0; // Size of a medium button
pub const UI_AUTO_BUILD_H: f64 = 60.0;
pub const UI_AUTO_BUILD_X: f64 = GRID_X0 + 100.0;
pub const UI_AUTO_BUILD_Y: f64 = GRID_Y0 + 0.0;

// The UI is a 3x3 grid of sections. The center section is the main part of the game, "the Floor"
// Define the coordinates of each "tab" (whatever the terminology ought to be) that defines the grid
pub const GRID_X0: f64 = GRID_PADDING;
pub const GRID_X1: f64 = GRID_X0 + GRID_LEFT_WIDTH + GRID_PADDING; // floor starts here
pub const GRID_X2: f64 = GRID_X1 + UI_FLOOR_WIDTH + GRID_PADDING;
pub const GRID_X3: f64 = GRID_X2 + GRID_RIGHT_WIDTH + GRID_PADDING;
pub const GRID_Y0: f64 = GRID_PADDING;
pub const GRID_Y1: f64 = GRID_Y0 + GRID_TOP_HEIGHT + GRID_PADDING; // floor starts here
pub const GRID_Y2: f64 = GRID_Y1 + UI_FLOOR_HEIGHT + GRID_PADDING;
pub const GRID_Y3: f64 = GRID_Y2 + GRID_BOTTOM_HEIGHT + GRID_PADDING; // debug offset
pub const GRID_Y4: f64 = GRID_Y3 + GRID_BOTTOM_DEBUG_HEIGHT + GRID_PADDING;

pub const ZONE_HELP: Zone = Zone::TopLeft;
pub const ZONE_QUESTS: Zone = Zone::Left;
pub const ZONE_SAVE_MAP: Zone = Zone::BottomLeft;
pub const ZONE_BOTTOM_BOTTOM_LEFT: Zone = Zone::BottomBottomLeft;
pub const ZONE_FLOOR: Zone = Zone::Middle;
pub const ZONE_BOTTOM_BOTTOM: Zone = Zone::BottomBottom;
pub const ZONE_TOP_RIGHT: Zone = Zone::TopRight;
pub const ZONE_RIGHT_BOTTOM: Zone = Zone::BottomRight;
pub const ZONE_BOTTOM_BOTTOM_RIGHT: Zone = Zone::BottomBottomRight;
pub const ZONE_MANUAL: Zone = Zone::Manual;
pub const ZONE_MARGIN: Zone = Zone::Margin; // Between the cracks of each zone

pub fn coord_to_zone(options: &Options, state: &State, config: &Config, x: f64, y: f64, is_machine_selected: bool, factory: &Factory, selected_coord: usize) -> Zone {
  if state.manual_open {
    return ZONE_MANUAL;
  }

  if x >= GRID_X0 && x < GRID_X0 + GRID_LEFT_WIDTH {
    return if y >= GRID_Y0 && y < GRID_Y0 + GRID_TOP_HEIGHT {
      // top-left, help section
      ZONE_HELP
    } else if y >= GRID_Y1 && y < GRID_Y1 + UI_FLOOR_HEIGHT {
      // left, quests
      ZONE_QUESTS
    } else if y >= GRID_Y2 && y < GRID_Y2 + GRID_BOTTOM_HEIGHT {
      // bottom-left, unused
      ZONE_SAVE_MAP
    } else if y >= GRID_Y3 && y < GRID_Y3 + GRID_BOTTOM_DEBUG_HEIGHT {
      // bottom-bottom-left, debug
      ZONE_BOTTOM_BOTTOM_LEFT
    } else {
      ZONE_MARGIN
    };
  }
  else if x >= GRID_X1 && x < GRID_X1 + UI_FLOOR_WIDTH {
    return if y >= GRID_Y0 && y < GRID_Y0 + GRID_TOP_HEIGHT {
      // top
      Zone::Top
    } else if y >= GRID_Y1 && y < GRID_Y1 + UI_FLOOR_HEIGHT {
      // middle, the floor
      ZONE_FLOOR
    } else if y >= GRID_Y2 && y < GRID_Y2 + GRID_BOTTOM_HEIGHT {
      // bottom, menu
      Zone::Bottom
    } else if y >= GRID_Y3 && y < GRID_Y3 + GRID_BOTTOM_DEBUG_HEIGHT {
      // bottom-bottom, debug
      ZONE_BOTTOM_BOTTOM
    } else {
      ZONE_MARGIN
    };
  }
  else if x >= GRID_X2 && x < GRID_X2 + GRID_RIGHT_WIDTH {
    return if y >= GRID_Y0 && y < GRID_Y0 + GRID_TOP_HEIGHT {
      // top-right, unused
      ZONE_TOP_RIGHT
    } else if y >= GRID_Y1 && y < GRID_Y1 + UI_FLOOR_HEIGHT {
      // right, woops
      Zone::Right
    } else if y >= GRID_Y2 && y < GRID_Y2 + GRID_BOTTOM_HEIGHT {
      // right-bottom, not really used but trucks turn here
      ZONE_RIGHT_BOTTOM
    } else if y >= GRID_Y3 && y < GRID_Y3 + GRID_BOTTOM_DEBUG_HEIGHT {
      // bottom-bottom, debug
      ZONE_BOTTOM_BOTTOM_RIGHT
    } else {
      ZONE_MARGIN
    };
  }

  return ZONE_MARGIN;
}

pub fn get_quest_xy(visible_index: usize, delta: f64) -> (f64, f64 ) {
  // TODO: take io into account when it is not in sync with index
  let x = UI_QUESTS_OFFSET_X + UI_QUEST_X;
  let y = UI_QUESTS_OFFSET_Y + delta + (visible_index as f64 * (UI_QUEST_HEIGHT + UI_QUEST_MARGIN));

  return ( x, y );
}

pub fn machine_dims_to_button_coords(width: usize, height: usize) -> (f64, f64, f64, f64) {
  // TODO: other machines are no longer painted. can remove stuff for it.
  match (width, height) {
    (1, 2) => (UI_MENU_MACHINE_BUTTON_1X2_X, UI_MENU_MACHINE_BUTTON_1X2_Y, UI_MENU_MACHINE_BUTTON_1X2_WIDTH, UI_MENU_MACHINE_BUTTON_1X2_HEIGHT),
    (2, 1) => (UI_MENU_MACHINE_BUTTON_2X1_X, UI_MENU_MACHINE_BUTTON_2X1_Y, UI_MENU_MACHINE_BUTTON_2X1_WIDTH, UI_MENU_MACHINE_BUTTON_2X1_HEIGHT),
    (2, 2) => (UI_MENU_MACHINE_BUTTON_2X2_X, UI_MENU_MACHINE_BUTTON_2X2_Y, UI_MENU_MACHINE_BUTTON_2X2_WIDTH, UI_MENU_MACHINE_BUTTON_2X2_HEIGHT),
    (3, 3) => (UI_MENU_MACHINE_BUTTON_3X3_X, UI_MENU_MACHINE_BUTTON_3X3_Y, UI_MENU_MACHINE_BUTTON_3X3_WIDTH, UI_MENU_MACHINE_BUTTON_3X3_HEIGHT),
    _ => panic!("Machine size of {}x{} is not supported here. add me!", width, height),
  }
}
