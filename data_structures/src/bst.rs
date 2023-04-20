// use std::mem;
use std::fmt::{Debug, Display};
use std::borrow::{BorrowMut};

type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
enum Color {
  Red,
  Black
}

#[derive(Debug)]
pub struct Node<K, V>
where
  K: Debug + Display,
  V: Debug + Display
{
  key: K,
  value: V,
  right: Link<K, V>,
  left: Link<K, V>,
  color: Color,
  count: usize
}

#[derive(Debug)]
pub struct BST<K, V>
where
  K: Debug + Display,
  V: Debug + Display
{
  root: Link<K, V>
}

impl<K, V> BST<K, V>
where
  K: Debug + Display + Eq + Ord,
  V: Debug + Display
{
  pub fn new() -> Self {
    BST {
      root: None
    }
  }

  pub fn put(&mut self, key: K, value: V) {
    let mut node = BST::put_recurse(self.root.take(), key, value);
    self.root = node.take();
  }

  fn put_recurse(mut h: Link<K, V>, key: K, value: V) -> Link<K, V> {
    match h {
      None => Some(Box::new(Node {
        key,
        value,
        right: None,
        left: None,
        color: Color::Red,
        count: 1
      })),
      Some(ref mut h_next) => {
        let h_node: &mut Node<K, V> = h_next.borrow_mut();
        
        if key > h_node.key {
          h_node.right = BST::put_recurse(h_node.right.take(), key, value);
        } else if key < h_node.key {
          h_node.left = BST::put_recurse(h_node.left.take(), key, value);
        } else {
          h_node.value = value;
        }
        
        h
      }
    }
  }

  // isRed(Node) -> bool
  // rotateLeft(Node) -> Node
  // rotateRight(Node) -> Node
  // flipColors(Node) -> ()
  // size() -> usize
  // pub fn put(Key, Value) -> ()
  // fn put(Node, Key, Value)
  // get(Key) -> Node
  // min() -> Node
  // max() -> Node
  // deleteMin()
  // deleteMax()
  // keys
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn smoke() {
    let mut bst: BST<&str, usize> = BST::new();
    // let bst = BST::new(31usize);
    bst.put("Test", 2);
    bst.put("Xayne", 4);
    bst.put("Andrew", 45);
    bst.put("David", 70);
    bst.put("Rita", 70);
    bst.put("Harley", 10000);

    println!("bst: {:#?}", &bst);
  }
}