use std::collections::HashMap;

type Memo = HashMap<&'static str, Option<Vec<Vec<&'static str>>>>;

fn do_all_construct(
  target: &'static str,
  wordbank: &Vec<&'static str>,
  memo: &mut Memo,
) -> Option<Vec<Vec<&'static str>>> {
  if memo.contains_key(target) {
    return memo.get(&target).unwrap().clone();
  }

  if target == "" {
    return Some(vec![vec![]]);
  }

  let mut found_completions = vec![];

  for &word in wordbank {
    if let Some(idx) = target.find(word) {
      if idx == 0 {
        let suffix = &target[word.len()..];
        let words_that_construct = do_all_construct(suffix, wordbank, memo);

        if let Some(mut words_found) = words_that_construct {
          for inner_words in words_found.iter_mut() {
            inner_words.insert(0, &word);
          }

          found_completions.extend(words_found);
          memo.insert(target.clone(), Some(found_completions.clone()));
        }
      }
    }
  }

  return Some(found_completions);
}

pub fn can_all_construct(m: &'static str, n: Vec<&'static str>) -> Option<Vec<Vec<&'static str>>> {
  let mut memo = Memo::new();
  return do_all_construct(m, &n, &mut memo);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_construct_all_1_small_string() {
    let all_construct_results =
      can_all_construct("abcdef", vec!["ab", "abc", "cd", "def", "abcd", "ef", "c"]);
    assert!(
      all_construct_results
        == Some(vec![
          vec!["ab", "cd", "ef"],
          vec!["ab", "c", "def"],
          vec!["abc", "def"],
          vec!["abcd", "ef"]
        ])
    );
  }

  #[test]
  fn can_construct_all_2_small_string() {
    let all_construct_results = can_all_construct("purple", vec!["purp", "p", "ur", "le", "purpl"]);
    println!("OUT {:?}", all_construct_results);
    assert!(all_construct_results == Some(vec![vec!["purp", "le"], vec!["p", "ur", "p", "le"]]));
  }

  #[test]
  fn can_construct_all_0_small_string() {
    let all_construct_results = can_all_construct(
      "skateboard",
      vec!["bo", "rd", "ate", "t", "ska", "sk", "boar"],
    );
    assert!(all_construct_results == Some(vec![]));
  }

  #[test]
  fn can_construct_all_0_large_string() {
    let all_construct_results = can_all_construct(
      "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaz",
      vec!["a", "aa", "aaa", "aaaa", "aaaaa"],
    );
    assert!(all_construct_results == Some(vec![]));
  }
}
