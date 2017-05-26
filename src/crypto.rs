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


#[cfg(test)]
pub fn decrypt_aes_128_ecb(cipher: &[u8], key: &[u8]) -> Vec<u8> {
  use openssl::symm::*;

  let mut crypter =
    Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key, None).unwrap();
  crypter.pad(false);
  let mut result = vec![0; cipher.len() * 8];
  let len = crypter.update(cipher, result.as_mut_slice()).unwrap();

  result[..len].to_vec()
}

#[cfg(test)]
pub fn encrypt_aes_128_ecb(msg: &[u8], key: &[u8]) -> Vec<u8> {
  use openssl::symm::*;

  let mut crypter =
    Crypter::new(Cipher::aes_128_ecb(), Mode::Encrypt, key, None).unwrap();
  crypter.pad(false);
  let mut result = vec![0; msg.len() * 32];
  let len = crypter.update(msg, result.as_mut_slice()).unwrap();

  result[..len].to_vec()
}

#[cfg(test)]
pub fn pkcs_pad(input: &[u8], len: u8) -> Vec<u8> {
  let mut result = input.to_vec();
  if (result.len() as u8) < len {
    let value: u8 = len - result.len() as u8;
    while (result.len() as u8) < len {
      result.push(value);
    }
  }
  result
}

#[cfg(test)]
pub fn encrypt_aes_128_cbc(msg: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
  use convert;

  let mut result: Vec<u8> = vec![];

  let mut prev = iv.to_vec();
  for block in msg.chunks(16) {
    let xored = convert::xor_bytes(block, &prev);
    let encrypted = encrypt_aes_128_ecb(&xored, key);
    result.extend_from_slice(&encrypted);
    prev = encrypted;
  }

  result
}
#[cfg(test)]
pub fn decrypt_aes_128_cbc(cipher: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
  use convert;

  let mut result: Vec<u8> = vec![];

  let mut prev = iv;
  for block in cipher.chunks(16) {
    let decrypted = decrypt_aes_128_ecb(block, key);
    result.extend(&convert::xor_bytes(&decrypted, &prev));
    prev = block;
  }

  result
}

#[cfg(test)]
fn rand_key(len: usize) -> Vec<u8> {
  use rand;
  use rand::Rng;
  let mut rng = rand::thread_rng();
  (0..len).map(|_| rng.gen::<u8>()).collect()
}

#[cfg(test)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AESMode {
  ECB,
  CBC,
}

#[cfg(test)]
pub fn rand_encrypt(input: &[u8]) -> (AESMode, Vec<u8>) {
  use rand;
  use rand::Rng;
  let mut rng = rand::thread_rng();
  let key = rand_key(16);
  let mode = if rng.gen() {
    AESMode::ECB
  } else {
    AESMode::CBC
  };

  let pad_len = rng.gen_range(5, 11);
  let mut padded: Vec<u8> = vec![];
  padded
    .extend_from_slice(&(0..pad_len).map(|_| rng.gen()).collect::<Vec<_>>());
  padded.extend_from_slice(input);
  padded
    .extend_from_slice(&(0..pad_len).map(|_| rng.gen()).collect::<Vec<_>>());

  (mode,
   match mode {
     AESMode::ECB => encrypt_aes_128_ecb(&padded, &key),
     AESMode::CBC => encrypt_aes_128_cbc(&padded, &key, &rand_key(16)),
   })
}

#[cfg(test)]
pub fn encryption_oracle(cipher: &[u8]) -> AESMode {
  let mut blocks: Vec<&[u8]> = cipher.chunks(16).collect();
  let total = blocks.len();
  blocks.sort();
  blocks.dedup();
  if blocks.len() != total {
    AESMode::ECB
  } else {
    AESMode::CBC
  }
}
