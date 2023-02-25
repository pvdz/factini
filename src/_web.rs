// This file should only be included for `wasm-pack build --target web`
// The main.rs will include this file when `#[cfg(target_arch = "wasm32")]`


// Compile with --profile to try and get some sense of shit
// - import/export
//   - import/export with clipboard
// - small problem with tick_belt_take_from_belt when a belt crossing is next to a supply and another belt; it will ignore the other belt as input. because the belt will not let a part proceed to the next port unless it's free and the processing order will process the neighbor belt first and then the crossing so by the time it's free, the part will still be at 50% whereas the supply part is always ready. fix is probably to make supply parts take a tick to be ready, or whatever.
//   - affects machine speed so should be fixed
// - machines
//   - investigate different machine speeds at different configs
//   - allow smaller machines still?
//   - throughput problem. part has to wait at 50% for next part to clear, causing delays. if there's enough outputs there's always room and no such delay. if supply-to-machine is one belt there's also no queueing so it's faster
//   - putting machine down next to two dead end belts will only connect one?
//   - make the menu-machine "process" the finished parts before generating trucks
//   - animate machines at work
//   - paint the prepared parts of a machine while not selected?
// - belts
//   - does snaking bother me when a belt should move all at once or not at all? should we change the algo? probably not that hard to move all connected cells between intersections/entry/exit points at once. if one moves, all move, etc.
//   - a part that reaches 100% of a cell but can't be moved to the side should not block the next part from entering the cell until all ports are taken like that. the part can sit in the port and a belt can only take parts if it has an available port.
// - make sun move across the day bar? in a sort of rainbow path?
// - what's up with these assertion traps :(
//   - `let (received_part_index, received_count) = factory.floor[coord].demand.received[i];` threw oob (1 while len=0). i thin it's somehow related to dropping a demander on the edge
// - hover over craftable offer should highlight craft-inputs (offers)
// - rebalance the fps frame limiter
// - car polish; should make nice corners, should drive same speed to any height
// - click edge to add supplier. click supplier/demander to toggle.
// - updating machine should open machines
// - change preview of craftable when selected (the tiny preview on top is not working
// - create tutorial
// - should machine give hint of creating/missing in/outbound connection?

// https://docs.rs/web-sys/0.3.28/web_sys/struct.CanvasRenderingContext2d.html


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

use std::collections::HashMap;
use std::collections::VecDeque;

use super::asset::*;
use super::belt::*;
use super::belt_type::*;
use super::bouncer::*;
use super::cell::*;
use super::cli_serialize::*;
use super::config::*;
use super::craft::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::init::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::paste::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::quest_state::*;
use super::quote::*;
use super::state::*;
use super::truck::*;
use super::utils::*;
use super::zone::*;

// This explicitly import shoulnd't be necessary anymore according to https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files but ... well I did at the time of writing.
use super::log;

// These are the actual pixels we can paint to
const CANVAS_WIDTH: f64 = 1100.0;
const CANVAS_HEIGHT: f64 = 1100.0;

// Need this for mouse2world coord conversion. Rest of the coords/sizes are in world (canvas) pixels.
const CANVAS_CSS_WIDTH: f64 = 1100.0;
const CANVAS_CSS_HEIGHT: f64 = 1100.0;

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
  pub fn getGameConfig() -> String; // GAME_CONFIG
  pub fn getGameMap() -> String; // GAME_MAP
  pub fn getGameOptions() -> String; // GAME_OPTIONS
  pub fn setGameOptions(str: JsValue, on_load: JsValue); // GAME_OPTIONS
  pub fn getExamples() -> js_sys::Array; // GAME_EXAMPLES, array of string
  pub fn getAction() -> String; // queuedAction, polled every frame
  pub fn receiveConfigNode(name: JsValue, node: JsValue);
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

fn dnow() -> u64 {
  js_sys::Date::now() as u64
}

fn load_tile(src: &str) -> Result<web_sys::HtmlImageElement, JsValue> {
  let document = document();

  let img = document
    .create_element("img")?
    .dyn_into::<web_sys::HtmlImageElement>()?;

  img.set_src(src);

  // // let body = document.body().expect("body should exist");
  // let div = document.get_element_by_id("$tdb").unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();
  // div.append_child(&img).expect("to work");

  return Ok(img);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  // Must run this once in web-mode to enable dumping panics to console.log
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  // console_error_panic_hook::set_once();

  log!("web start...");
  let document = document();
  let canvas = document
    .create_element("canvas")?
    .dyn_into::<web_sys::HtmlCanvasElement>()?;
  document.get_element_by_id("$main_game").unwrap().append_child(&canvas)?;
  canvas.set_id("$main_game_canvas");
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

  pub fn load_config(print_fmd_trace: bool, config_str: String) -> Config {
    let mut config = parse_fmd(print_fmd_trace, config_str);

    if print_fmd_trace {
      config.nodes.iter().for_each(|node| {
        if node.sprite_config.frames[0].file != "" {
          log!("node `{}` wants to load `{}` at canvas index {}", node.raw_name, node.sprite_config.frames[0].file, node.sprite_config.frames[0].file_canvas_cache_index);
        }
      });
    }

    // Load sprite maps. Once per image.
    config.sprite_cache_canvas = config.sprite_cache_order.iter().enumerate().map(|(_index, src)| {
      // log!("Canvas {} src {}", _index, src);
      return load_tile(src.clone().as_str()).expect("worky worky");
    }).collect();
    if print_fmd_trace { log!("Loading {} sprite files for the parts: {:?}", config.sprite_cache_canvas.len(), config.sprite_cache_lookup); }
    else { log!("Loading {} sprite files (enable print_fmd_trace for details)", config.sprite_cache_canvas.len()); }

    {
      let kinds: JsValue = [ConfigNodeKind::Part, ConfigNodeKind::Quest, ConfigNodeKind::Supply, ConfigNodeKind::Demand, ConfigNodeKind::Dock, ConfigNodeKind::Machine, ConfigNodeKind::Belt].iter().map(|&kind| {
        return JsValue::from(match kind {
          ConfigNodeKind::Asset => "Asset",
          ConfigNodeKind::Part => "Part",
          ConfigNodeKind::Quest => "Quest",
          ConfigNodeKind::Supply => "Supply",
          ConfigNodeKind::Demand => "Demand",
          ConfigNodeKind::Dock => "Dock",
          ConfigNodeKind::Machine => "Machine",
          ConfigNodeKind::Belt => "Belt",
        });
      }).collect::<js_sys::Array>().into();

      let nodes: JsValue = config_to_jsvalue(&config);

      receiveConfigNode("wat".into(), vec!(
        vec!(JsValue::from("kinds"), kinds).iter().collect::<js_sys::Array>(),
        vec!(JsValue::from("nodes"), nodes).iter().collect::<js_sys::Array>(),
      ).iter().collect::<js_sys::Array>().into());
    }

    return config;
  }

  // Load game "level" and part content config dynamic so we don't have to recompile it for
  // ingame changes relating to parts and unlock order of them. This config includes sprite details.
  let def_options = create_options(0.0, 0.0);
  let mut config = load_config(def_options.print_fmd_trace, getGameConfig());

  let img_machine1: web_sys::HtmlImageElement = load_tile("./img/machine1.png")?;
  let img_machine2: web_sys::HtmlImageElement = load_tile("./img/machine2.png")?;
  let img_machine3: web_sys::HtmlImageElement = load_tile("./img/machine3.png")?;
  let img_machine4: web_sys::HtmlImageElement = load_tile("./img/machine4.png")?;
  let img_machine_1_1: web_sys::HtmlImageElement = load_tile("./img/machine_1_1.png")?;
  let img_machine_2_1: web_sys::HtmlImageElement = load_tile("./img/machine_2_2.png")?;
  let img_machine_3_2: web_sys::HtmlImageElement = load_tile("./img/machine_3_2.png")?;
  let img_dumptruck: web_sys::HtmlImageElement = load_tile("./img/dumptruck.png")?;
  let img_loading_sand: web_sys::HtmlImageElement = load_tile("./img/sand.png")?;
  let img_help_black: web_sys::HtmlImageElement = load_tile("./img/help.png")?;
  let img_help_red: web_sys::HtmlImageElement = load_tile("./img/help_red.png")?;
  let img_manual: web_sys::HtmlImageElement = load_tile("./img/manual.png")?;
  let img_lmb: web_sys::HtmlImageElement = load_tile("./img/lmb.png")?;
  let img_rmb: web_sys::HtmlImageElement = load_tile("./img/rmb.png")?;
  let img_save: web_sys::HtmlImageElement = load_tile("./img/save.png")?;
  let img_quest_frame: web_sys::HtmlImageElement = load_tile("./img/quest_frame.png")?;

  // https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern
  // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.CanvasRenderingContext2d.html#method.create_pattern_with_html_image_element
  // let ptrn_dock1 = context.create_pattern_with_html_image_element(&img_loading_dock, "repeat").expect("trying to load dock1 tile");

  // Tbh this whole Rc approach is copied from the original template. It works so why not, :shrug:
  let mouse_x = Rc::new(Cell::new(0.0));
  let mouse_y = Rc::new(Cell::new(0.0));
  let mouse_moved = Rc::new(Cell::new(false));
  let last_down_event_type = Rc::new(Cell::new(EventSourceType::Mouse)); // Was the last "down" event a MOUSE or TOUCH event?
  let last_mouse_was_down = Rc::new(Cell::new(false));
  let last_mouse_down_x = Rc::new(Cell::new(0.0));
  let last_mouse_down_y = Rc::new(Cell::new(0.0));
  let last_mouse_down_button = Rc::new(Cell::new(0));
  let last_mouse_was_up = Rc::new(Cell::new(false));
  let last_mouse_up_x = Rc::new(Cell::new(0.0));
  let last_mouse_up_y = Rc::new(Cell::new(0.0));
  let last_mouse_up_button = Rc::new(Cell::new(0));
  let counted = Rc::new(canvas);

  // mousedown
  {
    let last_down_event_type = last_down_event_type.clone();
    let mouse_x = mouse_x.clone();
    let mouse_y = mouse_y.clone();
    let last_mouse_was_down = last_mouse_was_down.clone();
    let last_mouse_down_x = last_mouse_down_x.clone();
    let last_mouse_down_y = last_mouse_down_y.clone();
    let last_mouse_down_button = last_mouse_down_button.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
      event.stop_propagation();
      event.prevent_default();

      log!("mouse down: button: {:?}", event.buttons());

      last_down_event_type.set(EventSourceType::Mouse);

      let mx = event.offset_x() as f64;
      let my = event.offset_y() as f64;

      last_mouse_was_down.set(true);
      mouse_x.set(mx);
      mouse_y.set(my);
      last_mouse_down_x.set(mx);
      last_mouse_down_y.set(my);
      last_mouse_down_button.set(event.buttons()); // 1=left, 2=right, 3=left-then-also-right (but right-then-also-left is still 2)
    }) as Box<dyn FnMut(_)>);
    counted.clone().add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  // mousemove
  {
    let mouse_x = mouse_x.clone();
    let mouse_y = mouse_y.clone();
    let mouse_moved = mouse_moved.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
      event.stop_propagation();
      event.prevent_default();

      // log!("mouse move: button: {:?}", event.buttons());

      let mx = event.offset_x() as f64;
      let my = event.offset_y() as f64;

      mouse_x.set(mx);
      mouse_y.set(my);
      mouse_moved.set(true);
    }) as Box<dyn FnMut(_)>);
    counted.clone().add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  // mouseup
  {
    let last_mouse_was_up = last_mouse_was_up.clone();
    let last_mouse_up_x = last_mouse_up_x.clone();
    let last_mouse_up_y = last_mouse_up_y.clone();
    let last_mouse_up_button = last_mouse_up_button.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
      event.stop_propagation();
      event.prevent_default();

      log!("mouse up: button: {:?}", event.buttons());

      let mx = event.offset_x() as f64;
      let my = event.offset_y() as f64;

      last_mouse_was_up.set(true);
      last_mouse_up_x.set(mx);
      last_mouse_up_y.set(my);
      last_mouse_up_button.set(event.buttons()); // 1=left, 2=right, 3=left-then-also-right (but right-then-also-left is still 2)
    }) as Box<dyn FnMut(_)>);
    counted.clone().add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  // context menu (just to disable it so we can use rmb for interaction)
  {
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
      event.stop_propagation();
      event.prevent_default();
    }) as Box<dyn FnMut(_)>);
    counted.clone().add_event_listener_with_callback("contextmenu", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  // touchdown
  {
    let last_down_event_type = last_down_event_type.clone();
    let canvas = counted.clone();
    let mouse_x = mouse_x.clone();
    let mouse_y = mouse_y.clone();
    let last_mouse_was_down = last_mouse_was_down.clone();
    let last_mouse_down_x = last_mouse_down_x.clone();
    let last_mouse_down_y = last_mouse_down_y.clone();
    let last_mouse_down_button = last_mouse_down_button.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
      event.stop_propagation();
      event.prevent_default();

      log!("touch start: number of touches: {}", event.changed_touches().length());

      last_down_event_type.set(EventSourceType::Touch);

      let bound = canvas.get_bounding_client_rect();
      let event = event.touches().get(0).unwrap();

      let mx = -bound.left() + event.client_x() as f64;
      let my = -bound.top() + event.client_y() as f64;

      last_mouse_was_down.set(true);
      mouse_x.set(mx);
      mouse_y.set(my);
      last_mouse_down_x.set(mx);
      last_mouse_down_y.set(my);
      last_mouse_down_button.set(1); // 1=left, 2=right, 3=left-then-also-right (but right-then-also-left is still 2). touch is always 1.
    }) as Box<dyn FnMut(_)>);
    counted.clone().add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  // touchmove
  {
    let canvas = counted.clone();
    let mouse_x = mouse_x.clone();
    let mouse_y = mouse_y.clone();
    let mouse_moved = mouse_moved.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
      event.stop_propagation();
      event.prevent_default();

      // log!("touch move: number of touches: {}", event.changed_touches().length());

      let bound = canvas.get_bounding_client_rect();
      let event = event.touches().get(0).unwrap();

      let mx = -bound.left() + event.client_x() as f64;
      let my = -bound.top() + event.client_y() as f64;

      mouse_x.set(mx);
      mouse_y.set(my);
      mouse_moved.set(true);
    }) as Box<dyn FnMut(_)>);
    counted.clone().add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  // touchend
  {
    let canvas = counted.clone();
    let mouse_x = mouse_x.clone();
    let mouse_y = mouse_y.clone();
    let last_mouse_was_up = last_mouse_was_up.clone();
    let last_mouse_up_x = last_mouse_up_x.clone();
    let last_mouse_up_y = last_mouse_up_y.clone();
    let last_mouse_up_button = last_mouse_up_button.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
      event.stop_propagation();
      event.prevent_default();

      log!("touch end: number of touches: {}", event.changed_touches().length());
      let bound = canvas.get_bounding_client_rect();
      let event = event.changed_touches().get(0).unwrap();

      let mx = -bound.left() + event.client_x() as f64;
      let my = -bound.top() + event.client_y() as f64;

      mouse_x.set(mx);
      mouse_y.set(my);
      last_mouse_was_up.set(true);
      last_mouse_up_x.set(mx);
      last_mouse_up_y.set(my);
      last_mouse_up_button.set(1); // 1=left, 2=right, 3=left-then-also-right (but right-then-also-left is still 2)
    }) as Box<dyn FnMut(_)>);
    counted.clone().add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }

  // TODO: we could load the default map first (or put it under the stack) but I don't think we want that..?
  let initial_map_from_source;
  let initial_map = {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let last_map = local_storage.get_item("factini.lastMap").unwrap();
    match last_map {
      Some(last_map) => {
        log!("Init map: Loading last known map from local storage...");
        initial_map_from_source = false;
        last_map
      },
      None => {
        log!("Init map: Loading default map from source...");
        initial_map_from_source = true;
        getGameMap()
      },
    }
  };
  let initial_map_from_source = if initial_map_from_source { 0 } else { initial_map.len() as u64 };
  let ( mut options, mut state, mut factory ) = init(&config, initial_map);
  let mut saves: [Option<(web_sys::HtmlCanvasElement, String)>; 9] = [(); 9].map(|_| None);
  fn create_placeholder_canvas(document: &web_sys::Document, img: &web_sys::HtmlImageElement) -> web_sys::HtmlCanvasElement{
    // Pre-load snapshots from localStorage, if any were found
    let placeholder_canvas = document.create_element("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    placeholder_canvas.set_width(100);
    placeholder_canvas.set_height(100);
    let context = placeholder_canvas.get_context("2d").expect("get context must work").unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    context.set_fill_style(&"#ccc".into());
    context.fill_rect(0.0, 0.0, 100.0, 100.0);
    context.draw_image_with_html_image_element_and_dw_and_dh(&img, 0.0, 0.0, 100.0, 100.0).expect("placeholder icon");






    // TODO: create actual snapshot here rather than a blank page. kinda tricky I guess.





    return placeholder_canvas;
  }

  let ( saved_map1, saved_map2, saved_map3, saved_map4 ) = {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    (
      local_storage.get_item("factini.save0").unwrap(),
      local_storage.get_item("factini.save1").unwrap(),
      local_storage.get_item("factini.save2").unwrap(),
      local_storage.get_item("factini.save3").unwrap(),
    )
  };
  if let Some(saved_map) = saved_map1 { saves[0] = Some(( create_placeholder_canvas(&document, &img_lmb), saved_map )); }
  if let Some(saved_map) = saved_map2 { saves[1] = Some(( create_placeholder_canvas(&document, &img_lmb), saved_map )); }
  if let Some(saved_map) = saved_map3 { saves[2] = Some(( create_placeholder_canvas(&document, &img_lmb), saved_map )); }
  if let Some(saved_map) = saved_map4 { saves[3] = Some(( create_placeholder_canvas(&document, &img_lmb), saved_map )); }

  let saved_options = {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage.get_item("factini.options").unwrap()
  };
  let ( option_string, options_started_from_source ) = match saved_options {
    Some(str) => {
      log!("Using options json from localStorage ({} bytes)", str.len());
      ( str, false )
    },
    None => ( getGameOptions(), true ),
  };
  let options_started_from_source = if options_started_from_source { 0 } else { option_string.len() as u64 };
  options.initial_map_from_source = initial_map_from_source;
  parse_and_save_options_string(option_string, &mut options, true, options_started_from_source, true);
  state.event_type_swapped = options.initial_event_type_swapped;
  state_add_examples(getExamples(), &mut state);

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
    log!("start time: {}", start_time);

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
      moved_since_start: false,

      cell_x: 0.0,
      cell_y: 0.0,
      cell_x_floored: 0.0,
      cell_y_floored: 0.0,

      last_cell_x: 0.0,
      last_cell_y: 0.0,

      over_zone: Zone::None,
      down_zone: Zone::None,
      up_zone: Zone::None,

      last_down_event_type: EventSourceType::Unknown,

      is_down: false,
      was_down: false,
      is_dragging: false,
      is_drag_start: false,

      over_day_bar: false,

      over_floor_area: false,
      over_floor_not_corner: false,
      down_floor_area: false,
      down_floor_not_corner: false,

      over_quest: false,
      over_quest_visible_index: 0, // Only if over_quote
      down_quest: false,
      down_quest_visible_index: 0, // Only if down_quest
      up_quote: false,
      up_quest_visible_index: 0, // Only if up_quote

      over_menu_row: MenuRow::None,
      over_menu_button: MenuButton::None,
      down_menu_row: MenuRow::None,
      down_menu_button: MenuButton::None,
      up_menu_row: MenuRow::None,
      up_menu_button: MenuButton::None,

      help_hover: false,
      help_down: false,

      offer_down: false,
      offer_down_offer_index: 0,
      offer_hover: false,
      offer_hover_offer_index: 0,
      offer_selected: false,
      offer_selected_index: 0, // Offer index, not part index
      dragging_offer: false,
      dragging_machine: false,

      craft_over_ci: CraftInteractable::None,
      craft_over_ci_wx: 0.0,
      craft_over_ci_wy: 0.0,
      craft_over_ci_ww: 0.0,
      craft_over_ci_wh: 0.0,
      craft_over_ci_icon: '#',
      craft_over_ci_index: 99, // <99 means circle button index. >99 means machine cell index -100.
      craft_over_ci_part_kind: PARTKIND_NONE,
      craft_down_ci: CraftInteractable::None,
      craft_down_ci_wx: 0.0,
      craft_down_ci_wy: 0.0,
      craft_down_ci_ww: 0.0,
      craft_down_ci_wh: 0.0,
      craft_down_ci_icon: '#',
      craft_down_ci_part_kind: PARTKIND_NONE,
      craft_down_ci_index: 99, // <99 means circle button index. >99 means machine cell index -100.
      craft_up_ci: CraftInteractable::None,
      craft_up_ci_wx: 0.0,
      craft_up_ci_wy: 0.0,
      craft_up_ci_ww: 0.0,
      craft_up_ci_wh: 0.0,
      craft_up_ci_icon: '#',
      craft_up_ci_index: 99, // <99 means circle button index. >99 means machine cell index -100.
      craft_up_ci_part_kind: PARTKIND_NONE,
      craft_dragging_ci: false,

      was_dragging: false,
      is_up: false,
      was_up: false,

      last_down_button: 0,

      last_down_canvas_x: 0.0,
      last_down_canvas_y: 0.0,
      last_down_world_x: 0.0,
      last_down_world_y: 0.0,
      last_down_cell_x: 0.0,
      last_down_cell_y: 0.0,
      last_down_cell_x_floored: 0.0,
      last_down_cell_y_floored: 0.0,

      last_up_canvas_x: 0.0,
      last_up_canvas_y: 0.0,
      last_up_world_x: 0.0,
      last_up_world_y: 0.0,
      last_up_cell_x: 0.0,
      last_up_cell_y: 0.0,

      over_save_map: false,
      over_save_map_index: 0,
      over_undo: false,
      over_clear: false,
      over_redo: false,
      down_save_map: false,
      down_save_map_index: 0,
      down_undo: false,
      down_clear: false,
      down_redo: false,
      up_save_map: false,
      up_save_map_index: 0,
      up_undo: false,
      up_clear: false,
      up_redo: false,
    };

    // From https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
      // This is the raF frame callback:

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
      let ticks_per_second_wanted = ONE_SECOND as f64 * options.speed_modifier_floor;
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

      if state.load_example_next_frame {
        state.load_example_next_frame = false;
        let map = state.examples[state.example_pointer % state.examples.len()].clone();
        log!("Loading example[{}]; size: {} bytes", state.example_pointer, map.len());
        factory_load_map(&mut options, &mut state, &config, &mut factory, map);
        state.example_pointer += 1;
      }
      if state.reset_next_frame {
        state.reset_next_frame = false;
        let map = getGameMap();
        log!("Loading getGameMap(); size: {} bytes", map.len());
        factory_load_map(&mut options, &mut state, &config, &mut factory, map);
      }
      if state.load_snapshot_next_frame {
        // Note: state.load_snapshot_next_frame remains true because factory.changed has special undo-stack behavior for it
        let map = state.snapshot_stack[state.snapshot_undo_pointer % UNDO_STACK_SIZE].clone();
        log!("Loading snapshot[{} / {}]; size: {} bytes", state.snapshot_undo_pointer, state.snapshot_pointer, map.len());
        factory_load_map(&mut options, &mut state, &config, &mut factory, map);
      }

      if !state.paused  {
        for _ in 0..ticks_todo.min(MAX_TICKS_PER_FRAME) {
          tick_factory(&mut options, &mut state, &config, &mut factory);
        }
      }

      let queued_action = getAction();
      if queued_action != "" { log!("getAction() had `{}`", queued_action); }
      let options_started_from_source = options.options_started_from_source; // Don't change this value here.
      match queued_action.as_str() {
        "apply_options" => parse_and_save_options_string(getGameOptions(), &mut options, false, options_started_from_source, false),
        "load_map" => state.reset_next_frame = true, // implicitly will call getGameMap() which loads the map from UI indirectly
        "load_config" => config = load_config(options.print_fmd_trace, getGameConfig()), // Might crash the game
        "" => {},
        _ => panic!("getAction() returned an unsupported value: `{}`", queued_action),
      }

      if options.web_output_cli {
        paint_world_cli(&context, &mut options, &mut state, &factory);
      } else {
        let was_down = last_mouse_was_down.get();
        let was_mouse = if was_down { if last_down_event_type.get() == EventSourceType::Mouse { EventSourceType::Mouse } else { EventSourceType::Touch } } else { EventSourceType::Unknown }; // Only read if set. May be an over-optimization but eh.
        update_mouse_state(&mut options, &mut state, &config, &mut factory, &mut cell_selection, &mut mouse_state, mouse_x.get(), mouse_y.get(), mouse_moved.get(), was_mouse, was_down, last_mouse_down_x.get(), last_mouse_down_y.get(), last_mouse_down_button.get(), last_mouse_was_up.get(), last_mouse_up_x.get(), last_mouse_up_y.get(), last_mouse_up_button.get());
        last_mouse_was_down.set(false);
        last_mouse_was_up.set(false);

        // Handle drag-end or click
        handle_input(&mut cell_selection, &mut mouse_state, &mut options, &mut state, &config, &mut factory, &mut saves);

        if factory.changed {
          // If currently looking at a historic snapshot, then now copy that
          // snapshot to the front of the stack before adding a new state to it
          if !state.load_snapshot_next_frame && state.snapshot_pointer != state.snapshot_undo_pointer {
            let snap = state.snapshot_stack[state.snapshot_undo_pointer].clone();
            log!("Pushing current undo snapshot to the front of the stack; size: {} bytes, undo pointer: {}, pointer: {}", snap.len(), state.snapshot_undo_pointer, state.snapshot_pointer);
            state.snapshot_pointer += 1;
            state.snapshot_undo_pointer = state.snapshot_pointer;
            state.snapshot_stack[state.snapshot_pointer % UNDO_STACK_SIZE] = snap;
          }

          log!("Auto porting after modification");
          keep_auto_porting(&mut options, &mut state, &mut factory);
          fix_ins_and_outs_for_all_belts_and_machines(&mut factory);

          // Recreate cell traversal order
          let prio: Vec<usize> = create_prio_list(&mut options, &config, &mut factory.floor);
          log!("Updated prio list: {:?}", prio);
          factory.prio = prio;

          if !state.load_snapshot_next_frame {
            // Create snapshot in history, except for unredo
            let snap = generate_floor_dump(&options, &state, &config, &factory, dnow()).join("\n");
            // log!("Snapshot:\n{}", snap);
            log!("Pushed snapshot to the front of the stack; size: {} bytes, undo pointer: {}, pointer: {}", snap.len(), state.snapshot_undo_pointer, state.snapshot_pointer);

            state.snapshot_pointer += 1;
            state.snapshot_undo_pointer = state.snapshot_pointer;
            state.snapshot_stack[state.snapshot_pointer % UNDO_STACK_SIZE] = snap;
          }

          factory.modified_at = factory.ticks;
          if factory.last_day_start == 0 {
            factory.last_day_start = factory.ticks;
            factory.finished_at = 0;
            factory.finished_with = 0;
          }
          factory.changed = false;
          factory.accepted = 0;
          factory.produced = 0;
          factory.trashed = 0;
          factory.supplied = 0;
          state.load_snapshot_next_frame = false;

          // Dump current map to debug UI
          let game_map = document.get_element_by_id("$game_map").unwrap();
          game_map.dyn_into::<web_sys::HtmlTextAreaElement>().unwrap().set_value(state.snapshot_stack[state.snapshot_undo_pointer % UNDO_STACK_SIZE].as_str());

          let last_map = state.snapshot_stack[state.snapshot_undo_pointer % UNDO_STACK_SIZE].as_str();
          let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
          local_storage.set_item("factini.lastMap", last_map).unwrap();
          log!("Stored last map to local storage ({} bytes)", last_map.len());

          // This works but it's not nice. Maybe it should be an option :shrug:
          if options.game_auto_reset_day {
            day_reset(&mut options, &mut state, &config, &mut factory);
          }
        }

        // Paint the world (we should not do input or world mutations after this point)

        context.set_font(&"12px monospace");

        // Clear canvas
        context.set_fill_style(&"#E86A17".into());
        // context.set_fill_style(&"lightblue".into());
        context.fill_rect(0.0, 0.0, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);

        // Global background
        if let Some(ptrn_sand) = context.create_pattern_with_html_image_element(&img_loading_sand, "repeat").expect("trying to load sand ztile") {
          context.set_fill_style(&ptrn_sand);
          context.fill_rect(0.0, 0.0, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);
        } else {
          context.set_stroke_style(&"#aaa".into());
          context.stroke_rect(UI_FLOOR_OFFSET_X, UI_FLOOR_OFFSET_Y, FLOOR_CELLS_W as f64 * CELL_W, FLOOR_CELLS_H as f64 * CELL_H);
        }
        // Put a semi-transparent layer over the inner floor part to make it darker
        // context.set_fill_style(&"#00000077".into());
        context.set_fill_style(&"#ffcf8e".into());
        context.fill_rect(UI_FLOOR_OFFSET_X + CELL_W, UI_FLOOR_OFFSET_Y + CELL_H, (FLOOR_CELLS_W - 2) as f64 * CELL_W, (FLOOR_CELLS_H - 2) as f64 * CELL_H);

        paint_zone_hovers(&options, &state, &context, &mouse_state);
        // paint_top_stats(&context, &mut factory);
        paint_corner_help_icon(&options, &state, &mut factory, &context, if mouse_state.help_hover { &img_help_red } else { &img_help_black});
        paint_top_bars(&options, &state, &mut factory, &context, &mouse_state);
        paint_quests(&options, &state, &config, &context, &factory, &mouse_state, &img_quest_frame);
        paint_ui_offers(&options, &state, &config, &context, &factory, &mouse_state, &cell_selection);
        paint_lasers(&options, &mut state, &config, &context);
        paint_trucks(&options, &state, &config, &context, &mut factory, &img_dumptruck);
        paint_bottom_menu(&options, &state, &context, &img_machine_1_1, &mouse_state);
        // TODO: wait for tiles to be loaded because first few frames won't paint anything while the tiles are loading...
        paint_background_tiles1(&options, &state, &config, &context, &factory, &img_machine4, &img_machine_1_1, &img_machine_2_1, &img_machine_3_2);
        paint_background_tiles2(&options, &state, &config, &context, &factory, &img_machine4, &img_machine_1_1, &img_machine_2_1, &img_machine_3_2);
        paint_background_tiles3(&options, &state, &config, &context, &factory, &img_machine4, &img_machine_1_1, &img_machine_2_1, &img_machine_3_2);
        paint_port_arrows(&options, &state, &config, &context, &factory);
        paint_belt_dbg_id(&options, &state, &config, &context, &factory);
        // paint_belt_items(&options, &state, &config, &context, &factory);
        paint_machine_craft_menu(&options, &state, &config, &context, &factory, &cell_selection, &mouse_state);
        paint_ui_offer_hover_droptarget_hint_conditionally(&options, &state, &config, &context, &mut factory, &mouse_state, &cell_selection);
        paint_debug_app(&options, &state, &context, &fps, real_world_ms_at_start_of_curr_frame, real_world_ms_since_start_of_prev_frame, ticks_todo, estimated_fps, rounded_fps, &factory, &mouse_state);
        paint_debug_selected_belt_cell(&context, &factory, &cell_selection, &mouse_state);
        paint_debug_selected_machine_cell(&context, &factory, &cell_selection, &mouse_state);
        paint_debug_selected_supply_cell(&context, &factory, &cell_selection, &mouse_state);
        paint_debug_selected_demand_cell(&context, &factory, &cell_selection, &mouse_state);
        paint_map_state_buttons(&state, &context, &mouse_state, &img_save);
        paint_load_thumbs(&options, &state, &config, &context, &mouse_state, &saves, &img_save);

        // Probably after all backround/floor stuff is finished
        paint_zone_borders(&options, &state, &context);

        // Over all the UI stuff
        paint_mouse_cursor(&context, &mouse_state);

        // In front of all game stuff
        paint_bouncers(&options, &state, &config, &context, &mut factory);
        // When dragging make sure that stays on top of bouncers
        paint_mouse_action(&options, &state, &config, &factory, &context, &mouse_state, &cell_selection);

        // In front of everything else
        paint_manual(&options, &state, &context, &img_manual);
      }

      // Schedule next frame
      request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
  }

  Ok(())
}

