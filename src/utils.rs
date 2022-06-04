
pub fn progress(ticks: u64, since: u64, range: u64) -> f64 {
  return (ticks - since) as f64 / (range as f64);
}
