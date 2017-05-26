#[cfg(test)]
use convert;
#[cfg(test)]
use crypto;
#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
fn char_freqs() -> HashMap<char, u64> {
  vec![('a', 651738),
       ('b', 124248),
       ('c', 217339),
       ('d', 349835),
       ('e', 1041442),
       ('f', 197881),
       ('g', 158610),
       ('h', 492888),
       ('i', 558094),
       ('j', 9033),
       ('k', 50529),
       ('l', 331490),
       ('m', 202124),
       ('n', 564513),
       ('o', 596302),
       ('p', 137645),
       ('q', 8606),
       ('r', 497563),
       ('s', 515760),
       ('t', 729357),
       ('u', 225134),
       ('v', 82903),
       ('w', 171272),
       ('x', 13692),
       ('y', 145984),
       ('z', 7836),
       (' ', 1918182)]
      .into_iter()
      .collect()
}

#[cfg(test)]
pub fn score_str(string: &str) -> u64 {
  let freqs: HashMap<char, u64> = char_freqs();
  let mut sum: u64 = 0;
  for c in string.chars() {
    let ch = (c as char).to_lowercase().next().unwrap();
    if let Some(freq) = freqs.get(&ch) {
      sum += *freq;
    }
  }
  sum
}

#[cfg(test)]
pub fn crack_single_xor(bytes: &[u8]) -> (u8, String) {
  let mut max = 0u64;
  let mut max_str = String::new();
  let mut max_k: u8 = 0;
  let mut key = vec![0; bytes.len()];
  for k in 0x00..0xff {
    for e in &mut key {
      *e = k;
    }
    let mb = convert::xor_bytes(&key, bytes);
    let m: String = convert::to_text(&mb);
    let score = score_str(&m);
    if score > max {
      max_str = m;
      max_k = k;
      max = score;
    }
  }
  (max_k, max_str)
}

#[cfg(test)]
pub fn crack_repeating_xor(bytes: &[u8]) -> (Vec<u8>, String) {
  let keysizes: Vec<usize> = (2..41).collect();
  let mut hamming: Vec<(usize, f64)> = keysizes
    .iter()
    .map(|&k| {
      let chunk1 = &bytes[0 * k..1 * k];
      let chunk2 = &bytes[1 * k..2 * k];
      let chunk3 = &bytes[2 * k..3 * k];
      let chunk4 = &bytes[3 * k..4 * k];
      let result = (crypto::hamming_dist(chunk1, chunk2) as f64 / k as f64 +
                    crypto::hamming_dist(chunk2, chunk3) as f64 / k as f64 +
                    crypto::hamming_dist(chunk3, chunk4) as f64 / k as f64) /
                   3.0;
      (k, result)
    })
    .collect();
  hamming.sort_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap());

  let mut max_str = String::new();
  let mut max_key = vec![];
  let mut max_score = 0u64;

  for &(keysize, _) in hamming.iter().take(5) {
    // Transposed blocks.
    let mut blocks: Vec<Vec<u8>> = vec![vec![]; keysize];
    for chunk in bytes.chunks(keysize) {
      for (i, &c) in chunk.iter().enumerate() {
        blocks[i].push(c);
      }
    }

    let key: Vec<u8> = blocks
      .iter()
      .map(|b| {
             let (k, _) = crack_single_xor(&b);
             k
           })
      .collect();

    let m = convert::to_text(&crypto::repeating_xor(bytes, &key));
    let score = score_str(&m);
    if score > max_score {
      max_score = score;
      max_str = m;
      max_key = key;
    }
  }
  (max_key, max_str)
}
