#[derive(Debug)]
pub struct UF<T> {
  pub sites: Vec<T>,
  pub tree_depths: Vec<u32>,
}

impl UF<usize> {
  pub fn new(n: usize) -> Self {
    let mut sites = vec![0; n];
    let tree_depths = vec![1; n];

    sites.iter_mut().enumerate().for_each(|(ix, site)| {
      *site = ix;
    });

    UF { sites, tree_depths }
  }

  pub fn root_of(&mut self, mut i: usize) -> usize {
    while self.sites[i] != i {
      self.sites[i] = self.sites[self.sites[i]];
      i = self.sites[i];
    }

    i
  }

  pub fn connected(&mut self, p: usize, q: usize) -> bool {
    self.root_of(p) == self.root_of(q)
  }

  pub fn union(&mut self, p: usize, q: usize) {
    let p_root = self.root_of(p);
    let q_root = self.root_of(q);

    // println!("un call: {p}, {q}, roots: {p_root};{q_root}, depths: {:?};{:?}", self.tree_depths[p_root], self.tree_depths[q_root]);

    if p_root == q_root {
      return;
    }

    // attach the smaller tree to the root of the bigger tree
    if self.tree_depths[p_root] < self.tree_depths[q_root] {
      self.sites[p_root] = q_root;
      self.tree_depths[q_root] = self.tree_depths[q_root] + self.tree_depths[p_root];
    } else {
      self.sites[q_root] = p_root;
      self.tree_depths[p_root] = self.tree_depths[p_root] + self.tree_depths[q_root];
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_init_n_vec() {
    let expected = vec![0, 1, 2, 3, 4];
    let uf = UF::new(5);

    assert_eq!(expected, uf.sites);
  }

  #[test]
  fn can_union() {
    let mut uf = UF::new(5);
    uf.union(2, 3);

    assert_eq!([2, 2], &uf.sites[2..=3]);
    assert_eq!(2, uf.tree_depths[2]);

    uf.union(4, 3);

    assert_eq!([2, 2], &uf.sites[3..=4]);
    assert_eq!(1, uf.tree_depths[4]);
    assert_eq!(3, uf.tree_depths[2]);
  }

  #[test]
  fn can_see_connected_sites() {
    let mut uf = UF::new(5);
    uf.union(2, 3);
    uf.union(4, 3);

    assert_eq!(true, uf.connected(2, 3));
    assert_eq!(true, uf.connected(3, 4));
  }

  #[test]
  fn can_find_roots() {
    let mut uf = UF::new(5);
    uf.union(2, 4);

    println!("AFTER: {:?}", uf);

    assert_eq!(2, uf.root_of(2));
    assert_eq!(2, uf.root_of(4));
  }

  #[test]
  fn multiple_unions_on_same_site() {
    let mut uf = UF::new(9 + 2);
    uf.union(8, 10);
    println!("multiple_unions_on_same_site: {:?}", uf);
    uf.union(2, 0);
    println!("multiple_unions_on_same_site: {:?}", uf);
    uf.union(9, 8);
    println!("multiple_unions_on_same_site: {:?}", uf);
    uf.union(9, 10);
    println!("multiple_unions_on_same_site: {:?}", uf);
    uf.union(9, 10);
    println!("multiple_unions_on_same_site: {:?}", uf);
    uf.union(9, 10);
    println!("multiple_unions_on_same_site: {:?}", uf);

    assert_eq!(3, uf.tree_depths[8]);
  }
}
