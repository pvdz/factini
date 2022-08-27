use super::utils::*;

#[derive(Clone, Debug)]
pub struct Part {
  pub kind: PartKind,
  pub icon: char,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PartKind {
  None,

  BlueWand,
  BookGreen,
  BookRed,
  BookBlue,
  BookWhite,
  BookBrown,
  BookHeart, // Red with heart
  BookPurple,
  BookYellow,
  BookBlack,
  BookSkull,
  BookShield, // Blue with shield
  EmptyBottle, // The "transparent potion"
  GoldDust,
  GoldenBlueWand,
  IngotBabyBlue, // The faded dark blue
  IngotGrey,
  IngotLawnGreen, // The slime green
  IngotOrange,
  IngotTurquoise, // The light blue
  IngotWhite,
  MaterialTurquoise, // The raw pile of dirt and pixels
  Paper,
  Parchment,
  PotionWhite,
  PotionBlack,
  PotionPurple,
  PotionGreen,
  PotionRed,
  PotionBlue,
  PotionBrown,
  PotionTurquoise,
  PotionYellow,
  PotionOrange,
  Sapphire,
  Trash, // Pseudo special case
  WoodenStick,
  DirtTurquoise,
  DirtBlue,
  DirtWhite,
  Rope,
  Ruby,
  Wood,
  ShieldWood,
  ShieldBlue,
}

pub const PARTS: [PartKind; 46] = [
  PartKind::None,

  PartKind::BlueWand,
  PartKind::BookGreen,
  PartKind::BookRed,
  PartKind::BookBlue,
  PartKind::BookWhite,
  PartKind::BookBrown,
  PartKind::BookHeart, // Red with heart
  PartKind::BookPurple,
  PartKind::BookYellow,
  PartKind::BookBlack,
  PartKind::BookSkull,
  PartKind::BookShield, // Blue with shield
  PartKind::EmptyBottle, // The "transparent potion"
  PartKind::GoldDust,
  PartKind::GoldenBlueWand,
  PartKind::IngotBabyBlue, // The faded dark blue
  PartKind::IngotGrey,
  PartKind::IngotLawnGreen, // The slime green
  PartKind::IngotOrange,
  PartKind::IngotTurquoise, // The light blue
  PartKind::IngotWhite,
  PartKind::MaterialTurquoise, // The raw pile of dirt and pixels
  PartKind::Paper,
  PartKind::Parchment,
  PartKind::PotionWhite,
  PartKind::PotionBlack,
  PartKind::PotionPurple,
  PartKind::PotionGreen,
  PartKind::PotionRed,
  PartKind::PotionBlue,
  PartKind::PotionBrown,
  PartKind::PotionTurquoise,
  PartKind::PotionYellow,
  PartKind::PotionOrange,
  PartKind::Sapphire,
  PartKind::Trash, // Pseudo special case
  PartKind::WoodenStick,
  PartKind::DirtTurquoise,
  PartKind::DirtBlue,
  PartKind::DirtWhite,
  PartKind::Rope,
  PartKind::Ruby,
  PartKind::Wood,
  PartKind::ShieldWood,
  PartKind::ShieldBlue,
];

fn part_assert_complete() {
  for p in PARTS {
    match p {
      // If you add something here, add it to PARTS too
      | PartKind::None
      | PartKind::BlueWand
      | PartKind::BookGreen
      | PartKind::BookRed
      | PartKind::BookBlue
      | PartKind::BookWhite
      | PartKind::BookBrown
      | PartKind::BookHeart
      | PartKind::BookPurple
      | PartKind::BookYellow
      | PartKind::BookBlack
      | PartKind::BookSkull
      | PartKind::BookShield
      | PartKind::EmptyBottle
      | PartKind::GoldDust
      | PartKind::GoldenBlueWand
      | PartKind::IngotBabyBlue
      | PartKind::IngotGrey
      | PartKind::IngotLawnGreen
      | PartKind::IngotOrange
      | PartKind::IngotTurquoise
      | PartKind::IngotWhite
      | PartKind::MaterialTurquoise
      | PartKind::Paper
      | PartKind::Parchment
      | PartKind::PotionWhite
      | PartKind::PotionBlack
      | PartKind::PotionPurple
      | PartKind::PotionGreen
      | PartKind::PotionRed
      | PartKind::PotionBlue
      | PartKind::PotionBrown
      | PartKind::PotionTurquoise
      | PartKind::PotionYellow
      | PartKind::PotionOrange
      | PartKind::Sapphire
      | PartKind::Trash
      | PartKind::WoodenStick
      | PartKind::DirtTurquoise
      | PartKind::DirtBlue
      | PartKind::DirtWhite
      | PartKind::Rope
      | PartKind::Ruby
      | PartKind::Wood
      | PartKind::ShieldWood
      | PartKind::ShieldBlue
      // If you add something here, add it to PARTS too
      => {}
    }
  }
}


pub const fn part_none() -> Part {
  return Part {
    kind: part_icon_to_kind(' '),
    icon: ' ',
  }
}

pub const fn part_c(icon: char) -> Part {
  return Part {
    kind: part_icon_to_kind(icon),
    icon,
  }
}

// h e -> W
// i p j -> D
// D W -> C
// n -> Q
// k Q -> l
// W l -> o
// C o -> K

// blue-dirt + empty-potion -> blue-potion
// rope + paper -> white-book
// white-book + blue-potion -> blue-book
// white-dirt -> white-ingot
// wood + white-ingot -> shield
// blue-potion + shield -> blue-shield
// blue-book + blue-shield -> book-shield

// "achievements" to finish. like 100 bottles of blue in one day
// achievements unlock new achievements with new craftables
// should we show craftables in a grid to the right, like the unlock tree
// could use this to show the craft config to have a machine create a certain thing
// (should we just show that inside the machine UI, anyways?
// Offers become machines and special buildings etc

pub const fn part_icon_to_kind(c: char) -> PartKind {
  match c {
    'w' => PartKind::WoodenStick,
    'b' => PartKind::BlueWand,
    'e' => PartKind::EmptyBottle,
    'd' => PartKind::GoldDust,
    'g' => PartKind::GoldenBlueWand,
    'm' => PartKind::MaterialTurquoise,
    's' => PartKind::Sapphire,
    't' => PartKind::Trash,
    'p' => PartKind::Paper,
    'q' => PartKind::Parchment,
    'h' => PartKind::DirtTurquoise,
    'i' => PartKind::Rope,
    'j' => PartKind::Ruby,
    'k' => PartKind::Wood,
    'l' => PartKind::ShieldWood,
    'n' => PartKind::DirtWhite,
    'o' => PartKind::ShieldBlue,
    'r' => PartKind::DirtBlue,

    'A' => PartKind::BookGreen,
    'B' => PartKind::BookRed,
    'C' => PartKind::BookBlue,
    'D' => PartKind::BookWhite,
    'E' => PartKind::BookBrown,
    'F' => PartKind::BookHeart, // Red with heart
    'G' => PartKind::BookPurple,
    'H' => PartKind::BookYellow,
    'I' => PartKind::BookBlack,
    'J' => PartKind::BookSkull,
    'K' => PartKind::BookShield, // Blue with shield
    'L' => PartKind::IngotBabyBlue, // The faded dark blue
    'M' => PartKind::IngotGrey,
    'N' => PartKind::IngotLawnGreen, // The slime green
    'O' => PartKind::IngotOrange,
    'P' => PartKind::IngotTurquoise, // The light blue
    'Q' => PartKind::IngotWhite,
    'R' => PartKind::PotionWhite,
    'S' => PartKind::PotionBlack,
    'T' => PartKind::PotionPurple,
    'U' => PartKind::PotionGreen,
    'V' => PartKind::PotionRed,
    'W' => PartKind::PotionBlue,
    'X' => PartKind::PotionBrown,
    'Y' => PartKind::PotionTurquoise,
    'Z' => PartKind::PotionYellow,
    'x' => PartKind::PotionOrange,

    ' ' => PartKind::None,
    _ => {
      PartKind::None
    },
  }
}
pub const fn part_kind_to_icon(kind: PartKind) -> char {
  match kind {
    PartKind::WoodenStick => 'w',
    PartKind::BlueWand => 'b',
    PartKind::EmptyBottle => 'e',
    PartKind::GoldDust => 'd',
    PartKind::GoldenBlueWand => 'g',
    PartKind::MaterialTurquoise => 'm',
    PartKind::Sapphire => 's',
    PartKind::Trash => 't',
    PartKind::Paper => 'p',
    PartKind::Parchment => 'q',
    PartKind::DirtTurquoise => 'h',
    PartKind::Rope => 'i',
    PartKind::Ruby => 'j',
    PartKind::Wood => 'k',
    PartKind::ShieldWood => 'l',
    PartKind::DirtWhite => 'n',
    PartKind::ShieldBlue => 'o',
    PartKind::DirtBlue => 'r',

    PartKind::BookGreen => 'A',
    PartKind::BookRed => 'B',
    PartKind::BookBlue => 'C',
    PartKind::BookWhite => 'D',
    PartKind::BookBrown => 'E',
    PartKind::BookHeart => 'F', // Red with heart
    PartKind::BookPurple => 'G',
    PartKind::BookYellow => 'H',
    PartKind::BookBlack => 'I',
    PartKind::BookSkull => 'J',
    PartKind::BookShield => 'K', // Blue with shield
    PartKind::IngotBabyBlue => 'L', // The faded dark blue
    PartKind::IngotGrey => 'M',
    PartKind::IngotLawnGreen => 'N', // The slime green
    PartKind::IngotOrange => 'O',
    PartKind::IngotTurquoise => 'P', // The light blue
    PartKind::IngotWhite => 'Q',
    PartKind::PotionWhite => 'R',
    PartKind::PotionBlack => 'S',
    PartKind::PotionPurple => 'T',
    PartKind::PotionGreen => 'U',
    PartKind::PotionRed => 'V',
    PartKind::PotionBlue => 'W',
    PartKind::PotionBrown => 'X',
    PartKind::PotionTurquoise => 'Y',
    PartKind::PotionYellow => 'Z',
    PartKind::PotionOrange => 'x',

    PartKind::None => ' ',
  }
}

pub const fn part_to_sprite_coord(kind: PartKind) -> ( f64, f64 ) {
  return match kind {
    PartKind::BlueWand => {
      // This is the slightly bigger blue wand
      (2.0, 11.0)
    },
    PartKind::GoldDust => {
      // Kinda like gold dust?
      (8.0, 3.0)
    },
    PartKind::GoldenBlueWand => {
      // This is the golden blue wand
      (4.0, 11.0)
    },
    PartKind::Sapphire => {
      // This is a sapphire
      (1.0, 3.0)
    },
    PartKind::Trash => {
      // This is something that looks like a grey rock
      (11.0, 10.0)
    },
    PartKind::WoodenStick => {
      // This is a club? Piece of wood I guess? From which wands are formed.
      (0.0, 11.0)
    },
    PartKind::Paper => {
      // The clean paper
      ( 12.0, 10.0 )
    },
    PartKind::Parchment => {
      // The old paper
      ( 9.0, 4.0 )
    },

    PartKind::DirtWhite => {
      ( 2.0, 4.0 )
    }
    PartKind::DirtTurquoise => {
      ( 5.0, 4.0 )
    }
    PartKind::DirtBlue => {
      ( 4.0, 4.0 )
    }
    PartKind::Rope => {
      ( 3.0, 6.0 )
    }
    PartKind::Ruby => {
      ( 3.0, 3.0 )
    }
    PartKind::Wood => {
      ( 2.0, 6.0 )
    }
    PartKind::ShieldWood => {
      ( 5.0, 11.0 )
    }
    PartKind::ShieldBlue => {
      ( 9.0, 11.0 )
    }

    PartKind::BookGreen => {
      ( 7.0, 12.0 )
    }
    PartKind::BookRed => {
      ( 8.0, 12.0 )
    }
    PartKind::BookBlue => {
      ( 9.0, 12.0 )
    }
    PartKind::BookWhite => {
      ( 10.0, 12.0 )
    }
    PartKind::BookBrown => {
      ( 11.0, 12.0 )
    }
    PartKind::BookHeart => {
      ( 12.0, 12.0 )
    }
    PartKind::BookPurple => {
      ( 7.0, 13.0 )
    }
    PartKind::BookYellow => {
      ( 8.0, 13.0 )
    }
    PartKind::BookBlack => {
      ( 10.0, 13.0 )
    }
    PartKind::BookSkull => {
      ( 11.0, 13.0 )
    }
    PartKind::BookShield => {
      ( 12.0, 13.0 )
    }
    PartKind::MaterialTurquoise => {
      ( 5.0, 4.0 )
    }
    PartKind::IngotBabyBlue => {
      ( 5.0, 5.0 )
    }
    PartKind::IngotGrey => {
      ( 6.0, 5.0 )
    }
    PartKind::IngotLawnGreen => {
      ( 3.0, 5.0 )
    }
    PartKind::IngotOrange => {
      ( 1.0, 5.0 )
    }
    PartKind::IngotTurquoise => {
      ( 5.0, 5.0 )
    }
    PartKind::IngotWhite => {
      ( 2.0, 5.0 )
    }
    PartKind::PotionWhite => {
      ( 7.0, 4.0 )
    }
    PartKind::PotionBlack => {
      ( 8.0, 4.0 )
    }
    PartKind::PotionPurple => {
      ( 9.0, 4.0 )
    }
    PartKind::PotionGreen => {
      ( 10.0, 4.0 )
    }
    PartKind::PotionRed => {
      ( 11.0, 4.0 )
    }
    PartKind::PotionBlue => {
      ( 12.0, 4.0 )
    }
    PartKind::PotionBrown => {
      ( 7.0, 5.0 )
    }
    PartKind::PotionTurquoise => {
      ( 8.0, 5.0 )
    }
    PartKind::PotionYellow => {
      ( 9.0, 5.0 )
    }
    PartKind::PotionOrange => {
      ( 10.0, 5.0 )
    }
    PartKind::EmptyBottle => {
      ( 11.0, 5.0 )
    }

    PartKind::None => {
      // Ignore, this belt segment or machine input is empty
      ( 0.123, 0.123 )
    },
  };
}

pub fn part_icon_to_recipe(c: char) -> String {
  // Note: empty strings are considered raw resources
  match c {
    'w' => "   .   .   .   .   .   .   .   .   .   ",
    'b' => "   .   .   .   .   .   .   .   .   .   ",
    'e' => "   .   .   .   .   .   .   .   .   .   ",
    'd' => "   .   .   .   .   .   .   .   .   .   ",
    'g' => "   .   .   .   .   .   .   .   .   .   ",
    'm' => "   .   .   .   .   .   .   .   .   .   ",
    's' => "   .   .   .   .   .   .   .   .   .   ",
    't' => "   .   .   .   .   .   .   .   .   .   ",
    'p' => "   .   .   .   .   .   .   .   .   .   ",
    'q' => "   .   .   .   .   .   .   .   .   .   ",
    'h' => "   .   .   .   .   .   .   .   .   .   ",
    'i' => "   .   .   .   .   .   .   .   .   .   ",
    'j' => "   .   .   .   .   .   .   .   .   .   ",
    'k' => "   .   .   .   .   .   .   .   .   .   ",
    'l' => "   .   Q   .   k   k   k   k   k   k   ",
    'n' => "   .   .   .   W   l   W   .   .   .   ",
    'o' => "   .   .   .   .   .   .   .   .   .   ",
    'r' => "   .   .   .   .   .   .   .   .   .   ",

    'A' => "   .   .   .   .   .   .   .   .   .   ",
    'B' => "   .   .   .   .   .   .   .   .   .   ",
    'C' => "   .   w   .   W   D   W   .   W   .   ",
    'D' => "   i   p   p   i   p   p   i   p   p   ",
    'E' => "   .   .   .   .   .   .   .   .   .   ",
    'F' => "   .   .   .   .   .   .   .   .   .   ",
    'G' => "   .   .   .   .   .   .   .   .   .   ",
    'H' => "   .   .   .   .   .   .   .   .   .   ",
    'I' => "   .   .   .   .   .   .   .   .   .   ",
    'J' => "   .   .   .   .   .   .   .   .   .   ",
    'K' => "   .   .   .   C   o   .   .   .   .   ",
    'L' => "   .   .   .   .   .   .   .   .   .   ",
    'M' => "   .   .   .   .   .   .   .   .   .   ",
    'N' => "   .   .   .   .   .   .   .   .   .   ",
    'O' => "   .   .   .   .   .   .   .   .   .   ",
    'P' => "   .   .   .   .   .   .   .   .   .   ",
    'Q' => "   .   .   .   n   n   n   n   n   n   ",
    'R' => "   .   .   .   .   .   .   .   .   .   ",
    'S' => "   .   .   .   .   .   .   .   .   .   ",
    'T' => "   .   .   .   .   .   .   .   .   .   ",
    'U' => "   .   .   .   .   .   .   .   .   .   ",
    'V' => "   .   .   .   .   .   .   .   .   .   ",
    'W' => "   .   r   .   .   e   .   .   r   .   ",
    'X' => "   .   .   .   .   .   .   .   .   .   ",
    'Y' => "   .   .   .   .   .   .   .   .   .   ",
    'Z' => "   .   .   .   .   .   .   .   .   .   ",
    'x' => "   .   .   .   .   .   .   .   .   .   ",

    ' ' => "   .   .   .   .   .   .   .   .   .   ",
    _ => {
      "   .   .   .   .   .   .   .   .   .   "
    },
  }.to_string()
}
