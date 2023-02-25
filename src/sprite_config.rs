use super::sprite_frame::*;

#[derive(Debug, Clone)]
pub struct SpriteConfig {
  // Which frame should be the first? This is the frame index
  pub frame_offset: usize,

  // How many frames should this config automatically extrapolate?
  // If fewer than this frames were specified, the system will automatically expand to that.
  pub frame_count: u64,

  // When extrapolating, in which direction should the system extrapolate from the previous frame?
  pub frame_direction: SpriteConfigDirection,

  // Show the initial frame this long before starting the rest of the animation
  pub initial_delay: u64,

  // Show each frame for this long (pause between frames)
  pub frame_delay: u64,

  // Restart animation after last frame was shown?
  pub looping: bool,

  // Pause this long between loops
  pub loop_delay: u64,

  // Play frames in reverse order
  pub loop_backwards: bool,

  // Frame details for painting this sprite
  // Must be non-empty
  pub frames: Vec<SpriteFrame>
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpriteConfigDirection {
  Up,
  Right,
  Down,
  Left,
}
