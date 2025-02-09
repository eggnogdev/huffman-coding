# huffman-coding

This is my implementation of compression with huffman coding.

## Purpose

I wrote this to practice Data Structures & Algorithms, as well as dip my toes into compression, because I have always been fascinated by the ability to pack as much data in as little space as possible. I also wrote it in Rust because I have been having a burning desire to learn Rust, and it's a perfect language to work with bits in such precise detail like I never have before.

## Some Explanation and Pitfalls 

Of course, Huffman coding is one of the earliest (maybe the earliest?) and most basic compression algorithm, so it doesn't perform quite well anymore up against modern methods. This program, nonetheless does still work!

Due to the nature of Huffman coding, and the need to somehow pass along the Huffman tree along with the file, it can do the opposite of compression with small files. You can see in [data](./data) that the `short_test.txt`, was "compressed" from 44B to 243B. Obviously this isn't compression, so this method should be avoided for small files. Larger amounts of text, however, does actually get compressed, as seen by the `test.txt`.

As mentioned, the Huffman tree needs to be passed along in some form with the file, which takes up precious bits. My method is probably not very efficient. I definitely could have made it take up less space, but I also wanted it to be easy for myself to work with. The metadata which gives the Huffman codes is quite sizeable. It's 10 bytes per character in the original file's alphabet, plus 20 bytes which signifies the start and end of the metadata. So, the metadata's size will be `10bytes * alphabet_length + 20bytes`.

There are some limitations, though. Each character in the text file must be representable by Rust's `char` type. It takes up the space of a `u32`, and can be converted into a `u32`, but not every `u32` is a valid `char`. The metadata entry which signifies the end of the metadata is where it tells the program how many bits after the metadata it should care about. The reason I do this is because I had to write the file in byte sized chunks, and the very last character(s) might not take up the entire byte, so the rest of the bits must be ignored. This does mean that the compressed data must be less than Rust's `u64::MAX` bits. That isn't much of an issue though (this day), because the compressed data must fit within 2048 Petabytes (thats 2 million Terabytes or 2 billion Gigabytes!), which some would say is quite a lot of text.

I'm sure there are many improvements to be made with how it works, even while still doing pure Huffman coding and just improving efficiency with space and time complexities, but I'm really happy with the result and I really enjoyed the entire process.

## Try it out yourself!

If you have Rust and Cargo already installed, go ahead and build yourself a binary and try it out! If you don't want to do that then follow the steps below.

### Get the Binary

Check the [Releases](https://github.com/eggnogdev/huffman-coding/releases) page for the binary. Download it and place it wherever you would like to execute it from.

### Running the Binary

Now you're ready to get compressing!

Run the following to compress your text file.

```
/path/to/binary -c -f /path/to/file -o /path/to/output
```

Try decompressing as well!

```
/path/to/binary -d -f /path/to/file -o /path/to/output
```
