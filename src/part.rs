use super::config::*;
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
pub fn part_to_sprite_coord_from_config(config: &Config, kind: PartKind) -> ( f64, f64, f64, f64, &web_sys::HtmlImageElement ) {
  let x = config.map.get(part_kind_to_icon(kind).to_string().as_str());
  if let Some(&y) = x {
    let node = &config.nodes[y];
    return ( node.x as f64, node.y as f64, node.w as f64, node.h as f64, &config.sprite_cache_canvas[node.file_canvas_cache_index] );
  }

  panic!("noppes: part={:?}={}", kind, part_kind_to_icon(kind).to_string().as_str());
}
