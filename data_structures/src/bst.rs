// use std::mem;
use std::fmt::{Debug, Display};
use std::borrow::{BorrowMut, Borrow};
use std::mem;

type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
  Red,
  Black
}

#[derive(Debug, Clone)]
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

impl<K, V> Default for Node<K, V>
where
  K: Default + Debug + Display,
  V: Default + Debug + Display
{
  fn default() -> Self {
      Node {
        key: K::default(),
        value: V::default(),
        right: None,
        left: None,
        color: Color::Red,
        count: 1
      }
  }
}

#[derive(Debug)]
pub struct BST<K, V>
where
  K: Default + Debug + Display,
  V: Default + Debug + Display
{
  root: Link<K, V>
}

impl<'tree, K, V> BST<K, V>
where
  K: Default + Debug + Display + Eq + Ord,
  V: Default + Debug + Display
{
  pub fn new() -> Self {
    BST {
      root: None
    }
  }

  pub fn get(&self, key: &'static K) -> Option<&V> {
    BST::get_node(&self.root, key)
  }

  pub fn get_node(node: &'tree Link<K, V>, key: &'static K) -> Option<&'tree V> {
    match node {
      None => None,
      Some(n) => {
        let n_test: &Node<K, V> = n.borrow();

        if key > &n_test.key {
          BST::get_node(&n_test.right, key)
        } else if key < &n_test.key {
          BST::get_node(&n_test.left, key)
        } else {
          Some(&n_test.value)
        }
      }
    }
  }

  pub fn size(&self) -> usize {
    BST::size_tree(&self.root)
  }

  fn size_tree(node: &Link<K, V>) -> usize {
    match node {
      None => 0,
      Some(n) => n.count
    }
  }
  
  pub fn put(&mut self, key: K, value: V) {
    let root = self.root.take();
    let mut node = self.put_descend(root, key, value);
    self.root = node.take();
  }

  fn put_descend(&self, mut h: Link<K, V>, key: K, value: V) -> Link<K, V> {
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
          h_node.right = self.put_descend(h_node.right.take(), key, value);
        } else if key < h_node.key {
          h_node.left = self.put_descend(h_node.left.take(), key, value);
        } else {
          h_node.value = value;
        }

        h_node.count = 1 + BST::size_tree(&h_node.left) + BST::size_tree(&h_node.right);

        drop(h_node);

        // h = BST::balance(mem::take(h_next));
        
        println!("after Balance: {:?}", &h);

        h
      }
    }
  }

  pub fn is_red(node: &Link<K, V>) -> bool {
    match node {
      None => false,
      Some(n) => {
        let node_b = n;

        match &node_b.color {
          Color::Red => true,
          Color::Black => false
        }
      }
    }
  }

  pub fn flip_colors(h: &mut Node<K, V>) {
    // let mut h = h.take().unwrap();
    // let mut h: &mut Node<K, V> = h.borrow_mut();

    h.color = Color::Red;

    h.right.as_mut().unwrap().color = Color::Black;
    // let h_right = h.right.as_mut().unwrap();
    // let mut h_right: &mut Node<K, V> = h_right.borrow_mut();
    // h_right.color = Color::Black;

    h.left.as_mut().unwrap().color = Color::Black;
  }

  /*
  fn balance(h: Node<K, V>) -> Link<K, V> {
    // let h = node;
    // let mut h: &mut Node<K, V> = node.borrow_mut();

    println!("Balancing: {:?}", &h);

    let mut node_rotate = Link::default();

    if (h.right.is_some() && BST::is_red(&h.right)) && (h.left.is_none() || h.left.is_some() && !BST::is_red(&h.left)) {
      println!("rotating left");
      node_rotate = BST::rotate_left(h);
    }
    let h_r = *(&node_rotate.as_ref().unwrap());
    if ((&h_r.left).is_some() && BST::is_red(&h_r.left)) && (h_r.left.is_some() && (&h_r.left).as_ref().unwrap().left.is_some() && !BST::is_red(&h_r.left.unwrap().left)) {
      println!("rotating right");
      BST::rotate_right(**h_r);
    }
    // if (h.left.is_some() && BST::is_red(&h.left)) && (h.right.is_some() && !BST::is_red(&h.right)) {
    //   println!("flipping colors");
    //   BST::flip_colors(h);
    // }

    // h
    // node_rotate
    Link::default()
  }
  */
  pub fn rotate_left(mut h: Node<K, V>) -> Link<K, V> {
    // H is above X and is < X. X is on H's right.
    // They will switch places and X.
    // being < H, will be above H and X will be on H's left.

    let x_moved = mem::replace(&mut h.right, None);
    // let mut h_moved = mem::replace(h, Node::default());

    let mut x_u = *(x_moved.unwrap());
    println!("BEFORE h: {:?}, x: {:?}", &h, &x_u);
    
    h.right = mem::replace(&mut x_u.left, None);
    x_u.count = h.count;
    h.count = 1 + BST::size_tree(&h.left) + BST::size_tree(&h.right);
    
    x_u.color = h.color;
    h.color = Color::Red;

    mem::swap(&mut x_u.left, &mut Some(Box::new(h)));
    
    Some(Box::new(x_u))
  }

  pub fn rotate_right(mut h: Node<K, V>) -> Link<K, V> {
    let x_moved = mem::replace(&mut h.right, None);
    // let mut h_moved = mem::replace(h, Node::default());

    let mut x_u = *(x_moved.unwrap());
    println!("BEFORE h: {:?}, x: {:?}", &h, &x_u);
    
    h.left = mem::replace(&mut x_u.right, None);
    x_u.count = h.count;
    h.count = 1 + BST::size_tree(&h.left) + BST::size_tree(&h.right);
    
    x_u.color = h.color;
    h.color = Color::Red;

    mem::swap(&mut x_u.right, &mut Some(Box::new(h)));
    
    Some(Box::new(x_u))

    // let x_moved = mem::replace(&mut h.left, None);
    // let mut h_moved = mem::replace(h, Node::default());

    // let mut x_u = *(x_moved.unwrap());

    // h_moved.left = mem::replace(&mut x_u.right, None);
    // x_u.count = h_moved.count;
    // h_moved.count = 1 + BST::size_tree(&h_moved.left) + BST::size_tree(&h_moved.right);

    // x_u.color = h_moved.color;
    // h_moved.color = Color::Red;

    // // mem::replace(&mut x_u.right, Some(h_moved));

    // *h = h_moved;
    // x_u
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
  fn tree_rotates() {
    let mut bst: BST<&str, usize> = BST::new();

    bst.put("Test", 2);
    bst.put("Xayne", 4);
    println!("bst: {:#?}", &bst);
    bst.put("Andrew", 45);
    // bst.put("David", 70);
    // bst.put("Rita", 70);
    // bst.put("Harley", 10000);

    println!("bst: {:#?}", &bst);

    assert_eq!(bst.size(), 6usize);
  }

  #[test]
  fn tree_has_a_count() {
    let mut bst: BST<&str, usize> = BST::new();

    bst.put("Test", 2);
    bst.put("Xayne", 4);
    bst.put("Andrew", 45);
    bst.put("David", 70);
    bst.put("Rita", 70);
    bst.put("Harley", 10000);

    // println!("bst: {:#?}", &bst);

    assert_eq!(bst.size(), 6usize);
  }

  #[test]
  fn can_get_elements() {
    let mut bst: BST<&str, usize> = BST::new();

    bst.put("Root", 2);
    bst.put("L. Ashley", 4);
    bst.put("W. Andrew", 45);
    bst.put("G. David", 70);
    bst.put("J. Rita", 70);
    bst.put("Harley", 10000);

    let andrew_value = bst.get(&"Harley");

    println!("bst: {:#?}", &bst);

    assert_eq!(andrew_value, Some(&10000));
  }
}