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

// Simple wrapper for `log(format!())` into `log!()`
#[macro_export]
macro_rules! log {
  ($fmt_str:literal) => {
      log(format!($fmt_str))
  };

  ($fmt_str:literal, $($args:expr),*) => {
      log(format!($fmt_str, $($args),*))
  };
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

pub fn xorshift(z: usize) -> usize {
  // "xorshift" "prng" => https://en.wikipedia.org/wiki/Xorshift
  let z = z ^ z << 13;
  let z = z ^ z >> 17;
  let z = z ^ z << 5;
  return z;
}

// Examples: https://easings.net/
fn ease_cos(v: f64) -> f64 {
  return (1.0 - (v * std::f64::consts::PI).cos()) / 2.0;
}
fn ease_cubic(v: f64) -> f64 {
  return v*v*v*v;
}
fn ease_sin(v: f64) -> f64 {
  return (v * std::f64::consts::PI).sin();
}
fn ease_out(v: f64) -> f64 {
  return 1.0 - (1.0 - v).powf(2.0);
}

#[derive(Clone, Debug)]
pub enum Ease {
  None,
  Cos,
  Cubic,
  Sin,
  Out,
}

pub fn ease_progress(a: f64, b: f64, p: f64, ease: Ease) -> f64 {
  let p = match ease {
    Ease::None => p,
    Ease::Sin => ease_sin(p),
    Ease::Cubic => ease_cubic(p),
    Ease::Cos => ease_cos(p),
    Ease::Out => ease_out(p),
  };
  return a + p * (b - a);
}
