//
// (C) 2022 Kanta Oikawa
//
// abc001 - a.rs
//

use std::io;

pub fn main() {
  let mut s1: String = String::new();
  let mut s2: String = String::new();

  io::stdin().read_line(&mut s1).ok();
  io::stdin().read_line(&mut s2).ok();

  let h1: i16 = s1.trim().parse().ok().unwrap();
  let h2: i16 = s2.trim().parse().ok().unwrap();

  println!("{}", h1 - h2);
}