fn get_x_while_dragging_offer_machine(cell_x: f64, offer_width: usize) -> f64 {
  // Abstracted this to make sure the preview and actual action use the same computation
  let compx = if offer_width % 2 == 1 { 0.0 } else { 0.5 };
  let ox = (cell_x + compx).floor() - (offer_width / 2) as f64;
  return ox;
}
fn world_y_to_top_left_cell_y_while_dragging_offer_machine(cell_y: f64, offer_height: usize) -> f64 {
  let compy = if offer_height % 2 == 1 { 0.0 } else { 0.5 };
  let oy = (cell_y + compy).floor() - (offer_height / 2) as f64;
  return oy;
}

fn update_mouse_state(
  options: &Options, state: &State, config: &Config, factory: &Factory,
  cell_selection: &mut CellSelection, mouse_state: &mut MouseState,
  mouse_x: f64, mouse_y: f64, mouse_moved_since_app_start: bool,
  last_down_event_type: EventSourceType, // MOUSE or TOUCH event
  last_mouse_was_down: bool, last_mouse_down_x: f64, last_mouse_down_y: f64, last_mouse_down_button: u16,
  last_mouse_was_up: bool, last_mouse_up_x: f64, last_mouse_up_y: f64, last_mouse_up_button: u16,
) {
  // Note: event handlers should not be called from here. This should only update mouse_state.
  //       this is why only the mouse_state is mutable.

  // Compensate input coord for the finger while touch dragging
  let (mouse_x, mouse_y, last_mouse_up_x, last_mouse_up_y) =
    if options.touch_drag_compensation && mouse_state.is_dragging && !mouse_state.is_up {
      (mouse_x - 50.0, mouse_y - 50.0, last_mouse_up_x - 50.0, last_mouse_up_y - 50.0)
    } else {
      (mouse_x, mouse_y, last_mouse_up_x, last_mouse_up_y)
    };

  // Reset
  mouse_state.moved_since_start = mouse_moved_since_app_start;
  mouse_state.is_drag_start = false;
  if mouse_state.is_up { // Note: this was the state in the previous frame
    mouse_state.down_zone = Zone::None;
    mouse_state.craft_down_ci = CraftInteractable::None;
    mouse_state.craft_dragging_ci = false;
    mouse_state.offer_down = false;
    mouse_state.help_down = false;
    mouse_state.is_down = false;
    mouse_state.down_floor_not_corner = false;
    mouse_state.down_menu_button = MenuButton::None;
    mouse_state.up_menu_button = MenuButton::None;
    mouse_state.dragging_offer = false;
    mouse_state.dragging_machine = false;
    mouse_state.down_quest = false;
    mouse_state.up_quote = false;
    mouse_state.down_save_map = false;
    mouse_state.up_save_map = false;
    mouse_state.down_undo = false;
    mouse_state.down_clear = false;
    mouse_state.down_redo = false;
    mouse_state.up_undo = false;
    mouse_state.up_clear = false;
    mouse_state.up_redo = false;
  }
  mouse_state.was_down = false;
  mouse_state.is_up = false;
  mouse_state.was_up = false;
  mouse_state.was_dragging = false;
  mouse_state.offer_hover = false;
  mouse_state.over_quest = false;
  mouse_state.over_menu_button = MenuButton::None;
  mouse_state.help_hover = false;
  mouse_state.over_day_bar = false;
  mouse_state.over_save_map = false;
  mouse_state.over_undo = false;
  mouse_state.over_clear = false;
  mouse_state.over_redo = false;

  mouse_state.up_zone = Zone::None;
  mouse_state.over_zone = Zone::None;

  mouse_state.over_floor_not_corner = false;
  mouse_state.craft_over_ci = CraftInteractable::None;
  mouse_state.craft_up_ci = CraftInteractable::None;

  // Mouse coords
  // Note: mouse2world coord is determined by _css_ size, not _canvas_ size
  mouse_state.canvas_x = mouse_x; // Where your mouse actually is on your screen / in your browser
  mouse_state.canvas_y = mouse_y;
  mouse_state.world_x = mouse_x / CANVAS_CSS_WIDTH * CANVAS_WIDTH;
  mouse_state.world_y = mouse_y / CANVAS_CSS_HEIGHT * CANVAS_HEIGHT;
  mouse_state.cell_x = (mouse_x - UI_FLOOR_OFFSET_X) / CELL_W;
  mouse_state.cell_y = (mouse_y - UI_FLOOR_OFFSET_Y) / CELL_H;
  mouse_state.cell_x_floored = mouse_state.cell_x.floor();
  mouse_state.cell_y_floored = mouse_state.cell_y.floor();

  let is_machine_selected = cell_selection.on && factory.floor[cell_selection.coord].kind == CellKind::Machine;

  mouse_state.over_zone = coord_to_zone(options, state, config, mouse_state.world_x, mouse_state.world_y, is_machine_selected, factory, cell_selection.coord);
  match mouse_state.over_zone {
    Zone::None => panic!("cant be over on no zone"),
    ZONE_MANUAL => {}
    ZONE_CRAFT => {
      if options.enable_craft_menu_interact {
        if !mouse_state.is_dragging {
          let ( what, wx, wy, ww, wh, icon, part_index, craft_index) = hit_test_get_craft_interactable_machine_at(options, state, factory, cell_selection, mouse_state.world_x, mouse_state.world_y);
          mouse_state.craft_over_ci = what;
          mouse_state.craft_over_ci_wx = wx;
          mouse_state.craft_over_ci_wy = wy;
          mouse_state.craft_over_ci_wx = ww;
          mouse_state.craft_over_ci_wy = wh;
          mouse_state.craft_over_ci_icon = icon;
          mouse_state.craft_over_ci_part_kind = part_index;
          mouse_state.craft_over_ci_index = craft_index;
        }
      } else {
        log!("Mouse over on craft is ignored because options.enable_craft_menu_interact = false")
      }
    }
    ZONE_HELP => {
      if hit_test_help_button(mouse_state.world_x, mouse_state.world_y) {
        mouse_state.help_hover = true;
      }
    }
    ZONE_QUOTES => {
      if hit_test_undo(mouse_state.world_x, mouse_state.world_y) {
        mouse_state.over_undo = true;
      }
      else if hit_test_clear(mouse_state.world_x, mouse_state.world_y) {
        mouse_state.over_clear = true;
      }
      else if hit_test_redo(mouse_state.world_x, mouse_state.world_y) {
        mouse_state.over_redo = true;
      }
      else {
        mouse_state.over_quest =
          mouse_state.world_x >= UI_QUOTES_OFFSET_X + UI_QUOTE_X && mouse_state.world_x < UI_QUOTES_OFFSET_X + UI_QUOTE_X + UI_QUEST_WIDTH &&
          (mouse_state.world_y - (UI_QUOTES_OFFSET_Y + UI_QUOTE_Y)) % (UI_QUEST_HEIGHT + UI_QUEST_MARGIN) < UI_QUEST_HEIGHT;
        mouse_state.over_quest_visible_index = if mouse_state.over_quest { ((mouse_state.world_y - (UI_QUOTES_OFFSET_Y + UI_QUOTE_Y)) / (UI_QUEST_HEIGHT + UI_QUEST_MARGIN)) as usize } else { 0 };
      }
    }
    ZONE_SAVE_MAP => {
      let button_index = hit_test_save_map(mouse_state.world_x, mouse_state.world_y);
      if button_index == 100 { return; } // Not up on a button
      mouse_state.over_save_map = true;
      mouse_state.over_save_map_index = button_index;
    }
    Zone::BottomBottomLeft => {}
    ZONE_DAY_BAR => {
      mouse_state.over_day_bar = bounds_check(mouse_state.world_x, mouse_state.world_y, UI_DAY_PROGRESS_OFFSET_X, UI_DAY_PROGRESS_OFFSET_Y, UI_DAY_PROGRESS_OFFSET_X + UI_DAY_PROGRESS_WIDTH, UI_DAY_PROGRESS_OFFSET_Y + UI_DAY_PROGRESS_HEIGHT);
    }
    ZONE_FLOOR => {
      mouse_state.over_floor_area = true;
      mouse_state.over_floor_not_corner =
        // Over floor cells
        mouse_state.cell_x >= 0.0 && mouse_state.cell_x < (FLOOR_CELLS_W as f64) && mouse_state.cell_y >= 0.0 && mouse_state.cell_y < (FLOOR_CELLS_H as f64) &&
        // Not corner
        !((mouse_state.cell_x_floored == 0.0 || mouse_state.cell_x_floored == (FLOOR_CELLS_W - 1) as f64) && (mouse_state.cell_y_floored == 0.0 || mouse_state.cell_y_floored == (FLOOR_CELLS_H - 1) as f64));
    }
    ZONE_MENU => {
      let ( menu_button, button_row ) = hit_test_menu_button(mouse_state.world_x, mouse_state.world_y);
      if menu_button != MenuButton::None {
        // time controls, first, second row of menu buttons
        mouse_state.over_menu_row = button_row;
        mouse_state.over_menu_button = menu_button;
      }
      else if hit_test_paint_toggle(mouse_state.world_x, mouse_state.world_y) {
        // the paint toggle left of the machine
        mouse_state.over_menu_row = MenuRow::None;
        mouse_state.over_menu_button = MenuButton::PaintToggleButton;
      }
      else if hit_test_machine_button(mouse_state.world_x, mouse_state.world_y) {
        // the big machine button
        mouse_state.over_menu_row = MenuRow::None;
        mouse_state.over_menu_button = MenuButton::FactoryButton;
      }
    }
    Zone::BottomBottom => {}
    Zone::TopRight => {}
    ZONE_OFFERS => {
      if !mouse_state.is_dragging {
        // When already dragging do not update offer visual state, do not record the "over" state at all
        // When dragging an offer, the offer_down_offer_index will be set to the initial offer index (keep it!)
        let (offer_hover, offer_hover_offer_index) = hit_test_offers(factory, mouse_state.world_x, mouse_state.world_y);
        if offer_hover {
          // Do not consider offers that are not visible / interactive to be hoverable either
          if factory.available_parts_rhs_menu[offer_hover_offer_index].1 {
            mouse_state.offer_hover = true;
            mouse_state.offer_hover_offer_index = offer_hover_offer_index;
          }
        }
      }
    }
    Zone::BottomRight => {}
    Zone::BottomBottomRight => {}
  }

  // on mouse down
  if last_mouse_was_down {
    log!("ok something down...");
    mouse_state.last_down_event_type = if state.event_type_swapped { if last_down_event_type == EventSourceType::Touch { EventSourceType::Mouse } else { EventSourceType::Touch } } else { if last_down_event_type == EventSourceType::Mouse { EventSourceType::Mouse } else { EventSourceType::Touch } };
    mouse_state.last_down_button = last_mouse_down_button;
    mouse_state.last_down_canvas_x = last_mouse_down_x;
    mouse_state.last_down_canvas_y = last_mouse_down_y;
    mouse_state.last_down_world_x = last_mouse_down_x / CANVAS_CSS_WIDTH * CANVAS_WIDTH;
    mouse_state.last_down_world_y = last_mouse_down_y / CANVAS_CSS_HEIGHT * CANVAS_HEIGHT;
    mouse_state.last_down_cell_x = (mouse_state.last_down_world_x - UI_FLOOR_OFFSET_X) / CELL_W;
    mouse_state.last_down_cell_y = (mouse_state.last_down_world_y - UI_FLOOR_OFFSET_Y) / CELL_H;
    mouse_state.last_down_cell_x_floored = mouse_state.last_down_cell_x.floor();
    mouse_state.last_down_cell_y_floored = mouse_state.last_down_cell_y.floor();

    mouse_state.is_down = true; // Unset after on_up
    mouse_state.was_down = true; // Unset after this frame

    mouse_state.down_zone = coord_to_zone(options, state, config, mouse_state.last_down_world_x, mouse_state.last_down_world_y, is_machine_selected, factory, cell_selection.coord);
    log!("DOWN event (type={:?}) in zone {:?}, coord {}x{}", if mouse_state.last_down_event_type == EventSourceType::Mouse { "Mouse" } else { "Touch" }, mouse_state.down_zone, mouse_state.world_x, mouse_state.world_y);

    match mouse_state.down_zone {
      Zone::None => panic!("cant be down on no zone"),
      ZONE_MANUAL => {}
      ZONE_CRAFT => {
        if options.enable_craft_menu_interact {
          let ( what, wx, wy, ww, wh, icon, part_index, craft_index) = hit_test_get_craft_interactable_machine_at(options, state, factory, cell_selection, mouse_state.last_down_world_x, mouse_state.last_down_world_y);
          log!("down inside craft selection -> {:?} {:?} {} at craft index {}", what, part_index, config.nodes[part_index].raw_name, craft_index);
          if part_index == PARTKIND_NONE {
            log!("  started dragging from an empty input, ignoring...");
            mouse_state.craft_down_ci = CraftInteractable::None;
          } else {
            log!("  started dragging from a {:?}", what);
            mouse_state.craft_down_ci = what;
            mouse_state.craft_down_ci_wx = wx;
            mouse_state.craft_down_ci_wy = wy;
            mouse_state.craft_down_ci_wx = ww;
            mouse_state.craft_down_ci_wy = wh;
            mouse_state.craft_down_ci_icon = icon;
            mouse_state.craft_down_ci_part_kind = part_index;
            mouse_state.craft_down_ci_index = craft_index;
          }
        } else {
          log!("down on craft is ignored because options.enable_craft_menu_interact = false")
        }
      }
      ZONE_HELP => {
        if mouse_state.help_hover {
          mouse_state.help_down = true;
        }
      }
      ZONE_QUOTES => {
        if hit_test_undo(mouse_state.last_down_world_x, mouse_state.last_down_world_y) {
          mouse_state.down_undo = true;
        }
        else if hit_test_clear(mouse_state.last_down_world_x, mouse_state.last_down_world_y) {
          mouse_state.down_clear = true;
        }
        else if hit_test_redo(mouse_state.last_down_world_x, mouse_state.last_down_world_y) {
          mouse_state.down_redo = true;
        }
        else {
          mouse_state.down_quest =
            mouse_state.last_down_world_x >= UI_QUOTES_OFFSET_X + UI_QUOTE_X && mouse_state.last_down_world_x < UI_QUOTES_OFFSET_X + UI_QUOTE_X + UI_QUEST_WIDTH &&
            (mouse_state.last_down_world_y - (UI_QUOTES_OFFSET_Y + UI_QUOTE_Y)) % (UI_QUEST_HEIGHT + UI_QUEST_MARGIN) < UI_QUEST_HEIGHT;
          mouse_state.down_quest_visible_index = if mouse_state.down_quest { ((mouse_state.last_down_world_y - (UI_QUOTES_OFFSET_Y + UI_QUOTE_Y)) / (UI_QUEST_HEIGHT + UI_QUEST_MARGIN)) as usize } else { 0 };
        }
      }
      ZONE_SAVE_MAP => {
        let button_index = hit_test_save_map(mouse_state.last_down_world_x, mouse_state.last_down_world_y);
        if button_index == 100 { return; } // Not up on a button
        mouse_state.down_save_map = true;
        mouse_state.down_save_map_index = button_index;
      }
      Zone::BottomBottomLeft => {}
      ZONE_DAY_BAR => {}
      ZONE_FLOOR => {
        mouse_state.down_floor_area = true;
        mouse_state.down_floor_not_corner =
          // Over floor cells
          mouse_state.last_down_cell_x >= 0.0 && mouse_state.last_down_cell_x < (FLOOR_CELLS_W as f64) && mouse_state.last_down_cell_y >= 0.0 && mouse_state.last_down_cell_y < (FLOOR_CELLS_H as f64) &&
          // Not corner
          !((mouse_state.last_down_cell_x_floored == 0.0 || mouse_state.last_down_cell_x_floored == (FLOOR_CELLS_W - 1) as f64) && (mouse_state.last_down_cell_y_floored == 0.0 || mouse_state.last_down_cell_y_floored == (FLOOR_CELLS_H - 1) as f64));
      }
      ZONE_MENU => {
        let ( menu_button, button_row ) = hit_test_menu_button(mouse_state.last_down_world_x, mouse_state.last_down_world_y);
        if menu_button != MenuButton::None {
          // time controls, first, second row of menu buttons
          mouse_state.down_menu_row = button_row;
          mouse_state.down_menu_button = menu_button;
        }
        else if hit_test_paint_toggle(mouse_state.last_down_world_x, mouse_state.last_down_world_y) {
          // the paint toggle left of the machine
          mouse_state.down_menu_row = MenuRow::None;
          mouse_state.down_menu_button = MenuButton::PaintToggleButton;
        }
        else if hit_test_machine_button(mouse_state.last_down_world_x, mouse_state.last_down_world_y) {
          // the big machine button
          mouse_state.down_menu_row = MenuRow::None;
          mouse_state.down_menu_button = MenuButton::FactoryButton;
        }
      }
      Zone::BottomBottom => {}
      Zone::TopRight => {}
      ZONE_OFFERS => {
        if mouse_state.offer_hover {
          mouse_state.offer_down = true;
          mouse_state.offer_down_offer_index = mouse_state.offer_hover_offer_index;
        }
      }
      Zone::BottomRight => {}
      Zone::BottomBottomRight => {}
    }
  }

  // on drag start (maybe)
  // Note: keep out of button down check because it needs to wait for movement
  // determine whether mouse is considered to be dragging (there's a buffer of movement before
  // we consider a mouse down to mouse up to be dragging. But once we do, we stick to it.)
  if mouse_state.is_down && !mouse_state.is_dragging && mouse_state.moved_since_start && ((mouse_state.last_down_world_x - mouse_state.world_x).abs() > 5.0 || (mouse_state.last_down_world_y - mouse_state.world_y).abs() > 5.0) {
    // 5 world pixels? sensitivity tbd
    log!("is_drag_start from zone {:?}, down at {}x{}, now at {}x{}", mouse_state.down_zone, mouse_state.last_down_world_x, mouse_state.last_down_world_y, mouse_state.world_x, mouse_state.world_y);
    mouse_state.is_drag_start = true;
    mouse_state.is_dragging = true;

    match mouse_state.down_zone {
      ZONE_CRAFT => {
        if options.enable_craft_menu_interact {
          // Prevent any other interaction to the floor regardless of whether an interactable was hit
          if mouse_state.craft_down_ci != CraftInteractable::None && mouse_state.craft_down_ci != CraftInteractable::BackClose {
            log!("drag start, craft interactable; {}-{} and {}-{}; dragging a {} at index {}", mouse_state.last_down_world_x, mouse_state.world_x, mouse_state.last_down_world_y, mouse_state.world_y, mouse_state.craft_down_ci_part_kind, mouse_state.craft_down_ci_index);
            mouse_state.craft_dragging_ci = true;
          }
          else {
            log!("drag start, craft, but not interactable; ignoring");
          }
        } else {
          log!("drag start on craft is ignored because options.enable_craft_menu_interact = false")
        }
      }
      ZONE_FLOOR => {
        let is_machine_selected = cell_selection.on && factory.floor[cell_selection.coord].kind == CellKind::Machine;
        if is_machine_selected {
          log!("Closing craft menu because drag start on floor");
          cell_selection.on = false;
        }
      }
      _ => {
        log!("drag start, non-craft; {}-{} and {}-{}", mouse_state.last_down_world_x, mouse_state.world_x, mouse_state.last_down_world_y, mouse_state.world_y);
      }
    }
  }

  // on mouse up
  if last_mouse_was_up {
    mouse_state.last_up_canvas_x = last_mouse_up_x;
    mouse_state.last_up_canvas_y = last_mouse_up_y;
    mouse_state.last_up_world_x = last_mouse_up_x / CANVAS_CSS_WIDTH * CANVAS_WIDTH;
    mouse_state.last_up_world_y = last_mouse_up_y / CANVAS_CSS_HEIGHT * CANVAS_HEIGHT;
    mouse_state.last_up_cell_x = (mouse_state.last_up_world_x - UI_FLOOR_OFFSET_X) / CELL_W;
    mouse_state.last_up_cell_y = (mouse_state.last_up_world_y - UI_FLOOR_OFFSET_Y) / CELL_H;
    mouse_state.is_down = false;
    mouse_state.is_up = true;
    mouse_state.was_up = true;

    if mouse_state.is_drag_start {
      mouse_state.is_drag_start = false; // ignore :shrug:
    }
    if mouse_state.is_dragging {
      mouse_state.is_dragging = false;
      mouse_state.was_dragging = true;
    }

    mouse_state.up_zone = coord_to_zone(options, state, config, mouse_state.last_up_world_x, mouse_state.last_up_world_y, is_machine_selected, factory, cell_selection.coord);
    log!("UP event (type={:?}) in zone {:?}, was down in zone {:?}, coord {}x{}", if mouse_state.last_down_event_type == EventSourceType::Mouse { "Mouse" } else { "Touch" }, mouse_state.up_zone, mouse_state.down_zone, mouse_state.last_up_world_x, mouse_state.last_up_world_y);

    match mouse_state.up_zone {
      Zone::None => panic!("cant be up on no zone"),
      ZONE_MANUAL => {}
      ZONE_CRAFT => {
        let ( what, wx, wy, ww, wh, icon, part_index, craft_index) = hit_test_get_craft_interactable_machine_at(options, state, factory, cell_selection, mouse_state.last_up_world_x, mouse_state.last_up_world_y);
        if mouse_state.is_dragging {
          log!("up / drag end inside craft selection -> {:?} -> dropping {} ({:?})", what, mouse_state.craft_down_ci_part_kind, config.nodes[mouse_state.craft_down_ci_part_kind].raw_name);
        } else {
          log!("up inside craft selection -> {:?}", what);
        }
        if mouse_state.is_dragging || options.enable_craft_menu_interact {
          mouse_state.craft_up_ci = what;
          mouse_state.craft_up_ci_wx = wx;
          mouse_state.craft_up_ci_wy = wy;
          mouse_state.craft_up_ci_wx = ww;
          mouse_state.craft_up_ci_wy = wh;
          mouse_state.craft_up_ci_icon = icon;
          mouse_state.craft_up_ci_part_kind = part_index;
          mouse_state.craft_up_ci_index = craft_index;
        } else {
          log!("up non-drag on craft is ignored because options.enable_craft_menu_interact = false");
        }
      }
      ZONE_HELP => {
      }
      ZONE_QUOTES => {
        if hit_test_undo(mouse_state.last_up_world_x, mouse_state.last_up_world_y) {
          mouse_state.up_undo = true;
        }
        else if hit_test_clear(mouse_state.last_up_world_x, mouse_state.last_up_world_y) {
          mouse_state.up_clear = true;
        }
        else if hit_test_redo(mouse_state.last_up_world_x, mouse_state.last_up_world_y) {
          mouse_state.up_redo = true;
        }
        else {
          mouse_state.up_quote =
            mouse_state.last_up_world_x >= UI_QUOTES_OFFSET_X + UI_QUOTE_X && mouse_state.last_up_world_x < UI_QUOTES_OFFSET_X + UI_QUOTE_X + UI_QUEST_WIDTH &&
            (mouse_state.last_up_world_y - (UI_QUOTES_OFFSET_Y + UI_QUOTE_Y)) % (UI_QUEST_HEIGHT + UI_QUEST_MARGIN) < UI_QUEST_HEIGHT;
          mouse_state.up_quest_visible_index = if mouse_state.up_quote { ((mouse_state.last_up_world_y - (UI_QUOTES_OFFSET_Y + UI_QUOTE_Y)) / (UI_QUEST_HEIGHT + UI_QUEST_MARGIN)) as usize } else { 0 };
        }
      }
      ZONE_SAVE_MAP => {
        let button_index = hit_test_save_map(mouse_state.last_up_world_x, mouse_state.last_up_world_y);
        if button_index == 100 { return; } // Not up on a button
        mouse_state.up_save_map = true;
        mouse_state.up_save_map_index = button_index;
      }
      Zone::BottomBottomLeft => {}
      ZONE_DAY_BAR => {
      }
      ZONE_FLOOR => {
      }
      ZONE_MENU => {
        let ( menu_button, button_row ) = hit_test_menu_button(mouse_state.last_up_world_x, mouse_state.last_up_world_y);
        if menu_button != MenuButton::None {
          // time controls, first, second row of menu buttons
          mouse_state.up_menu_button = menu_button;
          mouse_state.up_menu_row = button_row;
        } else if hit_test_paint_toggle(mouse_state.last_up_world_x, mouse_state.last_up_world_y) {
          // the paint toggle left of the machine
          mouse_state.up_menu_row = MenuRow::None;
          mouse_state.up_menu_button = MenuButton::PaintToggleButton;
        }
        else if hit_test_machine_button(mouse_state.last_up_world_x, mouse_state.last_up_world_y) {
          // the big machine button
          mouse_state.up_menu_row = MenuRow::None;
          mouse_state.up_menu_button = MenuButton::FactoryButton;
        }
      }
      Zone::BottomBottom => {}
      Zone::TopRight => {}
      ZONE_OFFERS => {
      }
      Zone::BottomRight => {}
      Zone::BottomBottomRight => {}
    }
  }
}
fn handle_input(cell_selection: &mut CellSelection, mouse_state: &mut MouseState, options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, saves: &mut [Option<(web_sys::HtmlCanvasElement, String)>; 9]) {
  if state.manual_open {
    // If the manual is open, ignore all other events
    if mouse_state.is_up {
      state.manual_open = false;
    }
    log!("Ignoring most mouse/touch input while manual is open");
    return;
  }

  if mouse_state.is_drag_start {
    match mouse_state.down_zone {
      ZONE_CRAFT => {
        if options.enable_craft_menu_interact {
          on_drag_start_craft(options, state, config, factory, mouse_state, cell_selection);
        } else {
          log!("ignoring drag start on craft menu");
        }
      }
      ZONE_OFFERS => {
        if mouse_state.offer_down {
          on_drag_start_offer(options, state, config, factory, mouse_state, cell_selection);
        }
      }
      ZONE_MENU => {
        if mouse_state.down_menu_button == MenuButton::FactoryButton {
          on_drag_start_machine_button(options, state, config, mouse_state);
        }
      }
      _ => {}
    }
  }
  else if mouse_state.was_down {
    match mouse_state.down_zone {
      ZONE_FLOOR => {
        on_down_floor(mouse_state);
      }
      ZONE_QUOTES => {
        if mouse_state.down_undo {
          on_down_undo(options, state, config, factory, mouse_state);
        }
        else if mouse_state.down_clear {
          on_down_clear(options, state, config, factory, mouse_state);
        }
        else if mouse_state.down_redo {
          on_down_redo(options, state, config, factory, mouse_state);
        }
        else if mouse_state.down_quest {
          on_down_quest(options, state, config, factory, mouse_state);
        }
      }
      _ => {}
    }
  }
  else if mouse_state.is_down && !mouse_state.is_up {
    match mouse_state.down_zone {
      ZONE_FLOOR => {
        if mouse_state.down_zone == mouse_state.down_zone {
          // Dragging on floor when started from floor and not up
          on_drag_floor(options, state, config, factory, mouse_state);
        }
      }
      _ => {}
    }
  }

  if mouse_state.is_up {
    if state.mouse_mode_selecting && mouse_state.up_zone == ZONE_FLOOR {
      on_up_selecting(options, state, config, factory, mouse_state, cell_selection);
      return;
    }

    if mouse_state.was_dragging {
      match mouse_state.up_zone {
        ZONE_CRAFT => {
          if mouse_state.dragging_offer {
            on_drag_end_offer_over_craft(options, state, config, factory, mouse_state, cell_selection);
          }
          else {
            on_drag_end_craft(options, state, config, factory, cell_selection, mouse_state);
          }
        }
        ZONE_FLOOR => {
          if mouse_state.dragging_offer {
            on_drag_end_offer_over_floor(options, state, config, factory, mouse_state);
          }
          else if mouse_state.down_menu_button == MenuButton::FactoryButton {
            if mouse_state.dragging_machine {
              on_drag_end_machine_over_floor(options, state, config, factory, mouse_state);
            }
          }
          else if mouse_state.down_zone == ZONE_CRAFT {
            on_drag_end_craft_over_floor(options, state, config, factory, cell_selection, mouse_state);
          }
          else {
            on_drag_end_floor(options, state, config, factory, cell_selection, mouse_state);
          }
        }
        _ => {}
      }
    } else {
      match mouse_state.up_zone {
        ZONE_DAY_BAR => {
          on_up_top_bar(options, state, config, factory, mouse_state)
        }
        ZONE_CRAFT => {
          on_up_craft(options, state, config, factory, cell_selection, mouse_state);
        }
        ZONE_OFFERS => {
          if mouse_state.offer_down {
            on_up_offer(options, state, config, factory, mouse_state);
          }
        }
        ZONE_QUOTES => {
          if mouse_state.up_undo {
            on_up_undo(options, state, config, factory, mouse_state);
          }
          else if mouse_state.up_clear {
            on_up_clear(options, state, config, factory, mouse_state);
          }
          else if mouse_state.up_redo {
            on_up_redo(options, state, config, factory, mouse_state);
          }
          else if mouse_state.up_quote {
            on_up_quest(options, state, config, factory, mouse_state);
          }
        }
        ZONE_SAVE_MAP => {
          if mouse_state.up_save_map {
            on_up_save_map(options, state, config, factory, mouse_state, saves);
          }
        }
        ZONE_HELP => {
          if mouse_state.help_down {
            on_click_help(options, state, config);
          }
        }
        ZONE_MENU => {
          on_up_menu(cell_selection, mouse_state, options, state, config, factory);
        }
        ZONE_FLOOR => {
          on_up_floor(options, state, config, factory, cell_selection, &mouse_state);
        }
        _ => {}
      }
    }
  }
}

// on over, out, hover, down, up, drag start, dragging, drag end. but not everything makes sense for all cases.

