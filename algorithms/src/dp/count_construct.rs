use std::collections::HashMap;

type Memo = HashMap<&'static str, i32>;

fn do_count_construct(target: &'static str, wordbank: &Vec<&str>, memo: &mut Memo) -> i32 {
  if memo.contains_key(target) {
    return memo.get(&target).unwrap().clone();
  }

  if target == "" {
    return 1;
  }

  let mut branch_counts = 0;

  for &word in wordbank {
    if let Some(idx) = target.find(word) {
      if idx == 0 {
        let suffix = &target[word.len()..];
        let count = do_count_construct(suffix, wordbank, memo);
        branch_counts = branch_counts + count;
      }
    }
  }

  memo.insert(target, branch_counts);
  return branch_counts;
}

pub fn can_construct_count(m: &'static str, n: Vec<&str>) -> i32 {
  let mut memo = Memo::new();
  return do_count_construct(m, &n, &mut memo);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_construct_count_1_small_string() {
    let can_construct_count = can_construct_count("abcdef", vec!["ab", "abc", "cd", "def", "abcd"]);
    assert!(can_construct_count == 1);
  }

  #[test]
  fn can_construct_count_2_small_string() {
    let can_construct_count = can_construct_count("purple", vec!["purp", "p", "ur", "le", "purpl"]);
    assert!(can_construct_count == 2);
  }

  #[test]
  fn can_construct_count_0_small_string() {
    let can_construct_count = can_construct_count(
      "skateboard",
      vec!["bo", "rd", "ate", "t", "ska", "sk", "boar"],
    );
    assert!(can_construct_count == 0);
  }

  #[test]
  fn can_construct_count_0_large_string() {
    let can_construct_count = can_construct_count(
      "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
      vec!["e", "ee", "eee", "eeee", "eeeeee"],
    );
    assert!(can_construct_count == 0);
  }
}
