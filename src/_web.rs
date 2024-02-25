// This file should only be included for `wasm-pack build --target web`
// The main.rs will include this file when `#[cfg(target_arch = "wasm32")]`


// Compile with --profile to try and get some sense of shit

// road to release
// - maze
//   - maze fuel could blow-up-fade-out when collected, with a 3x for the better one, maybe rainbow wiggle etc? or just 1x 2x 3x instead of icon
// - help the player
//   - update tutorial with current status
//   - something with that ikea help icon
//   - add hint that two machines next to each other do not share port?
//   - for touch, clicking the floor with an atom or woop selected should create that there
//   - grid snapping for woops should ignore half cell grace period for edges
//   - keep showing tooltip while dragging woop
// - cleanup
//   - repo
// - bug
//   - ai will use woops as suppliers
//   - full maze not enabled by default
//   - clone as a button?

// features
// - belts
//   - does snaking bother me when a belt should move all at once or not at all? should we change the algo? probably not that hard to move all connected cells between intersections/entry/exit points at once. if one moves, all move, etc.
//   - a part that reaches 100% of a cell but can't be moved to the side should not block the next part from entering the cell until all ports are taken like that. the part can sit in the port and a belt can only take parts if it has an available port.
// - machines
//   - investigate different machine speeds at different configs
//   - throughput problem. part has to wait at 50% for next part to clear, causing delays. if there's enough outputs there's always room and no such delay. if supply-to-machine is one belt there's also no queueing so it's faster
//   - animate machines at work
//   - paint the prepared parts of a machine while not selected?
//   - make the menu-machine "process" (-> animation) the finished parts before generating trucks
//   - machine top layer should paint _over_ the parts. machines could be nicer with how they ingest the parts that arrive there.
//   - machine image could use different sprite depending on which ports are connected. requires some sprite painting changes and some pretty cool machine art to support it proper. would be nice.
// - import export
//   - do we want/need to support serialization of maps with more than 60 machines? 2x2 can only go up to 49. but 2x1 or 1x2 would double that, up to 84. if not we should gracefully handle it rather than let it throw
// - animations
//   - certain things should be painted as a background layer once
// - make maze prettier
//   - the whole thing could just explode into animated particles once you finish or smth
//   - what does the maze runner ultimately find that allows you to move to the next level? (no plus, one plus, two plus)
// - convert maze to rgb and implement some kind of image thing
// - auto build
//   - allow to let it run continuously.
// - clicking on machine should cycle through available parts
// - clicking on empty machine should select one of the parts it can create based on the history
// - machine hint based on history received should decay
// - copy paste should copy machines too? why not
// - experiment with bigger maps and scrolling
// - help the player
//   - help, okay, secret menu
// - implement undo/redo action support
//   - https://dev.to/chromiumdev/-native-undo--redo-for-the-web-3fl3  https://github.com/samthor/undoer
// - cleanup
//   - get rid of CONFIG_NODE_MACHINE_1X1 in favor of CONFIG_NODE_ASSET_MACHINE_3_3 etc
// - allow machine icons (weewoo, output, etc) position to be configurable through config MD

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

use super::atom::*;
use super::auto::*;
use super::belt::*;
use super::belt_type::*;
use super::bouncer::*;
use super::cell::*;
use super::canvas::*;
use super::cli_serialize::*;
use super::config::*;
use super::direction::*;
use super::factory::*;
use super::floor::*;
use super::init::*;
use super::options::*;
use super::machine::*;
use super::maze::*;
use super::part::*;
use super::paste::*;
use super::port::*;
use super::port_auto::*;
use super::prio::*;
use super::quest_state::*;
use super::quick_save::*;
use super::quest::*;
use super::state::*;
use super::truck::*;
use super::utils::*;
use super::woop::*;
use super::zone::*;

// This explicitly import shoulnd't be necessary anymore according to https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files but ... well I did at the time of writing.
use super::log;

// Temp placeholder
const COLOR_SUPPLY: &str = "pink";
const COLOR_SUPPLY_SEMI: &str = "#6f255154";
const COLOR_DEMAND: &str = "lightgreen";
const COLOR_DEMAND_SEMI: &str = "#00aa0055";
const COLOR_MACHINE: &str = "lightyellow";
const COLOR_MACHINE_SEMI: &str = "#aaaa0099";

// Index on button_canvii
const BUTTON_PRERENDER_INDEX_SMALL_SQUARE_UP: usize = 0;
const BUTTON_PRERENDER_INDEX_SMALL_SQUARE_DOWN: usize = 1;
const BUTTON_PRERENDER_INDEX_MEDIUM_SQUARE_UP: usize = 2;
const BUTTON_PRERENDER_INDEX_MEDIUM_SQUARE_DOWN: usize = 3;
const BUTTON_PRERENDER_INDEX_SAVE_BIG_UP: usize = 4;
const BUTTON_PRERENDER_INDEX_SAVE_BIG_DOWN: usize = 5;
const BUTTON_PRERENDER_INDEX_SAVE_THIN_UP: usize = 6;
const BUTTON_PRERENDER_INDEX_SAVE_THIN_DOWN: usize = 7;

const FLOOR_YELLOW_COLOR: &str = "#ffcf8e";
const BUTTON_COLOR_BORDER_DARK: &str = "#24142c";
const BUTTON_COLOR_BORDER_LIGHT: &str = "#928fb8";
const BUTTON_COLOR_BACK: &str = "#392946";
const BUTTON_COLOR_FRONT: &str = "#ffcf8e"; // Same as floor
const MACHINE_ORANGE: &str = "#c99110";

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
  pub fn onQuestUpdate(node: JsValue);
  pub fn getLastPaste() -> String; // queuedPaste
  pub fn getCurrentPaste(); // navigator.clipboard.readText() into action=paste. wont work in firefox.
  pub fn copyToClipboard(str: JsValue) -> bool; // navigator.clipboard.writeText(str). does work in firefox too.
  pub fn tryFullScreenFromJS() -> String;
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
  canvas.set_width(CANVAS_PIXEL_INITIAL_WIDTH as u32);
  canvas.set_height(CANVAS_PIXEL_INITIAL_HEIGHT as u32);
  canvas.style().set_property("border", "solid")?;
  canvas.style().set_property("width", format!("{}px", CANVAS_CSS_INITIAL_WIDTH as u32).as_str())?;
  canvas.style().set_property("height", format!("{}px", CANVAS_CSS_INITIAL_HEIGHT as u32).as_str())?;
  canvas.style().set_property("background-image", "url(./img/sand.png)").expect("should work");
  canvas.style().set_property("id", "sand_bg").expect("should work");
  // This is prettier for us :)
  canvas.style().set_property("image-rendering", "pixelated").expect("should work");

  // TODO: this may improve perf a little bit but requires rewiring the background stuff
  // let context_options = js_sys::Object::new();
  // js_sys::Reflect::set(&context_options, &JsValue::from_str("alpha"), &JsValue::from_bool(false)).unwrap();
  // let context = canvas.get_context_with_context_options("2d", &context_options)?.unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>()?;

  let context = canvas.get_context("2d")?.unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>()?;
  let context = Rc::new(context);

  context.set_image_smoothing_enabled(false);

  // Static state configuration (can still be changed by user). Prefer localStorage over options.md
  let mut options = create_options(1.0, 1.0);
  // If there are options in localStorage, apply them now
  let saved_options = {
    log!("onload: Reading options from localStorage");
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage.get_item(LS_OPTIONS).unwrap()
  };
  let ( option_string, options_started_from_source ) = match saved_options {
    Some(str) => {
      log!("Using options json from localStorage ({} bytes)", str.len());
      ( str, false )
    },
    None => ( getGameOptions(), true ),
  };
  let options_started_from_source = if options_started_from_source { 0 } else { option_string.len() as u64 };
  parse_and_save_options_string(option_string.clone(), &mut options, true, options_started_from_source, true);

  let mut config = parse_config_md(options.trace_parse_config_md, getGameConfig());
  load_config(options.trace_img_loader, &mut config);

  let h = if options.dbg_show_bottom_info { CANVAS_CSS_INITIAL_HEIGHT } else { CANVAS_CSS_INITIAL_HEIGHT - GRID_BOTTOM_DEBUG_HEIGHT - GRID_PADDING } as u32;
  canvas.set_height(h);
  canvas.style().set_property("height", format!("{}px", h).as_str()).expect("should work");

  // https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern
  // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.CanvasRenderingContext2d.html#method.create_pattern_with_html_image_element
  // let ptrn_dock1 = context.create_pattern_with_html_image_element(&img_loading_dock, "repeat").expect("trying to load dock1 tile");

  // Tbh this whole Rc approach is copied from the original template. It works so why not, :shrug:
  let saw_resize_event = Rc::new(Cell::new(true)); // Force reading it after the first frame because it can be whatever.
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
  let ref_counted_canvas = Rc::new(canvas);

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

      last_down_event_type.set(EventSourceType::Mouse);

      let mx = event.offset_x() as f64;
      let my = event.offset_y() as f64;

      log!("mouse down: {}x{}, button: {:?}", mx, my, event.buttons());

      last_mouse_was_down.set(true);
      mouse_x.set(mx);
      mouse_y.set(my);
      last_mouse_down_x.set(mx);
      last_mouse_down_y.set(my);
      last_mouse_down_button.set(event.buttons()); // 1=left, 2=right, 3=left-then-also-right (but right-then-also-left is still 2)
    }) as Box<dyn FnMut(_)>);
    ref_counted_canvas.clone().add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
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
    ref_counted_canvas.clone().add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
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

      let mx = event.offset_x() as f64;
      let my = event.offset_y() as f64;

      log!("mouse up: {}x{}, button: {:?}", mx, my, event.buttons());

      last_mouse_was_up.set(true);
      last_mouse_up_x.set(mx);
      last_mouse_up_y.set(my);
      last_mouse_up_button.set(event.buttons()); // 1=left, 2=right, 3=left-then-also-right (but right-then-also-left is still 2)
    }) as Box<dyn FnMut(_)>);
    ref_counted_canvas.clone().add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  // context menu (just to disable it so we can use rmb for interaction)
  {
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
      event.stop_propagation();
      event.prevent_default();
    }) as Box<dyn FnMut(_)>);
    ref_counted_canvas.clone().add_event_listener_with_callback("contextmenu", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  // touchdown
  {
    let last_down_event_type = last_down_event_type.clone();
    let canvas = ref_counted_canvas.clone();
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
    ref_counted_canvas.clone().add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  // touchmove
  {
    let canvas = ref_counted_canvas.clone();
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
    ref_counted_canvas.clone().add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  // touchend
  {
    let canvas = ref_counted_canvas.clone();
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
    ref_counted_canvas.clone().add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  // resize (fullscreen api will change size of canvas)
  {
    let canvas = ref_counted_canvas.clone();
    let saw_resize_event = saw_resize_event.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
      event.stop_propagation();
      event.prevent_default();

      // log!("(size) Saw canvas resize event");
      saw_resize_event.set(true);
    }) as Box<dyn FnMut(_)>);
    window().add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }

  // Note: Requires unstable API. Currently doing this same thing in the html to work around that.
  // // clipboard paste global event
  // // RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack build --target web --dev
  // // Note: the ::once_into_js() would also work and without the .forget() part but then only once (surprise)
  // let clipboard_callback = Closure::wrap(Box::new(move |event: web_sys::ClipboardEvent| {
  //   let clipboard_content = event.clipboard_data().unwrap().get_data("text/plain").unwrap();
  //   log!("clipboard: {}", clipboard_content);
  // }) as Box<dyn FnMut(_)>);
  // window().add_event_listener_with_callback("paste", &clipboard_callback.as_ref().unchecked_ref()).unwrap();
  // clipboard_callback.forget(); // Note: must do it this way otherwise the event just errors immediately

  // TODO: we could load the default map first (or put it under the stack) but I don't think we want that..?
  let initial_map_from_source;
  let initial_map = {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let last_map = local_storage.get_item(LS_LAST_MAP).unwrap();
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
  options.initial_map_from_source = initial_map_from_source;
  let ( mut state, mut factory ) = init(&mut options, &mut config, initial_map);
  state.showing_debug_bottom = options.dbg_show_bottom_info;
  let mut quick_saves: [Option<QuickSave>; 9] = [(); 9].map(|_| None);

  // Update UI to reflect actually loaded map
  log!("setGameOptions() (After loading map from localStorage)");
  setGameOptions(options_serialize(&options).into(), true.into());

  state_add_examples(getExamples(), &mut state);

  let ( saved_map1, saved_png1, saved_map2, saved_png2, saved_map3, saved_png3, saved_map4, saved_png4 ) = {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    (
      local_storage.get_item(LS_SAVE_SNAP0).unwrap(),
      local_storage.get_item(LS_SAVE_PNG0).unwrap(),
      local_storage.get_item(LS_SAVE_SNAP1).unwrap(),
      local_storage.get_item(LS_SAVE_PNG1).unwrap(),
      local_storage.get_item(LS_SAVE_SNAP2).unwrap(),
      local_storage.get_item(LS_SAVE_PNG2).unwrap(),
      local_storage.get_item(LS_SAVE_SNAP3).unwrap(),
      local_storage.get_item(LS_SAVE_PNG3).unwrap(),
    )
  };

  if let Some(saved_map) = saved_map1 { if let Some(saved_png) = saved_png1 { quick_saves[0] = Some(quick_save_create(0, &document, saved_map, saved_png)); } }
  if let Some(saved_map) = saved_map2 { if let Some(saved_png) = saved_png2 { quick_saves[1] = Some(quick_save_create(1, &document, saved_map, saved_png)); } }
  if let Some(saved_map) = saved_map3 { if let Some(saved_png) = saved_png3 { quick_saves[2] = Some(quick_save_create(2, &document, saved_map, saved_png)); } }
  if let Some(saved_map) = saved_map4 { if let Some(saved_png) = saved_png4 { quick_saves[3] = Some(quick_save_create(3, &document, saved_map, saved_png)); } }


  if options.dbg_onload_dump_factory {
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

      over_floor_zone: false,
      over_floor_not_corner: false,
      down_floor_area: false,
      down_floor_not_corner: false,

      over_quest: false,
      over_quest_visible_index: 0, // Only if over_quest
      down_quest: false,
      down_quest_visible_index: 0, // Only if down_quest
      up_quest: false,
      up_quest_visible_index: 0, // Only if up_quest

      over_menu_button: MenuButton::None,
      down_menu_button: MenuButton::None,
      up_menu_button: MenuButton::None,

      help_hover: false,
      help_down: false,

      // woop: previously named "offer". But I don't have a proper short word for this. So it's woop now.
      woop_down: false,
      woop_down_woop_index: 0,
      woop_hover: false,
      woop_hover_woop_index: 0,
      woop_selected: false,
      woop_selected_index: 0, // woop index, not part index

      atom_down: false,
      atom_down_atom_index: 0,
      atom_hover: false,
      atom_hover_atom_index: 0,
      atom_selected: false,
      atom_selected_index: 0, // Atom index, not part index
      dragging_atom: false,

      is_dragging_machine: false,
      dragging_machine_w: 0,
      dragging_machine_h: 0,
      dragging_machine_part: part_none(&config).kind,

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
      down_save_map: false,
      down_save_map_index: 0,
      up_save_map: false,
      up_save_map_index: 0,
    };
    let mut last_time: f64 = 0.0;
    let mut todo_create_buttons: bool = true;

    let button_canvii: Vec<web_sys::HtmlCanvasElement> = vec!(
      // Buttons on the left (undo, trash, redo)
      prerender_button(&options, &state, &config, UI_UNREDO_WIDTH, UI_UNREDO_HEIGHT, true),
      prerender_button(&options, &state, &config, UI_UNREDO_WIDTH, UI_UNREDO_HEIGHT, false),

      // TODO: merge this with above
      // paint toggle button (toggle paint/delete mode)
      prerender_button(&options, &state, &config, UI_UNREDO_WIDTH, UI_UNREDO_HEIGHT, true),
      prerender_button(&options, &state, &config, UI_UNREDO_WIDTH, UI_UNREDO_HEIGHT, false),

      // paint big quick save button
      prerender_button(&options, &state, &config, UI_SAVE_THUMB_WIDTH, UI_SAVE_THUMB_HEIGHT, true),
      prerender_button(&options, &state, &config, UI_SAVE_THUMB_WIDTH, UI_SAVE_THUMB_HEIGHT, false),

      // Thin save-delete buttons
      prerender_button(&options, &state, &config, UI_SAVE_THUMB_WIDTH - UI_SAVE_THUMB_IMG_WIDTH, UI_SAVE_THUMB_HEIGHT, true),
      prerender_button(&options, &state, &config, UI_SAVE_THUMB_WIDTH - UI_SAVE_THUMB_IMG_WIDTH, UI_SAVE_THUMB_HEIGHT, false),
    );

    let speed_menu_prerender_canvas = prerender_speed_menu(&options, &state, &config, &factory, &mouse_state);
    let mut save_menu_prerender_canvas: Option<web_sys::HtmlCanvasElement> = None;

    if options.trace_size_changes { log!("(size) Internal css size initially: {}x{}, canvas pixels: {}x{}", CANVAS_CSS_INITIAL_WIDTH, CANVAS_CSS_INITIAL_HEIGHT, CANVAS_PIXEL_INITIAL_WIDTH, CANVAS_PIXEL_INITIAL_HEIGHT); }

    // From https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
      // This is the raF frame callback

      if last_time == 0.0 || last_time == time {
        // Either it's the same frame (can happen) or the first frame. Bail.
        last_time = time;
        request_animation_frame(f.borrow().as_ref().unwrap());
        return;
      }

      let real_world_ms_at_start_of_curr_frame: f64 = time;
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
        }
        else if estimated_fps >= (1.0 - variation) * 60.0 && estimated_fps <= (1.0 + variation) * 60.0 {
          ( (ticks_per_second_wanted / 60.0).round() as u64, 60u64 )
        }
        else if estimated_fps >= (1.0 - variation) * 100.0 && estimated_fps <= (1.0 + variation) * 100.0 {
          ( (ticks_per_second_wanted / 100.0).round() as u64, 100u64 )
        }
        else if estimated_fps >= (1.0 - variation) * 120.0 && estimated_fps <= (1.0 + variation) * 120.0 {
          ( (ticks_per_second_wanted / 120.0).round() as u64, 120u64 )
        }
        else {
          ( ticks_todo, 0u64 )
        };

      let mut pregame = false;

      if (config.sprite_cache_loading || options.splash_keep_loader) && !options.splash_no_loader {
        let mut loading = 0;
        for img in config.sprite_cache_canvas.iter() {
          if !img.complete() {
            loading += 1;
          }
        }

        // Find the sprite that is the first frame of the loader. If it is loaded, paint the loading screen. Otherwise paint an ugly loading bar.
        if config.sprite_cache_canvas[config.nodes[CONFIG_NODE_ASSET_SCREEN_LOADER].sprite_config.frames[0].file_canvas_cache_index].complete() {
          paint_asset(&options, &state, &config, &context, CONFIG_NODE_ASSET_SCREEN_LOADER, factory.ticks,
            100.0, 100.0,
            800.0, 600.0
          );
        }

        context.set_font(&"24px monospace");
        context.set_fill_style(&"red".into());
        context.fill_text(format!("Images {}: {} of {}", if loading == 0 { "loaded" } else { "loading" }, config.sprite_cache_canvas.len() - loading, config.sprite_cache_canvas.len()).as_str(), UI_FLOOR_OFFSET_X + (UI_FLOOR_WIDTH / 2.0) - 150.0, UI_FLOOR_OFFSET_Y + (UI_FLOOR_HEIGHT / 2.0) + 35.0 + 200.0).expect("it to work");

        context.set_font(&"12px monospace");
        paint_debug_app(&options, &state, &config, &context, &fps, real_world_ms_at_start_of_curr_frame, real_world_ms_since_start_of_prev_frame, ticks_todo, estimated_fps, rounded_fps, &factory, &mouse_state);
        paint_debug_auto_build(&options, &state, &context, &factory, &mouse_state);

        if loading == 0 {
          log!("Loaded all {} images!", config.sprite_cache_canvas.len());
          config.sprite_cache_loading = false;

          let was_up = last_mouse_was_up.get();
          last_mouse_was_down.set(false);
          last_mouse_was_up.set(false);
          if was_up {
            options.splash_keep_loader = false;
          }
        }

        if loading > 0 || options.splash_keep_loader {
          pregame = true; // Handle config updates etc but shedule a new raF and return before the game loop
        }
      }
      if !pregame && (state.pregame || options.splash_keep_main) && !options.splash_no_main {
        paint_asset(&options, &state, &config, &context, CONFIG_NODE_ASSET_SCREEN_MAIN, factory.ticks,
          100.0, 100.0,
          800.0, 600.0
        );

        let was_up = last_mouse_was_up.get();
        last_mouse_was_down.set(false);
        last_mouse_was_up.set(false);

        if was_up {
          log!("clicked on main splash. closing it.");
          state.pregame = false; // Click anywhere to continue
          options.splash_keep_main = false;
        } else {
          pregame = true; // Handle config updates etc but shedule a new raF and return before the game loop
        }
      }

      if state.load_example_next_frame {
        state.load_example_next_frame = false;
        let map = state.examples[state.example_pointer % state.examples.len()].clone();
        log!("Loading example[{}]; size: {} bytes", state.example_pointer, map.len());
        factory_load_map(&mut options, &mut state, &mut config, &mut factory, map);
        state.example_pointer += 1;
      }
      if state.reset_next_frame {
        state.reset_next_frame = false;
        let map = getGameMap();
        if map.trim().len() > 0 {
          log!("Loading getGameMap(); size: {} bytes", map.len());
          factory_load_map(&mut options, &mut state, &mut config, &mut factory, map);
        } else {
          log!("Skipped attempt at loading an empty map");
        }
      }
      if state.load_paste_next_frame {
        state.load_paste_next_frame = false;
        state.load_paste_hint_since = factory.ticks;
        let paste = state.paste_to_load.clone(); // If we don't do this we'd "move" the state. Which is why we can't do this with load_paste_next_frame as an Option either.

        if paste == "" || paste == "test" {
          // This most likely means the paste failed hard.
          // Show a hint because ctrl+v / apple+v should work
          state.load_paste_hint_kind = LoadPasteHint::Empty;
          state.hint_msg_text = "ctrl+v / cmd+v".to_string();
          state.hint_msg_since = factory.ticks;
        } else {
          state.paste_to_load = "".to_string();
          // Expect a Factini header here
          if !paste.trim().starts_with("# Factini map\n") {
            log!("Error: Input is not a Factini map. Should start with a line `# Factini map` and it didn't...");
            state.load_paste_hint_kind = LoadPasteHint::Invalid;
          } else {
            log!("Loading map from paste; size: {} bytes", paste.len());
            factory_load_map(&mut options, &mut state, &mut config, &mut factory, paste);
            state.load_paste_hint_kind = LoadPasteHint::Success;
          }
        }
      }
      if state.load_snapshot_next_frame {
        // Note: state.load_snapshot_next_frame remains true because factory.changed has special undo-stack behavior for it
        let map = state.snapshot_stack[state.snapshot_undo_pointer % UNDO_STACK_SIZE].clone();
        log!("Loading snapshot[{} / {}]; size: {} bytes", state.snapshot_undo_pointer, state.snapshot_pointer, map.len());
        factory_load_map(&mut options, &mut state, &mut config, &mut factory, map);
      }

      let queued_action = getAction();
      if queued_action != "" { log!("getAction() had `{}`", queued_action); }
      let options_started_from_source = options.options_started_from_source; // Don't change this value here.
      match queued_action.as_str() {
        "apply_options" => {
          parse_and_save_options_string(getGameOptions(), &mut options, false, options_started_from_source, false);

          if options.dbg_show_bottom_info != state.showing_debug_bottom {
            state.showing_debug_bottom = options.dbg_show_bottom_info;
            let h = if state.showing_debug_bottom { CANVAS_CSS_INITIAL_HEIGHT } else { CANVAS_CSS_INITIAL_HEIGHT - GRID_BOTTOM_DEBUG_HEIGHT - GRID_PADDING } as u32;

            ref_counted_canvas.set_height(h);
            ref_counted_canvas.style().set_property("height", format!("{}px", h).as_str()).expect("should work");

            saw_resize_event.set(true);
            if options.trace_size_changes { log!("(size) Debug bar forced css dimensions to change"); }
          }
        },
        "load_map" => state.reset_next_frame = true, // implicitly will call getGameMap() which loads the map from UI indirectly
        "load_config" => {
          let mut config = parse_config_md(options.trace_parse_config_md, getGameConfig());
          load_config(options.trace_img_loader, &mut config);
        }, // Might crash the game
        "paste" => { // When you ctrl+v (or otherwise paste) in the window. This action will be triggered from the html.
          state.paste_to_load = getLastPaste();
          state.load_paste_next_frame = true;
        }
        "copy" => { // When you ctrl+c (or otherwise copy) in the window. This action will be triggered from the html.
          on_copy_factory(&options, &mut state, &config, &factory);
        }
        "" => {},
        _ => panic!("getAction() returned an unsupported value: `{}`", queued_action),
      }

      if !config.sprite_cache_loading && todo_create_buttons {
        log!("Filling in button styles now...");
        todo_create_buttons = false;
        prerender_button_stage2(&options, &state, &config, UI_UNREDO_WIDTH, UI_UNREDO_HEIGHT, &(button_canvii[BUTTON_PRERENDER_INDEX_SMALL_SQUARE_UP].get_context("2d").expect("get context must work").unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap()), true);
        prerender_button_stage2(&options, &state, &config, UI_UNREDO_WIDTH, UI_UNREDO_HEIGHT, &(button_canvii[BUTTON_PRERENDER_INDEX_SMALL_SQUARE_DOWN].get_context("2d").expect("get context must work").unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap()), false);
        prerender_button_stage2(&options, &state, &config, UI_UNREDO_WIDTH, UI_UNREDO_HEIGHT, &(button_canvii[BUTTON_PRERENDER_INDEX_MEDIUM_SQUARE_UP].get_context("2d").expect("get context must work").unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap()), true);
        prerender_button_stage2(&options, &state, &config, UI_UNREDO_WIDTH, UI_UNREDO_HEIGHT, &(button_canvii[BUTTON_PRERENDER_INDEX_MEDIUM_SQUARE_DOWN].get_context("2d").expect("get context must work").unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap()), false);
        prerender_button_stage2(&options, &state, &config, UI_SAVE_THUMB_WIDTH, UI_SAVE_THUMB_HEIGHT, &(button_canvii[BUTTON_PRERENDER_INDEX_SAVE_BIG_UP].get_context("2d").expect("get context must work").unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap()), true);
        prerender_button_stage2(&options, &state, &config, UI_SAVE_THUMB_WIDTH, UI_SAVE_THUMB_HEIGHT, &(button_canvii[BUTTON_PRERENDER_INDEX_SAVE_BIG_DOWN].get_context("2d").expect("get context must work").unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap()), false);
        prerender_button_stage2(&options, &state, &config, UI_SAVE_THUMB_WIDTH - UI_SAVE_THUMB_IMG_WIDTH, UI_SAVE_THUMB_HEIGHT, &(button_canvii[BUTTON_PRERENDER_INDEX_SAVE_THIN_UP].get_context("2d").expect("get context must work").unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap()), true);
        prerender_button_stage2(&options, &state, &config, UI_SAVE_THUMB_WIDTH - UI_SAVE_THUMB_IMG_WIDTH, UI_SAVE_THUMB_HEIGHT, &(button_canvii[BUTTON_PRERENDER_INDEX_SAVE_THIN_DOWN].get_context("2d").expect("get context must work").unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap()), false);
      }

      if pregame {
        // Showing the splash loading screen or main screen. Just return now.
        request_animation_frame(f.borrow().as_ref().unwrap());
        return;
      }

      // main game loop here
      if !state.paused  {
        for _ in 0..ticks_todo.min(MAX_TICKS_PER_FRAME) {
          tick_factory(&mut options, &mut state, &config, &mut factory);
        }
      }

      if factory.quest_updated {
        // Tell JS
        factory.quest_updated = false;

        // log!("Calling onQuestUpdate()");
        let pairs: Vec<js_sys::Array> = factory.quests.iter().map(|quest| {
          let node = &config.nodes[quest.config_node_index];
          let arr: Vec<_> = vec!(
            JsValue::from(node.raw_name.clone()),
            JsValue::from(format!("{:?}", quest.status))
          );
          return arr.iter().collect::<js_sys::Array>();
        }).collect();
        onQuestUpdate(pairs.iter().collect::<js_sys::Array>().into());
      }

      // main ui / paint loop here
      if options.dbg_animate_cli_output_in_web {
        paint_world_cli(&context, &mut options, &mut state, &factory);
      } else {
        let was_resize = saw_resize_event.get();
        if was_resize {
          saw_resize_event.set(false);
          on_after_resize_event(&options, &mut state, &config, &ref_counted_canvas);
        }
        let was_down = last_mouse_was_down.get();
        let was_mouse = if was_down { if last_down_event_type.get() == EventSourceType::Mouse { EventSourceType::Mouse } else { EventSourceType::Touch } } else { EventSourceType::Unknown }; // Only read if set. May be an over-optimization but eh.
        let was_up = last_mouse_was_up.get();
        update_mouse_state(&mut options, &mut state, &config, &mut factory, &mut cell_selection, &mut mouse_state, mouse_x.get(), mouse_y.get(), mouse_moved.get(), was_mouse, was_down, last_mouse_down_x.get(), last_mouse_down_y.get(), last_mouse_down_button.get(), was_up, last_mouse_up_x.get(), last_mouse_up_y.get(), last_mouse_up_button.get());
        last_mouse_was_down.set(false);
        last_mouse_was_up.set(false);
        if was_up && was_mouse != EventSourceType::Mouse {
          // Clear the hover coordinate because otherwise it will lead to confusing UI for touch devices
          mouse_x.set(0.0);
          mouse_y.set(0.0);
        }

        // Handle drag-end or click
        handle_input(&mut cell_selection, &mut mouse_state, &mut options, &mut state, &config, &mut factory, &mut quick_saves);

        if factory.auto_build.phase == AutoBuildPhase::Finishing {
          factory.auto_build.mouse_target_x = mouse_state.world_x;
          factory.auto_build.mouse_target_y = mouse_state.world_y;
        }

        if state.request_fullscreen {
          state.request_fullscreen = false;
          // Must use the boxed canvas ref
          let fse = document.fullscreen_element();
          if let Some(_) = fse {
            log!("Leaving full screen mode...");
            document.exit_fullscreen(); // This will panic hard if browser does not support it... Like on old safari on (unupdated?) ipads.
          } else {
            log!("Entering full screen mode...");
            match ref_counted_canvas.request_fullscreen() {
              Ok(_) => {}
              Err(msg) => {
                log!("Fullscreen raw error: {:?}", msg);
                state.hint_msg_since = factory.ticks;

                // Special case for my ipad: let JS call the webkit prefixed version of it. Rust won't try/know.
                let attempt = tryFullScreenFromJS();

                state.hint_msg_text = format!("Fullscreen failed... JS: {} Raw error: {:?}", attempt, msg);
              }
            }
          }
        }

        if factory.changed {
          // If currently looking at a historic snapshot, then now copy that
          // snapshot to the front of the stack before adding a new state to it
          if state.load_snapshot_next_frame && state.snapshot_pointer != state.snapshot_undo_pointer {
            let snap = state.snapshot_stack[state.snapshot_undo_pointer % UNDO_STACK_SIZE].clone();
            log!("Pushing current undo/redo snapshot to the front of the stack; size: {} bytes, undo pointer: {}, pointer: {}", snap.len(), state.snapshot_undo_pointer, state.snapshot_pointer + 1);
            state.snapshot_stack[(state.snapshot_pointer + 1) % UNDO_STACK_SIZE] = snap;
          }

          if options.trace_porting_step { log!("Auto porting after modification"); }
          keep_auto_porting(&mut options, &mut state, &mut factory);
          fix_ins_and_outs_for_all_belts_and_machines(&options, &mut factory);
          factory.machines = factory_collect_machines(&factory.floor);

          // Recreate cell traversal order
          let prio: Vec<usize> = create_prio_list(&mut options, &config, &mut factory.floor);
          log!("Updated prio list: {:?}", prio);
          factory.prio = prio;

          if state.load_snapshot_next_frame {
            // Do not change undo stack
          } else if factory.auto_build.phase != AutoBuildPhase::None {
            // Do not change for bot changes
          } else {
            if state.snapshot_undo_pointer != state.snapshot_pointer {
              log!("snapshot pointer was in the past({} < {}). its snapshot should be one ahead. move past it to {}", state.snapshot_undo_pointer, state.snapshot_pointer, state.snapshot_pointer + 1);
              state.snapshot_pointer += 1;
              state.snapshot_undo_pointer = state.snapshot_pointer;
            }

            // Create snapshot in history, except for unredo
            let snap = generate_floor_dump(&options, &state, &config, &factory, dnow()).join("\n");
            if options.dbg_onchange_dump_snapshot { log!("Snapshot:\n{}", snap); }
            log!("Pushed snapshot to the front of the stack; size: {} bytes, undo pointer: {}, pointer: {}", snap.len(), state.snapshot_undo_pointer, state.snapshot_pointer);

            state.snapshot_pointer += 1;
            state.snapshot_undo_pointer = state.snapshot_pointer;
            state.snapshot_stack[state.snapshot_pointer % UNDO_STACK_SIZE] = snap;
          }

          factory.changed = false;
          factory.accepted = 0;
          factory.produced = 0;
          factory.trashed = 0;
          factory.supplied = 0;
          if state.load_snapshot_next_frame {
            log!("now unsetting state.load_snapshot_next_frame");
            state.load_snapshot_next_frame = false;
          }

          // Dump current map to debug UI
          let game_map = document.get_element_by_id("$game_map").unwrap();
          game_map.dyn_into::<web_sys::HtmlTextAreaElement>().unwrap().set_value(state.snapshot_stack[state.snapshot_undo_pointer % UNDO_STACK_SIZE].as_str());

          let last_map = state.snapshot_stack[state.snapshot_undo_pointer % UNDO_STACK_SIZE].as_str();
          let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
          local_storage.set_item(LS_LAST_MAP, last_map).unwrap();
          log!("Stored last map to local storage ({} bytes) into `{}`", last_map.len(), LS_LAST_MAP);
          // log!("Map:\n{}", last_map);
          if options.trace_map_parsing { log!("Stored map:\n{}", last_map); }
        }

        // Paint the world (we should not do input or world mutations after this point)

        context.set_font(&"12px monospace");

        // Clear canvas
        // Global background
        context.clear_rect(0.0, 0.0, state.canvas_pixel_width, state.canvas_pixel_height);

        // This wil show the actual painted area of the canvas (useful for fullscreen mode)
        // context.set_fill_style(&"red".into());
        // context.fill_rect(0.0, 0.0, state.canvas_pixel_width, state.canvas_pixel_height);

        // Put a semi-transparent layer over the inner floor part to make it darker
        // context.set_fill_style(&"#00000077".into());
        context.set_fill_style(&FLOOR_YELLOW_COLOR.into());
        context.fill_rect(UI_FLOOR_OFFSET_X + CELL_W, UI_FLOOR_OFFSET_Y + CELL_H, (FLOOR_CELLS_W - 2) as f64 * CELL_W, (FLOOR_CELLS_H - 2) as f64 * CELL_H);

        paint_zone_hovers(&options, &state, &context, &mouse_state);
        // paint_top_stats(&context, &mut factory);
        paint_help_and_ai_button(&options, &state, &config, &factory, &mouse_state, &context, &button_canvii);
        paint_quests(&options, &state, &config, &context, &factory, &mouse_state);
        paint_atoms(&options, &state, &config, &context, &factory, &mouse_state, &cell_selection);
        let ( highlight_index, highlight_x, highlight_y ) = paint_woops(&options, &state, &config, &context, &factory, &mouse_state, &cell_selection);
        paint_lasers(&options, &mut state, &config, &context);
        paint_secret_menu_or_logo(&options, &state, &config, &factory, &context, &button_canvii, &mouse_state);
        paint_floor_round_way(&options, &state, &config, &factory, &context);
        paint_background_tiles1(&options, &state, &config, &factory, &context);
        paint_background_tiles2(&options, &state, &config, &factory, &context);
        paint_background_tiles3(&options, &state, &config, &factory, &context);
        paint_port_arrows(&options, &state, &config, &context, &factory);
        paint_belt_dbg_id(&options, &state, &config, &context, &factory);
        paint_machine_craft_menu(&options, &state, &config, &context, &factory, &cell_selection, &mouse_state);
        paint_ui_atom_woop_hover_droptarget_hint_conditionally(&options, &state, &config, &context, &mut factory, &mouse_state, &cell_selection);
        paint_debug_app(&options, &state, &config, &context, &fps, real_world_ms_at_start_of_curr_frame, real_world_ms_since_start_of_prev_frame, ticks_todo, estimated_fps, rounded_fps, &factory, &mouse_state);
        paint_debug_auto_build(&options, &state, &context, &factory, &mouse_state);
        paint_debug_selected_belt_cell(&context, &factory, &cell_selection, &mouse_state);
        paint_debug_selected_machine_cell(&context, &factory, &cell_selection, &mouse_state);
        paint_debug_selected_supply_cell(&context, &factory, &cell_selection, &mouse_state);
        paint_debug_selected_demand_cell(&context, &factory, &cell_selection, &mouse_state);
        paint_map_state_buttons(&options, &state, &config, &context, &button_canvii, &mouse_state);
        paint_fullscreen_button(&options, &state, &config, &context, &button_canvii, &mouse_state);

        paint_maze(&options, &state, &config, &factory, &context, &mouse_state);
        // The lazy truck will jump over the foxy maze stuff
        paint_trucks(&options, &state, &config, &context, &mut factory);

        // Probably after all backround/floor stuff is finished
        paint_zone_borders(&options, &state, &context);

        paint_ui_speed_menu(&options, &state, &config, &factory, &context, &mouse_state);
        paint_speed_menu_animation(&options, &mut state, &config, &factory, &context, &speed_menu_prerender_canvas);

        paint_load_thumbs(&options, &state, &config, &factory, &context, &button_canvii, &mouse_state, &mut quick_saves);
        paint_text_hint(&options, &state, &config, &factory, &context);
        if state.ui_save_menu_anim_progress > 0 && save_menu_prerender_canvas == None {
          // Lazy/deferred load. Ugly to do it in here but the rendering function would not have access to update the reference
          // (And it needs to be deferred anyways otherwise images may not be loaded)
          save_menu_prerender_canvas = Some(prerender_save_menu(&options, &state, &config, &factory, &mouse_state, &mut quick_saves, &button_canvii));
        }
        paint_save_menu_animation(&options, &mut state, &config, &factory, &context, &save_menu_prerender_canvas);

        paint_border_hint(&options, &state, &config, &factory, &context);
        // In front of all game stuff
        paint_bouncers(&options, &state, &config, &context, &mut factory);
        // Paint the factory icon now so bouncers go behind it
        paint_factory_img(&options, &state, &config, &factory, &context, &mouse_state);

        if highlight_index > 0 {
          // Hovering over / selected a woop
          paint_woop_tooltip(&options, &state, &config, &factory, &context, highlight_index - 1, highlight_x, highlight_y);
        }
        else if cell_selection.on && factory.floor[cell_selection.coord].kind == CellKind::Machine {
          // Selected a machine
          let main_coord = factory.floor[cell_selection.coord].machine.main_coord;
          paint_woop_tooltip_with_part(&options, &state, &config, &factory, &context, 0.0, 0.0, factory.floor[main_coord].machine.output_want.kind);
        }
        else if !cell_selection.on && mouse_state.over_zone == ZONE_FLOOR && !mouse_state.is_down && !mouse_state.is_up && !mouse_state.is_dragging && is_floor(mouse_state.cell_x_floored, mouse_state.cell_y_floored) {
          let hover_coord = to_coord(mouse_state.cell_x_floored as usize, mouse_state.cell_y_floored as usize);
          let main_coord = factory.floor[hover_coord].machine.main_coord;
          if factory.floor[main_coord].kind == CellKind::Machine {
            // Hovering over a machine
            paint_woop_tooltip_with_part(&options, &state, &config, &factory, &context, 0.0, 0.0, factory.floor[main_coord].machine.output_want.kind);
          }
        }

        // Over all the UI stuff
        paint_mouse_cursor(&options, &state, &config, &factory, &context, &mouse_state);
        // When dragging make sure that stays on top of bouncers
        paint_mouse_action(&options, &state, &config, &factory, &context, &mouse_state, &cell_selection);

        // In front of everything else
        paint_manual(&options, &state, &config, &factory, &context);
      }

      // Schedule next frame
      request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f64)>));

    request_animation_frame(g.borrow().as_ref().unwrap());
  }

  Ok(())
}

