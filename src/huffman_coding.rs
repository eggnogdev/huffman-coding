use crate::char_code::CharCodePair;
use crate::huffman_tree::{ HuffmanTree, HuffmanTreeNode };
use crate::metadata::MetadataKeyValuePair;

pub struct HuffmanCoding;

impl HuffmanCoding {
  pub fn compress(s: &str, tree: &HuffmanTree) -> Vec<u8> {
    let mut char_codes: Vec<CharCodePair> = Vec::new();
    Self::get_char_code_pairs_from_tree(
      &tree.trunk, // start with the trunk node
      0, // initial call so current_code is empty (0)
      0, // initial call so current_code_bits is none (0)
      &mut char_codes // the Vec where char_codes will be placed
    );

    // the final compressed bytes of the text
    let mut bytes: Vec<u8> = Vec::new();
    // byte currently being written
    let mut current_byte: u8 = 0;
    // index inside the current byte. (bit index in whole string)
    let mut current_byte_index: u64 = 0;
    for ch in s.chars() {
      let pair = Self::get_char_code_pair(ch, &char_codes);
      // rotate bits left by `8 - bits` to turn something like
      // 0b00111111
      // (assuming `pair.bits` is 6)
      // into
      // 0b11111100
      let rotate_amount = 8 - pair.bits;
      let mut char_code = pair.code.rotate_left(rotate_amount.into());
      for _ in 0..pair.bits {
        if char_code & 0b1000_0000 == 0b1000_0000 {
          // current bit of char_code is 1, write 1 to current bit of byte
          current_byte |= 0b0000_0001;
        } else {
          // current bit of char_code is 0, write 0 to current bit of byte
          current_byte &= 0b1111_1110;
        }

        // rotate the bits to prepare for next one
        char_code = char_code.rotate_left(1);

        // check if we just wrote to the last bit of the byte
        if current_byte_index % 8 == 7 { 
          bytes.push(current_byte);
          current_byte = 0;
        } else {
          // only rotate the current byte if we didn't write to the last bit
          current_byte = current_byte.rotate_left(1);
        }

        // move to next bit in byte
        current_byte_index = match current_byte_index.checked_add(1) {
          Some(n) => n,
          None => panic!("Length of compressed bits will be too long for the algorithm to handle"),
        }
      }
    }

    // push the last current byte to make sure it went through.
    // rotate amount 7 - ... because the else statement above already performed
    // one rotation whenever the last bit wasn't written to
    let rotate_amount = 7 - (current_byte_index % 8);
    current_byte = current_byte.rotate_left(rotate_amount as u32);
    bytes.push(current_byte);

    let mut result: Vec<u8> = Vec::new();

    // at this point, current_byte_index represents total number of bits that
    // are used to represent the compressed data. this is used in the metadata
    // to let the decompression algorithm know exactly the number of bits it
    // should care about, ignoring any extra bits left in a byte so it wont
    // think those extra bits are part of the message.
    let metadata = Self::generate_metadata(
      &char_codes,
      current_byte_index
    );

    result.append(&mut Self::metadata_to_bytes(&metadata));
    result.append(&mut bytes);

    return result;
  }

  pub fn decompress(mut b: Vec<u8>) -> String {
    const FIRST_BIT_1_U8: u8 = 0b1000_0000;

    const LAST_BIT_0_U32: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1110;
    const LAST_BIT_1_U32: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0001;

    let mut result = String::new();
    let metadata = Self::get_metadata_from_bytes(&b);
    let dict_entries = Self::get_metadata_dictionary_entries(&metadata);
    let char_codes = Self::dictionary_entries_to_char_code_pairs(dict_entries);

    let last_metadata = &metadata[metadata.len() - 1];
    let total_bits = match last_metadata.is_end() {
      true => last_metadata.value,
      false => panic!("Last metadata entry wasn't an END_METADATA entry"),
    };

    let metadata_byte_count = metadata.len() * 10;
    let compressed_bytes = &mut b[metadata_byte_count..];

    let mut current_bit_index: u64 = 0;
    let mut current_code: u32 = 0;
    let mut current_bits: u8 = 0;
    while current_bit_index < total_bits {
      let current_byte_index = current_bit_index / 8;
      let current_byte = &mut compressed_bytes[current_byte_index as usize];

      if *current_byte & FIRST_BIT_1_U8 == FIRST_BIT_1_U8 {
        // current bit of the byte is 1, write 1 to current bit in code
        current_code |= LAST_BIT_1_U32;
      } else {
        // current bit of the byte is 0, write 0 to current bit in code
        current_code &= LAST_BIT_0_U32;
      }

      // increment bits in the code
      current_bits += 1;

      let found_char = Self::get_char_for_code_and_bits(
        &char_codes,
        current_code,
        current_bits
      );

      match found_char {
        Some(c) => {
          // found a char!!
          result.push(c);
          current_code = 0;
          current_bits = 0;
        },
        None => {
          current_code = current_code.rotate_left(1);
        }
      }

      // move on to next bit
      *current_byte = current_byte.rotate_left(1);
      current_bit_index += 1;
    }

    return result;
  }

