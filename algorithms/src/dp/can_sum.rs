use std::collections::HashMap;

type Memo = HashMap<i32, bool>;

fn do_can_sum(target: i32, nums: &Vec<i32>, memo: &mut Memo) -> bool {
  if memo.contains_key(&target) {
    return memo.get(&target).unwrap().clone();
  }

  if target < 0 {
    return false;
  }

  if target == 0 {
    return true;
  }

  for num in nums {
    let remainder = target - num;
    // println!("checking target {} with num {}", &remainder, &num);

    if do_can_sum(remainder, &nums, memo) {
      memo.insert(target.clone(), true);
      return true;
    }
  }

  memo.insert(target.clone(), false);
  return false;
}

pub fn can_sum(m: i32, n: Vec<i32>) -> bool {
  let mut memo = Memo::new();
  return do_can_sum(m, &n, &mut memo);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn does_find_sum() {
    let can_sum = can_sum(7, vec![2, 3]);
    assert!(can_sum);
  }

  #[test]
  fn does_not_find_sum() {
    let can_sum = can_sum(7, vec![11, 3]);
    assert!(!can_sum);
  }

  #[test]
  fn does_not_find_large_sum() {
    let can_sum = can_sum(77, vec![4, 8, 6, 2]);
    assert!(!can_sum);
  }

  #[test]
  fn does_find_large_sum() {
    let can_sum = can_sum(300, vec![7, 14]);
    assert!(!can_sum);
  }
}
