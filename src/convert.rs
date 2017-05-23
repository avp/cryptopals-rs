#[cfg(test)]
use base64;
#[cfg(test)]
use std::char;

#[cfg(test)]
fn from_hex_char(c: char) -> u8 {
  match c {
    '0'...'9' => (c as u8 - b'0'),
    'a'...'f' => (c as u8 - b'a' + 10),
    'A'...'F' => (c as u8 - b'A' + 10),
    _ => panic!("Invalid hex char: {}", c),
  }
}

#[cfg(test)]
fn to_hex_char(b: u8) -> char {
  char::from_digit(b as u32, 16).unwrap()
}

#[cfg(test)]
pub fn from_hex(s: &str) -> Vec<u8> {
  let mut result = Vec::new();
  let mut chars = s.chars();
  loop {
    match (chars.next(), chars.next()) {
      (Some(a), Some(b)) => {
        result.push((from_hex_char(a) << 4) | from_hex_char(b));
      }
      (Some(_), None) => panic!("Invalid string from_hex"),
      _ => break,
    }
  }
  result
}

#[cfg(test)]
pub fn to_hex(s: &[u8]) -> String {
  let mut result = String::new();
  for b in s {
    result.push(to_hex_char(b >> 4));
    result.push(to_hex_char(b & 0xf));
  }
  result
}

#[cfg(test)]
pub fn to_base64(b: &[u8]) -> String {
  base64::encode(&b)
}

#[cfg(test)]
pub fn to_text(bytes: &[u8]) -> String {
  let mut result = String::new();
  for b in bytes {
    result.push(*b as char);
  }
  result
}

#[cfg(test)]
pub fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
  let mut xor = vec![];
  for (x, y) in a.iter().zip(b) {
    xor.push(x ^ y);
  }
  xor
}