  // Traverse the HuffmanTreeNode to get all the char code pairs
  fn get_char_code_pairs_from_tree(
    tree: &HuffmanTreeNode,
    current_code: u32,
    current_code_bits: u8,
    all_pairs: &mut Vec<CharCodePair>
  ) {
    // used to set last bit to 0
    let last_bit_0 = 0b1111_1111_1111_1111_1111_1111_1111_1110;
    // used to set last bit to 1
    let last_bit_1 = 0b0000_0000_0000_0000_0000_0000_0000_0001;

    let left_char = match &tree.left {
      Some(lnode) => lnode.value,
      None => None,
    };

    let right_char = match &tree.right {
      Some(rnode) => rnode.value,
      None => None,
    };

    if left_char.is_some() {
      // found char on the left. rotate the current code by 1 to make
      // space for the final bit. set the final bit to 0 (left). increment
      // the bit count for the left code.
      let left_char = left_char.unwrap();
      let left_code = current_code.rotate_left(1) & last_bit_0;

      all_pairs.push(CharCodePair::new(
        left_char,
        current_code_bits + 1,
        left_code,
      ));
    }

    if right_char.is_some() {
      // found char on the right. rotate the current code by 1 to make
      // space for the final bit. set the final bit to 1 (right). increment
      // the bit count for the right code.
      let right_char = right_char.unwrap();
      let right_code = current_code.rotate_left(1) | last_bit_1;

      all_pairs.push(CharCodePair::new(
        right_char,
        current_code_bits + 1,
        right_code,
      ));
    }

    if tree.left.is_some() {
      // moving on to the left side of the node, rotate the current code
      // to make room for this branch, and set the last bit to 0 (left)
      let branch_left_code = current_code.rotate_left(1) & last_bit_0;

      Self::get_char_code_pairs_from_tree(
        &tree.left.as_ref().unwrap(),
        branch_left_code,
        current_code_bits + 1,
        all_pairs
      );
    }

    if tree.right.is_some() {
      // moving on to the right side of the node, rotate the current code
      // to make room for this branch, and set the last bit to 1 (right)
      let branch_right_code = current_code.rotate_left(1) | last_bit_1;

      Self::get_char_code_pairs_from_tree(
        &tree.right.as_ref().unwrap(),
        branch_right_code,
        current_code_bits + 1,
        all_pairs
      );
    }
  }

  // get the CharCodePair for the given char `c` out of given `pairs`
  fn get_char_code_pair(c: char, pairs: &Vec<CharCodePair>) -> &CharCodePair {
    for pair in pairs {
      if pair.value == c {
        return &pair;
      }
    }

    // crash program if char doesn't exist in given pairs
    // maybe should do better handling but this works.
    panic!();
  }

  // generate the metadata for the CharCodePairs `pairs` and the length
  // of the compressed data `bits`. 
  fn generate_metadata(
    pairs: &Vec<CharCodePair>, 
    bits: u64
  ) -> Vec<MetadataKeyValuePair> {
    let mut result: Vec<MetadataKeyValuePair> = Vec::new();
    result.push(MetadataKeyValuePair::start_metadata());

    for pair in pairs {
      result.push(MetadataKeyValuePair::new_dict_entry(pair));
    }

    result.push(MetadataKeyValuePair::end_metadata(bits));

    return result;
  }

  // convert the Vec of MetadataKeyValuePair to a Vec of bytes (u8)
  fn metadata_to_bytes(md: &Vec<MetadataKeyValuePair>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for entry in md {
      result.extend_from_slice(&entry.as_bytes());
    }

    return result;
  }

  // go through the compressed bytes and gather just the metadata entries
  fn get_metadata_from_bytes(b: &Vec<u8>) -> Vec<MetadataKeyValuePair> {
    let mut result: Vec<MetadataKeyValuePair> = Vec::new();

    // iterate through each possible metadata section
    // one metadata entry is 10 bytes long
    for i in 0..(b.len() / 10) {
      let current_bytes: [u8; 10] = b[i*10..i*10+10].try_into().unwrap();
      let md = MetadataKeyValuePair::from_bytes(current_bytes);

      if md.is_end() {
        result.push(md);
        return result;
      } else {
        result.push(md);
      }
    }

    return result;
  }

  // filter only for the dictionary metadata entries
  fn get_metadata_dictionary_entries(
    md: &Vec<MetadataKeyValuePair>
  ) -> Vec<&MetadataKeyValuePair> {
    let mut result: Vec<&MetadataKeyValuePair> = Vec::new();

    for entry in md {
      if entry.is_dict_entry() {
        result.push(&entry);
      }
    }

    return result;
  }

  // convert all dictionary entries to char code pairs
  fn dictionary_entries_to_char_code_pairs(
    md: Vec<&MetadataKeyValuePair>,
  ) -> Vec<CharCodePair> {
    let mut result: Vec<CharCodePair> = Vec::new();

    for entry in md {
      result.push(entry.to_char_code_pair());
    }

    return result;
  }

  // search the given `pairs` for a pair that has the same
  // `bits` and `code`
  fn get_char_for_code_and_bits(
    pairs: &Vec<CharCodePair>,
    code: u32, 
    bits: u8
  ) -> Option<char> {
    for pair in pairs {
      if pair.bits == bits && pair.code == code {
        return Some(pair.value);
      }
    }

    return None;
  }
}
