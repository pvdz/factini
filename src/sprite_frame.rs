#[derive(Debug, Clone)]
pub struct SpriteFrame {
  // Image file to load
  pub file: String,

  // Name (for debugging). Anything after the `- frame` is put in here for no reason.
  pub name: String,

  // Canvas index that has loaded this image
  pub file_canvas_cache_index: usize,

  // Sprite offsets and size into the sprite file
  pub x: f64,
  pub y: f64,
  pub w: f64,
  pub h: f64
}
