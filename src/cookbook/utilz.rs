use rand::prelude::*;

pub fn reverse(pair: (i64, bool)) -> (bool, i64) {
  let (x, y) = pair;
  (y, x)
}

pub fn boolean_array(count: usize) -> Vec<bool> {
  vec![true; count]
}

/**
 * Generate a random UUID (Universally Unique Identifier)
 * This function generates a random UUID using the rand crate.
 * It generates a 36-character string in the format XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX,
 * where X represents a hexadecimal digit.
*/
#[allow(unused_parens)]
pub fn generate_uuid() -> String {
  let mut rng: ThreadRng = rand::thread_rng();
  let mut uuid: String = String::new();
  for itr in 0..32 {
    if (itr == 8 || itr == 12 || itr == 16 || itr == 20) { uuid += "-"; }
    let c: u8 = rng.gen_range(0..=16);
    let ch: char = match c {
      0..=9 => (c + 48) as char,
      10..=16 => (c + 55) as char,
      _ => continue,
    };
    uuid = format!("{}{}", uuid, ch);
  }
  uuid
}

// pub fn timestamp() -> DateTime {
//   Local::now()
// }