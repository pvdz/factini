// This is the segment of a belt.

use super::belt::*;
use super::cell::*;
use super::options::*;
use super::machine::*;
use super::part::*;
use super::state::*;

#[derive(Debug)]
pub struct Segment {
  pub some: bool, // Does this segment actually exist?
  pub dir: SegmentDirection, // For Edge Segments; which way does the port connect this segment?
  pub part: Part, // None if empty
  pub at: u64, // If not empty, when did the current part enter this segment?
  pub allocated: bool, // Has this part been cleared to move out of this segment when it gets there? This is necessary for all segments, not just center, because that's the only way to ensure a non-blocking fluid movement in unfinished models.
  pub claimed: bool, // Has the free spot in this segment been claimed by another segment/etc? Only relevant for center and machines, although machines probably need a more sophisticated model for it.
  pub from: SegmentDirection, // For the center segment; where did this part originate from?
  pub to: SegmentDirection, // For the center segment; once unblocked, where will this part be moving to?
  pub port: Port,
}

// These are indices to the Cell.segments array
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SegmentDirection {
  UP = 0,
  RIGHT = 1,
  DOWN = 2,
  LEFT = 3,
  CENTER = 4,
}

pub const fn segment_create(dir: SegmentDirection, port: Port) -> Segment {
  if matches!(port, Port::None) { segment_none(dir) }
  else { segment_some(dir, port) }
}

pub const fn segment_some(dir: SegmentDirection, port: Port) -> Segment {
  return Segment {
    some: true,
    dir,
    part: part_none(),
    at: 0,
    allocated: false,
    claimed: false,
    from: SegmentDirection::UP,
    to: SegmentDirection::UP,
    port,
  };
}

pub const fn segment_none(dir: SegmentDirection) -> Segment {
  // A segment that doesn't really exist. Can't receive parts. Don't paint this.
  return Segment {
    some: false,
    dir,
    part: part_none(),
    at: 0,
    allocated: false,
    claimed: false,
    from: SegmentDirection::UP,
    to: SegmentDirection::UP,
    port: Port::None,
  };
}

