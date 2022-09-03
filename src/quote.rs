// A Quote is a build order to complete. They are like achievements, quests, or build requests.
// Each Quote should have one or more things to complete and a reward upon completion, usually
// an unlock step of sorts, as well as the next Quote or set of quotes that it unlocks.

use super::config::*;
use super::part::*;
use super::utils::*;

#[derive(Debug)]
pub struct Quote {
  pub kind: QuoteKind,
  pub wants: Vec< (
    // Which part do we want?
    char,
    // How many do we want?
    u64,
    // How many have we received since last factory tick? Recomputed after every factory tick.
    u64
  ) >,
  pub unlocks_parts: Vec<char>,
  pub unlocks_quotes: Vec<QuoteKind>,
  pub unlock_conditions: Vec<QuoteKind>,
  pub added_at: u64,
  pub completed_at: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QuoteKind {
  Inglish,
  IngotsForLife,
  TheShieldsThatWood,
  DontDrinkThose,
  WhySoBlue,
  GotYaCovered,
  PapersPlease,
  BookWorm,
  PaintItBlue,
}

pub const QUOTE_KINDS: [QuoteKind; 9] = [
  QuoteKind::Inglish,
  QuoteKind::IngotsForLife,
  QuoteKind::TheShieldsThatWood,
  QuoteKind::DontDrinkThose,
  QuoteKind::WhySoBlue,
  QuoteKind::GotYaCovered,
  QuoteKind::PapersPlease,
  QuoteKind::BookWorm,
  QuoteKind::PaintItBlue,
];
fn quote_assert_complete() {
  for q in QUOTE_KINDS {
    match q {
      // If one is missing here, make sure it exists in QUOTE_KINDS. This is a compile time check only.
      | QuoteKind::Inglish
      | QuoteKind::IngotsForLife
      | QuoteKind::TheShieldsThatWood
      | QuoteKind::DontDrinkThose
      | QuoteKind::WhySoBlue
      | QuoteKind::GotYaCovered
      | QuoteKind::PapersPlease
      | QuoteKind::BookWorm
      | QuoteKind::PaintItBlue
      // If one is missing here, make sure it exists in QUOTE_KINDS. This is a compile time check only.
      => {}
    }
  }
}

// Start with ore
// 100 ores: unlock ingots, bottle
// 100 ingots: unlock wood, shield
// 100 wooden shields: unlock blue, dye, blue shield




pub fn quote_get(config: &Config, kind: QuoteKind) -> Quote {
  match kind {
    | QuoteKind::Inglish
    | _
    =>
    Quote {
      kind,
      wants: vec!( ( part_kind_to_icon(config, 1), 10, 0 ) ),
      unlocks_parts: vec!(part_kind_to_icon(config, 1)),
      unlocks_quotes: vec!(QuoteKind::IngotsForLife),
      unlock_conditions: vec!(),
      added_at: 0,
      completed_at: 0,
    },
  }
}

