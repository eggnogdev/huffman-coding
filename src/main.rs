use std::fs;

fn main() {
  let contents = fs::read_to_string("src/data/test.txt")
    .expect("Failed to read file");

  let mut frequencies: Vec<CharFrequency> = Vec::new();

  for ch in contents.chars() {
    increment_char_frequency(&mut frequencies, ch);
  }

  println!("{:?}", frequencies);
}

#[derive(Clone, Debug)]
struct CharFrequency(char, u64);

fn get_char_frequency(
  frequencies: &Vec<CharFrequency>,
  c: char
) -> Option<&CharFrequency> {
  for freq in frequencies {
    if freq.0 == c {
      return Some(&freq);
    }
  }

  return None;
}

fn get_char_frequency_mut(
  frequencies: &mut Vec<CharFrequency>,
  c: char
) -> Option<&mut CharFrequency> {
  for freq in frequencies {
    if freq.0 == c {
      return Some(freq);
    }
  }

  return None;
}

fn increment_char_frequency(
  frequencies: &mut Vec<CharFrequency>,
  c: char
) {
  let found: Option<&mut CharFrequency> = get_char_frequency_mut(frequencies, c);

  match found {
    Some(freq) => {
      freq.1 += 1;
    },
    None => {
      let freq = CharFrequency(c, 1);
      frequencies.push(freq);
    },
  }

}

struct Node<T> {
  left: Option<Box<Node<T>>>,
  right: Option<Box<Node<T>>>,
  value: Option<T>,
}
