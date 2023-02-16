// [
// a    Some([])
// b    None,
// c    Some([["ab"]]),
// d    Some([["abc"]]),
// e    Some([["abcd"], ["ab", "cd"]]),
// f    None
// ''   None
// ]

use std::borrow::Borrow;
use std::vec;
use std::cell::RefCell;

type OptionVector = Option<Vec<Vec<&'static str>>>;

pub fn all_construct(target: &'static str, wordbank: &Vec<&'static str>) -> OptionVector {
  let mut table: Vec<RefCell<OptionVector>> = vec![RefCell::new(None); target.len() + 1];
  table[0] = RefCell::new(Some(vec![]));

  (0..target.len()).for_each(|i| {
    let ref_cell = &table[i];
    let cell_source_vector = ref_cell.borrow();

    // println!("checking: {}, table[{}]={:?}", &target[i..], &i, &table[i]);

    if cell_source_vector.is_some() {
      let current_prefix = &target[i..];
      println!("outer {}", &current_prefix);
  
      for &word in wordbank {
        if current_prefix.starts_with(word) {
          let mut new_vec = (*cell_source_vector.borrow()).clone().unwrap();

          if new_vec.len() < 1 {
            new_vec.insert(0, vec![word]);
          } else {
            new_vec.iter_mut().for_each(|vecs| vecs.insert(vecs.len(), word));
          }

          let forward_ref_cell = &table[i + word.len()];
          let mut forward_vec = forward_ref_cell.borrow_mut();

          // println!("Word match={}, Forward vec: {:?}, forward_vec.is_some()={}, New Vec: {:?}", &word, &forward_vec, forward_vec.is_some(), &new_vec);
          
          if forward_vec.is_some() {
            let mut value = (*forward_vec.borrow()).clone().unwrap();
            value.extend(new_vec);
            forward_vec.replace(value);
          } else {
            forward_vec.replace(new_vec);
          }
          
        }
      }
    }
  });

  // println!("Table {:#?}", &table);
  println!("Return {:#?}", table[target.len()].borrow().clone());

  return table[target.len()].borrow().clone();
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn can_construct_all_1_small_string() {
    let all_construct_results = all_construct("abcdef", &vec!["ab", "abc", "cd", "def", "abcd", "ef", "c"]);
    assert!(all_construct_results == Some(vec![ vec!["abc", "def"], vec!["ab", "c", "def"], vec!["abcd", "ef"], vec!["ab", "cd", "ef"] ]));
  }

  #[test]
  fn can_construct_empty() {
    let all_construct_results = all_construct("", &vec!["a", "b"]);
    assert!(all_construct_results == Some(vec![]));
  }

  #[test]
  fn can_construct_all_2_small_string() {
    let all_construct_results = all_construct("purple", &vec!["purp", "p", "ur", "le", "purpl"]);
    println!("OUT {:?}", all_construct_results);
    assert!(all_construct_results == Some(vec![ vec!["purp", "le"], vec!["p", "ur", "p", "le"] ] ));
  }

  #[test]
  fn can_construct_all_0_small_string() {
    let all_construct_results = all_construct("skateboard", &vec!["bo", "rd", "ate", "t", "ska", "sk", "boar"]);
    assert!(all_construct_results  == None);
  }

  #[test]
  fn can_construct_all_0_large_string() {
    let all_construct_results = all_construct("aaaaaaaaaaaaaz", &vec!["a", "aa", "aaa", "aaaa", "aaaaa"]);
    assert!(all_construct_results == None);
  }
}