use std::fs;

fn main() {
  let contents = fs::read_to_string("src/data/test.txt")
    .expect("Failed to read file");

  let mut frequencies: Vec<CharFrequency> = Vec::new();

  for ch in contents.chars() {
    increment_char_frequency(&mut frequencies, ch);
  }

  println!("{:?}", frequencies);
  let frequencies = merge_sort_char_frequencies(frequencies);
  println!("{:?}", frequencies);
}

#[derive(Clone, Copy, Debug)]
struct CharFrequency(char, u64);

// Get the CharFrequency instance of the given char `c` from the given
// `frequencies`
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

// Get the CharFrequency like `get_char_frequency` but return a mutable value
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

// Increment (by one) the frequency of the given char `c` in the given frequency
// vector `frequencies`
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

// Merge two Vecs of CharFrequency that are already sorted themselves
fn merge_sorted_char_frequencies(a: Vec<CharFrequency>, b: Vec<CharFrequency>) -> Vec<CharFrequency> {
  let mut merged: Vec<CharFrequency> = Vec::new();

  let total_elements = a.len() + b.len();

  let mut a_index: usize = 0;
  let mut b_index: usize = 0;
  for _ in 0..total_elements {
    if !(a_index < a.len()) && b_index < b.len() {
      // have gone through all a_elements so just continue inserting the b_elements
      merged.push(b[b_index]);
      b_index += 1;
    } else if !(b_index < b.len()) && a_index < a.len() {
      // have gone through all b_elements so just continue inserting the a_elements
      merged.push(a[a_index]);
      a_index += 1;
    } else {
      // still have elements from both a and b to insert
      let a_element = a[a_index];
      let a_freq = a_element.1;

      let b_element = b[b_index];
      let b_freq = b_element.1;

      // sort by higher frequency to lower frequency
      if a_freq >= b_freq {
        merged.push(a_element);

        // inserted a_element into the merged Vec, so move to the next a_element
        // but don't move to the next b_element.
        a_index += 1;
      } else {
        // b_element is more frequent than a_element, push that into the merged
        // Vec before a_element
        merged.push(b_element);

        // inserted b_element so move to next b_element, without moving to next
        // a_element.
        b_index += 1;
      }
    }
  }

  return merged;
}

// Perform merge sort on a Vec of CharFrequency 
//
// sorting is done by highest to lowest frequency
fn merge_sort_char_frequencies(frequencies: Vec<CharFrequency>) -> Vec<CharFrequency> {
  if frequencies.len() < 2 {
    return frequencies;
  } else {
    let half = frequencies.len() / 2;
    let a = merge_sort_char_frequencies(frequencies[0..half].to_vec());
    let b = merge_sort_char_frequencies(frequencies[half..].to_vec());
    return merge_sorted_char_frequencies(a, b);
  }
}
