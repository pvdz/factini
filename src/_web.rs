// This file should only be included for `wasm-pack build --target web`
// The main.rs will include this file when `#[cfg(target_arch = "wasm32")]`

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

use super::belt::*;
use super::cell::*;
use super::factory::*;
use super::floor::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::segment::*;
use super::state::*;

// These are the actual pixels we can paint to
const CANVAS_WIDTH: f64 = 1000.0;
const CANVAS_HEIGHT: f64 = 1000.0;
// Need this for mouse2world coord conversion
const CANVAS_CSS_WIDTH: f64 = 1000.0;
const CANVAS_CSS_HEIGHT: f64 = 1000.0;

// Size of the world in cell count
const WORLD_CELLS_X: f64 = 7.0;
const WORLD_CELLS_Y: f64 = 7.0;

// World size in world pixels (as painted on the canvas)
const WORLD_OFFSET_X: f64 = 0.0;
const WORLD_OFFSET_Y: f64 = 0.0;
const WORLD_WIDTH: f64 = WORLD_CELLS_X * CELL_W;
const WORLD_HEIGHT: f64 = WORLD_CELLS_Y * CELL_H;

// Size of a cell (world pixels)
const CELL_W: f64 = 100.0;
const CELL_H: f64 = 100.0;
// 3x3 segments in a cell
const SEGMENT_W: f64 = 33.0;
const SEGMENT_H: f64 = 33.0;
// Size of parts on a belt
const PART_W: f64 = 25.0;
const PART_H: f64 = 25.0;

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

const UI_CELL_EDITOR_OX: f64 = UI_OX + UI_ML;
const UI_CELL_EDITOR_OY: f64 = UI_OY + (UI_LINE_H * (UI_DEBUG_LINES + 2.0));
const UI_CELL_EDITOR_W: f64 = UI_W;
const UI_CELL_EDITOR_H: f64 = UI_LINE_H * 7.0;

const UI_CELL_EDITOR_GRID_OX: f64 = UI_OX + UI_ML + 10.0;
const UI_CELL_EDITOR_GRID_OY: f64 = UI_OY + ((UI_DEBUG_LINES + 3.0) * UI_LINE_H) + UI_FONT_H;
const UI_CELL_EDITOR_GRID_W: f64 = 3.0 * UI_SEGMENT_W;
const UI_CELL_EDITOR_GRID_H: f64 = 3.0 * UI_SEGMENT_H;

const UI_CELL_EDITOR_KIND_OX: f64 = UI_CELL_EDITOR_GRID_OX + (3.0 * UI_SEGMENT_W) + 10.0;
const UI_CELL_EDITOR_KIND_OY: f64 = UI_CELL_EDITOR_GRID_OY + (2.0 * UI_FONT_H);
const UI_CELL_EDITOR_KIND_W: f64 = 60.0;
const UI_CELL_EDITOR_KIND_H: f64 = 2.0 * UI_FONT_H;

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