fn load_config(trace_img_loader: bool, config: &mut Config) {
  log!("load_config(options.trace_img_loader={})", trace_img_loader);

  if trace_img_loader {
    log!("  - Nodes that want to load a file:");
    config.nodes.iter().for_each(|node| {
      if node.sprite_config.frames[0].file != "" {
        log!("    - node `{}` wants to load `{}` at canvas index {}", node.raw_name, node.sprite_config.frames[0].file, node.sprite_config.frames[0].file_canvas_cache_index);
      }
    });
  }

  let image_loader_prio = vec!(
    // Load the loader splash screen first
    config.nodes[CONFIG_NODE_ASSET_SCREEN_LOADER].sprite_config.frames[0].file_canvas_cache_index,
  );

  // Load sprite maps. Once per image. Start with prio, then the rest, then unbox them.
  let mut boxed: Vec<Option<web_sys::HtmlImageElement>> =
    config.sprite_cache_order.iter().enumerate().map(|(index, src)| {
      if image_loader_prio.contains(&index) {
        if trace_img_loader { log!("Loading {} with prio", src); }
        return Some(load_tile(src.clone().as_str()));
      }
      return None;
    }).collect::<Vec<Option<web_sys::HtmlImageElement>>>();
  // Now load the no-prio's
  config.sprite_cache_order.iter().enumerate().for_each(|(index, src)| {
    if boxed[index] == None {
      boxed[index] = Some(load_tile(src.clone().as_str()));
    }
  });
  // Now unbox them so we don't have to do the unbox dance every time
  config.sprite_cache_canvas = boxed.into_iter().filter_map(|e| e).collect();

  config.sprite_cache_loading = true;

  if trace_img_loader { log!("Queued up {} sprite files for these parts: {:?}", config.sprite_cache_canvas.len(), config.sprite_cache_lookup); }
  else { log!("Queued up {} sprite files to load...", config.sprite_cache_canvas.len()); }

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
        ConfigNodeKind::Story => "Story",
      });
    }).collect::<js_sys::Array>().into();

    let nodes: JsValue = config_to_jsvalue(&config);

    // Send to html
    receiveConfigNode("wat".into(), vec!(
      vec!(JsValue::from("kinds"), kinds).iter().collect::<js_sys::Array>(),
      vec!(JsValue::from("nodes"), nodes).iter().collect::<js_sys::Array>(),
    ).iter().collect::<js_sys::Array>().into());
  }
}

fn load_tile(src: &str) -> web_sys::HtmlImageElement {
  let document = document();

  let img = document
    .create_element("img")
    .expect("to work")
    .dyn_into::<web_sys::HtmlImageElement>()
    .expect("to work");

  img.set_src(src);

  return img;
}

fn get_x_while_dragging_machine(cell_x: f64, machine_cell_width: usize) -> f64 {
  // Note: width is cell count of machine, not pixel size
  let compx = if machine_cell_width % 2 == 1 { 0.0 } else { 0.5 };
  let ox = (cell_x + compx).floor() - (machine_cell_width / 2) as f64;
  return ox;
}
fn get_y_while_dragging_machine(cell_y: f64, machine_cell_height: usize) -> f64 {
  // Note: height is cell count of machine, not pixel size
  let compy = if machine_cell_height % 2 == 1 { 0.0 } else { 0.5 };
  let oy = (cell_y + compy).floor() - (machine_cell_height / 2) as f64;
  return oy;
}

