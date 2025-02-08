use std::fs;

fn main() {
  let contents = fs::read_to_string("src/data/test.txt")
    .expect("Failed to read file");

  let mut frequencies: Vec<CharFrequency> = Vec::new();

  for ch in contents.chars() {
    increment_char_frequency(&mut frequencies, ch);
  }

  let frequencies = merge_sort_char_frequencies(frequencies);
  println!("{}", frequencies.len());
  let tree = build_huffman_tree(frequencies);

  let as_bits = string_to_bits_with_tree(contents, tree);
  let bit_count = as_bits.len();
  println!("{:?}", bit_count);
}

#[derive(Clone, Copy, Debug)]
struct CharFrequency(char, u64);

// Get the CharFrequency instance of the given char `c` from the given
// `frequencies`
//
// returns an Optional (possibly null) reference to the CharFrequency
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
//
// returns an Optional (possibly null) mutable reference to the CharFrequency
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

// Merge two Vecs of CharFrequency that are already sorted themselves
fn merge_sorted_char_frequencies(
  a: Vec<CharFrequency>,
  b: Vec<CharFrequency>
) -> Vec<CharFrequency> {
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
fn merge_sort_char_frequencies(
  frequencies: Vec<CharFrequency>
) -> Vec<CharFrequency> {
  if frequencies.len() < 2 {
    return frequencies;
  } else {
    let half = frequencies.len() / 2;
    let a = merge_sort_char_frequencies(frequencies[0..half].to_vec());
    let b = merge_sort_char_frequencies(frequencies[half..].to_vec());
    return merge_sorted_char_frequencies(a, b);
  }
}

#[derive(Debug)]
struct TreeNode {
  left: Option<Box<TreeNode>>,
  right: Option<Box<TreeNode>>,
  value: Option<char>,
}

// same as TreeNode but contains a frequency value.
//
// this is because while building the tree, we need the frequencies of
// chars below each node to compare them against each other.
// this is only used temporarily because once the tree is built, the frequencies
// do not need to be stored, so they would be a waste of space.
struct TreeNodeWithFrequency {
  left: Option<Box<TreeNodeWithFrequency>>,
  right: Option<Box<TreeNodeWithFrequency>>,
  value: Option<CharFrequency>,
}

impl TreeNodeWithFrequency {
  fn to_tree_node(&self) -> TreeNode {
    let left: Option<TreeNode> = match &self.left {
      Some(lnode) => Some(lnode.to_tree_node()),
      None => None,
    };

    let right: Option<TreeNode> = match &self.right {
      Some(rnode) => Some(rnode.to_tree_node()),
      None => None,
    };

    let value: Option<char> = match self.value {
      Some(cf) => Some(cf.0),
      None => None,
    };

    return TreeNode {
      left: match left {
        Some(lnode) => Some(Box::new(lnode)),
        None => None,
      },
      right: match right {
        Some(rnode) => Some(Box::new(rnode)),
        None => None,
      },
      value
    }
  }
}

// RETURN TreeNode
fn build_huffman_tree(frequencies: Vec<CharFrequency>) -> TreeNode {
  let mut queue: Vec<TreeNodeWithFrequency> = Vec::new();

  // put CharFrequency into the queue, where the order highest to lowest
  // corresponds to bottom to top of queue.
  for cf in frequencies {
    queue.push(TreeNodeWithFrequency {
      left: None,
      right: None,
      value: Some(cf),
    });
  }

  // continually combine nodes into each other until we are left with one
  // node that contains all of the nodes
  while queue.len() > 1 {
    // only enter the loop when queue has > 1 elements (at least 2 elements)
    // so popping twice should always succeed.
    let bottom_1 = queue.pop().expect("Failed to pop from empty vec");
    let bottom_2 = queue.pop().expect("Failed to pop from empty vec");

    let bottom_1_freq = get_total_frequency_below_node(&bottom_1);
    let bottom_2_freq = get_total_frequency_below_node(&bottom_2);

    // put the bottom two nodes into left and right of a new node
    // default to placing higher frequency nodes to the right
    if bottom_1_freq < bottom_2_freq {
      // bottom_1 has lesser frequency, put to the left
      let new_node = TreeNodeWithFrequency {
        left: Some(Box::new(bottom_1)),
        right: Some(Box::new(bottom_2)),
        value: None,
      };

      // insert the node back into the queue in order of total node frequency
      ordered_insert_node_into_queue(&mut queue, new_node);
    } else {
      // bottom_1 has greater or equal frequency, put to right
      let new_node = TreeNodeWithFrequency {
        left: Some(Box::new(bottom_2)),
        right: Some(Box::new(bottom_1)),
        value: None,
      };

      // insert the node back into the queue in order of total node frequency
      ordered_insert_node_into_queue(&mut queue, new_node);
    }
  }

  // tree is built, convert to normal tree without frequencies
  return queue[0].to_tree_node();
}

fn get_total_frequency_below_node(node: &TreeNodeWithFrequency) -> u64 {
  let left_frequency = match &node.left {
    Some(lnode) => get_total_frequency_below_node(&lnode),
    None => 0,
  };

  let right_frequency = match &node.right {
    Some(rnode) => get_total_frequency_below_node(&rnode),
    None => 0,
  };

  let char_frequency = match node.value {
    Some(cf) => cf.1,
    None => 0,
  };

  return left_frequency + right_frequency + char_frequency;
}

// insert the given `node` into the given `queue` in an ordered manner,
// where the total node frequencies are in order from highest to lowest
fn ordered_insert_node_into_queue(
  queue: &mut Vec<TreeNodeWithFrequency>,
  node: TreeNodeWithFrequency
) -> () {
  let node_frequency = get_total_frequency_below_node(&node);
  for i in 0..queue.len() {
    let current_frequency = get_total_frequency_below_node(&queue[i]);
    if node_frequency > current_frequency {
      return queue.insert(i, node);
    }
  }

  // didn't get inserted yet, so push to top of queue
  queue.push(node);
}

// convert the given string to bits through the given huffman tree
fn string_to_bits_with_tree(s: String, tree: TreeNode) -> Vec<bool> {
  // storing bits as bools because bools have only two possible values
  let mut bits: Vec<bool> = Vec::new();
  let mut char_bit_pairs: Vec<(char, Vec<bool>)> = Vec::new();

  for ch in s.chars() {
    let mut ch_bits = match get_char_bits_from_pair_vec(ch, &char_bit_pairs) {
      Some(ch_b) => ch_b,
      None => match get_char_bits_from_tree(ch, &tree, vec![]) {
        Some(ch_b) => {
          char_bit_pairs.push((ch, ch_b.clone()));
          ch_b
        },
        None => panic!(),
      },
    };

    bits.append(&mut ch_bits);
  }

  return bits;
}

// Search the tree for the given char and return a Vec of boolean values
// which represents the bits of the huffman coded version of the char
//
// using Vec of booleans because bools are 1bit and I want to work by the bit
// true  -> 1
// false -> 0 
fn get_char_bits_from_tree(
  c: char, 
  tree: &TreeNode, 
  mut current_bits: Vec<bool>
) -> Option<Vec<bool>> {
  let left_char = match &tree.left {
    Some(lnode) => lnode.value,
    None => None,
  };

  let right_char = match &tree.right {
    Some(rnode) => rnode.value,
    None => None,
  };

  if left_char == Some(c) {
    current_bits.push(false);
    return Some(current_bits);
  } else if right_char == Some(c) {
    current_bits.push(true);
    return Some(current_bits);
  } else {
    if tree.left.is_some() {
      let left = tree.left.as_ref().unwrap();
      let mut branch_left_bits = current_bits.clone();
      branch_left_bits.push(false);

      let branch_left = get_char_bits_from_tree(c, &left, branch_left_bits);
      if branch_left.is_some() {
        return branch_left;
      }
    }

    if tree.right.is_some() {
      let right = tree.right.as_ref().unwrap();
      let mut branch_right_bits = current_bits.clone();
      branch_right_bits.push(true);

      let branch_right = get_char_bits_from_tree(c, &right, branch_right_bits);      
      if branch_right.is_some() {
        return branch_right;
      }
    }

    return None;
  }
}

// search a Vec of pairs (char, Vec<bool>) for the desired char's `c` bit form
//
// this makes it so we can compute a char's bit form only once, store it in the
// format desired by this function, and search for it from here so the entire
// tree doesn't have to be searched again with get_char_bits_from_tree
fn get_char_bits_from_pair_vec(
  c: char, 
  pairs: &Vec<(char, Vec<bool>)>
) -> Option<Vec<bool>> {
  for pair in pairs {
    if pair.0 == c {
      return Some(pair.1.clone());
    }
  }

  return None;
}
