#[cfg(not(target_arch = "wasm32"))]
use web_sys::{HtmlCanvasElement, HtmlImageElement};

pub fn progress(ticks: u64, since: u64, range: u64) -> f64 {
  return ((ticks - since) as f64).max(0.00001) / (range as f64);
}

pub fn log(s: String) {
  #[cfg(not(target_arch = "wasm32"))]
  println!("{}", s);
  #[cfg(target_arch = "wasm32")]
  let s = s.as_str();
  #[cfg(target_arch = "wasm32")]
  web_sys::console::log_2(&"(rust)".into(), &s.into());
}

// pub fn error(s: String) {
  // 1.65.0
  // std::backtrace::Backtrace
  // let t = Backtrace::force_capture();
// }

pub fn get_time_now() -> u64{
  return js_sys::Date::now() as u64;
}

pub fn bounds_check(x: f64, y: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
  return x >= x1 && x < x2 && y >= y1 && y < y2;
}
pub fn rect_check(x0: f64, y0: f64, x: f64, y: f64, w: f64, h: f64) -> bool {
  return x0 >= x && x0 < x+w && y0 >= y && y0 < y+h;
}

pub fn line_check(n: f64, a: f64, b: f64) -> bool {
  return n >= a && n < b;
}
