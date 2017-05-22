extern crate base64;

mod convert;

#[cfg(test)]
mod tests {
  use convert;
  #[test]
  fn s1_c1() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b652061\
               20706f69736f6e6f7573206d757368726f6f6d";
    let b64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(convert::to_base64(convert::from_hex(hex)), b64);
  }
}
