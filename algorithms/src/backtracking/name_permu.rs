struct Permu {}

impl Permu {
  #[allow(unused)]
  pub fn smoke_r<'main>(
    letters: &Vec<&str>,
    words: &mut Vec<String>,
    word: &mut Vec<String>,
    used_letters: &mut Vec<bool>,
    k: usize,
  ) {
    if word.len() == k {
      words.push(word.join(""));
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
        word.push(letters[l].to_owned());
        Permu::smoke_r(&letters, words, word, used_letters, k);
        word.pop();
        used_letters[l] = false;
      }
    }
  }

  #[allow(unused)]
  pub fn smoke() {
    // let letters = vec!["a", "n", "d", "a", "s", "h", "l", "i", "a"];
    let letters = vec!["a", "a", "l"];
    let mut words = vec![];
    let mut word: Vec<String> = vec![];
    let mut used_letters = vec![false; 9];
    let len = letters.len();

    Permu::smoke_r(&letters, &mut words, &mut word, &mut used_letters, len);

    println!("Words:");
    println!("{:?}", &words);
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
