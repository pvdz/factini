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
use super::cli_serialize::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::state::*;
use super::utils::*;

// These are the actual pixels we can paint to
const CANVAS_WIDTH: f64 = 1000.0;
const CANVAS_HEIGHT: f64 = 1000.0;
// Need this for mouse2world coord conversion
const CANVAS_CSS_WIDTH: f64 = 1000.0;
const CANVAS_CSS_HEIGHT: f64 = 1000.0;

// World size in world pixels (as painted on the canvas)
const WORLD_OFFSET_X: f64 = 0.0;
const WORLD_OFFSET_Y: f64 = 0.0;
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

#[derive(Debug)]
struct CellSelection {
  on: bool,
  x: f64,
  y: f64,
  coord: usize,
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

  was_down: bool,
  was_dragging: bool,
  was_up: bool,

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

fn hit_check_between_world_belts(factory: &Factory, coord: usize, crx: f64, cry: f64) -> Option<Direction> {
  if factory.floor[coord].kind != CellKind::Belt {
    return None;
  }

  if crx >= 0.33 && crx < 0.66 {
    if cry < 0.20 {
      if let Some(coord) = factory.floor[coord].coord_u {
        if factory.floor[coord].kind == CellKind::Belt {
          return Some(Direction::Up);
        }
      }
    } else if cry > 0.80 {
      if let Some(coord) = factory.floor[coord].coord_d {
        if factory.floor[coord].kind == CellKind::Belt {
          return Some(Direction::Down);
        }
      }
    }
  } else if cry >= 0.33 && cry < 0.66 {
    if crx < 0.20 {
      if let Some(coord) = factory.floor[coord].coord_l {
        if factory.floor[coord].kind == CellKind::Belt {
          return Some(Direction::Left);
        }
      }
    } else if crx > 0.80 {
      if let Some(coord) = factory.floor[coord].coord_r {
        if factory.floor[coord].kind == CellKind::Belt {
          return Some(Direction::Right);
        }
      }
    }
  }

  return None;
}

fn hit_check_cell_editor_any(wx: f64, wy: f64) -> bool {
  // log(format!("hit_check_cell_editor_any({}, {}) {} {} {} {} = {}", wx, wy, UI_CELL_EDITOR_OX, UI_CELL_EDITOR_OY, UI_CELL_EDITOR_OX + UI_CELL_EDITOR_W, UI_CELL_EDITOR_OY + UI_CELL_EDITOR_H, wx >= UI_CELL_EDITOR_OX && wy >= UI_CELL_EDITOR_OY && wx < UI_CELL_EDITOR_OX + UI_CELL_EDITOR_W && wy < UI_CELL_EDITOR_OY + UI_CELL_EDITOR_H));
  return wx >= UI_CELL_EDITOR_OX && wy >= UI_CELL_EDITOR_OY && wx < UI_CELL_EDITOR_OX + UI_CELL_EDITOR_W && wy < UI_CELL_EDITOR_OY + UI_CELL_EDITOR_H;
}
fn hit_check_cell_editor_grid(wx: f64, wy: f64) -> bool {
  // log(format!("hit_check_cell_editor_grid({}, {}) {} {} {} {} = {}", wx, wy, UI_CELL_EDITOR_GRID_OX, UI_CELL_EDITOR_GRID_OY, UI_CELL_EDITOR_GRID_OX + UI_CELL_EDITOR_GRID_W, UI_CELL_EDITOR_GRID_OY + UI_CELL_EDITOR_GRID_H, wx >= UI_CELL_EDITOR_GRID_OX && wx < UI_CELL_EDITOR_GRID_OX + UI_CELL_EDITOR_GRID_W && wy >= UI_CELL_EDITOR_GRID_OY && wy < UI_CELL_EDITOR_GRID_OY + UI_CELL_EDITOR_GRID_H));
  return wx >= UI_CELL_EDITOR_GRID_OX && wx < UI_CELL_EDITOR_GRID_OX + UI_CELL_EDITOR_GRID_W && wy >= UI_CELL_EDITOR_GRID_OY && wy < UI_CELL_EDITOR_GRID_OY + UI_CELL_EDITOR_GRID_H;
}
fn hit_check_cell_editor_kind(wx: f64, wy: f64) -> bool {
  return wx >= UI_CELL_EDITOR_KIND_OX && wx < UI_CELL_EDITOR_KIND_OX + UI_CELL_EDITOR_KIND_W && wy >= UI_CELL_EDITOR_KIND_OY && wy < UI_CELL_EDITOR_KIND_OY + UI_CELL_EDITOR_KIND_H;
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
  canvas.style().set_property("width", format!("{}px", CANVAS_CSS_HEIGHT as u32).as_str())?;
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


  // Static state configuration (can still be changed by user)
  let mut options = create_options(10.0);

  // General app state
  let mut state = State {};

  let map = "\
    ...............s.\n\
    sb.111bbbbbbbb.b.\n\
    .b.111.......b.b.\n\
    .b.111bbbbb..bbb.\n\
    .b.b.b....bb...b.\n\
    .b.b.bbbb..b...b.\n\
    .b.b....b..b...b.\n\
    .bbb...222.b...b.\n\
    ...b...222.b..bb.\n\
    sbbb...222.b..b..\n\
    ........b..b..bbs\n\
    ..bbbbbbb..b.....\n\
    ..b.....b..bbbbbd\n\
    ..b.....b........\n\
    dbb..bbbb........\n\
    .....b...........\n\
    .....d...........\n\
    m1 = ws -> b\n\
    m2 = b -> g\n\
    s1 = w\n\
    s2 = w\n\
    s3 = s\n\
    s4 = s\n\
    d1 = s\n\
    d2 = w\n\
    d3 = g\n\
    d4 = g\n\
  ";
  let mut factory = create_factory(&mut options, &mut state, map.to_string());
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

    let mut prev_time = start_time;

    let mut fps: VecDeque<f64> = VecDeque::new();

    let mut cell_selection = CellSelection {
      on: false,
      x: 0.0,
      y: 0.0,
      coord: 0,
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

      update_mouse_state(&mut mouse_state, mouse_x.get(), mouse_y.get(), last_mouse_down_x.get(), last_mouse_down_y.get(), last_mouse_down_button.get(), last_mouse_up_x.get(), last_mouse_up_y.get());
      last_mouse_down_x.set(0.0);
      last_mouse_down_y.set(0.0);
      last_mouse_up_x.set(0.0);
      last_mouse_up_y.set(0.0);

      context.set_font(&"12px monospace");

      for _ in 0..ticks_todo.min(MAX_TICKS_PER_FRAME) {
        tick_factory(&mut options, &mut state, &mut factory);
      }

      if options.web_output_cli {
        paint_world_cli(&context, &mut options, &mut state, &factory);
      } else {
        // Handle drag-end or click
        handle_input(&mut cell_selection, &mut mouse_state, &mut options, &mut state, &mut factory);

        paint_green_debug(&context, &fps, now, since_prev, ticks_todo, &factory, &mouse_state);

        // Paint the world

        // Clear world
        context.set_fill_style(&"#E86A17".into());
        // context.set_fill_style(&"lightblue".into());
        context.fill_rect(WORLD_OFFSET_X, WORLD_OFFSET_Y, FLOOR_CELLS_W as f64 * CELL_W + 20.0, FLOOR_CELLS_H as f64 * CELL_H + 20.0);

        // TODO: wait for tiles to be loaded because first few frames won't paint anything while the tiles are loading...
        paint_background_tiles(&context, &factory, &belt_tile_images, &img_machine2);
        paint_ports(&context, &factory);
        paint_belt_items(&context, &factory, &part_tile_sprite);

        if mouse_state.cell_x >= 0.0 && mouse_state.cell_y >= 0.0 && mouse_state.cell_x < FLOOR_CELLS_W as f64 && mouse_state.cell_y < FLOOR_CELLS_H as f64 {
          paint_mouse_pointer_in_world(&context, &factory, &cell_selection, &mouse_state, &belt_tile_images);
        }

        paint_cell_editor(&context, &factory, &cell_selection, &mouse_state);
      }

      // Schedule next frame
      request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
  }

  Ok(())
}

fn update_mouse_state(mouse_state: &mut MouseState, mouse_x: f64, mouse_y: f64, last_mouse_down_x: f64, last_mouse_down_y: f64, last_mouse_down_button: u16, last_mouse_up_x: f64, last_mouse_up_y: f64) {
  // https://docs.rs/web-sys/0.3.28/web_sys/struct.CanvasRenderingContext2d.html

  // Reset
  mouse_state.was_down = false;
  mouse_state.was_up = false;
  mouse_state.was_dragging = false;

  // Mouse coords
  // Note: mouse2world coord is determined by _css_ size, not _canvas_ size
  mouse_state.canvas_x = mouse_x;
  mouse_state.canvas_y = mouse_y;
  mouse_state.world_x = mouse_x / CANVAS_CSS_WIDTH * CANVAS_WIDTH;
  mouse_state.world_y = mouse_y / CANVAS_CSS_WIDTH * CANVAS_WIDTH;
  mouse_state.cell_x = (mouse_x / CELL_W).floor();
  mouse_state.cell_y = (mouse_y / CELL_W).floor();
  mouse_state.cell_coord = to_coord(mouse_state.cell_x as usize, mouse_state.cell_y as usize);
  mouse_state.cell_rel_x = (mouse_state.world_x / CELL_W) - mouse_state.cell_x;
  mouse_state.cell_rel_y = (mouse_state.world_y / CELL_H) - mouse_state.cell_y;

  if last_mouse_down_x > 0.0 || last_mouse_down_y > 0.0 {
    mouse_state.last_down_button = last_mouse_down_button;
    mouse_state.last_down_canvas_x = last_mouse_down_x;
    mouse_state.last_down_canvas_y = last_mouse_down_y;
    mouse_state.last_down_world_x = last_mouse_down_x / CANVAS_CSS_WIDTH * CANVAS_WIDTH;
    mouse_state.last_down_world_y = last_mouse_down_y / CANVAS_CSS_HEIGHT * CANVAS_HEIGHT;
    mouse_state.is_down = true;
    mouse_state.was_down = true; // this frame, in case there's also an up event
  }

  // determine whether mouse is considered to be dragging (there's a buffer of movement before
  // we consider a mouse down to mouse up to be dragging. But once we do, we stick to it.)
  if mouse_state.is_down && !mouse_state.is_dragging {
    // 5 world pixels? sensitivity tbd
    if (mouse_state.last_down_world_x - mouse_state.world_x).abs() > 5.0 || (mouse_state.last_down_world_y - mouse_state.world_y).abs() > 5.0 {
      mouse_state.is_dragging = true;
    }
  }

  if last_mouse_up_x > 0.0 || last_mouse_up_y > 0.0 {
    mouse_state.last_up_canvas_x = last_mouse_up_x;
    mouse_state.last_up_canvas_y = last_mouse_up_y;
    mouse_state.last_up_world_x = last_mouse_up_x / CANVAS_CSS_WIDTH * CANVAS_WIDTH;
    mouse_state.last_up_world_y = last_mouse_up_y / CANVAS_CSS_HEIGHT * CANVAS_HEIGHT;
    mouse_state.is_down = false;
    mouse_state.was_up = true;
    if mouse_state.is_dragging {
      mouse_state.is_dragging = false;
      mouse_state.was_dragging = true;
    }
  }
}
fn paint_green_debug(context: &Rc<web_sys::CanvasRenderingContext2d>, fps: &VecDeque<f64>, now: f64, since_prev: f64, ticks_todo: u64, factory: &Factory, mouse_state: &MouseState) {

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
  // context.fill_text(format!("$ /  1s    : {}", factory.stats.2).as_str(), UI_OX + UI_ML, UI_OY + (ui_lines * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

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
fn handle_input(cell_selection: &mut CellSelection, mouse_state: &mut MouseState, options: &mut Options, state: &mut State, factory: &mut Factory) {
  if mouse_state.was_up {
    if mouse_state.is_dragging {
      // This is more a visual thing I think
    } else {
      // Was the click inside the painted world?
      // In that case we change/toggle the cell selection
      if mouse_state.last_up_world_x >= WORLD_OFFSET_X && mouse_state.last_up_world_y >= WORLD_OFFSET_Y && mouse_state.last_up_world_x < WORLD_OFFSET_X + WORLD_WIDTH && mouse_state.last_up_world_y < WORLD_OFFSET_Y + WORLD_HEIGHT {
        on_up_inside_floor(options, state, factory, cell_selection, &mouse_state);
      }
      // Is the click inside the cell editor?
      else if cell_selection.on && hit_check_cell_editor_any(mouse_state.last_up_world_x, mouse_state.last_up_world_y) {
        // Is the mouse clicking on one of the focused grid segments?
        if hit_check_cell_editor_grid(mouse_state.last_up_world_x, mouse_state.last_up_world_y) {
          on_click_inside_cell_editor_grid(options, state, factory, &cell_selection, &mouse_state);
        } else {
          // Is the mouse clicking on the focused cell's "kind" box?
          if hit_check_cell_editor_kind(mouse_state.last_up_world_x, mouse_state.last_up_world_y) {
            on_click_inside_cell_editor_kind(options, state, factory, &cell_selection, &mouse_state);
          }
        }
      }
    }
  }
}
fn on_click_inside_cell_editor_kind(options: &Options, state: &State, factory: &mut Factory, cell_selection: &CellSelection, mouse_state: &MouseState) {
  // There are two cases; edge and middle cells. Supply/Demand can only go on edge.
  // Machine and Belt can only go in middle. Empty can go anywhere.
  log(format!("from the {} {} {} {}", mouse_state.cell_x, mouse_state.cell_y, FLOOR_CELLS_W as f64 - 1.0, FLOOR_CELLS_H as f64 - 1.0));
  if cell_selection.x == 0.0 || cell_selection.y == 0.0 || cell_selection.x == FLOOR_CELLS_W as f64 - 1.0 || cell_selection.y == FLOOR_CELLS_H as f64 - 1.0 {
    log(format!("from the top"));
    // Edge. Cycle between Empty, Supply, and Demand
    match factory.floor[cell_selection.coord].kind {
      CellKind::Empty => {
        // x: usize, y: usize, part: Part, speed: u64, cooldown: u64, price: i32
        factory.floor[cell_selection.coord] = supply_cell(factory.floor[cell_selection.coord].x, factory.floor[cell_selection.coord].y, part_c('g'), 1000, 10000, 10000);
      },
      CellKind::Supply => {
        // x: usize, y: usize, part: Part
        factory.floor[cell_selection.coord] = demand_cell(factory.floor[cell_selection.coord].x, factory.floor[cell_selection.coord].y, part_c('g'));
      },
      CellKind::Demand => {
        factory.floor[cell_selection.coord] = empty_cell(factory.floor[cell_selection.coord].x, factory.floor[cell_selection.coord].y);
      },
      | CellKind::Belt
      | CellKind::Machine
      => panic!("edge should not contain machine or belt"),
    }
  } else {
    log(format!("from the middle"));
    // Middle. Cycle between Empty, Machine, and Belt
    match factory.floor[cell_selection.coord].kind {
      CellKind::Empty => {
        factory.floor[cell_selection.coord] = belt_cell(factory.floor[cell_selection.coord].x, factory.floor[cell_selection.coord].y, BELT_NONE);
      },
      CellKind::Belt => {
        factory.floor[cell_selection.coord] = machine_cell(factory.floor[cell_selection.coord].x, factory.floor[cell_selection.coord].y, MachineKind::Main, part_none(), part_none(), part_none(), part_none(), -15, -3);
      },
      CellKind::Machine => {
        factory.floor[cell_selection.coord] = empty_cell(factory.floor[cell_selection.coord].x, factory.floor[cell_selection.coord].y);
      }
      | CellKind::Supply
      | CellKind::Demand
      => panic!("middle should not contain supply or demand"),
    }
  };

  // Recreate cell traversal order
  let prio: Vec<usize> = create_prio_list(options, &mut factory.floor);
  log(format!("Updated prio list: {:?}", prio));
  factory.prio = prio;
}
fn paint_background_tiles(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, belt_tile_images: &Vec<web_sys::HtmlImageElement>, img_machine2: &web_sys::HtmlImageElement) {
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
        context.draw_image_with_html_image_element_and_dw_and_dh(img_machine2, ox, oy, CELL_W, CELL_H).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
      },
      CellKind::Supply => {
        // TODO: paint supply image
        context.set_fill_style(&"pink".into());
        context.fill_rect( ox, oy, CELL_W, CELL_H);
        context.set_fill_style(&"black".into());
        context.fill_text("S", ox + 13.0, oy + 21.0).expect("something lower error fill_text");
      }
      CellKind::Demand => {
        // TODO: paint demand image
        context.set_fill_style(&"lightgreen".into());
        context.fill_rect( ox, oy, CELL_W, CELL_H);
        context.set_fill_style(&"black".into());
        context.fill_text("D", ox + 13.0, oy + 21.0).expect("something lower error fill_text");
      }
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
        let progress_c = progress(factory.ticks, cell.belt.part_at, cell.belt.speed).min(1.0);
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
fn paint_mouse_pointer_in_world(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState, belt_tile_images: &Vec<web_sys::HtmlImageElement>) {

  // // If near the center of a cell edge show a click hint to change the direction of flow
  // // Note mouse_inside_cell_x/y is normalized progress of mouse in the current cell (mouse_cell_x/y), so 0.0<=n<1.0
  //
  // let found = hit_check_between_world_belts(factory, mouse_state.cell_coord, mouse_state.cell_rel_x, mouse_state.cell_rel_y);
  //
  // match found {
  //   Some(Direction::Up) => {
  //     context.set_fill_style(&"blue".into()); // Semi transparent circles
  //     context.fill_rect((mouse_state.cell_x + 0.3) * CELL_W, (mouse_state.cell_y - 0.2) * CELL_H, CELL_W * 0.35, CELL_W * 0.4);
  //   }
  //   Some(Direction::Right) => {
  //     context.set_fill_style(&"blue".into()); // Semi transparent circles
  //     context.fill_rect((mouse_state.cell_x + 0.8) * CELL_W, (mouse_state.cell_y + 0.3) * CELL_H, CELL_W * 0.4, CELL_W * 0.35);
  //   }
  //   Some(Direction::Down) => {
  //     context.set_fill_style(&"blue".into()); // Semi transparent circles
  //     context.fill_rect((mouse_state.cell_x + 0.3) * CELL_W, (mouse_state.cell_y + 0.8) * CELL_H, CELL_W * 0.35, CELL_W * 0.4);
  //   }
  //   Some(Direction::Left) => {
  //     context.set_fill_style(&"blue".into()); // Semi transparent circles
  //     context.fill_rect((mouse_state.cell_x - 0.2) * CELL_W, (mouse_state.cell_y + 0.3) * CELL_H, CELL_W * 0.4, CELL_W * 0.35);
  //   }
  //   None => {
  //  // Not on a belt connector
      context.set_fill_style(&"#ff00ff7f".into()); // Semi transparent circles
      context.begin_path();
      context.ellipse(mouse_state.world_x, mouse_state.world_y, PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
      context.fill();
  //   }
  // }

  if mouse_state.cell_x != cell_selection.x || mouse_state.cell_y != cell_selection.y {
    context.set_stroke_style(&"red".into());
    context.stroke_rect(WORLD_OFFSET_X + mouse_state.cell_x * CELL_W, WORLD_OFFSET_Y + mouse_state.cell_y * CELL_H, CELL_W, CELL_H);
  }

  if mouse_state.is_dragging {
    paint_belt_drag_preview(context, factory, cell_selection, mouse_state, belt_tile_images);
  }
}
fn paint_belt_drag_preview(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState, belt_tile_images: &Vec<web_sys::HtmlImageElement>) {
  let track = ray_trace_dragged_line(
    factory,
    (mouse_state.last_down_world_x / CELL_W).floor(),
    (mouse_state.last_down_world_y / CELL_H).floor(),
    mouse_state.cell_x.floor(),
    mouse_state.cell_y.floor(),
  );

  // Draw line from mouse down to mouse up (algo will pick different cells tho)
  // context.set_stroke_style(&"black".into());
  // context.begin_path();
  // // Mouse to mouse
  // context.move_to(WORLD_OFFSET_X + x0 * CELL_W, WORLD_OFFSET_X + y0 * CELL_H);
  // context.line_to(WORLD_OFFSET_X + x1 * CELL_W, WORLD_OFFSET_X + y1 * CELL_H);
  // // Cell origin to cell origin
  // // context.move_to(mouse_state.last_down_world_x, mouse_state.last_down_world_y);
  // // context.line_to(mouse_state.world_x, mouse_state.world_y);
  // context.stroke();

  for index in 0..track.len() {
    let ((x, y), bt, in_port_dir, out_port_dir) = track[index];
    context.set_fill_style(&"#00770044".into());
    context.fill_rect(WORLD_OFFSET_X + CELL_W * (x as f64), WORLD_OFFSET_Y + CELL_H * (y as f64), CELL_W, CELL_H);
    paint_ghost_belt_of_type(x, y, if mouse_state.last_down_button == 2 { BeltType::INVALID } else { bt }, &context, &belt_tile_images);
  }
}
fn ray_trace_dragged_line(factory: &Factory, x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<((usize, usize), BeltType, Direction, Direction)> {
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
  let (mut lx, mut ly) = covered[0];
  let mut last_from = Direction::Up; // first one ignores this value
  for index in 1..covered.len() {
    let (x, y) = covered[index];
    // Always set the previous one.
    let new_from = get_from_dir_between_xy(lx, ly, x, y);
    let last_to = direction_reverse(new_from);
    // For the first one, pass on the same "to" port since there is no "from" port (it'll be a noop)
    let bt =
      if index == 1 {
        add_one_ports_to_cell(factory, to_coord(lx, ly), last_to)
      } else {
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
fn paint_ghost_belt_of_type(cell_x: usize, cell_y: usize, belt_type: BeltType, context: &Rc<web_sys::CanvasRenderingContext2d>, belt_tile_images: &Vec<web_sys::HtmlImageElement>) {
  let img: &HtmlImageElement = &belt_tile_images[belt_type as usize];

  context.set_global_alpha(0.5);
  context.draw_image_with_html_image_element_and_dw_and_dh(&img, WORLD_OFFSET_X + cell_x as f64 * CELL_W + 5.0, WORLD_OFFSET_X + cell_y as f64 * CELL_H + 5.0, CELL_W - 10.0, CELL_H - 10.0).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
  context.set_global_alpha(1.0);
}
fn paint_cell_editor(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState) {
  // Clear cell editor
  context.set_fill_style(&"white".into());
  context.fill_rect(UI_OX, UI_OY + (UI_LINE_H * 10.0), UI_W, UI_LINE_H * 10.0);

  if !cell_selection.on {
    return;
  }

  // Mark the currently selected cell
  context.set_stroke_style(&"blue".into());
  context.stroke_rect(WORLD_OFFSET_X + cell_selection.x * CELL_W, WORLD_OFFSET_Y + cell_selection.y * CELL_H, CELL_W, CELL_H);

  // Paint cell editor
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_CELL_EDITOR_OX, UI_CELL_EDITOR_OY, UI_CELL_EDITOR_W, UI_CELL_EDITOR_H);
  context.set_fill_style(&"black".into());
  context.fill_text(format!("Cell {} x {} ({})", cell_selection.x, cell_selection.y, to_coord(cell_selection.x as usize, cell_selection.y as usize)).as_str(), UI_OX + UI_ML + 10.0, UI_OY + ((UI_DEBUG_LINES + 2.0) * UI_LINE_H) + UI_FONT_H).expect("something error fill_text");

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

  if hit_check_cell_editor_grid(mouse_state.world_x, mouse_state.world_y) {
    // Mouse is inside the grid editor
    // Determine which segment and then paint it
    let mouse_cell_editor_grid_x = ((mouse_state.world_x - UI_CELL_EDITOR_GRID_OX) / UI_SEGMENT_W).floor();
    let mouse_cell_editor_grid_y = ((mouse_state.world_y - UI_CELL_EDITOR_GRID_OY) / UI_SEGMENT_H).floor();
    context.set_stroke_style(&"red".into());
    context.stroke_rect(ox + (mouse_cell_editor_grid_x * UI_SEGMENT_W), oy + (mouse_cell_editor_grid_y * UI_SEGMENT_H), UI_SEGMENT_W, UI_SEGMENT_H);
  }

  // Box where the type of the cell will be painted. Like a button.
  context.set_stroke_style(&"black".into());
  context.stroke_rect(UI_CELL_EDITOR_KIND_OX, UI_CELL_EDITOR_KIND_OY, UI_CELL_EDITOR_KIND_W, UI_CELL_EDITOR_KIND_H);
  // Draw the type of the cell in this box
  context.set_stroke_style(&"black".into());
  let type_name = match factory.floor[cell_selection.coord].kind {
    CellKind::Empty => "Empty",
    CellKind::Belt => "Belt",
    CellKind::Machine => "Machine",
    CellKind::Supply => "Supply",
    CellKind::Demand => "Demand",
  };
  context.stroke_text(type_name, UI_CELL_EDITOR_KIND_OX + 4.0, UI_CELL_EDITOR_KIND_OY + UI_FONT_H + 3.0).expect("to paint port");

  // Paint ports
  context.set_stroke_style(&"black".into());
  match factory.floor[cell_selection.coord].port_u {
    Port::Inbound => {
      context.stroke_text("in", ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (0.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
    Port::Outbound => {
      context.stroke_text("out", ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (0.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
    Port::None => {},
    Port::Unknown => {
      context.stroke_text("???", ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (0.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
  }

  match factory.floor[cell_selection.coord].port_r {
    Port::Inbound => {
      context.stroke_text("in", ox + (2.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
    Port::Outbound => {
      context.stroke_text("out", ox + (2.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
    Port::None => {},
    Port::Unknown => {
      context.stroke_text("???", ox + (2.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
  }

  match factory.floor[cell_selection.coord].port_d {
    Port::Inbound => {
      context.stroke_text("in", ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (2.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
    Port::Outbound => {
      context.stroke_text("out", ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (2.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
    Port::None => {},
    Port::Unknown => {
      context.stroke_text("???", ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (2.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
  }

  match factory.floor[cell_selection.coord].port_l {
    Port::Inbound => {
      context.stroke_text("in", ox + (0.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
    Port::Outbound => {
      context.stroke_text("out", ox + (0.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
    Port::None => {},
    Port::Unknown => {
      context.stroke_text("???", ox + (0.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + UI_FONT_H).expect("to paint port");
    },
  }

  if factory.floor[cell_selection.coord].kind == CellKind::Belt {
    // Paint current part
    context.stroke_text(format!("{}",
      if factory.floor[cell_selection.coord].belt.part.kind != PartKind::None { 'p' } else { ' ' },
      // if factory.floor[cell_selection.coord].allocated { 'a' } else { ' ' },
      // if factory.floor[cell_selection.coord].claimed { 'c' } else { ' ' },
    ).as_str(), ox + (1.0 * UI_SEGMENT_W) + 2.0, oy + (1.0 * UI_SEGMENT_H) + 2.0 * UI_FONT_H).expect("to paint port");
  }
}
fn on_up_inside_floor(options: &mut Options, state: &mut State, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState) {
  let last_mouse_up_cell_x = (mouse_state.last_up_world_x / CELL_W).floor();
  let last_mouse_up_cell_y = (mouse_state.last_up_world_y / CELL_H).floor();
  let last_mouse_up_cell_coord = to_coord(last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize);
  let last_mouse_up_inside_cell_x = (mouse_state.last_up_world_x / CELL_W) - last_mouse_up_cell_x;
  let last_mouse_up_inside_cell_y = (mouse_state.last_up_world_y / CELL_H) - last_mouse_up_cell_y;

  // Check if the icon between connected belts was clicked
  if last_mouse_up_cell_x >= 0.0 && last_mouse_up_cell_y >= 0.0 && last_mouse_up_cell_x < FLOOR_CELLS_W as f64 && last_mouse_up_cell_y < FLOOR_CELLS_H as f64 {
    if mouse_state.was_dragging {
      on_drag_inside_floor(options, state, factory, cell_selection, mouse_state);
    } else {
      on_click_inside_floor(options, state, factory, cell_selection, mouse_state, last_mouse_up_cell_x, last_mouse_up_cell_y);
    }
  }
}
fn on_drag_inside_floor(options: &mut Options, state: &mut State, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState) {
  // Finalize pathing, regenerate floor
  let track = ray_trace_dragged_line(
    factory,
    (mouse_state.last_down_world_x / CELL_W).floor(),
    (mouse_state.last_down_world_y / CELL_H).floor(),
    mouse_state.cell_x.floor(),
    mouse_state.cell_y.floor(),
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
      let ((x, y), _belt_type, _unused, _port_out_dir) = track[0]; // First element has no inbound port here
      let coord = to_coord(x, y);

      clear_part_from_cell(options, state, factory, coord);
    } else {
      // Other mouse button. ignore for now / ever.
      // I think this allows you to cancel a drag by pressing the rmb
    }
  } else if len == 2 {
    let ((x1, y1), belt_type1, _unused, _port_out_dir1) = track[0]; // First element has no inbound port here
    let coord1 = to_coord(x1, y1);
    let ((x2, y2), belt_type2, _port_in_dir2, _unused) = track[1]; // LAst element has no outbound port here
    let coord2 = to_coord(x2, y2);

    let dx = (x1 as i8) - (x2 as i8);
    let dy = (y1 as i8) - (y2 as i8);
    assert!((dx == 0) != (dy == 0), "one and only one of dx or dy is zero");
    assert!(dx >= -1 && dx <= 1 && dy >= -1 && dy <= 1, "since they are adjacent they must be -1, 0, or 1");

    if mouse_state.last_down_button == 1 {
      // Convert empty cells to belt cells.
      // Create a port between these two cells, but none of the other cells.

      if factory.floor[coord1].kind == CellKind::Empty {
        factory.floor[coord1] = belt_cell(x1, y1, belt_type_to_belt_meta(belt_type1));
      }
      if factory.floor[coord2].kind == CellKind::Empty {
        factory.floor[coord2] = belt_cell(x2, y2, belt_type_to_belt_meta(belt_type2));
      }

      cell_connect_if_possible(options, state, factory, coord1, coord2, dx, dy);
    } else if mouse_state.last_down_button == 2 {
      // Delete the port between the two cells but leave everything else alone.
      // The coords must be adjacent to one side.

      let ( dir1, dir2) = match ( dx, dy ) {
        ( 0 , -1 ) => {
          // y2 was bigger so xy1 is above xy2
          (Direction::Down, Direction::Up)
        }
        ( 1 , 0 ) => {
          // x1 was bigger so xy1 is right of xy2
          (Direction::Left, Direction::Right)
        }
        ( 0 , 1 ) => {
          // x1 was bigger so xy1 is under xy2
          (Direction::Up, Direction::Down)
        }
        ( -1 , 0 ) => {
          // x2 was bigger so xy1 is left of xy2
          (Direction::Right, Direction::Left)
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
      let ((x, y), belt_type, _port_in_dir, _port_out_dir) = track[index];
      log(format!("- track {} at {} {} isa {:?}", index, x, y, belt_type));
      let coord = to_coord(x, y);

      if mouse_state.last_down_button == 1 {
        // Staple the track on top of the existing layout
        match factory.floor[coord].kind {
          CellKind::Belt => {
            // if factory.floor[coord].belt.meta.btype != belt_type {
            //   update_meta_to_belt_type_and_replace_cell(factory, coord, belt_type);
            // }
            // update_ports_of_neighbor_cells(factory, coord, true);
          }
          CellKind::Empty => {
            factory.floor[coord] = belt_cell(x, y, belt_type_to_belt_meta(belt_type));

            // // Force-connect this cell to the previous cell, provided that cell is now a Belt
            // // Do not change existing ports, if any (previously existing belts with those ports)
            // if index > 0 {
            //   floor_create_cell_at_partial(options, state, factory, pcoord, px as i8, py as i8, coord, x as i8, y as i8);
            // }

            // Connect the end points to any existing neighboring cells if not already connected
            if index == 0 || index == len - 1 {
              // log(format!("    -- okay @{} got {:?} ;; {:?} {:?} {:?} {:?}", coord, belt_type, factory.floor[coord].port_u, factory.floor[coord].port_r, factory.floor[coord].port_d, factory.floor[coord].port_l));
              // log(format!("  - connect_belt_to_existing_neighbor_belts(), before: {:?} {:?} {:?} {:?}", factory.floor[coord].port_u, factory.floor[coord].port_r, factory.floor[coord].port_d, factory.floor[coord].port_l));
              connect_belt_to_existing_neighbor_cells(factory, coord);
            }
          }
          CellKind::Machine => {
            // If this is the first one then ignore it
            // Otherwise, if the previous cell is a belt, connect it to this machine
            // if index > 0 {
            //   log(format!("  - machine @{}, index {} connecting to previous cell @{}", coord, index, pcoord));
            //   connect_machine_if_to_belt(factory, coord, x, y,pcoord, px, py);
            // }

            // TODO: do we need this? why not?
            // Connect the end points to any existing neighboring _belt_ cells if not already connected
            // if index == 0 || index == len - 1 {
            //   connect_machine_to_existing_neighbor_belts(factory, coord);
            // }

          }
          _ => (), // Do not overwrite machines, suppliers, or demanders
        }

        if index > 0 {
          // (First element has no inbound)
          cell_connect_if_possible(options, state, factory, pcoord, coord, (px as i8) - (x as i8), (py as i8) - (y as i8));
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

      px = x;
      py = y;
      pcoord = coord;
    }
  }

  // auto_layout(factory); // I don't want to auto-layout. Just auto-port the new unknowns.
  log(format!("Auto porting after modification"));
  keep_auto_porting(options, state,factory);

  // Recreate cell traversal order
  let prio: Vec<usize> = create_prio_list(options, &mut factory.floor);
  log(format!("Updated prio list: {:?}", prio));
  factory.prio = prio;
}
fn on_click_inside_floor(options: &mut Options, state: &mut State, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState, last_mouse_up_cell_x: f64, last_mouse_up_cell_y: f64) {
  if mouse_state.last_down_button == 2 {
    // Clear the cell if that makes sense for it. Delete a belt with one or zero ports.
    let coord = to_coord(last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize);

    let mut ports = 999;
    if factory.floor[coord].kind == CellKind::Belt {
      ports = 0;
      if factory.floor[coord].port_u != Port::None { ports += 1; }
      if factory.floor[coord].port_r != Port::None { ports += 1; }
      if factory.floor[coord].port_d != Port::None { ports += 1; }
      if factory.floor[coord].port_l != Port::None { ports += 1; }
      if ports <= 1 {
        log(format!("Deleting stub @{} after rmb click", coord));
        floor_delete_cell_at_partial(options, state, factory, coord);

        // auto_layout(factory); // I don't want to auto-layout. Just auto-port the new unknowns.
        log(format!("Auto porting after modification"));
        keep_auto_porting(options, state,factory);

        // Recreate cell traversal order
        let prio: Vec<usize> = create_prio_list(options, &mut factory.floor);
        log(format!("Updated prio list: {:?}", prio));
        factory.prio = prio;
      }
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

fn on_click_inside_cell_editor_grid(options: &mut Options, state: &mut State, factory: &mut Factory, cell_selection: &CellSelection, mouse_state: &MouseState) {
  log(format!("TODO: on_click_inside_cell_editor_grid()"));

  // Clicked inside the grid
  // Determine which segment and then rotate that segment
  let click_cell_x = ((mouse_state.last_up_world_x - UI_CELL_EDITOR_GRID_OX) / UI_SEGMENT_W).floor();
  let click_cell_y = ((mouse_state.last_up_world_y - UI_CELL_EDITOR_GRID_OY) / UI_SEGMENT_H).floor();

  log(format!("sxy: {} {}", click_cell_x, click_cell_y));

  let seg = match (click_cell_x as i8, click_cell_y as i8) {
    (1, 0) => Some(Direction::Up),
    (2, 1) => Some(Direction::Right),
    (1, 2) => Some(Direction::Down),
    (0, 1) => Some(Direction::Left),
    _ => None, // ignore center and corners
  };

  if seg != None {
    // Cycle the port on this side
    let old_port = match (click_cell_x as i8, click_cell_y as i8) {
      (1, 0) => factory.floor[cell_selection.coord].port_u,
      (2, 1) => factory.floor[cell_selection.coord].port_r,
      (1, 2) => factory.floor[cell_selection.coord].port_d,
      (0, 1) => factory.floor[cell_selection.coord].port_l,
      _ => panic!("asserted to be valid at this point"),
    };

    let new_port = match old_port {
      Port::Inbound => Port::Outbound,
      Port::Outbound => Port::None,
      Port::None => Port::Inbound,
      Port::Unknown => Port::Unknown,
    };

    factory.floor[cell_selection.coord].port_u = new_port;
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
