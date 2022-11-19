use super::sprite_frame::*;

#[derive(Debug, Clone)]
pub struct SpriteConfig {
  // Show each frame this many world ticks (not actual in frame count)
  pub pause_between: u64,

  // Which frame should be the first?
  pub frame_offset: u64,

  // Show the initial frame this long before starting the rest of the animation
  pub initial_delay: u64,

  // Restart animation after last frame was shown?
  pub looping: bool,

  // Frame details for painting this sprite
  // Must be non-empty
  pub frames: Vec<SpriteFrame>
}
