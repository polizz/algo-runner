pub fn two_num_calc_fib(n: usize) -> i64 {
  let mut num_0 = 0;
  let mut num_1 = 1;
  let mut num_fib = 0;

  if n == 1 {
    return 1;
  }

  (1..n).for_each(|_i| {
    num_fib = num_0 + num_1;
    num_0 = num_1;
    num_1 = num_fib;
  });

  num_fib
}

pub fn calc_fib(n: usize) -> i64 {
  let mut fib_tab = vec![0; n + 1];
  fib_tab[1] = 1;

  (2..fib_tab.len()).for_each(|i| fib_tab[i] = fib_tab[i - 1] + fib_tab[i - 2]);

  fib_tab[n]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_calc_fib_9() {
    assert!(calc_fib(9) == 34);
  }

  #[test]
  fn can_calc_fib_6() {
    assert!(calc_fib(6) == 8);
  }

  #[test]
  fn can_calc_fib_50() {
    println!("{}", calc_fib(50));
    assert!(calc_fib(50) == 12586269025);
  }

  #[test]
  fn can_two_num_calc_fib_1() {
    assert!(two_num_calc_fib(1) == 1);
  }

  #[test]
  fn can_two_num_calc_fib_2() {
    assert!(two_num_calc_fib(2) == 1);
  }

  #[test]
  fn can_two_num_calc_fib_9() {
    assert!(two_num_calc_fib(9) == 34);
  }

  #[test]
  fn can_two_num_calc_fib_6() {
    assert!(two_num_calc_fib(6) == 8);
  }

  #[test]
  fn can_two_num_calc_fib_50() {
    assert!(two_num_calc_fib(50) == 12586269025);
  }
}