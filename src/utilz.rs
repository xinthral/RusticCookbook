
pub fn reverse(pair: (i64, bool)) -> (bool, i64) {
  let (x, y) = pair;
  (y, x)
}

pub fn boolean_array(count: usize) -> Vec<bool> {
  vec![true; count]
}