pub fn log(s: &str) {
  // web_sys::console::log_2(&"Color : %s ".into(),&context.fill_style().into());
  web_sys::console::log_2(&"(rust)".into(), &s.into());
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

fn hit_check_between_belts(factory: &mut Factory, cx: f64, cy: f64, crx: f64, cry: f64) -> SegmentDirection {
  if crx >= 0.33 && crx < 0.66 {
    if cry < 0.20 {
      if cy > 0.0 {
        let coord = to_coord(WORLD_CELLS_X as usize, cx as usize, cy as usize);
        if factory.floor.cells[coord].segments[SegmentDirection::UP as usize].port != Port::None {
          if factory.floor.cells[to_coord_up(coord, WORLD_CELLS_X as usize)].segments[SegmentDirection::DOWN as usize].port != Port::None {
            return SegmentDirection::UP;
          }
        }
      }
    } else if cry > 0.80 {
      if cy < WORLD_CELLS_Y-1.0 {
        let coord = to_coord(WORLD_CELLS_X as usize, cx as usize, cy as usize);
        if factory.floor.cells[coord].segments[SegmentDirection::DOWN as usize].port != Port::None {
          if factory.floor.cells[to_coord_down(coord, WORLD_CELLS_X as usize)].segments[SegmentDirection::UP as usize].port != Port::None {
            return SegmentDirection::DOWN;
          }
        }
      }
    }
  } else if cry >= 0.33 && cry < 0.66 {
    if crx < 0.20 {
      if cx > 0.0 {
        let coord = to_coord(WORLD_CELLS_X as usize, cx as usize, cy as usize);
        if factory.floor.cells[coord].segments[SegmentDirection::LEFT as usize].port != Port::None {
          if factory.floor.cells[to_coord_left(coord, WORLD_CELLS_X as usize)].segments[SegmentDirection::RIGHT as usize].port != Port::None {
            return SegmentDirection::LEFT;
          }
        }
      }
    } else if crx > 0.80 {
      if cx < WORLD_CELLS_Y-1.0 {
        let coord = to_coord(WORLD_CELLS_X as usize, cx as usize, cy as usize);
        if factory.floor.cells[coord].segments[SegmentDirection::RIGHT as usize].port != Port::None {
          if factory.floor.cells[to_coord_right(coord, WORLD_CELLS_X as usize)].segments[SegmentDirection::LEFT as usize].port != Port::None {
            return SegmentDirection::RIGHT;
          }
        }
      }
    }
  }

  return SegmentDirection::CENTER; // No
}

fn hit_check_cell_editor_any(wx: f64, wy: f64) -> bool {
  // log(format!("hit_check_cell_editor_any({}, {}) {} {} {} {} = {}", wx, wy, UI_CELL_EDITOR_OX, UI_CELL_EDITOR_OY, UI_CELL_EDITOR_OX + UI_CELL_EDITOR_W, UI_CELL_EDITOR_OY + UI_CELL_EDITOR_H, wx >= UI_CELL_EDITOR_OX && wy >= UI_CELL_EDITOR_OY && wx < UI_CELL_EDITOR_OX + UI_CELL_EDITOR_W && wy < UI_CELL_EDITOR_OY + UI_CELL_EDITOR_H).as_str());
  return wx >= UI_CELL_EDITOR_OX && wy >= UI_CELL_EDITOR_OY && wx < UI_CELL_EDITOR_OX + UI_CELL_EDITOR_W && wy < UI_CELL_EDITOR_OY + UI_CELL_EDITOR_H;
}
fn hit_check_cell_editor_grid(wx: f64, wy: f64) -> bool {
  // log(format!("hit_check_cell_editor_grid({}, {}) {} {} {} {} = {}", wx, wy, UI_CELL_EDITOR_GRID_OX, UI_CELL_EDITOR_GRID_OY, UI_CELL_EDITOR_GRID_OX + UI_CELL_EDITOR_GRID_W, UI_CELL_EDITOR_GRID_OY + UI_CELL_EDITOR_GRID_H, wx >= UI_CELL_EDITOR_GRID_OX && wx < UI_CELL_EDITOR_GRID_OX + UI_CELL_EDITOR_GRID_W && wy >= UI_CELL_EDITOR_GRID_OY && wy < UI_CELL_EDITOR_GRID_OY + UI_CELL_EDITOR_GRID_H).as_str());
  return wx >= UI_CELL_EDITOR_GRID_OX && wx < UI_CELL_EDITOR_GRID_OX + UI_CELL_EDITOR_GRID_W && wy >= UI_CELL_EDITOR_GRID_OY && wy < UI_CELL_EDITOR_GRID_OY + UI_CELL_EDITOR_GRID_H;
}
fn hit_check_cell_editor_kind(wx: f64, wy: f64) -> bool {
  return wx >= UI_CELL_EDITOR_KIND_OX && wx < UI_CELL_EDITOR_KIND_OX + UI_CELL_EDITOR_KIND_W && wy >= UI_CELL_EDITOR_KIND_OY && wy < UI_CELL_EDITOR_KIND_OY + UI_CELL_EDITOR_KIND_H;
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  // Must run this once in web-mode to enable dumping panics to console.log
  panic::set_hook(Box::new(console_error_panic_hook::hook));

  log("web start...");
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document
    .create_element("canvas")?
    .dyn_into::<web_sys::HtmlCanvasElement>()?;
  document.body().unwrap().append_child(&canvas)?;
  canvas.set_width(CANVAS_WIDTH as u32);
  canvas.set_height(CANVAS_HEIGHT as u32);
  canvas.style().set_property("border", "solid")?;
  canvas.style().set_property("width", format!("{}px", CANVAS_CSS_WIDTH as u32).as_str())?;
  canvas.style().set_property("width", format!("{}px", CANVAS_CSS_HEIGHT as u32).as_str())?;
  let context = canvas
    .get_context("2d")?
    .unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
  let context = Rc::new(context);

  let todo = load_tile("./img/todo.png").expect("can'tpub const CELL_BELT_NONE.src");

  // Preload the belt tiles. Create an array with a to-do image for every slot. Then create img tags
  let mut belt_tile_images: Vec<web_sys::HtmlImageElement> = vec![todo; CELL_BELT_TYPE_COUNT]; // Prefill with todo images
  belt_tile_images[BeltType::NONE as usize] = load_tile(CELL_BELT_NONE.src)?;
  belt_tile_images[BeltType::U_R as usize] = load_tile(CELL_BELT_U_R.src)?;
  belt_tile_images[BeltType::R_U as usize] = load_tile(CELL_BELT_R_U.src)?;
  belt_tile_images[BeltType::R_D as usize] = load_tile(CELL_BELT_R_D.src)?;
  belt_tile_images[BeltType::D_R as usize] = load_tile(CELL_BELT_D_R.src)?;
  belt_tile_images[BeltType::D_L as usize] = load_tile(CELL_BELT_D_L.src)?;
  belt_tile_images[BeltType::L_D as usize] = load_tile(CELL_BELT_L_D.src)?;
  belt_tile_images[BeltType::L_U as usize] = load_tile(CELL_BELT_L_U.src)?;
  belt_tile_images[BeltType::U_L as usize] = load_tile(CELL_BELT_U_L.src)?;
  belt_tile_images[BeltType::U_D as usize] = load_tile(CELL_BELT_U_D.src)?;
  belt_tile_images[BeltType::D_U as usize] = load_tile(CELL_BELT_D_U.src)?;
  belt_tile_images[BeltType::L_R as usize] = load_tile(CELL_BELT_L_R.src)?;
  belt_tile_images[BeltType::R_L as usize] = load_tile(CELL_BELT_R_L.src)?;
  belt_tile_images[BeltType::U_LR as usize] = load_tile(CELL_BELT_U_LR.src)?;
  belt_tile_images[BeltType::RU_L as usize] = load_tile(CELL_BELT_RU_L.src)?;
  belt_tile_images[BeltType::LU_R as usize] = load_tile(CELL_BELT_LU_R.src)?;
  belt_tile_images[BeltType::L_RU as usize] = load_tile(CELL_BELT_L_RU.src)?;
  belt_tile_images[BeltType::LR_U as usize] = load_tile(CELL_BELT_LR_U.src)?;
  belt_tile_images[BeltType::R_LU as usize] = load_tile(CELL_BELT_R_LU.src)?;
  belt_tile_images[BeltType::R_DU as usize] = load_tile(CELL_BELT_R_DU.src)?;
  belt_tile_images[BeltType::RU_D as usize] = load_tile(CELL_BELT_RU_D.src)?;
  belt_tile_images[BeltType::DR_U as usize] = load_tile(CELL_BELT_DR_U.src)?;
  belt_tile_images[BeltType::DU_R as usize] = load_tile(CELL_BELT_DU_R.src)?;
  belt_tile_images[BeltType::U_DR as usize] = load_tile(CELL_BELT_U_DR.src)?;
  belt_tile_images[BeltType::D_RU as usize] = load_tile(CELL_BELT_D_RU.src)?;
  belt_tile_images[BeltType::D_LR as usize] = load_tile(CELL_BELT_D_LR.src)?;
  belt_tile_images[BeltType::DL_R as usize] = load_tile(CELL_BELT_DL_R.src)?;
  belt_tile_images[BeltType::DR_L as usize] = load_tile(CELL_BELT_DR_L.src)?;
  belt_tile_images[BeltType::LR_D as usize] = load_tile(CELL_BELT_LR_D.src)?;
  belt_tile_images[BeltType::L_DR as usize] = load_tile(CELL_BELT_L_DR.src)?;
  belt_tile_images[BeltType::R_DL as usize] = load_tile(CELL_BELT_R_DL.src)?;
  belt_tile_images[BeltType::L_DU as usize] = load_tile(CELL_BELT_L_DU.src)?;
  belt_tile_images[BeltType::LU_D as usize] = load_tile(CELL_BELT_LU_D.src)?;
  belt_tile_images[BeltType::DL_U as usize] = load_tile(CELL_BELT_DL_U.src)?;
  belt_tile_images[BeltType::DU_L as usize] = load_tile(CELL_BELT_DU_L.src)?;
  belt_tile_images[BeltType::U_DL as usize] = load_tile(CELL_BELT_U_DL.src)?;
  belt_tile_images[BeltType::D_UL as usize] = load_tile(CELL_BELT_D_LU.src)?;
  belt_tile_images[BeltType::U_DLR as usize] = load_tile(CELL_BELT_U_DLR.src)?;
  belt_tile_images[BeltType::R_DLU as usize] = load_tile(CELL_BELT_R_DLU.src)?;
  belt_tile_images[BeltType::D_LRU as usize] = load_tile(CELL_BELT_D_LRU.src)?;
  belt_tile_images[BeltType::L_DRU as usize] = load_tile(CELL_BELT_L_DRU.src)?;
  belt_tile_images[BeltType::RU_DL as usize] = load_tile(CELL_BELT_RU_DL.src)?;
  belt_tile_images[BeltType::DU_LR as usize] = load_tile(CELL_BELT_DU_LR.src)?;
  belt_tile_images[BeltType::LU_DR as usize] = load_tile(CELL_BELT_LU_DR.src)?;
  belt_tile_images[BeltType::LD_RU as usize] = load_tile(CELL_BELT_DL_RU.src)?;
  belt_tile_images[BeltType::DR_LU as usize] = load_tile(CELL_BELT_DR_LU.src)?;
  belt_tile_images[BeltType::LR_DU as usize] = load_tile(CELL_BELT_LR_DU.src)?;
  belt_tile_images[BeltType::DLR_U as usize] = load_tile(CELL_BELT_DLR_U.src)?;
  belt_tile_images[BeltType::DLU_R as usize] = load_tile(CELL_BELT_DLU_R.src)?;
  belt_tile_images[BeltType::RLU_D as usize] = load_tile(CELL_BELT_LRU_D.src)?;
  belt_tile_images[BeltType::DRU_L as usize] = load_tile(CELL_BELT_DRU_L.src)?;
  belt_tile_images[BeltType::INVALID as usize] = load_tile(CELL_BELT_INVALID.src)?;

  let part_tile_sprite: web_sys::HtmlImageElement = load_tile("./img/roguelikeitems.png")?;

  let img_machine1 = load_tile("./img/machine1.png")?;
  let img_machine2 = load_tile("./img/machine2.png")?;
  let img_machine3 = load_tile("./img/machine3.png")?;

  // Tbh this whole Rc approach is copied from the original template. It works so why not, :shrug:
  let mouse_x = Rc::new(Cell::new(0.0));
  let mouse_y = Rc::new(Cell::new(0.0));
  let is_mouse_down = Rc::new(Cell::new(false));
  let last_mouse_down_x = Rc::new(Cell::new(0.0));
  let last_mouse_down_y = Rc::new(Cell::new(0.0));
  let last_mouse_up_x = Rc::new(Cell::new(0.0));
  let last_mouse_up_y = Rc::new(Cell::new(0.0));

  // mousedown
  {
    let is_mouse_down = is_mouse_down.clone();
    let last_mouse_down_x = last_mouse_down_x.clone();
    let last_mouse_down_y = last_mouse_down_y.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
      let mx = event.offset_x() as f64;
      let my = event.offset_y() as f64;
      is_mouse_down.set(true);
      last_mouse_up_x.set(mx);
      last_mouse_up_y.set(my);
      // log("mousedown?");
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  // mousemove
  {
    let mouse_x = mouse_x.clone();
    let mouse_y = mouse_y.clone();
    // let context = context.clone();
    // let pressed = pressed.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
      let mx = event.offset_x() as f64;
      let my = event.offset_y() as f64;
      mouse_x.set(mx);
      mouse_y.set(my);
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
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }


  // Static state configuration (can still be changed by user)
  let mut options = create_options(5.0);

  // General app state
  let mut state = State {};

  let mut factory = create_factory(&mut options, &mut state);

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
    log(&format!("start time: {}", start_time));

    let context = context.clone();

    let mut prev_time = start_time;

    let mut fps: VecDeque<f64> = VecDeque::new();
    
    let mut is_cell_dragging: bool = false;
    let mut cell_drag_ocx: f64 = 0.0; // in cell coords
    let mut cell_drag_ocy: f64 = 0.0;
    
    let mut selected_cell = false;
    let mut selected_cell_x: f64 = 0.0;
    let mut selected_cell_y: f64 = 0.0;

    // From https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
      let now: f64 = perf.now();
      let since_prev: f64 = now - prev_time;
      prev_time = now;

      let min = now - 1000.0;
      while fps.len() > 0 && fps[0] < min {
        fps.pop_front();
      }
      fps.push_back(now);

      // ONE_SECOND is how many ticks I want to pass in one real world second
      // We could do an absolute "we should have this many ticks at this point" but that will
      // be problematic if there's ever a pause for whatever reason since there'll be some
      // catch-up frames and they may never catch up.
      // Unfortunately, the current way of calculating the time since previous frame is always
      // lagging one frame behind and has some rounding problems, especially with low % modifiers.
      let ticks_todo: u64 = (since_prev * ((ONE_SECOND as f64) * options.speed_modifier / 1000.0)) as u64;

      // Mouse coords
      let mx = mouse_x.get();
      let my = mouse_y.get();
      // Mouse position in world pixels
      // Note: mouse2world coord is determined by _css_ size, not _canvas_ size
      let mwx = mx / CANVAS_CSS_WIDTH * CANVAS_WIDTH;
      let mwy = my / CANVAS_CSS_HEIGHT * CANVAS_HEIGHT;
      let mcx = (mwx / CELL_W).floor();
      let mcy = (mwy / CELL_H).floor();
      // cell relative coord (if any)
      let mcrx = (mwx / CELL_W) - mcx;
      let mcry = (mwy / CELL_H) - mcy;

      context.set_font(&"12px monospace");

      let mut ui_lines = 0.0;

      context.set_fill_style(&"lightgreen".into());
      context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
      context.set_fill_style(&"grey".into());
      context.fill_text(format!("fps: {}", fps.len()).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

      ui_lines += 1.0;
      context.set_fill_style(&"lightgreen".into());
      context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
      context.set_fill_style(&"grey".into());
      context.fill_text(format!("App time  : {}", now / 1000.0).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

      ui_lines += 1.0;
      context.set_fill_style(&"lightgreen".into());
      context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
      context.set_fill_style(&"grey".into());
      context.fill_text(format!("Since prev: {}", since_prev).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

      ui_lines += 1.0;
      context.set_fill_style(&"lightgreen".into());
      context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
      context.set_fill_style(&"grey".into());
      context.fill_text(format!("Ticks todo: {}", ticks_todo).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

      ui_lines += 1.0;
      context.set_fill_style(&"lightgreen".into());
      context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
      context.set_fill_style(&"grey".into());
      context.fill_text(format!("$ /  1s    : {}", factory.stats.2).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

      ui_lines += 1.0;
      context.set_fill_style(&"lightgreen".into());
      context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
      context.set_fill_style(&"grey".into());
      context.fill_text(format!("$ / 10s    : {}", factory.stats.3).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

      ui_lines += 1.0;
      context.set_fill_style(&"lightgreen".into());
      context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
      context.set_fill_style(&"grey".into());
      context.fill_text(format!("mouse abs  : {} x {} {}", mouse_x.get(), mouse_y.get()).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H, if is_mouse_down.get() { "down" } else { "up" } ).expect("something error fill_text");

      ui_lines += 1.0;
      context.set_fill_style(&"lightgreen".into());
      context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
      context.set_fill_style(&"grey".into());
      context.fill_text(format!("mouse world: {} x {}", mcx, mcy).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

      ui_lines += 1.0;
      context.set_fill_style(&"lightgreen".into());
      context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * ui_lines), UI_W, UI_LINE_H);
      context.set_fill_style(&"grey".into());
      context.fill_text(format!("mouse cell : {:.2} x {:.2}", mcrx, mcry).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

      assert_eq!(ui_lines, UI_DEBUG_LINES, "keep these in sync for simplicity");

      for _ in 0..ticks_todo.min(MAX_TICKS_PER_FRAME) {
        tick_factory(&mut options, &mut state, &mut factory);
      }

      if options.web_output_cli {
        // Clear world
        context.set_fill_style(&"white".into());
        context.fill_rect(50.0, 20.0, 350.0, 700.0);

        let lines = serialize_cli_lines(&factory);

        context.set_font(&"20px monospace");
        context.set_fill_style(&"black".into());
        for n in 0..lines.len() {
          context.fill_text(format!("{}", lines[n]).as_str(), 50.0, (n as f64) * 24.0 + 50.0).expect("something lower error fill_text");
        }
      } else {
        // https://docs.rs/web-sys/0.3.28/web_sys/struct.CanvasRenderingContext2d.html

        // Handle click. Yes, you can't click at x=0 or y=0
        // "lcm*" = "last click mouse *"
        // TODO: do we want to do this before the tick? Not sure it matters.
        let lcmx = last_mouse_up_x.get();
        let lcmy = last_mouse_up_y.get();
        if lcmx > 0.0 && lcmy > 0.0 {
          // Clear the click state
          last_mouse_up_x.set(0.0);
          last_mouse_up_y.set(0.0);

          // last mouse click world x/y
          let lcmwx = lcmx / CANVAS_CSS_WIDTH * CANVAS_WIDTH;
          let lcmwy = lcmy / CANVAS_CSS_HEIGHT * CANVAS_HEIGHT;

          // Was the click inside the painted world?
          // In that case we change/toggle the cell selection
          if lcmwx >= WORLD_OFFSET_X && lcmwy >= WORLD_OFFSET_Y && lcmwx < WORLD_OFFSET_X + WORLD_WIDTH && lcmwy < WORLD_OFFSET_Y + WORLD_HEIGHT {
            // "lcm*" = "last click mouse *"
            let lcmcx = (lcmwx / CELL_W).floor();
            let lcmcy = (lcmwy / CELL_H).floor();
            let lmccoord = to_coord(factory.floor.width + 2, lcmcx as usize, lcmcy as usize);
            let lcmcrx = (lcmwx / CELL_W) - lcmcx;
            let lcmcry = (lcmwy / CELL_H) - lcmcy;

            // Check if the icon between connected belts was clicked
            if lcmcx >= 0.0 && lcmcy >= 0.0 && lcmcx < WORLD_CELLS_X && lcmcy < WORLD_CELLS_Y {
              let found = hit_check_between_belts(&mut factory, lcmcx, lcmcy, lcmcrx, lcmcry);
              log(format!("found {:?}", found).as_str());
              // If found (not CENTER) then drop the ports between these cells
              match found {
                SegmentDirection::UP => {
                  factory.floor.cells[lmccoord].segments[SegmentDirection::UP as usize].port = Port::None;
                  factory.floor.cells[to_coord_up(lmccoord, factory.floor.width + 2)].segments[SegmentDirection::DOWN as usize].port = Port::None;
                },
                SegmentDirection::RIGHT => {
                  factory.floor.cells[lmccoord].segments[SegmentDirection::RIGHT as usize].port = Port::None;
                  factory.floor.cells[to_coord_right(lmccoord, factory.floor.width + 2)].segments[SegmentDirection::LEFT as usize].port = Port::None;
                },
                SegmentDirection::DOWN => {
                  factory.floor.cells[lmccoord].segments[SegmentDirection::DOWN as usize].port = Port::None;
                  factory.floor.cells[to_coord_down(lmccoord, factory.floor.width + 2)].segments[SegmentDirection::UP as usize].port = Port::None;
                },
                SegmentDirection::LEFT => {
                  factory.floor.cells[lmccoord].segments[SegmentDirection::LEFT as usize].port = Port::None;
                  factory.floor.cells[to_coord_left(lmccoord, factory.floor.width + 2)].segments[SegmentDirection::RIGHT as usize].port = Port::None;
                },
                SegmentDirection::CENTER => {
                  // Center means the click was not between belts with connected ports...
                  log(format!("clicked {} {} selected {} on {} {}", lcmcx, lcmcy, selected_cell, selected_cell_x, selected_cell_y).as_str());

                  if selected_cell && lcmcx == selected_cell_x && lcmcy == selected_cell_y {
                    selected_cell = false;
                    selected_cell_x = 100.0;
                    selected_cell_y = 100.0;
                  } else {
                    selected_cell = true;
                    selected_cell_x = lcmcx;
                    selected_cell_y = lcmcy;
                  }
                }
              }
            }
          }
          // Is the click inside the cell editor?
          else if selected_cell && hit_check_cell_editor_any(lcmwx, lcmwy) {
            // Is the mouse clicking on one of the focused grid segments?
            if hit_check_cell_editor_grid(lcmwx, lcmwy) {
              // Clicked inside the grid
              // Determine which segment and then rotate that segment
              let sx = ((lcmwx - UI_CELL_EDITOR_GRID_OX) / UI_SEGMENT_W).floor();
              let sy = ((lcmwy - UI_CELL_EDITOR_GRID_OY) / UI_SEGMENT_H).floor();

              log(format!("sxy: {} {}", sx, sy).as_str());

              let seg = match (sx as i8, sy as i8) {
                (1, 0) => SegmentDirection::UP,
                (2, 1) => SegmentDirection::RIGHT,
                (1, 2) => SegmentDirection::DOWN,
                (0, 1) => SegmentDirection::LEFT,
                _ => SegmentDirection::CENTER, // ignore center and corners
              };

              if seg != SegmentDirection::CENTER {
                let selected_coord = to_coord(factory.floor.width + 2, selected_cell_x as usize, selected_cell_y as usize);
                // Cycle the port on this side
                let port = factory.floor.cells[selected_coord].segments[seg as usize].port;
                let new_port = match port {
                  Port::Inbound => Port::Outbound,
                  Port::Outbound => Port::None,
                  Port::None => Port::Inbound,
                };
                factory.floor.cells[selected_coord].segments[seg as usize].port = new_port;
                factory.floor.cells[selected_coord].belt = port_config_to_belt(
                  factory.floor.cells[selected_coord].segments[SegmentDirection::UP as usize].port,
                  factory.floor.cells[selected_coord].segments[SegmentDirection::RIGHT as usize].port,
                  factory.floor.cells[selected_coord].segments[SegmentDirection::DOWN as usize].port,
                  factory.floor.cells[selected_coord].segments[SegmentDirection::LEFT as usize].port,
                );
              }
            } else {
              // Is the mouse clicking on the focused grid kind box?
              if hit_check_cell_editor_kind(lcmwx, lcmwy) {
                let selected_coord = to_coord(factory.floor.width + 2, selected_cell_x as usize, selected_cell_y as usize);

                // There are two cases; edge and middle cells. Supply/Demand can only go on edge.
                // Machine and Belt can only go in middle. Empty can go anywhere.
                log(format!("from the {} {} {} {}", mcx, mcy, WORLD_CELLS_X - 1.0, WORLD_CELLS_Y - 1.0).as_str());
                if selected_cell_x == 0.0 || selected_cell_y == 0.0 || selected_cell_x == WORLD_CELLS_X - 1.0 || selected_cell_y == WORLD_CELLS_Y - 1.0 {
                  log("from the top");
                  // Edge. Cycle between Empty, Supply, and Demand
                  match factory.floor.cells[selected_coord].kind {
                    CellKind::Empty => {
                      factory.floor.cells[selected_coord] = supply_cell(factory.floor.cells[selected_coord].x, factory.floor.cells[selected_coord].y, CELL_BELT_NONE, part_none(), 10000, 10000, -800);
                    },
                    CellKind::Supply => {
                      factory.floor.cells[selected_coord] = demand_cell(factory.floor.cells[selected_coord].x, factory.floor.cells[selected_coord].y, CELL_BELT_NONE, part_none(), 10000, 10000, -800);
                    },
                    CellKind::Demand => {
                      factory.floor.cells[selected_coord] = empty_cell(factory.floor.cells[selected_coord].x, factory.floor.cells[selected_coord].y);
                    },
                    | CellKind::Belt
                    | CellKind::Machine
                    => panic!("edge should not contain machine or belt"),
                  }
                } else {
                  log("from the middle");
                  // Middle. Cycle between Empty, Machine, and Belt
                  match factory.floor.cells[selected_coord].kind {
                    CellKind::Empty => {
                      factory.floor.cells[selected_coord] = belt_cell(factory.floor.cells[selected_coord].x, factory.floor.cells[selected_coord].y, CELL_BELT_NONE);
                    },
                    CellKind::Belt => {
                      factory.floor.cells[selected_coord] = machine_cell(factory.floor.cells[selected_coord].x, factory.floor.cells[selected_coord].y, Machine::Composer, part_none(), part_none(), part_none(), part_none(), -15, -3);
                    },
                    CellKind::Machine => {
                      factory.floor.cells[selected_coord] = empty_cell(factory.floor.cells[selected_coord].x, factory.floor.cells[selected_coord].y);
                    }
                    | CellKind::Supply
                    | CellKind::Demand
                    => panic!("middle should not contain supply or demand"),
                  }
                };

                // Recreate cell traversal order
                let prio: Vec<usize> = create_prio_list(&mut options, &mut factory.floor);
                log(format!("Updated prio list: {:?}", prio).as_str());
                factory.prio = prio;
              }
            }
          }
        }

        // Paint the world

        // Clear world
        context.set_fill_style(&"orange".into());
        context.fill_rect(WORLD_OFFSET_X, WORLD_OFFSET_Y, WORLD_CELLS_X * CELL_W + 20.0, WORLD_CELLS_Y * CELL_H + 20.0);

        let fw = factory.floor.width + 2;

        // Paint background cell tiles
        for coord in 0..factory.fsum {
          let (cx, cy) = to_xy(coord, fw);
          // This is cheating since we defer the loading stuff to the browser. Sue me.
          match factory.floor.cells[coord].kind {
            CellKind::Empty => (),
            CellKind::Belt => {
              let belt = &factory.floor.cells[coord].belt;
              let img: &HtmlImageElement = &belt_tile_images[belt.btype as usize];
              context.draw_image_with_html_image_element_and_dw_and_dh( &img, WORLD_OFFSET_X + CELL_W * (cx as f64), WORLD_OFFSET_Y + CELL_H * (cy as f64), CELL_W, CELL_H).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
            },
            CellKind::Machine => {
              match factory.floor.cells[coord].machine {
                Machine::None => (),
                Machine::Composer => {
                  context.draw_image_with_html_image_element_and_dw_and_dh( &img_machine2, WORLD_OFFSET_X + CELL_W * (cx as f64), WORLD_OFFSET_Y + CELL_H * (cy as f64), CELL_W, CELL_H).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
                  context.set_fill_style(&"#ff00007f".into()); // Semi transparent circles
                  if factory.floor.cells[coord].machine_input_1_want.kind != PartKind::None {
                    context.begin_path();
                    context.ellipse(WORLD_OFFSET_X + CELL_W * (cx as f64) + (0.5 * SEGMENT_W), WORLD_OFFSET_X + CELL_W * (cx as f64) + (0.5 * SEGMENT_W), PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(WORLD_OFFSET_X + CELL_W * (cx as f64) + (0.5 * SEGMENT_W), WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.5 * SEGMENT_W), PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                  }
                  if factory.floor.cells[coord].machine_input_2_want.kind != PartKind::None {
                    context.begin_path();
                    context.ellipse(WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.5 * SEGMENT_W), WORLD_OFFSET_X + CELL_W * (cx as f64) + (0.5 * SEGMENT_W), PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.5 * SEGMENT_W), WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.5 * SEGMENT_W), PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                  }
                  if factory.floor.cells[coord].machine_input_3_want.kind != PartKind::None {
                    context.begin_path();
                    context.ellipse(WORLD_OFFSET_X + CELL_W * (cx as f64) + (2.5 * SEGMENT_W), WORLD_OFFSET_X + CELL_W * (cx as f64) + (0.5 * SEGMENT_W), PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(WORLD_OFFSET_X + CELL_W * (cx as f64) + (2.5 * SEGMENT_W), WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.5 * SEGMENT_W), PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                  }
                  context.begin_path();
                  context.ellipse(WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.5 * SEGMENT_W), WORLD_OFFSET_X + CELL_W * (cx as f64) + (2.5 * SEGMENT_W), PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                  context.fill();
                },
                Machine::Smasher => {
                  context.draw_image_with_html_image_element_and_dw_and_dh( &img_machine1, WORLD_OFFSET_X + CELL_W * (cx as f64), WORLD_OFFSET_Y + CELL_H * (cy as f64), CELL_W, CELL_H).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
                  context.set_fill_style(&"#ff00007f".into()); // Semi transparent circles
                  if factory.floor.cells[coord].machine_input_1_want.kind != PartKind::None {
                    context.begin_path();
                    context.ellipse(WORLD_OFFSET_X + CELL_W * (cx as f64) + (0.5 * SEGMENT_W), WORLD_OFFSET_X + CELL_W * (cx as f64) + (0.5 * SEGMENT_W), PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(WORLD_OFFSET_X + CELL_W * (cx as f64) + (0.5 * SEGMENT_W), WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.5 * SEGMENT_W), PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                  }
                  if factory.floor.cells[coord].machine_input_2_want.kind != PartKind::None {
                    context.begin_path();
                    context.ellipse(WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.5 * SEGMENT_W), WORLD_OFFSET_X + CELL_W * (cx as f64) + (0.5 * SEGMENT_W), PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.5 * SEGMENT_W), WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.5 * SEGMENT_W), PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                  }
                  if factory.floor.cells[coord].machine_input_3_want.kind != PartKind::None {
                    context.begin_path();
                    context.ellipse(WORLD_OFFSET_X + CELL_W * (cx as f64) + (2.5 * SEGMENT_W), WORLD_OFFSET_X + CELL_W * (cx as f64) + (0.5 * SEGMENT_W), PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(WORLD_OFFSET_X + CELL_W * (cx as f64) + (2.5 * SEGMENT_W), WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.5 * SEGMENT_W), PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                  }
                  context.begin_path();
                  context.ellipse(WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.5 * SEGMENT_W), WORLD_OFFSET_X + CELL_W * (cx as f64) + (2.5 * SEGMENT_W), PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                  context.fill();
                },
              }
            },
            CellKind::Supply => {
              // TODO: paint supply image
            }
            CellKind::Demand => {
              // TODO: paint demand image
            }
          }
        }

        // Paint cell segment grids
        if options.paint_cell_segment_grid {
          for coord in 0..factory.fsum {
            let (x, y) = to_xy(coord, fw);
            if factory.floor.cells[coord].kind != CellKind::Empty {
              context.begin_path();

              context.move_to(WORLD_OFFSET_X + (x as f64) * CELL_W,      WORLD_OFFSET_Y + (y as f64) * CELL_H);
              context.line_to(WORLD_OFFSET_X + (x as f64) * CELL_W + CELL_W, WORLD_OFFSET_Y + (y as f64) * CELL_H);
              context.move_to(WORLD_OFFSET_X + (x as f64) * CELL_W,      WORLD_OFFSET_Y + (y as f64) * CELL_H + SEGMENT_H);
              context.line_to(WORLD_OFFSET_X + (x as f64) * CELL_W + CELL_W, WORLD_OFFSET_Y + (y as f64) * CELL_H + SEGMENT_H);
              context.move_to(WORLD_OFFSET_X + (x as f64) * CELL_W,      WORLD_OFFSET_Y + (y as f64) * CELL_H + SEGMENT_H + SEGMENT_H);
              context.line_to(WORLD_OFFSET_X + (x as f64) * CELL_W + CELL_W, WORLD_OFFSET_Y + (y as f64) * CELL_H + SEGMENT_H + SEGMENT_H);

              context.move_to(WORLD_OFFSET_X + (x as f64) * CELL_W,                         WORLD_OFFSET_Y + (y as f64) * CELL_H);
              context.line_to(WORLD_OFFSET_X + (x as f64) * CELL_W,                         WORLD_OFFSET_Y + (y as f64) * CELL_H + CELL_H);
              context.move_to(WORLD_OFFSET_X + (x as f64) * CELL_W + SEGMENT_W,             WORLD_OFFSET_Y + (y as f64) * CELL_H);
              context.line_to(WORLD_OFFSET_X + (x as f64) * CELL_W + SEGMENT_W,             WORLD_OFFSET_Y + (y as f64) * CELL_H + CELL_H);
              context.move_to(WORLD_OFFSET_X + (x as f64) * CELL_W + SEGMENT_W + SEGMENT_W, WORLD_OFFSET_Y + (y as f64) * CELL_H);
              context.line_to(WORLD_OFFSET_X + (x as f64) * CELL_W + SEGMENT_W + SEGMENT_W, WORLD_OFFSET_Y + (y as f64) * CELL_H + CELL_H);

              context.set_stroke_style(&"black".into());
              context.stroke();
            }
          }
        }

        fn pro(ticks: u64, at: u64, speed: u64) -> f64 {
          let tnow = ((ticks - at) as f64).max(0.001).min(speed as f64);
          let progress = tnow / (speed as f64);
          return progress;
        }
        fn pro_lu(ticks: u64, at: u64, speed: u64, distance: f64, dir: Port) -> f64 {
          let tnow = ((ticks - at) as f64).max(0.001).min(speed as f64);
          let progress = (tnow / (speed as f64)) * distance;
          return if dir == Port::Inbound { progress } else { distance - progress };
        }
        fn pro_dr(ticks: u64, at: u64, speed: u64, distance: f64, dir: Port) -> f64 {
          let tnow = ((ticks - at) as f64).max(0.001).min(speed as f64);
          let progress = (tnow / (speed as f64)) * distance;
          return if dir == Port::Inbound { distance - progress } else { progress };
        }

        // Paint elements on the belt over the background tiles now
        for coord in 0..factory.fsum {
          let (cx, cy) = to_xy(coord, fw);
          // This is cheating since we defer the loading stuff to the browser. Sue me.
          let cell = &factory.floor.cells[coord];
          match cell.kind {
            CellKind::Empty => (),
            CellKind::Belt => {
              // There are potentially five belt segment items to paint. Gotta check all of them.
              // let tnow = ((factory.ticks - cell.segments[SegmentDirection::UP].at) as f64).max(0.001).min(cell.speed as f64);
              // let progress = (tnow / (cell.speed as f64)) * segment_h;
              // let progress_y = if cell.belt.direction_u == Port::In { progress } else { part_h - progress };

              let progress_u = pro_lu(factory.ticks, cell.segments[SegmentDirection::UP as usize].at, cell.speed, SEGMENT_H, cell.belt.direction_u);
              let dux = WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.0 * SEGMENT_W) + (SEGMENT_W / 2.0) + -(PART_W / 2.0);
              let duy = WORLD_OFFSET_Y + CELL_H * (cy as f64) + (0.0 * SEGMENT_H) + progress_u + -(PART_H / 2.0);
              paint_segment_part(&context, &part_tile_sprite, cell.segments[SegmentDirection::UP as usize].part.clone(), 16.0, 16.0, dux, duy, PART_W, PART_H);
              let progress_r = pro_dr(factory.ticks, cell.segments[SegmentDirection::RIGHT as usize].at, cell.speed, SEGMENT_W, cell.belt.direction_r);
              let drx = WORLD_OFFSET_X + CELL_W * (cx as f64) + (2.0 * SEGMENT_W) + progress_r + -(PART_W / 2.0);
              let dry = WORLD_OFFSET_Y + CELL_H * (cy as f64) + (1.0 * SEGMENT_H) + (SEGMENT_H / 2.0) + -(PART_H / 2.0);
              paint_segment_part(&context, &part_tile_sprite, cell.segments[SegmentDirection::RIGHT as usize].part.clone(), 16.0, 16.0, drx, dry, PART_W, PART_H);
              let progress_d = pro_dr(factory.ticks, cell.segments[SegmentDirection::DOWN as usize].at, cell.speed, SEGMENT_H, cell.belt.direction_d);
              let ddx = WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.0 * SEGMENT_W) + (SEGMENT_W / 2.0) + -(PART_W / 2.0);
              let ddy = WORLD_OFFSET_Y + CELL_H * (cy as f64) + (2.0 * SEGMENT_H) + progress_d + -(PART_H / 2.0);
              paint_segment_part(&context, &part_tile_sprite, cell.segments[SegmentDirection::DOWN as usize].part.clone(), 16.0, 16.0, ddx, ddy, PART_W, PART_H);
              let progress_l = pro_lu(factory.ticks, cell.segments[SegmentDirection::LEFT as usize].at, cell.speed, SEGMENT_W, cell.belt.direction_l);
              let dlx = WORLD_OFFSET_X + CELL_W * (cx as f64) + (0.0 * SEGMENT_W) + progress_l + -(PART_W / 2.0);
              let dly = WORLD_OFFSET_Y + CELL_H * (cy as f64) + (1.0 * SEGMENT_H) + (SEGMENT_H / 2.0) + -(PART_H / 2.0);
              paint_segment_part(&context, &part_tile_sprite, cell.segments[SegmentDirection::LEFT as usize].part.clone(), 16.0, 16.0, dlx, dly, PART_W, PART_H);

              // Center segments are the most annoying because it depends on which belt it came
              // from and to which belt it is going. To this end we have the .allocated props.
              // Before 50% progress paint the icon that much towards the center coming from the
              // segment that handed it to the center. At 50% the part should be at the center
              // of the cell. From there on out paint the progress from center to outgoing port
              // of the preassigned target belt. (cell.to or whatever)

              let progress_c = pro(factory.ticks, cell.segments[SegmentDirection::CENTER as usize].at, cell.speed);
              let first_half = progress_c < 0.5;
              // world offset in canvas + cell offset (x,y) + segment offset (1,1 for center) - half the icon size to put anchor to icon center. then conditionally add the progress
              let sx = WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.0 * SEGMENT_W) + -(PART_W * 0.5);
              let sy = WORLD_OFFSET_Y + CELL_H * (cy as f64) + (1.0 * SEGMENT_H) + -(PART_H * 0.5);
              let (px, py) =
                match if first_half { cell.segments[SegmentDirection::CENTER as usize].from } else { cell.segments[SegmentDirection::CENTER as usize].to } {
                  SegmentDirection::UP => {
                    let cux = sx + (SEGMENT_W * 0.5);
                    let cuy = sy + (SEGMENT_W * (if first_half { progress_c } else { 1.0 - progress_c }));
                    (cux, cuy)
                  }
                  SegmentDirection::RIGHT => {
                    let dlx = sx + (SEGMENT_W * (if first_half { 1.0 - progress_c } else { progress_c }));
                    let dly = sy + (SEGMENT_H * 0.5);
                    (dlx, dly)
                  }
                  SegmentDirection::DOWN => {
                    let cux = sx + (SEGMENT_W * 0.5);
                    let cuy = sy + (SEGMENT_W * (if first_half { 1.0 - progress_c } else { progress_c }));
                    (cux, cuy)
                  }
                  SegmentDirection::LEFT => {
                    let dlx = sx + (SEGMENT_W * (if first_half { progress_c } else { 1.0 - progress_c }));
                    let dly = sy + (SEGMENT_H * 0.5);
                    (dlx, dly)
                  }
                  SegmentDirection::CENTER => panic!(".from cannot be center"),
                };

              if paint_segment_part(&context, &part_tile_sprite, cell.segments[SegmentDirection::CENTER as usize].part.clone(), 16.0, 16.0, px, py, PART_W, PART_H) {
                // context.set_font(&"8px monospace");
                // context.set_fill_style(&"green".into());
                // context.fill_text(format!("{} {}x{}", coord, x, y).as_str(), px + 3.0, py + 10.0).expect("something error fill_text");
                // context.fill_text(format!("{}", progress_c).as_str(), px + 3.0, py + 21.0).expect("something error fill_text");
              }
            },
            CellKind::Machine => {
              match cell.machine {
                Machine::None => panic!("Machine cells should not be Machine::None"),
                | Machine::Composer
                | Machine::Smasher => {
                  // Paint the inputs (1, 2, or 3) and output
                  paint_segment_part(&context, &part_tile_sprite, cell.machine_input_1_want.clone(), 16.0, 16.0, WORLD_OFFSET_X + CELL_W * (cx as f64) + (0.0 * SEGMENT_W) + 4.0, WORLD_OFFSET_Y + CELL_H * (cy as f64) + (0.0 * SEGMENT_H) + 4.0, PART_W, PART_H);
                  paint_segment_part(&context, &part_tile_sprite, cell.machine_input_2_want.clone(), 16.0, 16.0, WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.0 * SEGMENT_W) + 4.0, WORLD_OFFSET_Y + CELL_H * (cy as f64) + (0.0 * SEGMENT_H) + 4.0, PART_W, PART_H);
                  paint_segment_part(&context, &part_tile_sprite, cell.machine_input_3_want.clone(), 16.0, 16.0, WORLD_OFFSET_X + CELL_W * (cx as f64) + (2.0 * SEGMENT_W) + 4.0, WORLD_OFFSET_Y + CELL_H * (cy as f64) + (0.0 * SEGMENT_H) + 4.0, PART_W, PART_H);
                  paint_segment_part(&context, &part_tile_sprite, cell.machine_input_1_have.clone(), 16.0, 16.0, WORLD_OFFSET_X + CELL_W * (cx as f64) + (0.0 * SEGMENT_W) + 4.0, WORLD_OFFSET_Y + CELL_H * (cy as f64) + (1.0 * SEGMENT_H) + 4.0, PART_W, PART_H);
                  paint_segment_part(&context, &part_tile_sprite, cell.machine_input_2_have.clone(), 16.0, 16.0, WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.0 * SEGMENT_W) + 4.0, WORLD_OFFSET_Y + CELL_H * (cy as f64) + (1.0 * SEGMENT_H) + 4.0, PART_W, PART_H);
                  paint_segment_part(&context, &part_tile_sprite, cell.machine_input_3_have.clone(), 16.0, 16.0, WORLD_OFFSET_X + CELL_W * (cx as f64) + (2.0 * SEGMENT_W) + 4.0, WORLD_OFFSET_Y + CELL_H * (cy as f64) + (1.0 * SEGMENT_H) + 4.0, PART_W, PART_H);
                  paint_segment_part(&context, &part_tile_sprite, cell.machine_output_want.clone(), 16.0, 16.0, WORLD_OFFSET_X + CELL_W * (cx as f64) + (1.0 * SEGMENT_W) + 4.0, WORLD_OFFSET_Y + CELL_H * (cy as f64) + (2.0 * SEGMENT_H) + 4.0, PART_W, PART_H);
                },
              }
            },
            CellKind::Supply => {
              // TODO: paint outbound supply part
            }
            CellKind::Demand => {
              // TODO: paint demand parts (none?)
            }
          }
        }

        // Paint mouse
        if mcx >= 0.0 && mcy >= 0.0 && mcx < WORLD_CELLS_X && mcy < WORLD_CELLS_Y {
          // If near the center of a cell edge show a click hint to change the direction of flow
          // Note mcrx/y is normalized progress of mouse in the current cell (mcx/y), so 0.0<=n<1.0

          let found = hit_check_between_belts(&mut factory, mcx, mcy, mcrx, mcry);

          match found {
            SegmentDirection::UP => {
              context.set_fill_style(&"blue".into()); // Semi transparent circles
              context.fill_rect((mcx + 0.3) * CELL_W, (mcy - 0.2) * CELL_H, CELL_W * 0.35, CELL_W * 0.4);
            }
            SegmentDirection::RIGHT => {
              context.set_fill_style(&"blue".into()); // Semi transparent circles
              context.fill_rect((mcx + 0.8) * CELL_W, (mcy + 0.3) * CELL_H, CELL_W * 0.4, CELL_W * 0.35);
            }
            SegmentDirection::DOWN => {
              context.set_fill_style(&"blue".into()); // Semi transparent circles
              context.fill_rect((mcx + 0.3) * CELL_W, (mcy + 0.8) * CELL_H, CELL_W * 0.35, CELL_W * 0.4);
            }
            SegmentDirection::LEFT => {
              context.set_fill_style(&"blue".into()); // Semi transparent circles
              context.fill_rect((mcx - 0.2) * CELL_W, (mcy + 0.3) * CELL_H, CELL_W * 0.4, CELL_W * 0.35);
            }
            SegmentDirection::CENTER => {
              // Not on a belt connector
              context.set_fill_style(&"#ff00ff7f".into()); // Semi transparent circles
              context.begin_path();
              context.ellipse(mwx, mwy, PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
              context.fill();
            }
          }

          if mcx != selected_cell_x || mcy != selected_cell_y {
            context.set_stroke_style(&"red".into());
            context.stroke_rect(WORLD_OFFSET_X + mcx * CELL_W, WORLD_OFFSET_Y + mcy * CELL_H, CELL_W, CELL_H);
          }
        }

        // Clear cell editor
        context.set_fill_style(&"white".into());
        context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * 10.0), UI_W, UI_LINE_H * 10.0);

        if selected_cell {
          // Mark the currently selected cell
          context.set_stroke_style(&"blue".into());
          context.stroke_rect(WORLD_OFFSET_X + selected_cell_x * CELL_W, WORLD_OFFSET_Y + selected_cell_y * CELL_H, CELL_W, CELL_H);

          // Paint cell editor
          context.set_fill_style(&"lightgreen".into());
          context.fill_rect(UI_CELL_EDITOR_OX, UI_CELL_EDITOR_OY, UI_CELL_EDITOR_W, UI_CELL_EDITOR_H);
          context.set_fill_style(&"black".into());
          context.fill_text(format!("Cell {} x {} ({})", selected_cell_x, selected_cell_y, to_coord(WORLD_CELLS_X as usize, selected_cell_x as usize, selected_cell_y as usize)).as_str(), UI_OX + UI_ML + 10.0, UI_OY + ((UI_DEBUG_LINES + 2.0) * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

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

          if hit_check_cell_editor_grid(mwx, mwy) {
            // Mouse is inside the grid editor
            // Determine which segment and then paint it
            let sx = ((mwx - UI_CELL_EDITOR_GRID_OX) / UI_SEGMENT_W).floor();
            let sy = ((mwy - UI_CELL_EDITOR_GRID_OY) / UI_SEGMENT_H).floor();
            context.set_stroke_style(&"red".into());
            context.stroke_rect(ox + (sx * UI_SEGMENT_W), oy + (sy * UI_SEGMENT_H), UI_SEGMENT_W, UI_SEGMENT_H);
          }

          // Box where the type of the cell will be painted. Like a button.
          context.set_stroke_style(&"black".into());
          context.stroke_rect(UI_CELL_EDITOR_KIND_OX, UI_CELL_EDITOR_KIND_OY, UI_CELL_EDITOR_KIND_W, UI_CELL_EDITOR_KIND_H);
          // Draw the type of the cell in this box
          context.set_stroke_style(&"black".into());
          let coord = to_coord(factory.floor.width + 2, selected_cell_x as usize, selected_cell_y as usize);
          let type_name = match factory.floor.cells[coord].kind {
            CellKind::Empty => "Empty",
            CellKind::Belt => "Belt",
            CellKind::Machine => "Machine",
            CellKind::Supply => "Supply",
            CellKind::Demand => "Demand",
          };
          context.stroke_text(type_name, UI_CELL_EDITOR_KIND_OX + 4.0, UI_CELL_EDITOR_KIND_OY + UI_FONT_H + 3.0).expect("to paint port");

          if factory.floor.cells[coord].kind == CellKind::Belt {
            // Paint ports
            context.set_stroke_style(&"black".into());
            match factory.floor.cells[coord].segments[SegmentDirection::UP as usize].port {
              Port::Inbound => {
                context.stroke_text("in", ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (0.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
              },
              Port::Outbound => {
                context.stroke_text("out", ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (0.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
              },
              Port::None => {},
            }

            context.stroke_text(format!("{} {} {}",
              if factory.floor.cells[coord].segments[SegmentDirection::UP as usize].part.kind != PartKind::None { 'p' } else { ' ' },
              if factory.floor.cells[coord].segments[SegmentDirection::UP as usize].allocated { 'a' } else { ' ' },
              if factory.floor.cells[coord].segments[SegmentDirection::UP as usize].claimed { 'c' } else { ' ' },
            ).as_str(), ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (0.0 * UI_SEGMENT_H) + 2.0 * UI_FONT_H).expect("to paint port");

            match factory.floor.cells[coord].segments[SegmentDirection::RIGHT as usize].port {
              Port::Inbound => {
                context.stroke_text("in", ox + (2.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
              },
              Port::Outbound => {
                context.stroke_text("out", ox + (2.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
              },
              Port::None => {},
            }

            context.stroke_text(format!("{} {} {}",
              if factory.floor.cells[coord].segments[SegmentDirection::RIGHT as usize].part.kind != PartKind::None { 'p' } else { ' ' },
              if factory.floor.cells[coord].segments[SegmentDirection::RIGHT as usize].allocated { 'a' } else { ' ' },
              if factory.floor.cells[coord].segments[SegmentDirection::RIGHT as usize].claimed { 'c' } else { ' ' },
            ).as_str(), ox + (2.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + 2.0 * UI_FONT_H).expect("to paint port");

            match factory.floor.cells[coord].segments[SegmentDirection::DOWN as usize].port {
              Port::Inbound => {
                context.stroke_text("in", ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (2.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
              },
              Port::Outbound => {
                context.stroke_text("out", ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (2.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
              },
              Port::None => {},
            }

            context.stroke_text(format!("{} {} {}",
              if factory.floor.cells[coord].segments[SegmentDirection::DOWN as usize].part.kind != PartKind::None { 'p' } else { ' ' },
              if factory.floor.cells[coord].segments[SegmentDirection::DOWN as usize].allocated { 'a' } else { ' ' },
              if factory.floor.cells[coord].segments[SegmentDirection::DOWN as usize].claimed { 'c' } else { ' ' },
            ).as_str(), ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (2.0 * UI_SEGMENT_H) + 2.0 * UI_FONT_H).expect("to paint port");

            match factory.floor.cells[coord].segments[SegmentDirection::LEFT as usize].port {
              Port::Inbound => {
                context.stroke_text("in", ox + (0.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
              },
              Port::Outbound => {
                context.stroke_text("out", ox + (0.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
              },
              Port::None => {},
            }

            context.stroke_text(format!("{} {} {}",
              if factory.floor.cells[coord].segments[SegmentDirection::LEFT as usize].part.kind != PartKind::None { 'p' } else { ' ' },
              if factory.floor.cells[coord].segments[SegmentDirection::LEFT as usize].allocated { 'a' } else { ' ' },
              if factory.floor.cells[coord].segments[SegmentDirection::LEFT as usize].claimed { 'c' } else { ' ' },
            ).as_str(), ox + (0.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + 2.0 * UI_FONT_H).expect("to paint port");

            // center
            context.stroke_text(format!("{} {} {}",
              if factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].part.kind != PartKind::None { 'p' } else { ' ' },
              if factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].allocated { 'a' } else { ' ' },
              if factory.floor.cells[coord].segments[SegmentDirection::CENTER as usize].claimed { 'c' } else { ' ' },
            ).as_str(), ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + 2.0 * UI_FONT_H).expect("to paint port");
          }
        }
      }

      // Schedule next frame
      request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
  }

  Ok(())
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
