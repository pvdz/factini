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
  Craft,
  Manual,
  Margin,
}

// Size of the floor determines dimensions of all other grid items and depends on cell size and cell count.
pub const GRID_TOP_HEIGHT: f64 = 50.0;
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
pub const UI_QUOTES_OFFSET_X: f64 = GRID_X0;
pub const UI_QUOTES_OFFSET_Y: f64 = GRID_Y1;
pub const UI_QUOTES_WIDTH: f64 = GRID_LEFT_WIDTH;
pub const UI_QUOTES_HEIGHT: f64 = UI_FLOOR_HEIGHT;
pub const UI_QUOTE_X: f64 = 15.0;
pub const UI_QUOTE_Y: f64 = 0.0;
pub const UI_QUEST_WIDTH: f64 = GRID_LEFT_WIDTH - (2.0 * UI_QUOTE_X);
pub const UI_QUEST_HEIGHT: f64 = CELL_H + 4.0;
pub const UI_QUEST_MARGIN: f64 = 5.0;
pub const QUEST_FADE_TIME: u64 = 4 * ONE_SECOND;

// Undo, redo, clear, and sample buttons
pub const UI_UNREDO_OFFSET_X: f64 = GRID_X0 + 5.0;
pub const UI_UNREDO_OFFSET_Y: f64 = GRID_Y2 - 80.0;
// pub const UI_UNREDO_WIDTH: f64 = 75.0;
// pub const UI_UNREDO_HEIGHT: f64 = 60.0;
pub const UI_UNREDO_UNDO_OFFSET_X: f64 = UI_UNREDO_OFFSET_X;
pub const UI_UNREDO_UNDO_OFFSET_Y: f64 = UI_UNREDO_OFFSET_Y;
pub const UI_UNREDO_UNDO_WIDTH: f64 = 60.0;
pub const UI_UNREDO_UNDO_HEIGHT: f64 = 60.0;
pub const UI_UNREDO_CLEAR_OFFSET_X: f64 = UI_UNREDO_OFFSET_X + UI_UNREDO_UNDO_WIDTH + 5.0;
pub const UI_UNREDO_CLEAR_OFFSET_Y: f64 = UI_UNREDO_OFFSET_Y;
pub const UI_UNREDO_CLEAR_WIDTH: f64 = 60.0;
pub const UI_UNREDO_CLEAR_HEIGHT: f64 = 60.0;
pub const UI_UNREDO_REDO_OFFSET_X: f64 = UI_UNREDO_CLEAR_OFFSET_X + UI_UNREDO_CLEAR_WIDTH + 5.0;
pub const UI_UNREDO_REDO_OFFSET_Y: f64 = UI_UNREDO_OFFSET_Y;
pub const UI_UNREDO_REDO_WIDTH: f64 = 60.0;
pub const UI_UNREDO_REDO_HEIGHT: f64 = 60.0;

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
pub const UI_SAVE_THUMB_HEIGHT: f64 = 60.0;
pub const UI_SAVE_THUMB_IMG_WIDTH: f64 = UI_SAVE_THUMB_WIDTH * 0.66; // Leave room for the close button
pub const UI_SAVE_THUMB_IMG_HEIGHT: f64 = UI_SAVE_THUMB_HEIGHT;
pub const UI_SAVE_MARGIN: f64 = 7.0;
// Note: we have 3x2 save tiles
pub const UI_SAVE_THUMB_X1: f64 = UI_SAVE_OFFSET_X;
pub const UI_SAVE_THUMB_Y1: f64 = UI_SAVE_OFFSET_Y;

pub const UI_BOTTOM_OFFSET_X: f64 = GRID_X1 + 15.0;
pub const UI_BOTTOM_OFFSET_Y: f64 = GRID_Y2 + 10.0;

pub const UI_SPEED_BUBBLE_OFFSET_X: f64 = UI_BOTTOM_OFFSET_X + 5.0;
pub const UI_SPEED_BUBBLE_OFFSET_Y: f64 = UI_BOTTOM_OFFSET_Y + 5.0;
pub const UI_SPEED_BUBBLE_RADIUS: f64 = 20.0; // half the diameter...
pub const UI_SPEED_BUBBLE_SPACING: f64 = 15.0;

pub const UI_MENU_BUTTONS_COUNT_WIDTH_MAX: f64 = 6.0; // Update after adding new button
pub const UI_MENU_BUTTONS_OFFSET_X: f64 = UI_BOTTOM_OFFSET_X + 2.0;
pub const UI_MENU_BUTTONS_OFFSET_Y: f64 = UI_BOTTOM_OFFSET_Y + 55.0;
pub const UI_MENU_BUTTONS_OFFSET_Y2: f64 = UI_BOTTOM_OFFSET_Y + 85.0;
pub const UI_MENU_BUTTONS_WIDTH: f64 = 50.0;
pub const UI_MENU_BUTTONS_HEIGHT: f64 = 20.0;
pub const UI_MENU_BUTTONS_SPACING: f64 = 10.0;
pub const UI_MENU_BUTTONS_WIDTH_MAX: f64 = UI_MENU_BUTTONS_COUNT_WIDTH_MAX * (UI_MENU_BUTTONS_WIDTH + UI_MENU_BUTTONS_SPACING);

