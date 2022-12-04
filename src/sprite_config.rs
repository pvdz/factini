use super::sprite_frame::*;

#[derive(Debug, Clone)]
pub struct SpriteConfig {
  // Which frame should be the first? This is the frame index
  pub frame_offset: usize,

  // Show the initial frame this long before starting the rest of the animation
  pub initial_delay: u64,

  // Show each frame for this long (pause between frames)
  pub frame_delay: u64,

  // Restart animation after last frame was shown?
  pub looping: bool,

  // Pause this long between loops
  pub loop_delay: u64,

  // Frame details for painting this sprite
  // Must be non-empty
  pub frames: Vec<SpriteFrame>
}
