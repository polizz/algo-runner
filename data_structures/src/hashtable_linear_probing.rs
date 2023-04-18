use std::fs::File;
use std::fmt::{Debug, Display};
use std::io::prelude::*;
use std::iter::zip;
use std::path::PathBuf;
use std::str;
use std::str::Chars;
use std::string::String;

const FILE_BUFF_SZ: usize = 4096;
const R: usize = 5381;

pub trait Hashable {
  fn get_hash(&self, m: usize) -> usize;

  fn hash(&self, c: Chars, m: usize) -> usize {
    (c.fold(R as usize, |r, e| {
      r.wrapping_mul(33).wrapping_add(e as usize) % m
    })) as usize
  }
}

impl Hashable for &str {
  fn get_hash(&self, m: usize) -> usize {
    self.hash(self.chars(), m)
  }
}

impl Hashable for String {
  fn get_hash(&self, m: usize) -> usize {
    self.hash(self.chars(), m)
  }
}

#[derive(Hash, Debug)]
pub struct HashTableLinear<K, V> {
  m: usize,
  keys: Vec<Option<K>>,
  values: Vec<Option<V>>,
}

impl<K, V> HashTableLinear<K, V>
where
  K: Clone + Hashable + Display + PartialEq + Debug,
  V: Clone + Display + Debug,
{
  pub fn new(m: usize) -> Self {
    HashTableLinear {
      m,
      keys: vec![None; m],
      values: vec![None; m],
    }
  }

  pub fn put(&mut self, key: K, value: V) {
    let mut assign_idx = key.get_hash(self.m);
    let existing_key = &self.keys[assign_idx];

    if let Some(ekey) = existing_key {
      if key != *ekey {
        // collision
        let next_bucket = self.get_probe_range(assign_idx, |&test_idx| match self.keys[test_idx] {
          Some(_) => false,
          None => true,
        });
        
        assign_idx = next_bucket.unwrap_or_else(|| {
          panic!(
            "Collision not resolved for new key {:?}. Existing key: {:?} at index {:?}",
            &key,
            &existing_key.as_ref().unwrap(),
            &assign_idx
          );
        });
      }
    }

    self.keys[assign_idx] = Some(key);
    self.values[assign_idx] = Some(value);
  }

  pub fn get(&self, key: &K) -> &Option<V> {
    let mut idx = key.get_hash(self.m);
    let existing_key = &self.keys[idx];

    match existing_key {
      Some(key_found) => {
        if key_found != key {
          let next_bucket = self.get_probe_range(idx, |&test_idx| match &self.keys[test_idx] {
            Some(test_key) => test_key == key,
            None => false,
          });

          if let Some(next_idx) = next_bucket {
            idx = next_idx;
            println!("Successful (get) probe for key: {}", &key);
          } else {
            return &None;
          };
        }
        &self.values[idx]
      }
      None => &None,
    }
  }

  fn delete_idx(&mut self, idx: usize, redo_range: Vec<usize>) -> (K, V) {
    let deleted_key = self.keys[idx].clone().unwrap();
    let deleted_val = self.values[idx].clone().unwrap();
    self.keys[idx] = None;
    self.values[idx] = None;
    // let deleted_val = self.values[idx].replace(Some("a")).unwrap();

    // let rehash_key_indexes = self.keys.iter()
    //   .skip(idx + 1)
    //   .take_while(|e| e.is_some())
    //   .enumerate()
    //   .map(|(idx, _)| idx)
    //   .collect::<Vec<usize>>();

    let mut rehash_keys= Vec::new();
    let mut rehash_values= Vec::new();


    println!("Redo range: {:#?}", &redo_range);

    redo_range
      .iter()
      .for_each(|&idx| {
        rehash_keys.push(self.keys.remove(idx));
        rehash_values.push(self.values.remove(idx));
      });

    zip(rehash_keys, rehash_values)
      .for_each(|(k, v)| {
        println!("ZIP: K={:#?} V={:#?}", &k, &v);
        self.put(k.unwrap(), v.unwrap())
      });

    (deleted_key, deleted_val)
  }

  pub fn delete(&mut self, key: &K) -> Option<(K, V)> {
    let idx = key.get_hash(self.m);
    let existing_key = &self.keys[idx];

    println!("Key: {:?} Existing Key: {:#?} Idx: {:#?}", &key, &existing_key, &idx);

    match existing_key {
      Some(key_found) => {
        if key_found != key {
          println!("Keys don't match...");

          let del_idx = self.get_probe_range(idx, |&test_idx| match &self.keys[test_idx] {
            Some(test_key) => test_key == key,
            None => false,
          });

          
          if let Some(new_idx) = del_idx {
            println!("Successful (del) probe for key: {} at idx: {}", &key, &new_idx);
            let redo_range: Vec<usize> = self.get_redo_range(new_idx);
            
            // probe found key, delete it and re-add everything below it up to the first None
            Some(self.delete_idx(new_idx, redo_range))
          } else {
            println!("Probe on del not successful");
            None
          }
        } else {
          // key matches, delete it and re-add everything below it up to the first None
          let redo_range: Vec<usize> = self.get_redo_range(idx);
          Some(self.delete_idx(idx, redo_range))
        }
      },
      _ => None
    }
  }

  fn get_redo_range(&self, del_idx: usize) -> Vec<usize> {
    if del_idx == self.keys.len() - 1 || del_idx == 0 {
      (0..(self.keys.len() - 1))
        .take_while(|&n| n != del_idx && self.keys[n].is_some())
        .collect()
    } else {
      (del_idx + 1..self.keys.len() - 1)
        .chain(0..del_idx - 1)
        .take_while(|&n| self.keys[n].is_some())
        .collect()
    }
  }

  fn get_probe_range(
    &self,
    start_idx: usize,
    local_filter: impl FnMut(&usize) -> bool,
  ) -> Option<usize> {
    if start_idx == self.keys.len() - 1 || start_idx == 0 {
      (0..(self.keys.len() - 1))
        .filter(|&n| n != start_idx)
        .find(local_filter)
    } else {
      (start_idx + 1..self.keys.len() - 1)
        .chain(0..start_idx - 1)
        .find(local_filter)
    }
  }
}

