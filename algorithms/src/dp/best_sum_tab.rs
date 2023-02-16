pub fn best_sum_tab(target: usize, numbers: Vec<i32>) -> Option<Vec<i32>> {
  let mut tab: Vec<Option<Vec<i32>>> = vec![None; target + 1];

  tab[0] = Some(vec![]);

  (0..=target).for_each(|i| {
    for forward_num in &numbers {
      if let Some(source_arr) = &tab[i] {
        let next_idx = i + *forward_num as usize; //skip numbers that might be larger than our target sum

        if next_idx <= target {
          if let Some(pre_existing_arr) = &tab[next_idx] {
            if pre_existing_arr.len() < source_arr.len() + 1 {
              continue;
            }
          }

          let mut forward_arr = source_arr.clone();
          forward_arr.insert(0, *forward_num);
          tab[next_idx] = Some(forward_arr);
        }
      }
    }
  });

  // println!("{:#?}", &tab);

  tab[target].clone()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn does_find_best_sum_7() {
    let best_sum_res = best_sum_tab(7, vec![5, 3, 4, 7]);
    // println!("howsum {:#?}", best_sum_res);
    assert!(best_sum_res == Some(vec![7]));
  }

  #[test]
  fn does_find_best_sum_8() {
    let best_sum_res = best_sum_tab(8, vec![2, 3, 5]);
    // println!("howsum {:#?}", best_sum_res);
    assert!(best_sum_res == Some(vec![3, 5]));
  }

  #[test]
  fn does_find_best_sum_8_2() {
    let best_sum_res = best_sum_tab(8, vec![1, 4, 5]);
    // println!("howsum {:#?}", best_sum_res);
    assert!(best_sum_res == Some(vec![4, 4]));
  }

  #[test]
  fn does_find_best_large_sum() {
    let best_sum_res = best_sum_tab(100, vec![1, 2, 5, 25]);
    // println!("howsum {:#?}", best_sum_res);
    assert!(best_sum_res == Some(vec![25, 25, 25, 25]));
  }
}