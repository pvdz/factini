use wasm_bindgen::{JsCast, JsValue};

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
use super::quest_state::*;
use super::sprite_config::*;
use super::sprite_frame::*;
use super::state::*;
use super::supply::*;
use super::truck::*;
use super::utils::*;
use super::woop::*;
use super::zone::*;
use super::log;

const SPRITER_PADDING: f64 = 5.0;

pub fn spriter_assets(options: &Options, state: &State, config: &Config) -> Result<(), JsValue> {
  // Gather all assets
  // Find all the animation configs
  // For each asset
  // - draw their sprite(s)
  // - remember positions
  // - write config segment

  let types: Vec<ConfigNodeKind> = vec!(
    ConfigNodeKind::Asset,
    ConfigNodeKind::Belt,
    ConfigNodeKind::Part
  );
  for t in types {
    let mut list: Vec<(
      PartKind,
      &SpriteConfig
    )> = vec!();
    let mut total_width: f64 = 0.0;
    let mut total_height: f64 = 0.0;

    for index in 0..config.nodes.len() {
      let kind = index as PartKind;
      let node = &config.nodes[index];

      if node.kind == t {
        let len = config.nodes[kind].sprite_config.frames.len();
        for f in 0..len {
          let frame = &config.nodes[kind].sprite_config.frames[f];
          total_width += frame.w + SPRITER_PADDING;
          total_height = total_height.max(frame.h);
          // log!("frame {}/{} of kind {} ({}) has size: {}x{}, total width now at: {}", f+1, len, kind, node.raw_name, frame.w, frame.h, total_width);
        }

        list.push((
          kind,
          &config.nodes[kind].sprite_config
        ));
      }
    }

    // log!("result: {:?}", list);

    spriter(options, state, config, list);
  }


  return Ok(());
}

