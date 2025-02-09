use crate::char_code::CharCodePair;

const START_METADATA: u16 = 0b0000_0000_0000_0000;

const DICTIONARY_ENTRY: u16 = 0b0001_0000_0000_0000;
const MAX_DICTIONARY_ENTRY: u16 = 0b0001_1111_1111_1111;

const END_METADATA: u16 = 0b1111_1111_1111_1111;

const FIRST_BIT_0_U8: u8 = 0b0111_1111;
const FIRST_BIT_1_U8: u8 = 0b1000_0000;

const FIRST_BIT_0_U16: u16 = 0b0111_1111_1111_1111;
const FIRST_BIT_1_U16: u16 = 0b1000_0000_0000_0000;

const FIRST_BIT_0_U32: u32 = 0b0111_1111_1111_1111_1111_1111_1111_1111;
const FIRST_BIT_1_U32: u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000;

const FIRST_BIT_0_U64: u64 = 9223372036854775807;
const FIRST_BIT_1_U64: u64 = 9223372036854775808;

const LAST_BIT_1_U8: u8 = 0b0000_0001;

const LAST_BIT_0_U16: u16 = 0b1111_1111_1111_1110;
const LAST_BIT_1_U16: u16 = 0b0000_0000_0000_0001;

const LAST_BIT_0_U64: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110;
const LAST_BIT_1_U64: u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001;

const MID_BIT_0_U64: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1110_1111_1111_1111_1111_1111_1111_1111_1111;
const MID_BIT_1_U64: u64 = 0b0000_0000_0000_0000_0000_0000_0000_0001_0000_0000_0000_0000_0000_0000_0000_0000;

#[derive(Debug)]
pub struct MetadataKeyValuePair {
  key: u16,
  pub value: u64,
}

impl MetadataKeyValuePair {
  pub fn from_bytes(b: [u8; 10]) -> MetadataKeyValuePair {
    let mut key: u16 = 0;
    let mut value: u64 = 0;

    // write the key
    for byte in &b[0..2] {
      for i in 0..8 {
        if byte.rotate_left(i) & FIRST_BIT_1_U8 == FIRST_BIT_1_U8 {
          // current bit in the byte is 1, write 1 to key
          key |= FIRST_BIT_1_U16;
        } else {
          // current bit in the byte is 0, write 0 to key
          key &= FIRST_BIT_0_U16;
        }

        // rotate the key bits to edit the next one
        key = key.rotate_left(1);
      }
    }

    // write the value
    for byte in &b[2..] {
      for i in 0..8 {
        if byte.rotate_left(i) & FIRST_BIT_1_U8 == FIRST_BIT_1_U8 {
          // current bit in the byte is 1, write 1 to key
          value |= FIRST_BIT_1_U64;
        } else {
          // current bit in the byte is 0, write 0 to key
          value &= FIRST_BIT_0_U64;
        }

        // rotate the key bits to edit the next one
        value = value.rotate_left(1);
      }
    }

    return MetadataKeyValuePair { key, value };
  }

  pub fn as_bytes(&self) -> [u8; 10] {
    let mut result: [u8; 10] = [0; 10];

    let mut current_bit: u8 = 0;
    let mut key = self.key;
    let mut value = self.value;
    while current_bit < 80 {
      let current_byte_index = (current_bit / 8) as usize;
      let current_byte = &mut result[current_byte_index];

      if current_bit < 16 {
        // writing the key bits
        if key & FIRST_BIT_1_U16 == FIRST_BIT_1_U16 {
          // current bit in key is 1, write 1 to current byte
          *current_byte |= FIRST_BIT_1_U8;
        } else {
          // current bit in key is 0, write 0 to current byte
          *current_byte &= FIRST_BIT_0_U8;
        }

        key = key.rotate_left(1);
      } else {
        // writing the value bits
        if value & FIRST_BIT_1_U64 == FIRST_BIT_1_U64 {
          // current bit in value is 1, write 1 to current byte
          *current_byte |= FIRST_BIT_1_U8;
        } else {
          // current bit in value is 0, write 0 to current byte
          *current_byte &= FIRST_BIT_0_U8;
        }

        value = value.rotate_left(1);
      }

      *current_byte = current_byte.rotate_left(1);
      current_bit += 1;
    }

    return result;
  }

  pub fn start_metadata() -> MetadataKeyValuePair {
    return MetadataKeyValuePair {
      key: START_METADATA,
      value: 0, 
    };
  }

  pub fn end_metadata(bits: u64) -> MetadataKeyValuePair {
    return MetadataKeyValuePair {
      key: END_METADATA,
      value: bits,
    };
  }

