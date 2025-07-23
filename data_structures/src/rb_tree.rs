// miri testing: cargo +nightly-2024-06-11 miri test tree_rotates

use std::cmp;
use std::fmt::{Debug, Display};
use std::ptr;

pub trait TreeStat {
  fn get_min_max_black_depth(&self) -> (usize, usize);
}

impl<K, V> TreeStat for BST<K, V>
where
  K: Debug + Display + Default,
  V: Debug + Display + Clone + Default,
{
  fn get_min_max_black_depth(&self) -> (usize, usize) {
    fn get_heights_r<K, V>(
      node: Link<K, V>,
      mut min: usize,
      mut max: usize,
      mut black_depth: usize,
    ) -> (usize, usize)
    where
      K: Debug + Display + Default,
      V: Debug + Display + Clone,
    {
      unsafe {
        if node.is_null() {
          if black_depth > max {
            max = black_depth;
          }
          if black_depth < min {
            min = black_depth;
          }

          return (min, max);
        }

        if (*node).color == Color::Black {
          black_depth = black_depth.wrapping_add(1);
        }

        let (min_l, max_l) = get_heights_r((*node).left, min, max, black_depth);
        let (min_r, max_r) = get_heights_r((*node).right, min, max, black_depth);

        (cmp::min(min_l, min_r), cmp::max(max_l, max_r))
      }
    }

    let min = usize::MAX;
    let max = 0;

    get_heights_r(self.root, min, max, 0)
  }
}

type Link<K, V> = *mut Node<K, V>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
  Red,
  Black,
}

#[derive(Debug, Clone)]
pub struct Node<K, V>
where
  K: Debug + Display,
  V: Debug + Display + Clone,
{
  key: K,
  pub value: V,
  right: Link<K, V>,
  left: Link<K, V>,
  color: Color,
  count: usize,
}

impl<K, V> Default for Node<K, V>
where
  K: Default + Debug + Display,
  V: Default + Debug + Display + Clone,
{
  fn default() -> Self {
    Node {
      key: K::default(),
      value: V::default(),
      right: ptr::null_mut(),
      left: ptr::null_mut(),
      color: Color::Red,
      count: 1,
    }
  }
}

impl<K, V> Display for Node<K, V>
where
  K: Default + Debug + Display,
  V: Default + Debug + Display + Clone,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    fn print_tree<K: Display + Debug, V: Display + Debug + Clone>(
      indent: &mut String,
      tree: Link<K, V>,
    ) -> String {
      unsafe {
        if tree.is_null() {
          format!("{}└── Empty\n", &indent)
        } else {
          let mut tree_string = format!("{}├──{} ({:?})\n", &indent, &(*tree).key, &(*tree).color);
          indent.push_str("│  ");
          tree_string.push_str(&print_tree(&mut indent.clone(), (*tree).right));
          tree_string.push_str(&print_tree(&mut indent.clone(), (*tree).left));

          tree_string
        }
      }
    }

    write!(
      f,
      "{}",
      print_tree(
        &mut String::new(),
        self as *const Node<K, V> as *mut Node<K, V>
      )
    )
  }
}

impl<K, V> Display for BST<K, V>
where
  K: Default + Debug + Display,
  V: Default + Debug + Display + Clone,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    unsafe { write!(f, "bst:\n{}", *self.root) }
  }
}

impl<K, V> Drop for BST<K, V>
where
  K: Default + Debug + Display,
  V: Default + Debug + Display + Clone,
{
  fn drop(&mut self) {
    fn drop_r<K, V>(node: Link<K, V>)
    where
      K: Default + Debug + Display,
      V: Default + Debug + Display + Clone,
    {
      unsafe {
        if !(*node).left.is_null() {
          drop_r((*node).left)
        }
        if !(*node).right.is_null() {
          drop_r((*node).right)
        }
        if !node.is_null() {
          drop(Box::from_raw(node));
        }
      }
    }
    drop_r(self.root);
  }
}

