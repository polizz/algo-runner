pub fn can_sum_tab(target: usize, numbers: Vec<i32>) -> bool {
  let mut tab = vec![false; target + 1];

  tab[0] = true;

  (0..=target).for_each(|i| {
    (0..numbers.len())
      .for_each(|j| {
        if tab[i] {
          let forward_num = numbers[j];

          if i + forward_num as usize <= target {
            tab[i + forward_num as usize] = true;
          }
        }
      })
  });

  // println!("{:#?}", &tab);

  tab[target]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn does_find_sum() {
    let can_sum = can_sum_tab(7, vec![2, 3]);
    assert!(can_sum);
  }

  #[test]
  fn does_not_find_sum() {
    let can_sum = can_sum_tab(7, vec![11, 3]);
    assert!(!can_sum);
  }

  #[test]
  fn does_not_find_large_sum() {
    let can_sum = can_sum_tab(77, vec![4, 8, 6, 2]);
    assert!(!can_sum);
  }

  #[test]
  fn does_find_large_sum() {
    let can_sum = can_sum_tab(300, vec![7, 14]);
    assert!(!can_sum);
  }
}