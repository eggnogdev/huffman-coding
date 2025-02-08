use std::marker::PhantomData;

pub struct MergeSort<T: Copy> {
  // phantom data so rust compiler doesn't complain that T is never used
  // even though I need it for the impl below
  phantom_data: PhantomData<T>,
}

impl<T: Copy> MergeSort<T> {
  pub fn run(values: Vec<T>, condition: fn(T, T) -> bool) -> Vec<T> {
    if values.len() < 2 {
      return values;
    } else {
      let half = values.len() / 2;
      let a = Self::run(values[0..half].to_vec(), condition);
      let b = Self::run(values[half..].to_vec(), condition);
      return Self::merge(a, b, condition);
    }
  }

  fn merge(a: Vec<T>, b: Vec<T>, condition: fn(T, T) -> bool) -> Vec<T> {
    let mut merged: Vec<T> = Vec::new();

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
        let b_element = b[b_index];

        if condition(a_element, b_element) {
          merged.push(a_element);
          a_index += 1;
        } else {
          merged.push(b_element);
          b_index += 1;
        }
      }
    }

    return merged;
  }
}