#[derive(Debug)]
pub struct BST<K, V>
where
  K: Default + Debug + Display,
  V: Default + Debug + Display + Clone,
{
  root: Link<K, V>,
}

impl<K, V> BST<K, V>
where
  K: Default + Debug + Display + Eq + Ord,
  V: Default + Debug + Display + Clone + 'static,
{
  pub fn new() -> Self {
    BST {
      root: ptr::null_mut(),
    }
  }

  pub fn get(&self, key: &'static K) -> Option<&V> {
    BST::get_node(self.root, key)
  }

  fn get_node(node: Link<K, V>, key: &'static K) -> Option<&V> {
    unsafe {
      if !node.is_null() {
        if key > &(*node).key {
          BST::get_node((*node).right, key)
        } else if key < &(*node).key {
          BST::get_node((*node).left, key)
        } else {
          Some(&(*node).value)
        }
      } else {
        return None;
      }
    }
  }

  pub fn size(&self) -> usize {
    BST::size_tree(self.root)
  }

  fn size_tree(node: Link<K, V>) -> usize {
    unsafe {
      if !node.is_null() {
        (*node).count
      } else {
        0
      }
    }
  }

  pub fn put(&mut self, key: K, value: V) {
    let node = self.put_r(self.root, key, value);

    unsafe {
      (*node).color = Color::Black;
    }

    self.root = node;
  }

  fn put_r(&self, h: Link<K, V>, key: K, value: V) -> Link<K, V> {
    unsafe {
      if !h.is_null() {
        if key > (*h).key {
          // println!("key({}) > h.key({})", &key, &(*h).key);
          (*h).right = self.put_r((*h).right, key, value);
        } else if key <= (*h).key {
          // println!("key({}) < h.key({})", &key, &(*h).key);
          (*h).left = self.put_r((*h).left, key, value);
        } else {
          // println!("key({}) == h.key({})", &key, &(*h).key);
          (*h).value = value;
        }

        (*h).count = 1 + BST::size_tree((*h).left) + BST::size_tree((*h).right);

        BST::balance(h)

        // println!("Before Balance");
        // println!("{}", *h);
        // let ret = BST::balance(h);
        // println!("After Balance");
        // println!("{}", *ret);
        // ret
      } else {
        Box::into_raw(Box::new(Node {
          key,
          value,
          right: ptr::null_mut(),
          left: ptr::null_mut(),
          color: Color::Red,
          count: 1,
        }))
      }
    }
  }

  pub fn is_red(node: Link<K, V>) -> bool {
    unsafe {
      if !node.is_null() {
        match (*node).color {
          Color::Red => true,
          Color::Black => false,
        }
      } else {
        false
      }
    }
  }

  #[inline(always)]
  pub fn flip_colors(h: Link<K, V>) {
    unsafe {
      (*h).color = Color::Red;
      (*(*h).left).color = Color::Black;
      (*(*h).right).color = Color::Black;
    }
  }

  fn balance(mut h: Link<K, V>) -> Link<K, V> {
    unsafe {
      // if red node on right while no red node on left is not allowed, rotate left to fix
      if (BST::is_red((*h).right)) && !BST::is_red((*h).left) {
        println!("rotating left");
        h = BST::rotate_left(h);
      }

      // if both left child and left grandchild are red, rotate right
      if (BST::is_red((*h).left)) && BST::is_red((*(*h).left).left) {
        println!("rotating right");
        h = BST::rotate_right(h);
      }

      // if both children are red, flip colors
      if BST::is_red((*h).left) && BST::is_red((*h).right) {
        println!("flipping colors");
        BST::flip_colors(h.clone());
      }

      h
    }
  }

  #[inline(always)]
  pub fn rotate_left(h: Link<K, V>) -> Link<K, V> {
    // H is above X and is < X. X is on H's right.
    // They will switch places and X
    // being > H, will be above H and H will be on X's left.
    //
    //   h             x
    //   |--red   ->   |-----
    //   |    |       red   |
    //  nil   x        h   nil

    unsafe {
      let x = (*h).right;

      (*h).right = (*x).left;

      (*x).count = (*h).count;
      (*h).count = 1 + BST::size_tree((*h).left) + BST::size_tree((*h).right);

      (*x).color = (*h).color;
      (*h).color = Color::Red;

      (*x).left = h;
      x
    }
  }

  #[inline(always)]
  pub fn rotate_right(h: Link<K, V>) -> Link<K, V> {
    // H is above X and is > X. X is on H's left.
    // They will switch places and X
    // being < H, will be above H and H will be on X's right.
    //
    //   h(s)             x(e)
    //   |------     ->   |------
    //  red    |          |    red
    //  x(e)  nil        nil   h(s)

    unsafe {
      let x = (*h).left;

      (*h).left = (*x).right;
      (*x).count = (*h).count;
      (*h).count = 1 + BST::size_tree((*h).left) + BST::size_tree((*h).right);

      (*x).color = (*h).color;
      (*h).color = Color::Red;

      (*x).right = h;
      x
    }
  }

  pub fn delete(h: Link<K, V>) -> Link<K, V> {
    //   delete x
    //
    //   h
    //   |-----
    //  red   |
    //   x   nil
    //
    //
    //
    //
    //   drop(Box::from_raw(h))
    //
    //
    //
    //
    unimplemented!()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn tree_rotates() {
    let mut bst: BST<&str, usize> = BST::new();

    bst.put("A", 1);
    bst.put("B", 2);
    bst.put("C", 3);
    bst.put("D", 4);
    bst.put("E", 5);
    bst.put("F", 6);
    bst.put("G", 7);
    bst.put("H", 8);
    bst.put("I", 9);
    bst.put("J", 10);
    bst.put("K", 11);
    bst.put("L", 12);

    // bst.put("S", 1);
    // bst.put("E", 2);
    // bst.put("A", 3);
    // bst.put("R", 4);
    // bst.put("C", 5);
    // bst.put("H", 6);
    // bst.put("X", 7);
    // bst.put("M", 8);
    // bst.put("P", 9);
    // bst.put("L", 10);

    println!("tree:");
    println!("{}", &bst);

    assert_eq!(bst.size(), 12usize);
  }

  #[test]
  fn tree_has_a_count() {
    let mut bst: BST<&str, usize> = BST::new();

    bst.put("T", 2);
    bst.put("X", 4);
    bst.put("A", 45);
    bst.put("D", 70);
    bst.put("R", 70);
    bst.put("H", 10000);

    assert_eq!(bst.size(), 6usize);
  }

  #[test]
  fn can_get_elements() {
    let mut bst: BST<&str, usize> = BST::new();

    bst.put("Root", 2);
    bst.put("L.", 4);
    bst.put("W.", 45);
    bst.put("G.", 70);
    bst.put("J.", 70);
    bst.put("H.", 10000);

    let value = bst.get(&"H.");

    assert_eq!(value, Some(&10000));
  }

  #[test]
  fn has_perfect_black_balance() {
    let mut bst: BST<&str, usize> = BST::new();

    // bst.put("A", 1);
    // bst.put("B", 2);
    // bst.put("C", 3);
    // bst.put("D", 4);
    // bst.put("E", 5);
    // bst.put("F", 6);
    // bst.put("G", 7);
    // bst.put("H", 8);
    // bst.put("I", 9);
    // bst.put("J", 10);
    // bst.put("K", 11);
    // bst.put("L", 12);

    bst.put("S", 1);
    bst.put("E", 2);
    bst.put("A", 3);
    bst.put("R", 4);
    bst.put("C", 5);
    bst.put("H", 6);
    bst.put("X", 7);
    bst.put("M", 8);
    bst.put("P", 9);
    bst.put("L", 10);

    let (min, max) = bst.get_min_max_black_depth();

    println!("tree:");
    println!("{}", &bst);
    println!("Black depth min: {}, max: {}", min, max);

    assert_eq!(min, max);
  }
}
