#[cfg(test)]
use base64;

#[cfg(test)]
fn from_hex_char(c: char) -> u8 {
  match c {
    '0'...'9' => (c as u8 - '0' as u8),
    'a'...'f' => (c as u8 - 'a' as u8 + 10),
    'A'...'F' => (c as u8 - 'A' as u8 + 10),
    _ => panic!("Invalid hex char: {}", c),
  }
}

#[cfg(test)]
pub fn from_hex(s: &str) -> Vec<u8> {
  let mut result = Vec::new();
  let mut chars = s.chars();
  loop {
    match (chars.next(), chars.next()) {
      (Some(a), Some(b)) => {
        println!("{}{}", a, b);
        let x = from_hex_char(a);
        let y = from_hex_char(b);
        result.push((x << 4) | y);
      }
      (Some(_), None) => panic!("Invalid string from_hex"),
      _ => break,
    }
  }
  println!("{}", result.len());
  result
}

#[cfg(test)]
pub fn to_base64(b: Vec<u8>) -> String {
  base64::encode(&b)
}
