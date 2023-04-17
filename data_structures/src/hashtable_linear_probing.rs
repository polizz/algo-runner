use std::fs::File;
// use std::hash::{Hash, Hasher};
// use std::collections::hash_map::DefaultHasher;
use std::fmt::{Debug, Display};
use std::io::prelude::*;
use std::path::PathBuf;
use std::string::String;

const BUFF_SZ: usize = 1024;
const R: usize = 5381;

// type K = String;

pub trait Hashable {
  fn get_hash(&self, m: usize) -> usize;
}

impl Hashable for &str {
  fn get_hash(&self, m: usize) -> usize {
    (self.chars().fold(7 as usize, |a, e| {
      a.wrapping_mul(R).wrapping_add(e as usize) % m as usize
    })) as usize
  }
}

impl Hashable for String {
  fn get_hash(&self, m: usize) -> usize {
    // not as good, clutering around upper end of range
    (self.chars().fold(7 as usize, |a, e| {
      a.wrapping_mul(R).wrapping_add(e as usize) % m as usize
    })) as usize
    // (self.chars().fold(R, |a, e| a.wrapping_mul(R).wrapping_add(e as u16)) % M as u16) as usize
  }
}

pub fn get_words_from_file(word_file_path: PathBuf) -> String {
  let mut word_file = File::open(word_file_path).expect("Input file was not found");

  let mut file_buffer = [0u8; BUFF_SZ];
  let mut read_buffer = [0u8; 4096];

  let mut read_bytes;
  let mut total_bytes = 0;

  loop {
    read_bytes = word_file.read(&mut file_buffer).expect("whoa");

    if read_bytes == 0 {
      break;
    }

    read_buffer[total_bytes..(total_bytes + read_bytes)]
      .copy_from_slice(&file_buffer[..read_bytes]);
    total_bytes += read_bytes;
  }

  let trim_bytes = read_buffer
    .into_iter()
    .filter(|&b| b > 0)
    .collect::<Vec<u8>>();

  let str_repr = String::from_utf8(trim_bytes).expect("Not a valid UTF-8 string");

  str_repr
}

#[derive(Hash, Debug)]
pub struct HashTableLinear<K, V> {
  m: usize,
  keys: Vec<Option<K>>,
  values: Vec<Option<V>>,
}

impl<K, V> HashTableLinear<K, V>
where
  K: Clone + Hashable + Display + PartialEq,
  V: Clone + Display + Debug,
{
  pub fn put(&mut self, key: K, value: V) {
    let mut assign_idx = key.get_hash(self.m);
    let existing_key = &self.keys[assign_idx];

    if let Some(ekey) = existing_key {
      if key != *ekey {
        // collision
        let search_range = self.get_probe_range(assign_idx);
        let next_bucket = search_range
          .into_iter()
          .find(|&test_idx| match self.keys[test_idx] {
            Some(_) => false,
            None => true,
          })
          .unwrap();

        assign_idx = next_bucket;
      }
    }

    self.keys[assign_idx] = Some(key);
    self.values[assign_idx] = Some(value);
  }

  pub fn get(&self, key: K) -> &Option<V> {
    let mut idx = key.get_hash(self.m);
    let existing_key = &self.keys[idx];

    match existing_key {
      Some(key_found) => {
        if key_found != &key {
          let search_range = self.get_probe_range(idx);
          let next_bucket = search_range
            .into_iter()
            .find(|&test_idx| match &self.keys[test_idx] {
              Some(test_key) => *test_key == key,
              None => false,
            });

          if let Some(next_idx) = next_bucket {
            idx = next_idx
          } else {
            return &None;
          };
        }
        &self.values[idx]
      }
      None => &None,
    }
  }

  fn get_probe_range(&self, start_idx: usize) -> Vec<usize> {
    if start_idx == self.keys.len() - 1 || start_idx == 0 {
      (0..(self.keys.len() - 1))
        .filter(|&n| n != start_idx)
        .collect()
    } else {
      (start_idx + 1..self.keys.len() - 1)
        .chain(0..start_idx - 1)
        .collect()
    }
  }

  pub fn new(m: usize) -> Self {
    HashTableLinear {
      m,
      keys: vec![None; m],
      values: vec![None; m],
    }
  }

  fn keys(&self) -> &Vec<Option<K>> {
    &self.keys
  }

  fn values(&self) -> &Vec<Option<V>> {
    &self.values
  }
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use super::*;

  #[test]
  #[ignore]
  fn combine_ranges() {
    let combined_vec = (0..=10).chain(13..=17).collect::<Vec<usize>>();
    println!("{:#?}", &combined_vec);
  }

  #[test]
  fn can_wrap_around_probe() {
    let mut ht: HashTableLinear<&str, usize> = HashTableLinear::new(3);

    ht.put(" ", 0);
    ht.put("a billion", 1);
    ht.put("two", 1000);

    // println!("{:#?}", &ht);
    // println!("Hash: {}", "two".get_hash(3));

    assert_eq!(ht.get("a billion"), &Some(1));
    assert_eq!(ht.get(" "), &Some(0));
    assert_eq!(ht.get("two"), &Some(1000));
  }

  #[test]
  fn can_get_values() {
    let mut ht: HashTableLinear<&str, usize> = HashTableLinear::new(31);

    ht.put(" ", 0);
    ht.put("a billion", 1);

    assert_eq!(*ht.get("a billion"), Some(1));
    assert_eq!(*ht.get(" "), Some(0));

    // println!("{:#?}", &ht);
  }

  #[test]
  #[ignore]
  fn can_make_hashes() {
    let hashes = vec![
      "1",
      " ",
      "one ",
      "one",
      "two",
      "three",
      "sixteen",
      "Ashley",
      "a billion",
    ]
    .iter()
    .map(|s| String::from_str(s).unwrap().get_hash(571))
    // .map(|s| String::from_str(s).unwrap().get_hash(571))
    .collect::<Vec<usize>>();

    println!("Hashes: {:#?}", hashes);
  }

  #[test]
  fn put_can_linear_probe() {
    let words = vec![" ", "a billion"];
    let mut ht: HashTableLinear<&str, usize> = HashTableLinear::new(31);

    words.into_iter().enumerate().for_each(|(ix, val)| {
      ht.put(val, ix);
    });

    // println!("HT: {:?}", &ht);

    assert_eq!(ht.keys()[3], Some(" "));
    assert_eq!(ht.values()[3], Some(0));

    assert_eq!(ht.keys()[4], Some("a billion"));
    assert_eq!(ht.values()[4], Some(1));

    assert_eq!(ht.keys().len(), 31);
    assert_eq!(ht.values().len(), 31);
  }

  #[test]
  fn can_read_words_from_file() {
    let mut path = std::env::current_dir().expect("whoa");
    let expected_words = "one two
three
four five six


seven
8 nine
";

    // println!("Directory starting with: {}", &path.display());
    path.push("fixtures/words.txt");
    // println!("Directory used: {}", &path.display());

    let _words = get_words_from_file(path);

    assert_eq!(_words, expected_words);
  }
}