fn on_drag_start_offer(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, mouse_state: &mut MouseState, cell_selection: &mut CellSelection) {
  // Is that offer visible / interactive yet?
  if factory.available_parts_rhs_menu[mouse_state.offer_down_offer_index].1 {
    // Need to remember which offer we are currently dragging (-> offer_down_offer_index).
    log!("is_drag_start from offer {} ({:?})", mouse_state.offer_down_offer_index, factory.available_parts_rhs_menu[mouse_state.offer_down_offer_index].0);
    mouse_state.dragging_offer = true;
    state.mouse_mode_selecting = false;

    log!("not closing machine while dragging atomic part");
    // let part_index = factory.available_parts_rhs_menu[mouse_state.offer_down_offer_index].0;
    // if config.nodes[part_index].pattern_unique_kinds.len() == 0 {
    //   log(format!("closing machine craft menu because dragging offer without pattern"));
    //   cell_selection.on = false;
    // }
  }
}
fn on_click_help(options: &Options, state: &mut State, config: &Config) {
  log!("on_click_help()");
  state.manual_open = !state.manual_open;
}
fn on_down_floor(mouse_state: &mut MouseState) {
  log!("on_down_floor_after(); type = {}", if mouse_state.last_down_event_type == EventSourceType::Mouse { "Mouse" } else { "Touch" });
  // Set the current cell as the last coord so we can track the next
  mouse_state.last_cell_x = mouse_state.last_down_cell_x_floored;
  mouse_state.last_cell_y = mouse_state.last_down_cell_y_floored;
}
fn on_drag_floor(options: &Options, state: &mut State, config: &Config, factory: &mut Factory, mouse_state: &mut MouseState) {
  // Do not log drag events by default :)
  // log!("on_drag_floor()");

  if mouse_state.last_down_event_type == EventSourceType::Touch {
    let cell_x1 = mouse_state.last_cell_x;
    let cell_y1 = mouse_state.last_cell_y;
    let cell_x2 = mouse_state.cell_x_floored;
    let cell_y2 = mouse_state.cell_y_floored;

    mouse_state.last_cell_x = mouse_state.cell_x_floored;
    mouse_state.last_cell_y = mouse_state.cell_y_floored;

    let sdx = cell_x2 - cell_x1;
    let sdy = cell_y1 - cell_y2;
    let dx = sdx.abs();
    let dy = sdy.abs();

    let coord1 = to_coord(cell_x1 as usize, cell_y1 as usize);
    let coord2 = to_coord(cell_x2 as usize, cell_y2 as usize);

    let action = mouse_button_to_action(state, mouse_state);

    // If touch-drag check if a cell boundary was crossed. If so, belt connect the cells.
    if action == Action::Add {
      // Must have crossed at least one cell border
      // Ignore dragging outside of the floor as it crashes the ray tracing
      if (dx > 0.0 || dy > 0.0) && is_floor(cell_x1 as f64, cell_y1 as f64) && is_floor(cell_x2 as f64, cell_y2 as f64) {
        // It's possible, due to lag or whatever, that the cells are not exact neighbors, or even
        // close to each other. To cover that case we draw a path between the last known and
        // current cell and connect them one by one.
        let track = ray_trace_dragged_line_expensive(
          factory,
          cell_x1,
          cell_y1,
          cell_x2,
          cell_y2,
          false
        );

        for i in 1..track.len() {
          let ((cell_x1, cell_y1), belt_type1, _unused, _port_out_dir1) = track[i-1]; // First element has no inbound port here
          let ((cell_x2, cell_y2), belt_type2, _port_in_dir2, _unused) = track[i]; // Last element has no outbound port here

          apply_action_between_two_cells(state, options, config, factory, action, cell_x1, cell_y1, belt_type1, cell_x2, cell_y2, belt_type2);
        }
      }
    } else {
      log!(" - Disconnecting the two cells, c1=@{}, c2=@{}", coord1, coord2);

      // Delete the port between the two cells but leave everything else alone.
      // (If a neighbor cell ends up as BeltType::NONE then it'll become CellKind::Empty as usual)
      // The coords must be adjacent to one side.

      if factory.floor[coord1].kind != CellKind::Empty {
        log!("Deleting stub @{} after rmb click", coord1);
        floor_delete_cell_at_partial(options, state, config, factory, coord1);
      }
    }
  }
}
fn on_up_offer(options: &Options, state: &State, config: &Config, factory: &Factory, mouse_state: &mut MouseState) {
  log!("on_up_offer({} -> {})", mouse_state.offer_down_offer_index, mouse_state.offer_hover_offer_index);

  let ( _part_index, visible ) = factory.available_parts_rhs_menu[mouse_state.offer_down_offer_index];
  if !visible {
    // Invisible offers are not interactive
    return;
  }
  if mouse_state.offer_down_offer_index != mouse_state.offer_hover_offer_index {
    // Did not pointer up on the same offer as we did the pointer down
    return;
  }

  if mouse_state.offer_selected && mouse_state.offer_selected_index == mouse_state.offer_hover_offer_index{
    log!("Deselecting offer {}", mouse_state.offer_hover_offer_index);
    mouse_state.offer_selected = false;
  } else {
    log!("Selecting offer {}", mouse_state.offer_hover_offer_index);
    mouse_state.offer_selected = true;
    mouse_state.offer_selected_index = mouse_state.offer_hover_offer_index;
  }
}
fn on_down_undo(options: &Options, state: &State, config: &Config, factory: &Factory, mouse_state: &mut MouseState) {
  log!("on_down_undo()");
}
fn on_up_undo(options: &Options, state: &mut State, config: &Config, factory: &Factory, mouse_state: &mut MouseState) {
  log!("on_up_undo()");
  // keep stack of n snapshots
  // when undoing, put pointer backwards on the existing stack
  // when redoing, move it forward
  // when making a change and the pointer is not last, copy the current snapshot to last and then add the new snapshot
  // this way you can still go back in time even after an undo and new change
  // perhaps a "normal" undo mode would be preferable though.
  // pointer rolls over after the max snap count. undo just rolls to the front if at zero
  // means we have to track an undo pointer as well, which is a temporary pointer as long as it is not equal to the real pointer

  if state.snapshot_undo_pointer > 0 {
    state.snapshot_undo_pointer -= 1;
    state.load_snapshot_next_frame = true;
  }
}
fn on_down_clear(options: &Options, state: &State, config: &Config, factory: &Factory, mouse_state: &mut MouseState) {
  log!("on_down_clear()");
}
fn on_up_clear(options: &Options, state: &State, config: &Config, factory: &mut Factory, mouse_state: &mut MouseState) {
  log!("on_up_clear()");
  log!("Removing all cells from the factory...");
  for coord in 0..factory.floor.len() {
    let (x, y) = to_xy(coord);
    factory.floor[coord] = empty_cell(config, x, y);
  }
  factory.changed = true;
}
fn on_down_redo(options: &Options, state: &State, config: &Config, factory: &Factory, mouse_state: &mut MouseState) {
  log!("on_down_redo()");
}
fn on_up_redo(options: &Options, state: &mut State, config: &Config, factory: &Factory, mouse_state: &mut MouseState) {
  log!("on_up_redo()");
  // if state.snapshot_undo_pointer is not equal to state.snapshot_pointer
  // move the pointer forward. otherwise assume that you can't go forward
  if state.snapshot_undo_pointer != state.snapshot_pointer {
    state.snapshot_undo_pointer += 1;
    state.load_snapshot_next_frame = true;
  }
}
fn on_down_quest(options: &Options, state: &State, config: &Config, factory: &Factory, mouse_state: &mut MouseState) {
  log!("on_down_quest({}). questss: {}", mouse_state.down_quest_visible_index, factory.quests.iter().filter(|q| q.status == QuestStatus::Active).collect::<Vec<_>>().len());
}
fn on_up_quest(options: &Options, state: &State, config: &Config, factory: &mut Factory, mouse_state: &mut MouseState) {
  log!("on_up_quest({}), quests: {}, down on {:?} {}", mouse_state.up_quest_visible_index, factory.quests.iter().filter(|q| q.status == QuestStatus::Active).collect::<Vec<_>>().len(), mouse_state.down_zone, mouse_state.down_quest_visible_index);

  if options.dbg_clickable_quests && mouse_state.down_quest && mouse_state.down_quest_visible_index == mouse_state.up_quest_visible_index {
    log!("  clicked on this quest (down=up). Completing it now...");
    let mut visible_index = 0;
    for quest_index in 0..factory.quests.len() {
      if factory.quests[quest_index].status != QuestStatus::Active && factory.quests[quest_index].status != QuestStatus::FadingAndBouncing {
        continue;
      }
      if visible_index == mouse_state.up_quest_visible_index {
        log!("  satisfying quest {} to production target", quest_index);
        factory.quests[quest_index].production_progress = factory.quests[quest_index].production_target;
        quest_update_status(&mut factory.quests[quest_index], QuestStatus::FadingAndBouncing, factory.ticks);
        factory.quests[quest_index].bouncer.bounce_from_index = visible_index;
        factory.quests[quest_index].bouncer.bouncing_at = factory.ticks;
        return;
      }
      visible_index += 1;
    }

    log!("Clicked on a quest index that doesnt exist right now. mouse_state.down_quest_visible_index={}, mouse_state.up_quest_visible_index={}", mouse_state.down_quest_visible_index, mouse_state.up_quest_visible_index);
  }
}
fn on_up_save_map(options: &Options, state: &mut State, config: &Config, factory: &mut Factory, mouse_state: &mut MouseState, saves: &mut [Option<(web_sys::HtmlCanvasElement, String)>; 9]) {
  log!("on_up_save_map()");

  if !mouse_state.down_save_map || mouse_state.down_save_map_index != mouse_state.up_save_map_index {
    log!("  down != up, bailing: {} {} {}", mouse_state.down_save_map, mouse_state.down_save_map_index, mouse_state.up_save_map_index);
    return;
  }

  if let Some((canvas, map_string)) = &saves[mouse_state.up_save_map_index] {
    let (row, col) = match mouse_state.up_save_map_index {
      0 => (0.0, 0.0),
      1 => (0.0, 1.0),
      2 => (1.0, 0.0),
      3 => (1.0, 1.0),
      _ => panic!("no such button: {}", mouse_state.up_save_map_index),
    };
    let button_x = hit_test_save_map_right(mouse_state.world_x, mouse_state.world_y, row, col);

    if button_x {
      log!("  deleting saved map");
      saves[mouse_state.up_save_map_index] = None;
      let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
      local_storage.remove_item(format!("factini.save{}", mouse_state.up_save_map_index).as_str()).unwrap();
    }
    else {
      log!("  loading saved map");
      state.snapshot_pointer += 1;
      state.snapshot_undo_pointer = state.snapshot_pointer;
      state.snapshot_stack[state.snapshot_pointer % UNDO_STACK_SIZE] = map_string.clone();
      state.load_snapshot_next_frame = true;
    }
  } else {
    log!("  storing saved map");
    let document = document();

    // This element is created in this file but it's just easier to query it from the DOM ;)
    let game_map: web_sys::HtmlCanvasElement = document.get_element_by_id("$main_game_canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    // Create a new canvas and draw the floor part onto that canvas
    let canvas = document.create_element("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    // document.get_element_by_id("$main_game").unwrap().append_child(&canvas)?;
    // canvas.set_id(&"$main_game_canvas".into());
    // canvas.set_width(600 as u32);
    // canvas.set_height(600 as u32);
    canvas.set_width((UI_SAVE_THUMB_WIDTH * 0.66) as u32);
    canvas.set_height(UI_SAVE_THUMB_HEIGHT as u32);
    let context = canvas.get_context("2d").expect("get context must work").unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    context.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
      &game_map,
      UI_FLOOR_OFFSET_X, UI_FLOOR_OFFSET_Y, UI_FLOOR_WIDTH, UI_FLOOR_HEIGHT,
      0.0, 0.0, UI_SAVE_THUMB_WIDTH * 0.66, UI_SAVE_THUMB_HEIGHT
    ).expect("canvas api call to work");

    // Get string of map
    let snap = generate_floor_dump(&options, &state, &config, &factory, dnow()).join("\n");

    // Store it there and in local storage
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage.set_item(format!("factini.save{}", mouse_state.up_save_map_index).as_str(), &snap).unwrap();
    saves[mouse_state.up_save_map_index] = Some( ( canvas, snap ) );
  }
}
fn on_drag_end_floor(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState) {
  log!("on_drag_end_floor()");
  // Is the mouse currently on the floor?
  on_drag_end_floor_other(options, state, config, factory, cell_selection, mouse_state);
}
fn on_up_floor(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState) {
  log!("on_up_floor()");
  on_click_inside_floor(options, state, config, factory, cell_selection, mouse_state);
}
fn on_click_inside_floor(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState) {
  log!("on_click_inside_floor()");
  let last_mouse_up_cell_x = mouse_state.last_up_cell_x.floor();
  let last_mouse_up_cell_y = mouse_state.last_up_cell_y.floor();

  let action = mouse_button_to_action(state, mouse_state);

  if action == Action::Remove {
    // Clear the cell if that makes sense for it. Delete a belt with one or zero ports.
    let coord = to_coord(last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize);

    let mut ports = 0;
    if factory.floor[coord].port_u != Port::None { ports += 1; }
    if factory.floor[coord].port_r != Port::None { ports += 1; }
    if factory.floor[coord].port_d != Port::None { ports += 1; }
    if factory.floor[coord].port_l != Port::None { ports += 1; }
    if ports <= 1 || factory.floor[coord].kind == CellKind::Machine {
      log!("Deleting stub @{} after rmb click", coord);
      floor_delete_cell_at_partial(options, state, config, factory, coord);
      factory.changed = true;
    }

    // If this wasn't a belt (ports=999) or the belt had more than 1 ports, then just drop its part.
    if ports > 1 {
      log!("Clearing part from @{} after rmb click (ports={})", coord, ports);
      clear_part_from_cell(options, state, config, factory, coord);
    }
  }
  else if action == Action::Add {
    // De-/Select this cell
    log!("clicked {} {} cell selection before: {:?}, belt: {:?}", last_mouse_up_cell_x, last_mouse_up_cell_y, cell_selection, factory.floor[to_coord(last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize)].belt);

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
fn on_up_craft(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState) {
  log!("on_up_craft()");

  log!("Clicked in the selection bubble of a machine");
  // Figure out whether any of the interactables were clicked

  match mouse_state.craft_up_ci {
    CraftInteractable::BackClose => {
      log!("Clicked the close button");
      cell_selection.on = false;
    }
    CraftInteractable::Resource => {
      log!("Clicked a resource: {}", mouse_state.craft_up_ci_icon);
    }
    CraftInteractable::InputCell => {
      log!("Clicked an input cell: {}", mouse_state.craft_up_ci_icon);

      // Force-clear this cell of the machine
      machine_change_want_kind(options, state, config, factory, factory.floor[cell_selection.coord].machine.main_coord, mouse_state.craft_up_ci_index as usize - 100, PARTKIND_NONE);
    }
    CraftInteractable::None => {
      log!("Clicked inside selection craft menu but not on an interactable; ignoring");
    }
  }
}
fn on_drag_end_craft_over_floor(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState) {
  log!("on_drag_end_craft_over_floor() dropping a craft icon on the floor does nothing, action ignored.");
}
fn on_drag_end_machine_over_floor(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, mouse_state: &MouseState) {
  log!("on_drag_offer_into_floor({}, {})", mouse_state.last_up_cell_x, mouse_state.last_up_cell_y);
  assert!(mouse_state.last_up_cell_x >= 0.0 && mouse_state.last_up_cell_y >= 0.0, "should not call this when mouse is oob. usize cant be negative");

  // Was dragging a machine and released it on the floor

  // First check eligibility: Would every part of the machine be on a middle cell, not edge?
  let ocw = 3; // Fixing to 3x3 for now
  let och = 3;
  let cx = get_x_while_dragging_offer_machine(mouse_state.last_up_cell_x, ocw);
  let cy = world_y_to_top_left_cell_y_while_dragging_offer_machine(mouse_state.last_up_cell_y, och);
  // Make sure the entire machine fits, not just the center or topleft cell
  if bounds_check(cx, cy, 1.0, 1.0, FLOOR_CELLS_W as f64 - (ocw as f64), FLOOR_CELLS_H as f64 - (och as f64)) {
    let ccoord = to_coord(cx as usize, cy as usize);

    // Get all machines and then get the first unused ID. First we round up all the existing
    // machine ids into a vector and then we iterate through the vector incrementally until
    // an ID is not used. This is O(n^2) but realistically worst case O(63^2) and good luck.

    let mut ids = vec!();
    for coord in 0..FLOOR_CELLS_WH {
      if factory.floor[coord].kind == CellKind::Machine && factory.floor[coord].machine.main_coord == coord {
        ids.push(factory.floor[coord].machine.id);
      }
    }

    // Now iterate through all valid IDs, that is: 0-9a-zA-Z. I guess bail if we exhaust that.
    // TODO: gracefully handle too many machines
    let mut found = '!';
    // Note: machine ids offset at 1 (because m0 is just too confusing for comfort)
    for id in 1..62 {
      let c =
        if id >= 36 {
          (('A' as u8) + (id - 36)) as char // A-Z
        } else if id > 9 {
          (('a' as u8) + (id - 10)) as char // a-z
        } else {
          (('0' as u8) + id) as char // 1-9
        };
      if !ids.contains(&c) {
        found = c;
        break;
      }
    }
    if found == '!' {
      panic!("Unable to find a fresh ID. Either there are too many machines on the floor or there is a bug with reclaiming them. Or d: something else.");
    }

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
          floor_delete_cell_at_partial(options, state, config, factory, coord);
        }

        if i == 0 && j == 0 {
          // Top-left cell is the main_coord here
          factory.floor[coord] = machine_main_cell(
            options,
            state,
            config,
            found,
            x, y,
            ocw, och,
            vec!(), // Could fill with trash but no need I guess
            part_c(config, 't'),
            2000,
            1, 1
          );
        } else {
          factory.floor[coord] = machine_sub_cell(options, state, config, found, x, y, ccoord);
        }
        factory.floor[ccoord].machine.coords.push(coord);

        factory.floor[coord].port_u = if j == 0 { port_u } else { Port::None };
        factory.floor[coord].port_r = if i == ocw - 1 { port_r } else { Port::None };
        factory.floor[coord].port_d = if j == och - 1 { port_d } else { Port::None };
        factory.floor[coord].port_l = if i == 0 { port_l } else { Port::None };
      }
    }

    log!("Attaching machine to neighbor dead ending belts");
    for i in 0..factory.floor[ccoord].machine.coords.len() {
      let coord = factory.floor[ccoord].machine.coords[i];
      connect_to_neighbor_dead_end_belts(options, state, config, factory, coord);
    }

    machine_discover_ins_and_outs(factory, ccoord);

    factory.changed = true;
  } else {
    log!("Dropped a machine on the edge. Ignoring. {} {}", mouse_state.last_up_cell_x, mouse_state.last_up_cell_y as usize);
  }
}
fn on_drag_end_offer_over_craft(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, mouse_state: &MouseState, cell_selection: &CellSelection) {
  log!("on_drag_end_offer_over_craft()");

  let dragged_part_index = factory.available_parts_rhs_menu[mouse_state.offer_down_offer_index].0;

  let coord = to_coord(mouse_state.cell_x_floored as usize, mouse_state.cell_y_floored as usize);
  let main_coord = factory.floor[coord].machine.main_coord;
  // Figure out whether it was dropped on the machine itself and if it was the selected machine
  if factory.floor[coord].kind == CellKind::Machine && factory.floor[cell_selection.coord].machine.main_coord == main_coord {
    if config.nodes[dragged_part_index].pattern_unique_kinds.len() > 0 {
      log!("Dropped an offer with pattern in the middle on a craft menu. Update the machine!");
      for i in 0..factory.floor[main_coord].machine.cell_width * factory.floor[main_coord].machine.cell_height {
        let part_index = config.nodes[dragged_part_index].pattern_by_index.get(i).unwrap_or(&PARTKIND_NONE);
        machine_change_want_kind(options, state, config, factory, main_coord, i, *part_index);
        // Make sure the haves are cleared as well
        factory.floor[main_coord].machine.haves[i] = part_none(config);
      }
    }
    else if options.enable_craft_menu_circle {
      log!("Dropped an offer without pattern in the middle on a craft menu. Update the machine!");
      // Add dragged offer pattern to machine want list. We know the actual cell. Clobber it with
      // the part of the dragged offer.

      assert!(mouse_state.craft_up_ci_index > 99, "This should now be the machine index + 100");

      // Get the machine index and blitz them
      let target_cell_index = (mouse_state.craft_up_ci_index - 100) as usize;

      machine_change_want_kind(options, state, config, factory, main_coord, target_cell_index, dragged_part_index);
      // Make sure the haves are cleared as well
      factory.floor[main_coord].machine.haves[target_cell_index] = part_none(config);
    }
    else {
      log!("Ignoring drop of offer without pattern because options.enable_craft_menu_circle = false");
    }
  } else {
    log!("Dropped an offer in the middle on a craft menu but not on the machine. Ignoring...");
  }
}
fn on_drag_end_offer_over_floor(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, mouse_state: &MouseState) {
  log!("on_drag_end_offer_over_floor()");

  let last_mouse_up_cell_x = mouse_state.last_up_cell_x.floor();
  let last_mouse_up_cell_y = mouse_state.last_up_cell_y.floor();
  let last_mouse_up_cell_coord = to_coord(last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize);

  let dragged_part_index = factory.available_parts_rhs_menu[mouse_state.offer_down_offer_index].0;

  if is_edge_not_corner(last_mouse_up_cell_x, last_mouse_up_cell_y) {
    log!("Dropped a supply on an edge cell that is not corner. Deploying... {} {}", last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize);
    log!("Drag started from offer {} ({:?})", mouse_state.offer_down_offer_index, dragged_part_index);
    let bools = ( last_mouse_up_cell_x == 0.0, last_mouse_up_cell_y == 0.0, last_mouse_up_cell_x as usize == FLOOR_CELLS_W - 1, last_mouse_up_cell_y as usize == FLOOR_CELLS_H - 1 );
    let prev_port = match bools {
      // On the top you need to look one cell down to the up port
      ( false, true, false, false ) => factory.floor[to_coord_down(last_mouse_up_cell_coord)].port_u,
      ( false, false, true, false ) => factory.floor[to_coord_left(last_mouse_up_cell_coord)].port_r,
      ( false, false, false, true ) => factory.floor[to_coord_up(last_mouse_up_cell_coord)].port_d,
      ( true, false, false, false ) => factory.floor[to_coord_right(last_mouse_up_cell_coord)].port_l,
      _ => panic!("Should be one side"),
    };
    log!("- Was neighbor connected to this cell? {:?}", prev_port);
    // If there's already something on this cell then we need to remove it first
    if factory.floor[last_mouse_up_cell_coord].kind != CellKind::Empty {
      // Must be supply or demand
      // We should be able to replace this one with the new tile without having to update
      // the neighbors (if any). We do have to update the prio list (in case demand->supply).
      log!("Remove old edge cell...");
      floor_delete_cell_at_partial(options, state, config, factory, last_mouse_up_cell_coord);
    }
    log!("Add new supply cell...");
    factory.floor[last_mouse_up_cell_coord] = supply_cell(config, last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize, part_from_part_index(config, dragged_part_index), 2000, 500, 1);
    connect_to_neighbor_dead_end_belts(options, state, config, factory, last_mouse_up_cell_coord);
    match bools {
      ( false, true, false, false ) => factory.floor[last_mouse_up_cell_coord].port_d = Port::Outbound,
      ( false, false, true, false ) => factory.floor[last_mouse_up_cell_coord].port_l = Port::Outbound,
      ( false, false, false, true ) => factory.floor[last_mouse_up_cell_coord].port_u = Port::Outbound,
      ( true, false, false, false ) => factory.floor[last_mouse_up_cell_coord].port_r = Port::Outbound,
      _ => panic!("Should be one side"),
    }
    if prev_port != Port::None {
      log!("- Neighbor was connected so restoring that now...");
      // Port was connected before so connect it now.
      match bools {
        ( false, true, false, false ) => {
          let ocoord = to_coord_down(last_mouse_up_cell_coord);
          factory.floor[ocoord].port_u = Port::Inbound;
          fix_belt_meta(options, state, config, factory, ocoord);
          belt_discover_ins_and_outs(factory, ocoord);
        },
        ( false, false, true, false ) => {
          let ocoord = to_coord_left(last_mouse_up_cell_coord);
          factory.floor[ocoord].port_r = Port::Inbound;
          fix_belt_meta(options, state, config, factory, ocoord);
          belt_discover_ins_and_outs(factory, ocoord);
        },
        ( false, false, false, true ) => {
          let ocoord = to_coord_up(last_mouse_up_cell_coord);
          factory.floor[ocoord].port_d = Port::Inbound;
          fix_belt_meta(options, state, config, factory, ocoord);
          belt_discover_ins_and_outs(factory, ocoord);
        },
        ( true, false, false, false ) => {
          let ocoord = to_coord_right(last_mouse_up_cell_coord);
          factory.floor[ocoord].port_l = Port::Inbound;
          fix_belt_meta(options, state, config, factory, ocoord);
          belt_discover_ins_and_outs(factory, ocoord);
        },
        _ => panic!("Should be one side"),
      }
    }
    factory.changed = true;
  }
  else if is_middle(last_mouse_up_cell_x, last_mouse_up_cell_y) {
    let coord = to_coord(mouse_state.cell_x_floored as usize, mouse_state.cell_y_floored as usize);
    // Figure out whether it was dropped on a machine
    if factory.floor[coord].kind == CellKind::Machine {
      let main_coord = factory.floor[coord].machine.main_coord;
      if config.nodes[dragged_part_index].pattern_unique_kinds.len() > 0 {
        log!("Dropped an offer with pattern on a machine. Update the machine!");
        // Update machine to the pattern of the dragged part
        for want_index in 0..factory.floor[main_coord].machine.cell_width * factory.floor[main_coord].machine.cell_height {
          let part_index = config.nodes[dragged_part_index].pattern_by_index.get(want_index).unwrap_or(&PARTKIND_NONE);
          machine_change_want_kind(options, state, config, factory, main_coord, want_index, *part_index);
          // Make sure the haves are cleared as well
          factory.floor[main_coord].machine.haves[want_index] = part_none(config);
        }
      }
      else {
        // Add dragged offer pattern to machine want list, if possible
        log!("Dropped an offer without pattern on a machine. Update the machine!");
        // Find first empty spot and fill it.
        for want_index in 0..factory.floor[main_coord].machine.wants.len() {
          if factory.floor[main_coord].machine.wants[want_index].kind == PARTKIND_NONE {
            log!("Machine want index {} is empty. Filling now with {}", want_index, dragged_part_index);
            machine_change_want_kind(options, state, config, factory, main_coord, want_index, dragged_part_index);
            // Make sure the haves are cleared as well
            factory.floor[main_coord].machine.haves[want_index] = part_none(config);
            break;
          }
        }
      }
    } else {
      log!("Dropped an offer in the floor but not on a machine. Ignoring...");
    }
  }
  else {
    log!("Dropped a supply offer ({:?}) without pattern on the floor but not machine, or a supply offer on a corner. Ignoring. {} {}", dragged_part_index, last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize);
  }
}
fn on_drag_end_floor_other(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState) {
  log!("on_drag_end_floor_other()");

  // If both x and y are on the edge then they're in a corner
  if !mouse_state.over_floor_not_corner || !mouse_state.down_floor_not_corner {
    log!("mouse not over or down floor");
    // Corner cell of the floor. Consider oob and ignore.
    return;
  }

  if mouse_state.last_down_event_type == EventSourceType::Touch {
    // Do nothing here
    log!("ignoring on_drag_end_floor event for touch");
    return;
  }

  // Finalize pathing, regenerate floor
  let track = ray_trace_dragged_line_expensive(
    factory,
    mouse_state.last_down_cell_x_floored,
    mouse_state.last_down_cell_y_floored,
    mouse_state.cell_x_floored,
    mouse_state.cell_y_floored,
    false
  );

  log!("track to solidify: {:?}, button {}", track, mouse_state.last_down_button);

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
    on_drag_end_floor_one_cell(state, options, config, factory, mouse_state, track);
  }
  else if len == 2 {
    on_drag_end_floor_two_cells(state, options, config, factory, mouse_state, track);
  }
  else {
    on_drag_end_floor_multi_cells(state, options, config, factory, mouse_state, track);
  }

  factory.changed = true;
}
fn on_drag_end_floor_one_cell(state: &State, options: &Options, config: &Config, factory: &mut Factory, mouse_state: &MouseState, track: Vec<((usize, usize), BeltType, Direction, Direction)>) {
  log!("One cell path with button {} and erase mode {}", mouse_state.last_down_button, state.mouse_mode_mirrored);

  let action = mouse_button_to_action(state, mouse_state);

  if action == Action::Add {
    log!(" - Ignore click on a single cell, as well as dragging across one cell. Allows you to cancel a drag.");
  } else if action == Action::Remove {
    log!(" - Removing the cell");
    // Clear the cell if that makes sense for it
    // Do not delete a cell, not even stubs, because this would be a drag-cancel
    // (Regular click would delete stubs)
    let ((cell_x, cell_y), _belt_type, _unused, _port_out_dir) = track[0]; // First element has no inbound port here
    let coord = to_coord(cell_x, cell_y);
    clear_part_from_cell(options, state, config, factory, coord);
  } else {
    // Other mouse button. ignore for now / ever.
    // I think this allows you to cancel a drag by pressing the rmb
    log!(" - Not left or right button; ignoring unknown button click");
  }
}
fn on_drag_end_floor_two_cells(state: &State, options: &Options, config: &Config, factory: &mut Factory, mouse_state: &MouseState, track: Vec<((usize, usize), BeltType, Direction, Direction)>) {
  log!("Two cell path with button {} and erase mode {}", mouse_state.last_down_button, state.mouse_mode_mirrored);
  let ((cell_x1, cell_y1), belt_type1, _unused, _port_out_dir1) = track[0]; // First element has no inbound port here
  let ((cell_x2, cell_y2), belt_type2, _port_in_dir2, _unused) = track[1]; // Last element has no outbound port here

  let action = mouse_button_to_action(state, mouse_state);

  return apply_action_between_two_cells(state, options, config, factory, action, cell_x1, cell_y1, belt_type1, cell_x2, cell_y2, belt_type2);
}

fn apply_action_between_two_cells(state: &State, options: &Options, config: &Config, factory: &mut Factory, add_or_remove: Action, cell_x1: usize, cell_y1: usize, belt_type1: BeltType, cell_x2: usize, cell_y2: usize, belt_type2: BeltType) {
  let coord1 = to_coord(cell_x1, cell_y1);
  let coord2 = to_coord(cell_x2, cell_y2);

  let dx = (cell_x2 as i8) - (cell_x1 as i8);
  let dy = (cell_y2 as i8) - (cell_y1 as i8);
  assert!((dx == 0) != (dy == 0), "one and only one of dx or dy is zero");
  assert!(dx >= -1 && dx <= 1 && dy >= -1 && dy <= 1, "since they are adjacent they must be -1, 0, or 1");

  if add_or_remove == Action::Add {
    log!(" - Connecting the two cells");

    // Convert empty cells to belt cells.
    // Create a port between these two cells, but none of the other cells.

    if is_edge(cell_x1 as f64, cell_y1 as f64) && is_edge(cell_x2 as f64, cell_y2 as f64) {
      // Noop. Just don't.
    }
    else {
      if factory.floor[coord1].kind == CellKind::Empty {
        if is_edge_not_corner(cell_x1 as f64, cell_y1 as f64) {
          // Cell is empty so place a trash supplier here as a placeholder
          factory.floor[coord1] = supply_cell(config, cell_x1, cell_y1, part_c(config, 't'), 2000, 0, 0);
        }
        else if is_middle(cell_x1 as f64, cell_y1 as f64) {
          factory.floor[coord1] = belt_cell(config, cell_x1, cell_y1, belt_type_to_belt_meta(belt_type1));
        }
      }
      if factory.floor[coord2].kind == CellKind::Empty {
        if is_edge_not_corner(cell_x2 as f64, cell_y2 as f64) {
          // Cell is empty so place a demander here
          factory.floor[coord2] = demand_cell(config, cell_x2, cell_y2, options.default_demand_speed, options.default_demand_cooldown);
        }
        else if is_middle(cell_x2 as f64, cell_y2 as f64) {
          factory.floor[coord2] = belt_cell(config, cell_x2, cell_y2, belt_type_to_belt_meta(belt_type2));
        }
      }

      cell_connect_if_possible(options, state, config, factory, coord1, coord2, dx, dy);
    }
  }
  else if add_or_remove == Action::Remove {
    log!(" - Disconnecting the two cells");

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

    port_disconnect_cells(options, state, config, factory, coord1, dir1, coord2, dir2);
  }
  else {
    // Other mouse button or multi-button. ignore for now / ever.
    // (Remember: this was a drag of two cells)
    log!(" - Not left or right button; ignoring unknown button click");
  }

  fix_belt_meta(options, state, config, factory, coord1);
  fix_belt_meta(options, state, config, factory, coord2);

  if add_or_remove == Action::Remove {
    if factory.floor[coord1].kind == CellKind::Belt && factory.floor[coord1].port_u == Port::None && factory.floor[coord1].port_r == Port::None && factory.floor[coord1].port_d == Port::None && factory.floor[coord1].port_l == Port::None {
      floor_delete_cell_at_partial(options, state, config, factory, coord1);
    } else {
      clear_part_from_cell(options, state, config, factory, coord1);
    }
    if factory.floor[coord2].kind == CellKind::Belt && factory.floor[coord2].port_u == Port::None && factory.floor[coord2].port_r == Port::None && factory.floor[coord2].port_d == Port::None && factory.floor[coord2].port_l == Port::None {
      floor_delete_cell_at_partial(options, state, config, factory, coord2);
    } else {
      clear_part_from_cell(options, state, config, factory, coord2);
    }
  }

  factory.changed = true;
}
fn on_drag_end_floor_multi_cells(state: &State, options: &Options, config: &Config, factory: &mut Factory, mouse_state: &MouseState, track: Vec<((usize, usize), BeltType, Direction, Direction)>) {
  log!("Multi cell path with button {} and erase mode {}", mouse_state.last_down_button, state.mouse_mode_mirrored);

  // len > 2
  // Draw track if lmb, remove cells on track if rmb

  let mut still_starting_on_edge = true; // start true until first middle cell
  let mut already_ending_on_edge = false; // start false until still_starting_on_edge and current cell is edge
  let mut px = 0;
  let mut py = 0;
  let mut pcoord = 0;
  let len = track.len();
  for index in 0..len {
    let ((cell_x, cell_y), belt_type, _port_in_dir, _port_out_dir) = track[index];
    log!("- track {} at {} {} isa {:?}", index, cell_x, cell_y, belt_type);
    let coord = to_coord(cell_x, cell_y);

    let action = mouse_button_to_action(state, mouse_state);

    if action == Action::Add {
      if still_starting_on_edge {
        // Note: if the first cell is in the middle then the track does not start on the edge
        if index == 0 {
          log!("({}) first track part...", index);
          if is_middle(cell_x as f64, cell_y as f64) {
            // The track starts in the middle of the floor. Do not add a trashcan.
            log!("({})  - in middle. still_starting_on_edge now false", index);
            still_starting_on_edge = false;
          }
        }
        // Still on the edge but not the first so the prior part of the track and all pieces
        // before it were all on the edge. If this one is not then the previous cell should
        // get the trashcan treatment. And otherwise we noop until the next cell.
        else if is_middle(cell_x as f64, cell_y as f64) {
          log!("({}) first middle part of track", index);
          // Track started on the edge but has at least one segment in the middle.
          // Create a trash on the previous (edge) cell if that cell is empty.
          if factory.floor[pcoord].kind == CellKind::Empty {
            factory.floor[pcoord] = supply_cell(config, px, py, part_c(config, 't'), 2000, 0, 0);
          }
          still_starting_on_edge = false;
        }
        // This means this and all prior track parts were on the edge. Move to next part.
        else {
          log!("({}) non-first-but-still-edge part of track", index);
        }
      }
      else if is_edge_not_corner(cell_x as f64, cell_y as f64) {
        log!("({}) ending edge part of track", index);
        if !already_ending_on_edge {
          log!("({}) - first ending edge part of track, already_ending_on_edge = true", index);
          // Note: the drag can only start inside the floor, so we don't have to worry about
          //       the index here since we always drag in a straight line. Once the edge is
          //       reached, we assume the line to end and we can put a trash Demand down.
          if factory.floor[coord].kind == CellKind::Empty {
            factory.floor[coord] = demand_cell(config, cell_x, cell_y, options.default_demand_speed, options.default_demand_cooldown);
          }

          already_ending_on_edge = true;
        }
      }

      log!("({}) head-on-edge? {} tail-on-edge? {}", index, still_starting_on_edge, already_ending_on_edge);

      // If not at the start or end of the track...
      if !still_starting_on_edge && !already_ending_on_edge {
        // Create middle cell

        // Staple the track on top of the existing layout. If the cell is not empty then either
        // it's a belt which we'll try to connect to the previous/next part of the belt. Or it's
        // another piece that we don't want to override anyways, and will also be connected.
        if factory.floor[coord].kind == CellKind::Empty {
          if is_middle(cell_x as f64, cell_y as f64) {
            factory.floor[coord] = belt_cell(config, cell_x, cell_y, belt_type_to_belt_meta(belt_type));

            // Connect the end points to any existing neighboring cells if not already connected
            if index == 0 || index == len - 1 {
              log!("  -- okay. first/last track segment: @{} got {:?}", coord, belt_type);
              log!("  - connect_belt_to_existing_neighbor_belts(), before: {:?} {:?} {:?} {:?}", factory.floor[coord].port_u, factory.floor[coord].port_r, factory.floor[coord].port_d, factory.floor[coord].port_l);
              connect_belt_to_existing_neighbor_cells(options, state, config, factory, coord);
              log!("  - connect_belt_to_existing_neighbor_belts(),  after: {:?} {:?} {:?} {:?}", factory.floor[coord].port_u, factory.floor[coord].port_r, factory.floor[coord].port_d, factory.floor[coord].port_l);
            }
          }
        }
      }

      if index > 0 {
        // (First element has no inbound)
        cell_connect_if_possible(options, state, config, factory, pcoord, coord, (cell_x as i8) - (px as i8), (cell_y as i8) - (py as i8));
      }
    }
    else if action == Action::Remove {
      // Delete the cell if it is a belt, and in that case any port to it
      // Do not delete machines, suppliers, or demanders. No need to delete empty cells
      if factory.floor[coord].kind == CellKind::Belt {
        // Delete this belt tile and update the neighbors accordingly
        floor_delete_cell_at_partial(options, state, config, factory, coord);
      }
    } else {
      // Ignore whatever this is.
    }

    px = cell_x;
    py = cell_y;
    pcoord = coord;
  }
}

