// This file should only be included for `wasm-pack build --target web`
// The main.rs will include this file when `#[cfg(target_arch = "wasm32")]`

// - export all the things
// - import/export with clipboard
// - input (string map) validation
// - score card stuff
// - stamp instead of paste button (paste on click)

// This is required to export panic to the web
use std::panic;

// This crate dumps panics to console.log in the browser
extern crate console_error_panic_hook;

// This is just to compile stuff to wasm.
use wasm_bindgen::prelude::*;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use std::collections::VecDeque;
use web_sys::{HtmlCanvasElement, HtmlImageElement};
use crate::port;

use super::belt::*;
use super::cell::*;
use super::cli_serialize::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::init::*;
use super::offer::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::paste::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::state::*;
use super::utils::*;

// These are the actual pixels we can paint to
const CANVAS_WIDTH: f64 = 1000.0;
const CANVAS_HEIGHT: f64 = 710.0;
// Need this for mouse2world coord conversion
const CANVAS_CSS_WIDTH: f64 = 1000.0;
const CANVAS_CSS_HEIGHT: f64 = 710.0;

// World size in world pixels (as painted on the canvas)
const WORLD_OFFSET_X: f64 = 20.0;
const WORLD_OFFSET_Y: f64 = 50.0;
const WORLD_WIDTH: f64 = FLOOR_CELLS_W as f64 * CELL_W;
const WORLD_HEIGHT: f64 = FLOOR_CELLS_H as f64 * CELL_H;

// Size of a cell (world pixels)
const CELL_W: f64 = 35.0;
const CELL_H: f64 = 35.0;
// 3x3 segments in a cell
const SEGMENT_W: f64 = 5.0;
const SEGMENT_H: f64 = 5.0;
// Size of parts on a belt
const PART_W: f64 = 20.0;
const PART_H: f64 = 20.0;

// UI = the right side boxes where stats and interface is painted
const UI_OX: f64 = 750.0;
const UI_OY: f64 = 10.0;
const UI_W: f64 = 230.0;
const UI_LINE_H: f64 = 25.0;
const UI_FONT_H: f64 = 16.0;
const UI_ML: f64 = 6.0;
const UI_SEGMENT_W: f64 = 40.0;
const UI_SEGMENT_H: f64 = 40.0;
const UI_DEBUG_LINES: f64 = 8.0;

const UI_OFFERS_OX: f64 = UI_OX - 100.0 - 10.0;
const UI_OFFERS_OY: f64 = UI_OY;
const UI_OFFERS_W: f64 = 100.0;
const UI_OFFERS_H: f64 = 90.0;
const UI_OFFERS_H_PLUS_MARGIN: f64 = UI_OFFERS_H + 10.0;

const UI_BUTTON_COUNT: f64 = 7.0; // Update after adding new button
const UI_BUTTONS_OX: f64 = WORLD_OFFSET_X;
const UI_BUTTONS_OY: f64 = CANVAS_HEIGHT - 50.0;
const UI_BUTTONS_OY2: f64 = CANVAS_HEIGHT - 25.0;
const UI_BUTTON_W: f64 = 50.0;
const UI_BUTTON_H: f64 = 20.0;
const UI_BUTTON_SPACING: f64 = 10.0;
const UI_SPEED_BUBBLE_OX: f64 = UI_BUTTONS_OX + UI_BUTTON_COUNT * (UI_BUTTON_W + UI_BUTTON_SPACING + 1.0);
const UI_SPEED_BUBBLE_OY: f64 = UI_BUTTONS_OY + 4.0;
const UI_SPEED_BUBBLE_RADIUS: f64 = 13.0;

const UI_CELL_EDITOR_OX: f64 = UI_OX;
const UI_CELL_EDITOR_OY: f64 = UI_OY + (UI_LINE_H * (UI_DEBUG_LINES + 2.0));
const UI_CELL_EDITOR_W: f64 = UI_W;
const UI_CELL_EDITOR_H: f64 = UI_LINE_H * 7.0;

const UI_CELL_EDITOR_GRID_OX: f64 = UI_OX + 10.0;
const UI_CELL_EDITOR_GRID_OY: f64 = UI_OY + ((UI_DEBUG_LINES + 3.0) * UI_LINE_H) + UI_FONT_H;
const UI_CELL_EDITOR_GRID_W: f64 = 3.0 * UI_SEGMENT_W;
const UI_CELL_EDITOR_GRID_H: f64 = 3.0 * UI_SEGMENT_H;

const UI_CELL_EDITOR_PART_OX: f64 = UI_CELL_EDITOR_GRID_OX + (3.0 * UI_SEGMENT_W) + 10.0;
const UI_CELL_EDITOR_PART_OY: f64 = UI_CELL_EDITOR_GRID_OY + (2.0 * UI_FONT_H);

const UI_MACHINE_EDITOR_OX: f64 = UI_OX;
const UI_MACHINE_EDITOR_OY: f64 = UI_CELL_EDITOR_OY + UI_CELL_EDITOR_H + UI_LINE_H;
const UI_MACHINE_EDITOR_W: f64 = UI_CELL_EDITOR_W;
const UI_MACHINE_EDITOR_H: f64 = 150.0;

// Temp placeholder
const COLOR_SUPPLY: &str = "pink";
const COLOR_SUPPLY_SEMI: &str = "#6f255154";
const COLOR_DEMAND: &str = "lightgreen";
const COLOR_DEMAND_SEMI: &str = "#00aa0055";
const COLOR_MACHINE: &str = "lightyellow";
const COLOR_MACHINE_SEMI: &str = "#aaaa0099";

// Exports from web (on a non-module context, define a global "log" and "dnow" function)
// Not sure how this works in threads. Probably the same. TBD.
// I think all natives are exposed in js_sys or web_sys somehow so not sure we need this at all.
#[wasm_bindgen]
extern {
  // pub fn log(s: &str); // -> console.log(s)
  // pub fn print_world(s: &str);
  // pub fn print_options(options: &str);
  // pub fn dnow() -> u64; // -> Date.now()
  // pub async fn await_next_frame() -> JsValue;
  // pub async fn suspend_app_to_start() -> JsValue;
}


// Would this be a better/more efficient way? Probably slow either way.
// // lifted from the `console_log` example
// #[wasm_bindgen]
// extern "C" {
//   #[wasm_bindgen(js_namespace = console)]
//   fn log(a: &str);
// }

#[derive(Debug)]
struct CellSelection {
  on: bool,
  x: f64,
  y: f64,
  coord: usize,
  area: bool,
  x2: f64,
  y2: f64,
}

#[derive(Debug)]
struct MouseState {
  canvas_x: f64,
  canvas_y: f64,

  world_x: f64,
  world_y: f64,

  cell_x: f64,
  cell_y: f64,
  cell_coord: usize,

  cell_rel_x: f64,
  cell_rel_y: f64,

  is_down: bool,
  is_dragging: bool,
  is_drag_start: bool,

  was_down: bool,
  was_dragging: bool,
  was_up: bool,

  over_offer: bool,
  dragging_offer: bool,
  offer_index: usize, // Only relevant when over_offer = true

  // https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons
  // bitwise field; 1=left, 2=right, 3=left|right, 4=middle, etc
  // (8 and 16 supposedly browser back/forward button but ehhhh)
  // On a phone/tablet this is not used of course
  last_down_button: u16,

  last_down_canvas_x: f64,
  last_down_canvas_y: f64,
  last_down_world_x: f64,
  last_down_world_y: f64,

  last_up_canvas_x: f64,
  last_up_canvas_y: f64,
  last_up_world_x: f64,
  last_up_world_y: f64,
}

fn dnow() -> u64 {
  js_sys::Date::now() as u64
}

