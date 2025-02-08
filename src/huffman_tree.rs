use crate::char_frequency::CharFrequencyPair;
use crate::merge_sort::MergeSort;

pub struct HuffmanTree {
  pub trunk: HuffmanTreeNode,
}

impl HuffmanTree {
  // Grow a new HuffmanTree based on the given &str `s`
  pub fn new(s: &str) -> HuffmanTree {
    let frequencies = Self::count_char_frequencies(s);
    let frequencies = MergeSort::run(frequencies, |a, b| {
      a.count >= b.count
    });

    let trunk = Self::grow(frequencies);
    return HuffmanTree { trunk };
  }

  // count all the char frequencies of the given string `s` and return
  // a Vec of CharFrequencyPair
  fn count_char_frequencies(s: &str) -> Vec<CharFrequencyPair> {
    let mut frequencies: Vec<CharFrequencyPair> = Vec::new();

    for ch in s.chars() {
      Self::increment_char_frequency_in_pairs(ch, &mut frequencies);
    }

    return frequencies;
  }

  // increment (by one) the `CharFrequencyPair.count` for the given char `c`
  // in the given `pairs` Vec
  fn increment_char_frequency_in_pairs(
    c: char, 
    pairs: &mut Vec<CharFrequencyPair>
  ) {
    // check for an existing CharFrequencyPair
    let found = Self::get_char_frequency_pair_mut(c, pairs);

    match found {
      Some(pair) => {
        pair.count += 1;
      },
      None => {
        pairs.push(CharFrequencyPair {
          value: c,
          count: 1,
        });
      }
    }
  }

  // get a mutable reference to the CharFrequencyPair for the given char `c`
  // from the given `pairs`
  fn get_char_frequency_pair_mut(
    c: char,
    pairs: &mut Vec<CharFrequencyPair>
  ) -> Option<&mut CharFrequencyPair> {
    for pair in pairs {
      if pair.value == c {
        return Some(pair);
      }
    }

    return None;
  }

  fn get_total_frequency_below_node(node: &GrowingHuffmanTreeNode) -> u64 {
    let left_frequency = match &node.left {
      Some(lnode) => Self::get_total_frequency_below_node(&lnode),
      None => 0,
    };

    let right_frequency = match &node.right {
      Some(rnode) => Self::get_total_frequency_below_node(&rnode),
      None => 0,
    };

    let char_frequency = match &node.value {
      Some(cf) => cf.count,
      None => 0,
    };

    return left_frequency + right_frequency + char_frequency;
  }

  // insert the given `node` into the given `queue` in an ordered manner,
  // where the total node frequencies are in order from highest to lowest
  fn ordered_insert_node_into_queue(
    queue: &mut Vec<GrowingHuffmanTreeNode>,
    node: GrowingHuffmanTreeNode
  ) -> () {
    let node_frequency = Self::get_total_frequency_below_node(&node);
    for i in 0..queue.len() {
      let current_frequency = Self::get_total_frequency_below_node(&queue[i]);
      if node_frequency > current_frequency {
        return queue.insert(i, node);
      }
    }

    // didn't get inserted yet, so push to top of queue
    queue.push(node);
  }

  // grow the HuffmanTree based on the given `frequencies`
  fn grow(frequencies: Vec<CharFrequencyPair>) -> HuffmanTreeNode {
    let mut queue: Vec<GrowingHuffmanTreeNode> = Vec::new();

    for pair in frequencies {
      queue.push(GrowingHuffmanTreeNode {
        left: None,
        right: None,
        value: Some(pair),
      });
    }

    while queue.len() > 1 {
      // only enter the loop when queue has > 1 elements (at least 2 elements)
      // so popping twice will always succeed.
      let bottom_1 = queue.pop().unwrap();
      let bottom_2 = queue.pop().unwrap();

      let bottom_1_freq = Self::get_total_frequency_below_node(&bottom_1);
      let bottom_2_freq = Self::get_total_frequency_below_node(&bottom_2);

      // put the bottom two nodes into left and right of a new node
      // default to placing higher frequency nodes to the right
      if bottom_1_freq < bottom_2_freq {
        // bottom_1 has lesser frequency, put to the left
        let new_node = GrowingHuffmanTreeNode {
          left: Some(Box::new(bottom_1)),
          right: Some(Box::new(bottom_2)),
          value: None,
        };

        // insert the node back into the queue in order of total node frequency
        Self::ordered_insert_node_into_queue(&mut queue, new_node);
      } else {
        // bottom_1 has greater or equal frequency, put to right
        let new_node = GrowingHuffmanTreeNode {
          left: Some(Box::new(bottom_2)),
          right: Some(Box::new(bottom_1)),
          value: None,
        };

        // insert the node back into the queue in order of total node frequency
        Self::ordered_insert_node_into_queue(&mut queue, new_node);
      }
    }

    // tree is built, convert to normal tree without frequencies
    return queue[0].to_tree_node();
  }
}

pub struct HuffmanTreeNode {
  pub left: Option<Box<HuffmanTreeNode>>,
  pub right: Option<Box<HuffmanTreeNode>>,
  pub value: Option<char>,
}

// impl HuffmanTreeNode {
//   pub fn child_nodes(&self) -> Vec<&Box<HuffmanTreeNode>> {
//     let mut result: Vec<&Box<HuffmanTreeNode>> = Vec::new();
//     match &self.left {
//       Some(lnode) => result.push(lnode),
//       _ => {},
//     };

//     match &self.right {
//       Some(rnode) => result.push(rnode),
//       _ => {},
//     };

//     return result;
//   }
// }

// A HuffmanTreeNode with extra info like frequency of the char `value`
//
// "Growing" because this is used while growing or creating a Huffman tree.
// The extra info is necessary during growth but is tossed out once it is done.
struct GrowingHuffmanTreeNode {
  left: Option<Box<GrowingHuffmanTreeNode>>,
  right: Option<Box<GrowingHuffmanTreeNode>>,
  value: Option<CharFrequencyPair>,
}

impl GrowingHuffmanTreeNode {
  fn to_tree_node(&self) -> HuffmanTreeNode {
    // convert left growing node to tree node
    let left: Option<HuffmanTreeNode> = match &self.left {
      Some(lnode) => Some(lnode.to_tree_node()),
      None => None,
    };

    // convert right growing node to tree node
    let right: Option<HuffmanTreeNode> = match &self.right {
      Some(rnode) => Some(rnode.to_tree_node()),
      None => None,
    };

    // isolate the char of this growing node without frequency
    let value: Option<char> = match &self.value {
      Some(cf) => Some(cf.value),
      None => None,
    };

    return HuffmanTreeNode {
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
