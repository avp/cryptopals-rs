extern crate base64;

#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Read;

mod convert;
mod crack;

#[test]
fn s1_c1() {
  let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b652061\
               20706f69736f6e6f7573206d757368726f6f6d";
  let b64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
  assert_eq!(convert::to_base64(&convert::from_hex(hex)), b64);
}

#[test]
fn s1_c2() {
  let s1 = "1c0111001f010100061a024b53535009181c";
  let s2 = "686974207468652062756c6c277320657965";
  let res = "746865206b696420646f6e277420706c6179";
  let xor = convert::xor_bytes(&convert::from_hex(s1), &convert::from_hex(s2));
  assert_eq!(convert::to_hex(&xor), res);
}

#[test]
fn s1_c3() {
  let c = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
  println!("{}", crack::crack_single_xor(&convert::from_hex(c)));
}

#[test]
fn s1_c4() {
  let mut file = File::open("s1_c4").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let mut decoded: Vec<(f64, String)> = vec![];
  for line in contents.lines() {
    let m = crack::crack_single_xor(&convert::from_hex(line));
    let score = crack::score_str(&m);
    decoded.push((score, m));
  }
  let &(_, ref m) = decoded
    .iter()
    .max_by(|&&(s1, _), &&(s2, _)| s1.partial_cmp(&s2).unwrap())
    .unwrap();
  println!("{}", m);
}
