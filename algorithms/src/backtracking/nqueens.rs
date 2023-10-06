pub struct Solution {}

impl Solution {
  pub fn is_safe(n: i32, board: &Vec<Vec<bool>>, row: i32, col: i32) -> bool {
    let r = row as usize;
    let c = col as usize;
    let n = n as usize;

    // check other columns for collisions
    for col in c..=0 {
      if board[r][col] {
        println!("-horz col check failed q at -> r:{r} c:{col}");
        return false;
      }
    }

    for col in 0..=c {
      if board[r][col] {
        println!("+horz col check failed q at -> r:{r} c:{col}");
        return false;
      }
    }

    // check upper left diagonals for collisions
    let mut ri = Some(r);
    let mut ci = Some(c);
    loop {
      // considered unstable by Rust 1.58 (on leetcode)
      // (ri, ci) = (ri.unwrap().checked_sub(1), ci.unwrap().checked_sub(1));
      ri = ri.unwrap().checked_sub(1);
      ci = ci.unwrap().checked_sub(1);
      if ri.is_some() && ci.is_some() {
        if board[ri.unwrap()][ci.unwrap()] {
          println!(
            "upper left diag failed q at -> r:{} c:{}",
            ri.unwrap(),
            ci.unwrap()
          );
          return false;
        }
      } else {
        break;
      }
    }

    // check upper right diagonals for collisions
    let mut ri = Some(r);
    let mut ci = Some(c);
    loop {
      (ri, ci) = (ri.unwrap().checked_sub(1), ci.unwrap().checked_add(1));
      if ri.is_some() && ci.is_some() && ci.unwrap() < n {
        if board[ri.unwrap()][ci.unwrap()] {
          println!(
            "upper right diag failed q at -> r:{} c:{}",
            ri.unwrap(),
            ci.unwrap()
          );
          return false;
        }
      } else {
        break;
      }
    }

    // check lower left diagonals for collisions
    let mut ri = Some(r);
    let mut ci = Some(c);
    loop {
      (ri, ci) = (ri.unwrap().checked_add(1), ci.unwrap().checked_sub(1));
      if ri.is_some() && ci.is_some() && ri.unwrap() < n {
        if board[ri.unwrap()][ci.unwrap()] {
          println!(
            "lower left diag failed q at -> r:{} c:{}",
            ri.unwrap(),
            ci.unwrap()
          );
          return false;
        }
      } else {
        break;
      }
    }

    // check lower right diagonals for collisions
    let mut ri = Some(r);
    let mut ci = Some(c);
    loop {
      (ri, ci) = (ri.unwrap().checked_add(1), ci.unwrap().checked_add(1));
      if ri.unwrap() < n && ci.unwrap() < n {
        if board[ri.unwrap()][ci.unwrap()] {
          println!(
            "lower right diag failed q at -> r:{} c:{}",
            ri.unwrap(),
            ci.unwrap()
          );
          return false;
        }
      } else {
        break;
      }
    }

    true
    // for pos in r..=0 {
    //   if board[pos][c] {
    //     return false;
    //   }
    // }
    //
    // for pos in 0..=r {
    //   if board[pos][c] {
    //     return false;
    //   }
    // }
  }

  pub fn place_queen(board: &mut Vec<Vec<bool>>, row: i32, col: i32) {
    board[row as usize][col as usize] = true;
  }

  pub fn remove_queen(board: &mut Vec<Vec<bool>>, row: i32, col: i32) {
    board[row as usize][col as usize] = false;
  }

  pub fn queens_r(
    n: i32,
    mut board: &mut Vec<Vec<bool>>,
    col: i32,
    good_boards: &mut i32,
    d: u32,
  ) -> bool {
    println!(
      "d:{d} n:{n} col:{col} good_boards:{good_boards} board:{:#?}",
      &board
    );
    if col == n {
      // good solution, return true
      println!("good board found");
      *good_boards = *good_boards + 1;
      return true;
    }

    // find an empty row starting at 0
    for r in 0..n {
      // in current col, startig at 0 row, try placing queen and check if queen is safe
      println!("d:{d} main loop checking... col:{} r:{} ", col, r);
      if Solution::is_safe(n, &board, r, col) {
        println!("d:{d} placing q in row: {r} col: {col}");
        Solution::place_queen(&mut board, r, col);
        Solution::queens_r(n, &mut board, col + 1, good_boards, d + 1);
        Solution::remove_queen(&mut board, r, col);
      }
    }

    // return false if we fall through here, because we should
    // have returned true in the loop above if we found a solution
    false
  }

  pub fn total_n_queens(n: i32) -> i32 {
    let u_n = n as usize;
    let mut board = vec![vec![false; u_n]; u_n];
    let mut num_good_boards = 0;
    // println!("board: {:#?}", &board);

    Solution::queens_r(n, &mut board, 0, &mut num_good_boards, 0);

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
  fn nq_test1() {
    assert_eq!(Solution::total_n_queens(4), 2);
  }

  #[test]
  fn nq_test2() {
    assert_eq!(Solution::total_n_queens(1), 1);
  }
}