pub fn get_words_from_file(word_file_path: PathBuf) -> String {
  let mut word_file = File::open(word_file_path).expect("Input file was not found");

  let mut file_buffer = [0u8; FILE_BUFF_SZ];
  let mut read_bytes;
  let mut str_repr = String::new();

  loop {
    read_bytes = word_file.read(&mut file_buffer).expect("whoa");

    if read_bytes == 0 {
      break;
    }

    let trim_bytes = file_buffer
      .into_iter()
      .filter(|&b| {
        match b {
          0 | 33..=47 | 58..=64 | 91..=96 | 123..=126 => false, // remove all punctuation
          _ => true,
        }
      })
      .collect::<Vec<u8>>();

    let str_slice = trim_bytes.as_slice();
    str_repr.push_str(str::from_utf8(str_slice).unwrap());
  }

  str_repr.to_ascii_lowercase()
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
  fn get_non_existent_key() {
    let ht: HashTableLinear<&str, usize> = HashTableLinear::new(3);

    println!("Before del: {:?}", &ht);
    // println!("Hash: {}", "two".get_hash(3));
    
    // ht.put("two", 1000);
    assert_eq!(ht.get(&"Does not exist"), &None);
  }

  #[test]
  fn can_delete_key() {
    let mut ht: HashTableLinear<&str, usize> = HashTableLinear::new(3);

    ht.put(" ", 999);
    ht.put("a billion", 834);

    assert_eq!(ht.get(&" "), &Some(999));
    assert_eq!(ht.get(&"a billion"), &Some(834));

    ht.delete(&"a billion");
    assert_eq!(ht.get(&"a billion"), &None);
    
    ht.put("two", 1000);
    assert_eq!(ht.get(&"two"), &Some(1000));
    
    // println!("After del: {:?}", &ht);
  }

  #[test]
  fn can_wrap_around_probe() {
    let mut ht: HashTableLinear<&str, usize> = HashTableLinear::new(3);

    ht.put(" ", 0);
    ht.put("a billion", 1);
    ht.put("two", 1000);

    // println!("{:#?}", &ht);
    // println!("Hash: {}", "two".get_hash(3));

    assert_eq!(ht.get(&"a billion"), &Some(1));
    assert_eq!(ht.get(&" "), &Some(0));
    assert_eq!(ht.get(&"two"), &Some(1000));
  }

  #[test]
  fn can_get_values() {
    let mut ht: HashTableLinear<&str, usize> = HashTableLinear::new(31);

    ht.put(" ", 0);
    ht.put("a billion", 1);

    assert_eq!(*ht.get(&"a billion"), Some(1));
    assert_eq!(*ht.get(&" "), Some(0));

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

    assert_eq!(ht.keys[6], Some(" "));
    assert_eq!(ht.values[6], Some(0));

    assert_eq!(ht.keys[1], Some("a billion"));
    assert_eq!(ht.values[1], Some(1));

    assert_eq!(ht.keys.len(), 31);
    assert_eq!(ht.values.len(), 31);
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

    path.push("fixtures/words.txt");
    let _words = get_words_from_file(path);

    assert_eq!(_words, expected_words);
  }

  #[test]
  #[ignore]
  fn can_read_large_word_file() {
    let mut path = std::env::current_dir().expect("whoa");

    // println!("Directory starting with: {}", &path.display());
    path.push("fixtures/sentences.txt");
    // println!("Directory used: {}", &path.display());

    let words = get_words_from_file(path);

    let words: Vec<&str> = words.split_ascii_whitespace().collect();
    let mut ht: HashTableLinear<&str, usize> = HashTableLinear::new(5731);

    words.iter().enumerate().for_each(|(ix, val)| {
      ht.put(val.clone(), ix);
    });

    // println!("HT: {:#?}", &ht);

    // assert_eq!(_words, expected_words);
  }
}
