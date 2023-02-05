use super::*;

#[derive(Debug)]
pub struct Percolator {
  pub open_sites: Vec<bool>, // true=open or false=blocked
  pub union_find: UF<usize>,
  pub num_sites: usize,
  pub n: usize,
}

impl Percolator {
  pub fn new(n: usize) -> Self {
    let num_sites = usize::pow(n, 2) ;
    let mut open_sites = vec![false; num_sites + 2]; // +2 sites for virtual top and bottom site
    let union_find = UF::new(num_sites + 2);

    // set_random_open_sites(&mut union_find);

    open_sites[0] = true;
    open_sites[num_sites + 1] = true;
    
    Percolator {
      open_sites,
      union_find,
      n,
      num_sites,
    }
  }

  pub fn open(&mut self, site: usize) {
    match self.is_open(site) {
      false => {
        self.open_sites[site] = true;
        self.connect_open_neighbors(site);
      }
      true => (),
    }
  }

  pub fn is_open(&self, site: usize) -> bool {
    self.open_sites[site] 
  }

  pub fn is_full(&mut self, site: usize) -> bool {
    self.union_find.connected(0, site)
  }

  pub fn number_of_open_sites(&self) -> usize {
    self.open_sites.iter()
      .filter(|&&site_open| true == site_open)
      .count() - 2
  }

  pub fn percolates(&mut self) -> bool {
    let bottom_virtual_site = self.open_sites.len() - 1;
    self.union_find.connected(0, bottom_virtual_site)
  }

  fn connect_open_neighbors(&mut self, site: usize) {
    let num_sites = self.num_sites;
    let bottom_virt_site = num_sites + 1;
    let n = self.n;

    // top row should get connected to 0-node, no further up calculation

    // site 1 should get connected to 0-node, no behind calculation

    if site <= n {
      self.union_find.union(site, 0);
    } else {
      let up = site - n;

      if self.open_sites[up] {
        self.union_find.union(site, up);
      }
    }

    if site <= 1 {
      self.union_find.union(site, 0);
    } else {
      let behind = site - 1;

      if self.open_sites[behind] {
        self.union_find.union(site, behind);
      }    
    }

    let ahead = site + 1;
    if ahead <= bottom_virt_site {
      if self.open_sites[ahead] {
          self.union_find.union(site, ahead);
      }
    } else {
      self.union_find.union(site, bottom_virt_site);
    }

    if site >= num_sites {

    }
    let down = site + n;
    if down <= bottom_virt_site {
      if self.open_sites[down] {
        self.union_find.union(site, down);
      }
    } else {
      self.union_find.union(site, bottom_virt_site);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn site_is_full_when_connected_to_open_top_row() {
    let mut perc = Percolator::new(4);

    assert_eq!(&false, &perc.is_full(8));
  }

  #[test]
  fn no_percolation_at_init() {
    let mut perc = Percolator::new(5);

    assert_eq!(&false, &perc.percolates());
  }

  #[test]
  fn percolates_when_all_open() {
    let mut perc = Percolator::new(3);
    let perc_ref = &mut perc;

    (1..=9).for_each(|site| {
      perc_ref.open(site);
    });

    assert_eq!(&true, &perc.percolates());
  }

  #[test]
  fn open_site_count_correct(){
    let mut perc = Percolator::new(4);
    let perc_ref = &mut perc;

    (1..10).for_each(|site| perc_ref.open(site));

    assert_eq!(9, perc.number_of_open_sites());
  }

  #[test]
  fn percolates_when_just_enough_open() {
    let mut perc = Percolator::new(4);

    perc.open(1);

    assert_eq!(&true, &perc.is_full(1));
    assert_eq!(&false, &perc.percolates());

    perc.open(4);

    assert_eq!(&true, &perc.is_full(4));
    assert_eq!(&false, &perc.percolates());

    perc.open(10);

    assert_eq!(&false, &perc.is_full(10));
    assert_eq!(&false, &perc.percolates());

    perc.open(13);
    assert_eq!(&false, &perc.is_full(13));
    assert_eq!(&false, &perc.percolates());

    perc.open(16);
    assert_eq!(&false, &perc.is_full(16));
    assert_eq!(&false, &perc.percolates());

    perc.open(6);
    assert_eq!(&false, &perc.is_full(6));
    assert_eq!(&false, &perc.percolates());

    perc.open(7);
    assert_eq!(&false, &perc.is_full(7));
    assert_eq!(&false, &perc.percolates());

    perc.open(5);
    assert_eq!(&true, &perc.is_full(5));
    assert_eq!(&true, &perc.is_full(10));
    assert_eq!(&false, &perc.percolates());

    perc.open(14);
    assert_eq!(&true, &perc.is_full(14));
    assert_eq!(&true, &perc.percolates());
  }

  #[test]
  fn can_count_num_of_open_sites() {
    let mut perc = Percolator::new(3);

    perc.open(4);

    assert_eq!(1, perc.number_of_open_sites());
  }

  #[test]
  fn can_open_site() {
    let mut perc = Percolator::new(3);

    perc.open(4);
    perc.open(5);

    let opens = &perc.open_sites[4..=5];
    assert_eq!(&[true, true], opens);
  }

  #[test]
  fn open_site_is_open() {
    let mut perc = Percolator::new(3);

    perc.open(4);
    perc.open(5);

    assert_eq!(&true, &perc.is_open(4));
    assert_eq!(&true, &perc.is_open(5));
  }

  #[test]
  fn can_init() {
    let expected: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let expected_open = vec![true, false, false, false, false, false, false, false, false, false, true];
    let perc = Percolator::new(3);

    let sites = &perc.union_find.sites;
    println!("{:#?}", &perc);

    assert_eq!(&expected, sites);
    assert_eq!(&expected_open, &perc.open_sites);
  }
}
