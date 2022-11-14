#[derive(Debug, Clone)]
pub struct SpriteFrame {
  // Image file to load
  pub file: String,

  // Canvas index that has loaded this image
  pub file_canvas_cache_index: usize,

  // Sprite offsets and size into the sprite file
  pub x: f64,
  pub y: f64,
  pub w: f64,
  pub h: f64
}