fn load_tile(src: &str) -> Result<web_sys::HtmlImageElement, JsValue> {
  let document = web_sys::window().unwrap().document().unwrap();

  let img = document
    .create_element("img")?
    .dyn_into::<web_sys::HtmlImageElement>()?;

  img.set_src(src);

  return Ok(img);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  // Must run this once in web-mode to enable dumping panics to console.log
  panic::set_hook(Box::new(console_error_panic_hook::hook));

  log(format!("web start..."));
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document
    .create_element("canvas")?
    .dyn_into::<web_sys::HtmlCanvasElement>()?;
  document.body().unwrap().append_child(&canvas)?;
  canvas.set_width(CANVAS_WIDTH as u32);
  canvas.set_height(CANVAS_HEIGHT as u32);
  canvas.style().set_property("border", "solid")?;
  canvas.style().set_property("width", format!("{}px", CANVAS_CSS_WIDTH as u32).as_str())?;
  canvas.style().set_property("height", format!("{}px", CANVAS_CSS_HEIGHT as u32).as_str())?;
  let context = canvas
    .get_context("2d")?
    .unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
  let context = Rc::new(context);

  let todo = load_tile("./img/todo.png").expect("can'tpub const BELT_NONE.src");

  // Preload the belt tiles. Create an array with a to-do image for every slot. Then create img tags
  let mut belt_tile_images: Vec<web_sys::HtmlImageElement> = vec![todo; BELT_TYPE_COUNT]; // Prefill with todo images
  belt_tile_images[BeltType::NONE as usize] = load_tile(BELT_NONE.src)?;
  // belt_tile_images[BeltType::U_R as usize] = load_tile(BELT_U_R.src)?;
  // belt_tile_images[BeltType::R_U as usize] = load_tile(BELT_R_U.src)?;
  // belt_tile_images[BeltType::R_D as usize] = load_tile(BELT_R_D.src)?;
  // belt_tile_images[BeltType::D_R as usize] = load_tile(BELT_D_R.src)?;
  // belt_tile_images[BeltType::D_L as usize] = load_tile(BELT_D_L.src)?;
  // belt_tile_images[BeltType::L_D as usize] = load_tile(BELT_L_D.src)?;
  // belt_tile_images[BeltType::L_U as usize] = load_tile(BELT_L_U.src)?;
  // belt_tile_images[BeltType::U_L as usize] = load_tile(BELT_U_L.src)?;
  // belt_tile_images[BeltType::U_D as usize] = load_tile(BELT_U_D.src)?;
  // belt_tile_images[BeltType::D_U as usize] = load_tile(BELT_D_U.src)?;
  // belt_tile_images[BeltType::L_R as usize] = load_tile(BELT_L_R.src)?;
  // belt_tile_images[BeltType::R_L as usize] = load_tile(BELT_R_L.src)?;
  // belt_tile_images[BeltType::U_LR as usize] = load_tile(BELT_U_LR.src)?;
  // belt_tile_images[BeltType::RU_L as usize] = load_tile(BELT_RU_L.src)?;
  // belt_tile_images[BeltType::LU_R as usize] = load_tile(BELT_LU_R.src)?;
  // belt_tile_images[BeltType::L_RU as usize] = load_tile(BELT_L_RU.src)?;
  // belt_tile_images[BeltType::LR_U as usize] = load_tile(BELT_LR_U.src)?;
  // belt_tile_images[BeltType::R_LU as usize] = load_tile(BELT_R_LU.src)?;
  // belt_tile_images[BeltType::R_DU as usize] = load_tile(BELT_R_DU.src)?;
  // belt_tile_images[BeltType::RU_D as usize] = load_tile(BELT_RU_D.src)?;
  // belt_tile_images[BeltType::DR_U as usize] = load_tile(BELT_DR_U.src)?;
  // belt_tile_images[BeltType::DU_R as usize] = load_tile(BELT_DU_R.src)?;
  // belt_tile_images[BeltType::U_DR as usize] = load_tile(BELT_U_DR.src)?;
  // belt_tile_images[BeltType::D_RU as usize] = load_tile(BELT_D_RU.src)?;
  // belt_tile_images[BeltType::D_LR as usize] = load_tile(BELT_D_LR.src)?;
  // belt_tile_images[BeltType::DL_R as usize] = load_tile(BELT_DL_R.src)?;
  // belt_tile_images[BeltType::DR_L as usize] = load_tile(BELT_DR_L.src)?;
  // belt_tile_images[BeltType::LR_D as usize] = load_tile(BELT_LR_D.src)?;
  // belt_tile_images[BeltType::L_DR as usize] = load_tile(BELT_L_DR.src)?;
  // belt_tile_images[BeltType::R_DL as usize] = load_tile(BELT_R_DL.src)?;
  // belt_tile_images[BeltType::L_DU as usize] = load_tile(BELT_L_DU.src)?;
  // belt_tile_images[BeltType::LU_D as usize] = load_tile(BELT_LU_D.src)?;
  // belt_tile_images[BeltType::DL_U as usize] = load_tile(BELT_DL_U.src)?;
  // belt_tile_images[BeltType::DU_L as usize] = load_tile(BELT_DU_L.src)?;
  // belt_tile_images[BeltType::U_DL as usize] = load_tile(BELT_U_DL.src)?;
  // belt_tile_images[BeltType::D_UL as usize] = load_tile(BELT_D_LU.src)?;
  // belt_tile_images[BeltType::U_DLR as usize] = load_tile(BELT_U_DLR.src)?;
  // belt_tile_images[BeltType::R_DLU as usize] = load_tile(BELT_R_DLU.src)?;
  // belt_tile_images[BeltType::D_LRU as usize] = load_tile(BELT_D_LRU.src)?;
  // belt_tile_images[BeltType::L_DRU as usize] = load_tile(BELT_L_DRU.src)?;
  // belt_tile_images[BeltType::RU_DL as usize] = load_tile(BELT_RU_DL.src)?;
  // belt_tile_images[BeltType::DU_LR as usize] = load_tile(BELT_DU_LR.src)?;
  // belt_tile_images[BeltType::LU_DR as usize] = load_tile(BELT_LU_DR.src)?;
  // belt_tile_images[BeltType::LD_RU as usize] = load_tile(BELT_DL_RU.src)?;
  // belt_tile_images[BeltType::DR_LU as usize] = load_tile(BELT_DR_LU.src)?;
  // belt_tile_images[BeltType::LR_DU as usize] = load_tile(BELT_LR_DU.src)?;
  // belt_tile_images[BeltType::DLR_U as usize] = load_tile(BELT_DLR_U.src)?;
  // belt_tile_images[BeltType::DLU_R as usize] = load_tile(BELT_DLU_R.src)?;
  // belt_tile_images[BeltType::RLU_D as usize] = load_tile(BELT_LRU_D.src)?;
  // belt_tile_images[BeltType::DRU_L as usize] = load_tile(BELT_DRU_L.src)?;
  belt_tile_images[BeltType::RU as usize] = load_tile(BELT_RU.src)?;
  belt_tile_images[BeltType::DR as usize] = load_tile(BELT_DR.src)?;
  belt_tile_images[BeltType::DL as usize] = load_tile(BELT_DL.src)?;
  belt_tile_images[BeltType::LU as usize] = load_tile(BELT_LU.src)?;
  belt_tile_images[BeltType::DU as usize] = load_tile(BELT_DU.src)?;
  belt_tile_images[BeltType::LR as usize] = load_tile(BELT_LR.src)?;
  belt_tile_images[BeltType::LRU as usize] = load_tile(BELT_LRU.src)?;
  belt_tile_images[BeltType::DRU as usize] = load_tile(BELT_DRU.src)?;
  belt_tile_images[BeltType::DLR as usize] = load_tile(BELT_DLR.src)?;
  belt_tile_images[BeltType::DLU as usize] = load_tile(BELT_DLU.src)?;
  belt_tile_images[BeltType::DLRU as usize] = load_tile(BELT_DLRU.src)?;
  belt_tile_images[BeltType::UNKNOWN as usize] = load_tile(BELT_UNKNOWN.src)?;
  belt_tile_images[BeltType::INVALID as usize] = load_tile(BELT_INVALID.src)?;

  let part_tile_sprite: web_sys::HtmlImageElement = load_tile("./img/roguelikeitems.png")?;

  let img_machine1: web_sys::HtmlImageElement = load_tile("./img/machine1.png")?;
  let img_machine2: web_sys::HtmlImageElement = load_tile("./img/machine2.png")?;
  let img_machine3: web_sys::HtmlImageElement = load_tile("./img/machine3.png")?;
  let img_machine4: web_sys::HtmlImageElement = load_tile("./img/machine4.png")?;

  // Tbh this whole Rc approach is copied from the original template. It works so why not, :shrug:
  let mouse_x = Rc::new(Cell::new(0.0));
  let mouse_y = Rc::new(Cell::new(0.0));
  let is_mouse_down = Rc::new(Cell::new(false));
  let last_mouse_down_x = Rc::new(Cell::new(0.0));
  let last_mouse_down_y = Rc::new(Cell::new(0.0));
  let last_mouse_down_button = Rc::new(Cell::new(0));
  let last_mouse_up_x = Rc::new(Cell::new(0.0));
  let last_mouse_up_y = Rc::new(Cell::new(0.0));

  // mousedown
  {
    let is_mouse_down = is_mouse_down.clone();
    let last_mouse_down_x = last_mouse_down_x.clone();
    let last_mouse_down_y = last_mouse_down_y.clone();
    let last_mouse_down_button = last_mouse_down_button.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
      let mx = event.offset_x() as f64;
      let my = event.offset_y() as f64;
      is_mouse_down.set(true);
      last_mouse_down_x.set(mx);
      last_mouse_down_y.set(my);
      last_mouse_down_button.set(event.buttons()); // 1=left, 2=right, 3=left-then-also-right (but right-then-also-left is still 2)

      event.stop_propagation();
      event.prevent_default();
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  // mousemove
  {
    let mouse_x = mouse_x.clone();
    let mouse_y = mouse_y.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
      let mx = event.offset_x() as f64;
      let my = event.offset_y() as f64;
      mouse_x.set(mx);
      mouse_y.set(my);

      event.stop_propagation();
      event.prevent_default();
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  // mouseup
  {
    let is_mouse_down = is_mouse_down.clone();
    let last_mouse_up_x = last_mouse_up_x.clone();
    let last_mouse_up_y = last_mouse_up_y.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
      let mx = event.offset_x() as f64;
      let my = event.offset_y() as f64;
      is_mouse_down.set(false);
      last_mouse_up_x.set(mx);
      last_mouse_up_y.set(my);

      event.stop_propagation();
      event.prevent_default();
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  // context menu (just to disable it so we can use rmb for interaction)
  {
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
      event.stop_propagation();
      event.prevent_default();
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("contextmenu", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }


  let ( mut options, mut state, mut factory ) = init();

  if options.print_initial_table {
    print_floor_with_views(&mut options, &mut state, &mut factory);
    print_floor_without_views(&mut options, &mut state, &mut factory);
  }

  // Do not record the cost of belt cells. assume them an ongoing 10k x belt cost cost/min modifier
  // Only record the non-belt costs, which happen far less frequently and mean the delta queue
  // will be less than 100 items. Probably slightly under 50, depending on how we tweak speeds.
  // Even 100 items seems well within acceptable ranges. We could even track 10s (1k items) which
  // might be useful to set consistency thresholds ("you need to maintain this efficiency for at
  // least 10s").

  let window = web_sys::window().unwrap();
  let perf = window.performance().expect("performance should be available"); // Requires web_sys crate feature "Performance"

  {
    let start_time: f64 = perf.now(); // perf.now is almost same as date.now except; it's not based on system clock (so changing system time affects date.now and does not change perf.now: https://developer.mozilla.org/en-US/docs/Web/API/Performance/now)
    log(format!("start time: {}", start_time));

    let context = context.clone();

    let mut real_world_ms_at_start_of_prev_frame = start_time;

    let mut fps: VecDeque<f64> = VecDeque::new();

    let mut cell_selection = CellSelection {
      on: false,
      x: 0.0,
      y: 0.0,
      coord: 0,
      area: false,
      x2: 0.0,
      y2: 0.0,
    };
    let mut mouse_state: MouseState = MouseState {
      canvas_x: 0.0,
      canvas_y: 0.0,

      world_x: 0.0,
      world_y: 0.0,

      cell_x: 0.0,
      cell_y: 0.0,
      cell_coord: 0,

      cell_rel_x: 0.0,
      cell_rel_y: 0.0,

      is_down: false,
      is_dragging: false,
      is_drag_start: false,

      over_offer: false,
      offer_index: 0,
      dragging_offer: false,

      was_down: false,
      was_dragging: false,
      was_up: false,

      last_down_button: 0,

      last_down_canvas_x: 0.0,
      last_down_canvas_y: 0.0,

      last_down_world_x: 0.0,
      last_down_world_y: 0.0,

      last_up_canvas_x: 0.0,
      last_up_canvas_y: 0.0,

      last_up_world_x: 0.0,
      last_up_world_y: 0.0,
    };

    // From https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
      let real_world_ms_at_start_of_curr_frame: f64 = perf.now();
      let real_world_ms_since_start_of_prev_frame: f64 = real_world_ms_at_start_of_curr_frame - real_world_ms_at_start_of_prev_frame;
      real_world_ms_at_start_of_prev_frame = real_world_ms_at_start_of_curr_frame;

      let min = real_world_ms_at_start_of_curr_frame - 1000.0; // minus one second
      while fps.len() > 0 && fps[0] < min {
        fps.pop_front();
      }
      fps.push_back(real_world_ms_at_start_of_curr_frame);

      // ONE_SECOND is how many ticks I want to pass in one real world second
      // We could do an absolute "we should have this many ticks at this point" but that will
      // be problematic if there's ever a pause for whatever reason since there'll be some
      // catch-up frames and they may never catch up.
      // Unfortunately, the current way of calculating the time since previous frame is always
      // lagging one frame behind and has some rounding problems, especially with low % modifiers.
      let ticks_per_second_wanted = ONE_SECOND as f64 * options.speed_modifier;
      let ticks_todo: u64 = ((real_world_ms_since_start_of_prev_frame / 1000.0 * ticks_per_second_wanted) as u64).min(MAX_TICKS_PER_FRAME);
      let estimated_fps = ticks_per_second_wanted / (ticks_todo as f64);
      let variation = 0.1;
      let ( ticks_todo, rounded_fps ) =
        if estimated_fps >= (1.0 - variation) * 30.0 && estimated_fps <= (1.0 + variation) * 30.0 {
          ( (ticks_per_second_wanted / 30.0).round() as u64, 30u64 )
        } else if estimated_fps >= (1.0 - variation) * 60.0 && estimated_fps <= (1.0 + variation) * 60.0 {
          ( (ticks_per_second_wanted / 60.0).round() as u64, 60u64 )
        } else if estimated_fps >= (1.0 - variation) * 100.0 && estimated_fps <= (1.0 + variation) * 100.0 {
          ( (ticks_per_second_wanted / 100.0).round() as u64, 100u64 )
        } else if estimated_fps >= (1.0 - variation) * 120.0 && estimated_fps <= (1.0 + variation) * 120.0 {
          ( (ticks_per_second_wanted / 120.0).round() as u64, 120u64 )
        } else {
          ( ticks_todo, 0u64 )
        };

      if state.reset_next_frame {
        let ( options1, state1, factory1 ) = init();
        options = options1;
        state = state1;
        factory = factory1;
      }

      if !state.paused {
        for _ in 0..ticks_todo.min(MAX_TICKS_PER_FRAME) {
          tick_factory(&mut options, &mut state, &mut factory);
        }

        factory_collect_stats(&mut options, &mut state, &mut factory);
      }

      if options.web_output_cli {
        paint_world_cli(&context, &mut options, &mut state, &factory);
      } else {
        update_mouse_state(&mut factory, &mut mouse_state, mouse_x.get(), mouse_y.get(), last_mouse_down_x.get(), last_mouse_down_y.get(), last_mouse_down_button.get(), last_mouse_up_x.get(), last_mouse_up_y.get());
        last_mouse_down_x.set(0.0);
        last_mouse_down_y.set(0.0);
        last_mouse_up_x.set(0.0);
        last_mouse_up_y.set(0.0);

        // Handle drag-end or click
        handle_input(&mut cell_selection, &mut mouse_state, &mut options, &mut state, &mut factory);

        if factory.changed {
          log(format!("Auto porting after modification"));
          keep_auto_porting(&mut options, &mut state, &mut factory);

          // Recreate cell traversal order
          let prio: Vec<usize> = create_prio_list(&mut options, &mut factory.floor);
          log(format!("Updated prio list: {:?}", prio));
          factory.prio = prio;

          factory.changed = false;
          factory.first_out_at = 0;
          factory.accepted = 0;
          factory.produced = 0;
          factory.trashed = 0;
          factory.supplied = 0;
        }

        // Paint the world (no input or world mutations after this point)

        context.set_font(&"12px monospace");

        // Clear canvas
        context.set_fill_style(&"#E86A17".into());
        // context.set_fill_style(&"lightblue".into());
        context.fill_rect(0.0, 0.0, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);

        context.set_stroke_style(&"#aaa".into());
        context.stroke_rect(WORLD_OFFSET_X, WORLD_OFFSET_Y, FLOOR_CELLS_W as f64 * CELL_W, FLOOR_CELLS_H as f64 * CELL_H);

        paint_green_debug(&options, &state, &context, &fps, real_world_ms_at_start_of_curr_frame, real_world_ms_since_start_of_prev_frame, ticks_todo, estimated_fps, rounded_fps, &factory, &mouse_state);

        paint_top_stats(&context, &mut factory);
        paint_ui_offers(&context, &mut factory, &mouse_state);
        paint_ui_buttons(&mut options, &mut state, &context, &mouse_state);
        paint_ui_buttons2(&mut options, &mut state, &context, &mouse_state);

        // TODO: wait for tiles to be loaded because first few frames won't paint anything while the tiles are loading...
        paint_background_tiles(&options, &state, &context, &factory, &belt_tile_images, &img_machine4);
        paint_ports(&context, &factory);
        paint_belt_items(&context, &factory, &part_tile_sprite);

        paint_mouse_cursor(&context, &mouse_state);

        if state.mouse_mode_erasing {
          // Don't paint anything or paint the invalid belt stub
          if bounds_check(mouse_state.cell_x, mouse_state.cell_y, 0.0, 0.0, FLOOR_CELLS_W as f64, FLOOR_CELLS_H as f64) {
            // Do paint the current cell
            context.set_stroke_style(&"red".into());
            context.stroke_rect(WORLD_OFFSET_X + mouse_state.cell_x * CELL_W, WORLD_OFFSET_Y + mouse_state.cell_y * CELL_H, CELL_W, CELL_H);
          }
        }
        else if state.mouse_mode_selecting {
          if mouse_state.is_down {
            let down_cell_x = ((mouse_state.last_down_world_x - WORLD_OFFSET_X) / CELL_W).floor();
            let down_cell_y = ((mouse_state.last_down_world_y - WORLD_OFFSET_Y) / CELL_H).floor();
            if down_cell_x >= 0.0 && down_cell_y >= 0.0 && is_floor(down_cell_x as usize, down_cell_y as usize) && mouse_state.cell_x >= 0.0 && mouse_state.cell_y >= 0.0 && is_floor(mouse_state.cell_x as usize, mouse_state.cell_y as usize) {
              // Draw dotted stroke rect around cells from mouse down cell to current cell
              context.set_stroke_style(&"blue".into());
              let now_cell_x = mouse_state.cell_x.floor();
              let now_cell_y = mouse_state.cell_y.floor();
              context.stroke_rect(WORLD_OFFSET_X + down_cell_x.min(now_cell_x) * CELL_W, WORLD_OFFSET_Y + down_cell_y.min(now_cell_y) * CELL_H, (1.0 + (down_cell_x - now_cell_x).abs()) * CELL_W, (1.0 + (down_cell_y - now_cell_y).abs()) * CELL_H);
            }
          } else {
            if cell_selection.on {
              // Rectangle around current selection, if any
              context.set_stroke_style(&"blue".into());
              context.stroke_rect(WORLD_OFFSET_X + cell_selection.x * CELL_W, WORLD_OFFSET_Y + cell_selection.y * CELL_H, (1.0 + (cell_selection.x - cell_selection.x2).abs()) * CELL_W, (1.0 + (cell_selection.y - cell_selection.y2).abs()) * CELL_H);
              if state.selected_area_copy.len() > 0 {
                // Draw a rectangle to indicate paste area. Always a rectangle of sorts.
                let w = state.selected_area_copy[0].len(); // There must be at least one
                let h = state.selected_area_copy.len();
                context.set_stroke_style(&"green".into());
                context.stroke_rect(WORLD_OFFSET_X + cell_selection.x * CELL_W, WORLD_OFFSET_Y + cell_selection.y * CELL_H, w as f64 * CELL_W, h as f64 * CELL_H);
              }
            }
            if bounds_check(mouse_state.cell_x, mouse_state.cell_y, 0.0, 0.0, FLOOR_CELLS_W as f64, FLOOR_CELLS_H as f64) {
              // Rectangle around current cell
              context.set_stroke_style(&"red".into());
              context.stroke_rect(WORLD_OFFSET_X + mouse_state.cell_x * CELL_W, WORLD_OFFSET_Y + mouse_state.cell_y * CELL_H, CELL_W, CELL_H);
            }
          }
        }
        else if mouse_state.dragging_offer {
          let offer = &factory.offers[mouse_state.offer_index];

          // Paint drop zone over the edge cells
          context.set_fill_style(&"#00004444".into());

          // Face out illegal options
          let ( paint_at_x, paint_at_y, legal ) =
            if offer.kind == CellKind::Machine {
              // All edges
              context.fill_rect(WORLD_OFFSET_X, WORLD_OFFSET_Y, CELL_W, WORLD_HEIGHT - CELL_H);
              context.fill_rect(WORLD_OFFSET_X + CELL_W, WORLD_OFFSET_Y, WORLD_WIDTH - CELL_W, CELL_H);
              context.fill_rect(WORLD_OFFSET_X + WORLD_WIDTH - CELL_W, WORLD_OFFSET_Y + CELL_H, CELL_W, WORLD_HEIGHT - CELL_H);
              context.fill_rect(WORLD_OFFSET_X, WORLD_OFFSET_Y + WORLD_HEIGHT - CELL_H, WORLD_WIDTH - CELL_W, CELL_H);

              // Note that mouse cell x is not where the top-left most cell of the machine would be
              let top_left_machine_cell_x = world_x_to_top_left_cell_x_while_dragging_offer(mouse_state.world_x, offer.cell_width);
              let top_left_machine_cell_y = world_y_to_top_left_cell_y_while_dragging_offer(mouse_state.world_y, offer.cell_height);
              // Make sure the entire machine fits, not just the center or topleft cell
              if
                !bounds_check(top_left_machine_cell_x, top_left_machine_cell_y, 1.0, 1.0, FLOOR_CELLS_W as f64 - (offer.cell_width as f64), FLOOR_CELLS_H as f64 - (offer.cell_height as f64))
              {
                // Do not snap if machine would cover the edge
                let ox = mouse_state.world_x - ((offer.cell_width as f64) * (CELL_W as f64) / 2.0 );
                let oy = mouse_state.world_y - ((offer.cell_height as f64) * (CELL_H as f64) / 2.0 );
                ( ox, oy, false )
              } else {
                ( WORLD_OFFSET_X + top_left_machine_cell_x.round() * CELL_W, WORLD_OFFSET_Y + top_left_machine_cell_y.round() * CELL_H, true )
              }
            } else {
              // Corners
              context.fill_rect(WORLD_OFFSET_X, WORLD_OFFSET_Y, CELL_W, CELL_H);
              context.fill_rect(WORLD_OFFSET_X + WORLD_WIDTH - CELL_W, WORLD_OFFSET_Y, CELL_W, CELL_H);
              context.fill_rect(WORLD_OFFSET_X, WORLD_OFFSET_Y + WORLD_HEIGHT - CELL_H, CELL_W, CELL_H);
              context.fill_rect(WORLD_OFFSET_X + WORLD_WIDTH - CELL_W, WORLD_OFFSET_Y + WORLD_HEIGHT - CELL_H, CELL_W, CELL_H);
              // Center
              context.fill_rect(WORLD_OFFSET_X + CELL_W, WORLD_OFFSET_Y + CELL_H, WORLD_WIDTH - CELL_W * 2.0, WORLD_HEIGHT - CELL_H * 2.0);

              // Snap if x or y is edge but not both or neither
              if
                (mouse_state.cell_x == 0.0 || mouse_state.cell_x == FLOOR_CELLS_W as f64 - 1.0)
                  !=
                (mouse_state.cell_y == 0.0 || mouse_state.cell_y == FLOOR_CELLS_H as f64 - 1.0)
              {
                ( WORLD_OFFSET_X + mouse_state.cell_x * CELL_W, WORLD_OFFSET_Y + mouse_state.cell_y * CELL_H, true )
              } else {
                ( mouse_state.world_x - ((CELL_W as f64) / 2.0), mouse_state.world_y - ((CELL_H as f64) / 2.0), false )
              }
            };

          fn paint_illegal(context: &Rc<web_sys::CanvasRenderingContext2d>, x: f64, y: f64, w: f64, h: f64) {
            // tbd. dont like this part but it gets the job done I guess.
            context.set_stroke_style(&"red".into());
            context.stroke_rect(x, y, w, h);
            // context.set_line_width(3.0);
            // context.set_line_cap("round");
            let n = 11.0;
            let ws = w / n;
            let hs = h / n;
            for i in 0..ws as u32 {
              for j in 0..hs as u32 {
                let fi = i as f64;
                let fj = j as f64;

                context.begin_path();
                context.move_to(x, y + fj * n);
                context.line_to(x + w, y + fj * n);
                context.stroke();

                context.begin_path();
                context.move_to(x + fi * n, y);
                context.line_to(x + fi * n, y + h);
                context.stroke();
              }
            }
          }

          context.set_fill_style(&"black".into());
          match offer.kind {
            CellKind::Empty => panic!("no"),
            CellKind::Belt => panic!("no"),
            CellKind::Machine => {
              context.set_fill_style(&COLOR_MACHINE_SEMI.into());
              context.fill_rect(paint_at_x, paint_at_y, (offer.cell_width as f64) * CELL_W, (offer.cell_height as f64) * CELL_H);
              if !legal { paint_illegal(&context, paint_at_x, paint_at_y, (offer.cell_width as f64) * CELL_W, (offer.cell_height as f64) * CELL_H); }
              context.set_fill_style(&"black".into());
              context.fill_text("M", paint_at_x + (offer.cell_width as f64) * CELL_W / 2.0 - 5.0, paint_at_y + (offer.cell_height as f64) * CELL_H / 2.0 + 2.0).expect("no error")
            },
            CellKind::Supply => {
              context.set_fill_style(&COLOR_SUPPLY_SEMI.into());
              context.fill_rect(paint_at_x, paint_at_y, CELL_W, CELL_H);
              if !legal { paint_illegal(&context, paint_at_x, paint_at_y, CELL_W, CELL_H); }
              context.set_fill_style(&"black".into());
              context.fill_text("S", paint_at_x + CELL_W / 2.0 - 5.0, paint_at_y + CELL_H / 2.0 + 2.0).expect("no error")
            },
            CellKind::Demand => {
              context.set_fill_style(&COLOR_DEMAND_SEMI.into());
              context.fill_rect(paint_at_x, paint_at_y, CELL_W, CELL_H);
              if !legal { paint_illegal(&context, paint_at_x, paint_at_y, CELL_W, CELL_H); }
              context.set_fill_style(&"black".into());
              context.fill_text("D", paint_at_x + CELL_W / 2.0 - 5.0, paint_at_y + CELL_H / 2.0 + 2.0).expect("no error")
            },
          }
        }
        else if mouse_state.cell_x >= 0.0 && mouse_state.cell_y >= 0.0 && mouse_state.cell_x < FLOOR_CELLS_W as f64 && mouse_state.cell_y < FLOOR_CELLS_H as f64 {
          paint_mouse_stuff_in_floor(&context, &factory, &cell_selection, &mouse_state, &belt_tile_images);
        }

        paint_cell_editor(&context, &factory, &cell_selection, &mouse_state);
        paint_machine_editor(&context, &factory, &cell_selection, &mouse_state);
        paint_supply_editor(&context, &factory, &cell_selection, &mouse_state);
        paint_demand_editor(&context, &factory, &cell_selection, &mouse_state);
      }

      // Schedule next frame
      request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
  }

  Ok(())
}

fn world_x_to_top_left_cell_x_while_dragging_offer(world_x: f64, offer_width: usize) -> f64 {
  // Abstracted this to make sure the preview and actual action use the same computation
  let compx = if offer_width % 2 == 1 { 0.0 } else { 0.5 };
  let ox = ((world_x + -WORLD_OFFSET_X).floor() / CELL_W + compx).floor() - (offer_width / 2) as f64;
  return ox;
}
fn world_y_to_top_left_cell_y_while_dragging_offer(world_y: f64, offer_height: usize) -> f64 {
  let compy = if offer_height % 2 == 1 { 0.0 } else { 0.5 };
  let oy = ((world_y + -WORLD_OFFSET_Y).floor() / CELL_H + compy).floor() - (offer_height / 2) as f64;
  return oy;
}

fn handle_input(cell_selection: &mut CellSelection, mouse_state: &mut MouseState, options: &mut Options, state: &mut State, factory: &mut Factory) {
  let mut was_offer_drag_start = false;

  if mouse_state.is_drag_start {
    // Do this one before the erasing/selecting. It may cancel those states even if active.
    let ( over_offer, offer_index ) = hit_test_offers(factory, mouse_state.last_down_world_x, mouse_state.last_down_world_y);
    if over_offer {
      // Need to remember which offer we are currently dragging.
      log(format!("Started to drag from offer {} ({:?})", offer_index, factory.offers[offer_index].kind));
      mouse_state.dragging_offer = true;
      mouse_state.over_offer = over_offer;
      was_offer_drag_start = true;
      state.mouse_mode_erasing = false;
      state.mouse_mode_selecting = false;
    }
  }

  if state.mouse_mode_erasing {
    if mouse_state.is_down {
      if bounds_check(mouse_state.world_x, mouse_state.world_y, WORLD_OFFSET_X, WORLD_OFFSET_Y, WORLD_OFFSET_X + WORLD_WIDTH, WORLD_OFFSET_Y + WORLD_HEIGHT) {
        // On the floor. Delete anything.
        let coord = mouse_state.cell_coord;
        if factory.floor[coord].kind != CellKind::Empty {
          floor_delete_cell_at_partial(options, state, factory, coord);
          factory.changed = true;
        }
      }
    }
    else if mouse_state.was_up {
      // Still allow to use menu buttons while deleting, but ignore other hit boxes
      log(format!("({}) handle_mouse_up_over_menu_buttons from erasing", factory.ticks));
      handle_mouse_up_over_menu_buttons(cell_selection, mouse_state, options, state, factory);
    }
    return;
  }

  if state.mouse_mode_selecting {
    if mouse_state.was_up {
      log(format!("mouse up with selection mode enabled..."));
      let down_cell_x = ((mouse_state.last_down_world_x - WORLD_OFFSET_X) / CELL_W).floor();
      let down_cell_y = ((mouse_state.last_down_world_y - WORLD_OFFSET_Y) / CELL_H).floor();
      if mouse_state.cell_x >= 0.0 && mouse_state.cell_y >= 0.0 && is_floor(mouse_state.cell_x as usize, mouse_state.cell_y as usize) {
        log(format!("  was up on floor"));
        if down_cell_x >= 0.0 && down_cell_y >= 0.0 && is_floor(down_cell_x as usize, down_cell_y as usize) {
          log(format!("  was down in floor, too. ok!"));
          let now_cell_x = mouse_state.cell_x.floor();
          let now_cell_y = mouse_state.cell_y.floor();

          cell_selection.x = down_cell_x.min(now_cell_x);
          cell_selection.y = down_cell_y.min(now_cell_y);
          cell_selection.x2 = down_cell_x.max(now_cell_x);
          cell_selection.y2 = down_cell_y.max(now_cell_y);
          cell_selection.on = true;
        } else {
          log(format!("  not down in floor"));
        }
      } else {
        // Still allow to use menu buttons while deleting, but ignore other hit boxes
        log(format!("({}) handle_mouse_up_over_menu_buttons() with selection mode enabled", factory.ticks));
        handle_mouse_up_over_menu_buttons(cell_selection, mouse_state, options, state, factory);
      }
    }
    return;
  }

  if mouse_state.is_drag_start {
    if bounds_check(mouse_state.last_down_world_x, mouse_state.last_down_world_y, WORLD_OFFSET_X, WORLD_OFFSET_Y, WORLD_OFFSET_X + WORLD_WIDTH, WORLD_OFFSET_Y + WORLD_HEIGHT) {
      // Drag start on floor. Do nothing here.
      // This is computed on the fly and state is already recorded through other means.
      log(format!("Started dragging from floor"));
    }
  }

  if mouse_state.was_up {
    if mouse_state.is_dragging {
      // This is more a visual thing I think
    }
    // Was the click inside the painted world?
    // In that case we change/toggle the cell selection
    else if bounds_check(mouse_state.last_up_world_x, mouse_state.last_up_world_y, WORLD_OFFSET_X, WORLD_OFFSET_Y, WORLD_OFFSET_X + WORLD_WIDTH, WORLD_OFFSET_Y + WORLD_HEIGHT) {
      on_up_inside_floor(options, state, factory, cell_selection, &mouse_state);
    }
    else {
      log(format!("({}) handle_mouse_up_over_menu_buttons from normal", factory.ticks));
      handle_mouse_up_over_menu_buttons(cell_selection, mouse_state, options, state, factory);
    }

    mouse_state.dragging_offer = false;
  }
}
fn handle_mouse_up_over_menu_buttons(cell_selection: &mut CellSelection, mouse_state: &mut MouseState, options: &mut Options, state: &mut State, factory: &mut Factory) {
  // Was one of the buttons below the floor clicked?
  if bounds_check(mouse_state.last_up_world_x, mouse_state.last_up_world_y, UI_BUTTONS_OX, UI_BUTTONS_OY, UI_BUTTONS_OX + UI_BUTTON_COUNT * (UI_BUTTON_W + UI_BUTTON_SPACING), UI_BUTTONS_OY + UI_BUTTON_H) {
    let button_index = (mouse_state.last_up_world_x - UI_BUTTONS_OX) / (UI_BUTTON_W + UI_BUTTON_SPACING);
    if button_index % 1.0 < (UI_BUTTON_W / (UI_BUTTON_W + UI_BUTTON_SPACING)) {
      log(format!("clicked inside button {}", button_index));
      match button_index.floor() as u8 {
        0 => { // Empty
          log(format!("Removing all cells from the factory..."));
          for coord in 0..factory.floor.len() {
            let (x, y) = to_xy(coord);
            factory.floor[coord] = empty_cell(x, y);
          }
          factory.changed = true;
        }
        1 => { // Unbelt
          log(format!("Removing all belts from the factory"));
          for coord in 0..factory.floor.len() {
            let (x, y) = to_xy(coord);
            match factory.floor[coord].kind {
              CellKind::Belt => factory.floor[coord] = empty_cell(x, y),
              CellKind::Empty => (),
              CellKind::Demand => (),
              CellKind::Supply => (),
              CellKind::Machine => {
                factory.floor[coord].port_u = Port::None;
                factory.floor[coord].port_r = Port::None;
                factory.floor[coord].port_d = Port::None;
                factory.floor[coord].port_l = Port::None;
              },
            }
          }
          factory.changed = true;
        }
        2 => { // Unpart
          log(format!("Removing all part data from the factory"));
          for coord in 0..factory.floor.len() {
            let (x, y) = to_xy(coord);
            match factory.floor[coord].kind {
              CellKind::Belt => {
                belt_receive_part(factory, coord, Direction::Up, part_none());
              },
              CellKind::Empty => (),
              CellKind::Demand => (),
              CellKind::Supply => {
                factory.floor[coord].supply.part_at = 0;
                factory.floor[coord].supply.last_part_out_at = 0;
              },
              CellKind::Machine => {
                factory.floor[coord].machine.input_1_have = part_none();
                factory.floor[coord].machine.input_2_have = part_none();
                factory.floor[coord].machine.input_3_have = part_none();
                factory.floor[coord].machine.start_at = 0;
              },
            }
          }
          factory.changed = true;
        }
        3 => { // Undir
          log(format!("Applying undir..."));
          for coord in 0..factory.floor.len() {
            let (x, y) = to_xy(coord);
            if factory.floor[coord].kind != CellKind::Supply && factory.floor[coord].kind != CellKind::Demand {
              if factory.floor[coord].port_u != Port::None {
                cell_set_port_u_to(factory, coord, Port::Unknown, to_coord_up(coord));
              }
              if factory.floor[coord].port_r != Port::None {
                cell_set_port_r_to(factory, coord, Port::Unknown, to_coord_right(coord));
              }
              if factory.floor[coord].port_d != Port::None {
                cell_set_port_d_to(factory, coord, Port::Unknown, to_coord_down(coord));
              }
              if factory.floor[coord].port_l != Port::None {
                cell_set_port_l_to(factory, coord, Port::Unknown, to_coord_left(coord));
              }
            }
          }
          factory.changed = true;
        }
        4 => { // Dump
          log(format!("Dumping factory..."));
          log(format!("\n{}", generate_floor_dump(options, state, &factory).join("\n")));
        }
        5 => {
          log(format!("Restarting game at the start of next frame"));
          state.reset_next_frame = true;
        }
        _ => panic!("Hit the panic button. Or another button without implementation."),
      }
    } else {
      log(format!("clicked margin after button {}", button_index));
    }
  }
  // Second row of buttons?
  else if bounds_check(mouse_state.last_up_world_x, mouse_state.last_up_world_y, UI_BUTTONS_OX, UI_BUTTONS_OY2, UI_BUTTONS_OX + UI_BUTTON_COUNT * (UI_BUTTON_W + UI_BUTTON_SPACING), UI_BUTTONS_OY2 + UI_BUTTON_H) {
    log(format!("Second row of buttons"));
    let button_index = (mouse_state.last_up_world_x - UI_BUTTONS_OX) / (UI_BUTTON_W + UI_BUTTON_SPACING);
    if button_index % 1.0 < (UI_BUTTON_W / (UI_BUTTON_W + UI_BUTTON_SPACING)) {
      log(format!("({}) clicked inside button {}", factory.ticks, button_index));
      match button_index.floor() as u8 {
        0 => { // Draw / Erase
          log(format!("toggle draw/erase mode"));
          state.mouse_mode_erasing = !state.mouse_mode_erasing;
          state.mouse_mode_selecting = false;
          cell_selection.area = false;
          cell_selection.on = false;
          state.selected_area_copy = vec!(); // Or retain this?
        }
        1 => { // Select
          log(format!("Toggle selection mode"));
          state.mouse_mode_selecting = !state.mouse_mode_selecting;
          state.mouse_mode_erasing = false;
          cell_selection.area = state.mouse_mode_selecting;
          cell_selection.on = false;
          state.selected_area_copy = vec!(); // Or retain this?
        }
        2 => { // Copy
          log(format!("Copy selection"));
          if state.mouse_mode_selecting && cell_selection.on {
            // clone each cell in the area verbatim
            // Store this copy in... state
            let mut area = vec!();
            // Only copy belts. Machines are too hard to deal with. Edge stuff is too tricky.
            let cox = cell_selection.x.min(cell_selection.x2) as usize;
            let coy = cell_selection.y.min(cell_selection.y2) as usize;
            for y in 0..1 + (cell_selection.y - cell_selection.y2).abs() as usize {
              area.push(vec!());
              for x in 0..1 + (cell_selection.x - cell_selection.x2).abs() as usize {
                area[y].push(factory.floor[to_coord(cox + x, coy + y)].clone());
              }
            }

            state.selected_area_copy = area;
          }
        }
        3 => { // Paste
          log(format!("Paste from clipboard"));
          if state.mouse_mode_selecting && cell_selection.on && state.selected_area_copy.len() > 0 {
            let selected_ox = cell_selection.x.min(cell_selection.x2) as usize;
            let selected_oy = cell_selection.y.min(cell_selection.y2) as usize;
            let clipboard_w = state.selected_area_copy[0].len();
            let clipboard_h = state.selected_area_copy.len();

            for y in 0..clipboard_h {
              for x in 0..clipboard_w {
                let cx = selected_ox + x;
                let cy = selected_oy + y;
                let coord = to_coord(cx, cy);

                paste_one_cell(options, state, factory, x, y, clipboard_w, clipboard_h, cx, cy, coord);
              }
            }

            for y in 0..clipboard_h {
              for x in 0..clipboard_w {
                // Only consider edge and corner cells of the pasted area
                // if x == 0 || x == clipboard_w -1 || y == 0 || y == clipboard_h -1 {
                  let cx = selected_ox + x;
                  let cy = selected_oy + y;
                  if is_middle(cx, cy) {
                    let coord = to_coord(cx, cy);
                    log(format!("patching edge: {} {} -> {}", x, y, coord));
                    log(format!("  fixing belt {:?} {:?} {:?} {:?}", factory.floor[coord].port_u, factory.floor[coord].port_r, factory.floor[coord].port_d, factory.floor[coord].port_l));

                    // Belt may be in an inconsistent state if it was connected to a machine, demand, or supply, since we don't copy those.
                    // For that reason we have to do a sanity check on each cell and remove/update any incorrectly connected ports
                    // This should only affect ports connecting to other cells of the paste but I suppose that overhead doesn't matter here.
                    belt_fix_semi_connected_ports_partial(options, state, factory, coord);

                    // At this point the ports and that of the neighbor should be fixed. Fix the meta.
                    fix_belt_meta(factory, coord);
                    // And fix the .ins and .outs of this and its neighbor
                    belt_discover_ins_and_outs(factory, coord);
                  } else {
                    log(format!("Skipping {} {} because they are not in the middle of the floor", cx, cy));
                  }
                }
              // }
            }

            factory.changed = true;
          }
        }
        4 => { // tbd
          log(format!("(no button here)"));
        }
        5 => { // tbd
          log(format!("(no button here)"));
        }
        6 => { // tbd
          log(format!("(no button here)"));
        }
        _ => panic!("Hit a button2 without implementation."),
      }
    } else {
      log(format!("clicked margin after button {}", button_index));
    }
  }
  else if hit_check_speed_bubbles_any(options, state, mouse_state) {
    on_click_speed_bubbles(options, state, mouse_state);
  }
}
fn update_mouse_state(factory: &mut Factory, mouse_state: &mut MouseState, mouse_x: f64, mouse_y: f64, last_mouse_down_x: f64, last_mouse_down_y: f64, last_mouse_down_button: u16, last_mouse_up_x: f64, last_mouse_up_y: f64) {
  // https://docs.rs/web-sys/0.3.28/web_sys/struct.CanvasRenderingContext2d.html

  // Reset
  mouse_state.is_drag_start = false;
  mouse_state.was_down = false;
  mouse_state.was_up = false;
  mouse_state.was_dragging = false;
  mouse_state.over_offer = false;

  // Mouse coords
  // Note: mouse2world coord is determined by _css_ size, not _canvas_ size
  mouse_state.canvas_x = mouse_x;
  mouse_state.canvas_y = mouse_y;
  mouse_state.world_x = mouse_x / CANVAS_CSS_WIDTH * CANVAS_WIDTH;
  mouse_state.world_y = mouse_y / CANVAS_CSS_HEIGHT * CANVAS_HEIGHT;
  mouse_state.cell_x = ((mouse_x - WORLD_OFFSET_X) / CELL_W).floor();
  mouse_state.cell_y = ((mouse_y - WORLD_OFFSET_Y) / CELL_H).floor();
  mouse_state.cell_coord = to_coord(mouse_state.cell_x as usize, mouse_state.cell_y as usize);
  mouse_state.cell_rel_x = ((mouse_x - WORLD_OFFSET_X) / CELL_W) - mouse_state.cell_x;
  mouse_state.cell_rel_y = ((mouse_y - WORLD_OFFSET_Y) / CELL_H) - mouse_state.cell_y;

  if !mouse_state.is_dragging {
    // When already dragging do not update offer visual state, do not record the "over" state at all
    // When dragging an offer, the offer_index will be set to the initial offer index (keep it!)
    let (over_offer, offer_index) = hit_test_offers(factory, mouse_state.world_x, mouse_state.world_y);
    mouse_state.over_offer = over_offer;
    mouse_state.offer_index = offer_index;
  }

  // on mouse down
  if last_mouse_down_x > 0.0 || last_mouse_down_y > 0.0 {
    mouse_state.last_down_button = last_mouse_down_button;
    mouse_state.last_down_canvas_x = last_mouse_down_x;
    mouse_state.last_down_canvas_y = last_mouse_down_y;
    mouse_state.last_down_world_x = last_mouse_down_x / CANVAS_CSS_WIDTH * CANVAS_WIDTH;
    mouse_state.last_down_world_y = last_mouse_down_y / CANVAS_CSS_HEIGHT * CANVAS_HEIGHT;
    mouse_state.is_down = true;
    mouse_state.was_down = true; // this frame, in case there's also an up event
  }

  // on drag start (maybe)
  // determine whether mouse is considered to be dragging (there's a buffer of movement before
  // we consider a mouse down to mouse up to be dragging. But once we do, we stick to it.)
  if mouse_state.is_down && !mouse_state.is_dragging {
    // 5 world pixels? sensitivity tbd
    if (mouse_state.last_down_world_x - mouse_state.world_x).abs() > 5.0 || (mouse_state.last_down_world_y - mouse_state.world_y).abs() > 5.0 {
      mouse_state.is_drag_start = true;
      mouse_state.is_dragging = true;
    }
  }

  // on mouse up
  if last_mouse_up_x > 0.0 || last_mouse_up_y > 0.0 {
    mouse_state.last_up_canvas_x = last_mouse_up_x;
    mouse_state.last_up_canvas_y = last_mouse_up_y;
    mouse_state.last_up_world_x = last_mouse_up_x / CANVAS_CSS_WIDTH * CANVAS_WIDTH;
    mouse_state.last_up_world_y = last_mouse_up_y / CANVAS_CSS_HEIGHT * CANVAS_HEIGHT;
    mouse_state.is_down = false;
    mouse_state.was_up = true;
    if mouse_state.is_drag_start {
      mouse_state.is_drag_start = false; // ignore :shrug:
    }
    if mouse_state.is_dragging {
      mouse_state.is_dragging = false;
      mouse_state.was_dragging = true;
    }
  }
}
fn on_up_inside_floor(options: &mut Options, state: &mut State, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState) {
  log(format!("on_up_inside_floor()"));

  if mouse_state.was_dragging {
    if mouse_state.dragging_offer {
      on_drag_end_offer_over_floor(options, state, factory, mouse_state);
    }
    // Drag ended on the floor, did drag start on the floor?
    else if bounds_check(mouse_state.last_down_world_x - WORLD_OFFSET_X, mouse_state.last_down_world_y - WORLD_OFFSET_Y, 0.0,  0.0,WORLD_WIDTH, WORLD_HEIGHT) {
      // Is the mouse currently on the floor?
      on_drag_end_inside_floor(options, state, factory, cell_selection, mouse_state);
    } else {
      log(format!("Drag ended on floor but did not start there. noop"));
    }
  } else {
    on_click_inside_floor(options, state, factory, cell_selection, mouse_state);
  }
}
fn on_drag_end_offer_over_floor(options: &mut Options, state: &mut State, factory: &mut Factory, mouse_state: &MouseState) {
  log(format!("on_drag_offer_into_floor()"));

  let last_mouse_up_cell_x = ((mouse_state.last_up_world_x - WORLD_OFFSET_X) / CELL_W).floor();
  let last_mouse_up_cell_y = ((mouse_state.last_up_world_y - WORLD_OFFSET_Y) / CELL_H).floor();
  let last_mouse_up_cell_coord = to_coord(last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize);
  let last_mouse_up_inside_cell_x = ((mouse_state.last_up_world_x - WORLD_OFFSET_X) / CELL_W) - last_mouse_up_cell_x;
  let last_mouse_up_inside_cell_y = ((mouse_state.last_up_world_y - WORLD_OFFSET_Y) / CELL_H) - last_mouse_up_cell_y;

  let offer = &factory.offers[mouse_state.offer_index];

  // Was dragging an offer and released it on the floor
  // Offers have cell constraints. In particular, supply/demand can only go on edges and
  // machines can not go on the edge.
  match offer.kind {
    CellKind::Machine => {
      // would every part of the machine be on a middle cell, not edge?
      let ocw = offer.cell_width;
      let och = offer.cell_height;
      let cx = world_x_to_top_left_cell_x_while_dragging_offer(mouse_state.last_up_world_x, ocw);
      let cy = world_y_to_top_left_cell_y_while_dragging_offer(mouse_state.last_up_world_y, och);
      // Make sure the entire machine fits, not just the center or topleft cell
      if bounds_check(cx, cy, 1.0, 1.0, FLOOR_CELLS_W as f64 - (ocw as f64), FLOOR_CELLS_H as f64 - (och as f64)) {
        let ccoord = to_coord(cx as usize, cy as usize);

        log(format!("Dropped a machine on the floor. Deploying... {} {} @{}", cx, cy, ccoord));

        // Fill the rest with sub machine cells
        for i in 0..ocw {
          for j in 0..och {
            let x = cx as usize + i;
            let y = cy as usize + j;
            let coord = to_coord(x, y);

            // Meh. But we want to remember this state for checks below.
            let ( port_u, port_r, port_d, port_l ) = match factory.floor[coord] {
              super::cell::Cell { port_u, port_r, port_d, port_l, .. } => ( port_u, port_r, port_d, port_l )
            };

            // Make sure to drop machines properly. Belts are 1x1 so no problem. Empty are fine.
            if factory.floor[coord].kind == CellKind::Machine {
              floor_delete_cell_at_partial(options, state, factory, coord);
            }

            if i == 0 && j == 0 {
              // Top-left cell is the main_coord here
              factory.floor[coord] = machine_main_cell(
                x, y,
                ocw, och,
                part_c(factory.offers[mouse_state.offer_index].machine_input1),
                part_c(factory.offers[mouse_state.offer_index].machine_input2),
                part_c(factory.offers[mouse_state.offer_index].machine_input3),
                part_c(factory.offers[mouse_state.offer_index].machine_output),
                factory.offers[mouse_state.offer_index].speed,
                1, 1
              );
            } else {
              factory.floor[coord] = machine_sub_cell(x, y, ccoord);
            }
            factory.floor[ccoord].machine.coords.push(coord);

            factory.floor[coord].port_u = if j == 0 { port_u } else { Port::None };
            factory.floor[coord].port_r = if i == ocw - 1 { port_r } else { Port::None };
            factory.floor[coord].port_d = if j == och - 1 { port_d } else { Port::None };
            factory.floor[coord].port_l = if i == 0 { port_l } else { Port::None };
          }
        }

        log(format!("Attaching machine to neighbor dead ending belts"));
        for i in 0..factory.floor[ccoord].machine.coords.len() {
          let coord = factory.floor[ccoord].machine.coords[i];
          connect_to_neighbor_dead_end_belts(options, state, factory, coord);
        }

        machine_discover_ins_and_outs(factory, ccoord);

        factory.changed = true;
      } else {
        log(format!("Dropped a machine on the edge. Ignoring. {} {}", last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize));
      }
    }
    CellKind::Supply => {
      if is_edge_not_corner(last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize) {
        log(format!("Dropped a supply on an edge cell that is not corner. Deploying... {} {}", last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize));
        // If there's already something on this cell then we need to remove it first
        if factory.floor[last_mouse_up_cell_coord].kind != CellKind::Empty {
          // Must be supply or demand
          // We should be able to replace this one with the new tile without having to update
          // the neighbors (if any). We do have to update the prio list (in case demand->supply).
          log(format!("Remove old edge cell..."));
          floor_delete_cell_at_partial(options, state, factory, last_mouse_up_cell_coord);
        }
        log(format!("Add new supply cell..."));
        factory.floor[last_mouse_up_cell_coord] = supply_cell(last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize, part_c(factory.offers[mouse_state.offer_index].supply_icon), 1, 1, 1);
        connect_to_neighbor_dead_end_belts(options, state, factory, last_mouse_up_cell_coord);
        factory.changed = true;
      } else {
        log(format!("Dropped a supply on the floor or a corner. Ignoring. {} {}", last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize));
      }
    }
    CellKind::Demand => {
      if is_edge_not_corner(last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize) {
        log(format!("Dropped a demand on an edge cell that is not corner. Deploying..."));
        // If there's already something on this cell then we need to remove it first
        if factory.floor[last_mouse_up_cell_coord].kind != CellKind::Empty {
          // Must be supply or demand
          // We should be able to replace this one with the new tile without having to update
          // the neighbors (if any). We do have to update the prio list (in case demand->supply).
          log(format!("Remove old edge cell..."));
          floor_delete_cell_at_partial(options, state, factory, last_mouse_up_cell_coord);
        }
        log(format!("Add new demand cell..."));
        factory.floor[last_mouse_up_cell_coord] = demand_cell(last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize, part_c(factory.offers[mouse_state.offer_index].supply_icon));
        connect_to_neighbor_dead_end_belts(options, state, factory, last_mouse_up_cell_coord);
        factory.changed = true;
      } else {
        log(format!("Dropped a demand on the floor or a corner. Ignoring. {} {}", last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize));
      }
    }
    CellKind::Empty => panic!("no"),
    CellKind::Belt => panic!("no"),
  }
}
fn on_drag_end_inside_floor(options: &mut Options, state: &mut State, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState) {
  log(format!("on_drag_end_inside_floor()"));

  // Finalize pathing, regenerate floor
  let track = ray_trace_dragged_line(
    factory,
    ((mouse_state.last_down_world_x - WORLD_OFFSET_X) / CELL_W).floor(),
    ((mouse_state.last_down_world_y - WORLD_OFFSET_Y) / CELL_H).floor(),
    mouse_state.cell_x.floor(),
    mouse_state.cell_y.floor(),
    false
  );

  log(format!("track to solidify: {:?}, button {}", track, mouse_state.last_down_button));

  // Special cases:
  // - len=1
  //   - lmb: ignore (cell selection toggle for a click, not a drag)
  //   - rmb:
  //     - delete cell if it is a belt that has one or zero ports
  //     - else clear part from cell
  // - len=2
  //   - lmb:
  //     - change empty cells to belts
  //     - create ports between only those two cells if possible
  //   - rmb:
  //     - delete ports between those two cells only
  //     - only delete affected cells if they have zero ports afterwards
  // - len>2
  //   - lmb:
  //     - convert any empty cell in the track to a belt, retain other cell kinds
  //     - connect head and tail (if belt) to any adjacent non-empty cell
  //     - connect middle parts only to prev and next part of the dragged track
  //     - if any existing cell is already belt, make sure to retain existing ports too
  //   - rmb: delete cells, disconnect them from everywhere


  let len = track.len();

  if len == 1 {
    if mouse_state.last_down_button == 1 {
      // Ignore for a drag. Allows you to cancel a drag.
    } else if mouse_state.last_down_button == 2 {
      // Clear the cell if that makes sense for it
      // Do not delete a cell, not even stubs, because this would be a drag-cancel
      // (Regular click would delete stubs)
      let ((cell_x, cell_y), _belt_type, _unused, _port_out_dir) = track[0]; // First element has no inbound port here
      let coord = to_coord(cell_x, cell_y);

      clear_part_from_cell(options, state, factory, coord);
    } else {
      // Other mouse button. ignore for now / ever.
      // I think this allows you to cancel a drag by pressing the rmb
    }
  } else if len == 2 {
    log(format!("two cell path with button {}", mouse_state.last_down_button));
    let ((cell_x1, cell_y1), belt_type1, _unused, _port_out_dir1) = track[0]; // First element has no inbound port here
    let coord1 = to_coord(cell_x1, cell_y1);
    let ((cell_x2, cell_y2), belt_type2, _port_in_dir2, _unused) = track[1]; // LAst element has no outbound port here
    let coord2 = to_coord(cell_x2, cell_y2);

    let dx = (cell_x2 as i8) - (cell_x1 as i8);
    let dy = (cell_y2 as i8) - (cell_y1 as i8);
    assert!((dx == 0) != (dy == 0), "one and only one of dx or dy is zero");
    assert!(dx >= -1 && dx <= 1 && dy >= -1 && dy <= 1, "since they are adjacent they must be -1, 0, or 1");

    if mouse_state.last_down_button == 1 {
      // Convert empty cells to belt cells.
      // Create a port between these two cells, but none of the other cells.

      if factory.floor[coord1].kind == CellKind::Empty {
        if is_edge_not_corner(cell_x1, cell_y1) {
          // Cell is empty so place a trash supplier here as a placeholder
          factory.floor[coord1] = supply_cell(cell_x1, cell_y1, part_c('t'), 0, 0, 0);
        }
        else if is_middle(cell_x1, cell_y1) {
          factory.floor[coord1] = belt_cell(cell_x1, cell_y1, belt_type_to_belt_meta(belt_type1));
        }
      }
      if factory.floor[coord2].kind == CellKind::Empty {
        if is_edge_not_corner(cell_x2, cell_y2) {
          // Cell is empty so place a trash demander here as a placeholder
          factory.floor[coord2] = demand_cell(cell_x2, cell_y2, part_c('t'));
        }
        else if is_middle(cell_x2, cell_y2) {
          factory.floor[coord2] = belt_cell(cell_x2, cell_y2, belt_type_to_belt_meta(belt_type2));
        }
      }

      cell_connect_if_possible(options, state, factory, coord1, coord2, dx, dy);
    } else if mouse_state.last_down_button == 2 {
      // Delete the port between the two cells but leave everything else alone.
      // The coords must be adjacent to one side.

      let ( dir1, dir2) = match ( dx, dy ) {
        ( 0 , -1 ) => {
          // x1 was bigger so xy1 is under xy2
          (Direction::Up, Direction::Down)
        }
        ( 1 , 0 ) => {
          // x2 was bigger so xy1 is left of xy2
          (Direction::Right, Direction::Left)
        }
        ( 0 , 1 ) => {
          // y2 was bigger so xy1 is above xy2
          (Direction::Down, Direction::Up)
        }
        ( -1 , 0 ) => {
          // x1 was bigger so xy1 is right of xy2
          (Direction::Left, Direction::Right)
        }
        _ => panic!("already asserted the range of x and y"),
      };

      port_disconnect_cells(factory, coord1, dir1, coord2, dir2);
    } else {
      // Other mouse button or multi-button. ignore for now / ever.
      // (Remember: this was a drag of two cells)
    }

    fix_belt_meta(factory, coord1);
    fix_belt_meta(factory, coord2);

    if mouse_state.last_down_button == 2 {
      if factory.floor[coord1].kind == CellKind::Belt && factory.floor[coord1].port_u == Port::None && factory.floor[coord1].port_r == Port::None && factory.floor[coord1].port_d == Port::None && factory.floor[coord1].port_l == Port::None {
        floor_delete_cell_at_partial(options, state, factory, coord1);
      } else {
        clear_part_from_cell(options, state, factory, coord1);
      }
      if factory.floor[coord2].kind == CellKind::Belt && factory.floor[coord2].port_u == Port::None && factory.floor[coord2].port_r == Port::None && factory.floor[coord2].port_d == Port::None && factory.floor[coord2].port_l == Port::None {
        floor_delete_cell_at_partial(options, state, factory, coord2);
      } else {
        clear_part_from_cell(options, state, factory, coord2);
      }
    }
  } else {
    // len > 2
    // Draw track if lmb, remove cells on track if rmb

    let mut px = 0;
    let mut py = 0;
    let mut pcoord = 0;
    for index in 0..len {
      let ((cell_x, cell_y), belt_type, _port_in_dir, _port_out_dir) = track[index];
      log(format!("- track {} at {} {} isa {:?}", index, cell_x, cell_y, belt_type));
      let coord = to_coord(cell_x, cell_y);

      if mouse_state.last_down_button == 1 {
        // Staple the track on top of the existing layout
        if factory.floor[coord].kind == CellKind::Empty {
          if is_edge_not_corner(cell_x, cell_y) {
            if index == 0 {
              factory.floor[coord] = supply_cell(cell_x, cell_y, part_c('t'), 0, 0, 0);
            }
            else {
              // Note: the drag can only start inside the floor, so we don't have to worry about
              //       the index here since we always drag in a straight line. Once the edge is
              //       reached, we assume the line to end and we can put a trash Demand down.
              factory.floor[coord] = demand_cell(cell_x, cell_y, part_c('t'));
            }
          }
          else if is_middle(cell_x, cell_y) {
            factory.floor[coord] = belt_cell(cell_x, cell_y, belt_type_to_belt_meta(belt_type));

            // Connect the end points to any existing neighboring cells if not already connected
            if index == 0 || index == len - 1 {
              // log(format!("    -- okay @{} got {:?} ;; {:?} {:?} {:?} {:?}", coord, belt_type, factory.floor[coord].port_u, factory.floor[coord].port_r, factory.floor[coord].port_d, factory.floor[coord].port_l));
              // log(format!("  - connect_belt_to_existing_neighbor_belts(), before: {:?} {:?} {:?} {:?}", factory.floor[coord].port_u, factory.floor[coord].port_r, factory.floor[coord].port_d, factory.floor[coord].port_l));
              connect_belt_to_existing_neighbor_cells(factory, coord);
            }
          }
        }

        if index > 0 {
          // (First element has no inbound)
          cell_connect_if_possible(options, state, factory, pcoord, coord, (cell_x as i8) - (px as i8), (cell_y as i8) - (py as i8));
        }

      } else if mouse_state.last_down_button == 2 {
        // Delete the cell if it is a belt, and in that case any port to it
        match factory.floor[coord].kind {
          CellKind::Belt => {
            // Delete this belt tile and update the neighbors accordingly
            floor_delete_cell_at_partial(options, state, factory, coord);
          }
          _ => (), // Do not delete machines, suppliers, or demanders. No need to delete empty cells
        }

      } else {
        // Ignore whatever this is.
      }

      px = cell_x;
      py = cell_y;
      pcoord = coord;

      if index > 0 && is_edge(cell_x, cell_y) {
        // The line can cover multiple edge cells but we only care about the first one.
        break;
      }
    }
  }

  factory.changed = true;
}
fn on_click_inside_floor(options: &mut Options, state: &mut State, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState) {
  log(format!("on_click_inside_floor()"));
  let last_mouse_up_cell_x = ((mouse_state.last_up_world_x - WORLD_OFFSET_X) / CELL_W).floor();
  let last_mouse_up_cell_y = ((mouse_state.last_up_world_y - WORLD_OFFSET_Y) / CELL_H).floor();

  if mouse_state.last_down_button == 2 {
    // Clear the cell if that makes sense for it. Delete a belt with one or zero ports.
    let coord = to_coord(last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize);

    let mut ports = 0;
    if factory.floor[coord].port_u != Port::None { ports += 1; }
    if factory.floor[coord].port_r != Port::None { ports += 1; }
    if factory.floor[coord].port_d != Port::None { ports += 1; }
    if factory.floor[coord].port_l != Port::None { ports += 1; }
    if ports <= 1 || factory.floor[coord].kind == CellKind::Machine {
      log(format!("Deleting stub @{} after rmb click", coord));
      floor_delete_cell_at_partial(options, state, factory, coord);
      factory.changed = true;
    }

    // If this wasn't a belt (ports=999) or the belt had more than 1 ports, then just drop its part.
    if ports > 1 {
      log(format!("Clearing part from @{} after rmb click (ports={})", coord, ports));
      clear_part_from_cell(options, state, factory, coord);
    }
  } else {
    // De-/Select this cell
    log(format!("clicked {} {} cell selection before: {:?}", last_mouse_up_cell_x, last_mouse_up_cell_y, cell_selection));

    if cell_selection.on && cell_selection.x == last_mouse_up_cell_x && cell_selection.y == last_mouse_up_cell_y {
      cell_selection.on = false;
    } else {
      cell_selection.on = true;
      cell_selection.x = last_mouse_up_cell_x;
      cell_selection.y = last_mouse_up_cell_y;
      cell_selection.coord = to_coord(last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize);
    }
  }
}

fn bounds_check(x: f64, y: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
  return x >= x1 && x < x2 && y >= y1 && y < y2;
}
fn hit_test_offers(factory: &Factory, mx: f64, my: f64) -> (bool, usize ) {
  if bounds_check(mx, my, UI_OFFERS_OX, UI_OFFERS_OY, UI_OFFERS_OX + UI_OFFERS_W, UI_OFFERS_OY + UI_OFFERS_H_PLUS_MARGIN * (factory.offers.len() as f64)) {
    let inside_offer_and_margin_index = ((my - UI_OFFERS_OY) / UI_OFFERS_H_PLUS_MARGIN).floor();
    let offer_heights_away_from_top_of_offer = inside_offer_and_margin_index - (UI_OFFERS_OY + inside_offer_and_margin_index * UI_OFFERS_H_PLUS_MARGIN) ;
    if offer_heights_away_from_top_of_offer < UI_OFFERS_H {
      return ( true, inside_offer_and_margin_index as usize );
    } else {
      return ( false, 0 );
    }
  } else {
    return ( false, 0 );
  };
}
fn hit_check_cell_editor_grid(wx: f64, wy: f64) -> bool {
  // log(format!("hit_check_cell_editor_grid({}, {}) {} {} {} {} = {}", wx, wy, UI_CELL_EDITOR_GRID_OX, UI_CELL_EDITOR_GRID_OY, UI_CELL_EDITOR_GRID_OX + UI_CELL_EDITOR_GRID_W, UI_CELL_EDITOR_GRID_OY + UI_CELL_EDITOR_GRID_H, wx >= UI_CELL_EDITOR_GRID_OX && wx < UI_CELL_EDITOR_GRID_OX + UI_CELL_EDITOR_GRID_W && wy >= UI_CELL_EDITOR_GRID_OY && wy < UI_CELL_EDITOR_GRID_OY + UI_CELL_EDITOR_GRID_H));
  return wx >= UI_CELL_EDITOR_GRID_OX && wx < UI_CELL_EDITOR_GRID_OX + UI_CELL_EDITOR_GRID_W && wy >= UI_CELL_EDITOR_GRID_OY && wy < UI_CELL_EDITOR_GRID_OY + UI_CELL_EDITOR_GRID_H;
}
fn ray_trace_dragged_line(factory: &Factory, x0: f64, y0: f64, x1: f64, y1: f64, for_preview: bool) -> Vec<((usize, usize), BeltType, Direction, Direction)> {
  // We raytracing
  // The dragged line becomes a ray that we trace through cells of the floor
  // We then generate a belt track such that it fits in with the existing belts, if any
  // - Figure out which cells the ray passes through
  // - If the ray crosses existing belts, generate the belt type as if the original was modified to support the new path (the pathing would not destroy existing ports)
  // - If the ray only spans one cell, force it to be invalid
  // - The first and last cells in the ray also auto-connect to any neighbor belts. Sections in the middle of the ray do not.

  let covered = get_cells_from_a_to_b(x0, y0, x1, y1);
  assert!(covered.len() >= 1, "Should always record at least one cell coord");

  if covered.len() == 1 {
    return vec!((covered[0], BeltType::INVALID, Direction::Up, Direction::Up));
  }

  // Note: in order of (dragging) appearance
  let mut track: Vec<((usize, usize), BeltType, Direction, Direction)> = vec!(); // ((x, y), new_bt)

  // Draw example tiles of the path you're drawing.
  // Take the existing cell and add one or two ports to it;
  // - first one only gets the "to" port added to it
  // - last one only gets the "from" port added to it
  // - middle parts get the "from" and "to" port added to them
  // let mut is_first = true;
  let (mut lx, mut ly) = covered[0];
  let mut last_from = Direction::Up; // first one ignores this value
  for index in 1..covered.len() {
    let (x, y) = covered[index];
    // Always set the previous one.
    let new_from = get_from_dir_between_xy(lx, ly, x, y);
    let last_to = direction_reverse(new_from);
    // For the first one, pass on the same "to" port since there is no "from" port (it'll be a noop)
    let bt =
      if !for_preview || x == 0 {
        // add_one_ports_to_cell(factory, to_coord(lx, ly), last_to)
        BeltType::INVALID
      } else {
        // This is necessary to make preview work but it may crash edge cells for actual placement
        // When placing the meta is updated to represent the final state after patching
        add_two_ports_to_cell(factory, to_coord(lx, ly), last_from, last_to)
      };
    track.push(((lx, ly), bt, last_from, last_to)); // Note: no inport for first element. consumer beware?

    lx = x;
    ly = y;
    last_from = new_from;
  }
  // Final step. Only has a from port.
  let bt = add_one_ports_to_cell(factory, to_coord(lx, ly), last_from);
  track.push(((lx, ly), bt, last_from, last_from)); // there's no out port for last element. consumer beware?

  return track;
}
fn get_cells_from_a_to_b(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<(usize, usize)>{
  // https://playtechs.blogspot.com/2007/03/raytracing-on-grid.html
  // Super cover int algo, ported from:
  //
  // void raytrace(int x0, int y0, int x1, int y1)
  // {
  //   int dx = abs(x1 - x0);
  //   int dy = abs(y1 - y0);
  //   int x = x0;
  //   int y = y0;
  //   int n = 1 + dx + dy;
  //   int x_inc = (x1 > x0) ? 1 : -1;
  //   int y_inc = (y1 > y0) ? 1 : -1;
  //   int error = dx - dy;
  //   dx *= 2;
  //   dy *= 2;
  //
  //   for (; n > 0; --n)
  //   {
  //     visit(x, y);
  //
  //     if (error > 0)
  //     {
  //       x += x_inc;
  //       error -= dy;
  //     }
  //     else
  //     {
  //       y += y_inc;
  //       error += dx;
  //     }
  //   }
  // }

  let dx = (x1 - x0).abs();
  let dy = (y1 - y0).abs();
  let mut x = x0;
  let mut y = y0;
  let n = 1.0 + dx + dy;
  let x_inc = if x1 > x0 { 1.0 } else { -1.0 };
  let y_inc = if y1 > y0 { 1.0 } else { -1.0 };
  let mut error = dx - dy;

  let mut covered = vec!();
  for n in 0..n as u64 {
    covered.push((x as usize, y as usize));
    if error > 0.0 {
      x += x_inc;
      error -= dy;
    } else {
      y += y_inc;
      error += dx;
    }
  }

  return covered;
}
fn hit_check_speed_bubbles_any(options: &mut Options, state: &mut State, mouse_state: &MouseState) -> bool {
  let ox = UI_SPEED_BUBBLE_OX + 10.0 - UI_SPEED_BUBBLE_RADIUS;
  let oy = UI_SPEED_BUBBLE_OY + 5.0 - UI_SPEED_BUBBLE_RADIUS;
  let bubble_box_width = 40.0;

  return bounds_check(mouse_state.world_x, mouse_state.world_y, ox, oy, ox + 5.0 * bubble_box_width, oy + 2.0 * UI_SPEED_BUBBLE_RADIUS);
}
fn on_click_speed_bubbles(options: &mut Options, state: &mut State, mouse_state: &MouseState) {
  log(format!("on_click_speed_bubbles()"));

  if hit_check_speed_bubble(options, state, mouse_state, 0, "-") {
    let m = options.speed_modifier;
    options.speed_modifier = options.speed_modifier.min(0.5) * 0.5;
    log(format!("pressed time minus, from {} to {}", m, options.speed_modifier));
  } else if hit_check_speed_bubble(options, state, mouse_state, 1, "½") {
    let m = options.speed_modifier;
    options.speed_modifier = 0.5;
    log(format!("pressed time half, from {} to {}", m, options.speed_modifier));
  } else if hit_check_speed_bubble(options, state, mouse_state, 2, "⏭") {
    let m = options.speed_modifier;
    if m == 1.0 {
      options.speed_modifier = 0.0;
      state.paused = true;
    } else {
      options.speed_modifier = 1.0;
      state.paused = false;
    }
    log(format!("pressed time one, from {} to {}", m, options.speed_modifier));
  } else if hit_check_speed_bubble(options, state, mouse_state, 3, "2") {
    let m = options.speed_modifier;
    options.speed_modifier = 2.0;
    log(format!("pressed time two, from {} to {}", m, options.speed_modifier));
  } else if hit_check_speed_bubble(options, state, mouse_state, 4, "+") {
    let m = options.speed_modifier;
    options.speed_modifier = options.speed_modifier.max(2.0) * 1.5;
    log(format!("pressed time plus, from {} to {}", m, options.speed_modifier));
  }
}
fn hit_check_speed_bubble(options: &Options, state: &State, mouse_state: &MouseState, index: usize, text: &str) -> bool {
  let bubble_box_width = 40.0;
  let cx = UI_SPEED_BUBBLE_OX + 10.0 + bubble_box_width * (index as f64);
  let cy = UI_SPEED_BUBBLE_OY + 5.0;
  let cr = UI_SPEED_BUBBLE_RADIUS;

  return bounds_check(mouse_state.world_x, mouse_state.world_y, cx - cr, cy - cr, cx + cr, cy + cr);
}

fn paint_green_debug(options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, fps: &VecDeque<f64>, now: f64, since_prev: f64, ticks_todo: u64, estimated_fps: f64, rounded_fps: u64, factory: &Factory, mouse_state: &MouseState) {

  let mut ui_lines = 0.0;

  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
  context.set_fill_style(&"grey".into());
  context.fill_text(format!("fps: {}", fps.len()).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
  context.set_fill_style(&"grey".into());
  context.fill_text(format!("App time  : {}", (now / 1000.0).floor()).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
  context.set_fill_style(&"grey".into());
  context.fill_text(format!("Since prev: {} (@{})", since_prev.floor(), estimated_fps).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
  context.set_fill_style(&"grey".into());
  context.fill_text(format!("Ticks todo: {} (r? {})", ticks_todo, rounded_fps).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
  context.set_fill_style(&"grey".into());
  context.fill_text(format!("Speed: {}", options.speed_modifier).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
  context.set_fill_style(&"grey".into());
  // context.fill_text(format!("$ / 10s    : {}", factory.stats.3).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
  context.set_fill_style(&"grey".into());
  context.fill_text(
    format!(
      "mouse abs  : {} x {} {} {}",
      mouse_state.world_x, mouse_state.world_y,
      if mouse_state.is_dragging { "drag" } else if mouse_state.is_down { "down" } else { "up" },
      mouse_state.last_down_button,
    ).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H
  ).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
  context.set_fill_style(&"grey".into());
  context.fill_text(format!("mouse world: {} x {}", mouse_state.cell_x, mouse_state.cell_y).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
  context.set_fill_style(&"grey".into());
  context.fill_text(format!("mouse cell : {:.2} x {:.2}", mouse_state.cell_rel_x, mouse_state.cell_rel_y).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

  assert_eq!(ui_lines, UI_DEBUG_LINES, "keep these in sync for simplicity");
}
fn paint_world_cli(context: &Rc<web_sys::CanvasRenderingContext2d>, options: &mut Options, state: &mut State, factory: &Factory) {
  // Clear world
  context.set_fill_style(&"white".into());
  context.fill_rect(50.0, 20.0, 350.0, 700.0);

  let lines = generate_floor_without_views(options, state, &factory);

  context.set_font(&"20px monospace");
  context.set_fill_style(&"black".into());
  for n in 0..lines.len() {
    context.fill_text(format!("{}", lines[n]).as_str(), 50.0, (n as f64) * 24.0 + 50.0).expect("something lower error fill_text");
  }
}
fn paint_background_tiles(options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, belt_tile_images: &Vec<web_sys::HtmlImageElement>, img_machine2: &web_sys::HtmlImageElement) {
  // Paint background cell tiles
  for coord in 0..FLOOR_CELLS_WH {
    let (cx, cy) = to_xy(coord);

    let ox = WORLD_OFFSET_X + CELL_W * (cx as f64);
    let oy = WORLD_OFFSET_Y + CELL_H * (cy as f64);

    // This is cheating since we defer the loading stuff to the browser. Sue me.
    match factory.floor[coord].kind {
      CellKind::Empty => (),
      CellKind::Belt => {
        let belt_meta = &factory.floor[coord].belt.meta;
        let img: &HtmlImageElement = &belt_tile_images[belt_meta.btype as usize];
        context.draw_image_with_html_image_element_and_dw_and_dh(&img, ox, oy, CELL_W, CELL_H).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
      },
      CellKind::Machine => {
        // For machines, paint the top-left cell only but make the painted area cover the whole machine
        // TODO: each machine size should have a unique, customized, sprite
        if factory.floor[coord].machine.main_coord == coord {
          context.draw_image_with_html_image_element_and_dw_and_dh(img_machine2, ox, oy, factory.floor[coord].machine.cell_width as f64 * CELL_W, factory.floor[coord].machine.cell_height as f64 * CELL_H).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
        }
      },
      CellKind::Supply => {
        // TODO: paint supply image
        context.set_fill_style(&COLOR_SUPPLY.into());
        context.fill_rect( ox, oy, CELL_W, CELL_H);
        if !options.print_priority_tile_order {
          context.set_fill_style(&"black".into());
          context.fill_text(format!("<{}>", factory.floor[coord].supply.gives.icon).as_str(), ox + 6.0, oy + 21.0).expect("something lower error fill_text");
        }
      }
      CellKind::Demand => {
        // TODO: paint demand image
        context.set_fill_style(&COLOR_DEMAND.into());
        context.fill_rect( ox, oy, CELL_W, CELL_H);
        if !options.print_priority_tile_order {
          context.set_fill_style(&"black".into());
          context.fill_text(format!(">{}<", factory.floor[coord].demand.part.icon).as_str(), ox + 8.0, oy + 21.0).expect("something lower error fill_text");
        }
      }
    }
  }

  if options.print_priority_tile_order {
    for i in 0..factory.prio.len() {
      let coord = factory.prio[i];
      let (cx, cy) = to_xy(coord);
      if factory.floor[coord].kind == CellKind::Belt { context.set_stroke_style(&"white".into()); }
      else { context.set_stroke_style(&"blue".into()); }
      let ox = WORLD_OFFSET_X + CELL_W * (cx as f64) + (CELL_W / 2.0 - 7.0);
      let oy = WORLD_OFFSET_Y + CELL_H * (cy as f64) + CELL_H / 2.0 + 3.0;
      context.stroke_text(format!("{}", i).as_str(), ox, oy);
    }
  }
}
fn paint_ports(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory) {
  context.set_stroke_style(&"gray".into());

  // Adjust for font size such that it gets centered. API falls a little short in this regard.
  let font_centering_delta_x: f64 = -5.0;
  let font_centering_delta_y: f64 = 4.0;

  for coord in 0..FLOOR_CELLS_WH {
    let (x, y) = to_xy(coord);
    if factory.floor[coord].kind != CellKind::Empty {
      // For each cell only paint the right and bottom port
      // Otherwise we're just gonna paint each port twice

      if factory.floor[coord].port_r == Port::Inbound {
        context.stroke_text("🡄", WORLD_OFFSET_X + (x as f64) * CELL_W + CELL_W + font_centering_delta_x, WORLD_OFFSET_Y + (y as f64) * CELL_H + CELL_H / 2.0 + font_centering_delta_y).expect("to paint");
      } else if factory.floor[coord].port_r == Port::Outbound {
        context.stroke_text("🡆", WORLD_OFFSET_X + (x as f64) * CELL_W + CELL_W + font_centering_delta_x, WORLD_OFFSET_Y + (y as f64) * CELL_H + CELL_H / 2.0 + font_centering_delta_y).expect("to paint");
      }

      if factory.floor[coord].port_d == Port::Inbound {
        context.stroke_text("🡅", WORLD_OFFSET_X + (x as f64) * CELL_W + CELL_W / 2.0 + font_centering_delta_x, WORLD_OFFSET_Y + (y as f64) * CELL_H + CELL_H + font_centering_delta_y).expect("to paint");
      } else if factory.floor[coord].port_d == Port::Outbound {
        context.stroke_text("🡇", WORLD_OFFSET_X + (x as f64) * CELL_W + CELL_W / 2.0 + font_centering_delta_x, WORLD_OFFSET_Y + (y as f64) * CELL_H + CELL_H + font_centering_delta_y).expect("to paint");
      }
    }
  }
}
fn paint_belt_items(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, part_tile_sprite: &web_sys::HtmlImageElement) {
  // Paint elements on the belt over the background tiles now
  for coord in 0..FLOOR_CELLS_WH {
    let (cx, cy) = to_xy(coord);
    // This is cheating since we defer the loading stuff to the browser. Sue me.
    let cell = &factory.floor[coord];
    match cell.kind {
      CellKind::Empty => (),
      CellKind::Belt => {
        let progress_c = ((cell.belt.part_progress as f64) / (cell.belt.speed as f64)).min(1.0);
        let first_half = progress_c < 0.5;

        // Start with the coordinate to paint the icon such that it ends up centered 
        // in the target cell.
        // Then increase or decrease one axis depending on the progress the part made.
        let sx = WORLD_OFFSET_X + CELL_W * (cx as f64) + -(PART_W * 0.5);
        let sy = WORLD_OFFSET_Y + CELL_H * (cy as f64) + -(PART_H * 0.5);

        let (px, py) =
          match if first_half { cell.belt.part_from } else { cell.belt.part_to } {
            Direction::Up => {
              let cux = sx + (CELL_W * 0.5);
              let cuy = sy + (CELL_H * (if first_half { progress_c } else { 1.0 - progress_c }));
              (cux, cuy)
            }
            Direction::Right => {
              let dlx = sx + (CELL_W * (if first_half { 1.0 - progress_c } else { progress_c }));
              let dly = sy + (CELL_H * 0.5);
              (dlx, dly)
            }
            Direction::Down => {
              let cux = sx + (CELL_W * 0.5);
              let cuy = sy + (CELL_H * (if first_half { 1.0 - progress_c } else { progress_c }));
              (cux, cuy)
            }
            Direction::Left => {
              let dlx = sx + (CELL_W * (if first_half { progress_c } else { 1.0 - progress_c }));
              let dly = sy + (CELL_H * 0.5);
              (dlx, dly)
            }
          };

        if paint_segment_part(&context, part_tile_sprite, cell.belt.part.clone(), 16.0, 16.0, px, py, PART_W, PART_H) {
          // context.set_font(&"8px monospace");
          // context.set_fill_style(&"green".into());
          // context.fill_text(format!("{} {}x{}", coord, x, y).as_str(), px + 3.0, py + 10.0).expect("something error fill_text");
          // context.fill_text(format!("{}", progress_c).as_str(), px + 3.0, py + 21.0).expect("something error fill_text");
        }
      },
      CellKind::Machine => {
        // TODO: paint machine somehow
        // 
        // match cell.machine {
        //   Machine::None => panic!("Machine cells should not be Machine::None"),
        //   | Machine::Composer
        //   | Machine::Smasher => {
        //     // Paint the inputs (1, 2, or 3) and output
        //     paint_segment_part(&context, &part_tile_sprite, cell.machine_input_1_want.clone(), 16.0, 16.0, WORLD_OFFSET_X + CELL_W * (cx as f64) + (0.0 * SEGMENT_W) + 4.0, WORLD_OFFSET_Y + CELL_H * (cy as f64) + (0.0 * SEGMENT_H) + 4.0, PART_W, PART_H);
        //     paint_segment_part(&context, &part_tile_sprite, cell.machine_input_2_want.clone(), 16.0, 16.0, WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.0 * SEGMENT_W) + 4.0, WORLD_OFFSET_Y + CELL_H * (cy as f64) + (0.0 * SEGMENT_H) + 4.0, PART_W, PART_H);
        //     paint_segment_part(&context, &part_tile_sprite, cell.machine_input_3_want.clone(), 16.0, 16.0, WORLD_OFFSET_X + CELL_W * (cx as f64) + (2.0 * SEGMENT_W) + 4.0, WORLD_OFFSET_Y + CELL_H * (cy as f64) + (0.0 * SEGMENT_H) + 4.0, PART_W, PART_H);
        //     paint_segment_part(&context, &part_tile_sprite, cell.machine_input_1_have.clone(), 16.0, 16.0, WORLD_OFFSET_X + CELL_W * (cx as f64) + (0.0 * SEGMENT_W) + 4.0, WORLD_OFFSET_Y + CELL_H * (cy as f64) + (1.0 * SEGMENT_H) + 4.0, PART_W, PART_H);
        //     paint_segment_part(&context, &part_tile_sprite, cell.machine_input_2_have.clone(), 16.0, 16.0, WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.0 * SEGMENT_W) + 4.0, WORLD_OFFSET_Y + CELL_H * (cy as f64) + (1.0 * SEGMENT_H) + 4.0, PART_W, PART_H);
        //     paint_segment_part(&context, &part_tile_sprite, cell.machine_input_3_have.clone(), 16.0, 16.0, WORLD_OFFSET_X + CELL_W * (cx as f64) + (2.0 * SEGMENT_W) + 4.0, WORLD_OFFSET_Y + CELL_H * (cy as f64) + (1.0 * SEGMENT_H) + 4.0, PART_W, PART_H);
        //     paint_segment_part(&context, &part_tile_sprite, cell.machine_output_want.clone(), 16.0, 16.0, WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.0 * SEGMENT_W) + 4.0, WORLD_OFFSET_Y + CELL_H * (cy as f64) + (2.0 * SEGMENT_H) + 4.0, PART_W, PART_H);
        //   },
        // }
      },
      CellKind::Supply => {
        // TODO: paint outbound supply part
      }
      CellKind::Demand => {
        // TODO: paint demand parts (none?)
      }
    }
  }
}
fn paint_mouse_cursor(context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  context.set_fill_style(&"#ff00ff7f".into()); // Semi transparent circles
  context.begin_path();
  context.ellipse(mouse_state.world_x, mouse_state.world_y, PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
  context.fill();
}
fn paint_mouse_stuff_in_floor(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState, belt_tile_images: &Vec<web_sys::HtmlImageElement>) {
  if mouse_state.cell_x != cell_selection.x || mouse_state.cell_y != cell_selection.y {
    context.set_stroke_style(&"red".into());
    context.stroke_rect(WORLD_OFFSET_X + mouse_state.cell_x * CELL_W, WORLD_OFFSET_Y + mouse_state.cell_y * CELL_H, CELL_W, CELL_H);
  }

  if mouse_state.is_dragging {
    if mouse_state.last_down_world_x - WORLD_OFFSET_X >= 0.0 && mouse_state.last_down_world_x - WORLD_OFFSET_X < WORLD_WIDTH && mouse_state.last_down_world_y - WORLD_OFFSET_Y >= 0.0 && mouse_state.last_down_world_y - WORLD_OFFSET_Y < WORLD_HEIGHT {
      paint_belt_drag_preview(context, factory, cell_selection, mouse_state, belt_tile_images);
    }
  }
}
fn paint_belt_drag_preview(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState, belt_tile_images: &Vec<web_sys::HtmlImageElement>) {
  let track = ray_trace_dragged_line(
    factory,
    ((mouse_state.last_down_world_x - WORLD_OFFSET_X) / CELL_W).floor(),
    ((mouse_state.last_down_world_y - WORLD_OFFSET_Y) / CELL_H).floor(),
    mouse_state.cell_x.floor(),
    mouse_state.cell_y.floor(),
    true, // if we dont then the preview will show only broken belt cells
  );

  for index in 0..track.len() {
    let ((cell_x, cell_y), bt, in_port_dir, out_port_dir) = track[index];
    context.set_fill_style(&"#00770044".into());
    context.fill_rect(WORLD_OFFSET_X + cell_x as f64 * CELL_W, WORLD_OFFSET_Y + cell_y as f64 * CELL_H, CELL_W, CELL_H);
    paint_ghost_belt_of_type(cell_x, cell_y, if mouse_state.last_down_button == 2 { BeltType::INVALID } else { bt }, &context, &belt_tile_images);
  }
}
fn paint_ghost_belt_of_type(cell_x: usize, cell_y: usize, belt_type: BeltType, context: &Rc<web_sys::CanvasRenderingContext2d>, belt_tile_images: &Vec<web_sys::HtmlImageElement>) {
  let img: &HtmlImageElement = &belt_tile_images[belt_type as usize];

  context.set_global_alpha(0.5);
  context.draw_image_with_html_image_element_and_dw_and_dh(&img, WORLD_OFFSET_X + cell_x as f64 * CELL_W + 5.0, WORLD_OFFSET_Y + cell_y as f64 * CELL_H + 5.0, CELL_W - 10.0, CELL_H - 10.0).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
  context.set_global_alpha(1.0);
}
fn paint_cell_editor(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState) {
  if !cell_selection.on {
    // // Clear cell editor
    // context.set_fill_style(&"white".into());
    // context.fill_rect(UI_CELL_EDITOR_OX, UI_CELL_EDITOR_OY, UI_CELL_EDITOR_W, UI_CELL_EDITOR_H);
    return;
  }

  // Mark the currently selected cell
  context.set_stroke_style(&"blue".into());
  context.stroke_rect(WORLD_OFFSET_X + cell_selection.x * CELL_W, WORLD_OFFSET_Y + cell_selection.y * CELL_H, CELL_W, CELL_H);

  // Paint cell editor
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_CELL_EDITOR_OX, UI_CELL_EDITOR_OY, UI_CELL_EDITOR_W, UI_CELL_EDITOR_H);
  context.set_fill_style(&"black".into());
  context.fill_text(format!("Cell {} x {} ({})", cell_selection.x, cell_selection.y, cell_selection.coord).as_str(), UI_OX + UI_ML + 10.0, UI_OY + ((UI_DEBUG_LINES + 2.0) * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

  let ox = UI_CELL_EDITOR_GRID_OX;
  let oy = UI_CELL_EDITOR_GRID_OY;
  let ow = UI_CELL_EDITOR_GRID_W;
  let oh = UI_CELL_EDITOR_GRID_H;

  // Paint cell segment grid
  context.begin_path();

  context.move_to(ox, oy);
  context.line_to(ox + ow, oy);
  context.move_to(ox,      oy + UI_SEGMENT_H);
  context.line_to(ox + ow, oy + UI_SEGMENT_H);
  context.move_to(ox,      oy + UI_SEGMENT_H + UI_SEGMENT_H);
  context.line_to(ox + ow, oy + UI_SEGMENT_H + UI_SEGMENT_H);
  context.move_to(ox,      oy + UI_SEGMENT_H + UI_SEGMENT_H + UI_SEGMENT_H);
  context.line_to(ox + ow, oy + UI_SEGMENT_H + UI_SEGMENT_H + UI_SEGMENT_H);

  context.move_to(ox, oy);
  context.line_to(ox,                                              oy + oh);
  context.move_to(ox + UI_SEGMENT_W, oy);
  context.line_to(ox + UI_SEGMENT_W,                               oy + oh);
  context.move_to(ox + UI_SEGMENT_W + UI_SEGMENT_W, oy);
  context.line_to(ox + UI_SEGMENT_W + UI_SEGMENT_W,                oy + oh);
  context.move_to(ox + UI_SEGMENT_W + UI_SEGMENT_W + UI_SEGMENT_W, oy);
  context.line_to(ox + UI_SEGMENT_W + UI_SEGMENT_W + UI_SEGMENT_W, oy + oh);

  context.set_stroke_style(&"black".into());
  context.stroke();

  context.set_fill_style(&"black".into());
  let type_name = match factory.floor[cell_selection.coord].kind {
    CellKind::Empty => "Empty",
    CellKind::Belt => "Belt",
    CellKind::Machine => "Machine",
    CellKind::Supply => "Supply",
    CellKind::Demand => "Demand",
  };
  context.fill_text(type_name, UI_OX + UI_ML + 10.0, UI_OY + ((UI_DEBUG_LINES + 2.0) * UI_LINE_H) + 2.0 * UI_FONT_H).expect("something error fill_text");

  // Paint ports
  context.set_fill_style(&"black".into());
  match factory.floor[cell_selection.coord].port_u {
    Port::Inbound => {
      context.fill_text("in", ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (0.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
    Port::Outbound => {
      context.fill_text("out", ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (0.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
    Port::None => {},
    Port::Unknown => {
      context.fill_text("???", ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (0.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
  }

  match factory.floor[cell_selection.coord].port_r {
    Port::Inbound => {
      context.fill_text("in", ox + (2.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
    Port::Outbound => {
      context.fill_text("out", ox + (2.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
    Port::None => {},
    Port::Unknown => {
      context.fill_text("???", ox + (2.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
  }

  match factory.floor[cell_selection.coord].port_d {
    Port::Inbound => {
      context.fill_text("in", ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (2.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
    Port::Outbound => {
      context.fill_text("out", ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (2.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
    Port::None => {},
    Port::Unknown => {
      context.fill_text("???", ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (2.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
  }

  match factory.floor[cell_selection.coord].port_l {
    Port::Inbound => {
      context.fill_text("in", ox + (0.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
    Port::Outbound => {
      context.fill_text("out", ox + (0.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
    Port::None => {},
    Port::Unknown => {
      context.fill_text("???", ox + (0.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
  }

  // // Paint in/out rotation index
  // let mut in_coords = factory.floor[cell_selection.coord].ins.iter().map(|(_dir, coord, _, _)| coord).collect::<Vec<&usize>>();
  context.fill_text(format!("ins:  {}", ins_outs_to_str(&factory.floor[cell_selection.coord].ins)).as_str(), UI_CELL_EDITOR_PART_OX + 4.0, UI_CELL_EDITOR_OY + 3.0 * UI_FONT_H).expect("to text");
  context.fill_text(format!("outs: {}", ins_outs_to_str(&factory.floor[cell_selection.coord].outs)).as_str(), UI_CELL_EDITOR_PART_OX + 4.0, UI_CELL_EDITOR_OY + 4.0 * UI_FONT_H).expect("to text");

  if factory.floor[cell_selection.coord].kind == CellKind::Belt && factory.floor[cell_selection.coord].belt.part.kind != PartKind::None{
    // Paint current part details
    let progress = ((factory.floor[cell_selection.coord].belt.part_progress as f64) / (factory.floor[cell_selection.coord].belt.speed as f64) * 100.0).round();
    let to =
      if factory.floor[cell_selection.coord].belt.part_to_tbd {
        "TBD"
      } else {
        match factory.floor[cell_selection.coord].belt.part_to {
          Direction::Up => "up",
          Direction::Right => "right",
          Direction::Down => "down",
          Direction::Left => "left",
        }
      };
    context.fill_text("part", UI_CELL_EDITOR_PART_OX + 4.0, UI_CELL_EDITOR_PART_OY + 3.0 * UI_FONT_H).expect("to paint port");
    context.fill_text(format!("{} %", progress).as_str(), UI_CELL_EDITOR_PART_OX + 4.0, UI_CELL_EDITOR_PART_OY + 4.0 * UI_FONT_H).expect("to paint port");
    context.fill_text(format!("to: {}", to).as_str(), UI_CELL_EDITOR_PART_OX + 4.0, UI_CELL_EDITOR_PART_OY + 5.0 * UI_FONT_H).expect("to paint port");
  }
}
fn paint_machine_editor(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState) {
  if !cell_selection.on {
    // // No cell selected. Clear machine editor
    // context.set_fill_style(&"white".into());
    // context.fill_rect(UI_MACHINE_EDITOR_OX, UI_MACHINE_EDITOR_OY, UI_MACHINE_EDITOR_W, UI_MACHINE_EDITOR_H);
    return;
  }

  let coord = cell_selection.coord;
  if factory.floor[coord].kind != CellKind::Machine {
    // // Not selected a machine. Clear machine area.
    // context.set_fill_style(&"white".into());
    // context.fill_rect(UI_MACHINE_EDITOR_OX, UI_MACHINE_EDITOR_OY, UI_MACHINE_EDITOR_W, UI_MACHINE_EDITOR_H);
    return;
  }

  // Each cell consolidates much of its information into the main coord, usually the top-left cell
  let main_coord = factory.floor[coord].machine.main_coord;
  let ( main_x, main_y ) = to_xy(main_coord);

  // Mark the currently selected machine main_coord
  context.set_stroke_style(&"cyan".into());
  context.stroke_rect(WORLD_OFFSET_X + main_x as f64 * CELL_W, WORLD_OFFSET_Y + main_y as f64 * CELL_H, CELL_W * factory.floor[main_coord].machine.cell_width as f64, CELL_H * factory.floor[main_coord].machine.cell_height as f64);

  // Paint cell editor
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_MACHINE_EDITOR_OX, UI_MACHINE_EDITOR_OY, UI_MACHINE_EDITOR_W, UI_MACHINE_EDITOR_H);
  context.set_fill_style(&"black".into());
  context.fill_text(format!("Machine main {} x {} ({})", main_x, main_y, main_coord).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H).expect("something error fill_text");
  let mut in_coords = factory.floor[main_coord].ins.iter().map(|(_dir, coord, _, _)| coord).collect::<Vec<&usize>>();
  in_coords.sort();
  in_coords.dedup();
  context.fill_text(format!("Ins : {:?}", in_coords).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 2.0).expect("something error fill_text");
  let mut out_coords = factory.floor[main_coord].outs.iter().map(|(_dir, coord, _, _)| coord).collect::<Vec<&usize>>();
  out_coords.sort();
  out_coords.dedup();
  context.fill_text(format!("Outs: {:?}", out_coords).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 3.0).expect("something error fill_text");
  context.fill_text(format!("Wants: {} {} {}   Gens: {}",
    factory.floor[main_coord].machine.input_1_want.icon,
    factory.floor[main_coord].machine.input_2_want.icon,
    factory.floor[main_coord].machine.input_3_want.icon,
    factory.floor[main_coord].machine.output_want.icon,
  ).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 4.0).expect("something error fill_text");
  context.fill_text(format!("Haves: {} {} {}", factory.floor[main_coord].machine.input_1_have.icon, factory.floor[main_coord].machine.input_2_have.icon, factory.floor[main_coord].machine.input_3_have.icon).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 5.0).expect("something error fill_text");
  context.fill_text(format!("Speed: {}", factory.floor[main_coord].machine.speed).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 6.0).expect("something error fill_text");
  context.fill_text(format!("Progress: {: >3}% ({})", (((factory.ticks - factory.floor[main_coord].machine.start_at) as f64 / factory.floor[main_coord].machine.speed as f64).min(1.0) * 100.0) as u8, factory.floor[main_coord].machine.start_at).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 7.0).expect("something error fill_text");
  context.fill_text(format!("Produced: {: >4}", factory.floor[main_coord].machine.produced).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 8.0).expect("something error fill_text");
  context.fill_text(format!("Trashed: {: >4}", factory.floor[main_coord].machine.trashed).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 9.0).expect("something error fill_text");
}
fn paint_supply_editor(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState) {
  if !cell_selection.on {
    // // No cell selected. Clear machine editor
    // context.set_fill_style(&"white".into());
    // context.fill_rect(UI_MACHINE_EDITOR_OX, UI_MACHINE_EDITOR_OY, UI_MACHINE_EDITOR_W, UI_MACHINE_EDITOR_H);
    return;
  }

  let coord = cell_selection.coord;
  if factory.floor[coord].kind != CellKind::Supply {
    // // Not selected a machine. Clear machine area.
    // context.set_fill_style(&"white".into());
    // context.fill_rect(UI_MACHINE_EDITOR_OX, UI_MACHINE_EDITOR_OY, UI_MACHINE_EDITOR_W, UI_MACHINE_EDITOR_H);
    return;
  }

  // Each cell consolidates much of its information into the main coord, usually the top-left cell
  let x = cell_selection.x;
  let y = cell_selection.y;

  // Mark the currently selected machine main_coord
  context.set_stroke_style(&"cyan".into());
  context.stroke_rect(WORLD_OFFSET_X + x as f64 * CELL_W, WORLD_OFFSET_Y + y as f64 * CELL_H, CELL_W, CELL_H);

  // Paint cell editor
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_MACHINE_EDITOR_OX, UI_MACHINE_EDITOR_OY, UI_MACHINE_EDITOR_W, UI_MACHINE_EDITOR_H);
  context.set_fill_style(&"black".into());
  context.fill_text(format!("Supply {} x {} ({})", x, y, coord).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H).expect("something error fill_text");
  let mut in_coords = factory.floor[coord].ins.iter().map(|(_dir, coord, _, _)| coord).collect::<Vec<&usize>>();
  in_coords.sort();
  in_coords.dedup();
  context.fill_text(format!("Ins : {:?}", in_coords).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 2.0).expect("something error fill_text");
  let mut out_coords = factory.floor[coord].outs.iter().map(|(_dir, coord, _, _)| coord).collect::<Vec<&usize>>();
  out_coords.sort();
  out_coords.dedup();
  context.fill_text(format!("Outs: {:?}", out_coords).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 3.0).expect("something error fill_text");
  context.fill_text(format!("Gives: {}", factory.floor[coord].supply.gives.icon).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 4.0).expect("something error fill_text");
  context.fill_text(format!("Speed: {}", factory.floor[coord].supply.speed).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 5.0).expect("something error fill_text");
  context.fill_text(format!("Cooldown: {: >3}% {}", (((factory.ticks - factory.floor[coord].supply.last_part_out_at) as f64 / factory.floor[coord].supply.cooldown.max(1) as f64).min(1.0) * 100.0) as u8, factory.floor[coord].supply.cooldown).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 6.0).expect("something error fill_text");
  context.fill_text(format!("Progress: {: >3}% ({})", (((factory.ticks - factory.floor[coord].supply.part_at) as f64 / factory.floor[coord].supply.speed.max(1) as f64).min(1.0) * 100.0) as u8, factory.floor[coord].supply.part_at).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 7.0).expect("something error fill_text");
  context.fill_text(format!("Supplied: {: >4}", factory.floor[coord].supply.supplied).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 8.0).expect("something error fill_text");
}
fn paint_demand_editor(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState) {
  if !cell_selection.on {
    // // No cell selected. Clear machine editor
    // context.set_fill_style(&"white".into());
    // context.fill_rect(UI_MACHINE_EDITOR_OX, UI_MACHINE_EDITOR_OY, UI_MACHINE_EDITOR_W, UI_MACHINE_EDITOR_H);
    return;
  }

  let coord = cell_selection.coord;
  if factory.floor[coord].kind != CellKind::Demand {
    // // Not selected a machine. Clear machine area.
    // context.set_fill_style(&"white".into());
    // context.fill_rect(UI_MACHINE_EDITOR_OX, UI_MACHINE_EDITOR_OY, UI_MACHINE_EDITOR_W, UI_MACHINE_EDITOR_H);
    return;
  }

  // Each cell consolidates much of its information into the main coord, usually the top-left cell
  let x = cell_selection.x;
  let y = cell_selection.y;

  // Mark the currently selected machine main_coord
  context.set_stroke_style(&"cyan".into());
  context.stroke_rect(WORLD_OFFSET_X + x as f64 * CELL_W, WORLD_OFFSET_Y + y as f64 * CELL_H, CELL_W, CELL_H);

  // Paint cell editor
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_MACHINE_EDITOR_OX, UI_MACHINE_EDITOR_OY, UI_MACHINE_EDITOR_W, UI_MACHINE_EDITOR_H);
  context.set_fill_style(&"black".into());
  context.fill_text(format!("Demand {} x {} ({})", x, y, coord).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H).expect("something error fill_text");
  let mut in_coords = factory.floor[coord].ins.iter().map(|(_dir, coord, _, _)| coord).collect::<Vec<&usize>>();
  in_coords.sort();
  in_coords.dedup();
  context.fill_text(format!("Ins : {:?}", in_coords).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 2.0).expect("something error fill_text");
  let mut out_coords = factory.floor[coord].outs.iter().map(|(_dir, coord, _, _)| coord).collect::<Vec<&usize>>();
  out_coords.sort();
  out_coords.dedup();
  context.fill_text(format!("Outs: {:?}", out_coords).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 3.0).expect("something error fill_text");
  context.fill_text(format!("Wants: {}", factory.floor[coord].demand.part.icon).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 4.0).expect("something error fill_text");
  context.fill_text(format!("Received: {: >4}", factory.floor[coord].demand.received).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 5.0).expect("something error fill_text");
  context.fill_text(format!("Trashed: {: >4}", factory.floor[coord].demand.trashed).as_str(), UI_MACHINE_EDITOR_OX + UI_ML, UI_MACHINE_EDITOR_OY + UI_FONT_H * 6.0).expect("something error fill_text");
}
fn paint_top_stats(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory) {
  context.set_fill_style(&"black".into());
  context.fill_text(format!("Ticks: {}, Supplied: {}, Produced: {}, Received: {}, Trashed: {}", factory.ticks, factory.supplied, factory.produced, factory.accepted, factory.trashed).as_str(), 20.0, 20.0).expect("to paint");
  if factory.first_out_at == 0 {
    context.fill_text(format!("Waiting for first part to reach a Demand...").as_str(), 20.0, 40.0).expect("to paint");
  } else {
    context.fill_text(format!("Current time: {}, goal: {}, best: {}", factory.ticks - factory.first_out_at, 10000, 9999).as_str(), 20.0, 40.0).expect("to paint");
  }
}
fn paint_ui_offers(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, mouse_state: &MouseState) {
  let (is_down_on_offer, down_inside_offer_index) =
    if mouse_state.is_dragging { ( false, 0 ) } // Drag start is handled elsewhere, while dragging do not highlight offers
    else { ( true, mouse_state.offer_index ) };

  for index in 0..factory.offers.len() {
    paint_ui_offer_supply(&context, factory, index, is_down_on_offer && index == down_inside_offer_index);
  }
}
fn paint_ui_buttons(options: &mut Options, state: &mut State, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  paint_ui_button(context, mouse_state, 0.0, "Empty");
  paint_ui_button(context, mouse_state, 1.0, "Unbelt");
  paint_ui_button(context, mouse_state, 2.0, "Unpart");
  paint_ui_button(context, mouse_state, 3.0, "Undir");
  paint_ui_button(context, mouse_state, 4.0, "Dump");
  paint_ui_button(context, mouse_state, 5.0, "Reset");
  paint_ui_button(context, mouse_state, 6.0, "Panic");
  assert!(UI_BUTTON_COUNT == 7.0, "Update after adding new buttons");

  paint_ui_time_control(options, state, context, mouse_state);
}
fn paint_ui_button(context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, index: f64, text: &str) {
  let x = UI_BUTTONS_OX + index * (UI_BUTTON_W + UI_BUTTON_SPACING);
  let y = UI_BUTTONS_OY;

  if bounds_check(mouse_state.world_x, mouse_state.world_y, x, y, x + UI_BUTTON_W, y + UI_BUTTON_H) {
    context.set_fill_style(&"#eee".into());
  } else {
    context.set_fill_style(&"#aaa".into());
  }
  context.fill_rect(x, y, UI_BUTTON_W, UI_BUTTON_H);
  context.set_stroke_style(&"black".into());
  context.stroke_rect(x, y, UI_BUTTON_W, UI_BUTTON_H);
  context.set_fill_style(&"black".into());
  context.fill_text(text, x + 5.0, y + 14.0).expect("to paint");
}
fn paint_ui_buttons2(options: &mut Options, state: &mut State, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  paint_ui_button2(context, mouse_state, 0.0, if state.mouse_mode_erasing { "erase" } else { "draw" }, state.mouse_mode_erasing);
  paint_ui_button2(context, mouse_state, 1.0, "select", state.mouse_mode_selecting);
  paint_ui_button2(context, mouse_state, 2.0, "copy", false);
  paint_ui_button2(context, mouse_state, 3.0, "paste", false);
  // paint_ui_button2(context, mouse_state, 4.0, "nodir", false);
  // paint_ui_button2(context, mouse_state, 5.0, "togoal"); // fast forward to goal
  // paint_ui_button2(context, mouse_state, 6.0, "Panic");
  assert!(UI_BUTTON_COUNT == 7.0, "Update after adding new buttons");
}
fn paint_ui_button2(context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, index: f64, text: &str, on: bool) {
  let x = UI_BUTTONS_OX + index * (UI_BUTTON_W + UI_BUTTON_SPACING);
  let y = UI_BUTTONS_OY2;

  if on {
    context.set_fill_style(&"lightgreen".into());
  } else if bounds_check(mouse_state.world_x, mouse_state.world_y, x, y, x + UI_BUTTON_W, y + UI_BUTTON_H) {
    context.set_fill_style(&"#eee".into());
  } else {
    context.set_fill_style(&"#aaa".into());
  }
  context.fill_rect(x, y, UI_BUTTON_W, UI_BUTTON_H);
  context.set_stroke_style(&"black".into());
  context.stroke_rect(x, y, UI_BUTTON_W, UI_BUTTON_H);
  context.set_fill_style(&"black".into());
  context.fill_text(text, x + 5.0, y + 14.0).expect("to paint");
}
fn paint_ui_time_control(options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  paint_ui_speed_bubble(options, state, context, mouse_state, 0, "-");
  paint_ui_speed_bubble(options, state, context, mouse_state, 1, "½");
  paint_ui_speed_bubble(options, state, context, mouse_state, 2, "⏭");
  paint_ui_speed_bubble(options, state, context, mouse_state, 3, "2");
  paint_ui_speed_bubble(options, state, context, mouse_state, 4, "+");
}
fn paint_ui_speed_bubble(options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, index: usize, text: &str) {
  let cx = UI_SPEED_BUBBLE_OX + 10.0 + 40.0 * (index as f64);
  let cy = UI_SPEED_BUBBLE_OY + 5.0;
  let cr = UI_SPEED_BUBBLE_RADIUS;

  if text == "⏭" && options.speed_modifier == 0.0 {
    context.set_fill_style(&"tomato".into());
  }
  else if text == "⏭" && options.speed_modifier == 1.0 {
    context.set_fill_style(&"#0f0".into());
  }
  else if text == "½" && options.speed_modifier == 0.5 {
    context.set_fill_style(&"#0f0".into());
  }
  else if text == "2" && options.speed_modifier == 2.0 {
    context.set_fill_style(&"#0f0".into());
  }
  else if text == "-" && (options.speed_modifier > 0.0 && options.speed_modifier < 0.5) {
    context.set_fill_style(&"#0f0".into());
  }
  else if text == "+" && options.speed_modifier > 2.0 {
    context.set_fill_style(&"#0f0".into());
  }
  else if bounds_check(mouse_state.world_x, mouse_state.world_y, cx - cr, cy - cr, cx + cr, cy + cr) {
    context.set_fill_style(&"#eee".into());
  }
  else {
    context.set_fill_style(&"#aaa".into());
  }
  context.begin_path();
  context.arc(cx, cy, cr, 0.0, 2.0 * 3.14).expect("to paint");
  context.fill();
  context.set_fill_style(&"stroke".into());
  context.stroke();
  context.set_fill_style(&"black".into());
  context.fill_text(text, cx - 4.0, cy + 4.0).expect("to paint");
}
fn paint_ui_offer_supply(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, index: usize, hovering: bool) {
  let offer = &factory.offers[index];

  let offer_height = UI_OFFERS_H + 10.0;

  // Clear area.

  match offer.kind {
    CellKind::Supply => {
      context.set_fill_style(&COLOR_SUPPLY.into());
    }
    CellKind::Demand => {
      context.set_fill_style(&COLOR_DEMAND.into());
    }
    CellKind::Machine => {
      context.set_fill_style(&COLOR_MACHINE.into());
    }
    _ => panic!("this kind should not get here"),
  }

  context.fill_rect(UI_OFFERS_OX, UI_OFFERS_OY + (index as f64) * offer_height, UI_OFFERS_W, UI_OFFERS_H);
  if hovering {
    context.set_stroke_style(&"red".into());
    context.stroke_rect(UI_OFFERS_OX, UI_OFFERS_OY + (index as f64) * offer_height, UI_OFFERS_W, UI_OFFERS_H);
  }

  context.set_stroke_style(&"black".into());
  context.set_fill_style(&"black".into());
  context.stroke_text(format!("{:?}", offer.kind).as_str(), UI_OFFERS_OX + UI_ML, UI_OFFERS_OY + (index as f64) * offer_height + 1.0 * UI_FONT_H).expect("something error stroke_text");
  match offer.kind {
    CellKind::Supply => {
      context.fill_text(format!("Gives: {}", offer.supply_icon).as_str(), UI_OFFERS_OX + UI_ML, UI_OFFERS_OY + (index as f64) * offer_height + 2.0 * UI_FONT_H).expect("something error fill_text");
      context.fill_text(format!("Speed: {}", offer.speed).as_str(), UI_OFFERS_OX + UI_ML, UI_OFFERS_OY + (index as f64) * offer_height + 3.0 * UI_FONT_H).expect("something error fill_text");
      context.fill_text(format!("Cool: {}", offer.cooldown).as_str(), UI_OFFERS_OX + UI_ML, UI_OFFERS_OY + (index as f64) * offer_height + 4.0 * UI_FONT_H).expect("something error fill_text");
    }
    CellKind::Demand => {
      context.fill_text(format!("Takes: {}", offer.demand_icon).as_str(), UI_OFFERS_OX + UI_ML, UI_OFFERS_OY + (index as f64) * offer_height + 2.0 * UI_FONT_H).expect("something error fill_text");
    }
    CellKind::Machine => {
      context.fill_text(format!("Size: {} x {}", offer.cell_width, offer.cell_height).as_str(), UI_OFFERS_OX + UI_ML, UI_OFFERS_OY + (index as f64) * offer_height + 2.0 * UI_FONT_H).expect("something error fill_text");
      context.fill_text(format!("Inputs: {} {} {}", offer.machine_input1, offer.machine_input2, offer.machine_input3).as_str(), UI_OFFERS_OX + UI_ML, UI_OFFERS_OY + (index as f64) * offer_height + 3.0 * UI_FONT_H).expect("something error fill_text");
      context.fill_text(format!("Output: {}", offer.machine_output).as_str(), UI_OFFERS_OX + UI_ML, UI_OFFERS_OY + (index as f64) * offer_height + 4.0 * UI_FONT_H).expect("something error fill_text");
      context.fill_text(format!("Speed: {}", offer.speed).as_str(), UI_OFFERS_OX + UI_ML, UI_OFFERS_OY + (index as f64) * offer_height + 5.0 * UI_FONT_H).expect("something error fill_text");
    }
    _ => panic!("this kind should not get here"),
  }
}
fn paint_segment_part(context: &Rc<web_sys::CanvasRenderingContext2d>, part_tile_sprite: &HtmlImageElement, segment_part: Part, spw: f64, sph: f64, dx: f64, dy: f64, dw: f64, dh: f64) -> bool {
  let (spx, spy) = match segment_part.kind {
    PartKind::WoodenStick => {
      // This is a club? Piece of wood I guess? From which wands are formed.
      (0.0, 11.0)
      // (10.0, 11.0) // Test image
      // context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(context, part_tile_sprite, dx, dy, dw, dh, 0.0, 11.0, segx, segy, sw, sh).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
    },
    PartKind::Sapphire => {
      // This is a sapphire
      (1.0, 3.0)
      // context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(context, part_tile_sprite, dx, dy, dw, dh, 3.0, 1.0, segx, segy, sw, sh).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
    },
    PartKind::BlueWand => {
      // This is the slightly bigger blue wand
      (2.0, 11.0)
      // context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(context, part_tile_sprite, dx, dy, dw, dh, 2.0, 11.0, segx, segy, sw, sh).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
    },
    PartKind::GoldenBlueWand => {
      // This is the golden blue wand
      (4.0, 11.0)
      // context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(context, part_tile_sprite, dx, dy, dw, dh, 4.0, 11.0, segx, segy, sw, sh).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
    },
    PartKind::Trash => {
      // This is something that looks like a grey rock
      (11.0, 10.0)
      // context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(context, part_tile_sprite, dx, dy, dw, dh, 4.0, 11.0, segx, segy, sw, sh).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
    },
    PartKind::None => {
      // Ignore, this belt segment or machine input is empty
      return false;
    },
  };

  context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
    &part_tile_sprite,
    // Sprite position
    spx * spw, spy * sph, spw, sph,
    // Paint onto canvas at
    dx, dy, dw, dh,
  ).expect("something error draw_image"); // requires web_sys HtmlImageElement feature

  return true;
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
  window()
    .request_animation_frame(f.as_ref().unchecked_ref())
    .expect("should register `requestAnimationFrame` OK");
}

fn window() -> web_sys::Window {
  web_sys::window().expect("no global `window` exists")
}
fn document() -> web_sys::Document {
  window()
    .document()
    .expect("should have a document on window")
}
fn body() -> web_sys::HtmlElement {
  document().body().expect("document should have a body")
}

fn ins_outs_to_str(list: &Vec<(Direction, usize, usize, Direction)>) -> String {
  let map = list.iter().map(|(d,..)| match d { Direction::Up => 'u', Direction::Right => 'r', Direction::Down => 'd', Direction::Left => 'l'});
  return map.collect::<String>();
}
