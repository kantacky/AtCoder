//
// (C) 2022 Kanta Oikawa
//
// abc001 - b.rs
//

use std::io;

pub fn main() {
  let mut m_s: String = String::new();
  io::stdin().read_line(&mut m_s).ok();
  let m: u32 = m_s.trim().parse().ok().unwrap();

  let vv: u8 = match m {
    100..=5000 => (m as f32 / 1000_f32 * 10_f32) as u8,
    6000..=30000 => (m as f32 / 1000_f32 + 50_f32) as u8,
    35000..=70000 => ((m as f32 / 1000_f32 - 30_f32) / 5_f32 + 80_f32) as u8,
    70001..=100000 => 89,
    _ => 0,
  };

  let vv_s: String = vv.to_string();

  println!("{:0>2}", vv_s);
}
