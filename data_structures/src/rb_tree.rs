use std::cell::{Ref, RefCell};
use std::fmt::{Debug, Display};
use std::rc::Rc;

type BareLink<K, V> = Rc<RefCell<Node<K, V>>>;
type Link<K, V> = Option<BareLink<K, V>>;

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
      right: None,
      left: None,
      color: Color::Red,
      count: 1,
    }
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
    BST { root: None }
  }

  pub fn get(&self, key: &'static K) -> Option<Ref<V>> {
    BST::get_node(&self.root, key)
  }

  // {
  fn get_node<'tree>(node: &'tree Link<K, V>, key: &'static K) -> Option<Ref<'tree, V>> {
    // match node {
    //   Some(ref n) => {
    //     if key > &n.borrow().key {
    //       BST::get_node(n.borrow().right.clone(), key)
    //       // BST::get_node(&n.borrow().right, key)
    //     } else if key < &n.borrow().key {
    //       // BST::get_node(&n.borrow().left, key)
    //       BST::get_node(n.borrow().left.clone(), key)
    //     } else {
    //       node
    //         .as_ref()
    //         .map(|node2| Ref::map(node2.borrow(), |n| &n.value))
    //
    //       // Some(Ref::map(node.as_ref().unwrap().borrow(), |n| &n.value))
    //       //
    //       // Some(Ref::map(n_test, |n: &Node<K, V>| &n.value))
    //       // node.as_ref().map(|n| Ref::map(n, |n| &n.value))
    //     }
    //   }
    //   _ => None,
    // }

    node.as_ref().map(|n| {
      if key > &n.borrow().key {
        BST::get_node(&n.borrow().right, key)
      } else if key < &n.borrow().key {
        BST::get_node(&n.borrow().left, key)
      } else {
        Some(Ref::map((**n).borrow(), |n: &Node<K, V>| &n.value))
      }
    })?
  }

  pub fn size(&self) -> usize {
    BST::size_tree(&self.root)
  }

  fn size_tree(node: &Link<K, V>) -> usize {
    // let i = node.unwrap().as_ref().borrow();

    match node {
      None => 0,
      Some(n) => {
        let n = n.as_ref().borrow();
        n.count
      }
    }
  }

  pub fn put(&mut self, key: K, value: V) {
    let root = self.root.take();

    let mut node = self.put_r(root, key, value);
    // let x: Link<K, V> = self.root.unwrap().borrow().borrow_mut();
    // let n1 = node.as_ref().unwrap();
    // let mut n1 = n1.borrow_mut();
    // n1.color = Color::Black;
    node.as_ref().unwrap().borrow_mut().color = Color::Black;

    self.root = node.take();
  }

  fn put_r(&self, h: Link<K, V>, key: K, value: V) -> Link<K, V> {
    match h {
      None => Some(Rc::new(RefCell::new(Node {
        key,
        value,
        right: None,
        left: None,
        color: Color::Red,
        count: 1,
      }))),
      Some(h_next) => {
        let mut h_node = h_next.borrow_mut();

        if key > h_node.key {
          h_node.right = self.put_r(h_node.right.take(), key, value);
        } else if key < h_node.key {
          h_node.left = self.put_r(h_node.left.take(), key, value);
        } else {
          h_node.value = value;
        }

        h_node.count = 1 + BST::size_tree(&h_node.left) + BST::size_tree(&h_node.right);

        BST::balance(h_next)
      }
    }
  }

  pub fn is_red(node: &Link<K, V>) -> bool {
    match node {
      None => false,
      Some(n) => {
        let node_b = n.as_ref().borrow();
        // let node_b = node_b.borrow();

        match &node_b.color {
          Color::Red => true,
          Color::Black => false,
        }
      }
    }
  }

  #[inline(always)]
  pub fn flip_colors(h: BareLink<K, V>) {
    h.borrow_mut().color = Color::Red;
    h.borrow_mut().left.as_ref().unwrap().borrow_mut().color = Color::Black;
    h.borrow_mut().right.as_ref().unwrap().borrow_mut().color = Color::Black;
  }

  fn balance(mut h: BareLink<K, V>) -> Link<K, V> {
    // let h_ref = h.as_ref();
    // let h_ref = h_ref.borrow_mut();

    // #[allow(unused_assignments)]
    // let rotated_node = h_ref;

    // if red node on right while no red node on left is not allowed, rotate left to fix
    if (h.borrow().right.is_some() && BST::is_red(&h.borrow().right))
      && (h.borrow().left.is_none() || h.borrow().left.is_some() && !BST::is_red(&h.borrow().left))
    {
      // println!("rotating left");
      // drop(h_ref);
      // let h_c1 = Rc::clone(&h);
      h = BST::rotate_left(h.clone()).unwrap();
    }

    // let h_left = (*(rotated_node.unwrap()))
    //   .borrow()
    //   .left
    //   .map(|l| (*l).borrow());

    // if both left child and left grandchild are red, rotate right
    if (h.borrow().left.is_some() && BST::is_red(&h.borrow().left))
      && (h.borrow().left.is_some() && BST::is_red(&h.borrow().left))
    {
      // println!("rotating right");
      h = BST::rotate_right(h.clone()).unwrap();
    }

    // let rotated_node_ref = *rotated_node.unwrap();
    // let mut rotated_node_b = rotated_node.borrow();

    // let (h_left, h_right) = (
    //   rotated_node.borrow().left.map(|l| (*l).borrow()),
    //   rotated_node.borrow().right.map(|l| (*l).borrow()),
    // );

    // let h_right = rotated_node.borrow().right.map(|l| (*l).borrow());

    // if both children are red, flip colors
    if (h.borrow().left.is_some() && BST::is_red(&h.borrow().left))
      && (h.borrow().right.is_some() && BST::is_red(&h.borrow().right))
    {
      // println!("flipping colors");
      BST::flip_colors(h.clone());
    }

    Some(h)

    // let mut h = *h.borrow_mut();
    //
    // #[allow(unused_assignments)]
    // let mut node_rotate = Node::default();
    //
    // // if red node on right while no red node on left is not allowed, rotate left to fix
    //
    // if (h.right.is_some() && BST::is_red(&h.right))
    //   && (h.left.is_none() || h.left.is_some() && !BST::is_red(&h.left))
    // {
    //   // println!("rotating left");
    //   BST::rotate_left(h);
    //   // node_rotate = BST::rotate_left(h);
    // } else {
    //   node_rotate = h;
    // }
    //
    // // if left child and it's left child are red, rotate right
    //
    // // let mut rot_node = node_rotate.unwrap();
    // if (node_rotate.left.is_some() && BST::is_red(&node_rotate.left))
    //   && (node_rotate.left.as_ref().unwrap().left.is_some()
    //     && BST::is_red(&node_rotate.left.as_ref().unwrap().left))
    // {
    //   // println!("rotating right");
    //   let new_node = BST::rotate_right(node_rotate);
    //   node_rotate = new_node;
    // }
    //
    // // if both children are red, flip colorsk
    // if (node_rotate.left.is_some() && BST::is_red(&node_rotate.left))
    //   && (node_rotate.right.is_some() && BST::is_red(&node_rotate.right))
    // {
    //   // println!("flipping colors");
    //   BST::flip_colors(&mut node_rotate);
    // }
    //
    // Some(Rc::new(node_rotate))
  }

  #[inline(always)]
  pub fn rotate_left(h: BareLink<K, V>) -> Link<K, V> {
    // H is above X and is < X. X is on H's right.
    // They will switch places and X
    // being > H, will be above H and H will be on X's left.
    //
    //   h             x
    //   |--red   ->   |-----
    //   |    |       red   |
    //  nil   x        h   nil

    let mut h_b = h.borrow_mut();

    // let x = mem::replace(&mut h_b.right, None);
    let x = h_b.right.take();
    // let mut x_u = x_moved.clone().unwrap();

    let mut x_moved = x.unwrap().borrow_mut();
    h_b.right = x_moved.left.take();

    x_moved.count = h_b.count;
    h_b.count = 1 + BST::size_tree(&h_b.left) + BST::size_tree(&h_b.right);

    x_moved.color = h_b.color;
    h_b.color = Color::Red;

    x_moved.left = Some(Rc::clone(&h));
    x
    // mem::swap(&mut x_u.left, &mut Some(Rc::new(h)));
    // Rc::clone(&x.unwrap())

    // // H is above X and is < X. X is on H's right.
    // // They will switch places and X
    // // being > H, will be above H and H will be on X's left.
    // let mut h = *h.borrow_mut();
    //
    // let x_moved = mem::replace(&mut h.right, None);
    // let mut x_u = x_moved.clone().unwrap();
    //
    // h.right = mem::replace(&mut x_u.left, None);
    // x_u.count = h.count;
    // h.count = 1 + BST::size_tree(&h.left) + BST::size_tree(&h.right);
    //
    // x_u.color = h.color;
    // h.color = Color::Red;
    //
    // mem::swap(&mut x_u.left, &mut Some(Rc::new(h)));
    //
    // x_u
  }

  #[inline(always)]
  pub fn rotate_right(h: BareLink<K, V>) -> Link<K, V> {
    let mut h_b = h.borrow_mut();

    // let x = mem::replace(&mut h_b.left, None);
    let x = h_b.left.take();
    let xt = x.unwrap();
    let x_u = Rc::clone(&xt);
    // let mut x_u = x_moved.clone().unwrap();

    let mut x_moved = (*x_u).borrow_mut();
    h_b.left = x_moved.right.take();
    x_moved.count = h_b.count;
    h_b.count = 1 + BST::size_tree(&h_b.left) + BST::size_tree(&h_b.right);

    x_moved.color = h_b.color;
    h_b.color = Color::Red;

    x_moved.right = Some(Rc::clone(&h));
    x
    // mem::swap(&mut x_u.right, &mut Some(Rc::new(h)));
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn tree_rotates() {
    let mut bst: BST<&str, usize> = BST::new();

    // bst.put("c", 1);
    // bst.put("a", 1);
    // bst.put("b", 1);

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

    println!("tree: {:#?}", &bst);
    assert_eq!(bst.size(), 10usize);
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

  // #[test]
  // fn can_get_elements() {
  //   let mut bst: BST<&str, usize> = BST::new();
  //
  //   bst.put("Root", 2);
  //   bst.put("L.", 4);
  //   bst.put("W.", 45);
  //   bst.put("G.", 70);
  //   bst.put("J.", 70);
  //   bst.put("H.", 10000);
  //
  //   let value = bst.get(&"H.");
  //
  //   assert_eq!(value.borrow(), Some(&10000));
  // }
}
