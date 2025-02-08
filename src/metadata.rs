const START_METADATA: u16 = 0b0000_0000_0000_0000;
const DICTIONARY_ENTRY: u16 = 0b0001_0000_0000_0000;
const END_METADATA: u16 = 0b1111_1111_1111_1111;

pub struct MetadataKeyValuePair {
  key: u16,
  value: u32,
}

impl MetadataKeyValuePair {
  pub fn from_bytes(b: [u8; 6]) -> MetadataKeyValuePair {
    const FIRST_BIT_1_U8: u8 = 0b1000_0000;

    const FIRST_BIT_0_U16: u16 = 0b0111_1111_1111_1111;
    const FIRST_BIT_1_U16: u16 = 0b1000_0000_0000_0000;

    const FIRST_BIT_0_U32: u32 = 0b0111_1111_1111_1111_1111_1111_1111_1111;
    const FIRST_BIT_1_U32: u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000;

    let mut key: u16 = 0;
    let mut value: u32 = 0;

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
          value |= FIRST_BIT_1_U32;
        } else {
          // current bit in the byte is 0, write 0 to key
          value &= FIRST_BIT_0_U32;
        }

        // rotate the key bits to edit the next one
        value = value.rotate_left(1);
      }
    }

    return MetadataKeyValuePair { key, value };
  }

  pub fn as_bytes(&self) -> [u8; 6] {
    const FIRST_BIT_0_U8: u8 = 0b0111_1111;
    const FIRST_BIT_1_U8: u8 = 0b1000_0000;

    const FIRST_BIT_1_U16: u16 = 0b1000_0000_0000_0000;

    const FIRST_BIT_1_U32: u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000;

    let mut result: [u8; 6] = [0; 6];

    let mut current_bit: u8 = 0;
    let mut key = self.key;
    let mut value = self.value;
    while current_bit < 48 {
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
        if value & FIRST_BIT_1_U32 == FIRST_BIT_1_U32 {
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
}

