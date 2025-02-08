mod char_code;
mod char_frequency;
mod huffman_coding;
mod huffman_tree;
mod merge_sort;
mod metadata;

use std::fs;
use crate::huffman_coding::HuffmanCoding;
use crate::huffman_tree::HuffmanTree;
use crate::metadata::MetadataKeyValuePair;

fn main() {
  let contents = fs::read_to_string("src/data/test.txt")
    .expect("Failed to read file");

  let tree = HuffmanTree::new(&contents);
  let bytes = HuffmanCoding::compress(&contents, &tree);
  println!("original: {}, compressed: {}", contents.as_bytes().len(), bytes.len());

  let bytes: [u8; 6] = [
    255,
    254,
    253,
    252,
    251,
    250
  ];

  let md = MetadataKeyValuePair::from_bytes(bytes);
  println!("{:?}", md.as_bytes());
}