fn spriter(options: &Options, state: &State, config: &Config, list: Vec<(PartKind, &SpriteConfig)>) -> Result<(), JsValue> {
  // For each image, pick the smallest dimension and start trying to put it within the current canvas, left to right, top to bottom or the other way around (whichever is smaller)
  // If there's no existing hole, extend the direction that is the biggest?

  let mut placed_by_x: Vec<(u64, u64, u64, u64, PartKind, &SpriteConfig, usize)> = vec!();
  let mut placed_by_y: Vec<(u64, u64, u64, u64, PartKind, &SpriteConfig, usize)> = vec!();

  let p = SPRITER_PADDING as u64;

  let mut rng = xorshift(123456789);

  // Order the list by vertical size of their first frame (usually the same), biggest first
  let mut list_by_h = list;
  list_by_h.sort_by(|(_, a), (_, b)| b.frames[0].h.total_cmp(&a.frames[0].h));

  let mut verbose = false;
  let until = 1000000; // Set high enough to act like "never"

  let mut n = 0;
  let mut maxw: u64 = 0;
  let mut maxh: u64 = 0;
  for l in 0..list_by_h.len() {
    if l == until { verbose = true; }
    if l > until { break; }
    let ( part_kind, sprite_config ) = list_by_h[l];
    if verbose { log!("start {} ({}): {} frames; {}x{} {}x{}", l, config.nodes[part_kind].raw_name, sprite_config.frames.len(), sprite_config.frames[0].x, sprite_config.frames[0].y, sprite_config.frames[0].x + sprite_config.frames[0].w, sprite_config.frames[0].y + sprite_config.frames[0].h); }
    for f in 0..sprite_config.frames.len() {

      let frame = &sprite_config.frames[f];
      let dx = frame.w as u64 + p;
      let dy = frame.h as u64 + p;

      let mut x: u64 = 0;
      let mut y: u64 = 0;
      let mut x2 = x + dx;
      let mut y2 = y + dy;

      let mut changed = true;

      let mut row = 0;
      let mut col = 1;
      while changed {
        changed = false;
        if verbose { log!(""); }
        if verbose { log!("row {}, col {}, from {}x{} to {}x{}, max {}x{}", row + 1, col, x, y, x2, y2, maxw, maxh); }
        for i in 0..placed_by_x.len() {
          let ( x3, y3, x4, y4 , ..) = placed_by_x[i];
          if spriter_overlap(x, y, x2, y2, x3, y3, x4, y4) {
            if verbose { log!("- {} blocked {} {} {} {} x {} {} {} {} -> ({} {})", i, x, y, x2, y2, x3, y3, x4, y4, x4-x3-p, y4-y3-p); }
            if x4 + dx > maxw {
              if verbose { log!("  - row {}, end of row... {} > {}", row, x4, maxw); }
              // Finished checking with this row. Continue with the next row. Unless this was the last

              while placed_by_y[row].3 == y && row < placed_by_y.len() {
                // Skip row if y would not change
                row += 1;
              }
              let nexty = placed_by_y[row].3; // y4

              if nexty + dy > maxh {
                // log!("the end... {} > {}, of row {}", y, maxh, row);
                // Did not fit in image. Extend it (to the right?)
                x = maxw;
                x2 = x + dx;
                y = 0;
                y2 = dy;
                break;
              } else {
                // Start on line on the next "row"
                x = 0;
                x2 = dx;
                y = nexty;
                y2 = y + dy;
                row += 1; // We are now below this element so stop checking it in the loop
                col = 1;
                changed = true;
                break;
              }
            } else {
              // Next column
              x = x4;
              x2 = x + dx;
              changed = true;
              col += 1;
              break;
            }
          } else {
            // Not overlapping. Move to next placed item.
            // (If all of them don't overlap then we found a hole to put this frame)
          }
        }
      }
      if verbose { log!("- placed at {} {} {} {} (bigger? {})", x, y, x2, y2, x2 > maxw || y2 > maxh); }

      // vertically sorted
      placed_by_y.push((x, y, x2, y2, part_kind, sprite_config, f));
      // Sort the placements by their y2, low to high
      placed_by_y.sort_by(|(_, _, _, a, ..), (_, _, _, b, ..)| a.cmp(b));

      // also maintain elements left to right
      placed_by_x.push((x, y, x2, y2, part_kind, sprite_config, f));
      // Sort the placements by their x1, low to high
      placed_by_x.sort_by(|(a, _, _, _, ..), (b, _, _, _, ..)| a.cmp(b));

      if x2 > maxw { maxw = x2; }
      if y2 > maxh { maxh = y2; }
    }
  }

  // log!("v2 placement: {:?}", placed);
  log!("Canvas size: {}x{}", maxw, maxh);


  let document = document();
  let canvas = document.create_element("canvas")?.dyn_into::<web_sys::HtmlCanvasElement>()?;
  document.get_element_by_id("$main_game").unwrap().append_child(&canvas)?;
  if maxw > 32000 || maxh > 32000 { log!("WARNING: total size exceeds max canvas size of 32k x 32k so it will fail... ({} x {})", maxw, maxh); }
  canvas.set_width(maxw.min(32000) as u32);
  canvas.set_height(maxh.min(32000) as u32);
  canvas.style().set_property("border", "solid")?;
  canvas.style().set_property("max-width", format!("{}px", CANVAS_CSS_INITIAL_WIDTH as u32).as_str())?;
  canvas.style().set_property("max-height", format!("{}px", CANVAS_CSS_INITIAL_HEIGHT as u32).as_str())?;
  // We shouldn't be scaling so I don't think this property matters here...
  canvas.style().set_property("image-rendering", "pixelated").expect("should work");

  // // Background image on this canvas is irrelevant so slightly optimize it.
  // let context_options = js_sys::Object::new();
  // js_sys::Reflect::set(&context_options, &JsValue::from_str("alpha"), &JsValue::from_bool(false)).unwrap();
  // let context = canvas.get_context_with_context_options("2d", &context_options)?.unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>()?;
  let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
  context.set_stroke_style(&"black".into());

  // Note: frames are parsed in order, the "value" is treated like a comment. So we _must_ group and sort frames for the same asset together and print them in one group.
  let mut placed_by_frame = placed_by_y;
  placed_by_frame.sort_by(|( _, _, _, _, kind1, _, frame_index1 ), ( _, _, _, _, kind2, _, frame_index2 )| {
    if kind1 != kind2 {
      return kind1.cmp(kind2);
    }
    return frame_index1.cmp(frame_index2);
  });

  let mut n = 0;
  let mut strings = vec!();
  let mut last_kind = usize::MAX; // Let's never use this as a part kind ;)
  for ( x1, y1, x2, y2, kind, sprite_config, frame_index ) in placed_by_frame {
    let frame = &sprite_config.frames[frame_index];

    let x = x1 as f64;
    let y = y1 as f64;
    let w = (x2-x1-p) as f64; // is inc p
    let h = (y2-y1-p) as f64; // is inc p

    context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
      &config.sprite_cache_canvas[frame.file_canvas_cache_index],
      sprite_config.frames[frame_index].x, sprite_config.frames[frame_index].y, sprite_config.frames[frame_index].w, sprite_config.frames[frame_index].h,
      x, y, w, h
    ).expect("magic!");

    let header_maybe = if last_kind == kind { "".to_string() } else { format!("\n# {}\n- gen\n- file: ./export.png\n", config.nodes[kind].raw_name) };
    strings.push(format!("{}- frame: {}\n- x: {}\n- y: {}\n- w: {}\n- h: {}", header_maybe, frame_index, sprite_config.frames[frame_index].x, sprite_config.frames[frame_index].y, sprite_config.frames[frame_index].w, sprite_config.frames[frame_index].h));
    last_kind = kind;

    // context.stroke_rect(x, y, frame.w, frame.h);
    // context.stroke_text(&config.nodes[kind].raw_name, offset_x, frame.h);
    n += 1;
  }

  log!("drew {} frames", n);

  let ta = document.create_element("textarea")?.dyn_into::<web_sys::HtmlTextAreaElement>()?;
  document.get_element_by_id("$main_game").unwrap().append_child(&ta)?;
  ta.set_value(&strings.join("\n"));

  return Ok(());
}

fn spriter_overlap(x1: u64, y1: u64, x2: u64, y2: u64, x3: u64, y3: u64, x4: u64, y4: u64) -> bool {
  // Two rects <<x1, y1>, <x2, y2>>, <<x3, y3>, <x4, y4>>
  // Basically, if either box is to the left or right of the other box, or
  // above or below the other box, then they can't overlap. Otherwise they must.
  if x1 >= x4 || x3 >= x2 { return false; }
  if y1 >= y4 || y3 >= y2 { return false; }
  return true;
}