  // create a MetadataKeyValuePair that signifies a dictionary entry of the
  // huffman tree. So it states what char it is for, and what bits represent
  // that char, as well as how many bits are necessary for that char
  pub fn new_dict_entry(pair: &CharCodePair) -> MetadataKeyValuePair {
    let mut key = DICTIONARY_ENTRY;
    let mut value: u64 = u64::MAX;
    let mut char_value = pair.value as u32;
    let mut char_code = pair.code;
    let mut char_bits = pair.bits;
    // the dictionary key states that it's a dictionary, and how many bits
    // are used to represent the char (pair.bits). the first 8 bits in the
    // key states it's a dictionary, and the second 8 bits is the pair.bits

    // write the key
    for _ in 0..8 {
      if char_bits & LAST_BIT_1_U8 == LAST_BIT_1_U8 {
        // current bit in char_bits is 1, write 1 to key
        key |= LAST_BIT_1_U16;
      } else {
        // current bit in char_bits is 0, write 0 to key
        key &= LAST_BIT_0_U16;
      }

      // rotate the bits to work on the next bit
      key = key.rotate_right(1);
      char_bits = char_bits.rotate_right(1);
    }

    // reset the bits back to original position
    key = key.rotate_left(8);

    // write the value
    // first 32 bits are for the char, because whole char can be up to 32 bits
    // next 32 bits are for the code that represents the char
    for i in 0..32 {
      if char_value & FIRST_BIT_1_U32 == FIRST_BIT_1_U32 {
        // current bit of char value is 1, write 1 to current bit of
        // char value in the dictionary value
        value |= MID_BIT_1_U64;
      } else {
        // current bit of char value is 0, write 0 to current bit of
        // char value in the dictionary value
        value &= MID_BIT_0_U64;
      }

      if char_code & FIRST_BIT_1_U32 == FIRST_BIT_1_U32 {
        // current bit of char code is 1, write 1 to current bit of
        // char code in the dictionary value
        value |= LAST_BIT_1_U64;
      } else {
        // current bit of char code is 0, write 0 to current bit of
        // char code in the dictionary value
        value &= LAST_BIT_0_U64;
      }

      // only rotate if didn't write last bit
      if i != 31 {        
        // rotate all the bits to move to next bit
        char_value = char_value.rotate_left(1);
        char_code = char_code.rotate_left(1);
        value = value.rotate_left(1);
      }
    }

    return MetadataKeyValuePair {
      key,
      value,
    };
  }

  // check if this is the end of the metadata
  pub fn is_end(&self) -> bool {
    return self.key & END_METADATA == END_METADATA;
  }

  // check if this is a dictionary entry
  pub fn is_dict_entry(&self) -> bool {
    return self.key >= DICTIONARY_ENTRY && self.key <= MAX_DICTIONARY_ENTRY;
  }

  // convert this to a CharCodePair
  // 
  // only works if self.is_dict_entry() == true
  // will panic otherwise
  pub fn to_char_code_pair(&self) -> CharCodePair {
    if !self.is_dict_entry() {
      panic!("Tried to convert non- dictionary entry to CharCodePair");
    } else {
      let mut key = self.key;
      let mut value = self.value;

      let mut char_value: u32 = 0;
      let mut char_bits: u8 = 0;
      let mut char_code: u32 = 0;

      // extract the char_bits from the dictionary key
      for i in 0..8 {
        if key & LAST_BIT_1_U16 == LAST_BIT_1_U16 {
          // current bit in key is 1, write 1 to current bit in char bits
          char_bits |= FIRST_BIT_1_U8;
        } else {
          // current bit in key is 0, write 0 to current bit in char bits
          char_bits &= FIRST_BIT_0_U8;
        }

        // don't rotate the bits after writing the last one
        if i != 7 {
          key = key.rotate_right(1);
          char_bits = char_bits.rotate_right(1);
        }
      }

      // extract the char_value and char_code from the dictionary value
      for i in 0..32 {
        if value & MID_BIT_1_U64 == MID_BIT_1_U64 {
          // current char value bit in value is 1, write 1 to current bit in char_value
          char_value |= FIRST_BIT_1_U32;
        } else {
          // current char value bit in value is 0, write 0 to current bit in char_value
          char_value &= FIRST_BIT_0_U32;
        }

        if value & LAST_BIT_1_U64 == LAST_BIT_1_U64 {
          // current char code bit in value is 1, write 1 to current bit in char_code
          char_code |= FIRST_BIT_1_U32;
        } else {
          // current char code bit in value is 0, write 0 to current bit in char_code
          char_code &= FIRST_BIT_0_U32;
        }

        if i != 31 {
          value = value.rotate_right(1);
          char_value = char_value.rotate_right(1);
          char_code = char_code.rotate_right(1);
        }
      }

      return CharCodePair::new(
        char::from_u32(char_value).unwrap(),
        char_bits,
        char_code,
      );
    }
  }
}

