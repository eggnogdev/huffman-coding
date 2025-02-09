use clap::Parser;
use std::path::PathBuf;

// Simple program to compress and decompress text with Huffman coding
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct ClArgs {
  /// Run compression
  #[arg(short, long, default_value_t = false)]
  pub compress: bool,

  /// Run decompression
  #[arg(short, long, default_value_t = false)]
  pub decompress: bool,

  /// File to compress/decompress
  #[arg(short, long)]
  pub file: PathBuf,

  /// Output file of compression/decompression
  #[arg(short, long)]
  pub output: PathBuf,
}
