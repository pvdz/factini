use super::belt::*;
use super::belt_type::*;
use super::bouncer::*;
use super::canvas::*;
use super::cell::*;
use super::cli_serialize::*;
use super::config::*;
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
use super::quest::*;
use super::state::*;
use super::truck::*;
use super::utils::*;
use super::zone::*;

// Makes dyn_into work in this file
use wasm_bindgen::JsCast;

// This explicitly import shoulnd't be necessary anymore according to https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files but ... well I did at the time of writing.
use super::log;

pub struct QuickSave {
  pub index: usize,
  pub thumb: web_sys::HtmlCanvasElement, // thumbnail
  pub img: web_sys::HtmlImageElement, // new Image().src = this.png
  pub loaded: bool, // whether or not this image has been seen finished loading
  pub snapshot: String, // map string
  pub png: String, // From toDataUrl()
}

pub fn quick_save_create(index: usize, document: &web_sys::Document, snapshot: String, png: String) -> QuickSave {
  let thumb = document.create_element("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
  thumb.set_width(UI_SAVE_THUMB_IMG_WIDTH as u32);
  thumb.set_height(UI_SAVE_THUMB_IMG_HEIGHT as u32);

  let img = document.create_element("img").unwrap().dyn_into::<web_sys::HtmlImageElement>().unwrap();
  img.set_src(&png);
  return QuickSave {
    index,
    thumb,
    img,
    loaded: false,
    snapshot,
    png
  };
}

pub fn quick_save_onload(document: &web_sys::Document, quick_save: &mut QuickSave) {
  // The image finished loading since the previous frame.
  // Generate a cached thumbnail to use going forward
  let thumb_context = quick_save.thumb.get_context("2d").expect("canvas api to work").unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().expect("canvas api to work");
  {
    // Add to window for debugging
    // document.get_element_by_id("$main_game").unwrap().append_child(&quick_save.thumb);
  }
  let ptrn = thumb_context.create_pattern_with_html_image_element(&quick_save.img, "repeat").expect("trying to load thumb").unwrap();
  canvas_round_rect(&thumb_context, 0.0, 0.0, UI_SAVE_THUMB_IMG_WIDTH, UI_SAVE_THUMB_IMG_HEIGHT);
  thumb_context.set_fill_style(&ptrn);
  thumb_context.fill();
  thumb_context.set_stroke_style(&"black".into());
  thumb_context.stroke();

  quick_save.loaded = true;
}
