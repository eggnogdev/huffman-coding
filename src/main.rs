mod char_code;
mod char_frequency;
mod cl_args;
mod huffman_coding;
mod huffman_tree;
mod merge_sort;
mod metadata;

use crate::cl_args::ClArgs;
use crate::huffman_coding::HuffmanCoding;
use crate::huffman_tree::HuffmanTree;

use std::{ 
  fs,
  io::Write,
  path::PathBuf,
  time::Instant
};

use clap::Parser;

fn main() {
  let args = ClArgs::parse();

  let args_status_code = validate_args(&args);
  if args_status_code != 0 {
    std::process::exit(args_status_code);
  }

  if args.compress {
    let contents = fs::read_to_string(args.file)
      .expect("Failed to read passed file");

    run_compression(&contents, args.output, args.verbose);
  } else if args.decompress {
    let contents = fs::read(args.file) 
      .expect("Failed to read passed file");

    run_decompression(contents, args.output, args.verbose);
  }
}

fn validate_args(args: &ClArgs) -> i32 {
  let did_specify_mode = args.compress || args.decompress;
  let did_both_modes = args.compress && args.decompress;
  if !did_specify_mode {
    println!("Please specify the mode -c or -d!");
    return 1;
  } else if did_both_modes {
    println!("Please specify only one mode -c or -d!");
    return 2;
  }

  return 0;
}

fn run_compression(s: &str, output: PathBuf, verbose: bool) {
  let before = Instant::now();
  let tree = match verbose {
    true => HuffmanTree::new_verbose(s), 
    false => HuffmanTree::new(s),
  };

  let bytes = match verbose {
    true => HuffmanCoding::compress_verbose(s, &tree),
    false => HuffmanCoding::compress(s, &tree),
  };

  let mut file = fs::OpenOptions::new()
    .create(true)
    .write(true)
    .open(output).unwrap();

  match file.write_all(&bytes) {
    Ok(_) => {},
    Err(e) => println!("Error: {e}"),
  };

  if verbose {
    println!("Total time elapsed: {:?}", before.elapsed());
  }
}

fn run_decompression(b: Vec<u8>, output: PathBuf, verbose: bool) {
  let before = Instant::now();
  let string = match verbose {
    true => HuffmanCoding::decompress_verbose(b),
    false => HuffmanCoding::decompress(b),
  };

  let mut file = fs::OpenOptions::new()
    .create(true)
    .write(true)
    .open(output).unwrap();

  match file.write_all(&string.as_bytes()) {
    Ok(_) => {},
    Err(e) => println!("Error: {e}"),
  };

  if verbose {
    println!("Total time elapsed: {:?}", before.elapsed());
  }
}
