// miri testing: cargo +nightly-2024-06-11 miri test tree_rotates

use std::fmt::{Debug, Display};
use std::ptr;

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

// function printBST<K, V>(tree: Tree<K, V>) {
//   function printTree(indent: string, tree?: Tree<K, V>) {
//     if (tree === undefined) {
//       return indent + ' └── ' + 'Empty\n'
//     } else {
//       let treeString = indent + ' ├── ' + tree!.key + ` (${tree.color})\n`
//       indent += ' │  '
//       treeString += printTree(indent, tree!.left)
//       treeString += printTree(indent, tree!.right)
//
//       return treeString
//     }
//   }
//
//   return printTree("", tree)
// }

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
          // println!("dropping node: {}", (*node).key);
          drop(Box::from_raw(node));
        }
      }
    }
    // println!("starting BST drop");
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
    // let i = node.unwrap().as_ref().borrow();
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
