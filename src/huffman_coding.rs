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
      let char_code = pair.code.rotate_left(rotate_amount.into());
      for i in 1..=pair.bits {
        // rotate left so the current bit we are looking at is at
        // the very end of the byte
        let rot = char_code.rotate_left(i.into());
        if rot & 0b00000001 == 1 {
          // if last bit of rot is 1, set last bit of current byte to 1
          current_byte |= 0b00000001;

        } else {
          // if last bit of rot is 0, set last bit of current byte to 0
          current_byte &= 0b11111110;
        }

        // check if we just wrote to the last bit of the byte
        if current_byte_index % 8 == 7 {
          // finished byte, add it to the result and reset current byte
          bytes.push(current_byte);
          current_byte = 0;
        } else {
          // still working on the byte, so rotate it by one
          current_byte = current_byte.rotate_left(1);
        }

        // move to next bit in byte
        current_byte_index = match current_byte_index.checked_add(1) {
          Some(n) => n,
          None => panic!("Length of compressed bits will be too long for the algorithm to handle"),
        }
      }
    }

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

  // pub fn decompress(b: Vec<u8>, tree: &HuffmanTree) -> String {

  // }

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
}
