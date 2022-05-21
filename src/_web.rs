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
use crate::segment::SegmentDirection;

use super::belt::*;
use super::cell::*;
use super::factory::*;
use super::floor::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::state::*;

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

fn log(s: &str) {
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
  canvas.set_width(1000);
  canvas.set_height(1000);
  canvas.style().set_property("border", "solid")?;
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
  belt_tile_images[BeltType::D_UL as usize] = load_tile(CELL_BELT_D_UL.src)?;
  belt_tile_images[BeltType::U_DLR as usize] = load_tile(CELL_BELT_U_DLR.src)?;
  belt_tile_images[BeltType::R_DLU as usize] = load_tile(CELL_BELT_R_DLU.src)?;
  belt_tile_images[BeltType::D_LRU as usize] = load_tile(CELL_BELT_D_LRU.src)?;
  belt_tile_images[BeltType::L_DRU as usize] = load_tile(CELL_BELT_L_DRU.src)?;
  belt_tile_images[BeltType::RU_DL as usize] = load_tile(CELL_BELT_RU_DL.src)?;
  belt_tile_images[BeltType::DU_LR as usize] = load_tile(CELL_BELT_DU_LR.src)?;
  belt_tile_images[BeltType::LU_DR as usize] = load_tile(CELL_BELT_LU_DR.src)?;
  belt_tile_images[BeltType::LD_RU as usize] = load_tile(CELL_BELT_LD_RU.src)?;
  belt_tile_images[BeltType::DR_LU as usize] = load_tile(CELL_BELT_DR_LU.src)?;
  belt_tile_images[BeltType::LR_DU as usize] = load_tile(CELL_BELT_LR_DU.src)?;
  belt_tile_images[BeltType::DLR_U as usize] = load_tile(CELL_BELT_DLR_U.src)?;
  belt_tile_images[BeltType::DLU_R as usize] = load_tile(CELL_BELT_DLU_R.src)?;
  belt_tile_images[BeltType::RLU_D as usize] = load_tile(CELL_BELT_RLU_D.src)?;
  belt_tile_images[BeltType::DRU_L as usize] = load_tile(CELL_BELT_DRU_L.src)?;

  let part_tile_sprite: web_sys::HtmlImageElement = load_tile("./img/roguelikeitems.png")?;

  let img_machine1 = load_tile("./img/machine1.png")?;
  let img_machine2 = load_tile("./img/machine2.png")?;
  let img_machine3 = load_tile("./img/machine3.png")?;

  // let pressed = Rc::new(Cell::new(false));
  // mousedown
  // {
  //   let context = context.clone();
  //   let pressed = pressed.clone();
  //   let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
  //     log("mousedown?");
  //     context.begin_path();
  //     context.move_to(event.offset_x() as f64, event.offset_y() as f64);
  //     pressed.set(true);
  //   }) as Box<dyn FnMut(_)>);
  //   canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
  //   closure.forget();
  // }
  // mousemove
  // {
  //   let context = context.clone();
  //   let pressed = pressed.clone();
  //   let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
  //     if pressed.get() {
  //       log("mousedrag?");
  //       context.line_to(event.offset_x() as f64, event.offset_y() as f64);
  //       context.stroke();
  //       context.begin_path();
  //       context.move_to(event.offset_x() as f64, event.offset_y() as f64);
  //     }
  //   }) as Box<dyn FnMut(_)>);
  //   canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
  //   closure.forget();
  // }
  // mouseup
  // {
  //   let context = context.clone();
  //   let pressed = pressed.clone();
  //   let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
  //     log("mouseup?");
  //     pressed.set(false);
  //     context.line_to(event.offset_x() as f64, event.offset_y() as f64);
  //     context.stroke();
  //   }) as Box<dyn FnMut(_)>);
  //   canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
  //   closure.forget();
  // }


  // Static state configuration (can still be changed by user)
  let mut options = create_options(1.0);

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

      context.set_font(&"12px monospace");

      let box_x = 800.5;
      let box_y = 10.5;
      let box_w = 190.0;
      let box_h = 25.0;
      let font_h = 16.0;
      let m_l = 6.0;

      context.set_fill_style(&"lightgreen".into());
      context.fill_rect(box_x, box_y + (box_h * 0.0), box_w, box_h);
      context.set_fill_style(&"grey".into());
      context.fill_text(format!("fps: {}", fps.len()).as_str(), box_x + m_l, box_y + (0.0 * box_h) + font_h).expect("something error fill_text");

      context.set_fill_style(&"lightgreen".into());
      context.fill_rect(box_x, box_y + (box_h * 1.0), box_w, box_h);
      context.set_fill_style(&"grey".into());
      context.fill_text(format!("App time  : {}", now / 1000.0).as_str(), box_x + m_l, box_y + (1.0 * box_h) + font_h).expect("something error fill_text");

      context.set_fill_style(&"lightgreen".into());
      context.fill_rect(box_x, box_y + (box_h * 2.0), box_w, box_h);
      context.set_fill_style(&"grey".into());
      context.fill_text(format!("Since prev: {}", since_prev).as_str(), box_x + m_l, box_y + (2.0 * box_h) + font_h).expect("something error fill_text");

      context.set_fill_style(&"lightgreen".into());
      context.fill_rect(box_x, box_y + (box_h * 3.0), box_w, box_h);
      context.set_fill_style(&"grey".into());
      context.fill_text(format!("Ticks todo: {}", ticks_todo).as_str(), box_x + m_l, box_y + (3.0 * box_h) + font_h).expect("something error fill_text");

      context.set_fill_style(&"lightgreen".into());
      context.fill_rect(box_x, box_y + (box_h * 5.0), box_w, box_h);
      context.set_fill_style(&"grey".into());
      context.fill_text(format!("$ /  1s    : {}", factory.stats.2).as_str(), box_x + m_l, box_y + (5.0 * box_h) + font_h).expect("something error fill_text");

      context.set_fill_style(&"lightgreen".into());
      context.fill_rect(box_x, box_y + (box_h * 6.0), box_w, box_h);
      context.set_fill_style(&"grey".into());
      context.fill_text(format!("$ / 10s    : {}", factory.stats.3).as_str(), box_x + m_l, box_y + (6.0 * box_h) + font_h).expect("something error fill_text");

      for _ in 0..ticks_todo.min(MAX_TICKS_PER_FRAME) {
        tick_factory(&mut options, &mut state, &mut factory);
      }

      if options.web_output_cli {
        // Clear world
        context.set_fill_style(&"white".into());
        context.fill_rect(50.5, 20.5, 350.0, 700.0);

        let lines = serialize_cli_lines(&factory);

        context.set_font(&"20px monospace");
        context.set_fill_style(&"black".into());
        for n in 0..lines.len() {
          context.fill_text(format!("{}", lines[n]).as_str(), 50.5, (n as f64) * 24.0 + 50.5).expect("something lower error fill_text");
        }
      } else {
        // https://docs.rs/web-sys/0.3.28/web_sys/struct.CanvasRenderingContext2d.html

        // Paint the world

        let wx = 0.5;
        let wy = 0.5;
        let ww = 500.0;
        let wh = 500.0;

        let cw = 100.0;
        let ch = 100.0;

        // Offsets relative to cell. If cell is 100px and the belts are 3x3 grid then each part
        // can cover 33x33px at most, some buffer for movement etc, so it's 25x25? Centered, so:
        let segment_w = 33.0;
        let segment_h = 33.0;
        let part_w = 25.0;
        let part_h = 25.0;
        let part_mx = 4.0;
        let part_my = 4.0;

        let cells_x = 5;
        let cells_y = 5;

        // Clear world
        context.set_fill_style(&"white".into());
        context.fill_rect(wx, wy, cells_x as f64 * cw, cells_y as f64 * ch);

        let fw = factory.floor.width + 2;

        // Paint background cell tiles
        for coord in 0..factory.fsum {
          let (x, y) = to_xy(coord, fw);
          // This is cheating since we defer the loading stuff to the browser. Sue me.
          match factory.floor.cells[coord].kind {
            CellKind::Empty => (),
            CellKind::Belt => {
              let belt = &factory.floor.cells[coord].belt;
              let img: &HtmlImageElement = &belt_tile_images[belt.btype as usize];
              context.draw_image_with_html_image_element_and_dw_and_dh( &img, wx + cw * (x as f64), wy + ch * (y as f64), cw, ch).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
            },
            CellKind::Machine => {
              match factory.floor.cells[coord].machine {
                Machine::None => (),
                Machine::Composer => {
                  context.draw_image_with_html_image_element_and_dw_and_dh( &img_machine2, wx + cw * (x as f64), wy + ch * (y as f64), cw, ch).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
                  context.set_fill_style(&"#ff00007f".into()); // Semi transparent circles
                  if factory.floor.cells[coord].machine_input_1_want.kind != PartKind::None {
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (0.5 * segment_w), wx + cw * (x as f64) + (0.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (0.5 * segment_w), wx + cw * (x as f64) + (1.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                  }
                  if factory.floor.cells[coord].machine_input_2_want.kind != PartKind::None {
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (1.5 * segment_w), wx + cw * (x as f64) + (0.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (1.5 * segment_w), wx + cw * (x as f64) + (1.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                  }
                  if factory.floor.cells[coord].machine_input_3_want.kind != PartKind::None {
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (2.5 * segment_w), wx + cw * (x as f64) + (0.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (2.5 * segment_w), wx + cw * (x as f64) + (1.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                  }
                  context.begin_path();
                  context.ellipse(wx + cw * (x as f64) + (1.5 * segment_w), wx + cw * (x as f64) + (2.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                  context.fill();
                },
                Machine::Smasher => {
                  context.draw_image_with_html_image_element_and_dw_and_dh( &img_machine1, wx + cw * (x as f64), wy + ch * (y as f64), cw, ch).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
                  context.set_fill_style(&"#ff00007f".into()); // Semi transparent circles
                  if factory.floor.cells[coord].machine_input_1_want.kind != PartKind::None {
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (0.5 * segment_w), wx + cw * (x as f64) + (0.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (0.5 * segment_w), wx + cw * (x as f64) + (1.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                  }
                  if factory.floor.cells[coord].machine_input_2_want.kind != PartKind::None {
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (1.5 * segment_w), wx + cw * (x as f64) + (0.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (1.5 * segment_w), wx + cw * (x as f64) + (1.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                  }
                  if factory.floor.cells[coord].machine_input_3_want.kind != PartKind::None {
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (2.5 * segment_w), wx + cw * (x as f64) + (0.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (2.5 * segment_w), wx + cw * (x as f64) + (1.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                  }
                  context.begin_path();
                  context.ellipse(wx + cw * (x as f64) + (1.5 * segment_w), wx + cw * (x as f64) + (2.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
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


        // Paint cell segment grid
        for coord in 0..factory.fsum {
          let (x, y) = to_xy(coord, fw);
          if factory.floor.cells[coord].kind != CellKind::Empty {
            context.begin_path();

            context.move_to(wx + (x as f64) * cw,      wy + (y as f64) * ch);
            context.line_to(wx + (x as f64) * cw + cw, wy + (y as f64) * ch);
            context.move_to(wx + (x as f64) * cw,      wy + (y as f64) * ch + segment_h);
            context.line_to(wx + (x as f64) * cw + cw, wy + (y as f64) * ch + segment_h);
            context.move_to(wx + (x as f64) * cw,      wy + (y as f64) * ch + segment_h + segment_h);
            context.line_to(wx + (x as f64) * cw + cw, wy + (y as f64) * ch + segment_h + segment_h);

            context.move_to(wx + (x as f64) * cw,                         wy + (y as f64) * ch);
            context.line_to(wx + (x as f64) * cw,                         wy + (y as f64) * ch + ch);
            context.move_to(wx + (x as f64) * cw + segment_w,             wy + (y as f64) * ch);
            context.line_to(wx + (x as f64) * cw + segment_w,             wy + (y as f64) * ch + ch);
            context.move_to(wx + (x as f64) * cw + segment_w + segment_w, wy + (y as f64) * ch);
            context.line_to(wx + (x as f64) * cw + segment_w + segment_w, wy + (y as f64) * ch + ch);

            context.set_stroke_style(&"black".into());
            context.stroke();
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
          let (x, y) = to_xy(coord, fw);
          // This is cheating since we defer the loading stuff to the browser. Sue me.
          let cell = &factory.floor.cells[coord];
          match cell.kind {
            CellKind::Empty => (),
            CellKind::Belt => {
              // There are potentially five belt segment items to paint. Gotta check all of them.
              // let tnow = ((factory.ticks - cell.segments[SegmentDirection::UP].at) as f64).max(0.001).min(cell.speed as f64);
              // let progress = (tnow / (cell.speed as f64)) * segment_h;
              // let progress_y = if cell.belt.direction_u == Port::In { progress } else { part_h - progress };

              let progress_u = pro_lu(factory.ticks, cell.segments[SegmentDirection::UP as usize].at, cell.speed, segment_h, cell.belt.direction_u);
              let dux = wx + cw * (x as f64) + (1.0 * segment_w) + (segment_w / 2.0) + -(part_w / 2.0);
              let duy = wy + ch * (y as f64) + (0.0 * segment_h) + progress_u + -(part_h / 2.0);
              paint_segment_part(&context, &part_tile_sprite, cell.segments[SegmentDirection::UP as usize].part.clone(), 16.0, 16.0, dux, duy, part_w, part_h);
              let progress_r = pro_dr(factory.ticks, cell.segments[SegmentDirection::RIGHT as usize].at, cell.speed, segment_w, cell.belt.direction_r);
              let drx = wx + cw * (x as f64) + (2.0 * segment_w) + progress_r + -(part_w / 2.0);
              let dry = wy + ch * (y as f64) + (1.0 * segment_h) + (segment_h / 2.0) + -(part_h / 2.0);
              paint_segment_part(&context, &part_tile_sprite, cell.segments[SegmentDirection::RIGHT as usize].part.clone(), 16.0, 16.0, drx, dry, part_w, part_h);
              let progress_d = pro_dr(factory.ticks, cell.segments[SegmentDirection::DOWN as usize].at, cell.speed, segment_h, cell.belt.direction_d);
              let ddx = wx + cw * (x as f64) + (1.0 * segment_w) + (segment_w / 2.0) + -(part_w / 2.0);
              let ddy = wy + ch * (y as f64) + (2.0 * segment_h) + progress_d + -(part_h / 2.0);
              paint_segment_part(&context, &part_tile_sprite, cell.segments[SegmentDirection::DOWN as usize].part.clone(), 16.0, 16.0, ddx, ddy, part_w, part_h);
              let progress_l = pro_lu(factory.ticks, cell.segments[SegmentDirection::LEFT as usize].at, cell.speed, segment_w, cell.belt.direction_l);
              let dlx = wx + cw * (x as f64) + (0.0 * segment_w) + progress_l + -(part_w / 2.0);
              let dly = wy + ch * (y as f64) + (1.0 * segment_h) + (segment_h / 2.0) + -(part_h / 2.0);
              paint_segment_part(&context, &part_tile_sprite, cell.segments[SegmentDirection::LEFT as usize].part.clone(), 16.0, 16.0, dlx, dly, part_w, part_h);

              // Center segments are the most annoying because it depends on which belt it came
              // from and to which belt it is going. To this end we have the .allocated props.
              // Before 50% progress paint the icon that much towards the center coming from the
              // segment that handed it to the center. At 50% the part should be at the center
              // of the cell. From there on out paint the progress from center to outgoing port
              // of the preassigned target belt. (cell.to or whatever)

              let progress_c = pro(factory.ticks, cell.segments[SegmentDirection::CENTER as usize].at, cell.speed);
              let first_half = progress_c < 0.5;
              // world offset in canvas + cell offset (x,y) + segment offset (1,1 for center) - half the icon size to put anchor to icon center. then conditionally add the progress
              let sx = wx + cw * (x as f64) + (1.0 * segment_w) + -(part_w * 0.5);
              let sy = wy + ch * (y as f64) + (1.0 * segment_h) + -(part_h * 0.5);
              let (px, py) =
                match if first_half { cell.segments[SegmentDirection::CENTER as usize].from } else { cell.segments[SegmentDirection::CENTER as usize].to } {
                  SegmentDirection::UP => {
                    let cux = sx + (segment_w * 0.5);
                    let cuy = sy + (segment_w * (if first_half { progress_c } else { 1.0 - progress_c }));
                    (cux, cuy)
                  }
                  SegmentDirection::RIGHT => {
                    let dlx = sx + (segment_w * (if first_half { 1.0 - progress_c } else { progress_c }));
                    let dly = sy + (segment_h * 0.5);
                    (dlx, dly)
                  }
                  SegmentDirection::DOWN => {
                    let cux = sx + (segment_w * 0.5);
                    let cuy = sy + (segment_w * (if first_half { 1.0 - progress_c } else { progress_c }));
                    (cux, cuy)
                  }
                  SegmentDirection::LEFT => {
                    let dlx = sx + (segment_w * (if first_half { progress_c } else { 1.0 - progress_c }));
                    let dly = sy + (segment_h * 0.5);
                    (dlx, dly)
                  }
                  SegmentDirection::CENTER => panic!(".from cannot be center"),
                };

              if paint_segment_part(&context, &part_tile_sprite, cell.segments[SegmentDirection::CENTER as usize].part.clone(), 16.0, 16.0, px, py, part_w, part_h) {
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
                  paint_segment_part(&context, &part_tile_sprite, cell.machine_input_1_want.clone(), 16.0, 16.0, wx + cw * (x as f64) + (0.0 * segment_w) + part_mx, wy + ch * (y as f64) + (0.0 * segment_h) + part_my, part_w, part_h);
                  paint_segment_part(&context, &part_tile_sprite, cell.machine_input_2_want.clone(), 16.0, 16.0, wx + cw * (x as f64) + (1.0 * segment_w) + part_mx, wy + ch * (y as f64) + (0.0 * segment_h) + part_my, part_w, part_h);
                  paint_segment_part(&context, &part_tile_sprite, cell.machine_input_3_want.clone(), 16.0, 16.0, wx + cw * (x as f64) + (2.0 * segment_w) + part_mx, wy + ch * (y as f64) + (0.0 * segment_h) + part_my, part_w, part_h);
                  paint_segment_part(&context, &part_tile_sprite, cell.machine_input_1_have.clone(), 16.0, 16.0, wx + cw * (x as f64) + (0.0 * segment_w) + part_mx, wy + ch * (y as f64) + (1.0 * segment_h) + part_my, part_w, part_h);
                  paint_segment_part(&context, &part_tile_sprite, cell.machine_input_2_have.clone(), 16.0, 16.0, wx + cw * (x as f64) + (1.0 * segment_w) + part_mx, wy + ch * (y as f64) + (1.0 * segment_h) + part_my, part_w, part_h);
                  paint_segment_part(&context, &part_tile_sprite, cell.machine_input_3_have.clone(), 16.0, 16.0, wx + cw * (x as f64) + (2.0 * segment_w) + part_mx, wy + ch * (y as f64) + (1.0 * segment_h) + part_my, part_w, part_h);
                  paint_segment_part(&context, &part_tile_sprite, cell.machine_output_want.clone(), 16.0, 16.0, wx + cw * (x as f64) + (1.0 * segment_w) + part_mx, wy + ch * (y as f64) + (2.0 * segment_h) + part_my, part_w, part_h);
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
