// This file should only be included for `wasm-pack build --target web`
// The main.rs will include this file when `#[cfg(target_arch = "wasm32")]`

// This crate dumps panics to console.log in the browser
extern crate console_error_panic_hook;


// This is required to export panic to the web
use std::panic;

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
use super::demand::*;
use super::factory::*;
use super::floor::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::state::*;
use super::supply::*;

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
  log("wtf?");
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

  let todo = load_tile("./img/todo.png").expect("can't BELT_NONE.src");

  // Preload the belt tiles. Create an array with a to-do image for every slot. Then create img tags
  let mut belt_tile_images: Vec<web_sys::HtmlImageElement> = vec![todo; BELT_TYPE_COUNT]; // Prefill with todo images
  belt_tile_images[BeltType::NONE as usize] = load_tile(BELT_NONE.src)?;
  belt_tile_images[BeltType::U_R as usize] = load_tile(BELT_U_R.src)?;
  belt_tile_images[BeltType::R_U as usize] = load_tile(BELT_R_U.src)?;
  belt_tile_images[BeltType::R_D as usize] = load_tile(BELT_R_D.src)?;
  belt_tile_images[BeltType::D_R as usize] = load_tile(BELT_D_R.src)?;
  belt_tile_images[BeltType::D_L as usize] = load_tile(BELT_D_L.src)?;
  belt_tile_images[BeltType::L_D as usize] = load_tile(BELT_L_D.src)?;
  belt_tile_images[BeltType::L_U as usize] = load_tile(BELT_L_U.src)?;
  belt_tile_images[BeltType::U_L as usize] = load_tile(BELT_U_L.src)?;
  belt_tile_images[BeltType::U_D as usize] = load_tile(BELT_U_D.src)?;
  belt_tile_images[BeltType::D_U as usize] = load_tile(BELT_D_U.src)?;
  belt_tile_images[BeltType::L_R as usize] = load_tile(BELT_L_R.src)?;
  belt_tile_images[BeltType::R_L as usize] = load_tile(BELT_R_L.src)?;
  belt_tile_images[BeltType::U_LR as usize] = load_tile(BELT_U_LR.src)?;
  belt_tile_images[BeltType::RU_L as usize] = load_tile(BELT_RU_L.src)?;
  belt_tile_images[BeltType::LU_R as usize] = load_tile(BELT_LU_R.src)?;
  belt_tile_images[BeltType::L_RU as usize] = load_tile(BELT_L_RU.src)?;
  belt_tile_images[BeltType::LR_U as usize] = load_tile(BELT_LR_U.src)?;
  belt_tile_images[BeltType::R_LU as usize] = load_tile(BELT_R_LU.src)?;
  belt_tile_images[BeltType::R_DU as usize] = load_tile(BELT_R_DU.src)?;
  belt_tile_images[BeltType::RU_D as usize] = load_tile(BELT_RU_D.src)?;
  belt_tile_images[BeltType::DR_U as usize] = load_tile(BELT_DR_U.src)?;
  belt_tile_images[BeltType::DU_R as usize] = load_tile(BELT_DU_R.src)?;
  belt_tile_images[BeltType::U_DR as usize] = load_tile(BELT_U_DR.src)?;
  belt_tile_images[BeltType::D_RU as usize] = load_tile(BELT_D_RU.src)?;
  belt_tile_images[BeltType::D_LR as usize] = load_tile(BELT_D_LR.src)?;
  belt_tile_images[BeltType::DL_R as usize] = load_tile(BELT_DL_R.src)?;
  belt_tile_images[BeltType::DR_L as usize] = load_tile(BELT_DR_L.src)?;
  belt_tile_images[BeltType::LR_D as usize] = load_tile(BELT_LR_D.src)?;
  belt_tile_images[BeltType::L_DR as usize] = load_tile(BELT_L_DR.src)?;
  belt_tile_images[BeltType::R_DL as usize] = load_tile(BELT_R_DL.src)?;
  belt_tile_images[BeltType::L_DU as usize] = load_tile(BELT_L_DU.src)?;
  belt_tile_images[BeltType::LU_D as usize] = load_tile(BELT_LU_D.src)?;
  belt_tile_images[BeltType::DL_U as usize] = load_tile(BELT_DL_U.src)?;
  belt_tile_images[BeltType::DU_L as usize] = load_tile(BELT_DU_L.src)?;
  belt_tile_images[BeltType::U_DL as usize] = load_tile(BELT_U_DL.src)?;
  belt_tile_images[BeltType::D_UL as usize] = load_tile(BELT_D_UL.src)?;
  belt_tile_images[BeltType::U_DLR as usize] = load_tile(BELT_U_DLR.src)?;
  belt_tile_images[BeltType::R_DLU as usize] = load_tile(BELT_R_DLU.src)?;
  belt_tile_images[BeltType::D_LRU as usize] = load_tile(BELT_D_LRU.src)?;
  belt_tile_images[BeltType::L_DRU as usize] = load_tile(BELT_L_DRU.src)?;
  belt_tile_images[BeltType::RU_DL as usize] = load_tile(BELT_RU_DL.src)?;
  belt_tile_images[BeltType::DU_LR as usize] = load_tile(BELT_DU_LR.src)?;
  belt_tile_images[BeltType::LU_DR as usize] = load_tile(BELT_LU_DR.src)?;
  belt_tile_images[BeltType::LD_RU as usize] = load_tile(BELT_LD_RU.src)?;
  belt_tile_images[BeltType::DR_LU as usize] = load_tile(BELT_DR_LU.src)?;
  belt_tile_images[BeltType::LR_DU as usize] = load_tile(BELT_LR_DU.src)?;
  belt_tile_images[BeltType::DLR_U as usize] = load_tile(BELT_DLR_U.src)?;
  belt_tile_images[BeltType::DLU_R as usize] = load_tile(BELT_DLU_R.src)?;
  belt_tile_images[BeltType::RLU_D as usize] = load_tile(BELT_RLU_D.src)?;
  belt_tile_images[BeltType::DRU_L as usize] = load_tile(BELT_DRU_L.src)?;

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
  let mut options = create_options();

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

        // Paint background cell tiles
        for y in 0..cells_y {
          for x in 0..cells_x {
            // This is cheating since we defer the loading stuff to the browser. Sue me.
            match factory.floor.cells[x][y].kind {
              CellKind::Empty => (),
              CellKind::Belt => {
                let belt = &factory.floor.cells[x][y].belt;
                let img: &HtmlImageElement = &belt_tile_images[belt.btype as usize];
                context.draw_image_with_html_image_element_and_dw_and_dh( &img, wx + cw * (x as f64), wy + ch * (y as f64), cw, ch).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
              },
              CellKind::Machine => {
                match factory.floor.cells[x][y].machine {
                  Machine::None => (),
                  Machine::Composer => {
                    context.draw_image_with_html_image_element_and_dw_and_dh( &img_machine2, wx + cw * (x as f64), wy + ch * (y as f64), cw, ch).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
                    context.set_fill_style(&"#ffffff7f".into()); // Semi transparent circles
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (0.5 * segment_w), wx + cw * (x as f64) + (0.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (1.5 * segment_w), wx + cw * (x as f64) + (0.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (2.5 * segment_w), wx + cw * (x as f64) + (0.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (0.5 * segment_w), wx + cw * (x as f64) + (1.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (1.5 * segment_w), wx + cw * (x as f64) + (1.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (2.5 * segment_w), wx + cw * (x as f64) + (1.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (1.5 * segment_w), wx + cw * (x as f64) + (2.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                  },
                  Machine::Smasher => {
                    context.draw_image_with_html_image_element_and_dw_and_dh( &img_machine1, wx + cw * (x as f64), wy + ch * (y as f64), cw, ch).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
                    context.set_fill_style(&"#ffffff7f".into()); // Semi transparent circles
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (0.5 * segment_w), wx + cw * (x as f64) + (0.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (1.5 * segment_w), wx + cw * (x as f64) + (0.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (2.5 * segment_w), wx + cw * (x as f64) + (0.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (0.5 * segment_w), wx + cw * (x as f64) + (1.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (1.5 * segment_w), wx + cw * (x as f64) + (1.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (2.5 * segment_w), wx + cw * (x as f64) + (1.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                    context.begin_path();
                    context.ellipse(wx + cw * (x as f64) + (1.5 * segment_w), wx + cw * (x as f64) + (2.5 * segment_w), part_w / 2.0, part_h / 2.0, 3.14, 0.0, 6.28).expect("to paint a circle");
                    context.fill();
                  },
                }
              },
            }
          }
        }


        // Paint cell segment grid
        for y in 0..cells_y {
          for x in 0..cells_x {
            if factory.floor.cells[x][y].kind != CellKind::Empty {
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
        }

        fn pro_lu(ticks: u64, at: u64, speed: u64, distance: f64, dir: BeltDirection) -> f64 {
          let tnow = ((ticks - at) as f64).max(0.001).min(speed as f64);
          let progress = (tnow / (speed as f64)) * distance;
          return if dir == BeltDirection::In { progress } else { distance - progress };
        }
        fn pro_dr(ticks: u64, at: u64, speed: u64, distance: f64, dir: BeltDirection) -> f64 {
          let tnow = ((ticks - at) as f64).max(0.001).min(speed as f64);
          let progress = (tnow / (speed as f64)) * distance;
          return if dir == BeltDirection::In { distance - progress } else { progress };
        }

        // Paint elements on the belt over the background tiles now
        for y in 0..cells_y {
          for x in 0..cells_x {
            // This is cheating since we defer the loading stuff to the browser. Sue me.
            let cell = &factory.floor.cells[x][y];
            match cell.kind {
              CellKind::Empty => (),
              CellKind::Belt => {
                // There are potentially five belt segment items to paint. Gotta check all of them.
                // let tnow = ((factory.ticks - cell.segment_u_at) as f64).max(0.001).min(cell.speed as f64);
                // let progress = (tnow / (cell.speed as f64)) * segment_h;
                // let progress_y = if cell.belt.direction_u == BeltDirection::In { progress } else { part_h - progress };

                let progress_u = pro_lu(factory.ticks, cell.segment_u_at, cell.speed, segment_h, cell.belt.direction_u);
                paint_segment_part(&context, &part_tile_sprite, cell.segment_u_part.clone(), 16.0, 16.0, wx + cw * (x as f64) + (1.0 * segment_w) + part_mx, wy + (ch * (y as f64)) + (0.0 * segment_h) + part_my + progress_u - (part_h / 2.0), part_w, part_h);
                let progress_r = pro_dr(factory.ticks, cell.segment_r_at, cell.speed, segment_w, cell.belt.direction_r);
                paint_segment_part(&context, &part_tile_sprite, cell.segment_r_part.clone(), 16.0, 16.0, wx + cw * (x as f64) + (2.0 * segment_w) + part_mx + progress_r - (part_w / 2.0), wy + ch * (y as f64) + (1.0 * segment_h) + part_my, part_w, part_h);
                let progress_d = pro_dr(factory.ticks, cell.segment_d_at, cell.speed, segment_h, cell.belt.direction_d);
                paint_segment_part(&context, &part_tile_sprite, cell.segment_d_part.clone(), 16.0, 16.0, wx + cw * (x as f64) + (1.0 * segment_w) + part_mx, wy + ch * (y as f64) + (2.0 * segment_h) + part_my + progress_d - (part_h / 2.0), part_w, part_h);
                let progress_l = pro_lu(factory.ticks, cell.segment_l_at, cell.speed, segment_w, cell.belt.direction_l);
                paint_segment_part(&context, &part_tile_sprite, cell.segment_l_part.clone(), 16.0, 16.0, wx + cw * (x as f64) + (0.0 * segment_w) + part_mx + progress_l - (part_w / 2.0), wy + ch * (y as f64) + (1.0 * segment_h) + part_my, part_w, part_h);

                // Center segments are the most annoying because it very much depends on the cell tile and direction of the element, except we only know at the end which way a part ends up going when the belt splits...
                // This requires rethinking that logic a bit
                // A segment within the same cell is only ever filled from the center or an incoming neighbor but a single segment can't be filled both ways at the same time. So when a center
                // segment has to consider which neighbor segment will receive the part it has to check the beltdirection and the current part status. If the segment is outgoing and has no
                // item when the part is mid-way the center segment, then it should be a safe pick regardless of what happens until the part leaves the center piece, since it is the only
                // valid supplier of that segment.
                // This does mean that the center segment has to track which way a part goes and determine mid-way (rather than at the end) which segment will receive the part.
                // Additionally, this means the part could be "stuck" at the center, rather than the end, which was the initial generic design. A small deviation.
                paint_segment_part(&context, &part_tile_sprite, cell.segment_c_part.clone(), 16.0, 16.0, wx + cw * (x as f64) + (1.0 * segment_w) + part_mx, wy + ch * (y as f64) + (1.0 * segment_h) + part_my, part_w, part_h);
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

fn paint_segment_part(context: &Rc<web_sys::CanvasRenderingContext2d>, part_tile_sprite: &HtmlImageElement, segment_part: Part, spw: f64, sph: f64, dx: f64, dy: f64, dw: f64, dh: f64) {
  let (spx, spy) = match segment_part.kind {
    PartKind::WoodenStick => {
      // This is a club? Piece of wood I guess? From which wands are formed.
      (0.0, 11.0)
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
      return;
    },
  };

  context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
    &part_tile_sprite,
    // Sprite position
    spx * spw, spy * sph, spw, sph,
    // Paint onto canvas at
    dx, dy, dw, dh,
  ).expect("something error draw_image"); // requires web_sys HtmlImageElement feature
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
