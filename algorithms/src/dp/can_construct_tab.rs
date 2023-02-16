pub fn can_construct_tab(target: &'static str, wordbank: &Vec<&str>) -> bool {
  let target_len = target.len();
  let mut table = vec![false; target_len + 1];
  table[0] = true;

  /*
   Table index 0 is is seeded with true because the empty string (i.e. an imaginary -1..0 slice) can always be found without any matching.
   Table indices 0 up to but not including table length correspond to the target string letters and represent each substring's reachability by words in the bank.
   The last table indice at table.len() represents the reachability of the ending of the entire target string by all previous words in the bank.
  */

  (0..target.len()).for_each(|i| {
    // println!("checking: {}, table[{}]={}", &target[i..], &i, &table[i]);
    if table[i] {
      let current_prefix = &target[i..];
      // println!("outer {}", &current_prefix);
  
      for &word in wordbank {
        if current_prefix.starts_with(word) {
          table[i + word.len()] = true;
        }
      }
    }
  });
  
  println!("table: {:#?}", &table);
  return table[target.len()];
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_construct_small_string() {
    let can_construct = can_construct_tab("abcdef", &vec!["ab", "abc", "cd", "def", "abcd"]);
    assert!(can_construct);
  }

  #[test]
  fn cannot_construct_string() {
    let can_construct = can_construct_tab("skateboard", &vec!["bo", "rd", "ate", "t", "ska", "sk", "boar"]);
    assert!(!can_construct);
  }
  #[test]
  fn can_construct_large_string() {
    let can_construct = can_construct_tab("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef", &vec!["e", "ee", "eee", "eeee", "eeeeee"]);
    assert!(!can_construct);
  }
}