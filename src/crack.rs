#[cfg(test)]
use convert;
#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
fn char_freqs() -> HashMap<char, f64> {
  vec![('a', 0.0651738),
       ('b', 0.0124248),
       ('c', 0.0217339),
       ('d', 0.0349835),
       ('e', 0.1041442),
       ('f', 0.0197881),
       ('g', 0.0158610),
       ('h', 0.0492888),
       ('i', 0.0558094),
       ('j', 0.0009033),
       ('k', 0.0050529),
       ('l', 0.0331490),
       ('m', 0.0202124),
       ('n', 0.0564513),
       ('o', 0.0596302),
       ('p', 0.0137645),
       ('q', 0.0008606),
       ('r', 0.0497563),
       ('s', 0.0515760),
       ('t', 0.0729357),
       ('u', 0.0225134),
       ('v', 0.0082903),
       ('w', 0.0171272),
       ('x', 0.0013692),
       ('y', 0.0145984),
       ('z', 0.0007836),
       (' ', 0.1918182)]
      .into_iter()
      .collect()
}

#[cfg(test)]
fn score_str(string: &str) -> f64 {
  let freqs: HashMap<char, f64> = char_freqs();
  let mut sum: f64 = 0.0;
  for c in string.chars() {
    let ch = (c as char).to_lowercase().next().unwrap();
    if let Some(freq) = freqs.get(&ch) {
      sum += *freq;
    }
  }
  sum
}

#[cfg(test)]
pub fn crack_single_xor(bytes: &[u8]) -> String {
  let mut max = 0.0;
  let mut max_str = String::new();
  let mut key = vec![0; bytes.len()+1];
  for k in 0x00..0xff {
    for i in 0..key.len() - 1 {
      key[i] = k;
    }
    let mb = convert::xor_bytes(&key, bytes);
    let m: String = convert::to_text(&mb);
    let score = score_str(&m);
    if score > max {
      max_str = m;
      max = score;
    }
  }
  return max_str;
}
