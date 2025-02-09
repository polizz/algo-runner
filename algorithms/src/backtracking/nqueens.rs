use std::iter::zip;
pub struct Solution {}

impl Solution {
  #[inline(always)]
  pub fn is_safe(n: usize, board: &Vec<Vec<bool>>, row: usize, col: usize) -> bool {
    let r = row;
    let c = col;

    // check all columns in the current row
    if (0..c).any(|col| board[r][col]) {
      return false;
    }

    // check all rows in the current col
    if (0..r).any(|row| board[row][c]) {
      return false;
    }

    // check upper left diagonals for collisions
    if zip((0..r).rev(), (0..c).rev()).any(|(row, col)| board[row][col]) {
      return false;
    }

    // check lower left diagonals for collisions
    if zip(r + 1..n, (0..c).rev()).any(|(row, col)| board[row][col]) {
      return false;
    }

    true
  }

  #[inline(always)]
  pub fn place_queen(board: &mut Vec<Vec<bool>>, row: usize, col: usize) {
    board[row][col] = true;
  }

  #[inline(always)]
  pub fn remove_queen(board: &mut Vec<Vec<bool>>, row: usize, col: usize) {
    board[row][col] = false;
  }

  pub fn queens_r(n: usize, mut board: &mut Vec<Vec<bool>>, col: usize, good_boards: &mut usize) {
    if col == n {
      *good_boards = *good_boards + 1;
      return;
    }

    // find an empty row starting at 0
    for r in 0..n {
      // in current col, startig at 0 row, try placing queen and check if queen is safe
      if Solution::is_safe(n, &board, r, col) {
        Solution::place_queen(&mut board, r, col);
        Solution::queens_r(n, &mut board, col + 1, good_boards);
        Solution::remove_queen(&mut board, r, col);
      }
    }
  }

  pub fn total_n_queens(n: usize) -> usize {
    let u_n = n;
    let mut board = vec![vec![false; u_n]; u_n];
    let mut num_good_boards = 0usize;

    Solution::queens_r(n, &mut board, 0, &mut num_good_boards);

    num_good_boards
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn is_safe() {
    let mut board = vec![vec![false; 4]; 4];
    board[2][0] = true;

    assert_eq!(Solution::is_safe(4, &board, 3, 3), true);
    assert_eq!(Solution::is_safe(4, &board, 0, 2), false);
    assert_eq!(Solution::is_safe(4, &board, 0, 1), true);
    assert_eq!(Solution::is_safe(4, &board, 0, 3), true);
    assert_eq!(Solution::is_safe(4, &board, 1, 3), true);
  }

  #[test]
  fn nq_test3() {
    let n = Solution::total_n_queens(12);
    println!("6x6 total slns: {n}");
  }

  #[test]
  fn nq_test1() {
    assert_eq!(Solution::total_n_queens(4), 2);
  }

  #[test]
  fn nq_test2() {
    assert_eq!(Solution::total_n_queens(1), 1);
  }
}
