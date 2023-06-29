
pub mod linked_list {

  #[derive(Debug)]
  pub struct Node<T> {
    data: T,
    id: usize,
    next: Option<*mut Node<T>>
  }

  impl<T> Node<T>
  where
    T: Copy
  {
    pub fn new(data: T, id: usize, next: Option<*mut Node<T>>) -> Self {
      Node {
        id,
        data,
        next
      }
    }
  }

  #[derive(Debug)]
  pub struct List<T> {
    len: usize,
    head: Option<*mut Node<T>>
  }

  trait ListTrait<T> {
    fn new() -> Self;
    fn add(&mut self, val: T);
    fn iter(&self) -> ListIter<T>;
  }

  impl<T> ListTrait<T> for List<T>
  where
    T: Copy
  {
    fn new() -> Self {
      List {
        len: 0,
        head: None,
      }
    }

    fn add(&mut self, val: T) {
      let new_node = Box::new(Node::new(val, 1, self.head));

      self.head = Some(Box::into_raw(new_node) as *const Node<T> as *mut Node<T>);
      self.len += 1;
    }

    fn iter(&self) -> ListIter<T> {
      ListIter {
        current_iter_item: None,
        head: self.head,
        end: false
      }
    }
  }

  #[derive(Debug)]
  pub struct ListIter<T> {
    current_iter_item: Option<*mut Node<T>>,
    head: Option<*mut Node<T>>,
    end: bool
  }
  
  impl<T> Iterator for ListIter<T>
  where
    T: Copy
  {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
      if self.end == true {
        return None
      }

      match self.current_iter_item {
        None => {
          if let Some(head) = self.head {
            unsafe {
              self.current_iter_item = Some(head);
              Some((*head).data)
            }
          } else {
            self.end = true;

            None
          }
        },
        Some(last_node) => {
          unsafe {
            let next_node = (*last_node).next;
            self.current_iter_item = next_node;

            if next_node.is_none() {
              self.end = true;
              return None
            }

            Some((*next_node.unwrap()).data)
          }
        }
      }

    }
  }

  #[cfg(test)]
  mod test {
    use super::*;

    #[test]
    fn init() {
      let mut l = List::new();
      
      l.add(32);
      l.add(24);
      l.add(2);
      l.add(5);
      l.add(11);


      println!("List len: {}", &l.len);
    }

    #[test]
    fn iter_test() {
      let mut l = List::new();
      
      l.add(2);
      l.add(245);
      l.add(32);
      l.add(110);
      l.add(5);

      let mut col = l.iter();

      println!("List len: {}", &l.len);
      println!("item {:#?}", col.next());
      println!("item {:#?}", col.next());
      println!("item {:#?}", col.next());
      println!("item {:#?}", col.next());
      println!("item {:#?}", col.next());
      println!("item {:#?}", col.next());

      let col2: Vec<usize> = l.iter().filter(|item| *item < 50).collect();

      println!("Iter collected: {:#?}", &col2);
    }
  }

}