pub const UI_MENU_BOTTOM_PAINT_TOGGLE_X: f64 = UI_MENU_BOTTOM_MACHINE3X3_X - (UI_MENU_BOTTOM_PAINT_TOGGLE_WIDTH + 5.0);
pub const UI_MENU_BOTTOM_PAINT_TOGGLE_Y: f64 = UI_BOTTOM_OFFSET_Y + 5.0;
pub const UI_MENU_BOTTOM_PAINT_TOGGLE_WIDTH: f64 = 70.0;
pub const UI_MENU_BOTTOM_PAINT_TOGGLE_HEIGHT: f64 = 70.0;

pub const UI_MENU_BOTTOM_MACHINE3X3_X: f64 = GRID_X2 - (UI_MENU_BOTTOM_MACHINE3X3_WIDTH + 5.0);
pub const UI_MENU_BOTTOM_MACHINE3X3_Y: f64 = UI_BOTTOM_OFFSET_Y + 5.0;
pub const UI_MENU_BOTTOM_MACHINE3X3_WIDTH: f64 = 70.0;
pub const UI_MENU_BOTTOM_MACHINE3X3_HEIGHT: f64 = 70.0;

pub const UI_MENU_BOTTOM_MACHINE1X2_X: f64 = UI_MENU_BOTTOM_MACHINE2X2_X - 10.0 - UI_MENU_BOTTOM_MACHINE1X2_WIDTH;
pub const UI_MENU_BOTTOM_MACHINE1X2_Y: f64 = UI_BOTTOM_OFFSET_Y + 5.0;
pub const UI_MENU_BOTTOM_MACHINE1X2_WIDTH: f64 = 23.0;
pub const UI_MENU_BOTTOM_MACHINE1X2_HEIGHT: f64 = 47.0;

pub const UI_MENU_BOTTOM_MACHINE2X1_X: f64 = UI_MENU_BOTTOM_MACHINE2X2_X - 10.0;
pub const UI_MENU_BOTTOM_MACHINE2X1_Y: f64 = UI_MENU_BOTTOM_MACHINE1X2_Y + UI_MENU_BOTTOM_MACHINE1X2_HEIGHT + 10.0;
pub const UI_MENU_BOTTOM_MACHINE2X1_WIDTH: f64 = 47.0;
pub const UI_MENU_BOTTOM_MACHINE2X1_HEIGHT: f64 = 23.0;

pub const UI_MENU_BOTTOM_MACHINE2X2_X: f64 = UI_MENU_BOTTOM_PAINT_TOGGLE_X - 10.0 - UI_MENU_BOTTOM_MACHINE2X2_WIDTH;
pub const UI_MENU_BOTTOM_MACHINE2X2_Y: f64 = UI_BOTTOM_OFFSET_Y + 5.0;
pub const UI_MENU_BOTTOM_MACHINE2X2_WIDTH: f64 = 47.0;
pub const UI_MENU_BOTTOM_MACHINE2X2_HEIGHT: f64 = 47.0;

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
pub const UI_DEBUG_LINES: f64 = 11.0; // Update after adding more lines
// Selected cell/machine details
pub const UI_DEBUG_CELL_OFFSET_X: f64 = UI_DEBUG_OFFSET_X;
pub const UI_DEBUG_CELL_OFFSET_Y: f64 = UI_DEBUG_OFFSET_Y;
pub const UI_DEBUG_CELL_WIDTH: f64 = 300.0;
pub const UI_DEBUG_CELL_HEIGHT: f64 = 250.0;
pub const UI_DEBUG_CELL_MARGIN: f64 = 5.0;
pub const UI_DEBUG_CELL_FONT_HEIGHT: f64 = 16.0; // at 12px + bottom spacing

pub const UI_OFFERS_OFFSET_X: f64 = GRID_X2 + 10.0;
pub const UI_OFFERS_OFFSET_Y: f64 = GRID_Y1;
pub const UI_OFFER_WIDTH: f64 = 50.0;
pub const UI_OFFER_HEIGHT: f64 = 50.0;
pub const UI_OFFER_WIDTH_PLUS_MARGIN: f64 = UI_OFFER_WIDTH + 10.0;
pub const UI_OFFER_HEIGHT_PLUS_MARGIN: f64 = UI_OFFER_HEIGHT + 10.0;
pub const UI_OFFERS_PER_ROW: f64 = 6.0;
pub const UI_OFFERS_WIDTH: f64 = UI_OFFER_WIDTH + ((UI_OFFERS_PER_ROW - 1.0) * UI_OFFER_WIDTH_PLUS_MARGIN);
pub const UI_OFFER_TOOLTIP_WIDTH: f64 = 185.0;
pub const UI_OFFER_TOOLTIP_HEIGHT: f64 = 3.0 + (0.75 * CELL_H) + 5.0 + (0.75 * CELL_H) + 5.0 + (0.75 * CELL_H) + 3.0;

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
pub const ZONE_QUOTES: Zone = Zone::Left;
pub const ZONE_SAVE_MAP: Zone = Zone::BottomLeft;
pub const ZONE_BOTTOM_BOTTOM_LEFT: Zone = Zone::BottomBottomLeft;
pub const ZONE_FLOOR: Zone = Zone::Middle;
pub const ZONE_MENU: Zone = Zone::Bottom;
pub const ZONE_BOTTOM_BOTTOM: Zone = Zone::BottomBottom;
pub const ZONE_TOP_RIGHT: Zone = Zone::TopRight;
pub const ZONE_OFFERS: Zone = Zone::Right;
pub const ZONE_RIGHT_BOTTOM: Zone = Zone::BottomRight;
pub const ZONE_BOTTOM_BOTTOM_RIGHT: Zone = Zone::BottomBottomRight;
pub const ZONE_CRAFT: Zone = Zone::Craft;
pub const ZONE_MANUAL: Zone = Zone::Manual;
pub const ZONE_MARGIN: Zone = Zone::Margin; // Between the cracks of each zone

