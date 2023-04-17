pub fn calc_paths(m: usize, n: usize) -> usize {
  let mut paths = vec![vec![0; n + 1]; m + 1];

  paths[1][1] = 1;

  (0..=m).for_each(|r| {
    (0..=n).for_each(|c| {
      if r + 1 <= m {
        paths[r + 1][c] = paths[r + 1][c] + paths[r][c];
      }
      if c + 1 <= n {
        paths[r][c + 1] = paths[r][c + 1] + paths[r][c];
      }
    });
  });

  // println!("{:#?}", paths);

  paths[m][n]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn finds_correct_path_count_2_3() {
    let count = calc_paths(2, 3);
    assert!(count == 3);
  }

  #[test]
  fn finds_correct_path_count_3_3() {
    let count = calc_paths(3, 3);
    assert!(count == 6);
  }

  #[test]
  fn finds_large_grid_counts() {
    let count = calc_paths(18, 18);
    println!("count: {}", count);
    assert!(count == 2333606220);
  }
}
