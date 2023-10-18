struct Permu {}

impl Permu {
  pub fn smoke_r<'main>(letters: &Vec<u8>, words: &mut Vec<String>, word: &mut Vec<String>, used_letters: &mut Vec<bool>, k: usize) {
    if word.len() == k {
        let to_string: String = word.clone().iter().fold("".to_string(), |mut acc, l| {
          acc.push_str(l);
          acc
        });
        words.push(to_string);
        return;
      // match word[0].as_bytes() {
      //   b"l" => { //| b"s" | b"n" | b"i" 
      //   } 
      //   _ => return,
      // }
    }

    for l in 0..k {
      if used_letters[l] {
        continue;
      } else {
        used_letters[l] = true;
        word.push(letters[l].clone().to_string());
        Permu::smoke_r(&letters, words, word, used_letters, k);
        word.pop();
        used_letters[l] = false;
      }
    }
  }

  pub fn smoke() {
    // let letters = vec!["a", "n", "d", "a", "s", "h", "l", "i", "a"];
    let letters = vec![b'a', b'a', b'l'];
    let mut words = vec![];
    let mut word: Vec<String> = vec![];
    let mut used_letters = vec![false; 9];
    let len = letters.len();
  
    Permu::smoke_r(&letters, &mut words, &mut word, &mut used_letters, len);

    println!("{:#?}", &words);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn smoke() {
    Permu::smoke();
    assert!(true);
  }
}
