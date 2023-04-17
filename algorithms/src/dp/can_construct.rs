use std::collections::HashMap;

type Memo = HashMap<&'static str, bool>;

fn do_can_construct(target: &'static str, wordbank: &Vec<&str>, memo: &mut Memo) -> bool {
  if memo.contains_key(target) {
    return memo.get(&target).unwrap().clone();
  }

  if target == "" {
    return true;
  }

  for &word in wordbank {
    if let Some(idx) = target.find(word) {
      if idx == 0 {
        let suffix = &target[word.len()..];
        if do_can_construct(suffix, wordbank, memo) {
          memo.insert(suffix.clone(), true);
          return true;
        }
      }
    }
  }

  let false_string = target.clone();
  memo.insert(false_string, false);
  return false;
}

pub fn can_construct(m: &'static str, n: Vec<&str>) -> bool {
  let mut memo = Memo::new();
  return do_can_construct(m, &n, &mut memo);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_construct_small_string() {
    let can_construct = can_construct("abcdef", vec!["ab", "abc", "cd", "def", "abcd"]);
    assert!(can_construct);
  }

  #[test]
  fn cannot_construct_string() {
    let can_construct = can_construct(
      "skateboard",
      vec!["bo", "rd", "ate", "t", "ska", "sk", "boar"],
    );
    assert!(!can_construct);
  }
  #[test]
  fn can_construct_large_string() {
    let can_construct = can_construct(
      "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
      vec!["e", "ee", "eee", "eeee", "eeeeee"],
    );
    assert!(!can_construct);
  }
}