pub fn coord_to_zone(options: &Options, state: &State, config: &Config, x: f64, y: f64, is_machine_selected: bool, factory: &Factory, selected_coord: usize) -> Zone {
  if state.manual_open {
    return ZONE_MANUAL;
  }

  if is_machine_selected && hit_test_machine_craft_menu(options, factory, selected_coord, x, y) {
    return ZONE_CRAFT
  }

  if x >= GRID_X0 && x < GRID_X0 + GRID_LEFT_WIDTH {
    return if y >= GRID_Y0 && y < GRID_Y0 + GRID_TOP_HEIGHT {
      // top-left, help section
      ZONE_HELP
    } else if y >= GRID_Y1 && y < GRID_Y1 + UI_FLOOR_HEIGHT {
      // left, quotes
      ZONE_QUOTES
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
      ZONE_MENU
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
      // right, offers
      ZONE_OFFERS
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

pub fn hit_test_machine_craft_menu(options: &Options, factory: &Factory, any_machine_coord: usize, mwx: f64, mwy: f64) -> bool {
  let main_coord = factory.floor[any_machine_coord].machine.main_coord;
  if options.enable_craft_menu_circle {
    // When craft menu is displayed, test for whole circle
    let ( center_wx, center_wy, cr ) = get_machine_selection_circle_params(factory, main_coord);
    return hit_test_circle(mwx, mwy, center_wx, center_wy, cr);
  }
  else {
    // Without craft menu just check if selected machine
    let ( main_x, main_y) = to_xy(main_coord);
    let machine_width = factory.floor[main_coord].machine.cell_width as f64;
    let machine_height = factory.floor[main_coord].machine.cell_height as f64;
    return bounds_check((mwx - UI_FLOOR_OFFSET_X) / CELL_W, (mwy - UI_FLOOR_OFFSET_Y) / CELL_H, main_x as f64, main_y as f64, main_x as f64 + machine_width, main_y as f64 + machine_height);
  }
}

pub fn get_machine_selection_circle_params(factory: &Factory, main_coord: usize) -> ( f64, f64, f64 ) {
  // Find the center of the machine because .arc() requires the center x,y
  let ( main_x, main_y ) = to_xy(main_coord);
  let machine_width = factory.floor[main_coord].machine.cell_width as f64;
  let machine_height = factory.floor[main_coord].machine.cell_height as f64;
  let center_cell_x = main_x as f64 + machine_width / 2.0;
  let center_cell_y = main_y as f64 + machine_height / 2.0;
  let center_wx = UI_FLOOR_OFFSET_X + center_cell_x * CELL_W;
  let center_wy = UI_FLOOR_OFFSET_Y + center_cell_y * CELL_H;
  // Radius should be enough to fit half the biggest axis + margin + diagonal of resource bubble + border
  let cr = (machine_width as f64 * (CELL_W as f64)).max(machine_height as f64 * (CELL_H as f64)) * 0.5 + 10.0 + CELL_W.max(CELL_H) * 2.0 + 5.0;
  // let cr = 5.0;
  return ( center_wx, center_wy, cr );
}
pub fn hit_test_circle(x: f64, y: f64, cx: f64, cy: f64, r: f64) -> bool {
  // Hit test for a circle is testing whether the distance from the center of the circle to the
  // point is smaller than the radius. The formula is relatively simple: (x1-x2)^2+(y1-y2)^2<=r^2
  // https://www.xarg.org/book/computer-graphics/2d-hittest/
  return (cx-x).powf(2.0) + (cy-y).powf(2.0) <= r.powf(2.0);
}

pub fn get_quest_xy(visible_index: usize, delta: f64) -> (f64, f64 ) {
  // TODO: take io into account when it is not in sync with index
  let x = UI_QUOTES_OFFSET_X + UI_QUOTE_X;
  let y = UI_QUOTES_OFFSET_Y + delta + (visible_index as f64 * (UI_QUEST_HEIGHT + UI_QUEST_MARGIN));

  return ( x, y );
}