fn on_up_paint_toggle(state: &mut State) {
  log!("on_up_paint_toggle()");
  log!("inverting state.mouse_mode_mirrored");
  state.mouse_mode_mirrored = !state.mouse_mode_mirrored;
  // state.mouse_mode_selecting = false;
  // cell_selection.area = false;
  // cell_selection.on = false;
  // state.selected_area_copy = vec!(); // Or retain this?
}

fn on_up_machine_button() {
  log!("on_up_machine_button()");
}
fn on_drag_start_machine_button(options: &mut Options, state: &mut State, config: &Config, mouse_state: &mut MouseState) {
  log!("is_drag_start from machine");
  mouse_state.dragging_machine = true;
  state.mouse_mode_selecting = false;
}
fn on_up_menu(cell_selection: &mut CellSelection, mouse_state: &mut MouseState, options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory) {
  log!("on_up_menu() down: {:?}, up: {:?}", mouse_state.down_menu_button, mouse_state.up_menu_button);

  if mouse_state.down_menu_button != mouse_state.up_menu_button {
    if mouse_state.up_menu_button == MenuButton::None {
      log!("  Was up in menu region but not on a button so ignoring it");
    } else {
      log!("  Was up on a menu button but not down on the same button so ignoring it");
    }
    return;
  }

  match mouse_state.up_menu_button {
    MenuButton::None => {}

    MenuButton::FactoryButton => {
      on_up_machine_button();
    }
    MenuButton::PaintToggleButton => {
      on_up_paint_toggle(state);
    }

    MenuButton::Row1ButtonMin => {
      let m = options.speed_modifier_floor;
      options.speed_modifier_floor = options.speed_modifier_floor.min(0.5) * 0.5;
      log!("pressed time minus, from {} to {}", m, options.speed_modifier_floor);
    }
    MenuButton::Row1ButtonHalf => {
      let m = options.speed_modifier_floor;
      options.speed_modifier_floor = 0.5;
      log!("pressed time half, from {} to {}", m, options.speed_modifier_floor);
    }
    MenuButton::Row1ButtonPlay => {
      let m = options.speed_modifier_floor;
      if m == 1.0 {
        options.speed_modifier_floor = 0.0;
        state.paused = true;
      } else {
        options.speed_modifier_floor = 1.0;
        state.paused = false;
      }
      log!("pressed time one, from {} to {}", m, options.speed_modifier_floor);
    }
    MenuButton::Row1Button2x => {
      let m = options.speed_modifier_floor;
      options.speed_modifier_floor = 2.0;
      log!("pressed time two, from {} to {}", m, options.speed_modifier_floor);
    }
    MenuButton::Row1ButtonPlus => {
      let m = options.speed_modifier_floor;
      options.speed_modifier_floor = options.speed_modifier_floor.max(2.0) * 1.5;
      log!("pressed time plus, from {} to {}", m, options.speed_modifier_floor);
    }
    MenuButton::Row2Button0 => {
      log!("pressed blow button. blowing the localStorage cache");
      let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
      local_storage.remove_item("factini.options").unwrap();
      local_storage.remove_item("factini.lastMap").unwrap();
      local_storage.remove_item("factini.save0").unwrap();
      local_storage.remove_item("factini.save1").unwrap();
      local_storage.remove_item("factini.save2").unwrap();
      local_storage.remove_item("factini.save3").unwrap();
      log!("Done! Must reload to take effect");
    }
    MenuButton::Row2Button1 => {
      // Unbelt
      log!("Removing all belts from the factory");
      for coord in 0..factory.floor.len() {
        let (x, y) = to_xy(coord);
        match factory.floor[coord].kind {
          CellKind::Belt => factory.floor[coord] = empty_cell(config, x, y),
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
    MenuButton::Row2Button2 => {
      // Unpart
      log!("Removing all part data from the factory");
      unpart(options, state, config, factory, false);
    }
    MenuButton::Row2Button3 => {
      // Undir
      log!("Applying undir...");
      for coord in 0..factory.floor.len() {
        let (x, y) = to_xy(coord);
        if factory.floor[coord].kind != CellKind::Supply && factory.floor[coord].kind != CellKind::Demand {
          if factory.floor[coord].port_u != Port::None {
            cell_set_port_u_to(options, state, config, factory, coord, Port::Unknown, to_coord_up(coord));
          }
          if factory.floor[coord].port_r != Port::None {
            cell_set_port_r_to(options, state, config, factory, coord, Port::Unknown, to_coord_right(coord));
          }
          if factory.floor[coord].port_d != Port::None {
            cell_set_port_d_to(options, state, config, factory, coord, Port::Unknown, to_coord_down(coord));
          }
          if factory.floor[coord].port_l != Port::None {
            cell_set_port_l_to(options, state, config, factory, coord, Port::Unknown, to_coord_left(coord));
          }
        }
      }
      // This should trigger the auto-porting
      factory.changed = true;
    }
    MenuButton::Row2Button4 => {
      // Sample
      log!("pressed sample button");
      state.load_example_next_frame = true;
    }
    MenuButton::Row2Button5 => {
      log!("(no button here)");
    }
    MenuButton::Row2Button6 => {
      log!("(no button here)");
    }
    MenuButton::Row3Button0 => {
      log!("pressed toggle for mouse / touch event swapping, will be {}", !state.event_type_swapped);
      state.event_type_swapped = !state.event_type_swapped;
    }
    MenuButton::Row3Button1 => {
      // Select
      log!("Toggle selection mode");
      state.mouse_mode_selecting = !state.mouse_mode_selecting;
      cell_selection.area = state.mouse_mode_selecting;
      cell_selection.on = false;
      state.selected_area_copy = vec!(); // Or retain this?
    }
    MenuButton::Row3Button2 => {
      log!("Copy selection");
      if state.mouse_mode_selecting && cell_selection.on {
        // If there's no clipboard, fill it now. Otherwise clear the clipboard.
        if state.selected_area_copy.len() == 0 {
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
        else {
          state.selected_area_copy = vec!();
        }
      }
    }
    MenuButton::Row3Button3 => {
      log!("(no button here)");
    }
    MenuButton::Row3Button4 => {
      log!("Clearing the unlock status so you can start again");
      let available_parts = config_get_initial_unlocks(options, state, config);
      factory.available_parts_rhs_menu = available_parts.iter().map(|icon| ( part_icon_to_kind(config,*icon), true ) ).collect();
      factory.quests = get_fresh_quest_states(options, state, config, 0, &available_parts.iter().map(|icon| part_icon_to_kind(config,*icon) ).collect());
      factory.changed = true;
    }
    MenuButton::Row3Button5 => {
      panic!("Hit the panic button. Or another button without implementation.")
    }
    MenuButton::Row3Button6 => {
      log!("(no button here)");
    }
  }
}
fn on_down_top_bar() {

}
fn on_up_top_bar(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, mouse_state: &MouseState) {
  if !bounds_check(mouse_state.last_up_world_x, mouse_state.last_up_world_y, UI_DAY_PROGRESS_OFFSET_X, UI_DAY_PROGRESS_OFFSET_Y, UI_DAY_PROGRESS_OFFSET_X + UI_DAY_PROGRESS_WIDTH, UI_DAY_PROGRESS_OFFSET_Y + UI_DAY_PROGRESS_HEIGHT) {
    log!("Up inside bar zone but not over the actual bar");
    return;
  }
  if !bounds_check(mouse_state.last_down_world_x, mouse_state.last_down_world_y, UI_DAY_PROGRESS_OFFSET_X, UI_DAY_PROGRESS_OFFSET_Y, UI_DAY_PROGRESS_OFFSET_X + UI_DAY_PROGRESS_WIDTH, UI_DAY_PROGRESS_OFFSET_Y + UI_DAY_PROGRESS_HEIGHT) {
    // Dragged onto this button but did not start on this button so ignore the up.
    log!("Up on the bar but wasn't down on the bar");
    return;
  }
  day_reset(options, state, config, factory);
}
fn day_reset(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory) {
  log!("Resetting day... any time now!");

  unpart(options, state, config, factory, true);
  factory_reset_stats(options, state, factory);
  factory.last_day_start = factory.ticks;
  factory.modified_at = 0;
  factory.finished_at = 0;
  factory.finished_with = 0;
}
fn on_down_craft_menu() {

}
fn on_drag_start_craft(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, mouse_state: &MouseState, cell_selection: &CellSelection) {
  log!("is_drag_start from craft popup (before erase/selection check); kind={:?}", mouse_state.craft_down_ci_part_kind);

  // If this was dragging from a machine cell, clear that machine input at this index
  if mouse_state.craft_down_ci == CraftInteractable::InputCell {
    let selected_main_coord = factory.floor[cell_selection.coord].machine.main_coord;
    let index = mouse_state.craft_down_ci_index as usize - 100;
    log!("Clearing input @{} from machine @{} because drag start; has {} wants and {} haves", index, selected_main_coord, factory.floor[selected_main_coord].machine.wants.len(), factory.floor[selected_main_coord].machine.haves.len());

    machine_change_want_kind(options, state, config, factory, selected_main_coord, index, PARTKIND_NONE);
    // Make sure the haves are cleared as well
    factory.floor[selected_main_coord].machine.haves[index] = part_none(config);
  }
}
fn on_drag_end_craft(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState) {
  log!("on_drag_end_craft()");

  // If this was dragging from a machine cell or resource button and dropped on a machine
  // cell then set that machine cell. Otherwise ignore it. This may cause an input to stay clear
  if mouse_state.craft_down_ci == CraftInteractable::InputCell || mouse_state.craft_down_ci == CraftInteractable::Resource {
    if mouse_state.craft_up_ci == CraftInteractable::InputCell {
      let selected_main_coord = factory.floor[cell_selection.coord].machine.main_coord;
      let index = mouse_state.craft_up_ci_index as usize - 100;
      log!("Setting input @{} from machine @{} because drag start; has {} wants and {} haves", index, selected_main_coord, factory.floor[selected_main_coord].machine.wants.len(), factory.floor[selected_main_coord].machine.haves.len());
      machine_change_want_kind(options, state, config, factory, selected_main_coord, index, mouse_state.craft_down_ci_part_kind);
      // Clear the haves to make sure it doesn't contain an incompatible part now
      factory.floor[selected_main_coord].machine.haves[index] = part_from_part_index(config, mouse_state.craft_down_ci_part_kind);
    }
    else {
      log!("  Did not end on an input cell, ignoring");
    }
  }
  else {
    log!("  Did not start dragging from an input cell or resource, ignoring");
  }
}
fn on_up_selecting(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, mouse_state: &mut MouseState, cell_selection: &mut CellSelection) {
  log!("mouse up on floor with selection mode enabled...");
  if mouse_state.down_zone == ZONE_FLOOR {
    // Moving while there's stuff on the clipboard? This mouse up is a paste / stamp.
    if state.selected_area_copy.len() > 0 {
      log!("    clipboard has data so we stamp it now");
      paste(options, state, config, factory, mouse_state.cell_x_floored as usize, mouse_state.cell_y_floored as usize);
    }
    else {
      log!("  was down in floor, too. ok!");
      let now_cell_x = mouse_state.cell_x_floored;
      let now_cell_y = mouse_state.cell_y_floored;
      let down_cell_x = mouse_state.last_down_cell_x_floored;
      let down_cell_y = mouse_state.last_down_cell_y_floored;

      cell_selection.x = down_cell_x.min(now_cell_x);
      cell_selection.y = down_cell_y.min(now_cell_y);
      cell_selection.x2 = down_cell_x.max(now_cell_x);
      cell_selection.y2 = down_cell_y.max(now_cell_y);
      cell_selection.on = true;
      cell_selection.coord = to_coord(cell_selection.x as usize, cell_selection.y as usize);
    }
  } else {
    log!("mouse up with selection mode enabled but the down was not on the floor, ignoring");
  }
}

fn hit_test_menu_button(x: f64, y: f64) -> (MenuButton, MenuRow) {
  // The menu is three rows of buttons. The top row has circular buttons, the bottom two are rects.

  // Was one of the buttons below the floor clicked?
  if bounds_check(x, y, UI_MENU_BUTTONS_OFFSET_X, UI_MENU_BUTTONS_OFFSET_Y, UI_MENU_BUTTONS_OFFSET_X + UI_MENU_BUTTONS_WIDTH_MAX, UI_MENU_BUTTONS_OFFSET_Y + UI_MENU_BUTTONS_HEIGHT) {
    let button_index = (x - UI_MENU_BUTTONS_OFFSET_X) / (UI_MENU_BUTTONS_WIDTH + UI_MENU_BUTTONS_SPACING);
    if button_index % 1.0 < (UI_MENU_BUTTONS_WIDTH / (UI_MENU_BUTTONS_WIDTH + UI_MENU_BUTTONS_SPACING)) {
      let button = match button_index.floor() as u8 {
        0 => MenuButton::Row2Button0,
        1 => MenuButton::Row2Button1,
        2 => MenuButton::Row2Button2,
        3 => MenuButton::Row2Button3,
        4 => MenuButton::Row2Button4,
        5 => MenuButton::Row2Button5,
        6 => MenuButton::Row2Button6,
        _ => panic!("what button was clicked?"),
      };

      ( button, MenuRow::Second )
    } else {
      ( MenuButton::None, MenuRow::None )
    }
  }
  // Second row of buttons?
  else if bounds_check(x, y, UI_MENU_BUTTONS_OFFSET_X, UI_MENU_BUTTONS_OFFSET_Y2, UI_MENU_BUTTONS_OFFSET_X + UI_MENU_BUTTONS_WIDTH_MAX, UI_MENU_BUTTONS_OFFSET_Y2 + UI_MENU_BUTTONS_HEIGHT) {
    let button_index = (x - UI_MENU_BUTTONS_OFFSET_X) / (UI_MENU_BUTTONS_WIDTH + UI_MENU_BUTTONS_SPACING);
    if button_index % 1.0 < (UI_MENU_BUTTONS_WIDTH / (UI_MENU_BUTTONS_WIDTH + UI_MENU_BUTTONS_SPACING)) {
      let button = match button_index.floor() as u8 {
        0 => MenuButton::Row3Button0,
        1 => MenuButton::Row3Button1,
        2 => MenuButton::Row3Button2,
        3 => MenuButton::Row3Button3,
        4 => MenuButton::Row3Button4,
        5 => MenuButton::Row3Button5,
        6 => MenuButton::Row3Button6,
        _ => panic!("what button was clicked?"),
      };

      ( button, MenuRow::Third )
    } else {
      ( MenuButton::None, MenuRow::None )
    }
  }
  // Any of the speed bubbles? (most expensive to check)
  else if bounds_check(
    x, y,
    UI_SPEED_BUBBLE_OFFSET_X,
    UI_SPEED_BUBBLE_OFFSET_Y,
    UI_SPEED_BUBBLE_OFFSET_X + 5.0 * (2.0 * UI_SPEED_BUBBLE_RADIUS) + 4.0 * UI_SPEED_BUBBLE_SPACING,
    UI_SPEED_BUBBLE_OFFSET_Y + (2.0 * UI_SPEED_BUBBLE_RADIUS)
  ) {
    if hit_check_speed_bubble_x(x, y, 0) {
      ( MenuButton::Row1ButtonMin, MenuRow::First )
    } else if hit_check_speed_bubble_x(x, y, 1) {
      ( MenuButton::Row1ButtonHalf, MenuRow::First )
    } else if hit_check_speed_bubble_x(x, y, 2) {
      ( MenuButton::Row1ButtonPlay, MenuRow::First )
    } else if hit_check_speed_bubble_x(x, y, 3) {
      ( MenuButton::Row1Button2x, MenuRow::First )
    } else if hit_check_speed_bubble_x(x, y, 4) {
      ( MenuButton::Row1ButtonPlus, MenuRow::First )
    } else {
      ( MenuButton::None, MenuRow::None )
    }
  }
  else {
    ( MenuButton::None, MenuRow::None )
  }
}
fn hit_test_get_craft_interactable_machine_at(options: &Options, state: &State, factory: &Factory, cell_selection: &CellSelection, mwx: f64, mwy: f64) -> ( CraftInteractable, f64, f64, f64, f64, char, PartKind, u8 ) {
  // Figure out whether any of the interactables were clicked

  let selected_coord = cell_selection.coord;
  assert!(factory.floor[selected_coord].kind == CellKind::Machine, "should be checked earlier");

  // Each cell consolidates much of its information into the main coord, the top-left cell
  let selected_main_coord = factory.floor[selected_coord].machine.main_coord;
  let (selected_main_x, selected_main_y) = to_xy(selected_main_coord);

  let machine_wx = UI_FLOOR_OFFSET_X + (selected_main_x as f64) * CELL_W;
  let machine_wy = UI_FLOOR_OFFSET_Y + (selected_main_y as f64) * CELL_H;

  let machine_cw = factory.floor[selected_main_coord].machine.cell_width as f64;
  let machine_ch = factory.floor[selected_main_coord].machine.cell_height as f64;
  let machine_ww = machine_cw * CELL_W;
  let machine_wh = machine_ch * CELL_H;

  // Find the center of the machine because .arc() requires the center x,y
  let ( center_wx, center_wy, cr ) = get_machine_selection_circle_params(factory, selected_main_coord);

  if mwx >= machine_wx && mwx < machine_wx + machine_ww && mwy >= machine_wy && mwy < machine_wy + machine_wh {
    // log!("testing {} {} {}", machine_wy, mwy, ((machine_wy - mwy) / CELL_H));
    // Bit of a hack but we convert the in-machine coordinate to a linear index and set the icon to that number.
    let index = (((mwy - machine_wy) / CELL_H).floor() * machine_cw) + ((mwx - machine_wx) / CELL_W).floor();

    // Clicked inside machine. Determine cell and delete it.
    // log!("Clicked on a cell of the actual machine. Now determine the input cell and clear it. (TODO)");
    return ( CraftInteractable::InputCell, machine_wx, machine_wy, CELL_W, CELL_H, factory.floor[selected_main_coord].machine.wants[index as usize].icon, factory.floor[selected_main_coord].machine.wants[index as usize].kind, 100 + (index as u8) );
  }

  // Minimal distance of painting interactbles is the distance from the center to the furthest
  // angle (any machine corner) plus a small buffer. a^2+b^2=c^2
  // This radius determines the distance from the center of the circle to the _center_ of the cell.
  let minr = ((center_wx - machine_wx).powf(2.0) + (center_wy - machine_wy).powf(2.0)).powf(0.5) + 30.0;

  // The back/close button should always be under the machine, centered. Same size (one cell).
  let close_wx = center_wx - CELL_W / 2.0;
  let close_wy = center_wy + minr - CELL_H / 2.0;
  if bounds_check(mwx, mwy, close_wx, close_wy, close_wx + CELL_W, close_wy + CELL_H) {
    // log!("Clicked the back/close button. (TODO)");
    return ( CraftInteractable::BackClose, close_wx, close_wy, CELL_W, CELL_H, '#', PARTKIND_NONE, 99 );
  }

  // Actual number of seen inputs
  let len = factory.floor[selected_main_coord].machine.last_received.len();
  // Make sure that we always show something. If there aren't any elements, show trash as the only icon.
  let count = len.max(1);

  let angle_step = 5.5 - (count as f64 / 2.0).ceil() + (0.5 * ((count % 2) as f64));
  for i in 0..count {
    let r = hit_test_get_craft_interactable_machine_at_index(angle_step, minr, center_wx, center_wy, mwx, mwy, i, if len == 0 { 't' } else { factory.floor[selected_main_coord].machine.last_received[i].0.icon }, if len == 0 { PARTKIND_TRASH } else { factory.floor[selected_main_coord].machine.last_received[i].0.kind });
    if let Some(x) = r {
      return x;
    }
  }

  // log!("Clicked inside machine circle but did not hit any interactables");
  return ( CraftInteractable::None, 0.0, 0.0, 0.0, 0.0, '#', PARTKIND_NONE, 99 );
}
fn hit_test_get_craft_interactable_machine_at_index(angle_step: f64, minr: f64, center_wx: f64, center_wy: f64, mwx: f64, mwy: f64, craft_index: usize, icon: char, part_index: PartKind) -> Option< (CraftInteractable, f64, f64, f64, f64, char, PartKind, u8 ) > {
  let angle: f64 = (angle_step + craft_index as f64) * 0.1 * std::f64::consts::TAU;

  // TODO: could pre-compute these coords per factory and read the coords from a vec
  let btn_c_wx = angle.sin() * minr;
  let btn_c_wy = angle.cos() * minr;
  let wx = center_wx + btn_c_wx - CELL_W / 2.0;
  let wy = center_wy + btn_c_wy - CELL_H / 2.0;

  if bounds_check(mwx, mwy, wx, wy, wx + CELL_W, wy + CELL_H) {
    // log!("Clicked resource box {}. (TODO)", i);
    return Some( ( CraftInteractable::Resource, btn_c_wx, btn_c_wy, CELL_W, CELL_H, icon, part_index, craft_index as u8 ) );
  }

  return None;
}

fn hit_test_offers(factory: &Factory, mx: f64, my: f64) -> (bool, usize ) {
  if bounds_check(mx, my, UI_OFFERS_OFFSET_X, UI_OFFERS_OFFSET_Y, UI_OFFERS_OFFSET_X + UI_OFFERS_WIDTH_PLUS_MARGIN * UI_OFFERS_PER_ROW, UI_OFFERS_OFFSET_Y + UI_OFFERS_HEIGHT_PLUS_MARGIN * (factory.available_parts_rhs_menu.len() as f64 / UI_OFFERS_PER_ROW).ceil()) {
    let inside_offer_and_margin_x = (mx - UI_OFFERS_OFFSET_X) / UI_OFFERS_WIDTH_PLUS_MARGIN;
    if (mx - UI_OFFERS_OFFSET_X) - (inside_offer_and_margin_x.floor() * UI_OFFERS_WIDTH_PLUS_MARGIN) > UI_OFFERS_WIDTH {
      // In the horizontal margin. Miss.
      return ( false, 0 );
    }
    let inside_offer_and_margin_y = (my - UI_OFFERS_OFFSET_Y) / UI_OFFERS_HEIGHT_PLUS_MARGIN;
    if (my - UI_OFFERS_OFFSET_Y) - (inside_offer_and_margin_y.floor() * UI_OFFERS_HEIGHT_PLUS_MARGIN) > UI_OFFERS_HEIGHT {
      // In the vertical margin. Miss.
      return ( false, 0 );
    }

    let inside_offer_and_margin_index = (inside_offer_and_margin_x.floor() + inside_offer_and_margin_y.floor() * UI_OFFERS_PER_ROW) as usize;

    let mut count = 0;
    for i in 0..factory.available_parts_rhs_menu.len() {
      if factory.available_parts_rhs_menu[i].1 {
        if count == inside_offer_and_margin_index {
          return ( true, i );
        }
        count += 1;
      }
    }

    return ( false, 0 );
  } else {
    return ( false, 0 );
  };
}
fn hit_test_paint_toggle(x: f64, y: f64) -> bool {
  return bounds_check(x, y, UI_MENU_BOTTOM_PAINT_TOGGLE_X, UI_MENU_BOTTOM_PAINT_TOGGLE_Y, UI_MENU_BOTTOM_PAINT_TOGGLE_X + UI_MENU_BOTTOM_PAINT_TOGGLE_WIDTH, UI_MENU_BOTTOM_PAINT_TOGGLE_Y + UI_MENU_BOTTOM_PAINT_TOGGLE_HEIGHT);
}
fn hit_test_machine_button(x: f64, y: f64) -> bool {
  return bounds_check(x, y, UI_MENU_BOTTOM_MACHINE_X, UI_MENU_BOTTOM_MACHINE_Y, UI_MENU_BOTTOM_MACHINE_X + UI_MENU_BOTTOM_MACHINE_WIDTH, UI_MENU_BOTTOM_MACHINE_Y + UI_MENU_BOTTOM_MACHINE_HEIGHT);
}
fn hit_test_help_button(mx: f64, my: f64) -> bool {
  return bounds_check(mx, my, UI_HELP_X, UI_HELP_Y, UI_HELP_X + UI_HELP_WIDTH, UI_HELP_Y + UI_HELP_HEIGHT);
}
fn ray_trace_dragged_line_expensive(factory: &Factory, ix0: f64, iy0: f64, ix1: f64, iy1: f64, for_preview: bool) -> Vec<((usize, usize), BeltType, Direction, Direction)> {
  // We raytracing
  // The dragged line becomes a ray that we trace through cells of the floor
  // We then generate a belt track such that it fits in with the existing belts, if any
  // - Figure out which cells the ray passes through
  // - If the ray crosses existing belts, generate the belt type as if the original was modified to support the new path (the pathing would not destroy existing ports)
  // - If the ray only spans one cell, force it to be invalid
  // - The first and last cells in the ray also auto-connect to any neighbor belts. Sections in the middle of the ray do not.
  // - Special case: if the line starts on an edge but finishes away from that same edge, force the second step to be away from that edge. there's some manual logic to make that work.

  // Check start of path and compensate if on edge
  let x_left0 = ix0 == 0.0 && ix1 != 0.0;
  let y_top0 = iy0 == 0.0 && iy1 != 0.0;
  let x_right0 = ix0 == ((FLOOR_CELLS_W - 1) as f64) && ix1 != ((FLOOR_CELLS_W - 1) as f64);
  let y_bottom0 = iy0 == ((FLOOR_CELLS_H - 1) as f64) && iy1 != ((FLOOR_CELLS_H - 1) as f64);
  let x0 = if x_left0 { ix0 + 1.0 } else if x_right0 { ix0 - 1.0 } else { ix0 };
  let y0 = if y_top0 { iy0 + 1.0 } else if y_bottom0 { iy0 - 1.0 } else { iy0 };

  // Check end of path and compensate if on edge
  let x_left1 = ix1 == 0.0 && x0 != 0.0;
  let y_top1 = iy1 == 0.0 && y0 != 0.0;
  let x_right1 = ix1 == ((FLOOR_CELLS_W - 1) as f64) && x0 != ((FLOOR_CELLS_W - 1) as f64);
  let y_bottom1 = iy1 == ((FLOOR_CELLS_H - 1) as f64) && y0 != ((FLOOR_CELLS_H - 1) as f64);
  let x1 = if x_left1 { ix1 + 1.0 } else if x_right1 { ix1 - 1.0 } else { ix1 };
  let y1 = if y_top1 { iy1 + 1.0 } else if y_bottom1 { iy1 - 1.0 } else { iy1 };

  let mut covered = get_cells_from_a_to_b(x0, y0, x1, y1);
  assert!(covered.len() >= 1, "Should always record at least one cell coord");

  // Now put the start/end of path back if it was moved. This way the path will never have more than one edge cell on the same side
  if x_left0 || y_top0 || x_right0 || y_bottom0 {
    // "push_front"
    let mut t = vec!((ix0 as usize, iy0 as usize));
    t.append(&mut covered);
    covered = t;
  }
  if x_left1 || y_top1 || x_right1 || y_bottom1 {
    covered.push((ix1 as usize, iy1 as usize));
  }

  if covered.len() == 1 {
    return vec!((covered[0], BeltType::INVALID, Direction::Up, Direction::Up));
  }

  // Note: in order of (dragging) appearance
  let mut track: Vec<((usize, usize), BeltType, Direction, Direction)> = vec!(); // ((x, y), new_bt)

  let (mut lx, mut ly) = covered[0];
  let mut last_from = Direction::Up; // first one ignores this value

  // Draw example tiles of the path you're drawing.
  // Take the existing cell and add one (first/last segment) or two ports to it;
  // - first one only gets the "to" port added to it
  // - last one only gets the "from" port added to it
  // - middle parts get the "from" and "to" port added to them
  for index in 1..covered.len() {
    let (x, y) = covered[index];
    // Always set the previous one.
    let new_from = get_from_dir_between_xy(lx, ly, x, y);
    let last_to = direction_reverse(new_from);
    let bt =
      if track.len() == 0 {
        add_unknown_port_to_cell(factory, to_coord(lx, ly), last_to)
      } else {
        add_two_ports_to_cell(factory, to_coord(lx, ly), last_from, last_to)
      };
    track.push(((lx, ly), bt, last_from, last_to)); // Note: first segment has undefined "from"

    lx = x;
    ly = y;
    last_from = new_from;
  }
  // Final step. Only has a from port.
  let bt = add_unknown_port_to_cell(factory, to_coord(lx, ly), last_from);
  track.push(((lx, ly), bt, last_from, last_from)); // Note: last segment has undefined "from"

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
fn hit_check_speed_bubble_x(x: f64, y: f64, index: usize) -> bool {
  let diameter = 2.0 * UI_SPEED_BUBBLE_RADIUS;
  let ox = UI_SPEED_BUBBLE_OFFSET_X + (index as f64) * (diameter + UI_SPEED_BUBBLE_SPACING);

  return bounds_check(
    x, y,
    ox,
    UI_SPEED_BUBBLE_OFFSET_Y,
    ox + diameter,
    UI_SPEED_BUBBLE_OFFSET_Y + diameter
  );
}

fn paint_debug_app(options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, fps: &VecDeque<f64>, now: f64, since_prev: f64, ticks_todo: u64, estimated_fps: f64, rounded_fps: u64, factory: &Factory, mouse_state: &MouseState) {

  let mut ui_lines = 0.0;

  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_APP_OFFSET_X, UI_DEBUG_APP_OFFSET_Y + (UI_DEBUG_APP_LINE_H * ui_lines), UI_DEBUG_APP_WIDTH, UI_DEBUG_APP_LINE_H);
  context.set_fill_style(&"black".into());
  context.fill_text(format!("fps: {}", fps.len()).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_APP_OFFSET_X, UI_DEBUG_APP_OFFSET_Y + (UI_DEBUG_APP_LINE_H * ui_lines), UI_DEBUG_APP_WIDTH, UI_DEBUG_APP_LINE_H);
  context.set_fill_style(&"black".into());
  context.fill_text(format!("App time  : {}", (now / 1000.0).floor()).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");
  // context.fill_text(format!("color  : {:?}", get_drop_color(options, factory.ticks)).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_APP_OFFSET_X, UI_DEBUG_APP_OFFSET_Y + (UI_DEBUG_APP_LINE_H * ui_lines), UI_DEBUG_APP_WIDTH, UI_DEBUG_APP_LINE_H);
  context.set_fill_style(&"black".into());
  context.fill_text(format!("Since prev: {} (@{})", since_prev.floor(), estimated_fps).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_APP_OFFSET_X, UI_DEBUG_APP_OFFSET_Y + (UI_DEBUG_APP_LINE_H * ui_lines), UI_DEBUG_APP_WIDTH, UI_DEBUG_APP_LINE_H);
  context.set_fill_style(&"black".into());
  context.fill_text(format!("Ticks todo: {} (r? {})", ticks_todo, rounded_fps).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_APP_OFFSET_X, UI_DEBUG_APP_OFFSET_Y + (UI_DEBUG_APP_LINE_H * ui_lines), UI_DEBUG_APP_WIDTH, UI_DEBUG_APP_LINE_H);
  context.set_fill_style(&"black".into());
  context.fill_text(format!("Speed: floor = {}, ui = {}", options.speed_modifier_floor, options.speed_modifier_ui).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_APP_OFFSET_X, UI_DEBUG_APP_OFFSET_Y + (UI_DEBUG_APP_LINE_H * ui_lines), UI_DEBUG_APP_WIDTH, UI_DEBUG_APP_LINE_H);
  context.set_fill_style(&"black".into());
  context.fill_text(format!("mouse over : {:?}", mouse_state.over_zone).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_APP_OFFSET_X, UI_DEBUG_APP_OFFSET_Y + (UI_DEBUG_APP_LINE_H * ui_lines), UI_DEBUG_APP_WIDTH, UI_DEBUG_APP_LINE_H);
  context.set_fill_style(&"black".into());
  context.fill_text(
    format!(
      "mouse abs  : {} x {} {} {}",
      mouse_state.world_x, mouse_state.world_y,
      if mouse_state.is_dragging { "drag" } else if mouse_state.is_down { "down" } else { "up" },
      mouse_state.last_down_button,
    ).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H
  ).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_APP_OFFSET_X, UI_DEBUG_APP_OFFSET_Y + (UI_DEBUG_APP_LINE_H * ui_lines), UI_DEBUG_APP_WIDTH, UI_DEBUG_APP_LINE_H);
  context.set_fill_style(&"black".into());
  context.fill_text(format!("mouse world: {} x {}", mouse_state.cell_x_floored, mouse_state.cell_y_floored).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_APP_OFFSET_X, UI_DEBUG_APP_OFFSET_Y + (UI_DEBUG_APP_LINE_H * ui_lines), UI_DEBUG_APP_WIDTH, UI_DEBUG_APP_LINE_H);
  context.set_fill_style(&"black".into());
  context.fill_text(format!("mouse cell : {:.2} x {:.2}", mouse_state.cell_x - mouse_state.cell_x_floored, mouse_state.cell_y - mouse_state.cell_y_floored).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_APP_OFFSET_X, UI_DEBUG_APP_OFFSET_Y + (UI_DEBUG_APP_LINE_H * ui_lines), UI_DEBUG_APP_WIDTH, UI_DEBUG_APP_LINE_H);
  context.set_fill_style(&"black".into());
  context.fill_text(format!("mouse coord : {}", if mouse_state.cell_x_floored < 0.0 || mouse_state.cell_y_floored < 0.0 || mouse_state.cell_x_floored >= FLOOR_CELLS_W as f64 || mouse_state.cell_y_floored >= FLOOR_CELLS_W as f64 { "oob".to_string() } else { format!("{}", to_coord(mouse_state.cell_x_floored as usize, mouse_state.cell_y_floored as usize)) }).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_APP_OFFSET_X, UI_DEBUG_APP_OFFSET_Y + (UI_DEBUG_APP_LINE_H * ui_lines), UI_DEBUG_APP_WIDTH, UI_DEBUG_APP_LINE_H);
  context.set_fill_style(&"black".into());
  context.fill_text(format!("down: {}, dragging: {}", mouse_state.is_down, mouse_state.is_dragging).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_APP_OFFSET_X, UI_DEBUG_APP_OFFSET_Y + (UI_DEBUG_APP_LINE_H * ui_lines), UI_DEBUG_APP_WIDTH, UI_DEBUG_APP_LINE_H);
  context.set_fill_style(&"black".into());
  context.fill_text(format!("down event type: {}", if mouse_state.last_down_event_type == EventSourceType::Mouse { "Mouse" } else { "Touch" }).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

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
fn paint_supply_and_part_for_edge(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, cx: usize, cy: usize, part_index: PartKind) {
  let ox = UI_FLOOR_OFFSET_X + CELL_W * (cx as f64);
  let oy = UI_FLOOR_OFFSET_Y + CELL_H * (cy as f64);
  let dock_target =
    if cy == 0 {
      CONFIG_NODE_SUPPLY_UP
    } else if cx == FLOOR_CELLS_W-1 {
      CONFIG_NODE_SUPPLY_RIGHT
    } else if cy == FLOOR_CELLS_H-1 {
      CONFIG_NODE_SUPPLY_DOWN
    } else if cx == 0 {
      CONFIG_NODE_SUPPLY_LEFT
    } else {
      panic!("no");
    };
  // TODO: should we offer the option to draw the dock behind in case of semi-transparent supply imgs?
  context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
    &config.sprite_cache_canvas[config.nodes[dock_target].sprite_config.frames[0].file_canvas_cache_index],
    config.nodes[dock_target].sprite_config.frames[0].x, config.nodes[dock_target].sprite_config.frames[0].y, config.nodes[dock_target].sprite_config.frames[0].w, config.nodes[dock_target].sprite_config.frames[0].h,
    ox, oy, CELL_W, CELL_H
  ).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
  paint_segment_part_from_config(options, state, config, context, part_index, ox + CELL_W/4.0, oy + CELL_H/4.0, CELL_W/2.0, CELL_H/2.0);
}
fn paint_supply_and_part_not_edge(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, ox: f64, oy: f64, part_index: PartKind) {
  paint_segment_part_from_config(options, state, config, context, part_index, ox + CELL_W*0.13, oy + CELL_H*0.13, CELL_W*0.75, CELL_H*0.75);
}
fn paint_part_and_pattern_at_middle(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, main_coord: usize, part_index: PartKind) {
  // current coord must be a machine. Paint the pattern of the part we're currently dragging.
  let (mx, my) = to_xy(main_coord);
  let mwc = factory.floor[main_coord].machine.cell_width;
  let mhc = factory.floor[main_coord].machine.cell_height;
  let mw = mwc as f64 * CELL_W;
  let mh = mhc as f64 * CELL_H;
  let margin_w = mw * 0.125; // the pattern takes up 75% of machine, use remaining 25% for padding (12.5% each side)
  let margin_h = mh * 0.125;
  let pw = mw / 3.0 * 0.75; // paint pattern on a 3x3 grid on 75% of machine size
  let ph = mh / 3.0 * 0.75;

  // Paint the pattern for this part, slightly smaller than the box for the machine, regardless of its size
  context.set_fill_style(&"#ffffff55".into());
  context.fill_rect(UI_FLOOR_OFFSET_X + CELL_W * (mx as f64) + mw * 0.1, UI_FLOOR_OFFSET_Y + CELL_H * (my as f64) + mh * 0.1, mw * 0.80, mh * 0.80);

  // Get the pattern of what you're draggin. Paint it over the machine.
  for i in 0..config.nodes[part_index].pattern_by_index.len() {
    paint_segment_part_from_config(
      options, state, config, context,
      config.nodes[part_index].pattern_by_index[i],
      UI_FLOOR_OFFSET_X + CELL_W * (mx as f64) + margin_w + pw * (i%3) as f64, UI_FLOOR_OFFSET_Y + CELL_H * (my as f64) + margin_h + ph * (i/3) as f64,
      pw, ph
    );
  }
}
fn paint_supply_and_part_not_floor(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, ox: f64, oy: f64, part_index: PartKind) {
  paint_segment_part_from_config(options, state, config, context, part_index, ox + CELL_W*0.13, oy + CELL_H*0.13, CELL_W*0.75, CELL_H*0.75);
}
fn paint_dock_stripes(
  options: &Options,
  state: &State,
  config: &Config,
  context: &Rc<web_sys::CanvasRenderingContext2d>,
  dock_target: usize,
  ox: f64,
  oy: f64,
  w: f64,
  h: f64,
) {
  // Paint the loading docks, which is where the suppliers and demanders can go
  context.set_global_alpha(0.5); // TODO: the alpha should probably be governed by the image (semi-trans) or a configurable setting...
  // Paint the dock image for non-corner edge cells
  context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
    &config.sprite_cache_canvas[config.nodes[dock_target].sprite_config.frames[0].file_canvas_cache_index],
    config.nodes[dock_target].sprite_config.frames[0].x, config.nodes[dock_target].sprite_config.frames[0].y, config.nodes[dock_target].sprite_config.frames[0].w, config.nodes[dock_target].sprite_config.frames[0].h,
    ox, oy, w, h
  ).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
  context.set_global_alpha(1.0);
}
fn paint_background_tiles1(
  options: &Options,
  state: &State,
  config: &Config,
  context: &Rc<web_sys::CanvasRenderingContext2d>,
  factory: &Factory,
  img_machine2: &web_sys::HtmlImageElement,
  img_machine_1_1: &web_sys::HtmlImageElement,
  img_machine_2_1: &web_sys::HtmlImageElement,
  img_machine_3_2: &web_sys::HtmlImageElement,
) {
  // Paint background cell tiles
  for coord in 0..FLOOR_CELLS_WH {
    let (cx, cy) = to_xy(coord);

    let ox = UI_FLOOR_OFFSET_X + CELL_W * (cx as f64);
    let oy = UI_FLOOR_OFFSET_Y + CELL_H * (cy as f64);

    let cell = &factory.floor[coord];

    // This is cheating since we defer the loading stuff to the browser. Sue me.
    match cell.kind {
      CellKind::Empty => {
        if (cx == 0 || cx == FLOOR_CELLS_W - 1) && (cy == 0 || cy == FLOOR_CELLS_H - 1) {
          // corners
          continue;
        }

        // edge?
        let dock_target =
          if cy == 0 {
            CONFIG_NODE_DOCK_UP
          } else if cx == FLOOR_CELLS_W - 1 {
            CONFIG_NODE_DOCK_RIGHT
          } else if cy == FLOOR_CELLS_H - 1 {
            CONFIG_NODE_DOCK_DOWN
          } else if cx == 0 {
            CONFIG_NODE_DOCK_LEFT
          } else {
            continue;
          };

        paint_dock_stripes(options, state, config, context, dock_target, ox, oy, CELL_W, CELL_H);
      },
      CellKind::Belt => {
        paint_factory_belt(options, state, config, factory, coord, context, ox, oy, CELL_W, CELL_H);
      },
      CellKind::Machine => {
        // For machines, paint the top-left cell only but make the painted area cover the whole machine
        // TODO: each machine size should have a unique, customized, sprite
        if cell.machine.main_coord == coord {
          let machine_img = match ( cell.machine.cell_width, cell.machine.cell_height ) {
            ( 1, 1 ) => &config.sprite_cache_canvas[config.nodes[CONFIG_NODE_MACHINE_1X1].sprite_config.frames[0].file_canvas_cache_index],
            ( 2, 2 ) => &config.sprite_cache_canvas[config.nodes[CONFIG_NODE_MACHINE_2X2].sprite_config.frames[0].file_canvas_cache_index],
            ( 3, 3 ) => &config.sprite_cache_canvas[config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].file_canvas_cache_index],
            ( 4, 4 ) => img_machine_1_1,
            ( 2, 1 ) => img_machine_2_1,
            ( 4, 2 ) => img_machine_2_1,
            ( 3, 2 ) => img_machine_3_2,
            _ => &config.sprite_cache_canvas[config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].file_canvas_cache_index],
          };
          context.draw_image_with_html_image_element_and_dw_and_dh(machine_img, ox, oy, cell.machine.cell_width as f64 * CELL_W, cell.machine.cell_height as f64 * CELL_H).expect("something error draw_image"); // requires web_sys HtmlImageElement feature

          // Paint tiny output part
          paint_segment_part_from_config(options, state, config, context,
            cell.machine.output_want.kind, ox + config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].x, oy + config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].y,
            config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].w, config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].h
          );
        }
      },
      CellKind::Supply => {
        // Bottom layer: paint the belt so it appears to be part of the supplier
        // We need the animation to line up with other belts so we have to use separate sprite layers
        if cy == 0 {
          paint_belt(options, state, config, context, ox, oy, CELL_H, CELL_H, BeltType::U_D, 0, factory.ticks);
        } else if cx == FLOOR_CELLS_W-1 {
          paint_belt(options, state, config, context, ox, oy, CELL_H, CELL_H, BeltType::R_L, 0, factory.ticks);
        } else if cy == FLOOR_CELLS_H-1 {
          paint_belt(options, state, config, context, ox, oy, CELL_H, CELL_H, BeltType::D_U, 0, factory.ticks);
        } else if cx == 0 {
          paint_belt(options, state, config, context, ox, oy, CELL_H, CELL_H, BeltType::L_R, 0, factory.ticks);
        } else {
          panic!("no");
        };
      }
      CellKind::Demand => {
        // Bottom layer: paint the belt so it appears to be part of the demander
        // We need the animation to line up with other belts so we have to use separate sprite layers
        if cy == 0 {
          paint_belt(options, state, config, context, ox, oy, CELL_H, CELL_H, BeltType::D_U, 0, factory.ticks);
        } else if cx == FLOOR_CELLS_W-1 {
          paint_belt(options, state, config, context, ox, oy, CELL_H, CELL_H, BeltType::L_R, 0, factory.ticks);
        } else if cy == FLOOR_CELLS_H-1 {
          paint_belt(options, state, config, context, ox, oy, CELL_H, CELL_H, BeltType::U_D, 0, factory.ticks);
        } else if cx == 0 {
          paint_belt(options, state, config, context, ox, oy, CELL_H, CELL_H, BeltType::R_L, 0, factory.ticks);
        } else {
          panic!("no");
        };
      }
    }
  }
}
fn paint_background_tiles2(
  options: &Options,
  state: &State,
  config: &Config,
  context: &Rc<web_sys::CanvasRenderingContext2d>,
  factory: &Factory,
  img_machine2: &web_sys::HtmlImageElement,
  img_machine_1_1: &web_sys::HtmlImageElement,
  img_machine_2_1: &web_sys::HtmlImageElement,
  img_machine_3_2: &web_sys::HtmlImageElement,
) {
  // Paint background cell tiles
  for coord in 0..FLOOR_CELLS_WH {
    let (cx, cy) = to_xy(coord);

    let ox = UI_FLOOR_OFFSET_X + CELL_W * (cx as f64);
    let oy = UI_FLOOR_OFFSET_Y + CELL_H * (cy as f64);

    let cell = &factory.floor[coord];

    // This is cheating since we defer the loading stuff to the browser. Sue me.
    match factory.floor[coord].kind {
      CellKind::Empty => {
      },
      CellKind::Belt => {
        let progress_c = ((cell.belt.part_progress as f64) / (cell.belt.speed as f64)).min(1.0);
        let first_half = progress_c < 0.5;

        // Start with the coordinate to paint the icon such that it ends up centered
        // in the target cell.
        // Then increase or decrease one axis depending on the progress the part made.
        let sx = UI_FLOOR_OFFSET_X + CELL_W * (cx as f64) + -(PART_W * 0.5);
        let sy = UI_FLOOR_OFFSET_Y + CELL_H * (cy as f64) + -(PART_H * 0.5);

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

        if paint_segment_part_from_config(options, state, config, context, cell.belt.part.kind, px, py, PART_W, PART_H) {
          // context.set_font(&"8px monospace");
          // context.set_fill_style(&"green".into());
          // context.fill_text(format!("{} {}x{}", coord, x, y).as_str(), px + 3.0, py + 10.0).expect("something error fill_text");
          // context.fill_text(format!("{}", progress_c).as_str(), px + 3.0, py + 21.0).expect("something error fill_text");
        }
      },
      CellKind::Machine => {
      },
      CellKind::Supply => {
        // Paint part on top of belts
        let supply = &cell.supply;
        if supply.part_created_at > 0 {
          let p = supply.part_progress.min(supply.speed.max(1)) as f64 / supply.speed.max(1) as f64;

          let mut dx = ox + (CELL_W - PART_W) * 0.5;
          let mut dy = oy + (CELL_H - PART_H) * 0.5;

          if cy == 0 {
            dy += CELL_H * 0.5 * p;
          } else if cx == FLOOR_CELLS_W-1 {
            dx -= CELL_W * 0.5 * p;
          } else if cy == FLOOR_CELLS_H-1 {
            dy -= CELL_H * 0.5 * p;
          } else if cx == 0 {
            dx += CELL_W * 0.5 * p;
          } else {
            panic!("no");
          };
          paint_segment_part_from_config(options, state, config, context, supply.gives.kind, dx, dy, PART_W, PART_H);
        }
      }
      CellKind::Demand => {
        // TODO: painting the part will require some modifications to the demander to determine progress

        // paint demand.last_part_kind for as long as we want to
        // the demand speed is the number of ticks it takes for a part to move from the cell edge
        // to the center of the demand cell. after that it disappears. like behind closed doors
        // probably in most cases we'd just want a door to slide over/close the item or w/e.

        let demand = &cell.demand;
        if demand.last_part_at > 0 {
          let p = (factory.ticks - demand.last_part_at) as f64 / demand.speed.max(1) as f64;
          if p <= 1.0 {

            // let p = demand.part_progress.min(ONE_SECOND as f64 * options.speed_modifier_floor) as f64 / demand.speed.max(1) as f64;

            let mut dx = ox;// + (CELL_W - PART_W) * 0.5;
            let mut dy = oy;// + (CELL_H - PART_H) * 0.5;

            if cy == 0 {
              dx += (CELL_W - PART_W) * 0.5;
              dy += PART_H * (1.0 - p);
            } else if cx == FLOOR_CELLS_W - 1 {
              dx += -(PART_W * 0.5 * (1.0 - p)) + (CELL_W - PART_W) * p;
              dy += (CELL_H - PART_H) * 0.5;
            } else if cy == FLOOR_CELLS_H - 1 {
              dx += (CELL_W - PART_W) * 0.5;
              dy += - (PART_H * 0.5 * (1.0 - p)) + (CELL_H - PART_H) * p;
            } else if cx == 0 {
              dx += (CELL_W - (PART_W * 0.5)) * (1.0 - p);
              dy += (CELL_H - PART_H) * 0.5;
            } else {
              panic!("no");
            };
            paint_segment_part_from_config(options, state, config, context, demand.last_part_kind, dx, dy, PART_W, PART_H);
          }
        }
      }
    }
  }
}
fn paint_background_tiles3(
  options: &Options,
  state: &State,
  config: &Config,
  context: &Rc<web_sys::CanvasRenderingContext2d>,
  factory: &Factory,
  img_machine2: &web_sys::HtmlImageElement,
  img_machine_1_1: &web_sys::HtmlImageElement,
  img_machine_2_1: &web_sys::HtmlImageElement,
  img_machine_3_2: &web_sys::HtmlImageElement,
) {
  // Paint background cell tiles
  for coord in 0..FLOOR_CELLS_WH {
    let (cx, cy) = to_xy(coord);

    let ox = UI_FLOOR_OFFSET_X + CELL_W * (cx as f64);
    let oy = UI_FLOOR_OFFSET_Y + CELL_H * (cy as f64);

    let cell = &factory.floor[coord];

    // This is cheating since we defer the loading stuff to the browser. Sue me.
    match cell.kind {
      CellKind::Empty => {
      },
      CellKind::Belt => {
      },
      CellKind::Machine => {
        // For machines, paint the top-left cell only but make the painted area cover the whole machine
        // TODO: each machine size should have a unique, customized, sprite
        if cell.machine.main_coord == coord {
          let machine_img = match ( cell.machine.cell_width, cell.machine.cell_height ) {
            ( 1, 1 ) => &config.sprite_cache_canvas[config.nodes[CONFIG_NODE_MACHINE_1X1].sprite_config.frames[0].file_canvas_cache_index],
            ( 2, 2 ) => &config.sprite_cache_canvas[config.nodes[CONFIG_NODE_MACHINE_2X2].sprite_config.frames[0].file_canvas_cache_index],
            ( 3, 3 ) => &config.sprite_cache_canvas[config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].file_canvas_cache_index],
            ( 4, 4 ) => img_machine_1_1,
            ( 2, 1 ) => img_machine_2_1,
            ( 4, 2 ) => img_machine_2_1,
            ( 3, 2 ) => img_machine_3_2,
            _ => &config.sprite_cache_canvas[config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].file_canvas_cache_index],
          };
          context.draw_image_with_html_image_element_and_dw_and_dh(machine_img, ox, oy, cell.machine.cell_width as f64 * CELL_W, cell.machine.cell_height as f64 * CELL_H).expect("something error draw_image"); // requires web_sys HtmlImageElement feature

          // Paint tiny output part
          paint_segment_part_from_config(options, state, config, context, cell.machine.output_want.kind, ox + config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].x, oy + config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].y, config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].w, config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].h);
        }


        paint_asset(options, state, config, context, CONFIG_NODE_ASSET_WEE_WOO, factory.ticks,
          ox + config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].x, oy + config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].y,
          config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].w, config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].h
        );
      },
      CellKind::Supply => {
        // Paint the supplier image with partial transparency, making the belt and part appear semi-transparently
        paint_supplier(options, state, config, factory, context, ox, oy, CELL_W, CELL_H, cell.supply.part_created_at, factory.ticks, coord);
      }
      CellKind::Demand => {
        // Paint the supplier image with partial transparency, making the belt and part appear semi-transparently
        paint_demander(options, state, config, factory, context, ox, oy, CELL_W, CELL_H, cell.demand.last_part_at, factory.ticks, coord);
      }
    }
  }

  if options.print_priority_tile_order {
    for i in 0..factory.prio.len() {
      let coord = factory.prio[i];
      let (cx, cy) = to_xy(coord);
      if factory.floor[coord].kind == CellKind::Belt { context.set_stroke_style(&"white".into()); }
      else { context.set_stroke_style(&"blue".into()); }
      let ox = UI_FLOOR_OFFSET_X + CELL_W * (cx as f64) + (CELL_W / 2.0 - 7.0);
      let oy = UI_FLOOR_OFFSET_Y + CELL_H * (cy as f64) + CELL_H / 2.0 + 3.0;
      context.stroke_text(format!("{}", i).as_str(), ox, oy).expect("stroke_text");
    }
  }
}
fn paint_port_arrows(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory) {
  if !options.draw_port_arrows {
    return;
  }


  // "draw arrows"
  context.set_stroke_style(&"white".into());

  // Adjust for font size such that it gets centered. API falls a little short in this regard.
  let font_centering_delta_x: f64 = -5.0;
  let font_centering_delta_y: f64 = 4.0;

  for coord in 0..FLOOR_CELLS_WH {
    let (x, y) = to_xy(coord);
    if factory.floor[coord].kind != CellKind::Empty {
      // For each cell only paint the right and bottom port
      // Otherwise we're just gonna paint each port twice

      if factory.floor[coord].port_r == Port::Inbound {
        context.stroke_text("🡄", UI_FLOOR_OFFSET_X + (x as f64) * CELL_W + CELL_W + font_centering_delta_x, UI_FLOOR_OFFSET_Y + (y as f64) * CELL_H + CELL_H / 2.0 + font_centering_delta_y).expect("to paint");
      } else if factory.floor[coord].port_r == Port::Outbound {
        context.stroke_text("🡆", UI_FLOOR_OFFSET_X + (x as f64) * CELL_W + CELL_W + font_centering_delta_x, UI_FLOOR_OFFSET_Y + (y as f64) * CELL_H + CELL_H / 2.0 + font_centering_delta_y).expect("to paint");
      }

      if factory.floor[coord].port_d == Port::Inbound {
        context.stroke_text("🡅", UI_FLOOR_OFFSET_X + (x as f64) * CELL_W + CELL_W / 2.0 + font_centering_delta_x, UI_FLOOR_OFFSET_Y + (y as f64) * CELL_H + CELL_H + font_centering_delta_y).expect("to paint");
      } else if factory.floor[coord].port_d == Port::Outbound {
        context.stroke_text("🡇", UI_FLOOR_OFFSET_X + (x as f64) * CELL_W + CELL_W / 2.0 + font_centering_delta_x, UI_FLOOR_OFFSET_Y + (y as f64) * CELL_H + CELL_H + font_centering_delta_y).expect("to paint");
      }
    }
  }
}
fn paint_belt_dbg_id(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory) {
  if !options.draw_belt_dbg_id {
    return;
  }

  // "draw arrows"
  context.set_stroke_style(&"white".into());

  // Adjust for font size such that it gets centered. API falls a little short in this regard.
  let font_centering_delta_x: f64 = -5.0;
  let font_centering_delta_y: f64 = 4.0;

  for coord in 0..FLOOR_CELLS_WH {
    let (x, y) = to_xy(coord);
    if factory.floor[coord].kind != CellKind::Empty {
      // For each cell only paint the right and bottom port
      // Otherwise we're just gonna paint each port twice

      if factory.floor[coord].kind == CellKind::Belt {
        context.set_fill_style(&"white".into());
        let mut wat = factory.floor[coord].belt.meta.dbg.split('_');
        // let prefix = wat.next().unwrap();
        let ins = wat.next().or(Some("")).unwrap().trim();
        let outs = wat.next().or(Some("")).unwrap().trim();
        let uns = wat.next().or(Some("")).unwrap().trim();
        context.fill_text(format!(">{}", ins).as_str(), UI_FLOOR_OFFSET_X + (x as f64) * CELL_W + 1.0, UI_FLOOR_OFFSET_Y + (y as f64) * CELL_H + 10.0).expect("should work");
        context.fill_text(format!("<{}", outs).as_str(), UI_FLOOR_OFFSET_X + (x as f64) * CELL_W + 1.0, UI_FLOOR_OFFSET_Y + (y as f64) * CELL_H + 20.0).expect("should work");
        context.fill_text(format!("-{}", uns).as_str(), UI_FLOOR_OFFSET_X + (x as f64) * CELL_W + 1.0, UI_FLOOR_OFFSET_Y + (y as f64) * CELL_H + 30.0).expect("should work");
      }
    }
  }
}
fn paint_belt_items(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory) {
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
        let sx = UI_FLOOR_OFFSET_X + CELL_W * (cx as f64) + -(PART_W * 0.5);
        let sy = UI_FLOOR_OFFSET_Y + CELL_H * (cy as f64) + -(PART_H * 0.5);

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

        if paint_segment_part_from_config(options, state, config, context, cell.belt.part.kind, px, py, PART_W, PART_H) {
          // context.set_font(&"8px monospace");
          // context.set_fill_style(&"green".into());
          // context.fill_text(format!("{} {}x{}", coord, x, y).as_str(), px + 3.0, py + 10.0).expect("something error fill_text");
          // context.fill_text(format!("{}", progress_c).as_str(), px + 3.0, py + 21.0).expect("something error fill_text");
        }
      },
      CellKind::Machine => {
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
fn paint_machine_craft_menu(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState) {
  if !cell_selection.on {
    // No cell selected.
    return;
  }

  if state.mouse_mode_selecting {
    // Do not show craft menu while mouse selection is enabled
    return;
  }

  let selected_coord = cell_selection.coord;
  if factory.floor[selected_coord].kind != CellKind::Machine {
    // Not selected a machine.
    // log!("No machine selected");
    return;
  }

  // Each cell consolidates much of its information into the main coord, the top-left cell
  let selected_main_coord = factory.floor[selected_coord].machine.main_coord;
  let ( main_x, main_y ) = to_xy(selected_main_coord);

  let main_wx = UI_FLOOR_OFFSET_X + (main_x as f64) * CELL_W;
  let main_wy = UI_FLOOR_OFFSET_Y + (main_y as f64) * CELL_H;

  // Find the center of the machine because .arc() requires the center x,y
  let machine_cw = factory.floor[selected_main_coord].machine.cell_width as f64;
  let machine_ch = factory.floor[selected_main_coord].machine.cell_height as f64;
  let machine_ww = machine_cw * CELL_W;
  let machine_wh = machine_ch * CELL_H;
  let ( center_wx, center_wy, cr ) = get_machine_selection_circle_params(factory, selected_main_coord);

  // Only paint the craft menu when opt-in. It was an early design attempt but it was not intuitive enough.
  if options.enable_craft_menu_circle {
    // We'll draw a semi-transparent circle over the factory with a radius big enough to fit
    // input-type bubbles equally distributed in the ring around the factory. Those should be
    // interactable so their position must be fully predictable.
    // Perhaps they should be squares to make hitboxes easier, but that's tbd.

    // Cheat by using rgba for semi trans
    context.set_fill_style(&"#0000007f".into());
    // context.set_fill_style(&"red".into());
    // log!("arc({}, {}, {}) machine({},{})", center_wx, center_wy, cr, machine_cw, machine_ch);
    context.begin_path();
    context.arc(center_wx, center_wy, cr, 0.0, 2.0 * 3.14).expect("to paint");
    context.fill();
  }

  context.set_fill_style(&"#ffffff7f".into());
  context.fill_rect(main_wx, main_wy, machine_ww, machine_wh);

  if mouse_state.craft_over_ci_index > 99 { // This means it's over the machine index + 100
    let hover_cell_index = mouse_state.craft_over_ci_index - 100;
    let x = hover_cell_index % machine_cw as u8;
    let y = hover_cell_index / machine_cw as u8;
    // This should be an index. Color the appropriate index as indicator.
    context.set_fill_style(&"#00ff0040".into());
    context.fill_rect(main_wx + x as f64 * CELL_W, main_wy + y as f64 * CELL_H, CELL_W, CELL_H);
  }

  context.set_stroke_style(&"purple".into());
  for i in 0..machine_cw as usize {
    for j in 0..machine_ch as usize {
      let fi = i as f64;
      let fj = j as f64;

      context.begin_path();
      context.move_to(main_wx,                main_wy + fj * CELL_H);
      context.line_to(main_wx + machine_ww,   main_wy + fj * CELL_H);
      context.stroke();

      context.begin_path();
      context.move_to(main_wx + fi * CELL_W,  main_wy);
      context.line_to(main_wx + fi * CELL_W,  main_wy + machine_wh);
      context.stroke();
    }
  }

  // Draw the wants in the right spots
  let none = part_none(config);
  for i in 0..(machine_cw * machine_ch) as usize {
    if let Some(part) = factory.floor[selected_main_coord].machine.wants.get(i).or(Some(&none)) {
      paint_segment_part_from_config(options, state, config, context, part.kind, main_wx + CELL_W * (i as f64 % machine_cw).floor(), main_wy + CELL_H * (i as f64 / machine_cw).floor(), CELL_W, CELL_H);
    }
  }

  if options.enable_craft_menu_circle {
    // Minimal distance of painting interactbles is the distance from the center to the furthest
    // angle (any machine corner) plus a small buffer. a^2+b^2=c^2
    // This radius determines the distance from the center of the circle to the _center_ of the cell.
    let minr = ((center_wx - main_wx).powf(2.0) + (center_wy - main_wy).powf(2.0)).powf(0.5) + 30.0;

    fn btn(context: &Rc<web_sys::CanvasRenderingContext2d>, wx: f64, wy: f64, text: char, is_over: bool) {
      if is_over {
        context.set_fill_style(&"grey".into());
      } else {
        context.set_fill_style(&"white".into());
      }
      context.fill_rect(wx, wy, CELL_W, CELL_H);
      context.set_stroke_style(&"black".into());
      context.stroke_rect(wx, wy, CELL_W, CELL_H);
      context.set_fill_style(&"black".into());
      context.set_font(&"48px monospace");
      context.fill_text(format!("{}", text).as_str(), wx + 4.0, wy + 34.0).expect("oopsie fill_text"); // This would be a sprite, anyways
      context.set_font(&"12px monospace");
    }

    fn btn_img(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, wx: f64, wy: f64, part_index: PartKind, is_over: bool) {
      if is_over {
        context.set_fill_style(&"grey".into());
      } else {
        context.set_fill_style(&"white".into());
      }
      context.fill_rect(wx, wy, CELL_W, CELL_H);
      context.set_stroke_style(&"black".into());
      context.stroke_rect(wx, wy, CELL_W, CELL_H);

      paint_segment_part_from_config(options, state, config, context, part_index, wx, wy, CELL_W, CELL_H);
    }

    // The back/close button should always be under the machine, centered. Same size (one cell).
    let close_wx = center_wx - CELL_W / 2.0;
    let close_wy = center_wy + minr - CELL_H / 2.0;
    btn(context, close_wx, close_wy, '↩', mouse_state.craft_over_ci == CraftInteractable::BackClose);

    // Actual number of seen inputs
    let len = factory.floor[selected_main_coord].machine.last_received.len();
    // Make sure that we always show something. If there aren't any elements, show trash as the only icon.
    let count = len.max(1);

    let angle_step = 5.5 - (count as f64 / 2.0).ceil() + (0.5 * ((count % 2) as f64));
    for i in 0..count {
      let angle: f64 = (angle_step + i as f64) * 0.1 * std::f64::consts::TAU;

      // TODO: could pre-compute these coords per factory and read the coords from a vec
      let btn_c_wx = angle.sin() * minr;
      let btn_c_wy = angle.cos() * minr;
      let wx = center_wx + btn_c_wx - CELL_W / 2.0;
      let wy = center_wy + btn_c_wy - CELL_H / 2.0;

      // When hovering over the index, the _c is set to the char of the digit of that index.
      // If there are no last seen elements, show a trash icon
      btn_img(options, state, config, context, wx, wy, if len == 0 { PARTKIND_TRASH } else { factory.floor[selected_main_coord].machine.last_received[i].0.kind }, mouse_state.craft_over_ci_index == (i as u8));
    }
  }
}
fn paint_mouse_cursor(context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  context.set_fill_style(&"#ff00ff7f".into()); // Semi transparent circles
  context.begin_path();
  context.ellipse(mouse_state.world_x, mouse_state.world_y, PART_W / 2.0, PART_H / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
  context.fill();
}
fn paint_mouse_action(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, cell_selection: &CellSelection) {
  if mouse_state.craft_dragging_ci {
    paint_mouse_dragging_craft_interactable(options, state, config, factory, context, mouse_state, cell_selection);
  }
  else if state.mouse_mode_selecting {
    paint_mouse_in_selection_mode(options, state, config, factory, context, mouse_state, cell_selection);
  }
  else if mouse_state.dragging_offer {
    paint_mouse_while_dragging_offer(options, state, config, factory, context, mouse_state, cell_selection);
  }
  else if mouse_state.dragging_machine {
    paint_mouse_while_dragging_machine(options, state, factory, context, mouse_state);
  }
  else if mouse_state.over_floor_not_corner {
    paint_mouse_cell_location_on_floor(&context, &factory, &cell_selection, &mouse_state);
    if mouse_state.was_dragging || mouse_state.is_dragging {
      if mouse_state.down_zone == ZONE_CRAFT {
        // This drag stated in a craft popup so do not show a track preview; we're not doing that.
      }
      else if mouse_state.down_floor_not_corner {
        if mouse_state.last_down_event_type == EventSourceType::Mouse {
          paint_belt_drag_preview(options, state, config, context, factory, cell_selection, mouse_state);
        }
      }
    }
  }
}
fn paint_mouse_dragging_craft_interactable(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, cell_selection: &CellSelection) {
  // Support up to 9 elements in the craft menu
  // If something is is dragged, ignore it.
  if mouse_state.craft_down_ci_index != 99 { // <99 means circle button, >99 means machine cell
    let w = PART_W;
    let h = PART_H;

    let mwx = mouse_state.world_x - (w / 2.0);
    let mwy  = mouse_state.world_y - (h / 2.0);
    // fn btn(context: &Rc<web_sys::CanvasRenderingContext2d>, wx: f64, wy: f64, text: char, is_over: bool) {
    context.set_fill_style(&"white".into());
    context.fill_rect(mwx, mwy, w, h);
    context.set_stroke_style(&"black".into());
    context.stroke_rect(mwx, mwy, w, h);

    paint_segment_part_from_config(options, state, config, context, mouse_state.craft_down_ci_part_kind, mwx, mwy, w, h);
  }
}
fn paint_mouse_in_selection_mode(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, cell_selection: &CellSelection) {
  // When mouse is down and clipboard is empty; select the area to potentially copy. With clipboard, still show the ghost. Do not change the selection area.
  if mouse_state.is_down && state.selected_area_copy.len() == 0 {
    if mouse_state.down_zone == ZONE_FLOOR && mouse_state.over_zone == ZONE_FLOOR {
      // Draw dotted stroke rect around cells from mouse down cell to current cell
      let down_cell_x = mouse_state.last_down_cell_x_floored;
      let down_cell_y = mouse_state.last_down_cell_y_floored;
      context.set_stroke_style(&"blue".into());
      context.stroke_rect(
        UI_FLOOR_OFFSET_X + down_cell_x.min(mouse_state.cell_x_floored) * CELL_W,
        UI_FLOOR_OFFSET_Y + down_cell_y.min(mouse_state.cell_y_floored) * CELL_H,
        (1.0 + (down_cell_x - mouse_state.cell_x_floored).abs()) * CELL_W,
        (1.0 + (down_cell_y - mouse_state.cell_y_floored).abs()) * CELL_H
      );
    }
  }
  else {
    if cell_selection.on {
      // There is a current selection so draw it.
      // Rectangle around current selection, if any
      context.set_stroke_style(&"blue".into());
      context.stroke_rect(UI_FLOOR_OFFSET_X + cell_selection.x * CELL_W, UI_FLOOR_OFFSET_Y + cell_selection.y * CELL_H, (1.0 + (cell_selection.x - cell_selection.x2).abs()) * CELL_W, (1.0 + (cell_selection.y - cell_selection.y2).abs()) * CELL_H);
      if state.selected_area_copy.len() > 0 {
        // Draw a rectangle to indicate paste area. Always a rectangle of sorts.
        let w = state.selected_area_copy[0].len(); // There must be at least one
        let h = state.selected_area_copy.len();
        context.set_stroke_style(&"green".into());
        context.stroke_rect(UI_FLOOR_OFFSET_X + cell_selection.x * CELL_W, UI_FLOOR_OFFSET_Y + cell_selection.y * CELL_H, w as f64 * CELL_W, h as f64 * CELL_H);

        let cell_x = mouse_state.cell_x_floored;
        let cell_y = mouse_state.cell_y_floored;
        for j in 0..state.selected_area_copy.len() {
          for i in 0..state.selected_area_copy[j].len() {
            let x = cell_x + (i as f64);
            let y = cell_y + (j as f64);
            if is_middle(x, y) {
              let bt = state.selected_area_copy[j][i].belt.meta.btype;
              paint_ghost_belt_of_type(options, state, config, x as usize, y as usize, bt, &context, false);
            }
          }
        }
      }
    }
    if mouse_state.over_floor_not_corner {
      // Rectangle around current cell (generic) as a visual cue of selection mode; that you can start dragging there
      context.set_stroke_style(&"red".into());
      context.stroke_rect(UI_FLOOR_OFFSET_X + mouse_state.cell_x_floored * CELL_W, UI_FLOOR_OFFSET_Y + mouse_state.cell_y_floored * CELL_H, CELL_W, CELL_H);
    }
  }
}
fn paint_mouse_while_dragging_machine(options: &Options, state: &State, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  // For now, machines are fixed to 3x3
  let machine_cells_width = 3;
  let machine_cells_height = 3;

  // Paint drop zone over the edge cells
  context.set_fill_style(&"#00004444".into());

  // All edges
  context.fill_rect(UI_FLOOR_OFFSET_X, UI_FLOOR_OFFSET_Y, CELL_W, FLOOR_HEIGHT - CELL_H);
  context.fill_rect(UI_FLOOR_OFFSET_X + CELL_W, UI_FLOOR_OFFSET_Y, FLOOR_WIDTH - CELL_W, CELL_H);
  context.fill_rect(UI_FLOOR_OFFSET_X + FLOOR_WIDTH - CELL_W, UI_FLOOR_OFFSET_Y + CELL_H, CELL_W, FLOOR_HEIGHT - CELL_H);
  context.fill_rect(UI_FLOOR_OFFSET_X, UI_FLOOR_OFFSET_Y + FLOOR_HEIGHT - CELL_H, FLOOR_WIDTH - CELL_W, CELL_H);

  // Note that mouse cell x is not where the top-left most cell of the machine would be
  let top_left_machine_cell_x = get_x_while_dragging_offer_machine(mouse_state.cell_x, machine_cells_width);
  let top_left_machine_cell_y = world_y_to_top_left_cell_y_while_dragging_offer_machine(mouse_state.cell_y, machine_cells_height);

  // Make sure the entire machine fits, not just the center or topleft cell
  let legal = bounds_check(top_left_machine_cell_x, top_left_machine_cell_y, 1.0, 1.0, FLOOR_CELLS_W as f64 - (machine_cells_width as f64), FLOOR_CELLS_H as f64 - (machine_cells_height as f64));

  // Face out illegal options
  let ( paint_at_x, paint_at_y) =
    if legal {
      ( UI_FLOOR_OFFSET_X + top_left_machine_cell_x.round() * CELL_W, UI_FLOOR_OFFSET_Y + top_left_machine_cell_y.round() * CELL_H )
    } else {
      // Do not snap if machine would cover the edge
      let ox = mouse_state.world_x - ((machine_cells_width as f64) * (CELL_W as f64) / 2.0 );
      let oy = mouse_state.world_y - ((machine_cells_height as f64) * (CELL_H as f64) / 2.0 );
      ( ox, oy )
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
  context.set_fill_style(&COLOR_MACHINE_SEMI.into());
  context.fill_rect(paint_at_x, paint_at_y, (machine_cells_width as f64) * CELL_W, (machine_cells_height as f64) * CELL_H);
  if !legal { paint_illegal(&context, paint_at_x, paint_at_y, (machine_cells_width as f64) * CELL_W, (machine_cells_height as f64) * CELL_H); }
  context.set_fill_style(&"black".into());
  context.fill_text("M", paint_at_x + (machine_cells_width as f64) * CELL_W / 2.0 - 5.0, paint_at_y + (machine_cells_height as f64) * CELL_H / 2.0 + 2.0).expect("no error")
}
fn paint_mouse_while_dragging_offer(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, cell_selection: &CellSelection) {
  // Two cases:
  // - the offer has a pattern; only allow to drag to machines. with debug setting can be both?
  // - the offer has no pattern; only allow to edge as supply

  let part_index = factory.available_parts_rhs_menu[mouse_state.offer_down_offer_index].0;
  paint_ui_offer_hover_droptarget_hint(options, state, config, context, factory, part_index);

  let len = config.nodes[part_index].pattern_unique_kinds.len();
  if len > 0 {
    // Only machines unless debug setting is enabled
    // When over a machine, preview the pattern over the machine? Or snap the offer to its center?

    // Mouse position determines actual cell that we check
    let coord = to_coord(mouse_state.cell_x_floored as usize, mouse_state.cell_y_floored as usize);
    if is_middle(mouse_state.cell_x_floored, mouse_state.cell_y_floored) && factory.floor[coord].kind == CellKind::Machine {
      // If a craft menu is open and the hover is over a craft then only show it if the machine is
      // the current selection (dont hint for other machines under the craft menu because it wont work)
      let main_coord = factory.floor[coord].machine.main_coord;
      let selected_main_coord = factory.floor[cell_selection.coord].machine.main_coord;
      if mouse_state.over_zone != ZONE_CRAFT || selected_main_coord == main_coord {
        paint_part_and_pattern_at_middle(options, state, config, context, factory, main_coord, part_index);
      }
    } else {
      paint_supply_and_part_not_floor(options, state, config, context, mouse_state.world_x - ((CELL_W as f64) / 2.0), mouse_state.world_y - ((CELL_H as f64) / 2.0), part_index);
    }
  }
  else {
    // Only edge. No point in dumping into machine, I guess? Maybe as an expensive supply? Who cares?
    if is_edge_not_corner(mouse_state.cell_x_floored, mouse_state.cell_y_floored) {
      paint_supply_and_part_for_edge(options, state, config, context, mouse_state.cell_x_floored as usize, mouse_state.cell_y_floored as usize, part_index);
    } else {
      paint_supply_and_part_not_edge(options, state, config, context, mouse_state.world_x - ((CELL_W as f64) / 2.0), mouse_state.world_y - ((CELL_H as f64) / 2.0), part_index);
    }
  }
}
fn paint_mouse_cell_location_on_floor(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState) {
  if mouse_state.cell_x_floored != cell_selection.x || mouse_state.cell_y_floored != cell_selection.y {
    if
      // Ignore the corners as well
      line_check(mouse_state.cell_x_floored, 1.0, FLOOR_CELLS_W as f64 - 1.0) ||
      line_check(mouse_state.cell_y_floored, 1.0, FLOOR_CELLS_H as f64 - 1.0)
    {
      context.set_stroke_style(&"red".into());
      context.stroke_rect(UI_FLOOR_OFFSET_X + mouse_state.cell_x_floored * CELL_W, UI_FLOOR_OFFSET_Y + mouse_state.cell_y_floored * CELL_H, CELL_W, CELL_H);
    }
  }
}
fn paint_belt_drag_preview(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState) {
  // If both x and y are on the edge then they're in a corner
  if !mouse_state.over_floor_not_corner {
    // Corner cell of the floor. Consider oob and ignore.
    return;
  }

  let track = ray_trace_dragged_line_expensive(
    factory,
    mouse_state.last_down_cell_x_floored,
    mouse_state.last_down_cell_y_floored,
    mouse_state.cell_x_floored,
    mouse_state.cell_y_floored,
    true, // if we dont then the preview will show only broken belt cells
  );

  let action = mouse_button_to_action(state, mouse_state);

  for index in 0..track.len() {
    let ((cell_x, cell_y), bt, in_port_dir, out_port_dir) = track[index];
    // Correct for the edges, except when the track would be removed, cause then it's just red boxes
    if action == Action::Remove {
      if index == 0 {
        if cell_x == 0 {
          paint_ghost_supplier(options, state, config, factory, cell_x, cell_y, Direction::Left, context, false);
          continue;
        } else if cell_y == 0 {
          paint_ghost_supplier(options, state, config, factory, cell_x, cell_y, Direction::Up, context, false);
          continue;
        } else if cell_x == FLOOR_CELLS_W - 1 {
          paint_ghost_supplier(options, state, config, factory, cell_x, cell_y, Direction::Right, context, false);
          continue;
        } else if cell_y == FLOOR_CELLS_H - 1 {
          paint_ghost_supplier(options, state, config, factory, cell_x, cell_y, Direction::Down, context, false);
          continue;
        }
      } else if index == track.len() - 1 {
        if cell_x == 0 {
          paint_ghost_demander(options, state, config, factory, cell_x, cell_y, Direction::Left, context, false);
          continue;
        } else if cell_y == 0 {
          paint_ghost_demander(options, state, config, factory, cell_x, cell_y, Direction::Up, context, false);
          continue;
        } else if cell_x == FLOOR_CELLS_W - 1 {
          paint_ghost_demander(options, state, config, factory, cell_x, cell_y, Direction::Right, context, false);
          continue;
        } else if cell_y == FLOOR_CELLS_H - 1 {
          paint_ghost_demander(options, state, config, factory, cell_x, cell_y, Direction::Down, context, false);
          continue;
        }
      }
    }

    paint_ghost_belt_of_type(options, state, config, cell_x, cell_y, if action == Action::Remove { BeltType::INVALID } else { bt }, &context,
      // Skip over factory cells or if you're dragging straight on one edge (note that the first/last cell will take an earlier path above so this must be middle-path-cells)
      factory.floor[to_coord(cell_x, cell_y)].kind == CellKind::Machine || cell_x == 0 || cell_x == FLOOR_CELLS_W - 1 || cell_y == 0 || cell_y == FLOOR_CELLS_H - 1
    );
  }
}
fn paint_ghost_belt_of_type(options: &Options, state: &State, config: &Config, cell_x: usize, cell_y: usize, belt_type: BeltType, context: &Rc<web_sys::CanvasRenderingContext2d>, skip_tile: bool) {
  let tile_size_reduction = 1.0;

  context.set_fill_style(&"#ffffff40".into());
  context.fill_rect(UI_FLOOR_OFFSET_X + cell_x as f64 * CELL_W + (1.0 - tile_size_reduction / 2.0), UI_FLOOR_OFFSET_Y + cell_y as f64 * CELL_H + (1.0 - tile_size_reduction / 2.0), CELL_W * tile_size_reduction, CELL_H * tile_size_reduction);

  if !skip_tile {
    context.set_global_alpha(0.7);
    paint_zero_belt(options, state, config, context, UI_FLOOR_OFFSET_X + cell_x as f64 * CELL_W + 5.0, UI_FLOOR_OFFSET_Y + cell_y as f64 * CELL_H + 5.0, CELL_W - 10.0, CELL_H - 10.0, belt_type);
    context.set_global_alpha(1.0);
  }
}
fn paint_ghost_supplier(options: &Options, state: &State, config: &Config, factory: &Factory, cell_x: usize, cell_y: usize, dir: Direction, context: &Rc<web_sys::CanvasRenderingContext2d>, skip_tile: bool) {
  let tile_size_reduction = 1.0;

  context.set_fill_style(&"#ffffff40".into());
  context.fill_rect(UI_FLOOR_OFFSET_X + cell_x as f64 * CELL_W + (1.0 - tile_size_reduction / 2.0), UI_FLOOR_OFFSET_Y + cell_y as f64 * CELL_H + (1.0 - tile_size_reduction / 2.0), CELL_W * tile_size_reduction, CELL_H * tile_size_reduction);

  if !skip_tile {
    context.set_global_alpha(0.7);
    paint_supplier(options, state, config, factory, context, UI_FLOOR_OFFSET_X + cell_x as f64 * CELL_W + 5.0, UI_FLOOR_OFFSET_Y + cell_y as f64 * CELL_H + 5.0, CELL_W - 10.0, CELL_H - 10.0, 0, 0, to_coord(cell_x, cell_y));
    context.set_global_alpha(1.0);
  }
}
fn paint_ghost_demander(options: &Options, state: &State, config: &Config, factory: &Factory, cell_x: usize, cell_y: usize, dir: Direction, context: &Rc<web_sys::CanvasRenderingContext2d>, skip_tile: bool) {
  let tile_size_reduction = 1.0;

  context.set_fill_style(&"#ffffff40".into());
  context.fill_rect(UI_FLOOR_OFFSET_X + cell_x as f64 * CELL_W + (1.0 - tile_size_reduction / 2.0), UI_FLOOR_OFFSET_Y + cell_y as f64 * CELL_H + (1.0 - tile_size_reduction / 2.0), CELL_W * tile_size_reduction, CELL_H * tile_size_reduction);

  if !skip_tile {
    context.set_global_alpha(0.7);
    paint_demander(options, state, config, factory, context, UI_FLOOR_OFFSET_X + cell_x as f64 * CELL_W + 5.0, UI_FLOOR_OFFSET_Y + cell_y as f64 * CELL_H + 5.0, CELL_W - 10.0, CELL_H - 10.0, 0, 0, to_coord(cell_x, cell_y));
    // paint_demander(options, state, config, context, dir, UI_FLOOR_OFFSET_X + cell_x as f64 * CELL_W + 5.0, UI_FLOOR_OFFSET_Y + cell_y as f64 * CELL_H + 5.0, CELL_W - 10.0, CELL_H - 10.0);
    context.set_global_alpha(1.0);
  }
}
fn paint_debug_selected_belt_cell(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState) {
  if !cell_selection.on {
    return;
  }

  let selected_coord = cell_selection.coord;
  if factory.floor[selected_coord].kind != CellKind::Belt {
    return;
  }

  // Draw the following bits of info about this belt:
  // - coord
  // - port details
  // - ins / outs
  // - part details
  // - (neighbor decision stuff?)

  // Each cell consolidates much of its information into the main coord, usually the top-left cell
  let x = cell_selection.x;
  let y = cell_selection.y;

  // Mark the currently selected cell
  context.set_stroke_style(&"cyan".into());
  context.stroke_rect(UI_FLOOR_OFFSET_X + x as f64 * CELL_W, UI_FLOOR_OFFSET_Y + y as f64 * CELL_H, CELL_W, CELL_H);


  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_CELL_OFFSET_X, UI_DEBUG_CELL_OFFSET_Y, UI_DEBUG_CELL_WIDTH, UI_DEBUG_CELL_HEIGHT);
  context.set_stroke_style(&"black".into());
  context.stroke_rect(UI_DEBUG_CELL_OFFSET_X, UI_DEBUG_CELL_OFFSET_Y, UI_DEBUG_CELL_WIDTH, UI_DEBUG_CELL_HEIGHT);


  context.set_fill_style(&"black".into());
  context.fill_text(format!("Belt cell: {} x {} (@{})", x, y, selected_coord).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (1.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Ports: {}", cell_ports_to_str(&factory.floor[selected_coord])).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (2.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("ins:  {}", ins_outs_to_str(&factory.floor[selected_coord].ins)).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (3.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("to text");
  context.fill_text(format!("outs: {}", ins_outs_to_str(&factory.floor[selected_coord].outs)).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (4.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("to text");

  // let mut in_coords = factory.floor[coord].ins.iter().map(|(_dir, coord, _, _)| coord).collect::<Vec<&usize>>();
  // in_coords.sort();
  // in_coords.dedup();
  // context.fill_text(format!("Ins : {:?}", in_coords).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (2.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  // let mut out_coords = factory.floor[coord].outs.iter().map(|(_dir, coord, _, _)| coord).collect::<Vec<&usize>>();
  // out_coords.sort();
  // out_coords.dedup();
  // context.fill_text(format!("Outs: {:?}", out_coords).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (3.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  // context.fill_text(format!("Received: {:?}", factory.floor[coord].demand.received).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (4.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  //

  if factory.floor[selected_coord].belt.part.kind != PARTKIND_NONE{
    // Paint current part details
    let progress = ((factory.floor[selected_coord].belt.part_progress as f64) / (factory.floor[selected_coord].belt.speed as f64) * 100.0).round();
    let to =
      if factory.floor[selected_coord].belt.part_to_tbd {
        "TBD"
      } else {
        match factory.floor[selected_coord].belt.part_to {
          Direction::Up => "up",
          Direction::Right => "right",
          Direction::Down => "down",
          Direction::Left => "left",
        }
      };
    context.fill_text(format!("part: {} %, to: {}", progress, to).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (5.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("to paint port");
  } else {
    context.fill_text(format!("part: none").as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (5.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("to paint port");
  }

  context.fill_text(format!("cell code: {}", factory.floor[selected_coord].belt.meta.dbg).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (6.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("to paint cell code");
  context.fill_text(format!("cell icon: {}", factory.floor[selected_coord].belt.meta.cli_icon).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (7.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("to paint cell code");
  context.fill_text(format!("url: {}", factory.floor[selected_coord].belt.meta.src).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (8.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("to paint cell code");

  // TODO: could print neighbor progress decision stuff
}
fn paint_debug_selected_machine_cell(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState) {
  if !cell_selection.on {
    return;
  }

  let selected_coord = cell_selection.coord;
  if factory.floor[selected_coord].kind != CellKind::Machine {
    return;
  }
  let x = cell_selection.x;
  let y = cell_selection.y;

  // Draw the following bits of info about this cell and the machine it belongs to:
  // - coord of cell and machine
  // - port details of this cell
  // - ins / outs of this machine
  // - part details of machine
  // - wants and haves of machine
  // - seen of machine
  // - speed of machine
  // - progress of machine
  // - produced stats of machine
  // - trashed stats of machine


  // Each cell consolidates much of its information into the main coord, usually the top-left cell
  let selected_main_coord = factory.floor[selected_coord].machine.main_coord;
  let (selected_main_x, selected_main_y) = to_xy(selected_main_coord);

  // Mark the currently selected machine main_coord
  context.set_stroke_style(&"cyan".into());
  context.stroke_rect(UI_FLOOR_OFFSET_X + selected_main_x as f64 * CELL_W, UI_FLOOR_OFFSET_Y + selected_main_y as f64 * CELL_H, CELL_W * factory.floor[selected_main_coord].machine.cell_width as f64, CELL_H * factory.floor[selected_main_coord].machine.cell_height as f64);

  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_CELL_OFFSET_X, UI_DEBUG_CELL_OFFSET_Y, UI_DEBUG_CELL_WIDTH, UI_DEBUG_CELL_HEIGHT);
  context.set_stroke_style(&"black".into());
  context.stroke_rect(UI_DEBUG_CELL_OFFSET_X, UI_DEBUG_CELL_OFFSET_Y, UI_DEBUG_CELL_WIDTH, UI_DEBUG_CELL_HEIGHT);

  context.set_fill_style(&"black".into());
  // Sub details:
  context.fill_text(format!("Machine cell: {} x {} (@{})", x, y, selected_coord).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (1.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Ports: {}", cell_ports_to_str(&factory.floor[selected_coord])).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (2.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  // Main details
  context.fill_text(format!("Machine main: {} x {} (@{})", selected_main_x, selected_main_y, selected_main_coord).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (4.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Dimensions: {} x {}", factory.floor[selected_main_coord].machine.cell_width, factory.floor[selected_main_coord].machine.cell_height).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (5.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  let mut in_coords = factory.floor[selected_main_coord].ins.iter().map(|(_dir, coord, _, _)| coord).collect::<Vec<&usize>>();
  in_coords.sort();
  in_coords.dedup();
  context.fill_text(format!("Ins : {:?}", in_coords).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (6.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  let mut out_coords = factory.floor[selected_main_coord].outs.iter().map(|(_dir, coord, _, _)| coord).collect::<Vec<&usize>>();
  out_coords.sort();
  out_coords.dedup();
  context.fill_text(format!("Outs: {:?}", out_coords).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (7.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  let seen = factory.floor[selected_main_coord].machine.last_received.iter().map(|( Part { icon, .. }, ts)| icon).collect::<String>();
  context.fill_text(format!("Parts seen: {}", seen).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (8.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  let wants = factory.floor[selected_main_coord].machine.wants.iter().map(|Part { icon, .. }| if icon == &' ' { '.' } else { *icon }).collect::<String>();
  context.fill_text(format!("Wants: {}", wants).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (9.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  let haves = factory.floor[selected_main_coord].machine.haves.iter().map(|Part { icon, .. }| if icon == &' ' { '.' } else { *icon }).collect::<String>();
  context.fill_text(format!("Haves: {}", haves).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (10.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Generates: {}", factory.floor[selected_main_coord].machine.output_want.icon).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (11.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Speed: {}", factory.floor[selected_main_coord].machine.speed).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (12.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Progress: {: >3}% ({})", (((factory.ticks - factory.floor[selected_main_coord].machine.start_at) as f64 / factory.floor[selected_main_coord].machine.speed as f64).min(1.0) * 100.0) as u8, factory.floor[selected_main_coord].machine.start_at).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (13.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Produced: {: >4}", factory.floor[selected_main_coord].machine.produced).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (14.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Trashed: {: >4}", factory.floor[selected_main_coord].machine.trashed).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (15.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
}
fn paint_debug_selected_supply_cell(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState) {
  if !cell_selection.on {
    return;
  }

  let selected_coord = cell_selection.coord;
  if factory.floor[selected_coord].kind != CellKind::Supply {
    return;
  }

  // Draw the following bits of info about this supplier:
  // - coord
  // - port details
  // - ins / outs
  // - part details
  // - last out
  // - parts generated
  // - speed
  // - cooldown

  // Each cell consolidates much of its information into the main coord, usually the top-left cell
  let x = cell_selection.x;
  let y = cell_selection.y;

  // Mark the currently selected machine main_coord
  context.set_stroke_style(&"cyan".into());
  context.stroke_rect(UI_FLOOR_OFFSET_X + x as f64 * CELL_W, UI_FLOOR_OFFSET_Y + y as f64 * CELL_H, CELL_W, CELL_H);

  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_CELL_OFFSET_X, UI_DEBUG_CELL_OFFSET_Y, UI_DEBUG_CELL_WIDTH, UI_DEBUG_CELL_HEIGHT);
  context.set_stroke_style(&"black".into());
  context.stroke_rect(UI_DEBUG_CELL_OFFSET_X, UI_DEBUG_CELL_OFFSET_Y, UI_DEBUG_CELL_WIDTH, UI_DEBUG_CELL_HEIGHT);

  let supply = &factory.floor[selected_coord].supply;

  context.set_fill_style(&"black".into());
  context.fill_text(format!("Supply cell: {} x {} (@{})", x, y, selected_coord).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (1.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Ports: {}", cell_ports_to_str(&factory.floor[selected_coord])).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (2.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("ins:  {}", ins_outs_to_str(&factory.floor[selected_coord].ins)).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (3.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("to text");
  context.fill_text(format!("outs: {}", ins_outs_to_str(&factory.floor[selected_coord].outs)).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (4.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("to text");
  context.fill_text(format!("Gives: {}", supply.gives.icon).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (5.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Speed: {}", supply.speed).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (6.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Cooldown: {: >3}% {}", (((factory.ticks - supply.last_part_out_at) as f64 / supply.cooldown.max(1) as f64).min(1.0) * 100.0) as u8, supply.cooldown).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (7.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Progress: {: >3}% (tbd: {})", ((supply.part_progress as f64 / supply.speed.max(1) as f64).min(1.0) * 100.0) as u8, supply.part_tbd).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (8.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Supplied: {: >4}", supply.supplied).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (9.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
}
fn paint_debug_selected_demand_cell(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState) {
  if !cell_selection.on {
    return;
  }

  let selected_coord = cell_selection.coord;
  if factory.floor[selected_coord].kind != CellKind::Demand {
    return;
  }

  // Draw the following bits of info about this belt:
  // - coord
  // - port details
  // - ins / outs
  // - received details

  // Each cell consolidates much of its information into the main coord, usually the top-left cell
  let x = cell_selection.x;
  let y = cell_selection.y;

  // Mark the currently selected machine main_coord
  context.set_stroke_style(&"cyan".into());
  context.stroke_rect(UI_FLOOR_OFFSET_X + x as f64 * CELL_W, UI_FLOOR_OFFSET_Y + y as f64 * CELL_H, CELL_W, CELL_H);

  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_CELL_OFFSET_X, UI_DEBUG_CELL_OFFSET_Y, UI_DEBUG_CELL_WIDTH, UI_DEBUG_CELL_HEIGHT);
  context.set_stroke_style(&"black".into());
  context.stroke_rect(UI_DEBUG_CELL_OFFSET_X, UI_DEBUG_CELL_OFFSET_Y, UI_DEBUG_CELL_WIDTH, UI_DEBUG_CELL_HEIGHT);

  let demand = &factory.floor[selected_coord].demand;

  context.set_fill_style(&"black".into());
  context.fill_text(format!("Demand cell: {} x {} (@{})", x, y, selected_coord).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (1.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Ports: {}", cell_ports_to_str(&factory.floor[selected_coord])).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (2.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("ins:  {}", ins_outs_to_str(&factory.floor[selected_coord].ins)).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (3.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("to text");
  context.fill_text(format!("outs: {}", ins_outs_to_str(&factory.floor[selected_coord].outs)).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (4.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("to text");
  context.fill_text(format!("Speed: {}", demand.speed).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (6.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Cooldown: {}", demand.cooldown).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (7.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Waiting : {: >3}%", (((factory.ticks - demand.last_part_at) as f64 / demand.cooldown.max(1) as f64).min(1.0) * 100.0) as u8).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (8.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  context.fill_text(format!("Progress: {: >3}% (kind: {})", (((factory.ticks - demand.last_part_at) as f64 / demand.speed.max(1) as f64).min(1.0) * 100.0) as u8, demand.last_part_kind).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (9.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
  // context.fill_text(format!("Received: {:?}", demand.received).as_str(), UI_DEBUG_CELL_OFFSET_X + UI_DEBUG_CELL_MARGIN, UI_DEBUG_CELL_OFFSET_Y + (9.0 * UI_DEBUG_CELL_FONT_HEIGHT)).expect("something error fill_text");
}
fn paint_zone_borders(options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>) {
  if options.draw_ui_section_border {
    context.set_stroke_style(&options.ui_section_border_color.clone().into());
    context.stroke_rect(GRID_X0, GRID_Y0, GRID_LEFT_WIDTH, GRID_TOP_HEIGHT);
    context.stroke_rect(GRID_X1, GRID_Y0, FLOOR_WIDTH, GRID_TOP_HEIGHT);
    context.stroke_rect(GRID_X2, GRID_Y0, GRID_RIGHT_WIDTH, GRID_TOP_HEIGHT + GRID_SPACING + FLOOR_HEIGHT + GRID_SPACING + GRID_BOTTOM_HEIGHT);
    context.stroke_rect(GRID_X0, GRID_Y1, GRID_LEFT_WIDTH, FLOOR_HEIGHT);
    context.stroke_rect(GRID_X1, GRID_Y1, FLOOR_WIDTH, FLOOR_HEIGHT);
    context.stroke_rect(GRID_X0, GRID_Y2, GRID_LEFT_WIDTH, GRID_BOTTOM_HEIGHT);
    context.stroke_rect(GRID_X1, GRID_Y2, FLOOR_WIDTH, GRID_BOTTOM_HEIGHT);
    context.stroke_rect(GRID_X0, GRID_Y3, GRID_LEFT_WIDTH + GRID_SPACING + FLOOR_WIDTH + GRID_SPACING + GRID_RIGHT_WIDTH, GRID_BOTTOM_DEBUG_HEIGHT);
  }
}
fn paint_manual(options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, img_manual: &web_sys::HtmlImageElement) {
  if state.manual_open {
    context.draw_image_with_html_image_element_and_dw_and_dh(&img_manual, 100.0, 20.0, 740.0, 740.0).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
  }
}
fn paint_bouncers(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &mut Factory) {
  let f_one_second = ONE_SECOND as f64 ;
  let trail_time = options.bouncer_trail_time;
  let fade_time = options.bouncer_fade_time;
  // find bouncers that finished and create trucks with the new parts
  for quest_index in 0..factory.quests.len() {
    if factory.quests[quest_index].status != QuestStatus::FadingAndBouncing && factory.quests[quest_index].status != QuestStatus::Bouncing {
      continue;
    }

    // Paint all bouncer shadow/trail frames
    let trail_time = (options.bouncer_trail_time as f64 * ONE_SECOND as f64 * options.speed_modifier_ui) as u64;
    let fade_time = (options.bouncer_fade_time as f64 * ONE_SECOND as f64 * options.speed_modifier_ui) as u64;

    for ( x, y, added ) in factory.quests[quest_index].bouncer.frames.iter() {
      let frame_age = factory.ticks - added;
      let fading = frame_age > trail_time;

      if fading {
        let alpha = 1.0 - ((frame_age - trail_time) as f64 / fade_time as f64).max(0.0).min(1.0);
        context.set_global_alpha(alpha);
      }
      paint_segment_part_from_config(&options, &state, &config, &context, factory.quests[quest_index].production_part_index as usize, x + 40.0, UI_QUOTES_OFFSET_Y + UI_QUOTES_HEIGHT + 50.0 + y, CELL_W, CELL_H);
      if fading {
        context.set_global_alpha(1.0);
      }
    }

    while factory.quests[quest_index].bouncer.frames.len() > 0 && factory.ticks - factory.quests[quest_index].bouncer.frames[0].2 > trail_time + fade_time {
      factory.quests[quest_index].bouncer.frames.pop_front();
    }
  }
}
fn paint_zone_hovers(options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  if options.draw_zone_hovers {
    context.set_fill_style(&"#99999970".into()); // 100% background
    match mouse_state.over_zone {
      Zone::None => {}
      Zone::TopLeft =>            context.fill_rect(GRID_X0, GRID_Y0, GRID_X1 - GRID_X0, GRID_Y1 - GRID_Y0),
      Zone::Top =>                context.fill_rect(GRID_X1, GRID_Y0, GRID_X2 - GRID_X1, GRID_Y1 - GRID_Y0),
      Zone::TopRight =>           context.fill_rect(GRID_X2, GRID_Y0, GRID_X3 - GRID_X2, GRID_Y1 - GRID_Y0),
      Zone::Left =>               context.fill_rect(GRID_X0, GRID_Y1, GRID_X1 - GRID_X0, GRID_Y2 - GRID_Y1),
      Zone::Middle =>             context.fill_rect(GRID_X1, GRID_Y1, GRID_X2 - GRID_X1, GRID_Y2 - GRID_Y1),
      Zone::Right =>              context.fill_rect(GRID_X2, GRID_Y1, GRID_X3 - GRID_X2, GRID_Y2 - GRID_Y1),
      Zone::BottomLeft =>         context.fill_rect(GRID_X0, GRID_Y2, GRID_X1 - GRID_X0, GRID_Y3 - GRID_Y2),
      Zone::Bottom =>             context.fill_rect(GRID_X1, GRID_Y2, GRID_X2 - GRID_X1, GRID_Y3 - GRID_Y2),
      Zone::BottomRight =>        context.fill_rect(GRID_X2, GRID_Y2, GRID_X3 - GRID_X2, GRID_Y3 - GRID_Y2),
      Zone::BottomBottomLeft =>   context.fill_rect(GRID_X0, GRID_Y3, GRID_X1 - GRID_X0, GRID_Y4 - GRID_Y3),
      Zone::BottomBottom =>       context.fill_rect(GRID_X1, GRID_Y3, GRID_X2 - GRID_X1, GRID_Y4 - GRID_Y3),
      Zone::BottomBottomRight =>  context.fill_rect(GRID_X2, GRID_Y3, GRID_X3 - GRID_X2, GRID_Y4 - GRID_Y3),
      Zone::Craft => {}
      Zone::Manual => {}
    }
  }
}
fn paint_top_stats(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory) {
  context.set_fill_style(&"black".into());
  context.fill_text(format!("Ticks: {}, Supplied: {}, Produced: {}, Received: {}, Trashed: {}", factory.ticks, factory.supplied, factory.produced, factory.accepted, factory.trashed).as_str(), 20.0, 20.0).expect("to paint");
  context.fill_text(format!("Current time: {}, day start: {}, modified at: {}", factory.ticks, factory.last_day_start, factory.modified_at).as_str(), 20.0, 40.0).expect("to paint");
}
fn paint_corner_help_icon(options: &Options, state: &State, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, img_help: &web_sys::HtmlImageElement) {
  context.draw_image_with_html_image_element_and_dw_and_dh(img_help, UI_HELP_X, UI_HELP_Y, UI_HELP_WIDTH, UI_HELP_HEIGHT).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
}
fn paint_top_bars(options: &Options, state: &State, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  let hovering = mouse_state.over_zone == ZONE_DAY_BAR && !mouse_state.is_down && !mouse_state.is_up && mouse_state.over_day_bar;
  let corrupted = factory.day_corrupted;
  let invalid = factory.finished_at == 0 && factory.modified_at > factory.last_day_start && factory.modified_at < factory.last_day_start + ONE_MS * 1000 * 60 * 60;
  let day_ticks = ONE_MS * 1000 * 60; // one day a minute (arbitrary)

  if hovering {
    context.set_fill_style(&"white".into()); // 100% background
  } else {
    context.set_fill_style(&"grey".into()); // 100% background
  }
  context.fill_rect(UI_DAY_PROGRESS_OFFSET_X, UI_DAY_PROGRESS_OFFSET_Y, UI_DAY_PROGRESS_WIDTH, UI_DAY_PROGRESS_HEIGHT);
  if corrupted {
    context.set_fill_style(&"tomato".into()); // progress green
  } else {
    context.set_fill_style(&"lightgreen".into()); // progress green
  }
  context.fill_rect(UI_DAY_PROGRESS_OFFSET_X, UI_DAY_PROGRESS_OFFSET_Y, UI_DAY_PROGRESS_WIDTH * factory.curr_day_progress.min(1.0), UI_DAY_PROGRESS_HEIGHT);

  if hovering {
    context.set_stroke_style(&"red".into());
  } else {
    context.set_stroke_style(&"black".into());
  }
  context.stroke_rect(UI_DAY_PROGRESS_OFFSET_X, UI_DAY_PROGRESS_OFFSET_Y, UI_DAY_PROGRESS_WIDTH, UI_DAY_PROGRESS_HEIGHT);

  if !options.game_enable_clean_days {
    context.set_font(&"18px monospace");
    context.set_fill_style(&"black".into());
    context.fill_text("(Clean days not enabled, do whatever)", UI_DAY_PROGRESS_OFFSET_X + 37.0, UI_DAY_PROGRESS_OFFSET_Y + 22.0).expect("oopsie fill_text"); // Note: this won't scale with the floor size. But this should be a clipart or svg, anyways, which will scale.
  }
  else if invalid {
    context.set_font(&"18px monospace");
    context.set_fill_style(&"black".into());
    context.fill_text("Change detected! Click to restart day", UI_DAY_PROGRESS_OFFSET_X + 37.0, UI_DAY_PROGRESS_OFFSET_Y + 22.0).expect("oopsie fill_text"); // Note: this won't scale with the floor size. But this should be a clipart or svg, anyways, which will scale.
  }
  else if corrupted {
    context.set_font(&"18px monospace");
    context.set_fill_style(&"black".into());
    context.fill_text("Factory corrupted by trash", UI_DAY_PROGRESS_OFFSET_X + 130.0, UI_DAY_PROGRESS_OFFSET_Y + 22.0).expect("oopsie fill_text"); // Note: this won't scale with the floor size. But this should be a clipart or svg, anyways, which will scale.
  }
  else if factory.finished_at > 0 {
    context.set_font(&"18px monospace");
    context.set_fill_style(&"black".into());
    context.fill_text("Click to restart day", UI_DAY_PROGRESS_OFFSET_X + 150.0, UI_DAY_PROGRESS_OFFSET_Y + 22.0).expect("oopsie fill_text"); // Note: this won't scale with the floor size. But this should be a clipart or svg, anyways, which will scale.
  }

  context.set_font(&"30px monospace");
  context.set_fill_style(&"black".into());
  context.fill_text("🌄", UI_DAY_BAR_OFFSET_X, UI_DAY_BAR_OFFSET_Y + 26.0).expect("oopsie fill_text");
  context.fill_text("🎑", UI_DAY_PROGRESS_OFFSET_X + UI_DAY_PROGRESS_WIDTH + 5.0, UI_DAY_PROGRESS_OFFSET_Y + 26.0).expect("oopsie fill_text");

  context.set_font(&"12px monospace");
}
fn paint_quests(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, mouse_state: &MouseState, img_quest_frame: &web_sys::HtmlImageElement) {
  let mut visible_index = 0;

  for quest_index in 0..factory.quests.len() {
    if factory.quests[quest_index].status != QuestStatus::Active && factory.quests[quest_index].status != QuestStatus::FadingAndBouncing {
      continue;
    }

    let current_quest_progress = factory.quests[quest_index].production_progress;
    let current_quest_target = factory.quests[quest_index].production_target;
    let progress = current_quest_progress as f64 / current_quest_target as f64;

    let ( x, y ) = get_quest_xy(visible_index, 0.0);
    visible_index += 1;

    let part_index = factory.quests[quest_index].production_part_index as usize;
    assert!(
      config.nodes[factory.quests[quest_index].production_part_index as usize].kind == ConfigNodeKind::Part,
      "quest part index should refer to Part node but was {:?}... have index: {:?}, but it points to: {:?}",
      config.nodes[factory.quests[quest_index].production_part_index as usize].kind,
      factory.quests[quest_index].production_part_index,
      config.nodes[factory.quests[quest_index].production_part_index as usize]
    );

    context.set_fill_style(&"white".into()); // 100% background
    context.fill_rect(x + 5.0, y + 4.0, UI_QUEST_WIDTH - 10.0, UI_QUEST_HEIGHT - 8.0);
    if progress > 0.0 {
      context.set_fill_style(&"grey".into()); // 100% background
      context.fill_rect(x + 5.0, y + 4.0, (UI_QUEST_WIDTH - 10.0) * progress, UI_QUEST_HEIGHT - 8.0);
    }
    context.draw_image_with_html_image_element_and_dw_and_dh(&img_quest_frame, x, y, UI_QUEST_WIDTH, UI_QUEST_HEIGHT).expect("oopsie draw_image_with_html_image_element_and_dw_and_dh");
    paint_segment_part_from_config(options, state, config, context, part_index as usize, x + 15.0, y + 8.0, PART_W, PART_H);

    context.set_font(&"12px monospace");
    context.set_fill_style(&"#ddd".into()); // 100% background
    context.fill_text(format!("{} / {}x", current_quest_progress, current_quest_target).as_str(), x + 100.0, y + (UI_QUEST_HEIGHT / 2.0) + 5.0).expect("oopsie fill_text");

    if factory.quests[quest_index].status == QuestStatus::FadingAndBouncing {
      let fade_progress = ((factory.ticks - factory.quests[quest_index].status_at) as f64 / (QUEST_FADE_TIME as f64 * options.speed_modifier_ui)).min(1.0);
      let fade_cover_width = UI_QUEST_WIDTH * fade_progress;
      context.set_fill_style(&"#555".into()); // 100% background
      context.fill_rect(x, y, fade_cover_width, UI_QUEST_HEIGHT);
    }
  }

  // Debug
  if options.dbg_print_quest_states {
    context.set_fill_style(&"#ffffffaa".into());
    context.fill_rect(UI_QUOTES_OFFSET_X, UI_QUOTES_OFFSET_Y, UI_QUOTES_WIDTH, UI_QUOTES_HEIGHT);

    for quest_index in 0..factory.quests.len() {
      context.set_fill_style(&"black".into()); // 100% background
      context.fill_text(
        format!(
          "Q{:<2}:{} {}/{} @{} > {} : {:?}",
          quest_index,
          format!("{:?}", factory.quests[quest_index].status).chars().next().unwrap(),
          factory.quests[quest_index].production_progress,
          factory.quests[quest_index].production_target,
          factory.quests[quest_index].status_at,
          factory.quests[quest_index].production_part_index,
          factory.quests[quest_index].unlocks_todo
        ).as_str(),
        UI_QUOTES_OFFSET_X,
        UI_QUOTES_OFFSET_Y + 20.0 + (quest_index as f64) * 15.0
      ).expect("oopsie fill_text");
    }
  }
}
fn paint_ui_offers(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, mouse_state: &MouseState, cell_selection: &CellSelection) {
  let ( is_mouse_over_offer, offer_hover_index ) =
    if mouse_state.is_dragging || mouse_state.was_dragging { ( false, 0 ) } // Drag start is handled elsewhere, while dragging do not highlight offers
    else { ( mouse_state.offer_hover, mouse_state.offer_hover_offer_index ) };

  let mut inc = 0;
  for offer_index in 0..factory.available_parts_rhs_menu.len() {
    let ( part_index, part_interactable ) = factory.available_parts_rhs_menu[offer_index];
    if part_interactable {
      let highlight = (is_mouse_over_offer && offer_index == offer_hover_index) || (mouse_state.offer_selected && mouse_state.offer_selected_index == offer_index);
      paint_ui_offer(options, state, config, context, factory, mouse_state, cell_selection, offer_index, part_index, inc, highlight, config.nodes[part_index].pattern_unique_kinds.len() > 0);
      inc += 1;
    }
  }
}
fn paint_lasers(options: &Options, state: &mut State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>) {
  // Paint quote lasers (parts that are received draw a line to the left menu)
  let mut i = state.lasers.len();
  while i > 0 {
    i -= 1;

    let coord = state.lasers[i].coord;
    let (x, y) = to_xy(coord);
    let visible_quest_index = state.lasers[i].visible_quest_index;
    let color = &state.lasers[i].color;

    context.set_stroke_style(&color.into());
    context.begin_path();
    context.move_to(GRID_X1 + x as f64 * CELL_W + CELL_W / 2.0, GRID_Y1 + y as f64 * CELL_H + CELL_H / 2.0);
    context.line_to(GRID_X0 as f64 + UI_QUOTES_WIDTH as f64 / 2.0, GRID_Y1 + (UI_QUEST_HEIGHT + UI_QUEST_MARGIN) as f64 * visible_quest_index as f64 + (UI_QUEST_HEIGHT as f64) / 2.0);
    context.stroke();

    state.lasers[i].ttl -= 1;
    if state.lasers[i].ttl <= 0 {
      state.lasers.remove(i);
    }
  }
}
fn paint_trucks(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &mut Factory, img_dumptruck: &web_sys::HtmlImageElement) {
  // Paint trucks
  let truck_dur_1 = 3.0 * options.speed_modifier_ui; // seconds trucks take to cross the first part
  let truck_dur_2 = 1.0 * options.speed_modifier_ui; // turning circle
  let truck_dur_3 = 5.0 * options.speed_modifier_ui; // time to get up
  let truck_size = 50.0;
  let start_x = UI_MENU_BOTTOM_MACHINE_X + UI_MENU_BOTTOM_MACHINE_WIDTH - (truck_size + 5.0);
  let end_x = GRID_X2 + 5.0;
  // paint dump truck so it starts under the factory
  for t in 0..factory.trucks.len() {
    if factory.trucks[t].delay > 0 {
      continue;
    }

    // Draw dump truck at proper position // TODO: prevent overlapping of multiples etc
    // The first n seconds are spent driving under the floor to the right and then a corner
    // The rest is however long it takes to reach the final location where the button is created
    let ticks_since_truck = (factory.ticks - factory.trucks[t].created_at) as f64 / options.speed_modifier_floor;
    let time_since_truck = ticks_since_truck / (ONE_SECOND as f64);
    if time_since_truck < truck_dur_1 {
      let truck_x = start_x + (time_since_truck / truck_dur_1).min(1.0).max(0.0) * (end_x - start_x);
      let truck_y = UI_MENU_BOTTOM_MACHINE_Y + (UI_MENU_BOTTOM_MACHINE_HEIGHT / 2.0) - (truck_size / 2.0); // Factory mid

      context.save();
      // This is how canvas rotation works; you rotate around the center of what you're painting, paint it, then reset the translation matrix.
      // For this reason we must find the center of the dump truck, rotate around that point, and draw the dump track at minus half its size.
      context.translate(truck_x + truck_size / 2.0, truck_y + truck_size / 2.0).expect("oopsie translate");
      // pi/2 = quarter circle. what you draw upward will end up pointing to the right, which is what we want.
      context.rotate(std::f64::consts::FRAC_PI_2).expect("oopsie rotate");
      // Compensate for the origin currently being in the middle of the dump truck. Top-left is just easier.
      context.translate(-truck_size/2.0, -truck_size/2.0).expect("oopsie translate");
      // The truck starts _inside_ the factory and drives to the right (maybe slanted)
      context.draw_image_with_html_image_element_and_dw_and_dh(&img_dumptruck, 0.0, 0.0, truck_size, truck_size).expect("oopsie draw_image_with_html_image_element_and_dw_and_dh");
      // Paint the part icon on the back of the trick (x-centered, y-bottom)
      paint_segment_part_from_config(&options, &state, &config, &context, factory.trucks[t].part_index, 0.0 + (truck_size / 2.0) - ((truck_size / 3.0) / 2.0), 0.0 + truck_size + -6.0 + -(truck_size / 3.0), truck_size / 3.0, truck_size / 3.0);
      context.restore();
    } else if time_since_truck < (truck_dur_1 + truck_dur_2) {
      let progress = ((time_since_truck - truck_dur_1) / truck_dur_2).min(1.0).max(0.0);
      let truck_x = end_x + progress * 20.0;
      let truck_y = UI_MENU_BOTTOM_MACHINE_Y + (UI_MENU_BOTTOM_MACHINE_HEIGHT / 2.0) - (truck_size / 2.0) + (progress * -50.0); // Turn upward

      context.save();
      // This is how canvas rotation works; you rotate around the center of what you're painting, paint it, then reset the translation matrix.
      // For this reason we must find the center of the dump truck, rotate around that point, and draw the dump track at minus half its size.
      context.translate(truck_x + truck_size / 2.0, truck_y + truck_size / 2.0).expect("oopsie translate");
      // Note: same as before but we turn less as we progress in the turn
      context.rotate(std::f64::consts::FRAC_PI_2 * (1.0 - progress)).expect("oopsie rotate");
      // Compensate for the origin currently being in the middle of the dump truck. Top-left is just easier.
      context.translate(-truck_size/2.0, -truck_size/2.0).expect("oopsie translate");
      // The truck starts _inside_ the factory and drives to the right (maybe slanted)
      context.draw_image_with_html_image_element_and_dw_and_dh(&img_dumptruck, 0.0, 0.0, truck_size, truck_size).expect("oopsie draw_image_with_html_image_element_and_dw_and_dh");
      // Paint the part icon on the back of the trick (x-centered, y-bottom)
      paint_segment_part_from_config(&options, &state, &config, &context, factory.trucks[t].part_index, 0.0 + (truck_size / 2.0) - ((truck_size / 3.0) / 2.0), 0.0 + truck_size + -6.0 + -(truck_size / 3.0), truck_size / 3.0, truck_size / 3.0);
      context.restore();
    } else if time_since_truck < (truck_dur_1 + truck_dur_2 + truck_dur_3) {
      // Get target coordinate where this part will be permanently drawn so we know where the truck has to move to
      let ( target_x, target_y ) = get_offer_xy(factory.trucks[t].target_menu_part_position);

      let progress = ((time_since_truck - (truck_dur_1 + truck_dur_2)) / truck_dur_3).min(1.0).max(0.0);
      let truck_x = end_x + 20.0;
      let truck_y = UI_MENU_BOTTOM_MACHINE_Y + (UI_MENU_BOTTOM_MACHINE_HEIGHT / 2.0) - (truck_size / 2.0) + -50.0; // Turn upward

      let x = truck_x + (target_x - truck_x) * progress;
      let y = truck_y + (target_y - truck_y) * progress;

      context.draw_image_with_html_image_element_and_dw_and_dh(&img_dumptruck, x, y, truck_size, truck_size).expect("oopsie draw_image_with_html_image_element_and_dw_and_dh");
      // Paint the part icon on the back of the trick (x-centered, y-bottom)
      paint_segment_part_from_config(&options, &state, &config, &context, factory.trucks[t].part_index, x + (truck_size / 2.0) - ((truck_size / 3.0) / 2.0), y + truck_size + -6.0 + -(truck_size / 3.0), truck_size / 3.0, truck_size / 3.0);
    } else {
      // Truck reached its destiny.
      // - Enable the button
      // - Drop the truck
      factory.available_parts_rhs_menu[factory.trucks[t].target_menu_part_position].1 = true;
    }
  }
}
fn paint_ui_offer_hover_droptarget_hint_conditionally(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, mouse_state: &MouseState, cell_selection: &CellSelection) {
  let ( is_mouse_over_offer, offer_hover_index ) =
    if mouse_state.is_dragging || mouse_state.was_dragging { ( false, 0 ) } // Drag start is handled elsewhere, while dragging do not highlight offers
    else { ( mouse_state.offer_hover, mouse_state.offer_hover_offer_index ) };

  // While not dragging, paint colored overlays over machines to indicate current eligibility.
  // For example, if a part a requires part b a nd c in its pattern, mark only those machines
  // eligible who have received parts b and c already. For now, red/green will have to do, even
  // though that's not very color blind friendly. TODO: work around that.

  if !is_mouse_over_offer {
    return;
  }

  // Skip this if any machine is currently selected because that risks being destructive to the UI.
  if cell_selection.on && factory.floor[cell_selection.coord].kind == CellKind::Machine {
    return;
  }

  let hover_part_index: PartKind = factory.available_parts_rhs_menu[mouse_state.offer_hover_offer_index].0;
  paint_ui_offer_hover_droptarget_hint(options, state, config, context, factory, hover_part_index);
}
fn paint_ui_offer_hover_droptarget_hint(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, part_index: usize) {
  // Parts with patterns go to machines. Parts without patterns (or empty patterns) are suppliers.
  // Machines will accept in both cases (one case adds the part, the other sets the pattern)

  paint_machines_droptarget_green(options, state, config, context, factory, config.nodes[part_index].pattern_by_index.len() > 0);
}
fn paint_machines_droptarget_green(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, for_pattern: bool) {
  // TODO: did I not have a shortcut to iterate over just machine cells?
  for coord in 0..factory.floor.len() {
    let (x, y) = to_xy(coord);
    if (!for_pattern && is_edge_not_corner(x as f64, y as f64)) || (for_pattern && factory.floor[coord].kind == CellKind::Machine) {
      let yo = get_drop_color(options, factory.ticks).to_string();
      context.set_fill_style(&yo.into());
      // context.set_fill_style(&"#00ff0077".into());
    } else {
      context.set_fill_style(&"#6b6b6b90".into());
    }
    context.fill_rect(UI_FLOOR_OFFSET_X + x as f64 * CELL_W, UI_FLOOR_OFFSET_Y + y as f64 * CELL_H, CELL_W, CELL_H);
  }
}
fn get_drop_color(options: &Options, ticks: u64) -> String {
  let color_offset = options.dropzone_color_offset; // 75
  let bounce_speed = options.dropzone_bounce_speed; // 10
  let bounce_distance = options.dropzone_bounce_distance; // 150
  let bounce_d2 = bounce_distance * 2;
  let mut p = ((((ticks as f64) / options.speed_modifier_floor * options.speed_modifier_ui) as u64) / bounce_speed) % bounce_d2;
  if p > bounce_distance { p = bounce_distance - (p - bounce_distance); } // This makes the color bounce rather than jump from black to green
  let yo = format!("#00{:02x}0077", p+ color_offset);
  return yo;
}
fn paint_ui_offer(
  options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, mouse_state: &MouseState, cell_selection: &CellSelection,
  offer_index: usize, part_index: usize, inc: usize, highlight: bool, is_machine_part: bool
) {
  let ( x, y ) = get_offer_xy(inc);

  if is_machine_part {
    context.set_fill_style(&"#c99110".into());
    context.fill_rect(x, y, UI_OFFERS_WIDTH, UI_OFFERS_HEIGHT);
  } else {
    paint_dock_stripes(options, state, config, context, CONFIG_NODE_DOCK_UP, x, y, UI_OFFERS_WIDTH, UI_OFFERS_HEIGHT);
  }

  let px = x + (UI_OFFERS_WIDTH / 2.0) - (CELL_W / 2.0);
  let py = y + (UI_OFFERS_HEIGHT / 2.0) - (CELL_H / 2.0);
  paint_segment_part_from_config(options, state, config, context, part_index, px, py, CELL_W, CELL_H);

  if highlight {
    context.set_stroke_style(&"black".into());
    if config.nodes[part_index].pattern_unique_kinds.len() > 0 {
      // Draw tiny machine (with arrow?)
      // Draw tiny parts

      let x = GRID_X2 + 15.0;
      let y = GRID_Y0 + 10.0;

      let machine_img = &config.sprite_cache_canvas[config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].file_canvas_cache_index];
      context.draw_image_with_html_image_element_and_dw_and_dh(machine_img, x, y, 0.75 * CELL_W, 0.75 * CELL_H).expect("something error draw_image"); // requires web_sys HtmlImageElement feature

      for i in 0..config.nodes[part_index].pattern_unique_kinds.len() {
        // Paint tiny output part
        paint_segment_part_from_config(options, state, config, context, config.nodes[part_index].pattern_unique_kinds[i], x + CELL_W + (CELL_W * 0.5 + 5.0) * i as f64, y + CELL_H * 0.125, 0.5 * CELL_W, 0.5 * CELL_H);
      }
    }
  } else {
    context.set_stroke_style(&"white".into());
  }
  context.stroke_rect(x, y, UI_OFFERS_WIDTH, UI_OFFERS_HEIGHT);

  let div = 50;

  // If current selected machine can paint this offer, paint some green rotating pixel around it
  // TODO: make this more performant. Maybe by pregenerated image or by pregenerating them onstart?
  let selected_coord = cell_selection.coord;
  let selected_main_coord = factory.floor[selected_coord].machine.main_coord;
  if
    cell_selection.on &&
    factory.floor[selected_coord].kind == CellKind::Machine &&
    config.nodes[part_index].pattern_unique_kinds.len() > 0 &&
    config.nodes[part_index].pattern_unique_kinds.iter().all(|part_index| {
      return factory.floor[selected_main_coord].machine.last_received_parts.contains(part_index);
    })
  {
    // paint some pixels green? (https://colordesigner.io/gradient-generator)
    paint_green_pixel(context, factory.ticks + 0 * div, x, y, div, "#9ac48b");
    paint_green_pixel(context, factory.ticks + 1 * div, x, y, div, "#8ebd7f");
    paint_green_pixel(context, factory.ticks + 2 * div, x, y, div, "#83b773");
    paint_green_pixel(context, factory.ticks + 3 * div, x, y, div, "#77b066");
    paint_green_pixel(context, factory.ticks + 4 * div, x, y, div, "#6baa5a");
    paint_green_pixel(context, factory.ticks + 5 * div, x, y, div, "#5fa34e");
    paint_green_pixel(context, factory.ticks + 7 * div, x, y, div, "#539c42");
    paint_green_pixel(context, factory.ticks + 8 * div, x, y, div, "#459635");
    paint_green_pixel(context, factory.ticks + 9 * div, x, y, div, "#368f27");
  }
}
fn paint_green_pixel(context: &Rc<web_sys::CanvasRenderingContext2d>, ticks: u64, x: f64, y: f64, div: u64, color: &str) {
  context.set_stroke_style(&color.into());
  let border_len = (UI_OFFERS_WIDTH + UI_OFFERS_HEIGHT + UI_OFFERS_WIDTH + UI_OFFERS_HEIGHT) as u64;
  let pos = ((ticks/div) % border_len) as f64;
  let fx = x as f64;
  let fy = y as f64;
  if pos < UI_OFFERS_WIDTH {
    context.stroke_rect(fx + pos, fy, 1.0, 1.0);
  } else if pos < UI_OFFERS_WIDTH + UI_OFFERS_HEIGHT {
    context.stroke_rect(fx + UI_OFFERS_WIDTH, fy + (pos - UI_OFFERS_WIDTH), 1.0, 1.0);
  } else if pos < UI_OFFERS_WIDTH + UI_OFFERS_HEIGHT + UI_OFFERS_WIDTH {
    context.stroke_rect(fx + UI_OFFERS_WIDTH - (pos - (UI_OFFERS_WIDTH + UI_OFFERS_HEIGHT)), fy + UI_OFFERS_HEIGHT, 1.0, 1.0);
  } else {
    context.stroke_rect(fx, fy + UI_OFFERS_HEIGHT - (pos - (UI_OFFERS_WIDTH + UI_OFFERS_HEIGHT + UI_OFFERS_WIDTH)), 1.0, 1.0);
  }
}
fn paint_bottom_menu(options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, img_machine_1_1: &web_sys::HtmlImageElement, mouse_state: &MouseState) {
  paint_ui_time_control(options, state, context, mouse_state);
  paint_paint_toggle(options, state, context, mouse_state);
  paint_machine_icon(options, state, context, img_machine_1_1, mouse_state);
  paint_ui_buttons(options, state, context, mouse_state);
  paint_ui_buttons2(options, state, context, mouse_state);
}
fn paint_paint_toggle(options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  if mouse_state.over_menu_button == MenuButton::PaintToggleButton {
    context.set_fill_style(&"#aaffaa".into());
  } else {
    context.set_fill_style(&"#aaa".into());
  }

  context.fill_rect(UI_MENU_BOTTOM_PAINT_TOGGLE_X, UI_MENU_BOTTOM_PAINT_TOGGLE_Y, UI_MENU_BOTTOM_PAINT_TOGGLE_WIDTH, UI_MENU_BOTTOM_PAINT_TOGGLE_HEIGHT);

  context.save();
  context.set_font(&"48px monospace");
  context.set_fill_style(&"black".into());
  context.fill_text("🖌", UI_MENU_BOTTOM_PAINT_TOGGLE_X + UI_MENU_BOTTOM_PAINT_TOGGLE_WIDTH / 2.0 - 24.0, UI_MENU_BOTTOM_PAINT_TOGGLE_Y + UI_MENU_BOTTOM_PAINT_TOGGLE_HEIGHT / 2.0 + 16.0).expect("canvas api call to work");
  if state.mouse_mode_mirrored {
    context.set_fill_style(&"red".into());
    context.fill_text("X", UI_MENU_BOTTOM_PAINT_TOGGLE_X + UI_MENU_BOTTOM_PAINT_TOGGLE_WIDTH / 2.0 - 12.0, UI_MENU_BOTTOM_PAINT_TOGGLE_Y + UI_MENU_BOTTOM_PAINT_TOGGLE_HEIGHT / 2.0 + 16.0).expect("canvas api call to work");
  }
  context.restore();

  // context.draw_image_with_html_image_element_and_dw_and_dh(
  //   &img_machine_1_1,
  //   // Paint onto canvas at
  //   UI_MENU_BOTTOM_PAINT_TOGGLE_X, UI_MENU_BOTTOM_PAINT_TOGGLE_Y, UI_MENU_BOTTOM_PAINT_TOGGLE_WIDTH, UI_MENU_BOTTOM_PAINT_TOGGLE_HEIGHT
  // ).expect("something error draw_image"); // requires web_sys HtmlImageElement feature

  context.set_stroke_style(&"black".into());
  context.stroke_rect(UI_MENU_BOTTOM_PAINT_TOGGLE_X, UI_MENU_BOTTOM_PAINT_TOGGLE_Y, UI_MENU_BOTTOM_PAINT_TOGGLE_WIDTH, UI_MENU_BOTTOM_PAINT_TOGGLE_HEIGHT);
}
fn paint_machine_icon(options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, img_machine_1_1: &web_sys::HtmlImageElement, mouse_state: &MouseState) {
  context.set_fill_style(&"#aaa".into());
  context.fill_rect(UI_MENU_BOTTOM_MACHINE_X, UI_MENU_BOTTOM_MACHINE_Y, UI_MENU_BOTTOM_MACHINE_WIDTH, UI_MENU_BOTTOM_MACHINE_HEIGHT);

  context.draw_image_with_html_image_element_and_dw_and_dh(
    &img_machine_1_1,
    // Paint onto canvas at
    UI_MENU_BOTTOM_MACHINE_X, UI_MENU_BOTTOM_MACHINE_Y, UI_MENU_BOTTOM_MACHINE_WIDTH, UI_MENU_BOTTOM_MACHINE_HEIGHT
  ).expect("something error draw_image"); // requires web_sys HtmlImageElement feature

  context.set_stroke_style(&"black".into());
  context.stroke_rect(UI_MENU_BOTTOM_MACHINE_X, UI_MENU_BOTTOM_MACHINE_Y, UI_MENU_BOTTOM_MACHINE_WIDTH, UI_MENU_BOTTOM_MACHINE_HEIGHT);
}
fn paint_ui_buttons(options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  // See on_up_menu for events

  paint_ui_button(context, mouse_state, 0.0, "Blow", MenuButton::Row2Button0);
  paint_ui_button(context, mouse_state, 1.0, "Unbelt", MenuButton::Row2Button1);
  paint_ui_button(context, mouse_state, 2.0, "Unpart", MenuButton::Row2Button2);
  paint_ui_button(context, mouse_state, 3.0, "Undir", MenuButton::Row2Button3);
  paint_ui_button(context, mouse_state, 4.0, "Sample", MenuButton::Row2Button4);
  assert!(UI_MENU_BUTTONS_COUNT_WIDTH_MAX == 7.0, "Update after adding new buttons");
}
fn paint_ui_button(context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, index: f64, text: &str, button_id: MenuButton) {
  let x = UI_MENU_BUTTONS_OFFSET_X + index * (UI_MENU_BUTTONS_WIDTH + UI_MENU_BUTTONS_SPACING);
  let y = UI_MENU_BUTTONS_OFFSET_Y;

  if mouse_state.over_menu_button == button_id {
    context.set_fill_style(&"#eee".into());
  } else {
    context.set_fill_style(&"#aaa".into());
  }
  context.fill_rect(x, y, UI_MENU_BUTTONS_WIDTH, UI_MENU_BUTTONS_HEIGHT);
  context.set_stroke_style(&"black".into());
  context.stroke_rect(x, y, UI_MENU_BUTTONS_WIDTH, UI_MENU_BUTTONS_HEIGHT);
  context.set_fill_style(&"black".into());
  context.fill_text(text, x + 5.0, y + 14.0).expect("to paint");
}
fn paint_ui_buttons2(options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  // See on_up_menu for events

  paint_ui_button2(context, mouse_state, 0.0, if state.event_type_swapped { "Touch" } else { "Mouse" }, state.event_type_swapped, true, MenuButton::Row3Button0);
  paint_ui_button2(context, mouse_state, 1.0, "Select", state.mouse_mode_selecting, true, MenuButton::Row3Button1);
  paint_ui_button2(context, mouse_state, 2.0, if state.selected_area_copy.len() > 0{ "Stamp" } else { "Copy" }, state.selected_area_copy.len() > 0, state.mouse_mode_selecting, MenuButton::Row3Button2);
  // paint_ui_button2(context, mouse_state, 3.0, "Undo", false, state.snapshot_undo_pointer > 0, MenuButton::Row3Button3); // should it be 1 for initial map? or dont car, MenuButton::Row3Button3e?
  paint_ui_button2(context, mouse_state, 4.0, "Again", false, true, MenuButton::Row3Button4);
  paint_ui_button2(context, mouse_state, 5.0, "Panic", false, true, MenuButton::Row3Button5);
  // paint_ui_button2(context, mouse_state, 6.0, "Panic");
  assert!(UI_MENU_BUTTONS_COUNT_WIDTH_MAX == 7.0, "Update after adding new buttons");
}
fn paint_ui_button2(context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, index: f64, text: &str, on: bool, enabled: bool, button_id: MenuButton) {
  let x = UI_MENU_BUTTONS_OFFSET_X + index * (UI_MENU_BUTTONS_WIDTH + UI_MENU_BUTTONS_SPACING);
  let y = UI_MENU_BUTTONS_OFFSET_Y2;

  if !enabled {
    context.set_fill_style(&"#777".into());
  } else if on {
    context.set_fill_style(&"lightgreen".into());
  } else if mouse_state.over_menu_button == button_id {
    context.set_fill_style(&"#eee".into());
  } else {
    context.set_fill_style(&"#aaa".into());
  }
  context.fill_rect(x, y, UI_MENU_BUTTONS_WIDTH, UI_MENU_BUTTONS_HEIGHT);
  context.set_stroke_style(&"black".into());
  context.stroke_rect(x, y, UI_MENU_BUTTONS_WIDTH, UI_MENU_BUTTONS_HEIGHT);
  if enabled {
    context.set_fill_style(&"black".into());
  } else {
    context.set_fill_style(&"#ccc".into());
  }
  context.fill_text(text, x + 5.0, y + 14.0).expect("to paint");
}
fn paint_ui_time_control(options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  // paint_buttons
  paint_ui_speed_bubble(MenuButton::Row1ButtonMin, options, state, context, mouse_state, 0, "-");
  paint_ui_speed_bubble(MenuButton::Row1ButtonHalf, options, state, context, mouse_state, 1, "½");
  paint_ui_speed_bubble(MenuButton::Row1ButtonPlay, options, state, context, mouse_state, 2, "⏭"); // "play" / "pause" / Row1ButtonPlay
  paint_ui_speed_bubble(MenuButton::Row1Button2x, options, state, context, mouse_state, 3, "2");
  paint_ui_speed_bubble(MenuButton::Row1ButtonPlus, options, state, context, mouse_state, 4, "+");
}
fn paint_ui_speed_bubble(button: MenuButton, options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, index: usize, text: &str) {
  let cx = UI_SPEED_BUBBLE_OFFSET_X + (2.0 * UI_SPEED_BUBBLE_RADIUS + UI_SPEED_BUBBLE_SPACING) * (index as f64) + UI_SPEED_BUBBLE_RADIUS;
  let cy = UI_SPEED_BUBBLE_OFFSET_Y + UI_SPEED_BUBBLE_RADIUS;

  if mouse_state.over_menu_button == button {
    context.set_stroke_style(&"red".into()); // border
  } else {
    context.set_stroke_style(&"white".into()); // border
  }

  if text == "⏭" && options.speed_modifier_floor == 0.0 {
    context.set_fill_style(&"tomato".into());
  }
  else if text == "⏭" && options.speed_modifier_floor == 1.0 {
    context.set_fill_style(&"#0f0".into());
  }
  else if text == "½" && options.speed_modifier_floor == 0.5 {
    context.set_fill_style(&"#0f0".into());
  }
  else if text == "2" && options.speed_modifier_floor == 2.0 {
    context.set_fill_style(&"#0f0".into());
  }
  else if text == "-" && (options.speed_modifier_floor > 0.0 && options.speed_modifier_floor < 0.5) {
    context.set_fill_style(&"#0f0".into());
  }
  else if text == "+" && options.speed_modifier_floor > 2.0 {
    context.set_fill_style(&"#0f0".into());
  }
  else if mouse_state.over_menu_button == button {
    context.set_fill_style(&"#eee".into());
  }
  else {
    context.set_fill_style(&"#aaa".into());
  }

  context.begin_path();
  context.arc(cx, cy, UI_SPEED_BUBBLE_RADIUS, 0.0, 2.0 * 3.14).expect("to paint"); // cx/cy must be _center_ coord of the circle, not top-left
  context.fill();
  context.stroke();
  context.set_fill_style(&"black".into());
  context.fill_text(text, cx - 4.0, cy + 4.0).expect("to paint");
}
fn get_offer_xy(index: usize) -> (f64, f64 ) {
  let x = UI_OFFERS_OFFSET_X + (index as f64 % UI_OFFERS_PER_ROW).floor() * UI_OFFERS_WIDTH_PLUS_MARGIN;
  let y = UI_OFFERS_OFFSET_Y + (index as f64 / UI_OFFERS_PER_ROW).floor() * UI_OFFERS_HEIGHT_PLUS_MARGIN;

  return ( x, y );
}
fn paint_segment_part_from_config(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, segment_part_index: PartKind, dx: f64, dy: f64, dw: f64, dh: f64) -> bool {
  return paint_segment_part_from_config_bug(options, state, config, context, segment_part_index, dx, dy, dw, dh, false);
}
fn paint_segment_part_from_config_bug(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, segment_part_index: PartKind, dx: f64, dy: f64, dw: f64, dh: f64, bug: bool) -> bool {
  if segment_part_index == PARTKIND_NONE {
    return false;
  }

  assert!(config.nodes[segment_part_index].kind == ConfigNodeKind::Part, "segment parts should refer to part nodes but received index: {}, kind: {:?}, node: {:?}", segment_part_index, config.nodes[segment_part_index].kind, config.nodes[segment_part_index]);

  let (spx, spy, spw, sph, canvas) = part_to_sprite_coord_from_config(config, segment_part_index);
  if bug { log!("meh? {} {} {} {}: {:?} --> {:?}", spx, spy, spw, sph, segment_part_index, config.nodes[segment_part_index]); }

  // log!("wat: {} {} {} {}     {} {} {} {}", spx, spy, spw, sph , dx, dy, dw, dh,);
  // document().get_element_by_id("$tdb").unwrap().dyn_into::<web_sys::HtmlElement>().unwrap().append_child(&canvas).expect("to work");

  context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
    &canvas,
    // Sprite position
    spx, spy, spw, sph,
    // Paint onto canvas at
    dx, dy, dw, dh,
  ).expect("something error draw_image"); // requires web_sys HtmlImageElement feature

  if options.draw_part_borders {
    context.set_stroke_style(&"black".into());
    context.stroke_rect(dx, dy, dw, dh);
  }
  if options.draw_part_char_icon || options.draw_part_kind {
    context.set_fill_style(&"#ffffff99".into());
    context.fill_rect(dx, dy, dw, dh);
    context.set_fill_style(&"black".into());
    if options.draw_part_kind {
      context.fill_text(segment_part_index.to_string().as_str(), dx + dw / 2.0 - (if segment_part_index < 9 { 4.0 } else { 14.0 }), dy + dh / 2.0 + 3.0).expect("to paint");
    } else if segment_part_index == PARTKIND_NONE {
      context.fill_text("ε", dx + dw / 2.0 - 4.0, dy + dh / 2.0 + 3.0).expect("to paint");
    } else {
      context.fill_text(format!("{}", config.nodes[segment_part_index].icon).as_str(), dx + dw / 2.0 - 4.0, dy + dh / 2.0 + 3.0).expect("to paint");
    }
  }

  return true;
}
fn paint_asset(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, config_node_index: usize, ticks: u64, dx: f64, dy: f64, dw: f64, dh: f64) -> bool {
  assert!(config.nodes[config_node_index].kind == ConfigNodeKind::Asset, "assets should refer to Asset nodes but received index: {}, kind: {:?}, node: {:?}", config_node_index, config.nodes[config_node_index].kind, config.nodes[config_node_index]);

  let (spx, spy, spw, sph, canvas) = config_get_sprite_details(config, config_node_index, 0, true, ticks);

  // let (spx, spy, spw, sph, canvas) = asset_to_sprite_coord_from_config(config, config_node_index);
  // if bug { log!("meh? {} {} {} {}: {:?} --> {:?}", spx, spy, spw, sph, segment_part_index, config.nodes[segment_part_index]); }

  // log!("wat: {} {} {} {}     {} {} {} {}", spx, spy, spw, sph , dx, dy, dw, dh,);
  // document().get_element_by_id("$tdb").unwrap().dyn_into::<web_sys::HtmlElement>().unwrap().append_child(&canvas).expect("to work");

  context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
    &canvas,
    // Sprite position
    spx, spy, spw, sph,
    // Paint onto canvas at
    dx, dy, dw, dh,
  ).expect("something error draw_image"); // requires web_sys HtmlImageElement feature

  if options.draw_part_borders {
    context.set_stroke_style(&"black".into());
    context.stroke_rect(dx, dy, dw, dh);
  }
  if options.draw_part_char_icon || options.draw_part_kind {
    context.set_fill_style(&"#ffffff99".into());
    context.fill_rect(dx, dy, dw, dh);
    context.set_fill_style(&"black".into());
    if options.draw_part_kind {
      context.fill_text(config_node_index.to_string().as_str(), dx + dw / 2.0 - (if config_node_index < 9 { 4.0 } else { 14.0 }), dy + dh / 2.0 + 3.0).expect("to paint");
    } else if config_node_index == PARTKIND_NONE {
      context.fill_text("ε", dx + dw / 2.0 - 4.0, dy + dh / 2.0 + 3.0).expect("to paint");
    } else {
      context.fill_text(format!("{}", config.nodes[config_node_index].icon).as_str(), dx + dw / 2.0 - 4.0, dy + dh / 2.0 + 3.0).expect("to paint");
    }
  }

  return true;
}
fn hit_test_undo(x: f64, y: f64) -> bool {
  return bounds_check(x, y, UI_UNREDO_UNDO_OFFSET_X, UI_UNREDO_UNDO_OFFSET_Y, UI_UNREDO_UNDO_OFFSET_X + UI_UNREDO_UNDO_WIDTH, UI_UNREDO_UNDO_OFFSET_Y + UI_UNREDO_UNDO_HEIGHTH);
}
fn hit_test_clear(x: f64, y: f64) -> bool {
  return bounds_check(x, y, UI_UNREDO_CLEAR_OFFSET_X, UI_UNREDO_CLEAR_OFFSET_Y, UI_UNREDO_CLEAR_OFFSET_X + UI_UNREDO_CLEAR_WIDTH, UI_UNREDO_CLEAR_OFFSET_Y + UI_UNREDO_CLEAR_HEIGHTH);
}
fn hit_test_redo(x: f64, y: f64) -> bool {
  return bounds_check(x, y, UI_UNREDO_REDO_OFFSET_X, UI_UNREDO_REDO_OFFSET_Y, UI_UNREDO_REDO_OFFSET_X + UI_UNREDO_REDO_WIDTH, UI_UNREDO_REDO_OFFSET_Y + UI_UNREDO_REDO_HEIGHTH);
}
fn hit_test_save_map(x: f64, y: f64) -> usize {
  return
    if hit_test_save_map_rc(x, y, 0.0, 0.0) { 0 }
    else if hit_test_save_map_rc(x, y, 0.0, 1.0) { 1 }
    else if hit_test_save_map_rc(x, y, 1.0, 0.0) { 2 }
    else if hit_test_save_map_rc(x, y, 1.0, 1.0) { 3 }
    else { 100 };
}
fn hit_test_save_map_rc(x: f64, y: f64, row: f64, col: f64) -> bool {
  return bounds_check(
    x, y,
    GRID_X0 + UI_SAVE_THUMB_X1 + col * (UI_SAVE_THUMB_WIDTH + UI_SAVE_MARGIN), GRID_Y2 + UI_SAVE_THUMB_Y1 + row * (UI_SAVE_THUMB_HEIGHT + UI_SAVE_MARGIN),
    GRID_X0 + UI_SAVE_THUMB_X1 + col * (UI_SAVE_THUMB_WIDTH + UI_SAVE_MARGIN) + UI_SAVE_THUMB_WIDTH, GRID_Y2 + UI_SAVE_THUMB_Y1 + row * (UI_SAVE_THUMB_HEIGHT + UI_SAVE_MARGIN) + UI_SAVE_THUMB_HEIGHT,
  );
}
fn hit_test_save_map_left(x: f64, y: f64, row: f64, col: f64) -> bool {
  return bounds_check(
    x, y,
    GRID_X0 + UI_SAVE_THUMB_X1 + col * (UI_SAVE_THUMB_WIDTH + UI_SAVE_MARGIN), GRID_Y2 + UI_SAVE_THUMB_Y1 + row * (UI_SAVE_THUMB_HEIGHT + UI_SAVE_MARGIN),
    GRID_X0 + UI_SAVE_THUMB_X1 + col * (UI_SAVE_THUMB_WIDTH + UI_SAVE_MARGIN) + UI_SAVE_THUMB_WIDTH * 0.66, GRID_Y2 + UI_SAVE_THUMB_Y1 + row * (UI_SAVE_THUMB_HEIGHT + UI_SAVE_MARGIN) + UI_SAVE_THUMB_HEIGHT,
  );
}
fn hit_test_save_map_right(x: f64, y: f64, row: f64, col: f64) -> bool {
  return bounds_check(
    x, y,
    GRID_X0 + UI_SAVE_THUMB_X1 + col * (UI_SAVE_THUMB_WIDTH + UI_SAVE_MARGIN) + UI_SAVE_THUMB_WIDTH * 0.66, GRID_Y2 + UI_SAVE_THUMB_Y1 + row * (UI_SAVE_THUMB_HEIGHT + UI_SAVE_MARGIN),
    GRID_X0 + UI_SAVE_THUMB_X1 + col * (UI_SAVE_THUMB_WIDTH + UI_SAVE_MARGIN) + UI_SAVE_THUMB_WIDTH, GRID_Y2 + UI_SAVE_THUMB_Y1 + row * (UI_SAVE_THUMB_HEIGHT + UI_SAVE_MARGIN) + UI_SAVE_THUMB_HEIGHT,
  );
}
fn paint_load_thumbs(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, saves: &[Option<(web_sys::HtmlCanvasElement, String)>; 9], img_icon: &web_sys::HtmlImageElement) {
  paint_map_load_button(0.0, 0.0, 0, context, &saves[0], img_icon, mouse_state);
  paint_map_load_button(1.0, 0.0, 1, context, &saves[1], img_icon, mouse_state);
  paint_map_load_button(0.0, 1.0, 2, context, &saves[2], img_icon, mouse_state);
  paint_map_load_button(1.0, 1.0, 3, context, &saves[3], img_icon, mouse_state);
}
fn paint_map_load_button(col: f64, row: f64, button_index: usize, context: &Rc<web_sys::CanvasRenderingContext2d>, save: &Option<(web_sys::HtmlCanvasElement, String)>, img_icon: &web_sys::HtmlImageElement, mouse_state: &MouseState) {
  assert!(button_index < 6, "there are only 6 save buttons");
  let ox = GRID_X0 + UI_SAVE_THUMB_X1 + col * (UI_SAVE_THUMB_WIDTH + UI_SAVE_MARGIN);
  let oy = GRID_Y2 + UI_SAVE_THUMB_Y1 + row * (UI_SAVE_THUMB_HEIGHT + UI_SAVE_MARGIN);
  if let Some((canvas, name)) = save {
    // I'm using patterns to get around rounded corners but maybe should just use the mask
    // approach instead? Neither is very portable anyways so why not make it a simple blit...
    if let Some(ptrn) = context.create_pattern_with_html_canvas_element(&canvas, "repeat").expect("trying to load thumb") {
      let close = hit_test_save_map_right(mouse_state.world_x, mouse_state.world_y, row, col);

      round_rect(context, ox, oy, UI_SAVE_THUMB_WIDTH, UI_SAVE_THUMB_HEIGHT);
      context.save();
      // Note: the translate is necessary because the pattern anchor point is always 0.0 of window, not the canvas
      context.translate(ox, oy).expect("canvas api call to work");
      context.set_fill_style(&ptrn);
      context.fill();
      context.restore();
      context.set_stroke_style(&"black".into());
      context.stroke();

      // Paint trash button
      let fill_color = if mouse_state.over_save_map && mouse_state.over_save_map_index == button_index && close { "#ffaaaa" } else { "#aaaaaa" };
      round_rect_and_fill_stroke(context, ox + UI_SAVE_THUMB_WIDTH * 0.66, oy, UI_SAVE_THUMB_WIDTH * 0.33, UI_SAVE_THUMB_HEIGHT, fill_color, "black");
      context.set_fill_style(&"red".into());
      context.fill_text("X", ox + UI_SAVE_THUMB_WIDTH - 20.0, oy + UI_SAVE_THUMB_HEIGHT / 2.0 + 5.0).expect("canvas api call to work");
    } else {
      round_rect_and_fill_stroke(context, ox, oy, UI_SAVE_THUMB_WIDTH, UI_SAVE_THUMB_HEIGHT, "orange", "black");
    }
  } else {
    let fill_color = if mouse_state.over_save_map && mouse_state.over_save_map_index == button_index { "#aaffaa" } else { "#aaaaaa" };
    round_rect_and_fill_stroke(context, ox, oy, UI_SAVE_THUMB_WIDTH, UI_SAVE_THUMB_HEIGHT, fill_color, "black");
    context.draw_image_with_html_image_element_and_dw_and_dh(
      img_icon,
      ox + UI_SAVE_THUMB_WIDTH * 0.35,
      oy + UI_SAVE_THUMB_HEIGHT * 0.2,
      UI_SAVE_THUMB_WIDTH / 3.0,
      UI_SAVE_THUMB_HEIGHT / 2.0
    ).expect("canvas api call to work");
  }
}

fn paint_map_state_buttons(state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, img_icon: &web_sys::HtmlImageElement) {
  // // Paint trash button
  context.save();
  context.set_font(&"48px monospace");

  let fill_color = if state.snapshot_undo_pointer <= 0 { "#777" } else if mouse_state.over_undo { "#aaffaa" } else { "#aaaaaa" };
  round_rect_and_fill_stroke(context, UI_UNREDO_UNDO_OFFSET_X, UI_UNREDO_UNDO_OFFSET_Y, UI_UNREDO_UNDO_WIDTH, UI_UNREDO_UNDO_HEIGHTH, fill_color, "black");
  let text_color = if state.snapshot_undo_pointer <= 0 { "#ccc" } else { "black" };
  context.set_fill_style(&text_color.into());
  context.fill_text("↶", UI_UNREDO_UNDO_OFFSET_X + UI_UNREDO_UNDO_WIDTH / 2.0 - 16.0, UI_UNREDO_UNDO_OFFSET_Y + UI_UNREDO_UNDO_HEIGHTH / 2.0 + 16.0).expect("canvas api call to work");

  round_rect_and_fill_stroke(context, UI_UNREDO_CLEAR_OFFSET_X, UI_UNREDO_CLEAR_OFFSET_Y, UI_UNREDO_CLEAR_WIDTH, UI_UNREDO_CLEAR_HEIGHTH, if mouse_state.over_clear { "#aaffaa" } else { "#aaaaaa" }, "black");
  context.set_fill_style(&"black".into());
  context.fill_text("🗑", UI_UNREDO_CLEAR_OFFSET_X + UI_UNREDO_UNDO_WIDTH / 2.0 - 15.0, UI_UNREDO_CLEAR_OFFSET_Y + UI_UNREDO_CLEAR_HEIGHTH / 2.0 + 16.0).expect("canvas api call to work");
  // 🚮

  let fill_color = if state.snapshot_undo_pointer == state.snapshot_pointer { "#777" } else if mouse_state.over_redo { "#aaffaa" } else { "#aaaaaa" };
  round_rect_and_fill_stroke(context, UI_UNREDO_REDO_OFFSET_X, UI_UNREDO_REDO_OFFSET_Y, UI_UNREDO_REDO_WIDTH, UI_UNREDO_REDO_HEIGHTH, fill_color, "black");
  let text_color = if state.snapshot_undo_pointer == state.snapshot_pointer { "#ccc" } else { "black" };
  context.set_fill_style(&text_color.into());
  context.fill_text("↷", UI_UNREDO_REDO_OFFSET_X + UI_UNREDO_REDO_WIDTH / 2.0 - 16.0, UI_UNREDO_REDO_OFFSET_Y + UI_UNREDO_REDO_HEIGHTH / 2.0 + 16.0).expect("canvas api call to work");

  context.restore();
}


fn round_rect_and_fill_stroke(context: &Rc<web_sys::CanvasRenderingContext2d>, x: f64, y: f64, w: f64, h: f64, fill: &str, stroke: &str) {
  round_rect(context, x, y, w, h);
  context.set_fill_style(&fill.into());
  context.fill();
  context.set_stroke_style(&stroke.into());
  context.stroke();
}
fn round_rect(context: &Rc<web_sys::CanvasRenderingContext2d>, x: f64, y: f64, w: f64, h: f64) {
  // web_sys is not exposing the new roundRect so this SO answer will have to do
  // https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/roundRect
  let mut r = 10.0;
  if w < 2.0 * r { r = w / 2.0; }
  if h < 2.0 * r { r = h / 2.0; }
  context.begin_path();
  context.move_to(x+r, y);
  context.arc_to(x+w, y,   x+w, y+h, r).expect("canvas api call to work");
  context.arc_to(x+w, y+h, x,   y+h, r).expect("canvas api call to work");
  context.arc_to(x,   y+h, x,   y,   r).expect("canvas api call to work");
  context.arc_to(x,   y,   x+w, y,   r).expect("canvas api call to work");
  context.close_path();
}

fn paint_factory_belt(options: &Options, state: &State, config: &Config, factory: &Factory, coord: usize, context: &Rc<web_sys::CanvasRenderingContext2d>, dx: f64, dy: f64, dw: f64, dh: f64) {
  if !options.paint_belts {
    return;
  }

  let belt_type = factory.floor[coord].belt.meta.btype;
  let sprite_start_at = factory.floor[coord].belt.sprite_start_at;

  paint_belt(options, state, config, context, dx, dy, dw, dh, belt_type, sprite_start_at, factory.ticks);
}
fn paint_zero_belt(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, dx: f64, dy: f64, dw: f64, dh: f64, belt_type: BeltType) {
  paint_belt(options, state, config, context, dx, dy, dw, dh, belt_type, 0, 0);
}
fn paint_belt(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, dx: f64, dy: f64, dw: f64, dh: f64, belt_type: BeltType, sprite_start_at: u64, ticks: u64) {
  if !options.paint_belts {
    return;
  }

  let (spx, spy, spw, sph, canvas) = config_get_sprite_for_belt_type(config, belt_type, sprite_start_at, ticks);

  context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
    &canvas,
    // Sprite position
    spx, spy, spw, sph,
    // Paint onto canvas at
    dx, dy, dw, dh,
  ).expect("paint_belt() something error draw_image"); // requires web_sys HtmlImageElement feature
}

fn paint_supplier(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, dx: f64, dy: f64, dw: f64, dh: f64, sprite_start_at: u64, ticks: u64, coord: usize) {
  let (x, y) = to_xy(coord);
  let supply_kind =
    if y == 0 {
      CONFIG_NODE_SUPPLY_UP
    } else if x == FLOOR_CELLS_W-1 {
      CONFIG_NODE_SUPPLY_RIGHT
    } else if y == FLOOR_CELLS_H-1 {
      CONFIG_NODE_SUPPLY_DOWN
    } else if x == 0 {
      CONFIG_NODE_SUPPLY_LEFT
    } else {
      panic!("no");
    };

  let (spx, spy, spw, sph, canvas) = config_get_sprite_details(config, supply_kind, sprite_start_at, false, ticks);

  context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
    &canvas,
    // Sprite position
    spx, spy, spw, sph,
    // Paint onto canvas at
    dx, dy, dw, dh,
  ).expect("paint_supplier() something error draw_image"); // requires web_sys HtmlImageElement feature
}

fn paint_demander(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, dx: f64, dy: f64, dw: f64, dh: f64, last_part_at: u64, ticks: u64, coord: usize) {
  let (x, y) = to_xy(coord);
  let demand_kind =
    if y == 0 {
      CONFIG_NODE_DEMAND_UP
    } else if x == FLOOR_CELLS_W-1 {
      CONFIG_NODE_DEMAND_RIGHT
    } else if y == FLOOR_CELLS_H-1 {
      CONFIG_NODE_DEMAND_DOWN
    } else if x == 0 {
      CONFIG_NODE_DEMAND_LEFT
    } else {
      panic!("no");
    };

  // TODO: do not loop
  let (spx, spy, spw, sph, canvas) = config_get_sprite_details(config, demand_kind, last_part_at, false, ticks);

  context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
    &canvas,
    // Sprite position
    spx, spy, spw, sph,
    // Paint onto canvas at
    dx, dy, dw, dh,
  ).expect("paint_demander() something error draw_image"); // requires web_sys HtmlImageElement feature

  // return
  //
  //
  // let dock_dir =
  //   match dir {
  //     Direction::Up => CONFIG_NODE_DEMAND_UP,
  //     Direction::Right => CONFIG_NODE_DEMAND_RIGHT,
  //     Direction::Down => CONFIG_NODE_DEMAND_DOWN,
  //     Direction::Left => CONFIG_NODE_DEMAND_LEFT,
  //   };
  //
  // // TODO: should we offer the option to draw the dock behind in case of semi-transparent supply imgs?
  // context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
  //   &config.sprite_cache_canvas[config.nodes[dock_dir].sprite_config.frames[0].file_canvas_cache_index],
  //   config.nodes[dock_dir].sprite_config.frames[0].x, config.nodes[dock_dir].sprite_config.frames[0].y, config.nodes[dock_dir].sprite_config.frames[0].w, config.nodes[dock_dir].sprite_config.frames[0].h,
  //   ox, oy, dw, dh
  // ).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
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

fn unpart(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, already_changed: bool) {
  for coord in 0..factory.floor.len() {
    clear_part_from_cell(options, state, config, factory, coord);
  }
  if !already_changed {
    factory.changed = true;
  }
}

fn parse_and_save_options_string(option_string: String, options: &mut Options, strict: bool, options_started_from_source: u64, on_load: bool) {
  log!("parse_and_save_options_string():\n{} with {}", option_string, options_started_from_source);
  let bak = options.initial_map_from_source;
  parse_options_into(option_string, options, true);
  options.options_started_from_source = options_started_from_source; // This prop will be overwritten by the above, first
  options.initial_map_from_source = bak; // Do not overwrite this.
  let exp = options_serialize(options);

  log!("Storing options into browser localStorage... ({} bytes)", exp.len());
  let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
  if !on_load {
    local_storage.set_item("factini.options", exp.as_str()).unwrap();
  }

  // Update UI to reflect actually loaded options
  setGameOptions(exp.into(), on_load.into());
}
