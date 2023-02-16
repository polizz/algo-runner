pub fn count_construct_tab(target: &'static str, wordbank: &Vec<&str>) -> usize {
  let target_len = target.len();
  let mut table = vec![0; target_len + 1];
  table[0] = 1;

  (0..target.len()).for_each(|i| {
    // println!("checking: {}, table[{}]={}", &target[i..], &i, &table[i]);
    if table[i] > 0 {
      let current_prefix = &target[i..];
      // println!("outer {}", &current_prefix);
  
      for &word in wordbank {
        if current_prefix.starts_with(word) {
          table[i + word.len()] = table[i] + table[i + word.len()];
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
  fn count_construct_count_1_small_string() {
    let count_construct_res = count_construct_tab("abcdef", &vec!["ab", "abc", "cd", "def", "abcd"]);
    assert!(count_construct_res == 1);
  }

  #[test]
  fn count_construct_count_2_small_string() {
    let count_construct_res = count_construct_tab("purple", &vec!["purp", "p", "ur", "le", "purpl"]);
    assert!(count_construct_res == 2);
  }

  #[test]
  fn count_construct_count_0_small_string() {
    let count_construct_res = count_construct_tab("skateboard", &vec!["bo", "rd", "ate", "t", "ska", "sk", "boar"]);
    assert!(count_construct_res  == 0);
  }

  #[test]
  fn count_construct_count_0_large_string() {
    let count_construct_res = count_construct_tab("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef", &vec!["e", "ee", "eee", "eeee", "eeeeee"]);
    assert!(count_construct_res == 0);
  }
}