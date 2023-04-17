use std::collections::HashMap;

type Memo = HashMap<i32, Option<Vec<i32>>>;

fn do_best_sum(target: i32, nums: &Vec<i32>, memo: &mut Memo) -> Option<Vec<i32>> {
  if memo.contains_key(&target) {
    return memo.get(&target).unwrap().clone();
  }

  let mut shortest_combination: Option<Vec<i32>> = None;

  if target < 0 {
    return None;
  }

  if target == 0 {
    return Some(vec![]);
  }

  for &num in nums {
    let remainder = target - num;

    if let Some(mut vec) = do_best_sum(remainder, &nums, memo) {
      vec.push(num);

      let short_clone = shortest_combination.clone();
      if short_clone.is_none() || vec.len() < short_clone.unwrap().len() {
        shortest_combination = Some(vec);
      }
    }
  }

  memo.insert(target.clone(), shortest_combination);
  return memo.get(&target).unwrap().clone();
}

pub fn best_sum(m: i32, n: Vec<i32>) -> Option<Vec<i32>> {
  let mut memo = Memo::new();
  return do_best_sum(m, &n, &mut memo);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn does_find_best_sum() {
    let how_sum_res = best_sum(8, vec![1, 4, 5]);
    // println!("howsum {:#?}", how_sum_res);
    assert!(how_sum_res == Some(vec![4, 4]));
  }

  #[test]
  fn does_find_best_large_sum() {
    let how_sum_res = best_sum(100, vec![1, 2, 5, 25]);
    // println!("howsum {:#?}", how_sum_res);
    assert!(how_sum_res == Some(vec![25, 25, 25, 25]));
  }
}
