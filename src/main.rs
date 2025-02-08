mod char_code;
mod char_frequency;
mod huffman_coding;
mod huffman_tree;
mod merge_sort;

use std::fs;
use crate::huffman_coding::HuffmanCoding;
use crate::huffman_tree::HuffmanTree;

fn main() {
  let contents = fs::read_to_string("src/data/test.txt")
    .expect("Failed to read file");

  let tree = HuffmanTree::new(&contents);
  let bytes = HuffmanCoding::compress(&contents, &tree);
  println!("original: {}, compressed: {}", contents.as_bytes().len(), bytes.len());
}
