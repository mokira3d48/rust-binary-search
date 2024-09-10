use std::cmp::Ordering;


pub fn find<T: Ord>(array: &[T], target: T) -> Option<usize> {
  // By default, if the slice is empty, the its length will be equal to 0.
  // So the low == high and this function will return None.

  let mut low = 0;
  let mut high = array.len();
  while low < high {
    let mid = (low + high) / 2;
    // if array[mid] == target {
    //   Some(mid)
    // } else if array[mid] < target {
    //   low = mid + 1;
    // } else {
    //   high = mid;
    // }
    match array[mid].cmp(&target) {
      Ordering::Equal => return Some(mid),
      Ordering::Less => low = mid + 1,
      Ordering::Greater => high = mid,
    };
  }
  None  // No such target in this array.
}
