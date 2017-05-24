#[cfg(test)]
pub fn repeating_xor(msg: &[u8], key: &[u8]) -> Vec<u8> {
  key.iter().cycle().zip(msg).map(|(x, y)| x ^ y).collect()
}
