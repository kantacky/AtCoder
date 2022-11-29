//
// (C) 2022 Kanta Oikawa
//
// abc001 - c.rs
//

use std::io;

pub fn main() {
  let mut s: String = String::new();
  io::stdin().read_line(&mut s).ok();

  let vec: Vec<&str> = s.split_whitespace().collect();

  let deg: u16 = vec[0].parse().ok().unwrap();
  let dis: u16 = vec[1].parse().ok().unwrap();

  let dir_s: String = String::from(match deg {
    113..=337 => "NNE",
    338..=562 => "NE",
    563..=787 => "ENE",
    788..=1012 => "E",
    1013..=1237 => "ESE",
    1238..=1462 => "SE",
    1463..=1687 => "SSE",
    1688..=1912 => "S",
    1913..=2137 => "SSW",
    2138..=2362 => "SW",
    2363..=2587 => "WSW",
    2588..=2812 => "W",
    2813..=3037 => "WNW",
    3038..=3262 => "NW",
    3263..=3487 => "NNW",
    _ => "N",
  });

  let new_dis: u16 = (dis as f32 / 60 as f32 * 10 as f32).round() as u16;

  let w: u16 = match new_dis {
    0..=2 => 0,
    3..=15 => 1,
    16..=33 => 2,
    34..=54 => 3,
    55..=79 => 4,
    80..=107 => 5,
    108..=138 => 6,
    139..=171 => 7,
    172..=207 => 8,
    208..=244 => 9,
    245..=284 => 10,
    285..=326 => 11,
    _ => 12,
  };

  if w == 0 as u16 {
    println!("C 0");
  } else {
    println!("{} {}", dir_s, w);
  }
}
