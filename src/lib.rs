extern crate base64;
extern crate openssl;
extern crate rand;

#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Read;

mod convert;
mod crack;
mod crypto;

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
  println!("{}", crack::crack_single_xor(&convert::from_hex(c)).1);
}

#[test]
fn s1_c4() {
  let mut file = File::open("s1_c4").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let mut decoded: Vec<(u64, String)> = vec![];
  for line in contents.lines() {
    let (_, m) = crack::crack_single_xor(&convert::from_hex(line));
    let score = crack::score_str(&m);
    decoded.push((score, m));
  }
  let &(_, ref m) = decoded
    .iter()
    .max_by(|&&(s1, _), &&(s2, _)| s1.partial_cmp(&s2).unwrap())
    .unwrap();
  println!("{}", m);
}

#[test]
fn s1_c5() {
  let msg = String::from("Burning 'em, if you ain't quick and nimble\n\
  I go crazy when I hear a cymbal")
      .into_bytes();
  let key = String::from("ICE").into_bytes();
  let c = convert::to_hex(&crypto::repeating_xor(&msg, &key));
  let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a\
                  26226324272765272\
                  a282b2f20430a652e2c652a3124333a653e2b2027630c692b202831652\
                  86326302e27282f";
  assert_eq!(c, expected);
}

#[test]
fn test_hamming() {
  assert_eq!(crypto::hamming_dist(&String::from("this is a test")
                                     .into_bytes(),
                                  &String::from("wokka wokka!!!")
                                     .into_bytes()),
             37);
}

#[test]
fn s1_c6() {
  let lines: Vec<&str> = include_str!("../data/s1_c6").lines().collect();
  let bytes: Vec<u8> = convert::from_base64(&lines.join(""));
  let (_, m) = crack::crack_repeating_xor(&bytes);
  println!("{}", m);
}

#[test]
fn s1_c7() {
  let lines: Vec<&str> = include_str!("../data/s1_c7").lines().collect();
  let bytes: Vec<u8> = convert::from_base64(&lines.join(""));
  let m = convert::to_text(&crypto::decrypt_aes_128_ecb(&bytes,
                                                        b"YELLOW SUBMARINE"));
  println!("{}", m);
}

#[test]
fn test_ecb_decrypt() {
  let bytes = &[9, 18, 48, 170, 222, 62, 179, 48, 219, 170, 67, 88, 248, 141,
                42, 108];
  println!("{:?}", convert::to_hex(bytes));
  let key = b"YELLOW SUBMARINE";
  println!("{:?}",
           convert::to_text(&crypto::decrypt_aes_128_ecb(bytes, key)));
}

#[test]
fn s1_c8() {
  use std::collections::HashSet;
  let lines: Vec<&str> = include_str!("../data/s1_c8").lines().collect();
  let bytelines: Vec<Vec<u8>> =
    lines.iter().map(|line| convert::from_hex(&line)).collect();
  for bytes in bytelines {
    let mut seen: HashSet<&[u8]> = HashSet::new();
    for chunk in bytes.chunks(16) {
      if seen.contains(chunk) {
        println!("{}", convert::to_hex(&bytes));
      }
      seen.insert(chunk);
    }
  }
}

#[test]
fn s2_c9() {
  assert_eq!(crypto::pkcs_pad(b"YELLOW SUBMARINE", 20),
             b"YELLOW SUBMARINE\x04\x04\x04\x04");
}

#[test]
fn s2_c10() {
  let lines: Vec<&str> = include_str!("../data/s2_c10").lines().collect();
  let bytes: Vec<u8> = convert::from_base64(&lines.join(""));
  let key = b"YELLOW SUBMARINE";
  let iv = &[0; 16];
  let msg = crypto::decrypt_aes_128_cbc(&bytes, key, iv);
  println!("{}", convert::to_text(&msg));
  assert_eq!(bytes, crypto::encrypt_aes_128_cbc(&msg, key, iv));
}

#[test]
fn s2_c11() {
  let mut input = vec![];
  for _ in 1..8 {
    input.extend_from_slice(b"ABCDEFG ABCDEFG ");
  }
  for _ in 1..100 {
    let (expected, cipher) = crypto::rand_encrypt(&input);
    let actual = crypto::encryption_oracle(&cipher);
    assert_eq!(expected, actual);
  }
}
