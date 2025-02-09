mod char_code;
mod char_frequency;
mod huffman_coding;
mod huffman_tree;
mod merge_sort;
mod metadata;

use std::fs;
use crate::char_code::CharCodePair;
use crate::huffman_coding::HuffmanCoding;
use crate::huffman_tree::HuffmanTree;
use crate::metadata::MetadataKeyValuePair;

fn main() {
  let contents = fs::read_to_string("src/data/test.txt")
    .expect("Failed to read file");

  let tree = HuffmanTree::new(&contents);
  let bytes = HuffmanCoding::compress(&contents, &tree);

  let string_decom = HuffmanCoding::decompress(bytes);
  println!("{:?}", string_decom.as_bytes() == contents.as_bytes());
}
