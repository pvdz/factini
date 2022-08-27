// A Quote is a build order to complete. They are like achievements, quests, or build requests.
// Each Quote should have one or more things to complete and a reward upon completion, usually
// an unlock step of sorts, as well as the next Quote or set of quotes that it unlocks.

use super::part::*;

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



pub fn quote_get(kind: QuoteKind) -> Quote {
  match kind {
    QuoteKind::Inglish => Quote {
      kind,
      wants: vec!( ( part_kind_to_icon(PartKind::DirtWhite), 10, 0 ) ),
      unlocks_parts: vec!(part_kind_to_icon(PartKind::IngotWhite), part_kind_to_icon(PartKind::EmptyBottle), part_kind_to_icon(PartKind::Rope)),
      unlocks_quotes: vec!(QuoteKind::IngotsForLife),
      unlock_conditions: vec!(),
      added_at: 0,
      completed_at: 0,
    },
    QuoteKind::IngotsForLife => Quote {
      kind,
      wants: vec!( ( part_kind_to_icon(PartKind::IngotWhite), 100, 0 ) ),
      unlocks_parts: vec!(part_kind_to_icon(PartKind::Wood), part_kind_to_icon(PartKind::ShieldWood), part_kind_to_icon(PartKind::DirtBlue), part_kind_to_icon(PartKind::Paper)),
      unlocks_quotes: vec!(QuoteKind::TheShieldsThatWood, QuoteKind::DontDrinkThose),
      unlock_conditions: vec!(),
      added_at: 0,
      completed_at: 0,
    },
    QuoteKind::PapersPlease => Quote {
      kind,
      wants: vec!( ( part_kind_to_icon(PartKind::Paper), 100, 0 ) ),
      unlocks_parts: vec!(part_kind_to_icon(PartKind::BookWhite), part_kind_to_icon(PartKind::PotionBlue)),
      unlocks_quotes: vec!(QuoteKind::BookWorm),
      unlock_conditions: vec!(),
      added_at: 0,
      completed_at: 0,
    },
    QuoteKind::BookWorm => Quote {
      kind,
      wants: vec!( ( part_kind_to_icon(PartKind::BookWhite), 100, 0 ) ),
      unlocks_parts: vec!(part_kind_to_icon(PartKind::BookBlue)),
      unlocks_quotes: vec!(QuoteKind::PaintItBlue),
      unlock_conditions: vec!(),
      added_at: 0,
      completed_at: 0,
    },
    QuoteKind::PaintItBlue => Quote {
      kind,
      wants: vec!( ( part_kind_to_icon(PartKind::BookBlue), 100, 0 ) ),
      unlocks_parts: vec!(part_kind_to_icon(PartKind::BookShield)),
      unlocks_quotes: vec!(QuoteKind::GotYaCovered),
      unlock_conditions: vec!(QuoteKind::WhySoBlue),
      added_at: 0,
      completed_at: 0,
    },
    QuoteKind::TheShieldsThatWood => Quote {
      kind,
      wants: vec!( ( part_kind_to_icon(PartKind::ShieldWood), 100, 0 ) ),
      unlocks_parts: vec!(part_kind_to_icon(PartKind::ShieldBlue)),
      unlocks_quotes: vec!(QuoteKind::WhySoBlue),
      unlock_conditions: vec!(QuoteKind::DontDrinkThose),
      added_at: 0,
      completed_at: 0,
    },
    QuoteKind::DontDrinkThose => Quote {
      kind,
      wants: vec!( ( part_kind_to_icon(PartKind::PotionBlue), 100, 0 ) ),
      unlocks_parts: vec!(part_kind_to_icon(PartKind::ShieldBlue)),
      unlocks_quotes: vec!(QuoteKind::WhySoBlue),
      unlock_conditions: vec!(QuoteKind::TheShieldsThatWood),
      added_at: 0,
      completed_at: 0,
    },
    QuoteKind::WhySoBlue => Quote {
      kind,
      wants: vec!( ( part_kind_to_icon(PartKind::ShieldBlue), 100, 0 ) ),
      unlocks_parts: vec!(part_kind_to_icon(PartKind::BookShield)),
      unlocks_quotes: vec!(QuoteKind::GotYaCovered),
      unlock_conditions: vec!(QuoteKind::PaintItBlue),
      added_at: 0,
      completed_at: 0,
    },
    QuoteKind::GotYaCovered => Quote {
      kind,
      wants: vec!( ( part_kind_to_icon(PartKind::BookShield), 100, 0 ) ),
      unlocks_parts: vec!(),
      unlocks_quotes: vec!(),
      unlock_conditions: vec!(),
      added_at: 0,
      completed_at: 0,
    },
  }
}

