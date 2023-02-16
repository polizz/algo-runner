use std::collections::HashMap;

type Memo = HashMap<i32, Option<Vec<i32>>>;

fn do_how_sum(target: i32, nums: &Vec<i32>, memo: &mut Memo) -> Option<Vec<i32>> {
  if memo.contains_key(&target) {
    return memo.get(&target).unwrap().clone();
  }

  if target < 0 {
    return None;
  }

  if target == 0 {
    return Some(vec![]);
  }

  for num in nums {
    let remainder = target - num;

    if let Some(mut vec) = do_how_sum(remainder, &nums, memo) {
      vec.push(num.clone());
      memo.insert(target.clone(), Some(vec));
      
      return memo.get(&target).unwrap().clone();
    }
  }

  memo.insert(target.clone(), None);
  return None;
}

pub fn how_sum(m: i32, n: Vec<i32>) -> Option<Vec<i32>> {
  let mut memo = Memo::new();
  return do_how_sum(m, &n, &mut memo);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn does_find_sum() {
    let how_sum_res = how_sum(7, vec![2, 3]);
    // println!("howsum {:#?}", how_sum_res);
    assert!(how_sum_res == Some(vec![3, 2, 2]));
  }

  #[test]
  fn does_find_large_sum() {
    let how_sum_res = how_sum(300, vec![7, 14]);
    // println!("howsum {:#?}", how_sum_res);
    assert!(how_sum_res == None);
  }
}