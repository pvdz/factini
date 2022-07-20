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

pub fn get_time_now() -> u64{
  return js_sys::Date::now() as u64;
}
