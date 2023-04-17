use std::collections::HashMap;

type Memo = HashMap<String, usize>;

fn calc_paths(m: usize, n: usize, memo: &mut Memo) -> usize {
  let grid_key = format!("{},{}", m, n);
  if memo.contains_key(&grid_key) {
    return memo.get(&grid_key).unwrap().clone();
  }

  if m == 0 || n == 0 {
    return 0;
  }

  if m == 1 && n == 1 {
    return 1;
  }

  let val1 = calc_paths(m - 1, n, memo);
  let val2 = calc_paths(m, n - 1, memo);

  memo.insert(grid_key.clone(), val1 + val2);

  return memo.get(&grid_key).unwrap().clone();
}

pub fn get_all_grid_paths(m: usize, n: usize) -> usize {
  let mut memo = Memo::new();

  return calc_paths(m, n, &mut memo);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn finds_correct_path_count() {
    let count = get_all_grid_paths(2, 3);
    assert!(count == 3);
  }

  #[test]
  fn finds_large_grid_counts() {
    let count = get_all_grid_paths(18, 18);
    println!("count: {}", count);
    assert!(count == 2333606220);
  }
}
