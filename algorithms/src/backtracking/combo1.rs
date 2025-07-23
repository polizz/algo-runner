#![allow(dead_code)]

use std::collections::HashSet;
struct Combination1;

impl Combination1 {
  pub fn do_n_choose_k_r(
    nums: &[usize],
    combinations: &mut Vec<Vec<usize>>,
    combo: &mut Vec<usize>,
    used: &mut Vec<bool>,
    seen: &mut HashSet<Vec<usize>>,
    k: usize,
    len: usize,
  ) {
    if k == nums.len() {
      combinations.push(nums[..].to_owned());
      return;
    }
    if len >= nums.len() {
      return;
    }
    if len == k {
      let mut sorted_combo = combo.clone();
      sorted_combo.sort();

      if !seen.contains(&sorted_combo) {
        seen.insert(sorted_combo);
        combinations.push(combo.clone());
      }
      return;
    }

    for n in 0..nums.len() {
      if !used[n] {
        combo.push(nums[n]);
        used[n] = true;
        Combination1::do_n_choose_k_r(nums, combinations, combo, used, seen, k, len + 1);
        used[n] = false;
        combo.pop();
      }
    }
  }

  pub fn n_choose_k(nums: &Vec<usize>, k: usize) -> Vec<Vec<usize>> {
    let mut used: Vec<bool> = vec![false; nums.len()];
    let mut combinations: Vec<Vec<usize>> = vec![];
    let mut combo: Vec<usize> = vec![];
    let mut seen: std::collections::HashSet<Vec<usize>> = Default::default();

    Combination1::do_n_choose_k_r(
      nums,
      &mut combinations,
      &mut combo,
      &mut used,
      &mut seen,
      k,
      0,
    );

    combinations
  }
}

pub fn main() {
  let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let result = Combination1::n_choose_k(&nums, 3);

  println!("Result: {:?}", &result);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_combo_smoke() {
    let nums = vec![1, 2, 3, 4];
    let result = Combination1::n_choose_k(&nums, 4);

    println!("Result: {:?}", &result);
  }
}
