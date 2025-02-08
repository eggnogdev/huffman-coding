#[derive(Debug)]
pub struct CharCodePair {
  pub value: char,
  pub bits: u8,
  pub code: u8
}

impl CharCodePair {
  pub fn new(value: char, bits: u8, code: u8) -> CharCodePair {
    return CharCodePair {
      value,
      bits,
      code,
    }
  }
}