fn update_mouse_state(
  options: &mut Options, state: &mut State, config: &Config, factory: &Factory,
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
    mouse_state.atom_down = false;
    mouse_state.woop_down = false;
    mouse_state.help_down = false;
    mouse_state.is_down = false;
    mouse_state.down_floor_not_corner = false;
    mouse_state.down_menu_button = MenuButton::None;
    mouse_state.up_menu_button = MenuButton::None;
    mouse_state.dragging_atom = false;
    mouse_state.is_dragging_machine = false;
    mouse_state.down_quest = false;
    mouse_state.up_quest = false;
    mouse_state.down_save_map = false;
    mouse_state.up_save_map = false;
  }
  mouse_state.was_down = false;
  mouse_state.is_up = false;
  mouse_state.was_up = false;
  mouse_state.was_dragging = false;
  mouse_state.atom_hover = false;
  mouse_state.woop_hover = false;
  mouse_state.over_quest = false;
  mouse_state.over_menu_button = MenuButton::None;
  mouse_state.help_hover = false;
  mouse_state.over_save_map = false;

  mouse_state.up_zone = Zone::None;
  mouse_state.over_zone = Zone::None;

  mouse_state.over_floor_not_corner = false;

  // Mouse coords
  // Note: mouse2world coord is determined by _css_ size, not _canvas_ size
  mouse_state.canvas_x = mouse_x; // Where your mouse actually is on your screen / in your browser
  mouse_state.canvas_y = mouse_y;
  mouse_state.world_x = (mouse_x - state.canvas_css_x) / state.canvas_css_width * state.canvas_pixel_width;
  mouse_state.world_y = (mouse_y - state.canvas_css_y) / state.canvas_css_height * state.canvas_pixel_height;
  mouse_state.cell_x = (mouse_state.world_x - UI_FLOOR_OFFSET_X) / CELL_W;
  mouse_state.cell_y = (mouse_state.world_y - UI_FLOOR_OFFSET_Y) / CELL_H;
  mouse_state.cell_x_floored = mouse_state.cell_x.floor();
  mouse_state.cell_y_floored = mouse_state.cell_y.floor();

  let is_machine_selected = cell_selection.on && factory.floor[cell_selection.coord].kind == CellKind::Machine;

  mouse_state.over_zone = coord_to_zone(options, state, config, mouse_state.world_x, mouse_state.world_y, is_machine_selected, factory, cell_selection.coord);
  match mouse_state.over_zone {
    Zone::None => panic!("cant be over on no zone"),
    ZONE_MANUAL => {} // popup
    ZONE_QUESTS => {
      mouse_state.over_quest =
        mouse_state.world_x >= UI_QUESTS_OFFSET_X + UI_QUEST_X && mouse_state.world_x < UI_QUESTS_OFFSET_X + UI_QUEST_X + UI_QUEST_WIDTH &&
        (mouse_state.world_y - (UI_QUESTS_OFFSET_Y + UI_QUEST_Y)) % (UI_QUEST_HEIGHT + UI_QUEST_MARGIN) < UI_QUEST_HEIGHT;
      mouse_state.over_quest_visible_index = if mouse_state.over_quest { ((mouse_state.world_y - (UI_QUESTS_OFFSET_Y + UI_QUEST_Y)) / (UI_QUEST_HEIGHT + UI_QUEST_MARGIN)) as usize } else { 0 };
    }
    ZONE_SAVE_MAP => {
      if options.enable_quick_save_menu {
        let button_index = hit_test_save_map_button_index(mouse_state.world_x, mouse_state.world_y);
        if button_index != 100 {
          // Up on a button
          // TOFIX: change to use MenuButton approach
          mouse_state.over_save_map = true;
          mouse_state.over_save_map_index = button_index;
        }
      }
    }
    Zone::BottomBottomLeft => {}
    Zone::TopLeft => {
      if hit_test_help_button(mouse_state.world_x, mouse_state.world_y) {
        mouse_state.help_hover = true;
      }
      else if hit_test_fullscreen_button(mouse_state.world_x, mouse_state.world_y) {
        mouse_state.over_menu_button = MenuButton::FullScreenButton;
      }
      else if bounds_check(mouse_state.world_x, mouse_state.world_y, UI_AUTO_BUILD_X, UI_AUTO_BUILD_Y, UI_AUTO_BUILD_X + UI_AUTO_BUILD_W, UI_AUTO_BUILD_Y + UI_AUTO_BUILD_H) {
        mouse_state.over_menu_button = MenuButton::AutoBuildButton;
      }
    }
    Zone::Top => {
      if hit_test_undo(mouse_state.world_x, mouse_state.world_y) {
        mouse_state.over_menu_button = MenuButton::UndoButton;
      }
      else if hit_test_clear(mouse_state.world_x, mouse_state.world_y) {
        mouse_state.over_menu_button = MenuButton::ClearButton;
      }
      else if hit_test_redo(mouse_state.world_x, mouse_state.world_y) {
        mouse_state.over_menu_button = MenuButton::RedoButton;
      }
      else if options.enable_speed_menu {
        let menu_button = hit_test_menu_speed_buttons(mouse_state.world_x, mouse_state.world_y);
        if menu_button != MenuButton::None {
          // time controls, first, second row of menu buttons
          mouse_state.over_menu_button = menu_button;
        }
      }
    }
    Zone::TopRight => {
      if hit_test_paint_toggle(mouse_state.world_x, mouse_state.world_y) {
        mouse_state.over_menu_button = MenuButton::PaintToggleButton;
      }
      else if options.dbg_show_secret_menu {
        let menu_button = hit_test_menu_buttons(mouse_state.world_x, mouse_state.world_y);
        if menu_button != MenuButton::None {
          // time controls, first, second row of menu buttons
          mouse_state.over_menu_button = menu_button;
        }
      }
    }
    ZONE_FLOOR => {
      mouse_state.over_floor_zone = true;
      mouse_state.over_floor_not_corner =
        // Over floor cells
        mouse_state.cell_x >= 0.0 && mouse_state.cell_x < (FLOOR_CELLS_W as f64) && mouse_state.cell_y >= 0.0 && mouse_state.cell_y < (FLOOR_CELLS_H as f64) &&
        // Not corner
        !((mouse_state.cell_x_floored == 0.0 || mouse_state.cell_x_floored == (FLOOR_CELLS_W - 1) as f64) && (mouse_state.cell_y_floored == 0.0 || mouse_state.cell_y_floored == (FLOOR_CELLS_H - 1) as f64));
    }
    Zone::Bottom => {
      if hit_test_copy_button(mouse_state.world_x, mouse_state.world_y) {
        mouse_state.over_menu_button = MenuButton::CopyFactory;
      }
      else if hit_test_paste_button(mouse_state.world_x, mouse_state.world_y) {
        mouse_state.over_menu_button = MenuButton::PasteFactory;
      }
      else if !mouse_state.is_dragging {
        // Check if atom is hit
        // When already dragging do not update atom visual state, do not record the "over" state at all
        // When dragging an atom, the atom_down_atom_index will be set to the initial atom index (keep it!)
        let (atom_hover, atom_hover_atom_index) = hit_test_atoms(factory, mouse_state.world_x, mouse_state.world_y);
        if atom_hover {
          // Do not consider atoms that are not visible / interactive to be hoverable either
          if factory.available_atoms[atom_hover_atom_index].1 {
            mouse_state.atom_hover = true;
            mouse_state.atom_hover_atom_index = atom_hover_atom_index;
          }
        }
      }
    }
    Zone::BottomBottom => {}
    Zone::Right => {
      if !mouse_state.is_dragging {
        // When already dragging do not update woop visual state, do not record the "over" state at all
        // When dragging a woop, the woop_down_woop_index will be set to the initial woop index (keep it!)
        let (woop_hover, woop_hover_woop_index) = hit_test_woops(factory, mouse_state.world_x, mouse_state.world_y);
        if woop_hover {
          // Do not consider woops that are not visible / interactive to be hoverable either
          if factory.available_woops[woop_hover_woop_index].1 {
            mouse_state.woop_hover = true;
            mouse_state.woop_hover_woop_index = woop_hover_woop_index;
          }
        }
      }
    }
    Zone::BottomRight => {}
    Zone::BottomBottomRight => {}
    Zone::Margin => {}
  }

  // on mouse down
  if last_mouse_was_down {
    log!("ok something down...");
    mouse_state.last_down_event_type = if state.event_type_swapped { if last_down_event_type == EventSourceType::Touch { EventSourceType::Mouse } else { EventSourceType::Touch } } else { if last_down_event_type == EventSourceType::Mouse { EventSourceType::Mouse } else { EventSourceType::Touch } };
    mouse_state.last_down_button = last_mouse_down_button;
    mouse_state.last_down_canvas_x = last_mouse_down_x;
    mouse_state.last_down_canvas_y = last_mouse_down_y;
    mouse_state.last_down_world_x = (last_mouse_down_x - state.canvas_css_x) / state.canvas_css_width * state.canvas_pixel_width;
    mouse_state.last_down_world_y = (last_mouse_down_y - state.canvas_css_y) / state.canvas_css_height * state.canvas_pixel_height;
    mouse_state.last_down_cell_x = (mouse_state.last_down_world_x - UI_FLOOR_OFFSET_X) / CELL_W;
    mouse_state.last_down_cell_y = (mouse_state.last_down_world_y - UI_FLOOR_OFFSET_Y) / CELL_H;
    mouse_state.last_down_cell_x_floored = mouse_state.last_down_cell_x.floor();
    mouse_state.last_down_cell_y_floored = mouse_state.last_down_cell_y.floor();

    mouse_state.is_down = true; // Unset after on_up
    mouse_state.was_down = true; // Unset after this frame

    mouse_state.down_zone = coord_to_zone(options, state, config, mouse_state.last_down_world_x, mouse_state.last_down_world_y, is_machine_selected, factory, cell_selection.coord);
    log!("DOWN event (type={:?}) in zone {:?}, screen {}x{}, world {}x{}", if mouse_state.last_down_event_type == EventSourceType::Mouse { "Mouse" } else { "Touch" }, mouse_state.down_zone, mouse_state.world_x, mouse_state.world_y, mouse_state.last_down_cell_x, mouse_state.last_down_cell_y);

    match mouse_state.down_zone {
      Zone::None => panic!("cant be down on no zone"),
      ZONE_MANUAL => {} // popup
      ZONE_QUESTS => {
        mouse_state.down_quest =
          mouse_state.last_down_world_x >= UI_QUESTS_OFFSET_X + UI_QUEST_X && mouse_state.last_down_world_x < UI_QUESTS_OFFSET_X + UI_QUEST_X + UI_QUEST_WIDTH &&
          (mouse_state.last_down_world_y - (UI_QUESTS_OFFSET_Y + UI_QUEST_Y)) % (UI_QUEST_HEIGHT + UI_QUEST_MARGIN) < UI_QUEST_HEIGHT;
        mouse_state.down_quest_visible_index = if mouse_state.down_quest { ((mouse_state.last_down_world_y - (UI_QUESTS_OFFSET_Y + UI_QUEST_Y)) / (UI_QUEST_HEIGHT + UI_QUEST_MARGIN)) as usize } else { 0 };
      }
      ZONE_SAVE_MAP => {
        if options.enable_quick_save_menu {
          let button_index = hit_test_save_map_button_index(mouse_state.last_down_world_x, mouse_state.last_down_world_y);
          if button_index != 100 {
            // Up on a button
            mouse_state.down_save_map = true;
            mouse_state.down_save_map_index = button_index;
          }
        } else {
          log!("Ignoring map save down");
        }
      }
      Zone::BottomBottomLeft => {}
      Zone::TopLeft => {
        if mouse_state.help_hover {
          mouse_state.help_down = true;
        }
        else if hit_test_fullscreen_button(mouse_state.world_x, mouse_state.world_y) {
          mouse_state.down_menu_button = MenuButton::FullScreenButton;
        }
        else if bounds_check(mouse_state.last_down_world_x, mouse_state.last_down_world_y, UI_AUTO_BUILD_X, UI_AUTO_BUILD_Y, UI_AUTO_BUILD_X + UI_AUTO_BUILD_W, UI_AUTO_BUILD_Y + UI_AUTO_BUILD_H) {
          mouse_state.down_menu_button = MenuButton::AutoBuildButton;
        }
        else {
          log!("missed buttons on down in top-left")
        }
      }
      Zone::Top => {
        if hit_test_undo(mouse_state.last_down_world_x, mouse_state.last_down_world_y) {
          mouse_state.down_menu_button = MenuButton::UndoButton;
        }
        else if hit_test_clear(mouse_state.last_down_world_x, mouse_state.last_down_world_y) {
          mouse_state.down_menu_button = MenuButton::ClearButton;
        }
        else if hit_test_redo(mouse_state.last_down_world_x, mouse_state.last_down_world_y) {
          mouse_state.down_menu_button = MenuButton::RedoButton;
        }
        else if !options.enable_speed_menu {
          log!("Speed menu disabled, ignoring down");
        } else {
          let menu_button = hit_test_menu_speed_buttons(mouse_state.last_down_world_x, mouse_state.last_down_world_y);
          if menu_button != MenuButton::None {
            // time controls, first, second row of menu buttons
            mouse_state.down_menu_button = menu_button;
          } else {
            log!("Did not hit a top menu button on down");
          }
        }
        log!("Top menu button down: {:?}", mouse_state.down_menu_button);
      }
      Zone::TopRight => {
        if hit_test_paint_toggle(mouse_state.last_down_world_x, mouse_state.last_down_world_y) {
          mouse_state.down_menu_button = MenuButton::PaintToggleButton;
        }
        else if options.dbg_show_secret_menu {
          let menu_button = hit_test_menu_buttons(mouse_state.last_down_world_x, mouse_state.last_down_world_y);
          if menu_button != MenuButton::None {
            // time controls, first, second row of menu buttons
            mouse_state.down_menu_button = menu_button;
          } else {
            log!("Ignored top-right down event");
          }
        }
      }
      ZONE_FLOOR => {
        mouse_state.down_floor_area = true;
        mouse_state.down_floor_not_corner =
          // Over floor cells
          mouse_state.last_down_cell_x >= 0.0 && mouse_state.last_down_cell_x < (FLOOR_CELLS_W as f64) && mouse_state.last_down_cell_y >= 0.0 && mouse_state.last_down_cell_y < (FLOOR_CELLS_H as f64) &&
          // Not corner
          !((mouse_state.last_down_cell_x_floored == 0.0 || mouse_state.last_down_cell_x_floored == (FLOOR_CELLS_W - 1) as f64) && (mouse_state.last_down_cell_y_floored == 0.0 || mouse_state.last_down_cell_y_floored == (FLOOR_CELLS_H - 1) as f64));
      }
      Zone::Bottom => {
        if mouse_state.atom_hover {
          mouse_state.atom_down = true;
          mouse_state.atom_down_atom_index = mouse_state.atom_hover_atom_index;
        }
        else if hit_test_copy_button(mouse_state.last_down_world_x, mouse_state.last_down_world_y) {
          mouse_state.down_menu_button = MenuButton::CopyFactory;
        }
        else if hit_test_paste_button(mouse_state.last_down_world_x, mouse_state.last_down_world_y) {
          mouse_state.down_menu_button = MenuButton::PasteFactory;
        }
        else {
          log!("Ignored down event in bottom");
        }
        log!("Bottom menu button down: {:?}", mouse_state.down_menu_button);
      }
      Zone::BottomBottom => {}
      Zone::Right => {
        if mouse_state.woop_hover {
          mouse_state.woop_down = true;
          mouse_state.woop_down_woop_index = mouse_state.woop_hover_woop_index;
        }
      }
      Zone::BottomRight => {}
      Zone::BottomBottomRight => {}
      Zone::Margin => {}
    }
    log!("DOWN menu button: {:?}", mouse_state.down_menu_button);
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
      ZONE_FLOOR => {
        log!("drag start, floor");
        let is_machine_selected = cell_selection.on && factory.floor[cell_selection.coord].kind == CellKind::Machine;
        if is_machine_selected {
          cell_selection.on = false;
        }
      }
      _ => {
        log!("drag start, non-floor");
      }
    }
  }

  // on mouse up
  if last_mouse_was_up {
    mouse_state.last_up_canvas_x = last_mouse_up_x;
    mouse_state.last_up_canvas_y = last_mouse_up_y;
    mouse_state.last_up_world_x = (last_mouse_up_x - state.canvas_css_x) / state.canvas_css_width * state.canvas_pixel_width;
    mouse_state.last_up_world_y = (last_mouse_up_y - state.canvas_css_y) / state.canvas_css_height * state.canvas_pixel_height;
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
    log!("UP event (type={:?}) in zone {:?}, was down in zone {:?}, screen {}x{}, world {}x{}", if mouse_state.last_down_event_type == EventSourceType::Mouse { "Mouse" } else { "Touch" }, mouse_state.up_zone, mouse_state.down_zone, mouse_state.world_x, mouse_state.world_y, mouse_state.last_up_cell_x, mouse_state.last_up_cell_y);

    match mouse_state.up_zone {
      Zone::None => panic!("cant be up on no zone"),
      ZONE_MANUAL => {} // the popup
      ZONE_QUESTS => {
        mouse_state.up_quest =
          mouse_state.last_up_world_x >= UI_QUESTS_OFFSET_X + UI_QUEST_X && mouse_state.last_up_world_x < UI_QUESTS_OFFSET_X + UI_QUEST_X + UI_QUEST_WIDTH &&
          (mouse_state.last_up_world_y - (UI_QUESTS_OFFSET_Y + UI_QUEST_Y)) % (UI_QUEST_HEIGHT + UI_QUEST_MARGIN) < UI_QUEST_HEIGHT;
        mouse_state.up_quest_visible_index = if mouse_state.up_quest { ((mouse_state.last_up_world_y - (UI_QUESTS_OFFSET_Y + UI_QUEST_Y)) / (UI_QUEST_HEIGHT + UI_QUEST_MARGIN)) as usize } else { 0 };
      }
      ZONE_SAVE_MAP => {
        if options.enable_quick_save_menu {
          let button_index = hit_test_save_map_button_index(mouse_state.last_up_world_x, mouse_state.last_up_world_y);
          if button_index != 100 {
            // Up on a button
            mouse_state.up_save_map = true;
            mouse_state.up_save_map_index = button_index;
          }
        } else {
          log!("Ignoring map save up");
        }
      }
      Zone::BottomBottomLeft => {}
      Zone::TopLeft => {
        if bounds_check(mouse_state.last_up_world_x, mouse_state.last_up_world_y, UI_AUTO_BUILD_X, UI_AUTO_BUILD_Y, UI_AUTO_BUILD_X + UI_AUTO_BUILD_W, UI_AUTO_BUILD_Y + UI_AUTO_BUILD_H) {
          mouse_state.up_menu_button = MenuButton::AutoBuildButton;
        }
        else if hit_test_fullscreen_button(mouse_state.world_x, mouse_state.world_y) {
          mouse_state.up_menu_button = MenuButton::FullScreenButton;
        }
        else {
          log!("missed buttons on down in top-left")
        }
      }
      Zone::Top => {
        if hit_test_undo(mouse_state.last_up_world_x, mouse_state.last_up_world_y) {
          mouse_state.up_menu_button = MenuButton::UndoButton;
        }
        else if hit_test_clear(mouse_state.last_up_world_x, mouse_state.last_up_world_y) {
          mouse_state.up_menu_button = MenuButton::ClearButton;
        }
        else if hit_test_redo(mouse_state.last_up_world_x, mouse_state.last_up_world_y) {
          mouse_state.up_menu_button = MenuButton::RedoButton;
        }
        else if !options.enable_speed_menu {
          log!("Speed menu disabled, ignoring up");
        }
        else {
          let menu_button = hit_test_menu_speed_buttons(mouse_state.last_up_world_x, mouse_state.last_up_world_y);
          if menu_button != MenuButton::None {
            // time controls, first, second row of menu buttons
            mouse_state.up_menu_button = menu_button;
          } else {
            log!("Did not hit a top menu button on up");
          }
        }
        log!("Top menu button up: {:?}", mouse_state.up_menu_button);
      }
      Zone::TopRight => {
        if hit_test_paint_toggle(mouse_state.last_up_world_x, mouse_state.last_up_world_y) {
          mouse_state.up_menu_button = MenuButton::PaintToggleButton;
        }
        else if options.dbg_show_secret_menu {
          let menu_button = hit_test_menu_buttons(mouse_state.last_up_world_x, mouse_state.last_up_world_y);
          if menu_button != MenuButton::None {
            // time controls, first, second row of menu buttons
            mouse_state.up_menu_button = menu_button;
          } else {
            log!("Ignored top-right up event");
          }
        } else {
          if bounds_check(mouse_state.last_up_world_x, mouse_state.last_up_world_y, UI_DEBUG_UNLOCK_X, UI_DEBUG_UNLOCK_Y, UI_DEBUG_UNLOCK_X + UI_DEBUG_UNLOCK_W, UI_DEBUG_UNLOCK_Y + UI_DEBUG_UNLOCK_H) {
            log!("Forward UI progress");
            update_game_ui_after_quest_finish(options, state);
          }
          else if bounds_check(mouse_state.last_up_world_x, mouse_state.last_up_world_y, UI_DEBUG_SECRET_X, UI_DEBUG_SECRET_Y, UI_DEBUG_SECRET_X + UI_DEBUG_SECRET_W, UI_DEBUG_SECRET_Y + UI_DEBUG_SECRET_H) {
            log!("Enabling secret debug menu...");
            options.dbg_show_secret_menu = true;
          }
        }
      }
      ZONE_FLOOR => {}
      Zone::Bottom => {
        if hit_test_copy_button(mouse_state.last_up_world_x, mouse_state.last_up_world_y) {
          mouse_state.up_menu_button = MenuButton::CopyFactory;
        }
        else if hit_test_paste_button(mouse_state.last_up_world_x, mouse_state.last_up_world_y) {
          mouse_state.up_menu_button = MenuButton::PasteFactory;
        }
        else {
          log!("Ignored bottom up event");
        }
        log!("Bottom menu button up: {:?}", mouse_state.up_menu_button);
      }
      Zone::BottomBottom => {}
      Zone::Right => {}
      Zone::BottomRight => {}
      Zone::BottomBottomRight => {}
      Zone::Margin => {}
    }

    log!("UP menu button: {:?}", mouse_state.up_menu_button);
  }
}
fn handle_input(cell_selection: &mut CellSelection, mouse_state: &mut MouseState, options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, quick_saves: &mut [Option<QuickSave>; 9]) {
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
      Zone::Right => {
        if mouse_state.woop_down {
          on_drag_start_woop(options, state, config, factory, mouse_state, cell_selection);
        }
      }
      Zone::Bottom => {
        if mouse_state.atom_down {
          on_drag_start_atom(options, state, config, factory, mouse_state, cell_selection);
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
      ZONE_QUESTS => {
        if mouse_state.down_quest {
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
        ZONE_FLOOR => {
          if mouse_state.dragging_atom {
            on_drag_end_atom_over_floor(options, state, config, factory, mouse_state, cell_selection);
          }
          else if mouse_state.is_dragging_machine {
            on_drag_end_machine_over_floor(options, state, config, factory, mouse_state);
          }
          else {
            log!("Was not dragging a known machine size... {}x{}", mouse_state.dragging_machine_w, mouse_state.dragging_machine_h);
            on_drag_end_floor(options, state, config, factory, cell_selection, mouse_state);
          }
        }
        _ => {}
      }
    } else {
      match mouse_state.up_zone {
        Zone::TopLeft => {
          if mouse_state.help_down {
            on_click_help(options, state, config);
          } else {
            on_up_menu(cell_selection, mouse_state, options, state, config, factory);
          }
        }
        Zone::Top => {
          on_up_menu(cell_selection, mouse_state, options, state, config, factory);
        }
        Zone::TopRight => {
          // For options etc
          on_up_menu(cell_selection, mouse_state, options, state, config, factory);
        }
        Zone::Bottom => {
          if mouse_state.atom_down {
            on_up_atom(options, state, config, factory, mouse_state);
          }
          else if mouse_state.up_menu_button == MenuButton::CopyFactory {
            on_copy_factory(options, state, config, factory);
          }
          else if mouse_state.up_menu_button == MenuButton::PasteFactory {
            on_paste_factory(options, state, config, factory);
          }
        }
        Zone::Right => {
          if mouse_state.woop_down {
            on_up_woop(options, state, config, factory, mouse_state);
          }
        }
        ZONE_QUESTS => {
          if mouse_state.up_quest {
            on_up_quest(options, state, config, factory, mouse_state);
          }
        }
        ZONE_SAVE_MAP => {
          if mouse_state.up_save_map {
            on_up_save_map(options, state, config, factory, mouse_state, quick_saves);
          }
        }
        ZONE_FLOOR => {
          on_up_floor_zone(options, state, config, factory, cell_selection, &mouse_state);
        }
        _ => {}
      }
    }
  }
}

// on over, out, hover, down, up, drag start, dragging, drag end. but not everything makes sense for all cases.

fn on_after_resize_event(options: &Options, state: &mut State, config: &Config, ref_counted_canvas: &Rc<web_sys::HtmlCanvasElement>) {
  // Need to check whether we are in fullscreen mode or not.
  // In fullscreen mode the painted canvas area may be implicitly scaled (with no clue
  // given of this fact) which screws up mouse coordinate translations.
  // To fix that we need to check fullscreen status, and in that case, assume the canvas
  // is 100% wide or high and then determine scale factor accordingly, then update here.
  // tldr; in fullscreen mode the api will lie about the size of the _painted area_

  let fse = document().fullscreen_element();
  if let Some(_) = fse {
    if options.trace_size_changes { log!("(size) We are in canvas fullscreen mode"); }

    // Actual pixels we paint is not affected by fullscreen so get them first
    state.canvas_pixel_width = ref_counted_canvas.width() as f64;
    state.canvas_pixel_height = ref_counted_canvas.height() as f64;
    if options.trace_size_changes { log!("(size) pixel size: {}x{}", state.canvas_pixel_width, state.canvas_pixel_height); }

    // Canvas dimensions are lying to us. We must assume the canvas is max wide or high.
    // We have to compute the maxed dimension manually and the scale factor too.

    // Note: screen api is for actual screen, not browser inside window. innerWidth is what we want here.
    let window = window();
    let sw = window.inner_width().unwrap().as_f64().expect("to work") as f64;
    let sh = window.inner_height().unwrap().as_f64().expect("to work") as f64;
    if options.trace_size_changes { log!("(size) Window size is {}x{}x", sw, sh); }

    // Determine ratios.
    let rw = sw / state.canvas_pixel_width;
    let rh = sh / state.canvas_pixel_height;
    // We want the lowest ratio
    let scale = rw.min(rh);
    if options.trace_size_changes { log!("(size) ratios: {} {}", rw, rh); }

    // Note: this will be the size of the visual canvas _excluding_ the secret padding added
    // by fullscreen api. This is what we use for mouse-to-world coordinate translations.
    state.canvas_css_width = (state.canvas_pixel_width * scale).floor();
    state.canvas_css_height = (state.canvas_pixel_height * scale).floor();
    // Full screen will center the painted pixels so the mouse has to compensate for padding
    state.canvas_css_x = ((ref_counted_canvas.client_width() as f64 - state.canvas_css_width) / 2.0).floor();
    state.canvas_css_y = ((ref_counted_canvas.client_height() as f64 - state.canvas_css_height) / 2.0).floor();
    if options.trace_size_changes { log!("(size) css size: {}x{} (scale {}), offset {}x{}", state.canvas_css_width, state.canvas_css_height, scale, state.canvas_css_x, state.canvas_css_y); }
  } else {
    if options.trace_size_changes { log!("(size) We are not in canvas fullscreen mode"); }

    state.canvas_pixel_width = ref_counted_canvas.width() as f64;
    state.canvas_pixel_height = ref_counted_canvas.height() as f64;
    state.canvas_css_x = 0.0;
    state.canvas_css_y = 0.0;
    state.canvas_css_width = ref_counted_canvas.client_width() as f64;
    state.canvas_css_height = ref_counted_canvas.client_height() as f64;
  }

  if options.trace_size_changes { log!("(size) Internal css size now: {}x{}, canvas pixels: {}x{}", state.canvas_css_width, state.canvas_css_height, state.canvas_pixel_width, state.canvas_pixel_height); }
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
fn on_up_floor_zone(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState) {
  log!("on_up_floor_zone()");
  on_click_inside_floor_zone(options, state, config, factory, cell_selection, mouse_state);
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
fn on_drag_end_floor(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState) {
  log!("on_drag_end_floor()");
  on_drag_end_floor_other(options, state, config, factory, cell_selection, mouse_state);
}
fn on_drag_start_atom(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, mouse_state: &mut MouseState, cell_selection: &mut CellSelection) {
  log!("on_drag_start_atom()");
  // Is that atom visible / interactive yet?
  if atom_is_visible(factory, mouse_state.atom_down_atom_index) {
    // Need to remember which atom we are currently dragging (-> atom_down_atom_index).
    log!("is_drag_start from atom {} ({:?})", mouse_state.atom_down_atom_index, factory.available_atoms[mouse_state.atom_down_atom_index].0);
    mouse_state.dragging_atom = true;
    state.mouse_mode_selecting = false;
    log!("not closing machine while dragging atomic part");
  }
}
fn on_drag_start_woop(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, mouse_state: &mut MouseState, cell_selection: &mut CellSelection) {
  log!("on_drag_start_woop()");
  // Is that woop visible / interactive yet?
  if woop_is_visible(factory, mouse_state.woop_down_woop_index) {
    // Need to remember which woop we are currently dragging (-> woop_down_woop_index).
    log!("is_drag_start from woop {} ({:?})", mouse_state.woop_down_woop_index, factory.available_woops[mouse_state.woop_down_woop_index].0);

    // Config determines which machine to use and what asset
    let node = &config.nodes[factory.available_woops[mouse_state.woop_down_woop_index].0];
    let w = node.machine_width;
    let h = node.machine_height;

    on_drag_start_machine(options, state, config, mouse_state, cell_selection, w, h, factory.available_woops[mouse_state.woop_down_woop_index].0);
  }
}
fn on_up_atom(options: &Options, state: &State, config: &Config, factory: &Factory, mouse_state: &mut MouseState) {
  log!("on_up_atom({} -> {})", mouse_state.atom_down_atom_index, mouse_state.atom_hover_atom_index);

  let ( _part_kind, visible ) = factory.available_atoms[mouse_state.atom_down_atom_index];
  if !visible {
    // Invisible atoms are not interactive
    return;
  }
  if mouse_state.atom_down_atom_index != mouse_state.atom_hover_atom_index {
    // Did not pointer up on the same atom as we did the pointer down
    return;
  }

  if mouse_state.atom_selected && mouse_state.atom_selected_index == mouse_state.atom_hover_atom_index {
    log!("Deselecting atom {}", mouse_state.atom_hover_atom_index);
    mouse_state.atom_selected = false;
  } else {
    log!("Selecting atom {}", mouse_state.atom_hover_atom_index);
    mouse_state.woop_selected = false;
    mouse_state.atom_selected = true;
    mouse_state.atom_selected_index = mouse_state.atom_hover_atom_index;
  }
}
fn on_up_woop(options: &Options, state: &State, config: &Config, factory: &Factory, mouse_state: &mut MouseState) {
  log!("on_up_woopwoop({} -> {})", mouse_state.woop_down_woop_index, mouse_state.woop_hover_woop_index);

  let ( _part_kind, visible ) = factory.available_woops[mouse_state.woop_down_woop_index];
  if !visible {
    // Invisible woops are not interactive
    return;
  }
  if mouse_state.woop_down_woop_index != mouse_state.woop_hover_woop_index {
    // Did not pointer up on the same woop as we did the pointer down
    return;
  }

  if mouse_state.woop_selected && mouse_state.woop_selected_index == mouse_state.woop_hover_woop_index {
    log!("Deselecting woop {}", mouse_state.woop_hover_woop_index);
    mouse_state.woop_selected = false;
  } else {
    log!("Selecting woop {}", mouse_state.woop_hover_woop_index);
    mouse_state.atom_selected = false;
    mouse_state.woop_selected = true;
    mouse_state.woop_selected_index = mouse_state.woop_hover_woop_index;
  }
}
fn on_click_inside_floor_zone(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState) {
  let last_mouse_up_cell_x = mouse_state.last_up_cell_x.floor();
  let last_mouse_up_cell_y = mouse_state.last_up_cell_y.floor();
  log!("on_click_inside_floor_zone(), {} {}", last_mouse_up_cell_x, last_mouse_up_cell_y);
  if bounds_check(
    last_mouse_up_cell_x, last_mouse_up_cell_y,
    0.0, 0.0, FLOOR_CELLS_W as f64, FLOOR_CELLS_H as f64
  ) {
    on_click_inside_floor(options, state, config, factory, cell_selection, mouse_state, last_mouse_up_cell_x, last_mouse_up_cell_y);
  }
}
fn on_click_inside_floor(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, cell_selection: &mut CellSelection, mouse_state: &MouseState, last_mouse_up_cell_x: f64, last_mouse_up_cell_y: f64) {
  log!("on_click_inside_floor()");
  let action = mouse_button_to_action(state, mouse_state);

  if action == Action::Remove {
    // Clear the cell if that makes sense for it. Delete a belt with one or zero ports.
    let coord = to_coord(last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize);

    if is_edge_not_corner(last_mouse_up_cell_x, last_mouse_up_cell_y) {
      // Remove supplier/demander
      log!("Deleting edge cell @{} after rmb click", coord);
      floor_delete_cell_at_partial(options, state, config, factory, coord);
      factory.changed = true;
    } else {
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
  }
  else if action == Action::Add {
    // De-/Select this cell
    // For suppliers; cycle through legit inputs

    let coord = to_coord(last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize);

    log!("clicked {} {} cell selection before: {:?}, belt: {:?}", last_mouse_up_cell_x, last_mouse_up_cell_y, cell_selection, factory.floor[coord].belt);

    if cell_selection.on && cell_selection.x == last_mouse_up_cell_x && cell_selection.y == last_mouse_up_cell_y {
      // For suppliers we will cycle rather than toggle.
      // For anything else we will toggle the selection.
      if factory.floor[coord].kind == CellKind::Supply {
        // Let's assume that we only place atoms in the edge
        let len = factory.available_atoms.len();
        // - Find the index of the current part
        // - Loop through available parts to find next part
        let current_part = factory.floor[coord].supply.gives.kind;
        let mut current_index = 0;
        for i in 0..len {
          if factory.available_atoms[i].1 && factory.available_atoms[i].0 == current_part {
            current_index = i;
            break;
          }
        }
        let mut new_part = current_part;
        for i in 1..len {
          // First check if the part is actually visible in menu
          let new_index = (current_index + i) % len;
          if factory.available_atoms[new_index].1 {
            let t_part = factory.available_atoms[new_index].0;
            if config.nodes[t_part].pattern_unique_kinds.len() == 0 {
              new_part = t_part;
              break;
            }
          }
        }
        // It seems new_part is the next available zero-pattern part :)
        log!("Going to change part of supplier @{} from {:?} to {:?}", coord, config.nodes[current_part].raw_name, config.nodes[new_part].raw_name);
        factory.floor[coord].supply.gives = part_from_part_kind(config, new_part);
        factory.changed = true;
      } else {
        cell_selection.on = false;
      }
    }
    else if factory.floor[coord].kind == CellKind::Empty {
      if is_edge_not_corner(last_mouse_up_cell_x, last_mouse_up_cell_y) {
        log!("Clicked on empty edge. Not selecting it. Removing current selection. Showing user edge drag hint.");
        cell_selection.on = false;

        // - find the first visible unlocked part that has no pattern (an atom)
        // - find the coord of its atom square
        // - record the current mouse coordinate
        // - record the start time and compute the time it should take to move to the current coordinate
        // - every frame while the animation is active, paint a shadow of the atom at the progress

        // Find the first craftable part config node index
        let mut part_kind = CONFIG_NODE_PART_NONE;
        let mut atom_index = 0;
        factory.available_atoms.iter().enumerate().any(|(i, (kind, visible))| {
          if !visible { return false; }

          if config.nodes[*kind].pattern_unique_kinds.len() == 0 {
            part_kind = *kind;
            atom_index = i;
            return true;
          }
          return false;
        });

        let dir = match (
          last_mouse_up_cell_x == 0.0, // left
          last_mouse_up_cell_y == 0.0, // up
          last_mouse_up_cell_x as usize == FLOOR_CELLS_W - 1, // right
          last_mouse_up_cell_y as usize == FLOOR_CELLS_H - 1 // down
        ) {
          ( false, true, false, false ) => Direction::Left,
          ( false, false, true, false ) => Direction::Up,
          ( false, false, false, true ) => Direction::Right,
          ( true, false, false, false ) => Direction::Down,
          _ => panic!("Should always ever be one side"),
        };

        // If the cell next to this edge cell is a dead end with only inports then create a demander instead
        let (x, y) = to_xy(coord);
        let neighbor_coord = get_edge_neighbor(x, y, coord);
        if factory.floor[neighbor_coord.0].kind != CellKind::Empty && !port_has_outbound(&factory, coord) {
          log!("Neighbor of edge is not empty but has no outgoing; creating a demander here");
          set_empty_edge_to_demander(options, state, config, factory, part_kind, coord, dir);
        } else {
          log!("Neighbor of edge is empty or has outgoing; creating a supplier here");

          factory.edge_hint = (
            part_kind,
            (UI_FLOOR_OFFSET_X + last_mouse_up_cell_x * CELL_W, UI_FLOOR_OFFSET_Y + last_mouse_up_cell_y * CELL_H),
            get_atom_xy(atom_index),
            factory.ticks,
            2 * (ONE_SECOND as f64 * options.speed_modifier_ui) as u64
          );
          log!("edge_hint is now: {:?}", factory.edge_hint);

          set_empty_edge_to_supplier(options, state, config, factory, part_kind, coord, dir);
        }
      } else {
        log!("Clicked on empty cell. Not selecting it. Removing current selection.");
        cell_selection.on = false;
      }
    }
    else {
      cell_selection.on = true;
      cell_selection.x = last_mouse_up_cell_x;
      cell_selection.y = last_mouse_up_cell_y;
      cell_selection.coord = coord;
    }
  }
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
    log!("Going back one snapshot from {} to {}, setting load_snapshot_next_frame=true", state.snapshot_undo_pointer, state.snapshot_undo_pointer - 1);
    state.snapshot_undo_pointer -= 1;
    state.load_snapshot_next_frame = true;
  } else {
    log!("ignored because there is no prior undo history");
  }
}
fn on_up_trash(options: &Options, state: &State, config: &Config, factory: &mut Factory, mouse_state: &mut MouseState) {
  log!("on_up_trash()");
  log!("Removing all cells from the factory...");
  for coord in 0..factory.floor.len() {
    let (x, y) = to_xy(coord);
    factory.floor[coord] = empty_cell(config, x, y);
  }
  factory.parts_in_transit.clear();
  factory.changed = true;
}
fn on_up_redo(options: &Options, state: &mut State, config: &Config, factory: &Factory, mouse_state: &mut MouseState) {
  log!("on_up_redo()");
  // if state.snapshot_undo_pointer is not equal to state.snapshot_pointer
  // move the pointer forward. otherwise assume that you can't go forward
  if state.snapshot_undo_pointer != state.snapshot_pointer {
    log!("Increasing snapshot pointer to {}, setting load_snapshot_next_frame=true", state.snapshot_undo_pointer + 1);
    state.snapshot_undo_pointer += 1;
    state.load_snapshot_next_frame = true;
  } else {
    log!("ignored because {} == {}", state.snapshot_undo_pointer, state.snapshot_pointer)
  }
}
fn on_down_quest(options: &Options, state: &State, config: &Config, factory: &Factory, mouse_state: &mut MouseState) {
  log!("on_down_quest({}). questss: {}", mouse_state.down_quest_visible_index, factory.quests.iter().filter(|q| q.status == QuestStatus::Active).collect::<Vec<_>>().len());
}
fn on_up_quest(options: &Options, state: &State, config: &Config, factory: &mut Factory, mouse_state: &mut MouseState) {
  log!("on_up_quest({}), quests: {}, down on {:?} {}", mouse_state.up_quest_visible_index, factory.quests.iter().filter(|q| q.status == QuestStatus::Active).collect::<Vec<_>>().len(), mouse_state.down_zone, mouse_state.down_quest_visible_index);

  if options.dbg_clickable_quests && mouse_state.down_quest && mouse_state.down_quest_visible_index == mouse_state.up_quest_visible_index {
    log!("  clicked on this quest (down=up). Completing it now...");

    let quest_index = quest_visible_index_to_quest_index(options, state, config, factory, mouse_state.up_quest_visible_index);
    if let Some(quest_index) = quest_index {
      log!("  quest_update_status: satisfying quest {} to production target", quest_index);
      factory.quests[quest_index].production_progress = factory.quests[quest_index].production_target;
      quest_update_status(factory, quest_index, QuestStatus::FadingAndBouncing, factory.ticks);
      factory.quests[quest_index].bouncer.bounce_from_index = mouse_state.up_quest_visible_index;
      factory.quests[quest_index].bouncer.bouncing_at = factory.ticks;
    } else {
      log!("Clicked on a quest index that doesnt exist right now. mouse_state.down_quest_visible_index={}, mouse_state.up_quest_visible_index={}", mouse_state.down_quest_visible_index, mouse_state.up_quest_visible_index);
    }
  }
}
fn on_up_save_map(options: &Options, state: &mut State, config: &Config, factory: &mut Factory, mouse_state: &mut MouseState, quick_saves: &mut [Option<QuickSave>; 9]) {
  log!("on_up_save_map()");

  if !mouse_state.down_save_map || mouse_state.down_save_map_index != mouse_state.up_save_map_index {
    log!("  down != up, bailing: {} {} {}", mouse_state.down_save_map, mouse_state.down_save_map_index, mouse_state.up_save_map_index);
    return;
  }

  if let Some(quick_save) = &quick_saves[mouse_state.up_save_map_index] {
    let (row, col) = match mouse_state.up_save_map_index {
      0 => (0.0, 0.0),
      1 => (0.0, 1.0),
      2 => (1.0, 0.0),
      3 => (1.0, 1.0),
      _ => panic!("no such button: {}", mouse_state.up_save_map_index),
    };
    let pressed_delete_area = hit_test_save_map_delete_part(mouse_state.world_x, mouse_state.world_y, row, col);

    if pressed_delete_area {
      log!("  deleting saved map");
      quick_saves[mouse_state.up_save_map_index] = None;
      let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
      local_storage.remove_item(format!("{}{}", LS_SAVE_SNAPX, mouse_state.up_save_map_index).as_str()).unwrap();
      local_storage.remove_item(format!("{}{}", LS_SAVE_PNGX, mouse_state.up_save_map_index).as_str()).unwrap();
    }
    else {
      log!("  loading saved map, snapshot pointer to {}, undo pointer too, setting load_snapshot_next_frame=true", state.snapshot_pointer);
      state.snapshot_pointer += 1;
      state.snapshot_undo_pointer = state.snapshot_pointer;
      state.snapshot_stack[state.snapshot_pointer % UNDO_STACK_SIZE] = quick_save.snapshot.clone();
      state.load_snapshot_next_frame = true;
    }
  } else {
    log!("  storing saved map");
    let document = document();

    // This element is created in this file but it's just easier to query it from the DOM ;)
    let game_map: web_sys::HtmlCanvasElement = document.get_element_by_id("$main_game_canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    // Create a new canvas and draw the floor area onto that canvas
    let floor_canvas = document.create_element("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    floor_canvas.set_width((UI_SAVE_THUMB_IMG_WIDTH) as u32);
    floor_canvas.set_height(UI_SAVE_THUMB_IMG_HEIGHT as u32);
    floor_canvas.style().set_property("image-rendering", "pixelated").expect("should work");
    {
      // Add temp canvas to DOM for debugging
      // document.get_element_by_id("$main_game").unwrap().append_child(&floor_canvas);
    }
    let floor_context = floor_canvas.get_context("2d").expect("get context must work").unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    floor_context.set_image_smoothing_enabled(false);
    floor_context.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
      &game_map,
      UI_FLOOR_OFFSET_X, UI_FLOOR_OFFSET_Y, FLOOR_WIDTH, FLOOR_HEIGHT,
      0.0, 0.0, (UI_SAVE_THUMB_WIDTH * 0.66).floor(), UI_SAVE_THUMB_HEIGHT
    ).expect("canvas api call to work");

    let thumb_canvas = document.create_element("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    thumb_canvas.set_width((UI_SAVE_THUMB_WIDTH) as u32);
    thumb_canvas.set_height(UI_SAVE_THUMB_HEIGHT as u32);
    thumb_canvas.style().set_property("image-rendering", "pixelated").expect("should work");
    let thumb_context = thumb_canvas.get_context("2d").expect("get context must work").unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    thumb_context.set_image_smoothing_enabled(false);

    // Now use the temporary floor canvas as a tile. We need that to paint rounded corners.
    // The rounded corners are created by a rounded corner path() with a .fill() action.
    if let Some(ptrn) = thumb_context.create_pattern_with_html_canvas_element(&floor_canvas, "repeat").expect("trying to load thumb") {
      canvas_round_rect(&thumb_context, 0.0, 0.0, UI_SAVE_THUMB_WIDTH, UI_SAVE_THUMB_HEIGHT);
      thumb_context.set_fill_style(&ptrn);
      thumb_context.fill();
      thumb_context.set_stroke_style(&"black".into());
      thumb_context.stroke();
    }
    {
      // Add temp canvas to DOM for debugging
      // document.get_element_by_id("$main_game").unwrap().append_child(&thumb_canvas);
    }

    // The thumb canvas now is a rounded corner rect with the floor in it.
    // It does not have the close button yet because that's hover sensitive so we paint that at runtime.

    let png: String = thumb_canvas.to_data_url_with_type(&"img/png").unwrap(); // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlCanvasElement.html#method.to_data_url
    log!("len: {}", png.len());
    log!("png: {}", png);

    // Get string of map
    let map_snapshot = generate_floor_dump(&options, &state, &config, &factory, dnow()).join("\n");

    // Store it there and in local storage
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage.set_item(format!("{}{}", LS_SAVE_SNAPX, mouse_state.up_save_map_index).as_str(), &map_snapshot).unwrap();
    local_storage.set_item(format!("{}{}", LS_SAVE_PNGX, mouse_state.up_save_map_index).as_str(), &png).unwrap();

    quick_saves[mouse_state.up_save_map_index] = Some(quick_save_create(mouse_state.up_save_map_index, &document, map_snapshot, png));
  }
}
fn on_copy_factory(options: &Options, state: &mut State, config: &Config, factory: &Factory) {
  log!("on_copy_factory()");
  let ok = copyToClipboard(generate_floor_dump(options, state, config, factory, dnow()).join("\n").into());
  if ok {
    state.load_copy_hint_kind = LoadCopyHint::Success;
    state.load_copy_hint_since = factory.ticks;
  }
}
fn on_paste_factory(options: &Options, state: &mut State, config: &Config, factory: &mut Factory) {
  log!("on_paste_factory()");

  // Unfortunately, at the time of writing all clipboard access in Rust requires enabling unsafe api's
  // Alternative ways don't seem to work. So I'm handling clipboard read/write from JS and
  // tunnel them back in through the "actions" API.
  getCurrentPaste(); // -> see index.html definition
}
fn on_drag_end_atom_over_floor(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, mouse_state: &MouseState, cell_selection: &mut CellSelection) {
  log!("on_drag_end_atom_over_floor()");

  let last_mouse_up_cell_x = mouse_state.last_up_cell_x.floor();
  let last_mouse_up_cell_y = mouse_state.last_up_cell_y.floor();
  let last_mouse_up_cell_coord = to_coord(last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize);

  let dragged_part_kind = factory.available_atoms[mouse_state.atom_down_atom_index].0;

  if is_edge_not_corner(last_mouse_up_cell_x, last_mouse_up_cell_y) {
    log!("Dropped an atom as supply on an edge cell that is not corner. Deploying... {} {}", last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize);
    log!("Drag started from atom {} ({:?})", mouse_state.atom_down_atom_index, dragged_part_kind);

    set_edge_to_part(options, state, config, factory, last_mouse_up_cell_x as usize, last_mouse_up_cell_y as usize, dragged_part_kind);
  }
  else if is_middle(last_mouse_up_cell_x, last_mouse_up_cell_y) {
    // There's currently no reason to drop atoms on machines
    log!("Dropped an atom in the middle of the floor. Ignoring");
  }
  else {
    log!("Dropped an atom on an edge corner. Ignoring");
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
            factory.floor[pcoord] = supply_cell(config, px, py, part_c(config, 't'), options.default_supply_speed, options.default_supply_cooldown, 0);
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
              if options.trace_cell_set_port { log!("  -- okay. first/last track segment: @{} got {:?}", coord, belt_type); }
              if options.trace_cell_set_port { log!("  - connect_belt_to_existing_neighbor_belts(), before: {:?} {:?} {:?} {:?}", factory.floor[coord].port_u, factory.floor[coord].port_r, factory.floor[coord].port_d, factory.floor[coord].port_l); }
              connect_belt_to_existing_neighbor_cells(options, state, config, factory, coord);
              if options.trace_cell_set_port { log!("  - connect_belt_to_existing_neighbor_belts(),  after: {:?} {:?} {:?} {:?}", factory.floor[coord].port_u, factory.floor[coord].port_r, factory.floor[coord].port_d, factory.floor[coord].port_l); }
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
fn on_up_fullscreen_button(options: &Options, state: &mut State, config: &Config, factory: &mut Factory, mouse_state: &MouseState) {
  log!("on_up_fullscreen_button()");
  state.request_fullscreen = true;
}
fn on_up_auto_build_button(options: &Options, state: &State, config: &Config, factory: &mut Factory, mouse_state: &MouseState) {
  log!("on_up_auto_build_button, factory.auto_build.phase={:?}", factory.auto_build.phase);

  if factory.auto_build.phase == AutoBuildPhase::None {
    log!("Starting AutoBuild...");
    auto_build_start(options, state, config, factory, mouse_state.world_x, mouse_state.world_y);
  } else {
    log!("Stopping AutoBuild...");
    factory.auto_build.phase = AutoBuildPhase::None;
  }
}
fn on_drag_start_machine(options: &mut Options, state: &mut State, config: &Config, mouse_state: &mut MouseState, cell_selection: &mut CellSelection, w: usize, h: usize, part: PartKind) {
  log!("on_drag_start_machine({}x{}, {})", w, h, part);
  mouse_state.is_dragging_machine = true;
  mouse_state.dragging_machine_w = w as u8;
  mouse_state.dragging_machine_h = h as u8;
  mouse_state.dragging_machine_part = part;
  state.mouse_mode_selecting = false;
  mouse_state.atom_selected = false;
  mouse_state.woop_selected = false;
  cell_selection.on = false;
}
fn on_drag_end_machine_over_floor(options: &mut Options, state: &mut State, config: &Config, factory: &mut Factory, mouse_state: &MouseState) {
  log!("on_drag_end_machine_over_floor({}, {}, {}, {})", mouse_state.last_up_cell_x, mouse_state.last_up_cell_y, mouse_state.dragging_machine_w, mouse_state.dragging_machine_h);
  assert!(mouse_state.last_up_cell_x >= 0.0 && mouse_state.last_up_cell_y >= 0.0, "should not call this when mouse is oob. usize cant be negative");

  let machine_cell_width = mouse_state.dragging_machine_w;
  let machine_cell_height = mouse_state.dragging_machine_h;
  let machine_part = mouse_state.dragging_machine_part;

  // Was dragging a machine and released it on the floor

  // First check eligibility: Would every part of the machine be on a middle cell, not edge?

  let cx = get_x_while_dragging_machine(mouse_state.last_up_cell_x, machine_cell_width as usize);
  let cy = get_y_while_dragging_machine(mouse_state.last_up_cell_y, machine_cell_height as usize);
  // Make sure the entire machine fits, not just the center or topleft cell
  if bounds_check(cx, cy, 1.0, 1.0, FLOOR_CELLS_W as f64 - (machine_cell_width as f64), FLOOR_CELLS_H as f64 - (machine_cell_height as f64)) {
    machine_add_to_factory(options, state, config, factory, cx as usize, cy as usize, machine_cell_width as usize, machine_cell_height as usize, machine_part);
  }
  else if is_edge_not_corner(mouse_state.last_up_cell_x, mouse_state.last_up_cell_y) {
    log!("Dropped a woop on an edge. That should be illegal! (TODO)"); // TODO: make this illegal or under option
    log!("Drag started from woop {} ({:?})", mouse_state.woop_down_woop_index, machine_part);

    set_edge_to_part(options, state, config, factory, mouse_state.last_up_cell_x as usize, mouse_state.last_up_cell_y as usize, machine_part);
  } else {
    log!("Machine not dropped inside the floor. Ignoring. {} {}", mouse_state.last_up_cell_x, mouse_state.last_up_cell_y);
  }
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

    MenuButton::FullScreenButton => {
      on_up_fullscreen_button(options, state, config, factory, mouse_state);
    }
    MenuButton::AutoBuildButton => {
      on_up_auto_build_button(options, state, config, factory, mouse_state);
    }

    MenuButton::UndoButton => {
      on_up_undo(options, state, config, factory, mouse_state);
    }
    MenuButton::ClearButton => {
      on_up_trash(options, state, config, factory, mouse_state);
    }
    MenuButton::RedoButton => {
      on_up_redo(options, state, config, factory, mouse_state);
    }
    MenuButton::PaintToggleButton => {
      on_up_paint_toggle(state);
    }

    MenuButton::SpeedMin => {
      let m = options.speed_modifier_floor;
      options.speed_modifier_floor = options.speed_modifier_floor.min(0.5) * 0.5;
      log!("pressed time minus, from {} to {}", m, options.speed_modifier_floor);
    }
    MenuButton::SpeedHalf => {
      let m = options.speed_modifier_floor;
      options.speed_modifier_floor = 0.5;
      log!("pressed time half, from {} to {}", m, options.speed_modifier_floor);
    }
    MenuButton::SpeedPlayPause => {
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
    MenuButton::SpeedDouble => {
      let m = options.speed_modifier_floor;
      options.speed_modifier_floor = 2.0;
      log!("pressed time two, from {} to {}", m, options.speed_modifier_floor);
    }
    MenuButton::SpeedPlus => {
      let m = options.speed_modifier_floor;
      options.speed_modifier_floor = options.speed_modifier_floor.max(2.0) * 1.5;
      log!("pressed time plus, from {} to {}", m, options.speed_modifier_floor);
    }
    MenuButton::Row2Button0 => {
      log!("pressed blow button. blowing the localStorage cache");
      let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
      local_storage.remove_item(LS_OPTIONS).unwrap();
      local_storage.remove_item(LS_LAST_MAP).unwrap();
      local_storage.remove_item(LS_SAVE_SNAP0).unwrap();
      local_storage.remove_item(LS_SAVE_PNG0).unwrap();
      local_storage.remove_item(LS_SAVE_SNAP1).unwrap();
      local_storage.remove_item(LS_SAVE_PNG1).unwrap();
      local_storage.remove_item(LS_SAVE_SNAP2).unwrap();
      local_storage.remove_item(LS_SAVE_PNG2).unwrap();
      local_storage.remove_item(LS_SAVE_SNAP3).unwrap();
      local_storage.remove_item(LS_SAVE_PNG3).unwrap();
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
      log!("(Test button)");
      log!("noop");
    }
    MenuButton::Row3Button4 => {
      log!("Clearing the unlock status so you can start again");
      quest_reset_progress(options, state, config, factory);
    }
    MenuButton::Row3Button5 => {
      panic!("Hit the panic button. Or another button without implementation.")
    }
    MenuButton::Row3Button6 => {
      log!("(no button here)");
    }

    MenuButton::CopyFactory => {
      on_copy_factory(options, state, config, factory);
    }
    MenuButton::PasteFactory => {
      on_paste_factory(options, state, config, factory);
    }
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

fn hit_test_menu_speed_buttons(x: f64, y: f64) -> MenuButton {
  if bounds_check(
    x, y,
    UI_SPEED_BUBBLE_OFFSET_X,
    UI_SPEED_BUBBLE_OFFSET_Y,
    UI_SPEED_BUBBLE_OFFSET_X + 5.0 * (2.0 * UI_SPEED_BUBBLE_RADIUS) + 4.0 * UI_SPEED_BUBBLE_SPACING,
    UI_SPEED_BUBBLE_OFFSET_Y + (2.0 * UI_SPEED_BUBBLE_RADIUS)
  ) {
    if hit_test_speed_bubble_x(x, y, BUTTON_SPEED_MIN_INDEX) {
      MenuButton::SpeedMin
    } else if hit_test_speed_bubble_x(x, y, BUTTON_SPEED_HALF_INDEX) {
      MenuButton::SpeedHalf
    } else if hit_test_speed_bubble_x(x, y, BUTTON_SPEED_PLAY_PAUSE_INDEX) {
      MenuButton::SpeedPlayPause
    } else if hit_test_speed_bubble_x(x, y, BUTTON_SPEED_DOUBLE_INDEX) {
      MenuButton::SpeedDouble
    } else if hit_test_speed_bubble_x(x, y, BUTTON_SPEED_PLUS_INDEX) {
      MenuButton::SpeedPlus
    } else {
      MenuButton::None
    }
  } else {
    MenuButton::None
  }
}
fn hit_test_menu_buttons(x: f64, y: f64) -> MenuButton {
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

      button
    } else {
      MenuButton::None
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

      button
    } else {
      MenuButton::None
    }
  }
  else {
    MenuButton::None
  }
}
fn hit_test_atoms(factory: &Factory, mx: f64, my: f64) -> (bool, usize) {
  if bounds_check(mx, my, UI_ATOMS_OFFSET_X, UI_ATOMS_OFFSET_Y, UI_ATOMS_OFFSET_X + UI_WOTOM_WIDTH_PLUS_MARGIN * UI_ATOMS_PER_ROW, UI_ATOMS_OFFSET_Y + UI_WOTOM_HEIGHT_PLUS_MARGIN * (factory.available_atoms.len() as f64 / UI_ATOMS_PER_ROW).ceil()) {
    let inside_atom_and_margin_x = (mx - UI_ATOMS_OFFSET_X) / UI_WOTOM_WIDTH_PLUS_MARGIN;
    if (mx - UI_ATOMS_OFFSET_X) - (inside_atom_and_margin_x.floor() * UI_WOTOM_WIDTH_PLUS_MARGIN) > UI_WOTOM_WIDTH {
      // In the horizontal margin. Miss.
      return ( false, 0 );
    }
    let inside_atom_and_margin_y = (my - UI_ATOMS_OFFSET_Y) / UI_WOTOM_HEIGHT_PLUS_MARGIN;
    if (my - UI_ATOMS_OFFSET_Y) - (inside_atom_and_margin_y.floor() * UI_WOTOM_HEIGHT_PLUS_MARGIN) > UI_WOTOM_HEIGHT {
      // In the vertical margin. Miss.
      return ( false, 0 );
    }

    let inside_atom_and_margin_index = (inside_atom_and_margin_x.floor() + inside_atom_and_margin_y.floor() * UI_ATOMS_PER_ROW) as usize;

    let mut count = 0;
    for i in 0..factory.available_atoms.len() {
      if factory.available_atoms[i].1 {
        if count == inside_atom_and_margin_index {
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
fn hit_test_woops(factory: &Factory, mx: f64, my: f64) -> (bool, usize) {
  if bounds_check(mx, my, UI_WOOPS_OFFSET_X, UI_WOOPS_OFFSET_Y, UI_WOOPS_OFFSET_X + UI_WOTOM_WIDTH_PLUS_MARGIN * UI_WOOPS_PER_ROW, UI_WOOPS_OFFSET_Y + UI_WOTOM_HEIGHT_PLUS_MARGIN * (factory.available_woops.len() as f64 / UI_WOOPS_PER_ROW).ceil()) {
    let inside_woop_and_margin_x = (mx - UI_WOOPS_OFFSET_X) / UI_WOTOM_WIDTH_PLUS_MARGIN;
    if (mx - UI_WOOPS_OFFSET_X) - (inside_woop_and_margin_x.floor() * UI_WOTOM_WIDTH_PLUS_MARGIN) > UI_WOTOM_WIDTH {
      // In the horizontal margin. Miss.
      return ( false, 0 );
    }
    let inside_woop_and_margin_y = (my - UI_WOOPS_OFFSET_Y) / UI_WOTOM_HEIGHT_PLUS_MARGIN;
    if (my - UI_WOOPS_OFFSET_Y) - (inside_woop_and_margin_y.floor() * UI_WOTOM_HEIGHT_PLUS_MARGIN) > UI_WOTOM_HEIGHT {
      // In the vertical margin. Miss.
      return ( false, 0 );
    }

    let inside_woop_and_margin_index = (inside_woop_and_margin_x.floor() + inside_woop_and_margin_y.floor() * UI_WOOPS_PER_ROW) as usize;

    let mut count = 0;
    for i in 0..factory.available_woops.len() {
      if factory.available_woops[i].1 {
        if count == inside_woop_and_margin_index {
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
fn hit_test_help_button(mx: f64, my: f64) -> bool {
  return bounds_check(mx, my, UI_HELP_X, UI_HELP_Y, UI_HELP_X + UI_HELP_WIDTH, UI_HELP_Y + UI_HELP_HEIGHT);
}
fn hit_test_speed_bubble_x(x: f64, y: f64, index: usize) -> bool {
  // Note: index should be one of the BUTTON_SPEED_***_INDEX
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

fn paint_machine_craft_menu(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, cell_selection: &CellSelection, mouse_state: &MouseState) {

  let hover_coord = to_coord(mouse_state.cell_x_floored as usize, mouse_state.cell_y_floored as usize);
  let hovering = mouse_state.over_floor_not_corner && factory.floor[hover_coord].kind == CellKind::Machine;
  let selected_coord = cell_selection.coord;
  let show = hovering || (state.mouse_mode_selecting && cell_selection.on && factory.floor[selected_coord].kind == CellKind::Machine);

  if !show {
    return;
  }

  let focus_coord = if hovering { hover_coord } else { selected_coord };
  let main_coord = factory.floor[focus_coord].machine.main_coord;
  let (main_x, main_y) = to_xy(main_coord);
  let main_wx = UI_FLOOR_OFFSET_X + (main_x as f64) * CELL_W;
  let main_wy = UI_FLOOR_OFFSET_Y + (main_y as f64) * CELL_H;

  // Find the center of the machine because .arc() requires the center x,y
  let machine_cw = factory.floor[main_coord].machine.cell_width as f64;
  let machine_ch = factory.floor[main_coord].machine.cell_height as f64;
  let machine_ww = machine_cw * CELL_W;
  let machine_wh = machine_ch * CELL_H;

  context.set_fill_style(&"#ffffff7f".into());
  context.fill_rect(main_wx, main_wy, machine_ww, machine_wh);

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

  // Draw the wants in the right spots inside the machine and green/red dots indicating having seen them
  let none = part_none(config);
  for i in 0..(machine_cw * machine_ch) as usize {
    if let Some(part) = factory.floor[main_coord].machine.wants.get(i).or(Some(&none)) {
      paint_segment_part_from_config(options, state, config, context, part.kind, main_wx + CELL_W * (i as f64 % machine_cw).floor(), main_wy + CELL_H * (i as f64 / machine_cw).floor(), CELL_W, CELL_H);

      // Draw an indicator that tells you what parts this machine needs and which ones it already received
      if part.kind != CONFIG_NODE_PART_NONE {
        let has = factory.floor[main_coord].machine.haves.iter().any(|p| p.kind == part.kind);
        context.set_fill_style(&(if has { "green" } else { "red" }).into());
        context.fill_rect(main_wx + CELL_W * (i as f64 % machine_cw).floor() + CELL_W - 8.0, main_wy + CELL_H * (i as f64 / machine_cw).floor() + CELL_H - 8.0, 5.0, 5.0);
      }
    }
  }
}
fn paint_debug_app(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, fps: &VecDeque<f64>, now: f64, since_prev: f64, ticks_todo: u64, estimated_fps: f64, rounded_fps: u64, factory: &Factory, mouse_state: &MouseState) {

  context.set_font(&"12px monospace");

  if !state.showing_debug_bottom {
    if options.dbg_show_fps || options.dbg_show_secret_menu {
      context.set_fill_style(&"black".into());
      context.fill_text(format!("fps: {}", fps.len()).as_str(), GRID_X3 - 70.0 + 0.5, GRID_Y0 + 15.0 + 0.5).expect("something error fill_text");
      return;
    }
  }

  let mut ui_lines = 0.0;

  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_APP_OFFSET_X, UI_DEBUG_APP_OFFSET_Y + (UI_DEBUG_APP_LINE_H * ui_lines), UI_DEBUG_APP_WIDTH, (UI_DEBUG_LINES + 1.0) * UI_DEBUG_APP_LINE_H);
  context.set_stroke_style(&"black".into());
  context.stroke_rect(UI_DEBUG_APP_OFFSET_X, UI_DEBUG_APP_OFFSET_Y + (UI_DEBUG_APP_LINE_H * ui_lines), UI_DEBUG_APP_WIDTH, (UI_DEBUG_LINES + 1.0) * UI_DEBUG_APP_LINE_H);

  context.set_fill_style(&"black".into());
  context.fill_text(format!("fps: {}  {}", fps.len(), factory.machines.len()).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("App time  : {}", (now / 1000.0).floor()).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");
  // context.fill_text(format!("color  : {:?}", get_drop_color(options, factory.ticks)).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("Since prev: {: >3} (@{})", since_prev.floor(), estimated_fps.round()).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("Ticks todo: {} (r? {})", ticks_todo, rounded_fps).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("Speed: floor = {}, ui = {}", options.speed_modifier_floor, options.speed_modifier_ui).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("mouse over : {:?}", mouse_state.over_zone).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(
    format!(
      "mouse abs  : {} x {} {} {}",
      (mouse_state.world_x * 100.0).round() / 100.0, (mouse_state.world_y * 100.0).round() / 100.0,
      if mouse_state.is_dragging { "drag" } else if mouse_state.is_down { "down" } else { "up" },
      mouse_state.last_down_button,
    ).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H
  ).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("mouse world: {} x {}", mouse_state.cell_x_floored, mouse_state.cell_y_floored).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("mouse cell : {:.2} x {:.2}", mouse_state.cell_x - mouse_state.cell_x_floored, mouse_state.cell_y - mouse_state.cell_y_floored).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("mouse coord : {}", if mouse_state.cell_x_floored < 0.0 || mouse_state.cell_y_floored < 0.0 || mouse_state.cell_x_floored >= FLOOR_CELLS_W as f64 || mouse_state.cell_y_floored >= FLOOR_CELLS_W as f64 { "oob".to_string() } else { format!("{}", to_coord(mouse_state.cell_x_floored as usize, mouse_state.cell_y_floored as usize)) }).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("down: {}, dragging: {}", mouse_state.is_down, mouse_state.is_dragging).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("down event type: {}", if mouse_state.last_down_event_type == EventSourceType::Mouse { "Mouse" } else { "Touch" }).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("active story: {}", state.active_story_index).as_str(), UI_DEBUG_APP_OFFSET_X + UI_DEBUG_APP_SPACING, UI_DEBUG_APP_OFFSET_Y + (ui_lines * UI_DEBUG_APP_LINE_H) + UI_DEBUG_APP_FONT_H).expect("something error fill_text");

  assert_eq!(ui_lines, UI_DEBUG_LINES, "keep these in sync for simplicity");
}
fn paint_debug_auto_build(options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, mouse_state: &MouseState) {

  if factory.auto_build.phase == AutoBuildPhase::None {
    return;
  }

  if !state.showing_debug_bottom {
    return;
  }

  let auto_build_mouse_x = (factory.auto_build.mouse_target_x - factory.auto_build.mouse_offset_x) * factory.auto_build.phase_progress;
  let auto_build_mouse_y = (factory.auto_build.mouse_target_y - factory.auto_build.mouse_offset_y) * factory.auto_build.phase_progress;

  let mut ui_lines = 0.0;

  context.set_fill_style(&"lightgreen".into());
  context.fill_rect(UI_DEBUG_AUTO_BUILD_OFFSET_X, UI_DEBUG_AUTO_BUILD_OFFSET_Y + (UI_DEBUG_AUTO_BUILD_LINE_H * ui_lines), UI_DEBUG_AUTO_BUILD_WIDTH, (UI_DEBUG_AUTO_BUILD_LINES + 1.0) * UI_DEBUG_AUTO_BUILD_LINE_H);
  context.set_stroke_style(&"black".into());
  context.stroke_rect(UI_DEBUG_AUTO_BUILD_OFFSET_X, UI_DEBUG_AUTO_BUILD_OFFSET_Y + (UI_DEBUG_AUTO_BUILD_LINE_H * ui_lines), UI_DEBUG_AUTO_BUILD_WIDTH, (UI_DEBUG_AUTO_BUILD_LINES + 1.0) * UI_DEBUG_AUTO_BUILD_LINE_H);

  context.set_fill_style(&"black".into());
  context.fill_text(format!("phase        : {:?} ({})", factory.auto_build.phase, factory.ticks - factory.auto_build.phase_at).as_str(), UI_DEBUG_AUTO_BUILD_OFFSET_X + UI_DEBUG_AUTO_BUILD_SPACING, UI_DEBUG_AUTO_BUILD_OFFSET_Y + (ui_lines * UI_DEBUG_AUTO_BUILD_LINE_H) + UI_DEBUG_AUTO_BUILD_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("mouse offset : {} x {}", factory.auto_build.mouse_offset_x, factory.auto_build.mouse_offset_y).as_str(), UI_DEBUG_AUTO_BUILD_OFFSET_X + UI_DEBUG_AUTO_BUILD_SPACING, UI_DEBUG_AUTO_BUILD_OFFSET_Y + (ui_lines * UI_DEBUG_AUTO_BUILD_LINE_H) + UI_DEBUG_AUTO_BUILD_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("mouse target : {} x {}", factory.auto_build.mouse_target_x, factory.auto_build.mouse_target_y).as_str(), UI_DEBUG_AUTO_BUILD_OFFSET_X + UI_DEBUG_AUTO_BUILD_SPACING, UI_DEBUG_AUTO_BUILD_OFFSET_Y + (ui_lines * UI_DEBUG_AUTO_BUILD_LINE_H) + UI_DEBUG_AUTO_BUILD_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("mouse now    : {} x {}", factory.auto_build.mouse_offset_x + auto_build_mouse_x.floor(), factory.auto_build.mouse_offset_y + auto_build_mouse_y.floor()).as_str(), UI_DEBUG_AUTO_BUILD_OFFSET_X + UI_DEBUG_AUTO_BUILD_SPACING, UI_DEBUG_AUTO_BUILD_OFFSET_Y + (ui_lines * UI_DEBUG_AUTO_BUILD_LINE_H) + UI_DEBUG_AUTO_BUILD_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("pause        : {} (left: {})", factory.auto_build.phase_pause, factory.auto_build.phase_pause - (factory.ticks - factory.auto_build.phase_at).min(factory.auto_build.phase_pause)).as_str(), UI_DEBUG_AUTO_BUILD_OFFSET_X + UI_DEBUG_AUTO_BUILD_SPACING, UI_DEBUG_AUTO_BUILD_OFFSET_Y + (ui_lines * UI_DEBUG_AUTO_BUILD_LINE_H) + UI_DEBUG_AUTO_BUILD_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("duration     : {} (left: {})", factory.auto_build.phase_duration, factory.auto_build.phase_duration - (factory.ticks - factory.auto_build.phase_pause - factory.auto_build.phase_at).min(factory.auto_build.phase_duration)).as_str(), UI_DEBUG_AUTO_BUILD_OFFSET_X + UI_DEBUG_AUTO_BUILD_SPACING, UI_DEBUG_AUTO_BUILD_OFFSET_Y + (ui_lines * UI_DEBUG_AUTO_BUILD_LINE_H) + UI_DEBUG_AUTO_BUILD_FONT_H).expect("something error fill_text");

  ui_lines += 1.0;
  context.set_fill_style(&"black".into());
  context.fill_text(format!("progress     : {}", (factory.auto_build.phase_progress * 100.0).floor() / 100.0).as_str(), UI_DEBUG_AUTO_BUILD_OFFSET_X + UI_DEBUG_AUTO_BUILD_SPACING, UI_DEBUG_AUTO_BUILD_OFFSET_Y + (ui_lines * UI_DEBUG_AUTO_BUILD_LINE_H) + UI_DEBUG_AUTO_BUILD_FONT_H).expect("something error fill_text");

  assert_eq!(ui_lines, UI_DEBUG_AUTO_BUILD_LINES, "keep these in sync for simplicity");
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
fn paint_supply_and_part_for_edge(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, cx: usize, cy: usize, part_kind: PartKind) {
  let ox = UI_FLOOR_OFFSET_X + CELL_W * (cx as f64);
  let oy = UI_FLOOR_OFFSET_Y + CELL_H * (cy as f64);
  let supply_config_node =
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
  paint_asset(&options, &state, &config, &context, supply_config_node, factory.ticks, ox, oy, CELL_W, CELL_H);
  paint_segment_part_from_config(options, state, config, context, part_kind, ox + CELL_W/4.0, oy + CELL_H/4.0, CELL_W/2.0, CELL_H/2.0);
}
fn paint_supply_and_part_not_edge(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, ox: f64, oy: f64, part_kind: PartKind) {
  paint_segment_part_from_config(options, state, config, context, part_kind, ox + CELL_W*0.13, oy + CELL_H*0.13, CELL_W*0.75, CELL_H*0.75);
}
fn paint_part_and_pattern_at_middle(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, main_coord: usize, part_kind: PartKind) {
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
  for i in 0..config.nodes[part_kind].pattern_by_index.len() {
    paint_segment_part_from_config(
      options, state, config, context,
      config.nodes[part_kind].pattern_by_index[i],
      UI_FLOOR_OFFSET_X + CELL_W * (mx as f64) + margin_w + pw * (i%3) as f64, UI_FLOOR_OFFSET_Y + CELL_H * (my as f64) + margin_h + ph * (i/3) as f64,
      pw, ph
    );
  }
}
fn paint_supply_and_part_not_floor(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, ox: f64, oy: f64, part_kind: PartKind) {
  paint_segment_part_from_config(options, state, config, context, part_kind, ox + CELL_W*0.13, oy + CELL_H*0.13, CELL_W*0.75, CELL_H*0.75);
}
fn paint_dock_stripes(
  options: &Options,
  state: &State,
  config: &Config,
  factory: &Factory,
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
  paint_asset(&options, &state, &config, &context, dock_target, factory.ticks, ox, oy, w, h);
  context.set_global_alpha(1.0);
}
fn paint_floor_round_way(
  options: &Options,
  state: &State,
  config: &Config,
  factory: &Factory,
  context: &Rc<web_sys::CanvasRenderingContext2d>,
) {
  // Paint a track around the floor. It will be smaller than the track on the floor.
  if !options.enable_maze_roundway_and_collection {
    return;
  }

  // The way-belt size is deliberately half of the floor-cell. This way we can predictably stack
  // the way-belt around the floor and start at arbitrary offsets with little computational overhead.
  let tsize = (CELL_H/2.0).floor();

  // We need the track to be a little wider than the floor
  let roundway_min_len = FLOOR_WIDTH + 4.0;
  let roundway_track_pieces = (roundway_min_len / tsize).ceil();
  let roundway_len = roundway_track_pieces * tsize;
  // At each end of this line there will be a corner piece
  let roundway_len_full = roundway_len + 2.0 * tsize;
  // Center it and we will find our x offset
  let x = (UI_FLOOR_OFFSET_X + FLOOR_WIDTH / 2.0 - roundway_len_full / 2.0).floor() + 0.5;
  let y = (UI_FLOOR_OFFSET_Y + FLOOR_HEIGHT / 2.0 - roundway_len_full / 2.0).floor() + 0.5;

  // Are there any demanders on the top row? We will paint the way above from right to left until the
  // first demander, and put a corner into it. If there is no demander at the top then we don't paint.

  let mut offset = 0.0;
  for i in 1..FLOOR_CELLS_W-1 {
    if factory.floor[i].kind == CellKind::Demand {
      offset = i as f64;
      break;
    }
  }
  // Each way-belt is half the size of a floor-belt. We want corners to cut exactly into the middle
  // of a cell. So we paint one way-belt centered next to the floor-edge-cell, meaning that the
  // other two way-belts 50/50 overlap the floor cell edge.
  // Provided offset is not zero, we start painting a corner belt at the offset at 25% of the cell.
  if offset > 0.0 {
    // Track above floor
    // Start painting at the offset'th floor-cell. Paint a corner at 25% and then a path to the
    // right side corner.
    paint_belt(options, state, config, context, x + 2.0 * tsize + offset * 2.0 * tsize, y, tsize, tsize, BeltType::D_R, 0, factory.ticks);
    paint_belt(options, state, config, context, x + 2.0 * tsize + offset * 2.0 * tsize, y + tsize, tsize, tsize, BeltType::D_U, 0, factory.ticks);

    for n in (2.0 + offset * 2.0) as usize..=(2 * FLOOR_CELLS_W) {
      if n % 2 == 1 && factory.floor[n/2].kind == CellKind::Demand {
        paint_belt(options, state, config, context, x + tsize + (n as f64) * tsize, y, tsize, tsize, BeltType::DL_R, 0, factory.ticks);
        paint_belt(options, state, config, context, x + tsize + (n as f64) * tsize, y + tsize, tsize, tsize, BeltType::D_U, 0, factory.ticks);
      } else {
        paint_belt(options, state, config, context, x + tsize + (n as f64) * tsize, y, tsize, tsize, BeltType::L_R, 0, factory.ticks);
      }
    }
  }

  // Continue to the right-side down. If there was an offset then assume to start painting from
  // the top. Otherwise start painting at the first right-side demander.
  if offset == 0.0 {
    offset = FLOOR_CELLS_H as f64;
    for i in 1..FLOOR_CELLS_H {
      if factory.floor[i * FLOOR_CELLS_H - 1].kind == CellKind::Demand {
        offset = i as f64;
        break;
      }
    }
  } else {
    offset = 0.0;
  }
  if offset as usize != FLOOR_CELLS_H {
    // Track right of floor
    // Start painting at the offset'th floor-cell. Paint a corner at 25% and then a path to the
    // bottom corner.
    if offset == 0.0 {
      // Top-right corner piece
      paint_belt(options, state, config, context, x + 2.0 * tsize + (FLOOR_CELLS_W as f64 * 2.0 * tsize), y, tsize, tsize, BeltType::L_D, 0, factory.ticks);
    } else {
      paint_belt(options, state, config, context, x + 2.0 * tsize + (FLOOR_CELLS_W as f64 * 2.0 * tsize), y + offset * 2.0 * tsize, tsize, tsize, BeltType::L_D, 0, factory.ticks);
      paint_belt(options, state, config, context, x + 2.0 * tsize + (FLOOR_CELLS_W as f64 * 2.0 * tsize) - tsize, y + offset * 2.0 * tsize, tsize, tsize, BeltType::L_R, 0, factory.ticks);
    }

    for n in (1.0 + offset * 2.0) as usize..=(2 * FLOOR_CELLS_H) + 1 {
      if n % 2 == 0 && factory.floor[(n/2) * FLOOR_CELLS_W - 1].kind == CellKind::Demand {
        paint_belt(options, state, config, context, x + 2.0 * tsize + (FLOOR_CELLS_W as f64 * 2.0 * tsize), y + (n as f64) * tsize, tsize, tsize, BeltType::LU_D, 0, factory.ticks);
        paint_belt(options, state, config, context, x + 2.0 * tsize + (FLOOR_CELLS_W as f64 * 2.0 * tsize) - tsize, y + (n as f64) * tsize, tsize, tsize, BeltType::L_R, 0, factory.ticks);
      } else {
        paint_belt(options, state, config, context, x + 2.0 * tsize + (FLOOR_CELLS_W as f64 * 2.0 * tsize), y + (n as f64) * tsize, tsize, tsize, BeltType::U_D, 0, factory.ticks);
      }
    }
  }

  // Now we repeat the same starting on the left side

  let mut offset2 = FLOOR_CELLS_W as f64;
  for i in 1..FLOOR_CELLS_H-1 {
    if factory.floor[i * FLOOR_CELLS_H].kind == CellKind::Demand {
      offset2 = i as f64;
      break;
    }
  }

  if offset2 as usize != FLOOR_CELLS_W {
    // Track left of floor
    // Start painting at the offset'th floor-cell. Paint a corner at 25% and then a path to the
    // bottom corner.
    paint_belt(options, state, config, context, x, y + 2.0 * tsize + offset2 * 2.0 * tsize, tsize, tsize, BeltType::R_D, 0, factory.ticks);
    paint_belt(options, state, config, context, x + tsize, y + 2.0 * tsize + offset2 * 2.0 * tsize, tsize, tsize, BeltType::R_L, 0, factory.ticks);

    for n in (1.0 + offset2 * 2.0) as usize..(2 * FLOOR_CELLS_H) {
      if n % 2 == 0 && factory.floor[(n/2) * FLOOR_CELLS_W].kind == CellKind::Demand {
        paint_belt(options, state, config, context, x, y + 2.0 * tsize + (n as f64) * tsize, tsize, tsize, BeltType::RU_D, 0, factory.ticks);
        paint_belt(options, state, config, context, x + tsize, y + 2.0 * tsize + (n as f64) * tsize, tsize, tsize, BeltType::R_L, 0, factory.ticks);
      } else {
        paint_belt(options, state, config, context, x, y + 2.0 * tsize + (n as f64) * tsize, tsize, tsize, BeltType::U_D, 0, factory.ticks);
      }
    }
  }

  // And the corner and bottom way

  if offset2 as usize == FLOOR_CELLS_W {
    for i in 1..FLOOR_CELLS_W-1 {
      if factory.floor[(FLOOR_CELLS_W * FLOOR_CELLS_H - FLOOR_CELLS_W) + i].kind == CellKind::Demand {
        offset2 = i as f64;
        break;
      }
    }
  } else {
    offset2 = 0.0;
  }
  if offset2 as usize != FLOOR_CELLS_W {
    // Track above floor
    if offset2 == 0.0 {
      // Bottom-left corner piece
      paint_belt(options, state, config, context, x, y + 2.0 * tsize + (FLOOR_CELLS_H as f64) * 2.0 * tsize, tsize, tsize, BeltType::U_R, 0, factory.ticks);
      paint_belt(options, state, config, context, x + 1.0 * tsize, y + 2.0 * tsize + (FLOOR_CELLS_H as f64) * 2.0 * tsize, tsize, tsize, BeltType::L_R, 0, factory.ticks);
      paint_belt(options, state, config, context, x + 2.0 * tsize, y + 2.0 * tsize + (FLOOR_CELLS_H as f64) * 2.0 * tsize, tsize, tsize, BeltType::L_R, 0, factory.ticks);
    } else {
      // Start painting at the offset'th floor-cell. Paint a corner at 25% and then a path to the
      // right side corner.
      paint_belt(options, state, config, context, x + 2.0 * tsize + offset2 * 2.0 * tsize, y + 2.0 * tsize + (FLOOR_CELLS_H as f64) * 2.0 * tsize, tsize, tsize, BeltType::U_R, 0, factory.ticks);
      paint_belt(options, state, config, context, x + 2.0 * tsize + offset2 * 2.0 * tsize, y + 2.0 * tsize + (FLOOR_CELLS_H as f64) * 2.0 * tsize - tsize, tsize, tsize, BeltType::U_D, 0, factory.ticks);
    }

    for n in (2.0 + offset2 * 2.0) as usize..=(2 * FLOOR_CELLS_W) {
      if n % 2 == 1 && factory.floor[(FLOOR_CELLS_W * FLOOR_CELLS_H - FLOOR_CELLS_W) + n/2].kind == CellKind::Demand {
        paint_belt(options, state, config, context, x + tsize + (n as f64) * tsize, y + 3.0 * tsize + (FLOOR_CELLS_H as f64) * 2.0 * tsize - tsize, tsize, tsize, BeltType::LU_R, 0, factory.ticks);
        paint_belt(options, state, config, context, x + tsize + (n as f64) * tsize, y + 3.0 * tsize + (FLOOR_CELLS_H as f64) * 2.0 * tsize - tsize - tsize, tsize, tsize, BeltType::U_D, 0, factory.ticks);
      } else {
        paint_belt(options, state, config, context, x + tsize + (n as f64) * tsize, y + 3.0 * tsize + (FLOOR_CELLS_H as f64) * 2.0 * tsize - tsize, tsize, tsize, BeltType::L_R, 0, factory.ticks);
      }
    }
  }

  if offset as usize != FLOOR_CELLS_H || offset2 as usize != FLOOR_CELLS_W {
    // Bottom-right piece
    let bt =
      if offset as usize != FLOOR_CELLS_H && offset2 as usize != FLOOR_CELLS_W { BeltType::LU_DR }
      else if offset as usize != FLOOR_CELLS_H { BeltType::U_DR }
      else { BeltType::L_DR };

    paint_belt(options, state, config, context, x + roundway_len_full - tsize, y + roundway_len_full - tsize, tsize, tsize, bt, 0, factory.ticks);

    // Down path into the machine
    paint_belt(options, state, config, context, x + roundway_len_full - tsize, y + roundway_len_full, tsize, tsize, BeltType::U_L, 0, factory.ticks);
    paint_belt(options, state, config, context, x + roundway_len_full - 2.0 * tsize, y + roundway_len_full, tsize, tsize, BeltType::R_D, 0, factory.ticks);
    paint_belt(options, state, config, context, x + roundway_len_full - 2.0 * tsize, y + roundway_len_full + tsize, tsize, tsize, BeltType::U_D, 0, factory.ticks);

    // Right into the stats
    paint_belt(options, state, config, context, x + roundway_len_full, y + roundway_len_full - tsize, tsize, tsize, BeltType::L_R, 0, factory.ticks);
    paint_belt(options, state, config, context, x + roundway_len_full + tsize, y + roundway_len_full - tsize, tsize, tsize, BeltType::L_R, 0, factory.ticks);
    paint_belt(options, state, config, context, x + roundway_len_full + 2.0 * tsize, y + roundway_len_full - tsize, tsize, tsize, BeltType::L_D, 0, factory.ticks);
    paint_belt(options, state, config, context, x + roundway_len_full + 2.0 * tsize, y + roundway_len_full, tsize, tsize, BeltType::U_DR, 0, factory.ticks);
    paint_belt(options, state, config, context, x + roundway_len_full + 3.0 * tsize, y + roundway_len_full, tsize, tsize, BeltType::L_R, 0, factory.ticks);
    paint_belt(options, state, config, context, x + roundway_len_full + 2.0 * tsize, y + roundway_len_full + tsize, tsize, tsize, BeltType::U_D, 0, factory.ticks);
    paint_belt(options, state, config, context, x + roundway_len_full + 2.0 * tsize, y + roundway_len_full + 2.0 * tsize, tsize, tsize, BeltType::U_DR, 0, factory.ticks);
    paint_belt(options, state, config, context, x + roundway_len_full + 3.0 * tsize, y + roundway_len_full + 2.0 * tsize, tsize, tsize, BeltType::L_R, 0, factory.ticks);
    paint_belt(options, state, config, context, x + roundway_len_full + 2.0 * tsize, y + roundway_len_full + 3.0 * tsize, tsize, tsize, BeltType::U_D, 0, factory.ticks);
    paint_belt(options, state, config, context, x + roundway_len_full + 2.0 * tsize, y + roundway_len_full + 4.0 * tsize, tsize, tsize, BeltType::U_DR, 0, factory.ticks);
    paint_belt(options, state, config, context, x + roundway_len_full + 3.0 * tsize, y + roundway_len_full + 4.0 * tsize, tsize, tsize, BeltType::L_R, 0, factory.ticks);
    paint_belt(options, state, config, context, x + roundway_len_full + 2.0 * tsize, y + roundway_len_full + 5.0 * tsize, tsize, tsize, BeltType::U_D, 0, factory.ticks);
    paint_belt(options, state, config, context, x + roundway_len_full + 2.0 * tsize, y + roundway_len_full + 6.0 * tsize, tsize, tsize, BeltType::U_R, 0, factory.ticks);
    paint_belt(options, state, config, context, x + roundway_len_full + 3.0 * tsize, y + roundway_len_full + 6.0 * tsize, tsize, tsize, BeltType::L_R, 0, factory.ticks);
  }

  if offset as usize != FLOOR_CELLS_H || offset2 as usize != FLOOR_CELLS_W {
    for ( p, px, py, phase ) in factory.parts_in_transit.iter() {
      paint_segment_part_from_config(options, state, config, context, *p, *px, *py, 10.0, 10.0);
    }
  }
}

fn paint_background_tiles1(
  options: &Options,
  state: &State,
  config: &Config,
  factory: &Factory,
  context: &Rc<web_sys::CanvasRenderingContext2d>,
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

        paint_dock_stripes(options, state, config, factory, context, dock_target, ox, oy, CELL_W, CELL_H);
      },
      CellKind::Belt => {
        paint_factory_belt(options, state, config, factory, coord, context, ox, oy, CELL_W, CELL_H);
      },
      CellKind::Machine => {
        // For machines, paint the top-left cell only but make the painted area cover the whole machine
        // TODO: each machine size should have a unique, customized, sprite
        if cell.machine.main_coord == coord {
          let machine_asset_index = machine_size_to_asset_index(cell.machine.cell_width, cell.machine.cell_height);
          paint_asset(options, state, config, context, machine_asset_index, factory.ticks,
            ox, oy,
            cell.machine.cell_width as f64 * CELL_W, cell.machine.cell_height as f64 * CELL_H
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
  factory: &Factory,
  context: &Rc<web_sys::CanvasRenderingContext2d>,
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
  factory: &Factory,
  context: &Rc<web_sys::CanvasRenderingContext2d>,
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
        // Note: The background was painted in paint_background_tiles1. This step paints stuff on top.
        if cell.machine.main_coord == coord {
          // Paint all overlays for this machine in this iteration, not just the one for this particular cell

          let mconfig = get_machine_ui_config(cell.machine.cell_width, cell.machine.cell_height);

          // Paint tiny output part in top-left
          paint_segment_part_from_config(options, state, config, context,
            cell.machine.output_want.kind,
            ox + mconfig.part_x * CELL_W,
            oy + mconfig.part_y * CELL_H,
            mconfig.part_w * CELL_W,
            mconfig.part_h * CELL_H
          );

          // Note: the set of all incoming and outgoing ports are stored on .ins and .outs
          // - If none of the outward ports are connected, then there is a problem
          // - If there's only outgoing or incoming ports, then there is a problem
          // - If there are no defined inputs, then there is a problem
          // - If the inputs don't define an output, then there is a problem (but that can't legally happen in the current iteration)

          // Paint alarm in bottom-right corner if the machine has a problem
          let mut weewoo = false;
          if factory.floor[cell.machine.main_coord].ins.len() == 0 {
            if options.show_machine_missing_hint {
              paint_asset(options, state, config, context, CONFIG_NODE_ASSET_MISSING_INPUTS, factory.ticks,
                ox + mconfig.missing_input_x, oy + mconfig.missing_input_y,
                CELL_W, CELL_H
              );
            }
            weewoo = true;
          } else if factory.floor[cell.machine.main_coord].outs.len() == 0 {
            if options.show_machine_missing_hint {
              paint_asset(options, state, config, context, CONFIG_NODE_ASSET_MISSING_OUTPUTS, factory.ticks,
                ox + mconfig.missing_output_x, oy + mconfig.missing_output_y,
                CELL_W, CELL_H
              );
            }
            weewoo = true;
          }

          if weewoo {
            paint_asset(options, state, config, context, CONFIG_NODE_ASSET_WEE_WOO, factory.ticks,
              ox + mconfig.wee_woo_x * CELL_W, oy + mconfig.wee_woo_y * CELL_H,
              config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].w, config.nodes[CONFIG_NODE_MACHINE_3X3].sprite_config.frames[0].h
            );
          }
        }
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

  if options.dbg_paint_tile_priority {
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
  if !options.dbg_paint_port_arrows {
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
  if !options.dbg_paint_belt_id {
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
fn paint_mouse_cursor(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  // paint_mouse_position
  let mut color = "#ff00ff7f";
  let mut diameter = PART_W / 2.0;

  let mut x = mouse_state.world_x;
  let mut y = mouse_state.world_y;
  if x == 0.0 && y == 0.0 {
    // Assume that the top-left corner coordinate means the mouse should not be painted
    return;
  }

  if factory.auto_build.phase != AutoBuildPhase::None {
    diameter = MOUSE_POINTER_RADIUS_AUTO_BUILD * 2.0;

    let auto_build_mouse_x = (factory.auto_build.mouse_target_x - factory.auto_build.mouse_offset_x) * factory.auto_build.phase_progress;
    let auto_build_mouse_y = (factory.auto_build.mouse_target_y - factory.auto_build.mouse_offset_y) * factory.auto_build.phase_progress;

    x = factory.auto_build.mouse_offset_x + auto_build_mouse_x.floor();
    y = factory.auto_build.mouse_offset_y + auto_build_mouse_y.floor();

    match factory.auto_build.phase {
      | AutoBuildPhase::DragTargetPartToMachine
      | AutoBuildPhase::DragInputPartToEdge
      | AutoBuildPhase::TrackToMachine
      | AutoBuildPhase::TrackFromMachine
      => {
        if factory.ticks - factory.auto_build.phase_at < factory.auto_build.phase_pause {
          // Do not show cursor as "pressing" while pausing the phase
          color = "#ffa500cc";
        } else {
          color = "#90ee90cc";
        }
      },
      AutoBuildPhase::Blocked => {
        // Hard red
        color = "red";
      },
      _ => {
        color = "#ffa500cc";
      },
    }
  }
  else if mouse_state.is_down {
    let action = mouse_button_to_action(state, mouse_state);
    if action == Action::Add {
      color = "#90ee90cc";
    } else {
      color = "tomato";
    }
  }

  context.set_fill_style(&color.into()); // Semi transparent circles
  context.begin_path();
  context.ellipse(x.floor() + 0.5, y.floor() + 0.5, diameter, diameter, 3.14, 0.0, 6.28).expect("to paint a circle");
  context.fill();
}
fn paint_mouse_action(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, cell_selection: &CellSelection) {
  if factory.auto_build.phase != AutoBuildPhase::None {
    match factory.auto_build.phase {
      AutoBuildPhase::DragInputPartToEdge => {
        if factory.ticks - factory.auto_build.phase_at < factory.auto_build.phase_pause {
          // Do not draw as dragging while paused at the start of a phase
        } else {
          let auto_build_mouse_x = (factory.auto_build.mouse_target_x - factory.auto_build.mouse_offset_x) * factory.auto_build.phase_progress;
          let auto_build_mouse_y = (factory.auto_build.mouse_target_y - factory.auto_build.mouse_offset_y) * factory.auto_build.phase_progress;
          let x = factory.auto_build.mouse_offset_x + auto_build_mouse_x.floor();
          let y = factory.auto_build.mouse_offset_y + auto_build_mouse_y.floor();

          let part_kind = factory.auto_build.machine_draggin_part_kind;

          if is_woop(config, part_kind) {
            // TODO: verify that this call is necessary here
            paint_ui_hover_droptarget_hint(options, state, config, context, factory, part_kind);

            // Only machines unless debug setting is enabled
            // When over a machine, preview the pattern over the machine? Or snap the offer to its center?

            // Mouse position determines actual cell that we check
            let coord = to_coord(x as usize, y as usize);
            if is_middle(x, y) && factory.floor[coord].kind == CellKind::Machine {
              let main_coord = factory.floor[coord].machine.main_coord;
              paint_part_and_pattern_at_middle(options, state, config, context, factory, main_coord, part_kind);
            } else {
              paint_supply_and_part_not_floor(options, state, config, context, x - ((CELL_W as f64) / 2.0), y - ((CELL_H as f64) / 2.0), part_kind);
            }
          }
          else {
            // Only edge. No point in dumping into machine, I guess? Maybe as an expensive supply? Who cares?
            if is_edge_not_corner(x, y) {
              paint_supply_and_part_for_edge(options, state, config, factory, context, x as usize, y as usize, part_kind);
            } else {
              paint_supply_and_part_not_edge(options, state, config, context, x - ((CELL_W as f64) / 2.0), y - ((CELL_H as f64) / 2.0), part_kind);
            }
          }
        }
      }
      AutoBuildPhase::DragTargetPartToMachine => {
        if factory.ticks - factory.auto_build.phase_at < factory.auto_build.phase_pause {
          // Do not draw as dragging while paused at the start of a phase
        } else {
          let auto_build_mouse_x = (factory.auto_build.mouse_target_x - factory.auto_build.mouse_offset_x) * factory.auto_build.phase_progress;
          let auto_build_mouse_y = (factory.auto_build.mouse_target_y - factory.auto_build.mouse_offset_y) * factory.auto_build.phase_progress;
          let x = factory.auto_build.mouse_offset_x + auto_build_mouse_x.floor();
          let y = factory.auto_build.mouse_offset_y + auto_build_mouse_y.floor();
          paint_mouse_while_dragging_machine_at_cell(options, state, factory, context, x, y, factory.auto_build.machine_w as usize, factory.auto_build.machine_h as usize);
        }
      }
      _ => {}
    }
  }
  else if state.mouse_mode_selecting {
    paint_mouse_in_selection_mode(options, state, config, factory, context, mouse_state, cell_selection);
  }
  else if mouse_state.dragging_atom {
    paint_mouse_while_dragging_atom(options, state, config, factory, context, mouse_state, cell_selection);
  }
  else if mouse_state.is_dragging_machine {
    paint_mouse_while_dragging_machine(options, state, factory, context, mouse_state, mouse_state.dragging_machine_w as usize, mouse_state.dragging_machine_h as usize);
  }
  else if mouse_state.over_floor_not_corner {
    paint_mouse_cell_location_on_floor(&context, &factory, &cell_selection, &mouse_state);
    if mouse_state.was_dragging || mouse_state.is_dragging {
      if mouse_state.down_floor_not_corner {
        if mouse_state.last_down_event_type == EventSourceType::Mouse {
          paint_belt_drag_preview(options, state, config, context, factory, cell_selection, mouse_state);
        }
      }
    }
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
fn paint_mouse_while_dragging_machine(options: &Options, state: &State, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, machine_cells_width: usize, machine_cells_height: usize) {
  paint_mouse_while_dragging_machine_at_cell(options, state, factory, context, mouse_state.world_x, mouse_state.world_y, machine_cells_width, machine_cells_height);
}
fn paint_mouse_while_dragging_machine_at_cell(options: &Options, state: &State, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, world_x: f64, world_y: f64, machine_cells_width: usize, machine_cells_height: usize) {
  // Paint drop zone over the edge cells
  context.set_fill_style(&"#00004444".into());

  // All edges
  context.fill_rect(UI_FLOOR_OFFSET_X, UI_FLOOR_OFFSET_Y, CELL_W, FLOOR_HEIGHT - CELL_H);
  context.fill_rect(UI_FLOOR_OFFSET_X + CELL_W, UI_FLOOR_OFFSET_Y, FLOOR_WIDTH - CELL_W, CELL_H);
  context.fill_rect(UI_FLOOR_OFFSET_X + FLOOR_WIDTH - CELL_W, UI_FLOOR_OFFSET_Y + CELL_H, CELL_W, FLOOR_HEIGHT - CELL_H);
  context.fill_rect(UI_FLOOR_OFFSET_X, UI_FLOOR_OFFSET_Y + FLOOR_HEIGHT - CELL_H, FLOOR_WIDTH - CELL_W, CELL_H);

  // Note that mouse cell x is not where the top-left most cell of the machine would be
  let top_left_machine_cell_x = get_x_while_dragging_machine((world_x - UI_FLOOR_OFFSET_X) / CELL_W, machine_cells_width);
  let top_left_machine_cell_y = get_y_while_dragging_machine((world_y - UI_FLOOR_OFFSET_Y) / CELL_H, machine_cells_height);

  // Make sure the entire machine fits, not just the center or topleft cell
  let legal = bounds_check(top_left_machine_cell_x, top_left_machine_cell_y, 1.0, 1.0, FLOOR_CELLS_W as f64 - (machine_cells_width as f64), FLOOR_CELLS_H as f64 - (machine_cells_height as f64));

  // Face out illegal options
  let ( paint_at_x, paint_at_y) =
    if legal {
      ( UI_FLOOR_OFFSET_X + top_left_machine_cell_x.round() * CELL_W, UI_FLOOR_OFFSET_Y + top_left_machine_cell_y.round() * CELL_H )
    } else {
      // Do not snap if machine would cover the edge
      let ox = world_x - ((machine_cells_width as f64) * (CELL_W as f64) / 2.0 );
      let oy = world_y - ((machine_cells_height as f64) * (CELL_H as f64) / 2.0 );
      ( ox, oy )
    };

  context.set_fill_style(&"black".into());
  context.set_fill_style(&COLOR_MACHINE_SEMI.into());
  let w = (machine_cells_width as f64) * CELL_W;
  let h = (machine_cells_height as f64) * CELL_H;
  context.fill_rect(paint_at_x, paint_at_y, w, h);
  if !legal { paint_illegal_dragging_woop(&context, paint_at_x, paint_at_y, w, h); }
  context.set_fill_style(&"black".into());
  context.fill_text("M", paint_at_x + (machine_cells_width as f64) * CELL_W / 2.0 - 5.0, paint_at_y + (machine_cells_height as f64) * CELL_H / 2.0 + 2.0).expect("no error")
}
fn paint_illegal_dragging_woop(context: &Rc<web_sys::CanvasRenderingContext2d>, x: f64, y: f64, w: f64, h: f64) {
  // tbd. dont like this part but it gets the job done I guess.
  context.set_stroke_style(&"red".into());
  context.stroke_rect(x, y, w, h);
  // context.set_line_width(3.0);
  // context.set_line_cap("round");
  let goal = 15.0;
  let lines_w = (w / goal).floor();
  let lines_h = (h / goal).floor();
  let step_w = w / lines_w;
  let step_h = h / lines_h;
  context.begin_path();
  for i in 0..lines_w as u32 {
    for j in 0..lines_h as u32 {
      let tx = (i as f64 * step_w).floor() + 0.5;
      let ty = (j as f64 * step_h).floor() + 0.5;
      context.move_to(x, y + ty);
      context.line_to(x + w, y + ty);
      context.move_to(x + tx, y);
      context.line_to(x + tx, y + h);
    }
  }
  context.stroke();
}
fn paint_mouse_while_dragging_atom(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, cell_selection: &CellSelection) {
  // the atom has no pattern; only allow to edge as supply

  let part_kind = factory.available_atoms[mouse_state.atom_down_atom_index].0;
  // TODO: should this work? should we copy from woop?
  paint_ui_hover_droptarget_hint(options, state, config, context, factory, part_kind);

  // Only edge. No point in dumping into machine, I guess? Maybe as an expensive supply? Who cares?
  if is_edge_not_corner(mouse_state.cell_x_floored, mouse_state.cell_y_floored) {
    paint_supply_and_part_for_edge(options, state, config, factory, context, mouse_state.cell_x_floored as usize, mouse_state.cell_y_floored as usize, part_kind);
  } else {
    paint_supply_and_part_not_edge(options, state, config, context, mouse_state.world_x - ((CELL_W as f64) / 2.0), mouse_state.world_y - ((CELL_H as f64) / 2.0), part_kind);
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

  if factory.floor[selected_coord].belt.part.kind != CONFIG_NODE_PART_NONE {
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
  if options.dbg_paint_zone_borders {
    context.set_stroke_style(&options.dbg_zone_border_color.clone().into());
    context.stroke_rect(GRID_X0, GRID_Y0, GRID_LEFT_WIDTH, GRID_TOP_HEIGHT);
    context.stroke_rect(GRID_X1, GRID_Y0, UI_FLOOR_WIDTH, GRID_TOP_HEIGHT);
    context.stroke_rect(GRID_X2, GRID_Y0, GRID_RIGHT_WIDTH, GRID_TOP_HEIGHT + GRID_PADDING + UI_FLOOR_HEIGHT + GRID_PADDING + GRID_BOTTOM_HEIGHT);
    context.stroke_rect(GRID_X0, GRID_Y1, GRID_LEFT_WIDTH, UI_FLOOR_HEIGHT);
    context.stroke_rect(GRID_X1, GRID_Y1, UI_FLOOR_WIDTH, UI_FLOOR_HEIGHT);
    context.stroke_rect(GRID_X0, GRID_Y2, GRID_LEFT_WIDTH, GRID_BOTTOM_HEIGHT);
    context.stroke_rect(GRID_X1, GRID_Y2, UI_FLOOR_WIDTH, GRID_BOTTOM_HEIGHT);
    context.stroke_rect(GRID_X0, GRID_Y3, GRID_LEFT_WIDTH + GRID_PADDING + UI_FLOOR_WIDTH + GRID_PADDING + GRID_RIGHT_WIDTH, GRID_BOTTOM_DEBUG_HEIGHT);
  }
}
fn paint_manual(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>) {
  if state.manual_open {
    paint_asset(options, state, config, context, CONFIG_NODE_ASSET_MANUAL, factory.ticks,
      100.0, 20.0, 740.0, 740.0
    );
  }
}
fn paint_border_hint(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>) {
  if factory.edge_hint.3 + factory.edge_hint.4 < factory.ticks {
    return;
  }

  let progress = 1.0 - (factory.ticks - factory.edge_hint.3) as f64 / factory.edge_hint.4 as f64;

  let mx = factory.edge_hint.1.0 as f64;
  let my = factory.edge_hint.1.1 as f64;

  let x = mx + (factory.edge_hint.2.0 - mx) * progress;
  let y = my + (factory.edge_hint.2.1 - my) * progress;

  paint_dock_stripes(options, state, config, factory, context, CONFIG_NODE_DOCK_UP, x, y, UI_WOTOM_WIDTH, UI_WOTOM_HEIGHT);
  let px = x + (UI_WOTOM_WIDTH / 2.0) - (CELL_W / 2.0);
  let py = y + (UI_WOTOM_HEIGHT / 2.0) - (CELL_H / 2.0);
  paint_segment_part_from_config(options, state, config, context, factory.edge_hint.0, px, py, CELL_W, CELL_H);
  context.set_stroke_style(&"white".into());
  context.stroke_rect(x, y, UI_WOTOM_WIDTH, UI_WOTOM_HEIGHT);
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
      paint_segment_part_from_config(&options, &state, &config, &context, factory.quests[quest_index].production_part_kind, x + 40.0, UI_QUESTS_OFFSET_Y + UI_QUESTS_HEIGHT + 50.0 + y, CELL_W, CELL_H);
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
  if options.dbg_paint_zone_hovers {
    context.set_fill_style(&"#99999970".into()); // 100% background
    match mouse_state.over_zone {
      Zone::None => {}
      Zone::TopLeft =>            context.fill_rect(GRID_X0, GRID_Y0, GRID_LEFT_WIDTH, GRID_TOP_HEIGHT),
      Zone::Top =>                context.fill_rect(GRID_X1, GRID_Y0, UI_FLOOR_WIDTH, GRID_TOP_HEIGHT),
      Zone::TopRight =>           context.fill_rect(GRID_X2, GRID_Y0, GRID_RIGHT_WIDTH, GRID_TOP_HEIGHT),
      Zone::Left =>               context.fill_rect(GRID_X0, GRID_Y1, GRID_LEFT_WIDTH, UI_FLOOR_HEIGHT),
      Zone::Middle =>             context.fill_rect(GRID_X1, GRID_Y1, UI_FLOOR_WIDTH, UI_FLOOR_HEIGHT),
      Zone::Right =>              context.fill_rect(GRID_X2, GRID_Y1, GRID_RIGHT_WIDTH, UI_FLOOR_HEIGHT),
      Zone::BottomLeft =>         context.fill_rect(GRID_X0, GRID_Y2, GRID_LEFT_WIDTH, GRID_BOTTOM_HEIGHT),
      Zone::Bottom =>             context.fill_rect(GRID_X1, GRID_Y2, UI_FLOOR_WIDTH, GRID_BOTTOM_HEIGHT),
      Zone::BottomRight =>        context.fill_rect(GRID_X2, GRID_Y2, GRID_RIGHT_WIDTH, GRID_BOTTOM_HEIGHT),
      Zone::BottomBottomLeft =>   context.fill_rect(GRID_X0, GRID_Y3, GRID_LEFT_WIDTH, GRID_BOTTOM_DEBUG_HEIGHT),
      Zone::BottomBottom =>       context.fill_rect(GRID_X1, GRID_Y3, UI_FLOOR_WIDTH, GRID_BOTTOM_DEBUG_HEIGHT),
      Zone::BottomBottomRight =>  context.fill_rect(GRID_X2, GRID_Y3, GRID_RIGHT_WIDTH, GRID_BOTTOM_DEBUG_HEIGHT),
      Zone::Manual => {}
      Zone::Margin => {}
    }
  }
}
fn paint_top_stats(context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory) {
  context.set_fill_style(&"black".into());
  context.fill_text(format!("Ticks: {}, Supplied: {}, Produced: {}, Received: {}, Trashed: {}", factory.ticks, factory.supplied, factory.produced, factory.accepted, factory.trashed).as_str(), 20.0, 20.0).expect("to paint");
  context.fill_text(format!("Current time: {}", factory.ticks).as_str(), 20.0, 40.0).expect("to paint");
}
fn paint_help_and_ai_button(options: &Options, state: &State, config: &Config, factory: &Factory, mouse_state: &MouseState, context: &Rc<web_sys::CanvasRenderingContext2d>, button_canvii: &Vec<web_sys::HtmlCanvasElement>) {

  paint_asset(options, state, config, context, if mouse_state.help_hover { CONFIG_NODE_ASSET_HELP_RED } else { CONFIG_NODE_ASSET_HELP_BLACK }, factory.ticks,
    UI_HELP_X, UI_HELP_Y, UI_HELP_WIDTH, UI_HELP_HEIGHT
  );

  paint_button(options, state, config, context, button_canvii, if factory.auto_build.phase != AutoBuildPhase::None { BUTTON_PRERENDER_INDEX_MEDIUM_SQUARE_DOWN } else { BUTTON_PRERENDER_INDEX_MEDIUM_SQUARE_UP }, UI_AUTO_BUILD_X, UI_AUTO_BUILD_Y);
  if mouse_state.over_menu_button == MenuButton::AutoBuildButton {
    context.set_fill_style(&"white".into());
  } else {
    context.set_fill_style(&FLOOR_YELLOW_COLOR.into());
  }
  if factory.auto_build.phase == AutoBuildPhase::None {
    context.set_font(&"30px verdana");
    context.fill_text( format!("AI").as_str(), UI_AUTO_BUILD_X + 14.0, UI_AUTO_BUILD_Y + 40.0).expect("yes");
  } else {
    context.set_font(&"48px verdana, sans-serif");
    context.fill_text(format!("!").as_str(), UI_AUTO_BUILD_X + 20.0, UI_AUTO_BUILD_Y + 47.0).expect("yes");
  }
}
fn paint_quests(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, mouse_state: &MouseState) {
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

    context.set_fill_style(&"white".into()); // 100% background
    context.fill_rect(x + 5.0, y + 4.0, UI_QUEST_WIDTH - 10.0, UI_QUEST_HEIGHT - 8.0);
    if progress > 0.0 {
      context.set_fill_style(&"grey".into()); // 100% background
      context.fill_rect(x + 5.0, y + 4.0, (UI_QUEST_WIDTH - 10.0) * progress, UI_QUEST_HEIGHT - 8.0);
    }

    paint_asset(options, state, config, context, CONFIG_NODE_ASSET_QUEST_FRAME, factory.ticks, x, y, UI_QUEST_WIDTH, UI_QUEST_HEIGHT);

    let part_kind = factory.quests[quest_index].production_part_kind;
    assert!(
      config.nodes[factory.quests[quest_index].production_part_kind].kind == ConfigNodeKind::Part,
      "quest part index should refer to Part node but was {:?}... have index: {:?}, but it points to: {:?}",
      config.nodes[factory.quests[quest_index].production_part_kind].kind,
      factory.quests[quest_index].production_part_kind,
      config.nodes[factory.quests[quest_index].production_part_kind]
    );

    let composed_of = &config.nodes[part_kind].pattern_unique_kinds;
    // Print input parts inside a given width. Spacing depends on how many parts there are.
    let margin_left = 10.0;
    let space_left = 120.0 - (margin_left + 25.0 + PART_W + 5.0);
    // Either put parts to the left with 5px spacing, or put them closer together if there are too many
    let spacing = (space_left / composed_of.len() as f64).min(PART_W + 5.0);

    for i in 0..composed_of.len() {
      paint_segment_part_from_config(options, state, config, context, composed_of[i], x + margin_left + (i as f64 * spacing), y + 8.0, PART_W, PART_H);
    }

    paint_asset(options, state, config, context, CONFIG_NODE_ASSET_DOUBLE_ARROW_RIGHT, 0, x + 120.0 - PART_W - 18.0, y + 8.0, 10.0, PART_H);

    paint_segment_part_from_config(options, state, config, context, part_kind, x + 120.0 - PART_W - 5.0, y + 8.0, PART_W, PART_H);

    context.set_font(&"12px monospace");
    context.set_fill_style(&"#ddd".into()); // 100% background
    context.fill_text(format!("{}/{}x", current_quest_progress, current_quest_target).as_str(), x + 120.0, y + (UI_QUEST_HEIGHT / 2.0) + 5.0).expect("oopsie fill_text");

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
    context.fill_rect(UI_QUESTS_OFFSET_X, UI_QUESTS_OFFSET_Y, UI_QUESTS_WIDTH, UI_QUESTS_HEIGHT);

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
          factory.quests[quest_index].production_part_kind,
          factory.quests[quest_index].unlocks_todo
        ).as_str(),
        UI_QUESTS_OFFSET_X,
        UI_QUESTS_OFFSET_Y + 20.0 + (quest_index as f64) * 15.0
      ).expect("oopsie fill_text");
    }
  }
}
fn paint_atoms(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, mouse_state: &MouseState, cell_selection: &CellSelection) -> usize {
  let ( is_mouse_over_atom, atom_hover_index ) =
    if mouse_state.is_dragging || mouse_state.was_dragging { ( false, 0 ) } // Drag start is handled elsewhere, while dragging do not highlight atoms
    else { ( mouse_state.atom_hover, mouse_state.atom_hover_atom_index ) };

  let mut highlight_index = 0;

  let mut inc = 0;
  for atom_index in 0..factory.available_atoms.len() {
    let (part_kind, part_interactable ) = factory.available_atoms[atom_index];
    if part_interactable {
      let highlight = if is_mouse_over_atom { is_mouse_over_atom && atom_index == atom_hover_index } else { mouse_state.atom_selected && mouse_state.atom_selected_index == atom_index };
      if highlight {
        highlight_index = atom_index + 1;
      }
      paint_atom(options, state, config, context, factory, mouse_state, cell_selection, atom_index, part_kind, inc, highlight, config.nodes[part_kind].pattern_unique_kinds.len() > 0);
      inc += 1;
    }
  }

  return highlight_index;
}
fn paint_atom(
  options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, mouse_state: &MouseState, cell_selection: &CellSelection,
  atom_index: usize, part_kind: PartKind, inc: usize, highlight: bool, is_machine_part: bool
) {
  let ( x, y ) = get_atom_xy(inc);

  if is_machine_part {
    context.set_fill_style(&MACHINE_ORANGE.into());
    context.fill_rect(x, y, UI_WOTOM_WIDTH, UI_WOTOM_HEIGHT);
  } else {
    paint_dock_stripes(options, state, config, factory, context, CONFIG_NODE_DOCK_UP, x, y, UI_WOTOM_WIDTH, UI_WOTOM_HEIGHT);
  }

  let px = x + (UI_WOTOM_WIDTH / 2.0) - (UI_WOTOM_ICON_SIZE / 2.0);
  let py = y + (UI_WOTOM_HEIGHT / 2.0) - (UI_WOTOM_ICON_SIZE / 2.0);
  paint_segment_part_from_config(options, state, config, context, part_kind, px, py, UI_WOTOM_ICON_SIZE, UI_WOTOM_ICON_SIZE);

  if highlight {
    // Popup is drawn in parent function
    context.set_stroke_style(&"black".into());
    context.stroke_rect(x - 1.0, y - 1.0, UI_WOTOM_WIDTH + 2.0, UI_WOTOM_HEIGHT + 2.0);
    context.stroke_rect(x, y, UI_WOTOM_WIDTH, UI_WOTOM_HEIGHT);
  } else {
    context.set_stroke_style(&"white".into());
    context.stroke_rect(x, y, UI_WOTOM_WIDTH, UI_WOTOM_HEIGHT);
  }

  highlight_atom_woop(options, state, config, factory, context, mouse_state, cell_selection, part_kind, x, y, px, py);
}
fn paint_woops(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, mouse_state: &MouseState, cell_selection: &CellSelection) -> ( usize, f64, f64 ) {
  let ( is_mouse_over_woop, woop_hover_index ) =
    if mouse_state.is_dragging || mouse_state.was_dragging { ( false, 0 ) } // Drag start is handled elsewhere, while dragging do not highlight woops
    else { ( mouse_state.woop_hover, mouse_state.woop_hover_woop_index ) };

  let mut highlight_index = 0;
  let mut highlight_x = 0.0;
  let mut highlight_y = 0.0;

  let mut inc = 0;
  for woop_index in 0..factory.available_woops.len() {
    let (part_kind, part_interactable ) = factory.available_woops[woop_index];
    if part_interactable {
      let highlight = if is_mouse_over_woop { is_mouse_over_woop && woop_index == woop_hover_index } else { mouse_state.woop_selected && mouse_state.woop_selected_index == woop_index };
      let ( x, y ) = get_woop_xy(inc);
      if highlight {
        highlight_index = woop_index + 1;
        highlight_x = x;
        highlight_y = y;
      }
      paint_woop(options, state, config, context, factory, mouse_state, cell_selection, woop_index, part_kind, x, y, highlight, config.nodes[part_kind].pattern_unique_kinds.len() > 0);
      inc += 1;
    }
  }

  return ( highlight_index, highlight_x, highlight_y );
}
fn paint_woop(
  options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, mouse_state: &MouseState, cell_selection: &CellSelection,
  woop_index: usize, part_kind: PartKind, x: f64, y: f64, highlight: bool, is_machine_part: bool
) {
  if is_machine_part {
    context.set_fill_style(&MACHINE_ORANGE.into());
    context.fill_rect(x, y, UI_WOTOM_WIDTH, UI_WOTOM_HEIGHT);
  } else {
    paint_dock_stripes(options, state, config, factory, context, CONFIG_NODE_DOCK_UP, x, y, UI_WOTOM_WIDTH, UI_WOTOM_HEIGHT);
  }

  let px = x + (UI_WOTOM_WIDTH / 2.0) - (UI_WOTOM_ICON_SIZE / 2.0);
  let py = y + (UI_WOTOM_HEIGHT / 2.0) - (UI_WOTOM_ICON_SIZE / 2.0);
  paint_segment_part_from_config(options, state, config, context, part_kind, px, py, UI_WOTOM_ICON_SIZE, UI_WOTOM_ICON_SIZE);

  if highlight {
    // Popup is drawn in parent function
    context.set_stroke_style(&"black".into());
    context.stroke_rect(x - 1.0, y - 1.0, UI_WOTOM_WIDTH + 2.0, UI_WOTOM_HEIGHT + 2.0);
    context.stroke_rect(x, y, UI_WOTOM_WIDTH, UI_WOTOM_HEIGHT);
  } else {
    context.set_stroke_style(&"white".into());
    context.stroke_rect(x, y, UI_WOTOM_WIDTH, UI_WOTOM_HEIGHT);
  }

  highlight_atom_woop(options, state, config, factory, context, mouse_state, cell_selection, part_kind, x, y, px, py);
}

fn highlight_atom_woop(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, cell_selection: &CellSelection, part_kind: PartKind, x: f64, y: f64, px: f64, py: f64) {
  // If current selected machine can create this woop, paint some green rotating pixel around it
  let selected_coord = cell_selection.coord;
  let selected_main_coord = factory.floor[selected_coord].machine.main_coord;
  let mut highlight_woop_has = false;
  let mut highlight_woop = false;

  if
    cell_selection.on &&
    factory.floor[selected_coord].kind == CellKind::Machine &&
    // // Check if the machine received all pattern parts for this woop. If so, it can build it and we should highlight it.
    // config.nodes[part_kind].pattern_unique_kinds.iter().all(|part_kind| {
    //   return factory.floor[selected_main_coord].machine.last_received_parts.contains(part_kind);
    // });
    // Check if this woop is one of the pattern parts
    config.nodes[factory.floor[selected_main_coord].machine.output_want.kind].pattern_unique_kinds.contains(&part_kind)
  {
    highlight_woop = true;
    highlight_woop_has = factory.floor[selected_main_coord].machine.last_received_parts.contains(&part_kind);
  }

  // Check against current machine that you're hovering over (but not dragging)
  if
    !highlight_woop &&
    !cell_selection.on &&
    mouse_state.over_zone == ZONE_FLOOR &&
    !mouse_state.is_down &&
    !mouse_state.is_up &&
    !mouse_state.is_dragging &&
    is_floor(mouse_state.cell_x_floored, mouse_state.cell_y_floored)
  {
    let hover_coord = to_coord(mouse_state.cell_x_floored as usize, mouse_state.cell_y_floored as usize);
    let hover_main_coord = factory.floor[hover_coord].machine.main_coord;
    if
      factory.floor[hover_main_coord].kind == CellKind::Machine &&
      // // Check if the machine received all pattern parts for this woop. If so, it can build it and we should highlight it.
      // config.nodes[part_kind].pattern_unique_kinds.iter().all(|part_kind| {
      //   return factory.floor[hover_main_coord].machine.last_received_parts.contains(part_kind);
      // })
      // Check if this woop is one of the pattern parts
      config.nodes[factory.floor[hover_main_coord].machine.output_want.kind].pattern_unique_kinds.contains(&part_kind)
    {
      highlight_woop = true;
      highlight_woop_has = factory.floor[hover_main_coord].machine.last_received_parts.contains(&part_kind);
    }
  }

  if highlight_woop {
    context.set_stroke_style(&(if highlight_woop_has { "#008800ff" } else { "red" }).into());
    context.save();
    context.set_line_width(5.0);
    context.stroke_rect(px, py, UI_WOTOM_ICON_SIZE, UI_WOTOM_ICON_SIZE);
    context.restore();
    // paint some pixels green? (https://colordesigner.io/gradient-generator)
    let ui_throttled_second = factory_ticks_to_game_ui_time(options, factory.ticks) * 2.0;
    let progress = ui_throttled_second % 1.0;
    paint_green_pixel(context, progress, 0.0, x, y, "#9ac48b"); // &"green"
    paint_green_pixel(context, progress, 1.0, x, y, "#8ebd7f");
    paint_green_pixel(context, progress, 2.0, x, y, "#83b773");
    paint_green_pixel(context, progress, 3.0, x, y, "#77b066");
    paint_green_pixel(context, progress, 4.0, x, y, "#6baa5a");
    paint_green_pixel(context, progress, 5.0, x, y, "#5fa34e");
    paint_green_pixel(context, progress, 7.0, x, y, "#539c42");
    paint_green_pixel(context, progress, 8.0, x, y, "#459635");
    paint_green_pixel(context, progress, 9.0, x, y, "#368f27");
  }
}
fn paint_woop_tooltip(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, woop_index: usize, highlight_x: f64, highlight_y: f64) {
  // Paint the woop tooltip / popup, paint_woop_hint
  let (part_kind, _part_interactable ) = factory.available_woops[woop_index];
  let required_parts = &config.nodes[part_kind].pattern_unique_kinds;

  if required_parts.len() > 0 {
    paint_woop_tooltip_with_part(options, state, config, factory, context, highlight_x, highlight_y, part_kind);
  }
}
fn paint_woop_tooltip_with_part(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, highlight_x: f64, highlight_y: f64, part_kind: PartKind) {
  // Paint the woop tooltip / popup, paint_woop_hint, given a part.
  // Set highlight_x to 0 if not for a particular woop iteraction.

  let required_parts = &config.nodes[part_kind].pattern_unique_kinds;

  // Config determines which machine to use and what asset
  let woop_config_node = &config.nodes[part_kind];
  let w = woop_config_node.machine_width;
  let h = woop_config_node.machine_height;
  let machine_asset_node_index = woop_config_node.machine_asset_index;

  if options.show_woop_hover_hint && highlight_x > 0.0 {
    // TODO: This rays idea does not work because lines of woops on the same horizontal line will just overlap
    //       Keeping it because I liked the idea and who knows.

    // Paint rays from the input parts
    let ray_interval = ONE_SECOND as f64 * 3.0;
    let tesla_interval = ONE_SECOND as f64 * 0.8;
    let ray_progress = (factory.ticks % ray_interval as u64) as f64 / ray_interval;
    let tesla_progress = (factory.ticks % tesla_interval as u64) as f64 / tesla_interval;
    let tesla_amp = 15.0;
    for index in 0..required_parts.len() {
      let req_part_kind = required_parts[index];
      let ( ox, oy ) = if is_atom(config, req_part_kind) {
        // Must be found so init is irrelevant
        let mut req_atom_index = 0;
        for i in 0..factory.available_atoms.len() {
          if factory.available_atoms[i].0 == req_part_kind {
            req_atom_index = i;
          }
        }
        get_atom_xy(req_atom_index)
      } else {
        // Must be found so init is irrelevant
        let mut req_woop_index = 0;
        for i in 0..factory.available_woops.len() {
          if factory.available_woops[i].0 == req_part_kind {
            req_woop_index = i;
          }
        }
        get_woop_xy(req_woop_index)
      };

      // Draw a line from ox/y to highlight_x/y with twirly effect
      context.save();

      // This is how canvas rotation works; you rotate around the center of what you're painting, paint it, then reset the translation matrix.
      // For this reason we must find the center of the atom/woop, rotate around that point, and draw a straight line up to the selected atom
      // By rotating it we can paint the twirly effect without having to worry about angles (any further than we already have to)

      let ray_angle = (oy - highlight_y).atan2(ox - highlight_x); // radians (zero to std::f64::consts::TAU), 0.0 is right
      let ray_len = -((highlight_x - ox).powf(2.0) + (highlight_y - oy).powf(2.0)).sqrt();

      let offset = ray_len * ray_progress;
      let sin = (tesla_progress * std::f64::consts::PI).sin();
      let tesla_y = sin * tesla_amp - (tesla_amp * 0.5);


      context.translate(ox + UI_WOTOM_WIDTH / 2.0, oy + UI_WOTOM_HEIGHT / 2.0).expect("oopsie translate");
      // tau=2*pi. r=0.0 is right. This rotates the entire canvas so you can just paint like normal
      // but it's as if you temporarily rotated your screen. So we draw the line from left to right.
      context.rotate(ray_angle).expect("oopsie rotate");

      // If the tesla effect should go behind the ray, paint it now
      if tesla_progress < 0.5 {
        context.begin_path();
        context.move_to(offset, tesla_y);
        context.line_to(offset + 3.0, tesla_y);
        context.set_stroke_style(&"green".into());
        context.set_line_width(8.0);
        context.stroke();
      }

      // The actual line
      context.begin_path();
      context.move_to(0.0, 0.0);
      context.line_to(ray_len, 0.0);
      context.set_stroke_style(&"white".into());
      context.set_line_width(5.0);
      context.stroke();

      // If the tesla effect should go over the ray, paint it now
      if tesla_progress >= 0.5 {
        context.begin_path();
        context.move_to(offset, tesla_y);
        context.line_to(offset + 3.0, tesla_y);
        context.set_stroke_style(&"green".into());
        context.set_line_width(8.0);
        context.stroke();
      }

      context.restore();
    }
  }

  // Vertical offset: either above, starting slightly in the middle of the top woops, and if the
  // selection is in the area where the tooltip would be painted then paint it one tooltip-height lower.

  // Relative width/height of the painted machine
  let bwp = w as f64 / w.max(h) as f64;
  let bhp = h as f64 / h.max(w) as f64;

  let machine_full_w = CELL_W * 2.5;
  let machine_full_h = CELL_H * 2.5;
  let machine_ox = UI_WOOP_TOOLTIP_X + 3.0 + (CELL_W * 0.75 + 5.0) * 2.0 + 20.0;
  let machine_oy = UI_WOOP_TOOLTIP_Y + 0.25 * CELL_H;
  let machine_w = (machine_full_w * bwp).floor();
  let machine_h = (machine_full_h * bhp).floor();

  canvas_round_rect_rc(context, UI_WOOP_TOOLTIP_X + 0.5, UI_WOOP_TOOLTIP_Y + 0.5, UI_WOOP_TOOLTIP_WIDTH + 5.0, UI_WOOP_TOOLTIP_HEIGHT + 7.0);
  context.set_fill_style(&"#dddddddd".into());
  context.fill();
  context.set_stroke_style(&"#000000ee".into());
  context.stroke();

  // Paint tiny parts as input. If there's more than 8 then eh, halt and catch fire?
  // Special model for 1, 2, or 3 inputs.
  match required_parts.len() {
    1 => {
      paint_segment_part_from_config(options, state, config, context, required_parts[0],
        (UI_WOOP_TOOLTIP_X + 3.0 + 13.0).floor() + 0.5,
        (UI_WOOP_TOOLTIP_Y + 3.0 + CELL_H * 0.75 + 5.0).floor() + 0.5,
        CELL_W,
        CELL_H
      );
    }
    2 => {
      paint_segment_part_from_config(options, state, config, context, required_parts[0],
        (UI_WOOP_TOOLTIP_X + 3.0 + 13.0).floor() + 0.5,
        (UI_WOOP_TOOLTIP_Y + 3.0 + 8.0).floor() + 0.5,
        CELL_W,
        CELL_H
      );
      paint_segment_part_from_config(options, state, config, context, required_parts[1],
        (UI_WOOP_TOOLTIP_X + 3.0 + 13.0).floor() + 0.5,
        (UI_WOOP_TOOLTIP_Y + 3.0 + CELL_H + 5.0 + 11.0).floor() + 0.5,
        CELL_W,
        CELL_H
      );
    }
    3 => {
      paint_segment_part_from_config(options, state, config, context, required_parts[0],
        (UI_WOOP_TOOLTIP_X + 3.0 + 16.0).floor() + 0.5,
        (UI_WOOP_TOOLTIP_Y + 3.0).floor() + 0.5,
        CELL_W * 0.75,
        CELL_H * 0.75
      );
      paint_segment_part_from_config(options, state, config, context, required_parts[1],
        (UI_WOOP_TOOLTIP_X + 3.0 + 16.0).floor() + 0.5,
        (UI_WOOP_TOOLTIP_Y + 3.0 + CELL_H * 0.75 + 5.0).floor() + 0.5,
        CELL_W * 0.75,
        CELL_H * 0.75
      );
      paint_segment_part_from_config(options, state, config, context, required_parts[2],
        (UI_WOOP_TOOLTIP_X + 3.0 + 16.0).floor() + 0.5,
        (UI_WOOP_TOOLTIP_Y + 3.0 + CELL_H * 0.75 + 5.0 + CELL_H * 0.75 + 5.0).floor() + 0.5,
        CELL_W * 0.75,
        CELL_H * 0.75
      );
    }
    | _ => {
      for i in 0..required_parts.len().min(8) {
        paint_segment_part_from_config(options, state, config, context, required_parts[i],
          (UI_WOOP_TOOLTIP_X + 3.0 + if i == 6 || i == 7 { CELL_W * 0.37 + 2.5 } else { (CELL_W * 0.75 + 5.0) * (i % 2) as f64 }).floor() + 0.5,
          (UI_WOOP_TOOLTIP_Y + 3.0 + if i == 6 || i == 7 { (CELL_H * 0.37 + 2.5) + (CELL_H * 0.75 + 5.0) * (i % 2) as f64 } else { (CELL_H * 0.75 + 5.0) * (i / 2) as f64 }).floor() + 0.5,
          0.75 * CELL_W,
          0.75 * CELL_H
        );
      }
    }
  }

  // Arrow
  paint_asset_raw(options, state, config, &context, CONFIG_NODE_ASSET_SINGLE_ARROW_RIGHT, factory.ticks,
    (machine_ox - 21.0 + (factory.ticks as f64 / (ONE_SECOND as f64 / 4.0) % 3.0)).floor() + 0.5,
    (machine_oy + (machine_full_h / 2.0) - 19.0).floor() + 0.5,
    13.0,
    38.0
  );

  let mx = machine_ox + (machine_full_w - machine_w) / 2.0;
  let my = machine_oy + (machine_full_h - machine_h) / 2.0;

  // Note: We paint the indicated machine centered within a given box, regardless of actual size
  let machine_img = &config.sprite_cache_canvas[config.nodes[machine_asset_node_index].sprite_config.frames[0].file_canvas_cache_index];
  context.draw_image_with_html_image_element_and_dw_and_dh(machine_img,
    mx.floor(),
    my.floor(),
    machine_w,
    machine_h
  ).expect("something error draw_image"); // requires web_sys HtmlImageElement feature

  // Next two loops paint a small grid behind the machine to indicate cell size
  let s = machine_w / w as f64;
  context.begin_path();
  for i in 0..w {
    context.move_to((mx + (i as f64) * s).floor() + 0.5, my.floor() + 0.5);
    context.line_to((mx + (i as f64) * s).floor() + 0.5, (my + machine_h).floor() + 0.5);
  }
  let s = machine_h / h as f64;
  for i in 0..h {
    context.move_to(mx.floor() + 0.5, (my + (i as f64) * s).floor() + 0.5);
    context.line_to((mx + machine_w).floor() + 0.5, (my + (i as f64) * s).floor() + 0.5);
  }
  context.set_stroke_style(&"#eeeeee99".into());
  context.stroke();

  if machine_w == machine_h {
    // Paint part twice as big
    paint_segment_part_from_config(options, state, config, context, part_kind, (mx + machine_w / 2.0 - CELL_W).floor() + 0.5, (my + machine_h / 2.0 - CELL_H).floor() + 0.5, CELL_W * 2.0, CELL_H * 2.0);
  } else {
    paint_segment_part_from_config(options, state, config, context, part_kind, (mx + machine_w / 2.0 - CELL_W / 2.0).floor() + 0.5, (my + machine_h / 2.0 - CELL_H / 2.0).floor() + 0.5, CELL_W, CELL_H);
  }
}
fn paint_lasers(options: &Options, state: &mut State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>) {
  // Paint quest lasers (parts that are received draw a line to the left menu)
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
    context.line_to(GRID_X0 as f64 + UI_QUESTS_WIDTH as f64 / 2.0, GRID_Y1 + (UI_QUEST_HEIGHT + UI_QUEST_MARGIN) as f64 * visible_quest_index as f64 + (UI_QUEST_HEIGHT as f64) / 2.0);
    context.stroke();

    state.lasers[i].ttl -= 1;
    if state.lasers[i].ttl <= 0 {
      state.lasers.remove(i);
    }
  }
}
fn paint_truck(
  options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, part_kind: PartKind,
  xyrs1: ( f64, f64, f64, f64 ), // x y rotation size. rotation is 0..1 of 2pi where 0.0 is up-facing, 0.25 is right-facing, etc
  xyrs2: ( f64, f64, f64, f64 ),
  ease: ( Ease, Ease, Ease, Ease ),
  progress: f64,
  ticks: u64,
) {
  let (x1, y1, r1, s1) = xyrs1;
  let (x2, y2, r2, s2) = xyrs2;
  let ( ease_x, ease_y, ease_r, ease_s ) = ease;

  let p = progress;

  let truck_x = ease_progress(x1, x2, progress, ease_x);
  let truck_y = ease_progress(y1, y2, progress, ease_y);
  let truck_r = ease_progress(r1, r2, progress, ease_r);
  let truck_size = ease_progress(s1, s2, progress, ease_s);

  context.save();
  // This is how canvas rotation works; you rotate around the center of what you're painting, paint it, then reset the translation matrix.
  // For this reason we must find the center of the dump truck, rotate around that point, and draw the dump track at minus half its size.
  context.translate(truck_x + truck_size / 2.0, truck_y + truck_size / 2.0).expect("oopsie translate");
  // tau=2*pi. r=0.0 is upward. This rotates the entire canvas so you can just paint like normal but it's as if you temporarily rotated your screen.
  context.rotate(std::f64::consts::TAU * truck_r).expect("oopsie rotate");
  // Compensate for the origin currently being in the middle of the dump truck. Top-left is just easier.
  context.translate(-truck_size/2.0, -truck_size/2.0).expect("oopsie translate");
  // The truck starts _inside_ the factory and drives to the right (maybe slanted)
  paint_asset_raw(options, state, config, context, CONFIG_NODE_ASSET_DUMP_TRUCK, ticks,
    0.0, 0.0, truck_size, truck_size
  );
  // Paint the part icon on the back of the trick (x-centered, y-bottom)
  paint_segment_part_from_config(&options, &state, &config, &context, part_kind, 0.0 + (truck_size / 2.0) - ((truck_size / 3.0) / 2.0), 0.0 + truck_size + -6.0 + -(truck_size / 3.0), truck_size / 3.0, truck_size / 3.0);
  context.restore();
}
fn paint_atom_truck_at_age(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &mut Factory, age: f64, part: PartKind, target_menu_part_position: usize) -> bool {
  // Paint a truck that is heading to the menu on the bottom-left

  // Basically this is the speed of each segment of the truck
  let m = 1.0; // Lower is faster.
  let tb1 = 1000.0 * m;
  let tb2 = 150.0 * m;
  let tb3 = 2000.0 * m;

  // let part = CONFIG_NODE_PART_NONE;
  if age < tb1 {
    paint_truck(options, state, config, context, part,
      ATOM_TRUCK_WP1, ATOM_TRUCK_WP2A,
      ( Ease::Sin, Ease::None, Ease::Cos, Ease::None ),
      // (time_since_truck / truck_dur_1).min(1.0).max(0.0)
      age/ tb1 % 10.0,
      factory.ticks,
    );
  }
  else if age < tb1 + tb2 {
    paint_truck(options, state, config, context, part,
      ATOM_TRUCK_WP2B, ATOM_TRUCK_WP3,
      ( Ease::None, Ease::None, Ease::Out, Ease::None ),
      // (time_since_truck / truck_dur_1).min(1.0).max(0.0)
      (age- tb1)/ tb2 % 10.0,
      factory.ticks,
    );
  }
  else if age < tb1 + tb2 + tb3 {
    // Get target coordinate where this part will be permanently drawn so we know where the truck has to move to
    // let ( target_x, target_y ) = if factory.trucks[t].for_woop { get_woop_xy(factory.trucks[t].target_menu_part_position) } else { get_atom_xy(factory.trucks[t].target_menu_part_position) };
    let ( target_x, target_y ) = get_atom_xy(target_menu_part_position);
    paint_truck(options, state, config, context, part,
      ATOM_TRUCK_WP3, ( target_x, target_y + CELL_H + 5.0, 1.05, ATOM_TRUCK_WP3.3 ),
      ( Ease::Out, Ease::Cos, Ease::Cos, Ease::None ),
      // (time_since_truck / truck_dur_1).min(1.0).max(0.0)
      (age-(tb1 + tb2))/ tb3 % 10.0,
      factory.ticks,
    );
  }
  else {
    // Truck reached its destiny.
    // - Enable the button
    // - Drop the truck
    if factory.available_atoms.len() > target_menu_part_position {
      factory.available_atoms[target_menu_part_position].1 = true;
    } else {
      log!("Must be testing but target_menu_part_position ({}) was oob on (factory.available_atoms ({})", target_menu_part_position, factory.available_atoms.len())
    }
    return true;
  }

  return false;
}
fn paint_woop_truck_at_age(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &mut Factory, age: f64, part: PartKind, target_menu_part_position: usize, piped: bool) -> bool {
  // Paint a truck heading to the right menu

  // Basically this is the number of ticks available per segment of the truck.
  let m = 1.0; // Lower is faster.
  let ta1 = 1000.0 * m;
  let ta2 = 320.0 * m;
  let ta3 = 120.0 * m;
  let ta4 = 240.0 * m;
  let ta5 = 800.0 * m;
  let ta6 = 760.0 * m;

  // let part = CONFIG_NODE_PART_NONE;

  if age < ta1 {
    // Curve out of the gate
    paint_truck(options, state, config, context, part,
      WOOP_TRUCK_WP1, WOOP_TRUCK_WP2,
      ( Ease::None, Ease::None, Ease::Cos, Ease::None ),
      // (time_since_truck / truck_dur_1).min(1.0).max(0.0)
      age/ ta1 % 10.0,
      factory.ticks,
    );
  }
  else if age < ta1 + ta2 {
    // Bounce back.
    paint_truck(options, state, config, context, part,
      WOOP_TRUCK_WP2, if piped { WOOP_TRUCK_WP3_JUMP } else { WOOP_TRUCK_WP3_NOJUMP },
      ( Ease::None, Ease::None, Ease::None, Ease::None ),
      // (time_since_truck / truck_dur_1).min(1.0).max(0.0)
      (age - ta1) / ta2 % 10.0,
      factory.ticks,
    );
  }
  else if age < ta1 + ta2 + ta3 {
    // If maze is on, jump up to go over the path, otherwise do not jump
    paint_truck(options, state, config, context, part,
      if piped { WOOP_TRUCK_WP3_JUMP } else { WOOP_TRUCK_WP3_NOJUMP }, WOOP_TRUCK_WP4,
      ( Ease::None, Ease::None, Ease::None, Ease::None ),
      // (time_since_truck / truck_dur_1).min(1.0).max(0.0)
      (age - (ta1 + ta2)) / ta3 % 10.0,
      factory.ticks,
    );
  }
  else if age < ta1 + ta2 + ta3 + ta4 {
    // Jump back down if jumping, else just keep moving
    paint_truck(options, state, config, context, part,
      WOOP_TRUCK_WP4, WOOP_TRUCK_WP5,
      ( Ease::None, Ease::None, Ease::None, Ease::None ),
      // (time_since_truck / truck_dur_1).min(1.0).max(0.0)
      (age - (ta1 + ta2 + ta3)) / ta4 % 10.0,
      factory.ticks,
    );
  }
  else if age < ta1 + ta2 + ta3 + ta4 + ta5 {
    // Move straight up past the maze
    paint_truck(options, state, config, context, part,
      WOOP_TRUCK_WP5, WOOP_TRUCK_WP6,
      ( Ease::None, Ease::None, Ease::Out, Ease::None ),
      // (time_since_truck / truck_dur_1).min(1.0).max(0.0)
      (age - (ta1 + ta2 + ta3 + ta4)) / ta5 % 10.0,
      factory.ticks,
    );
  }
  else if age < ta1 + ta2 + ta3 + ta4 + ta5 + ta6 {
    // Now swirve into the slot

    // Get target coordinate where this part will be permanently drawn so we know where the truck has to move to
    // let ( target_x, target_y ) = if factory.trucks[t].for_woop { get_woop_xy(factory.trucks[t].target_menu_part_position) } else { get_atom_xy(factory.trucks[t].target_menu_part_position) };
    let ( target_x, target_y ) = get_woop_xy(target_menu_part_position);
    // Angle is tan(y1-y2, x1-x2) in 2pi
    let angle1 = (target_y - WOOP_TRUCK_WP6.1).atan2(target_x - WOOP_TRUCK_WP6.0); // radians (zero to std::f64::consts::TAU), 0.0 is right
    let angle = angle1 / std::f64::consts::TAU + 0.25;
    paint_truck(options, state, config, context, part,
      WOOP_TRUCK_WP6, ( target_x, target_y + CELL_H + 5.0, angle * 1.3, WOOP_TRUCK_WP6.3 ),
      ( Ease::Cubic, Ease::None, Ease::Cos, Ease::None ),
      // (time_since_truck / truck_dur_1).min(1.0).max(0.0)
      (age-(ta1 + ta2 + ta3 + ta4 + ta5))/ ta6 % 10.0,
      factory.ticks,
    );
  }
  else {
    // Truck reached its destiny.
    // - Enable the button
    // - Drop the truck
    if factory.available_woops.len() > target_menu_part_position {
      factory.available_woops[target_menu_part_position].1 = true;
    } else {
      log!("Must be testing but target_menu_part_position ({}) was oob on (factory.available_woops ({})", target_menu_part_position, factory.available_woops.len())
    }

    return true;
  }

  return false;
}

fn paint_trucks(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &mut Factory) {
  let piped = should_draw_roundway(options, factory);

  if options.dbg_loop_woop_truck {
    paint_woop_truck_at_age(options, state, config, context, factory, factory.ticks as f64 / 10.0 % 7000.0, CONFIG_NODE_PART_NONE, 5, piped);
  }
  if options.dbg_loop_atom_truck {
    paint_atom_truck_at_age(options, state, config, context, factory, factory.ticks as f64 / 10.0 % 7000.0, CONFIG_NODE_PART_NONE, 5);
  }

  let len = factory.trucks.len();

  for t in 0..len {
    if factory.trucks[t].delay > 0 {
      continue;
    }

    // TODO: Why are these trucks not released after they're done?

    let age = (factory.ticks - factory.trucks[t].created_at) as f64 / 10.0;
    let target_menu_part_position = factory.trucks[t].target_menu_part_position;
    let part = factory.trucks[t].part_kind;

    if factory.trucks[t].for_woop {
      let done = paint_woop_truck_at_age(options, state, config, context, factory, age, part, target_menu_part_position, piped);
      if done { factory.trucks[t].finished = done; }
    } else {
      let done = paint_atom_truck_at_age(options, state, config, context, factory, age, part, target_menu_part_position);
      if done { factory.trucks[t].finished = done; }
    }
  }

  for t in 0..len {
    if factory.trucks[len-t-1].finished {
      factory.trucks.remove(len-t-1);
    }
  }
}
fn paint_ui_atom_woop_hover_droptarget_hint_conditionally(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, mouse_state: &MouseState, cell_selection: &CellSelection) {
  let ( is_mouse_over_woop, woop_hover_index ) =
    if mouse_state.is_dragging || mouse_state.was_dragging { ( false, 0 ) } // Drag start is handled elsewhere, while dragging do not highlight woops
    else { ( mouse_state.woop_hover, mouse_state.woop_hover_woop_index ) };

  let ( is_mouse_over_atom, atom_hover_index ) =
    if mouse_state.is_dragging || mouse_state.was_dragging { ( false, 0 ) } // Drag start is handled elsewhere, while dragging do not highlight atoms
    else { ( mouse_state.atom_hover, mouse_state.atom_hover_atom_index ) };


  // While not dragging, paint colored overlays over machines to indicate current eligibility.
  // For example, if a part a requires part b a nd c in its pattern, mark only those machines
  // eligible who have received parts b and c already. For now, red/green will have to do, even
  // though that's not very color blind friendly. TODO: work around that.

  if mouse_state.is_dragging || (!is_mouse_over_atom && !is_mouse_over_woop && !mouse_state.atom_selected && !mouse_state.woop_selected) {
    return;
  }

  // Skip this if any machine is currently selected because that risks being destructive to the UI.
  if cell_selection.on && factory.floor[cell_selection.coord].kind == CellKind::Machine {
    return;
  }

  let hover_part_kind: PartKind =
    if is_mouse_over_atom {
      factory.available_atoms[mouse_state.atom_hover_atom_index].0
    }
    else if is_mouse_over_woop {
      factory.available_woops[mouse_state.woop_hover_woop_index].0
    }
    else if mouse_state.atom_selected {
      factory.available_atoms[mouse_state.atom_selected_index].0
    }
    else if mouse_state.woop_selected {
      factory.available_woops[mouse_state.woop_selected_index].0
    }
    else {
      panic!("should have checked that at least an atom or woop is hovered or selected");
    };

  paint_ui_hover_droptarget_hint(options, state, config, context, factory, hover_part_kind);
}
fn paint_ui_hover_droptarget_hint(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, part_kind: PartKind) {
  // Only show for atoms. We used to for woops too at some point but it's no longer relevant.
  if config.nodes[part_kind].pattern_unique_kinds.len() == 0 {
    paint_machines_droptarget_green(options, state, config, context, factory, config.nodes[part_kind].pattern_by_index.len());
  }
}
fn paint_machines_droptarget_green(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, factory: &Factory, pattern_len: usize) {
  let for_pattern = pattern_len > 0;
  // TODO: did I not have a shortcut to iterate over just machine cells?
  for coord in 0..factory.floor.len() {
    let (x, y) = to_xy(coord);
    if (!for_pattern && is_edge_not_corner(x as f64, y as f64)) || (for_pattern && factory.floor[coord].kind == CellKind::Machine) {
      let mut drop_color = get_drop_color(options, factory.ticks).to_string();
      if factory.floor[factory.floor[coord].machine.main_coord].machine.wants.len() < pattern_len {
        // Woop won't fit in this machine so omit it
        drop_color = "#ff000055".into();
      }
      context.set_fill_style(&drop_color.into());
      // context.set_fill_style(&"#00ff0077".into());
    } else {
      context.set_fill_style(&"#6b6b6b90".into());
    }
    context.fill_rect(UI_FLOOR_OFFSET_X + x as f64 * CELL_W, UI_FLOOR_OFFSET_Y + y as f64 * CELL_H, CELL_W, CELL_H);
  }
}
fn paint_green_pixel(context: &Rc<web_sys::CanvasRenderingContext2d>, progress: f64, delta: f64, x: f64, y: f64, color: &str) {
  context.set_stroke_style(&color.into());
  let border_len = UI_WOTOM_WIDTH + UI_WOTOM_HEIGHT + UI_WOTOM_WIDTH + UI_WOTOM_HEIGHT;
  let pos = (progress * border_len + delta) % border_len;
  let fx = x as f64;
  let fy = y as f64;
  if pos < UI_WOTOM_WIDTH {
    context.stroke_rect(fx + pos, fy, 1.0, 1.0);
  } else if pos < UI_WOTOM_WIDTH + UI_WOTOM_HEIGHT {
    context.stroke_rect(fx + UI_WOTOM_WIDTH, fy + (pos - UI_WOTOM_WIDTH), 1.0, 1.0);
  } else if pos < UI_WOTOM_WIDTH + UI_WOTOM_HEIGHT + UI_WOTOM_WIDTH {
    context.stroke_rect(fx + UI_WOTOM_WIDTH - (pos - (UI_WOTOM_WIDTH + UI_WOTOM_HEIGHT)), fy + UI_WOTOM_HEIGHT, 1.0, 1.0);
  } else {
    context.stroke_rect(fx, fy + UI_WOTOM_HEIGHT - (pos - (UI_WOTOM_WIDTH + UI_WOTOM_HEIGHT + UI_WOTOM_WIDTH)), 1.0, 1.0);
  }
}
fn paint_secret_menu_or_logo(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, button_canvii: &Vec<web_sys::HtmlCanvasElement>, mouse_state: &MouseState) {
  if options.dbg_show_secret_menu {
    context.set_font(&"12px monospace");
    paint_ui_buttons(options, state, context, mouse_state);
    paint_ui_buttons2(options, state, context, mouse_state);
  } else {
    paint_logo(options, state, config, context, mouse_state);
  }
}
fn paint_factory_img(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  context.set_fill_style(&"#aaa".into());
  context.fill_rect(UI_MENU_MACHINE_BUTTON_3X3_X, UI_MENU_MACHINE_BUTTON_3X3_Y, UI_MENU_MACHINE_BUTTON_3X3_WIDTH, UI_MENU_MACHINE_BUTTON_3X3_HEIGHT);

  paint_asset(options, state, config, context, CONFIG_NODE_ASSET_FACTORY, factory.ticks, UI_MENU_MACHINE_BUTTON_3X3_X, UI_MENU_MACHINE_BUTTON_3X3_Y, UI_MENU_MACHINE_BUTTON_3X3_WIDTH, UI_MENU_MACHINE_BUTTON_3X3_HEIGHT);

  context.set_stroke_style(&"black".into());
  context.stroke_rect(UI_MENU_MACHINE_BUTTON_3X3_X, UI_MENU_MACHINE_BUTTON_3X3_Y, UI_MENU_MACHINE_BUTTON_3X3_WIDTH, UI_MENU_MACHINE_BUTTON_3X3_HEIGHT);
}
fn paint_logo(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  paint_asset_raw(options, state, config, &context, CONFIG_NODE_ASSET_LOGO, 0, UI_LOGO_X, UI_LOGO_Y, UI_LOGO_W, UI_LOGO_H);
}
fn paint_ui_buttons(options: &Options, state: &State, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  // See on_up_menu for events

  paint_ui_button(context, mouse_state, 0.0, "Blow", MenuButton::Row2Button0);
  paint_ui_button(context, mouse_state, 1.0, "Unbelt", MenuButton::Row2Button1);
  paint_ui_button(context, mouse_state, 2.0, "Unpart", MenuButton::Row2Button2);
  paint_ui_button(context, mouse_state, 3.0, "Undir", MenuButton::Row2Button3);
  paint_ui_button(context, mouse_state, 4.0, "Sample", MenuButton::Row2Button4);
  assert!(UI_MENU_BUTTONS_COUNT_WIDTH_MAX == 6.0, "Update after adding new buttons");
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
  paint_ui_button2(context, mouse_state, 3.0, "Test", false, true, MenuButton::Row3Button3);
  paint_ui_button2(context, mouse_state, 4.0, "Again", false, true, MenuButton::Row3Button4);
  paint_ui_button2(context, mouse_state, 5.0, "Panic", false, true, MenuButton::Row3Button5);
  // paint_ui_button2(context, mouse_state, 6.0, "Panic");
  assert!(UI_MENU_BUTTONS_COUNT_WIDTH_MAX == 6.0, "Update after adding new buttons");
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
fn paint_ui_speed_menu(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  // paint_speed_buttons
  if options.enable_speed_menu {
    paint_ui_speed_bubble(options, state, config, factory, MenuButton::SpeedMin, context, mouse_state, BUTTON_SPEED_MIN_INDEX, "-");
    paint_ui_speed_bubble(options, state, config, factory, MenuButton::SpeedHalf, context, mouse_state, BUTTON_SPEED_HALF_INDEX, "½");
    paint_ui_speed_bubble(options, state, config, factory, MenuButton::SpeedPlayPause, context, mouse_state, BUTTON_SPEED_PLAY_PAUSE_INDEX, "⏭"); // "play" / "pause"
    paint_ui_speed_bubble(options, state, config, factory, MenuButton::SpeedDouble, context, mouse_state, BUTTON_SPEED_DOUBLE_INDEX, "2");
    paint_ui_speed_bubble(options, state, config, factory, MenuButton::SpeedPlus, context, mouse_state, BUTTON_SPEED_PLUS_INDEX, "+");
  }
}
fn paint_speed_menu_animation(options: &Options, state: &mut State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, speed_menu_prerender_canvas: &web_sys::HtmlCanvasElement) {
  if state.ui_speed_menu_anim_progress > 0 {
    let p = 1.0 - (state.ui_speed_menu_anim_progress as f64 / (options.speed_menu_animation_time as f64));
    context.draw_image_with_html_canvas_element_and_dw_and_dh(speed_menu_prerender_canvas,
      UI_MENU_MACHINE_BUTTON_3X3_X - p * (UI_MENU_MACHINE_BUTTON_3X3_X - (UI_SPEED_BUBBLE_OFFSET_X - UI_SPEED_MENU_PRERENDER_MARGIN)),
      UI_MENU_MACHINE_BUTTON_3X3_Y - p * (UI_MENU_MACHINE_BUTTON_3X3_Y - (UI_SPEED_BUBBLE_OFFSET_Y - UI_SPEED_MENU_PRERENDER_MARGIN)),
      UI_SPEED_MENU_PRERENDER_W * p,
      UI_SPEED_MENU_PRERENDER_H * p
    ).expect("draw_image_with_html_canvas_element should work"); // requires web_sys HtmlImageElement feature
  }
}
fn paint_ui_speed_bubble(options: &Options, state: &State, config: &Config, factory: &Factory, button: MenuButton, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, index: usize, text: &str) {
  // index should be one of the BUTTON_SPEED_***_INDEX constants

  let cx = UI_SPEED_BUBBLE_OFFSET_X + UI_SPEED_BUBBLE_W * (index as f64) + UI_SPEED_BUBBLE_RADIUS;
  let cy = UI_SPEED_BUBBLE_OFFSET_Y + UI_SPEED_BUBBLE_RADIUS;

  paint_ui_speed_bubble_xy(options, state, config, factory, button, context, mouse_state, text, cx, cy);
}
fn paint_ui_speed_bubble_xy(options: &Options, state: &State, config: &Config, factory: &Factory, button: MenuButton, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState, text: &str, cx: f64, cy: f64) {
  let hovering = mouse_state.over_menu_button == button;

  context.save();

  context.begin_path();
  context.arc(cx, cy, UI_SPEED_BUBBLE_RADIUS, 0.0, 2.0 * 3.14).expect("to paint"); // cx/cy must be _center_ coord of the circle, not top-left

  context.set_line_width(5.0);
  context.set_stroke_style(&BUTTON_COLOR_BORDER_DARK.into()); // border
  context.stroke();

  context.set_line_width(3.0);
  if mouse_state.over_menu_button == button {
    context.set_stroke_style(&BUTTON_COLOR_BORDER_DARK.into()); // border
  } else {
    context.set_stroke_style(&BUTTON_COLOR_BORDER_LIGHT.into()); // border
  }
  context.stroke();

  if text == "⏭" && options.speed_modifier_floor == 0.0 {
    context.set_fill_style(&"tomato".into());
  }
  else if text == "⏭" && options.speed_modifier_floor == 1.0 {
    context.set_fill_style(&BUTTON_COLOR_BORDER_LIGHT.into());
  }
  else if text == "½" && options.speed_modifier_floor == 0.5 {
    context.set_fill_style(&BUTTON_COLOR_BORDER_LIGHT.into());
  }
  else if text == "2" && options.speed_modifier_floor == 2.0 {
    context.set_fill_style(&BUTTON_COLOR_BORDER_LIGHT.into());
  }
  else if text == "-" && (options.speed_modifier_floor > 0.0 && options.speed_modifier_floor < 0.5) {
    context.set_fill_style(&BUTTON_COLOR_BORDER_LIGHT.into());
  }
  else if text == "+" && options.speed_modifier_floor > 2.0 {
    context.set_fill_style(&BUTTON_COLOR_BORDER_LIGHT.into());
  }
  else if mouse_state.over_menu_button == button {
    context.set_fill_style(&"#777".into());
  }
  else {
    context.set_fill_style(&BUTTON_COLOR_BACK.into());
  }
  context.fill();

  // Note: the grey and black versions of these icons do not work well with the hover colors
  let ( dx, dy, icon ) = match text  {
    "-" => {
      ( -9.0, -8.0, CONFIG_NODE_ASSET_FAST_BWD_WHITE )
    },
    "½" => {
      ( -10.0, -8.0, CONFIG_NODE_ASSET_BWD_WHITE )
    },
    "⏭" => {
      ( -6.0, -8.0, CONFIG_NODE_ASSET_PLAY_WHITE )
    }
    "2" => {
      ( -6.0, -8.0, CONFIG_NODE_ASSET_FWD_WHITE )
    },
    "+" => {
      ( -7.0, -8.0, CONFIG_NODE_ASSET_FAST_FWD_WHITE )
    },
    _ => panic!("update me in paint_ui_speed_bubble()..."),
  };

  paint_asset_raw(
    options, state, config, &context, icon, factory.ticks,
    cx + dx, cy + dy, 16.0, 16.0
  );

  context.restore();
}
fn paint_segment_part_from_config(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, part_kind: PartKind, dx: f64, dy: f64, dw: f64, dh: f64) -> bool {
  return paint_segment_part_from_config_bug(options, state, config, context, part_kind, dx, dy, dw, dh, false);
}
fn paint_segment_part_from_config_bug(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, part_kind: PartKind, dx: f64, dy: f64, dw: f64, dh: f64, bug: bool) -> bool {
  let dx = dx.floor();
  let dy = dy.floor();
  let dw = dw.floor();
  let dh = dh.floor();

  if part_kind == CONFIG_NODE_PART_NONE {
    return false;
  }

  assert!(config.nodes[part_kind].kind == ConfigNodeKind::Part, "segment parts should refer to part nodes but received index: {}, kind: {:?}, node: {:?}", part_kind, config.nodes[part_kind].kind, config.nodes[part_kind]);

  let (spx, spy, spw, sph, canvas) = part_to_sprite_coord_from_config(config, options, part_kind);
  if bug { log!("meh? {} {} {} {}: {:?} --> {:?}", spx, spy, spw, sph, part_kind, config.nodes[part_kind]); }

  context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
    &canvas,
    // Sprite position
    spx, spy, spw, sph,
    // Paint onto canvas at
    dx, dy, dw, dh,
  ).expect("something error draw_image"); // requires web_sys HtmlImageElement feature

  if options.dbg_paint_part_borders {
    context.set_stroke_style(&"black".into());
    context.stroke_rect(dx, dy, dw, dh);
  }
  if options.dbg_paint_part_char_icon || options.dbg_paint_part_kind_id {
    context.set_fill_style(&"#ffffff99".into());
    context.fill_rect(dx, dy, dw, dh);
    context.set_fill_style(&"black".into());
    if options.dbg_paint_part_kind_id {
      context.fill_text(part_kind.to_string().as_str(), dx + dw / 2.0 - (if part_kind < 9 { 4.0 } else { 14.0 }), dy + dh / 2.0 + 3.0).expect("to paint");
    } else if part_kind == CONFIG_NODE_PART_NONE {
      context.fill_text("ε", dx + dw / 2.0 - 4.0, dy + dh / 2.0 + 3.0).expect("to paint");
    } else {
      context.fill_text(format!("{}", config.nodes[part_kind].icon).as_str(), dx + dw / 2.0 - 4.0, dy + dh / 2.0 + 3.0).expect("to paint");
    }
  }

  return true;
}
fn paint_asset(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, config_node_index: usize, ticks: u64, dx: f64, dy: f64, dw: f64, dh: f64) -> bool {
  return paint_asset_raw(options, state, config, context, config_node_index, ticks, dx, dy, dw, dh);
}
fn paint_asset_raw(options: &Options, state: &State, config: &Config, context: &web_sys::CanvasRenderingContext2d, config_node_index: usize, ticks: u64, dx: f64, dy: f64, dw: f64, dh: f64) -> bool {
  let dx = dx.floor();
  let dy = dy.floor();
  let dw = dw.floor();
  let dh = dh.floor();

  assert!(
    config.nodes[config_node_index].kind == ConfigNodeKind::Asset ||
    config.nodes[config_node_index].kind == ConfigNodeKind::Dock ||
    config.nodes[config_node_index].kind == ConfigNodeKind::Supply ||
    config.nodes[config_node_index].kind == ConfigNodeKind::Demand
    , "assets should refer to Asset, Dock, Supply, or Demand nodes but received index: {}, kind: {:?}, node: {:?}", config_node_index, config.nodes[config_node_index].kind, config.nodes[config_node_index]);

  let (spx, spy, spw, sph, canvas) = config_get_sprite_details(config, options, config_node_index, 0, ticks);

  context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
    &canvas,
    // Sprite position
    spx, spy, spw, sph,
    // Paint onto canvas at
    dx, dy, dw, dh,
  ).expect("something error draw_image"); // requires web_sys HtmlImageElement feature

  if options.dbg_paint_part_borders {
    context.set_stroke_style(&"black".into());
    context.stroke_rect(dx, dy, dw, dh);
  }
  if options.dbg_paint_part_char_icon || options.dbg_paint_part_kind_id {
    context.set_fill_style(&"#ffffff99".into());
    context.fill_rect(dx, dy, dw, dh);
    context.set_fill_style(&"black".into());
    if options.dbg_paint_part_kind_id {
      context.fill_text(config_node_index.to_string().as_str(), dx + dw / 2.0 - (if config_node_index < 9 { 4.0 } else { 14.0 }), dy + dh / 2.0 + 3.0).expect("to paint");
    } else if config_node_index == CONFIG_NODE_PART_NONE {
      context.fill_text("ε", dx + dw / 2.0 - 4.0, dy + dh / 2.0 + 3.0).expect("to paint");
    } else {
      context.fill_text(format!("{}", config.nodes[config_node_index].icon).as_str(), dx + dw / 2.0 - 4.0, dy + dh / 2.0 + 3.0).expect("to paint");
    }
  }

  return true;
}
fn hit_test_fullscreen_button(x: f64, y: f64) -> bool {
  return bounds_check(x, y, UI_FULLSCREEN_X, UI_FULLSCREEN_Y, UI_FULLSCREEN_X + UI_FULLSCREEN_W, UI_FULLSCREEN_Y + UI_FULLSCREEN_H);
}
fn hit_test_undo(x: f64, y: f64) -> bool {
  return bounds_check(x, y, UI_UNREDO_UNDO_OFFSET_X, UI_UNREDO_UNDO_OFFSET_Y, UI_UNREDO_UNDO_OFFSET_X + UI_UNREDO_WIDTH, UI_UNREDO_UNDO_OFFSET_Y + UI_UNREDO_HEIGHT);
}
fn hit_test_redo(x: f64, y: f64) -> bool {
  return bounds_check(x, y, UI_UNREDO_REDO_OFFSET_X, UI_UNREDO_REDO_OFFSET_Y, UI_UNREDO_REDO_OFFSET_X + UI_UNREDO_WIDTH, UI_UNREDO_REDO_OFFSET_Y + UI_UNREDO_HEIGHT);
}
fn hit_test_clear(x: f64, y: f64) -> bool {
  return bounds_check(x, y, UI_UNREDO_CLEAR_OFFSET_X, UI_UNREDO_CLEAR_OFFSET_Y, UI_UNREDO_CLEAR_OFFSET_X + UI_UNREDO_WIDTH, UI_UNREDO_CLEAR_OFFSET_Y + UI_UNREDO_HEIGHT);
}
fn hit_test_paint_toggle(x: f64, y: f64) -> bool {
  return bounds_check(x, y, UI_UNREDO_PAINT_TOGGLE_X, UI_UNREDO_PAINT_TOGGLE_Y, UI_UNREDO_PAINT_TOGGLE_X + UI_UNREDO_WIDTH, UI_UNREDO_PAINT_TOGGLE_Y + UI_UNREDO_HEIGHT);
}
fn hit_test_save_map_button_index(x: f64, y: f64) -> usize {
  return
    if hit_test_save_map_row_col(x, y, 0.0, 0.0) { 0 }
    else if hit_test_save_map_row_col(x, y, 0.0, 1.0) { 1 }
    else if hit_test_save_map_row_col(x, y, 1.0, 0.0) { 2 }
    else if hit_test_save_map_row_col(x, y, 1.0, 1.0) { 3 }
    else { 100 };
}
fn hit_test_save_map_row_col(x: f64, y: f64, row: f64, col: f64) -> bool {
  // Do hit test for one of the four map thumbnail save/load buttons 
  return bounds_check(
    x, y,
    UI_SAVE_MENU_OFFSET_X + col * (UI_SAVE_THUMB_WIDTH + UI_SAVE_MARGIN), UI_SAVE_MENU_OFFSET_Y + row * (UI_SAVE_THUMB_HEIGHT + UI_SAVE_MARGIN),
    UI_SAVE_MENU_OFFSET_X + col * (UI_SAVE_THUMB_WIDTH + UI_SAVE_MARGIN) + UI_SAVE_THUMB_WIDTH, UI_SAVE_MENU_OFFSET_Y + row * (UI_SAVE_THUMB_HEIGHT + UI_SAVE_MARGIN) + UI_SAVE_THUMB_HEIGHT,
  );
}
fn hit_test_save_map_delete_part(x: f64, y: f64, row: f64, col: f64) -> bool {
  // Checks if you clicked in the right side of a map thumbnail. In that case we are going to delete the map.
  return bounds_check(
    x, y,
    UI_SAVE_MENU_OFFSET_X + col * (UI_SAVE_THUMB_WIDTH + UI_SAVE_MARGIN) + UI_SAVE_THUMB_WIDTH * 0.66, UI_SAVE_MENU_OFFSET_Y + row * (UI_SAVE_THUMB_HEIGHT + UI_SAVE_MARGIN),
    UI_SAVE_MENU_OFFSET_X + col * (UI_SAVE_THUMB_WIDTH + UI_SAVE_MARGIN) + UI_SAVE_THUMB_WIDTH, UI_SAVE_MENU_OFFSET_Y + row * (UI_SAVE_THUMB_HEIGHT + UI_SAVE_MARGIN) + UI_SAVE_THUMB_HEIGHT,
  );
}
fn hit_test_copy_button(x: f64, y: f64) -> bool {
  return bounds_check(x, y, UI_SAVE_MENU_OFFSET_X + UI_CLIPBOARD_COPY_X, UI_SAVE_MENU_OFFSET_Y + UI_CLIPBOARD_COPY_Y, UI_SAVE_MENU_OFFSET_X + UI_CLIPBOARD_COPY_X + UI_CLIPBOARD_WIDTH, UI_SAVE_MENU_OFFSET_Y + UI_CLIPBOARD_COPY_Y + UI_CLIPBOARD_HEIGHT);
}
fn hit_test_paste_button(x: f64, y: f64) -> bool {
  return bounds_check(x, y, UI_SAVE_MENU_OFFSET_X + UI_CLIPBOARD_PASTE_X, UI_SAVE_MENU_OFFSET_Y + UI_CLIPBOARD_PASTE_Y, UI_SAVE_MENU_OFFSET_X + UI_CLIPBOARD_PASTE_X + UI_CLIPBOARD_WIDTH, UI_SAVE_MENU_OFFSET_Y + UI_CLIPBOARD_PASTE_Y + UI_CLIPBOARD_HEIGHT);
}
fn paint_load_thumbs(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, button_canvii: &Vec<web_sys::HtmlCanvasElement>, mouse_state: &MouseState, quick_saves: &mut [Option<QuickSave>; 9]) {
  if !options.enable_quick_save_menu {
    return;
  }

  paint_map_save_load_button(options, state, config,factory, 0.0, 0.0, 0, context, &mut quick_saves[0], button_canvii, mouse_state, UI_SAVE_MENU_OFFSET_X, UI_SAVE_MENU_OFFSET_Y);
  paint_map_save_load_button(options, state, config,factory, 1.0, 0.0, 1, context, &mut quick_saves[1], button_canvii, mouse_state, UI_SAVE_MENU_OFFSET_X, UI_SAVE_MENU_OFFSET_Y);
  paint_map_save_load_button(options, state, config,factory, 0.0, 1.0, 2, context, &mut quick_saves[2], button_canvii, mouse_state, UI_SAVE_MENU_OFFSET_X, UI_SAVE_MENU_OFFSET_Y);
  paint_map_save_load_button(options, state, config,factory, 1.0, 1.0, 3, context, &mut quick_saves[3], button_canvii, mouse_state, UI_SAVE_MENU_OFFSET_X, UI_SAVE_MENU_OFFSET_Y);

  paint_copy_button(options, state, config, factory, context, button_canvii, mouse_state, UI_SAVE_MENU_OFFSET_X, UI_SAVE_MENU_OFFSET_Y);
  paint_paste_button(options, state, config, factory, context, button_canvii, mouse_state, UI_SAVE_MENU_OFFSET_X, UI_SAVE_MENU_OFFSET_Y);
}
fn paint_map_save_load_button(options: &Options, state: &State, config: &Config, factory: &Factory, col: f64, row: f64, button_index: usize, context: &Rc<web_sys::CanvasRenderingContext2d>, quick_save: &mut Option<QuickSave>, button_canvii: &Vec<web_sys::HtmlCanvasElement>, mouse_state: &MouseState, menu_offset_x: f64, menu_offset_y: f64) {
  assert!(button_index < 4, "there are only 4 save buttons");
  let ox = (menu_offset_x + col * (UI_SAVE_THUMB_WIDTH + UI_SAVE_MARGIN)).floor();
  let oy = (menu_offset_y + row * (UI_SAVE_THUMB_HEIGHT + UI_SAVE_MARGIN)).floor();
  if let Some(quick_save) = quick_save {
    // Save exists. Paint the thumb and then the trash icon on top of it.

    if !quick_save.loaded && quick_save.img.complete() {
      quick_save_onload(&document(), quick_save);
    }

    if quick_save.loaded {
      // The thumb has been prepared with a rounded corner. Draw it first.
      context.draw_image_with_html_canvas_element_and_dw_and_dh(
        &quick_save.thumb,
        ox, oy, UI_SAVE_THUMB_IMG_WIDTH, UI_SAVE_THUMB_IMG_HEIGHT,
      ).expect("draw_image_with_html_canvas_element_and_dw_and_dh should work"); // requires web_sys HtmlImageElement feature
    } else {
      canvas_round_rect_and_fill_stroke(context, ox, oy, UI_SAVE_THUMB_WIDTH, UI_SAVE_THUMB_HEIGHT,"#aaa", "black");
    }

    // Paint trash button

    let down = mouse_state.down_save_map && mouse_state.down_save_map_index == button_index;
    paint_button(options, state, config, context, button_canvii, if down { BUTTON_PRERENDER_INDEX_SAVE_THIN_DOWN } else { BUTTON_PRERENDER_INDEX_SAVE_THIN_UP }, ox + UI_SAVE_THUMB_IMG_WIDTH, oy);

    let hovering = mouse_state.over_save_map && mouse_state.over_save_map_index == button_index;
    let over_delete_part = mouse_state.world_x > ox + UI_SAVE_THUMB_IMG_WIDTH;

    paint_asset_raw(
      options, state, config, &context, if !hovering { CONFIG_NODE_ASSET_TRASH_LIGHT } else if over_delete_part { CONFIG_NODE_ASSET_TRASH_RED } else { CONFIG_NODE_ASSET_TRASH_GREEN }, factory.ticks,
      ox + UI_SAVE_THUMB_IMG_WIDTH + 5.0, oy + UI_SAVE_THUMB_HEIGHT / 2.0 - 8.0, 16.0, 16.0
    );
  } else {
    let down = mouse_state.down_save_map && mouse_state.down_save_map_index == button_index;

    paint_button(options, state, config, context, button_canvii, if down { BUTTON_PRERENDER_INDEX_SAVE_BIG_DOWN } else { BUTTON_PRERENDER_INDEX_SAVE_BIG_UP }, ox, oy);

    let hovering = mouse_state.over_save_map && mouse_state.over_save_map_index == button_index;

    // canvas_round_rect_and_fill_stroke(context, ox, oy, UI_SAVE_THUMB_WIDTH, UI_SAVE_THUMB_HEIGHT, fill_color, "black");
    paint_asset_raw(options, state, config, &context, if hovering { CONFIG_NODE_ASSET_SAVE_GREY } else { CONFIG_NODE_ASSET_SAVE_LIGHT }, factory.ticks,
      ox + UI_SAVE_THUMB_WIDTH * 0.35,
      oy + UI_SAVE_THUMB_HEIGHT * 0.25,
      UI_SAVE_THUMB_WIDTH / 3.0,
      UI_SAVE_THUMB_HEIGHT / 2.0
    );
  }
}
fn paint_copy_button(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, button_canvii: &Vec<web_sys::HtmlCanvasElement>, mouse_state: &MouseState, menu_offset_x: f64, menu_offset_y: f64) {
  paint_button(options, state, config, context, button_canvii, if mouse_state.down_menu_button == MenuButton::CopyFactory { BUTTON_PRERENDER_INDEX_SMALL_SQUARE_DOWN } else { BUTTON_PRERENDER_INDEX_SMALL_SQUARE_UP }, menu_offset_x + UI_CLIPBOARD_COPY_X, menu_offset_y + UI_CLIPBOARD_COPY_Y);
  paint_asset_raw(options, state, config, &context, if mouse_state.over_menu_button == MenuButton::CopyFactory { CONFIG_NODE_ASSET_COPY_GREY } else { CONFIG_NODE_ASSET_COPY_WHITE }, 0, menu_offset_x + UI_CLIPBOARD_COPY_X + UI_UNREDO_WIDTH / 2.0 - 16.0, menu_offset_y + UI_CLIPBOARD_COPY_Y + UI_UNREDO_HEIGHT / 2.0 - 16.0, 32.0, 32.0);

  let max = 40000;
  let delay = 15000;
  let since = factory.ticks - state.load_copy_hint_since;
  if state.load_copy_hint_since > 0 && since < max {
    match state.load_copy_hint_kind {
      LoadCopyHint::None => {}
      LoadCopyHint::Success => {
        let p = 1.0 - (since as f64 / (max - delay) as f64).min(1.0);
        let n = (p * 255.0) as u8;
        context.save();
        context.set_global_alpha(p);
        paint_asset_raw(
          options, state, config, &context, CONFIG_NODE_ASSET_COPY_GREEN, 0,
          menu_offset_x + UI_CLIPBOARD_COPY_X + UI_UNREDO_WIDTH / 2.0 - 16.0, menu_offset_y + UI_CLIPBOARD_COPY_Y + UI_UNREDO_HEIGHT / 2.0 - 16.0, 32.0, 32.0
        );
        context.restore();
      }
    }
  }
}
fn paint_paste_button(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, button_canvii: &Vec<web_sys::HtmlCanvasElement>, mouse_state: &MouseState, menu_offset_x: f64, menu_offset_y: f64) {
  paint_button(options, state, config, context, button_canvii, if mouse_state.down_menu_button == MenuButton::PasteFactory { BUTTON_PRERENDER_INDEX_SMALL_SQUARE_DOWN } else { BUTTON_PRERENDER_INDEX_SMALL_SQUARE_UP }, menu_offset_x + UI_CLIPBOARD_PASTE_X, menu_offset_y + UI_CLIPBOARD_PASTE_Y);
  paint_asset_raw(options, state, config, &context, if mouse_state.over_menu_button == MenuButton::PasteFactory { CONFIG_NODE_ASSET_PASTE_GREY } else { CONFIG_NODE_ASSET_PASTE_WHITE }, 0, menu_offset_x + UI_CLIPBOARD_PASTE_X + UI_UNREDO_WIDTH / 2.0 - 16.0, menu_offset_y + UI_CLIPBOARD_PASTE_Y + UI_UNREDO_HEIGHT / 2.0 - 16.0, 32.0, 32.0);

  let max = 40000;
  let delay = 15000;
  let since = factory.ticks - state.load_paste_hint_since;
  if since > 0 && since < max {
    match state.load_paste_hint_kind {
      LoadPasteHint::None => {}
      LoadPasteHint::Empty => {
        let p = if since < delay { 1.0 } else { 1.0 - (since - delay) as f64 / (max - delay) as f64 };
        let n = (p * 255.0) as u8;
        context.save();

        context.set_font(&"bold 52px Verdana");
        context.set_fill_style(&format!("#ff0000{:02x}", n).into());
        context.fill_text("!", menu_offset_x + UI_CLIPBOARD_PASTE_X + 20.0, menu_offset_y + UI_CLIPBOARD_PASTE_Y + 48.0).expect("canvas api call to work");
        context.set_stroke_style(&format!("#000000{:02x}", n).into());
        context.set_line_width(2.0);
        context.stroke_text("!", menu_offset_x + UI_CLIPBOARD_PASTE_X + 20.0, menu_offset_y + UI_CLIPBOARD_PASTE_Y + 48.0).expect("canvas api call to work");
        context.set_stroke_style(&format!("#ffffff{:02x}", n).into());
        context.set_line_width(1.0);
        context.stroke_text("!", menu_offset_x + UI_CLIPBOARD_PASTE_X + 20.0, menu_offset_y + UI_CLIPBOARD_PASTE_Y + 48.0).expect("canvas api call to work");

        context.restore();
      }
      LoadPasteHint::Invalid => {
        let p = if since < delay { 1.0 } else { 1.0 - (since - delay) as f64 / (max - delay) as f64 };
        let n = (p * 255.0) as u8;
        context.save();
        context.set_font(&"bold 52px Verdana");
        context.set_fill_style(&format!("#ff0000{:02x}", n).into());
        context.fill_text("?", menu_offset_x + UI_CLIPBOARD_PASTE_X + 16.0, menu_offset_y + UI_CLIPBOARD_PASTE_Y + 48.0).expect("canvas api call to work");
        context.set_stroke_style(&format!("#000000{:02x}", n).into());
        context.set_line_width(2.0);
        context.stroke_text("?", menu_offset_x + UI_CLIPBOARD_PASTE_X + 16.0, menu_offset_y + UI_CLIPBOARD_PASTE_Y + 48.0).expect("canvas api call to work");
        context.set_stroke_style(&format!("#ffffff{:02x}", n).into());
        context.set_line_width(1.0);
        context.stroke_text("?", menu_offset_x + UI_CLIPBOARD_PASTE_X + 16.0, menu_offset_y + UI_CLIPBOARD_PASTE_Y + 48.0).expect("canvas api call to work");
        context.restore();
      }
      LoadPasteHint::Success => {
        let p = 1.0 - (since as f64 / (max - delay) as f64).min(1.0);
        let n = (p * 255.0) as u8;
        context.save();
        context.set_global_alpha(p);
        paint_asset_raw(
          options, state, config, &context, CONFIG_NODE_ASSET_PASTE_GREEN, 0,
          menu_offset_x + UI_CLIPBOARD_PASTE_X + UI_UNREDO_WIDTH / 2.0 - 16.0, menu_offset_y + UI_CLIPBOARD_PASTE_Y + UI_UNREDO_HEIGHT / 2.0 - 16.0, 32.0, 32.0
        );
        context.restore();
      }
    }
  }
}
fn paint_text_hint(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>) {
  // Rare occasions where I want to print textual message. Like debugging on an ipad or clipboard.

  let max = 40000;
  let delay = 15000;
  let since = factory.ticks - state.hint_msg_since;
  if since > 0 && since < max {
    let p = if since < delay { 1.0 } else { 1.0 - (since - delay) as f64 / (max - delay) as f64 };
    let n = (p * 255.0) as u8;
    context.save();

    let font_size = if state.hint_msg_text.len() < 20 { 30.0 } else { 12.0 };

    context.set_font(&format!("bold {}px Verdana", font_size));
    context.set_fill_style(&format!("#000000{:02x}", n).into());
    context.fill_text(&state.hint_msg_text, UI_TEXT_HINT_OFFSET_X, UI_TEXT_HINT_OFFSET_Y + font_size).expect("canvas api call to work");
    context.set_stroke_style(&format!("#ffffff{:02x}", n).into());
    context.stroke_text(&state.hint_msg_text, UI_TEXT_HINT_OFFSET_X, UI_TEXT_HINT_OFFSET_Y + font_size).expect("canvas api call to work");

    context.restore();
  }
}
fn paint_save_menu_animation(options: &Options, state: &mut State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, save_menu_prerender_canvas: &Option<web_sys::HtmlCanvasElement>) {
  if state.ui_save_menu_anim_progress > 0 {
    // Should be prerendered now
    if let Some(save_menu_prerender_canvas) = save_menu_prerender_canvas {
      let p = 1.0 - (state.ui_save_menu_anim_progress as f64 / (options.save_menu_animation_time as f64));
      context.draw_image_with_html_canvas_element_and_dw_and_dh(save_menu_prerender_canvas,
        UI_MENU_MACHINE_BUTTON_3X3_X - p * (UI_MENU_MACHINE_BUTTON_3X3_X - UI_SAVE_MENU_OFFSET_X),
        UI_MENU_MACHINE_BUTTON_3X3_Y - p * (UI_MENU_MACHINE_BUTTON_3X3_Y - UI_SAVE_MENU_OFFSET_Y),
        UI_SAVE_MENU_W * p,
        UI_SAVE_MENU_H * p
      ).expect("draw_image_with_html_canvas_element should work"); // requires web_sys HtmlImageElement feature
    }
  }
}
fn paint_map_state_buttons(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, button_canvii: &Vec<web_sys::HtmlCanvasElement>, mouse_state: &MouseState) {
  // // Paint trash button
  context.save();
  context.set_font(&"48px monospace");

  paint_button(options, state, config, context, button_canvii, if state.snapshot_undo_pointer > 0 && mouse_state.down_menu_button == MenuButton::UndoButton { BUTTON_PRERENDER_INDEX_SMALL_SQUARE_DOWN } else { BUTTON_PRERENDER_INDEX_SMALL_SQUARE_UP }, UI_UNREDO_UNDO_OFFSET_X, UI_UNREDO_UNDO_OFFSET_Y);
  let text_color = if state.snapshot_undo_pointer <= 0 { "#777" } else if mouse_state.over_menu_button == MenuButton::UndoButton { "#aaa" } else { "#ddd" };
  // context.set_fill_style(&text_color.into());
  // context.fill_text("↶", UI_UNREDO_UNDO_OFFSET_X + UI_UNREDO_WIDTH / 2.0 - 16.0, UI_UNREDO_UNDO_OFFSET_Y + UI_UNREDO_HEIGHT / 2.0 + 16.0).expect("canvas api call to work");
  paint_asset_raw(
    options, state, config, &context, if mouse_state.over_menu_button == MenuButton::UndoButton { CONFIG_NODE_ASSET_UNDO_LIGHT } else { CONFIG_NODE_ASSET_UNDO_GREY }, 0,
    UI_UNREDO_UNDO_OFFSET_X + UI_UNREDO_WIDTH / 2.0 - 16.0, UI_UNREDO_UNDO_OFFSET_Y + UI_UNREDO_HEIGHT / 2.0 - 16.0, 32.0, 32.0
  );

  paint_button(options, state, config, context, button_canvii, if state.snapshot_undo_pointer != state.snapshot_pointer && mouse_state.down_menu_button == MenuButton::RedoButton { BUTTON_PRERENDER_INDEX_SMALL_SQUARE_DOWN } else { BUTTON_PRERENDER_INDEX_SMALL_SQUARE_UP }, UI_UNREDO_REDO_OFFSET_X, UI_UNREDO_REDO_OFFSET_Y);
  let text_color = if state.snapshot_undo_pointer == state.snapshot_pointer { "#777" } else if mouse_state.over_menu_button == MenuButton::RedoButton { "#aaa" } else { "#ddd" };
  // context.set_fill_style(&text_color.into());
  // context.fill_text("↷", UI_UNREDO_REDO_OFFSET_X + UI_UNREDO_WIDTH / 2.0 - 16.0, UI_UNREDO_REDO_OFFSET_Y + UI_UNREDO_HEIGHT / 2.0 + 16.0).expect("canvas api call to work");
  paint_asset_raw(
    options, state, config, &context, if mouse_state.over_menu_button == MenuButton::RedoButton { CONFIG_NODE_ASSET_REDO_LIGHT } else { CONFIG_NODE_ASSET_REDO_GREY }, 0,
    UI_UNREDO_REDO_OFFSET_X + UI_UNREDO_WIDTH / 2.0 - 16.0, UI_UNREDO_REDO_OFFSET_Y + UI_UNREDO_HEIGHT / 2.0 - 16.0, 32.0, 32.0
  );

  // 🗑 🚮
  paint_button(options, state, config, context, button_canvii, if mouse_state.down_menu_button == MenuButton::ClearButton { BUTTON_PRERENDER_INDEX_SMALL_SQUARE_DOWN } else { BUTTON_PRERENDER_INDEX_SMALL_SQUARE_UP }, UI_UNREDO_CLEAR_OFFSET_X, UI_UNREDO_CLEAR_OFFSET_Y);
  paint_asset_raw(
    options, state, config, &context, if mouse_state.over_menu_button == MenuButton::ClearButton { CONFIG_NODE_ASSET_TRASH_RED } else { CONFIG_NODE_ASSET_TRASH_LIGHT }, 0,
    UI_UNREDO_CLEAR_OFFSET_X + UI_UNREDO_WIDTH / 2.0 - 16.0, UI_UNREDO_CLEAR_OFFSET_Y + UI_UNREDO_HEIGHT / 2.0 - 16.0, 32.0, 32.0
  );

  paint_button(options, state, config, context, button_canvii, if state.mouse_mode_mirrored { BUTTON_PRERENDER_INDEX_MEDIUM_SQUARE_DOWN } else { BUTTON_PRERENDER_INDEX_MEDIUM_SQUARE_UP }, UI_UNREDO_PAINT_TOGGLE_X, UI_UNREDO_PAINT_TOGGLE_Y);
  // context.set_fill_style(&(if mouse_state.over_menu_button == MenuButton::PaintToggleButton { if state.mouse_mode_mirrored { "red" } else { "#aaa" } } else if state.mouse_mode_mirrored { "tomato" } else { "#ddd" }).into());
  // context.fill_text("🖌", UI_UNREDO_PAINT_TOGGLE_X + UI_UNREDO_WIDTH / 2.0 - 24.0, UI_UNREDO_PAINT_TOGGLE_Y + UI_UNREDO_HEIGHT / 2.0 + 16.0).expect("canvas api call to work");
  paint_asset_raw(
    options, state, config, &context, if mouse_state.over_menu_button == MenuButton::PaintToggleButton { CONFIG_NODE_ASSET_BRUSH_LIGHT } else if state.mouse_mode_mirrored { CONFIG_NODE_ASSET_BRUSH_RED } else { CONFIG_NODE_ASSET_BRUSH_GREEN }, 0,
    UI_UNREDO_PAINT_TOGGLE_X + UI_UNREDO_WIDTH / 2.0 - 16.0, UI_UNREDO_PAINT_TOGGLE_Y + UI_UNREDO_HEIGHT / 2.0 - 16.0, 32.0, 32.0
  );
  context.restore();
}
fn paint_fullscreen_button(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, button_canvii: &Vec<web_sys::HtmlCanvasElement>, mouse_state: &MouseState) {
  paint_button(options, state, config, context, button_canvii, if mouse_state.down_menu_button == MenuButton::FullScreenButton { BUTTON_PRERENDER_INDEX_SMALL_SQUARE_DOWN } else { BUTTON_PRERENDER_INDEX_SMALL_SQUARE_UP }, UI_FULLSCREEN_X, UI_FULLSCREEN_Y);
  let text_color = if state.snapshot_undo_pointer <= 0 { "#777" } else if mouse_state.over_menu_button == MenuButton::UndoButton { "#aaa" } else { "#ddd" };
  // context.set_fill_style(&text_color.into());
  // context.fill_text("↶", UI_UNREDO_UNDO_OFFSET_X + UI_UNREDO_WIDTH / 2.0 - 16.0, UI_UNREDO_UNDO_OFFSET_Y + UI_UNREDO_HEIGHT / 2.0 + 16.0).expect("canvas api call to work");
  paint_asset_raw(
    options, state, config, &context, if mouse_state.over_menu_button == MenuButton::FullScreenButton { CONFIG_NODE_ASSET_FULLSCREEN_GREY } else { CONFIG_NODE_ASSET_FULLSCREEN_WHITE }, 0,
    UI_FULLSCREEN_X + UI_FULLSCREEN_W / 2.0 - 16.0, UI_FULLSCREEN_Y + UI_FULLSCREEN_H / 2.0 - 16.0, 32.0, 32.0
  );
}
fn paint_maze(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, mouse_state: &MouseState) {
  if !options.enable_maze_roundway_and_collection {
    return;
  }

  let x = (GRID_X2 + GRID_RIGHT_WIDTH / 2.0 - MAZE_WIDTH / 2.0).floor() + 0.5;
  let y = (GRID_Y1 + UI_FLOOR_HEIGHT - (MAZE_HEIGHT + 10.0)).floor() + 0.5;

  let maze = &factory.maze;

  // Bar offsets above the maze:
  // Energy remaining
  let energy_x = x + 10.0;
  let energy_y = y - 30.0;
  // Speed indicator
  let speed_x = x + 100.0;
  let speed_y = y - 30.0;
  // Power indicator, paint one hammer per rock that can be broken
  let power_x = x + 140.0;
  let power_y = y - 30.0;
  // Collection / Volume indicator
  let volume_x = x + 210.0;
  let volume_y = y - 30.0;

  let max_bar_tab_count: f64 = 16.0;

  if options.enable_maze_full {
    if !options.dbg_maze_enable_runner {
      // (Runner is painted but won't move)
      context.set_fill_style(&"black".into());
      context.fill_text("Warning: options.dbg_maze_enable_runner = false", x, y + MAZE_HEIGHT + 15.0).expect("an ok");
    }

    // Paint four "current" runner bars

    context.set_fill_style(&"black".into());
    if options.dbg_maze_paint_stats_text { context.fill_text(format!("{} / {}", factory.maze_runner.energy_now, factory.maze_runner.energy_max).as_str(), energy_x, energy_y - 5.0).expect("it to work"); }
    context.set_fill_style(&"white".into());
    context.fill_rect(energy_x, energy_y, 80.0, 25.0);
    context.set_fill_style(&"yellow".into());
    context.fill_rect(energy_x, energy_y, (factory.maze_runner.energy_now as f64 / factory.maze_runner.energy_max as f64) * 80.0, 25.0);
    context.set_stroke_style(&"black".into());
    context.stroke_rect(energy_x, energy_y, 80.0, 25.0);

    context.set_fill_style(&"black".into());
    if options.dbg_maze_paint_stats_text { context.fill_text(format!("{}", factory.maze_runner.speed).as_str(), speed_x, speed_y - 5.0).expect("it to work"); }
    context.set_fill_style(&"white".into());
    context.fill_rect(speed_x, speed_y, 30.0, 25.0);
    context.set_stroke_style(&"black".into());
    context.stroke_rect(speed_x, speed_y, 30.0, 25.0);
    context.set_fill_style(&"black".into());
    context.fill_text(&format!("{}", factory.maze_runner.speed), speed_x + if factory.maze_runner.speed >= 100 { 4.0 } else if factory.maze_runner.speed >= 10 { 7.0 } else { 11.0 }, speed_y + 16.0).expect("it to work");

    context.set_fill_style(&"black".into());
    if options.dbg_maze_paint_stats_text { context.fill_text(format!("{} / {}", factory.maze_runner.power_now, factory.maze_runner.power_max).as_str(), power_x, power_y - 5.0).expect("it to work"); }
    context.set_fill_style(&"white".into());
    context.fill_rect(power_x, power_y, 60.0, 25.0);
    // paint one hammer evenly divided across the space. maintain location (dont "jump" when removing a hammer). max width to divide is field_width-margin-img_width-margin. max spacing is img_width+margin
    let max_power_space = 60.0 - 5.0 - 11.0;
    let power_offset_step = (max_power_space / (factory.maze_runner.power_max.min(MAX_ROCK_COUNT as u64) as f64)).min(40.0);
    for i in 0..factory.maze_runner.power_now.min(MAX_ROCK_COUNT as u64) {
      // will overlap. by design
      paint_asset(&options, &state, &config, &context, CONFIG_NODE_ASSET_PICKAXE, factory.ticks, power_x + 3.0 + (i as f64) * power_offset_step, power_y + 5.0, 15.0, 15.0);
    }
    context.set_stroke_style(&"black".into());
    context.stroke_rect(power_x, power_y, 60.0, 25.0);

    context.set_fill_style(&"black".into());
    if options.dbg_maze_paint_stats_text {
      context.fill_text(format!("{} / {}", factory.maze_runner.volume_now, factory.maze_runner.volume_max).as_str(), volume_x, volume_y - 5.0).expect("it to work");
    }
    context.set_fill_style(&"white".into());
    context.fill_rect(volume_x, volume_y, 80.0, 25.0);
    // Unlike power, here we may actually want to update the spacing as the volume goes up. Try to make it a "pile" or whatever. We can probably fake that to some degree with some kind of pre-defined positioning table etc.
    let max_volume_space = 60.0 - 5.0;
    let volume_offset_step = (max_volume_space / (factory.maze_runner.volume_max.min(40) as f64)).min(20.0);
    for i in 0..factory.maze_runner.volume_now.min(40) {
      // will overlap. by design
      paint_asset(&options, &state, &config, &context, CONFIG_NDOE_ASSET_TREASURE, factory.ticks, volume_x + 5.0 + (i as f64) * volume_offset_step, volume_y + 5.0, 15.0, 15.0);
    }
    context.set_stroke_style(&"black".into());
    context.stroke_rect(volume_x, volume_y, 80.0, 25.0);

    // Actual maze next...

    context.set_fill_style(&"white".into());
    context.fill_rect(x, y, MAZE_WIDTH, MAZE_HEIGHT);
    context.set_stroke_style(&"black".into());
    context.stroke_rect(x, y, MAZE_WIDTH, MAZE_HEIGHT);

    // maze visited fill
    for i in 0..MAZE_CELLS_W as usize {
      for j in 0..MAZE_CELLS_H as usize {
        let v = &maze[j* MAZE_CELLS_W +i];
        if v.state > 0 {
          context.set_fill_style(&format!("#ff{:02x}{:02x}", 255-v.state, 255-v.state).into());
          // context.set_fill_style(&format!("#ff0000").into());
          context.fill_rect(x + (i as f64) * MAZE_CELL_SIZE, y + (j as f64) * MAZE_CELL_SIZE, MAZE_CELL_SIZE, MAZE_CELL_SIZE);
        }
      }
    }

    for i in 0..MAZE_CELLS_W as usize {
      for j in 0..MAZE_CELLS_H as usize {
        let v = &maze[j* MAZE_CELLS_W +i];
        if !v.up {
          // can not go up, draw top-border
          context.begin_path();
          context.move_to(x + ((i) as f64) * MAZE_CELL_SIZE, y + ((j) as f64) * MAZE_CELL_SIZE);
          context.line_to(x + ((i+1) as f64) * MAZE_CELL_SIZE, y + ((j) as f64) * MAZE_CELL_SIZE);
          context.stroke();
        }
        if !v.left {
          // can not go left, draw left-border
          context.begin_path();
          context.move_to(x + ((i) as f64) * MAZE_CELL_SIZE, y + ((j) as f64) * MAZE_CELL_SIZE);
          context.line_to(x + ((i) as f64) * MAZE_CELL_SIZE, y + ((j+1) as f64) * MAZE_CELL_SIZE);
          context.stroke();
        }

        match v.special {
          // MAZE_TREASURE
          2 => {
            context.set_fill_style(&"#d4662f".into());
            context.fill_rect(x + i as f64 * MAZE_CELL_SIZE + 2.0, y + j as f64 * MAZE_CELL_SIZE + 2.0, MAZE_CELL_SIZE - 4.0, 6.0);
          }
          // MAZE_ROCK
          1 => {
            context.set_fill_style(&"black".into());
            context.fill_rect(x + i as f64 * MAZE_CELL_SIZE + 2.0, y + j as f64 * MAZE_CELL_SIZE + 2.0, MAZE_CELL_SIZE - 4.0, 6.0);
          }
          _ => {}
        }
      }
    }

    context.set_fill_style(&"blue".into());
    context.fill_rect(x + (factory.maze_runner.x as f64) * MAZE_CELL_SIZE + 2.0, y + (factory.maze_runner.y as f64) * MAZE_CELL_SIZE + 2.0, MAZE_CELL_SIZE - 4.0, 6.0);
  }

  // Stats bars below the maze

  let max_bar_tabs = (MAZE_MAX_UNITS_PER_REFUEL as f64) + 1.0; // First one is always filled and is more of a label
  let bar_width = MAZE_WIDTH / max_bar_tabs;
  let bar_height = 25.0;
  let fuel_width = 10.0;
  let fuel_height = 10.0;

  let ( e, s, p, v) = factory.maze_prep;
  let (flying_e, flying_s, flying_p, flying_v) = factory.fuel_in_flight;

  let delta = -6.0;

  let refuel_max_time = maze_get_refuel_time(options) as f64;
  let fueling_time = (factory.ticks - factory.maze_runner.maze_refueling_at) as f64;
  let fuel_progress1: f64 = if fueling_time < refuel_max_time { fueling_time as f64 / refuel_max_time } else { 1.0 };
  // Wait 10% of the time at the start before moving
  let fuel_progress = if fuel_progress1 < MAZE_REFUEL_PORTION { 0.0 } else { (fuel_progress1 - MAZE_REFUEL_PORTION) / (1.0 - MAZE_REFUEL_PORTION) };

  context.set_fill_style(&"black".into());
  if options.dbg_maze_paint_stats_text {
    context.fill_text(format!(
      "{}  {}  {}  {} :: {}% {}% :: fin {} re {} fuel {}",
      e, s, p , v,
      (fuel_progress1 * 100.0).floor(),
      (fuel_progress * 100.0).floor(),
      (maze_get_finish_pause_time(options) as i64 - (factory.ticks as i64 - factory.maze_runner.maze_finish_at as i64)).max(0),
      (maze_get_finish_pause_time(options) as i64 - (factory.ticks as i64 - factory.maze_runner.maze_restart_at as i64)).max(0),
      (maze_get_refuel_time(options) as i64- (factory.ticks as i64 - factory.maze_runner.maze_refueling_at as i64)).max(0),
    ).as_str(), 0.5 + x + 5.0 - 100.0, 0.5 + GRID_Y2 + (bar_height * 4.0 + 30.0)).expect("canvas api call to work");
  }

  // e = green
  let have_e = (e as f64/10.0).floor().min(max_bar_tab_count - 1.0); // first one is always filled
  let filled_tabs_e = bar_width * (1.0 + have_e);
  let semi_filled_tabs_e = if filled_tabs_e >= max_bar_tab_count * bar_width { 0.0 } else { (bar_width * ((e as f64 % 10.0) / 10.0)).floor() };
  let bar_e_offset_y = 0.5 + GRID_Y2 + delta;
  context.set_fill_style(&"white".into());
  context.fill_rect(0.5 + x, bar_e_offset_y, MAZE_WIDTH, bar_height);
  context.set_fill_style(&"#169d06".into());
  context.fill_rect(0.5 + x, bar_e_offset_y, filled_tabs_e, bar_height);
  context.set_fill_style(&"#169d0655".into());
  context.fill_rect(0.5 + x + filled_tabs_e, bar_e_offset_y, semi_filled_tabs_e, bar_height);
  context.set_stroke_style(&"black".into());
  context.stroke_rect(0.5 + x, bar_e_offset_y, MAZE_WIDTH, bar_height);
  context.set_fill_style(&"white".into());
  context.fill_text("E", 0.5 + x + 5.0, bar_e_offset_y + 17.0).expect("canvas api call to work");
  for i in 0..(max_bar_tabs as usize) {
    context.begin_path();
    context.move_to(0.5 + x + (bar_width * (i as f64)), bar_e_offset_y);
    context.line_to(0.5 + x + (bar_width * (i as f64)), bar_e_offset_y + bar_height);
    context.stroke();
  }

  // s = orange
  let have_s = (s as f64/10.0).floor().min(max_bar_tab_count - 1.0); // first one is always filled
  let filled_tabs_s = bar_width * (1.0 + have_s);
  let semi_filled_tabs_s = if filled_tabs_s >= max_bar_tab_count * bar_width { 0.0 } else { (bar_width * ((s as f64 % 10.0) / 10.0)).floor() };
  let bar_s_offset_y = 0.5 + GRID_Y2 + 32.0 + delta;
  context.set_fill_style(&"white".into());
  context.fill_rect(0.5 + x, bar_s_offset_y, MAZE_WIDTH, bar_height);
  context.set_fill_style(&"#a86007".into());
  context.fill_rect(0.5 + x, bar_s_offset_y, filled_tabs_s, bar_height);
  context.set_fill_style(&"#a8600777".into());
  context.fill_rect(0.5 + x + filled_tabs_s, bar_s_offset_y, semi_filled_tabs_s, bar_height);
  context.set_stroke_style(&"black".into());
  context.stroke_rect(0.5 + x, bar_s_offset_y, MAZE_WIDTH, bar_height);
  context.set_fill_style(&"white".into());
  context.fill_text("S", 0.5 + x + 5.0, bar_s_offset_y + 17.0).expect("canvas api call to work");
  for i in 0..(max_bar_tabs as usize) {
    context.begin_path();
    context.move_to(0.5 + x + (bar_width * (i as f64)), bar_s_offset_y);
    context.line_to(0.5 + x + (bar_width * (i as f64)), bar_s_offset_y + bar_height);
    context.stroke();
  }

  // p = pink
  let have_p = (p as f64/10.0).floor().min(max_bar_tab_count - 1.0); // first one is always filled
  let filled_tabs_p = bar_width * (1.0 + have_p);
  let semi_filled_tabs_p = if filled_tabs_p >= max_bar_tab_count * bar_width { 0.0 } else { (bar_width * ((p as f64 % 10.0) / 10.0)).floor() };
  let bar_p_offset_y = 0.5 + GRID_Y2 + 64.0 + delta;
  context.set_fill_style(&"white".into());
  context.fill_rect(0.5 + x, bar_p_offset_y, MAZE_WIDTH, bar_height);
  context.set_fill_style(&"#ef13bf".into());
  context.fill_rect(0.5 + x, bar_p_offset_y, filled_tabs_p, bar_height);
  context.set_fill_style(&"#ef13bf77".into());
  context.fill_rect(0.5 + x + filled_tabs_p, bar_p_offset_y, semi_filled_tabs_p, bar_height);
  context.set_stroke_style(&"black".into());
  context.stroke_rect(0.5 + x, bar_p_offset_y, MAZE_WIDTH, bar_height);
  context.set_fill_style(&"white".into());
  context.fill_text("P", 0.5 + x + 5.0, bar_p_offset_y + 17.0).expect("canvas api call to work");
  for i in 0..(max_bar_tabs as usize) {
    context.begin_path();
    context.move_to(0.5 + x + (bar_width * (i as f64)), bar_p_offset_y);
    context.line_to(0.5 + x + (bar_width * (i as f64)), bar_p_offset_y + bar_height);
    context.stroke();
  }

  // v = purple
  let have_v = (v as f64/10.0).floor().min(max_bar_tab_count - 1.0); // first one is always filled
  let filled_tabs_v = bar_width * (1.0 + have_v);
  let semi_filled_tabs_v = if filled_tabs_v >= max_bar_tab_count * bar_width { 0.0 } else { (bar_width * ((v as f64 % 10.0) / 10.0)).floor() };
  let bar_v_offset_y = 0.5 + GRID_Y2 + 96.0 + delta;
  context.set_fill_style(&"white".into());
  context.fill_rect(0.5 + x, bar_v_offset_y, MAZE_WIDTH, bar_height);
  context.set_fill_style(&"#360676".into());
  context.fill_rect(0.5 + x, bar_v_offset_y, filled_tabs_v, bar_height);
  context.set_fill_style(&"#36067677".into());
  context.fill_rect(0.5 + x + filled_tabs_v, bar_v_offset_y, semi_filled_tabs_v, bar_height);
  context.set_stroke_style(&"black".into());
  context.stroke_rect(0.5 + x, bar_v_offset_y, MAZE_WIDTH, bar_height);
  context.set_fill_style(&"white".into());
  context.fill_text("V", 0.5 + x + 5.0, bar_v_offset_y + 17.0).expect("canvas api call to work");
  for i in 0..(max_bar_tabs as usize) {
    context.begin_path();
    context.move_to(0.5 + x + (bar_width * (i as f64)), bar_v_offset_y);
    context.line_to(0.5 + x + (bar_width * (i as f64)), bar_v_offset_y + bar_height);
    context.stroke();
  }

  // Squares in flight
  context.set_fill_style(&"#169d06".into());
  context.set_stroke_style(&"black".into());
  for i in 0..flying_e {
    let ox = 0.5 + x + bar_width + (bar_width * (i as f64)) + 4.0;
    let oy = bar_e_offset_y + 8.0;
    let cx = ox + (energy_x + 6.0 - ox) * fuel_progress;
    let cy = oy + (energy_y + 6.0 - oy) * fuel_progress;
    context.fill_rect(cx, cy, fuel_width, fuel_height);
    context.stroke_rect(cx, cy, fuel_width, fuel_height);
  }
  context.set_fill_style(&"#a86007".into());
  context.set_stroke_style(&"black".into());
  for i in 0..flying_s {
    let ox = 0.5 + x + bar_width + (bar_width * (i as f64)) + 4.0;
    let oy = bar_s_offset_y + 8.0;
    let cx = ox + (speed_x + 6.0 - ox) * fuel_progress;
    let cy = oy + (speed_y + 6.0 - oy) * fuel_progress;
    context.fill_rect(cx, cy, fuel_width, fuel_height);
    context.stroke_rect(cx, cy, fuel_width, fuel_height);
  }
  context.set_fill_style(&"#ef13bf".into());
  context.set_stroke_style(&"black".into());
  for i in 0..flying_p {
    let ox = 0.5 + x + bar_width + (bar_width * (i as f64)) + 4.0;
    let oy = bar_p_offset_y + 8.0;
    let cx = ox + (power_x + 6.0 - ox) * fuel_progress;
    let cy = oy + (power_y + 6.0 - oy) * fuel_progress;
    context.fill_rect(cx, cy, fuel_width, fuel_height);
    context.stroke_rect(cx, cy, fuel_width, fuel_height);
  }
  context.set_fill_style(&"#360676".into());
  context.set_stroke_style(&"black".into());
  for i in 0..flying_v {
      let ox = 0.5 + x + bar_width + (bar_width * (i as f64)) + 4.0;
      let oy = bar_v_offset_y + 8.0;
      let cx = ox + (volume_x + 6.0 - ox) * fuel_progress;
      let cy = oy + (volume_y + 6.0 - oy) * fuel_progress;
      context.fill_rect(cx, cy, fuel_width, fuel_height);
      context.stroke_rect(cx, cy, fuel_width, fuel_height);
    }
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

  let dx = dx.floor();
  let dy = dy.floor();
  let dw = dw.floor();
  let dh = dh.floor();

  let (spx, spy, spw, sph, canvas) = config_get_sprite_for_belt_type(config, options, belt_type, sprite_start_at, ticks);

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
  let supply_config_node =
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

  paint_asset(&options, &state, &config, &context, supply_config_node, factory.ticks - sprite_start_at, dx, dy, dw, dh);
}
fn paint_demander(options: &Options, state: &State, config: &Config, factory: &Factory, context: &Rc<web_sys::CanvasRenderingContext2d>, dx: f64, dy: f64, dw: f64, dh: f64, last_part_at: u64, ticks: u64, coord: usize) {
  let (x, y) = to_xy(coord);
  let demand_config_node =
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

  paint_asset(&options, &state, &config, &context, demand_config_node, factory.ticks - last_part_at, dx, dy, dw, dh);
}

fn get_drop_color(options: &Options, ticks: u64) -> String {
  let color_offset = options.dropzone_color_offset; // 75
  let bounce_speed = options.dropzone_bounce_speed; // 100
  let bounce_distance = options.dropzone_bounce_distance; // 150
  let bounce_d2 = bounce_distance * 2;
  let mut p = ((((ticks as f64) / options.speed_modifier_ui) as u64) / bounce_speed) % bounce_d2;
  if p > bounce_distance { p = bounce_distance - (p - bounce_distance); } // This makes the color bounce rather than jump from black to green
  let yo = format!("#00{:02x}0077", p+ color_offset);
  return yo;
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
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
    // // Convert to a HtmlDocument, which is different (richer) from Document. Requires HtmlDocument feature in cargo.toml
    // .dyn_into::<web_sys::HtmlDocument>().unwrap()
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
  log!("parse_and_save_options_string(options.dbg_dump_options_string = {}) {} (len = {})", options.dbg_dump_options_string, if options_started_from_source > 0 { "from source" } else { "compiled defaults" }, option_string.len());
  let bak = options.initial_map_from_source;
  parse_options_into(option_string.clone(), options, true);
  options.options_started_from_source = options_started_from_source; // This prop will be overwritten by the above, first
  options.initial_map_from_source = bak; // Do not overwrite this.
  let exp = options_serialize(options);

  if options.dbg_dump_options_string { log!("{}", option_string); } // Default is on but localStorage could turn this off

  let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
  if !on_load {
    log!("parse_and_save_options_string: Storing options into browser localStorage... ({} bytes)", exp.len());
    local_storage.set_item(LS_OPTIONS, exp.as_str()).unwrap();
  }

  // Update UI to reflect actually loaded options
  log!("setGameOptions() (After loading them from localStorage)");
  setGameOptions(exp.into(), on_load.into());
}

fn prerender_button(options: &Options, state: &State, config: &Config, width: f64, height: f64, button_style_up: bool) -> web_sys::HtmlCanvasElement {
  let document = document();
  let canvas = document.create_element("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
  canvas.set_width(width as u32);
  canvas.set_height(height as u32);
  canvas.style().set_property("image-rendering", "pixelated").expect("should work");
  let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
  context.set_image_smoothing_enabled(false);

  // color: #392946
  context.clear_rect(0.0, 0.0, width, height); // make the whole thing semi trans first
  context.set_fill_style(&"#392946".into());
  context.fill_rect(4.0, 4.0, width - 8.0, height - 8.0);

  prerender_button_stage2(options, state, config, width, height, &context, button_style_up);

  return canvas;
}
fn prerender_button_stage2(options: &Options, state: &State, config: &Config, width: f64, height: f64, context: &web_sys::CanvasRenderingContext2d, button_style_up: bool) {
  const ZOOM: f64 = 2.0; // how big should the fixed corners be?

  paint_asset_raw(options, state, config, &context, if button_style_up { CONFIG_NODE_ASSET_BUTTON_UP_1 } else { CONFIG_NODE_ASSET_BUTTON_DOWN_1 }, 0, 0.0, 0.0, 14.0 * ZOOM, 10.0 * ZOOM);
  paint_asset_raw(options, state, config, &context, if button_style_up { CONFIG_NODE_ASSET_BUTTON_UP_3 } else { CONFIG_NODE_ASSET_BUTTON_DOWN_3 }, 0, width - 14.0 * ZOOM, 0.0, 14.0 * ZOOM, 10.0 * ZOOM);
  paint_asset_raw(options, state, config, &context, if button_style_up { CONFIG_NODE_ASSET_BUTTON_UP_7 } else { CONFIG_NODE_ASSET_BUTTON_DOWN_7 }, 0, 0.0, height - 11.0 * ZOOM, 14.0 * ZOOM, 11.0 * ZOOM);
  paint_asset_raw(options, state, config, &context, if button_style_up { CONFIG_NODE_ASSET_BUTTON_UP_9 } else { CONFIG_NODE_ASSET_BUTTON_DOWN_9 }, 0, width - 14.0 * ZOOM, height - 11.0 * ZOOM, 14.0 * ZOOM, 11.0 * ZOOM);

  for i in 0..(width - 28.0 * ZOOM).max(0.0) as u16 {
    paint_asset_raw(options, state, config, &context, if button_style_up { CONFIG_NODE_ASSET_BUTTON_UP_2 } else { CONFIG_NODE_ASSET_BUTTON_DOWN_2 }, 0, 14.0 * ZOOM + (i as f64), 0.0,           1.0, 10.0 * ZOOM);
    paint_asset_raw(options, state, config, &context, if button_style_up { CONFIG_NODE_ASSET_BUTTON_UP_8 } else { CONFIG_NODE_ASSET_BUTTON_DOWN_8 }, 0, 14.0 * ZOOM + (i as f64), height - 11.0 * ZOOM, 1.0, 11.0 * ZOOM);
  }
  for i in 0..(height - 21.0 * ZOOM).max(0.0) as u16 {
    paint_asset_raw(options, state, config, &context, if button_style_up { CONFIG_NODE_ASSET_BUTTON_UP_4 } else { CONFIG_NODE_ASSET_BUTTON_DOWN_4 }, 0, 0.0,          10.0 * ZOOM + (i as f64), 14.0 * ZOOM, 1.0);
    paint_asset_raw(options, state, config, &context, if button_style_up { CONFIG_NODE_ASSET_BUTTON_UP_6 } else { CONFIG_NODE_ASSET_BUTTON_DOWN_6 }, 0, width - 14.0 * ZOOM, 10.0 * ZOOM + (i as f64), 14.0 * ZOOM, 1.0);
  }
}
fn paint_button(options: &Options, state: &State, config: &Config, context: &Rc<web_sys::CanvasRenderingContext2d>, button_canvii: &Vec<web_sys::HtmlCanvasElement>, button_canvii_index: usize, x: f64, y: f64) {
  context.draw_image_with_html_canvas_element(&button_canvii[button_canvii_index], x.floor(), y.floor()).expect("draw_image_with_html_canvas_element should work"); // requires web_sys HtmlImageElement feature
}
fn prerender_speed_menu(options: &Options, state: &State, config: &Config, factory: &Factory, mouse_state: &MouseState) -> web_sys::HtmlCanvasElement {
  let cy = UI_SPEED_MENU_PRERENDER_MARGIN + UI_SPEED_BUBBLE_RADIUS;

  let document = document();
  let canvas = document.create_element("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
  canvas.set_width(UI_SPEED_MENU_PRERENDER_W as u32);
  canvas.set_height(UI_SPEED_MENU_PRERENDER_H as u32);
  canvas.style().set_property("image-rendering", "pixelated").expect("should work");
  let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
  context.set_image_smoothing_enabled(false);

  let context = Rc::new(context); // For the sake of the function arg. Not important for this one-of.

  paint_ui_speed_bubble_xy(options, state, config, factory, MenuButton::SpeedMin, &context, mouse_state, "-", UI_SPEED_MENU_PRERENDER_MARGIN + UI_SPEED_BUBBLE_W * (BUTTON_SPEED_MIN_INDEX as f64) + UI_SPEED_BUBBLE_RADIUS, cy);
  paint_ui_speed_bubble_xy(options, state, config, factory, MenuButton::SpeedHalf, &context, mouse_state, "½", UI_SPEED_MENU_PRERENDER_MARGIN + UI_SPEED_BUBBLE_W * (BUTTON_SPEED_HALF_INDEX as f64) + UI_SPEED_BUBBLE_RADIUS, cy);
  paint_ui_speed_bubble_xy(options, state, config, factory, MenuButton::SpeedPlayPause, &context, mouse_state, "⏭", UI_SPEED_MENU_PRERENDER_MARGIN + UI_SPEED_BUBBLE_W * (BUTTON_SPEED_PLAY_PAUSE_INDEX as f64) + UI_SPEED_BUBBLE_RADIUS, cy); // "play" / "pause"
  paint_ui_speed_bubble_xy(options, state, config, factory, MenuButton::SpeedDouble, &context, mouse_state, "2", UI_SPEED_MENU_PRERENDER_MARGIN + UI_SPEED_BUBBLE_W * (BUTTON_SPEED_DOUBLE_INDEX as f64) + UI_SPEED_BUBBLE_RADIUS, cy);
  paint_ui_speed_bubble_xy(options, state, config, factory, MenuButton::SpeedPlus, &context, mouse_state, "+", UI_SPEED_MENU_PRERENDER_MARGIN + UI_SPEED_BUBBLE_W * (BUTTON_SPEED_PLUS_INDEX as f64) + UI_SPEED_BUBBLE_RADIUS, cy);

  return canvas;
}
fn prerender_save_menu(options: &Options, state: &State, config: &Config, factory: &Factory, mouse_state: &MouseState, quick_saves: &mut [Option<QuickSave>; 9], button_canvii: &Vec<web_sys::HtmlCanvasElement>) -> web_sys::HtmlCanvasElement {
  let document = document();
  let canvas = document.create_element("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
  canvas.set_width(UI_SAVE_MENU_W as u32);
  canvas.set_height(UI_SAVE_MENU_H as u32);
  canvas.style().set_property("image-rendering", "pixelated").expect("should work");
  let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
  context.set_image_smoothing_enabled(false);

  let context = Rc::new(context); // For the sake of the function arg. Not important for this one-of.

  paint_map_save_load_button(options, state, config, factory, 0.0, 0.0, 0, &context, &mut quick_saves[0], button_canvii, mouse_state, 0.0, 0.0);
  paint_map_save_load_button(options, state, config, factory, 1.0, 0.0, 1, &context, &mut quick_saves[1], button_canvii, mouse_state, 0.0, 0.0);
  paint_map_save_load_button(options, state, config, factory, 0.0, 1.0, 2, &context, &mut quick_saves[2], button_canvii, mouse_state, 0.0, 0.0);
  paint_map_save_load_button(options, state, config, factory, 1.0, 1.0, 3, &context, &mut quick_saves[3], button_canvii, mouse_state, 0.0, 0.0);

  paint_copy_button(options, state, config, factory, &context, button_canvii, mouse_state, 0.0, 0.0);
  paint_paste_button(options, state, config, factory, &context, button_canvii, mouse_state, 0.0, 0.0);

  return canvas;
}
