// Create the `sum` in the shortest number of combinations of available ints
// Can use any of the available ints as many times as possible
use std::collections::HashMap;

type Memo = HashMap<i32, Option<Vec<i32>>>;

fn do_best_sum(target: i32, nums: &[i32], memo: &mut Memo) -> Option<Vec<i32>> {
  if let Some(val) = memo.get(&target) {
    return val.clone();
  }

  if target == 0 {
    return Some(vec![]);
  }

  if target < 0 {
    return None;
  }

  let mut min_combo: Option<Vec<i32>> = None;

  for &num in nums {
    let remainder = target - num;

    if let Some(mut vec) = do_best_sum(remainder, nums, memo) {
      vec.push(num);

      if min_combo.is_none() || min_combo.as_ref().is_some_and(|min| min.len() > vec.len()) {
        min_combo = Some(vec);
      }
    }
  }

  memo.insert(target, min_combo.clone());
  return min_combo;
}

pub fn best_sum(target: i32, nums: Vec<i32>) -> Option<Vec<i32>> {
  let mut memo = Memo::new();
  return do_best_sum(target, &nums, &mut memo);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn does_find_best_sum() {
    let how_sum_res = best_sum(8, vec![1, 4, 5]);
    assert_eq!(how_sum_res, Some(vec![4, 4]));
  }

  #[test]
  fn does_find_best_large_sum() {
    let how_sum_res = best_sum(100, vec![1, 2, 5, 25]);
    assert_eq!(how_sum_res, Some(vec![25, 25, 25, 25]));
  }

  #[test]
  fn does_find_best_massive_sum() {
    let how_sum_res = best_sum(3987, vec![1, 2, 5, 21]);
    assert_eq!(
      how_sum_res,
      Some(vec![
        21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
        21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
        21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
        21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
        21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
        21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
        21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
        21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
        21, 21, 21, 21, 21, 5, 5, 5, 2, 1
      ])
    );
  }
}
