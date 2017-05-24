#[cfg(test)]
pub fn repeating_xor(msg: &[u8], key: &[u8]) -> Vec<u8> {
  key.iter().cycle().zip(msg).map(|(x, y)| x ^ y).collect()
}

#[cfg(test)]
pub fn hamming_dist(a: &[u8], b: &[u8]) -> u32 {
  fn count_bits(x: u8) -> u32 {
    (0..8).map(|i| ((x as u32) >> i) & 1).sum()
  }

  a.iter().zip(b).map(|(x, y)| count_bits(x ^ y)).sum()
}
