use std::collections::HashMap;

type Memo = HashMap<i64, i64>;

pub fn do_fib_recurse(n: i64, memo: &mut Memo) -> i64 {
  if let Some(memo_item) = memo.get(&n) {
    return memo_item.clone();
  }
  match n {
    n if n <= 2 => 1,
    _ => {
      let fib = do_fib_recurse(n - 1, memo) + do_fib_recurse(n - 2, memo);
      memo.insert(n, fib.clone());
      return fib;
    }
  }
}

pub fn calc_fib_recurse(n: i64) -> i64 {
  let mut memo = Memo::new();
  return do_fib_recurse(n, &mut memo);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_calc_fib_9() {
    assert!(calc_fib_recurse(9) == 34);
  }

  #[test]
  fn can_calc_fib_6() {
    assert!(calc_fib_recurse(6) == 8);
  }

  #[test]
  fn can_calc_fib_50() {
    assert!(calc_fib_recurse(50) == 12586269025);
  }
}