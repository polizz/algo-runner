use super::*;

#[derive(Debug)]
pub struct Percolator {
  pub sites_open: Vec<bool>, // true=open or false=blocked
  pub union_find: UF<usize>,
}

impl Percolator {
  pub fn new(n: usize) -> Self {
    let num_sites = usize::pow(n, 2) + 2;
    let mut sites_open = vec![false; num_sites]; // +2 sites for virtual top and bottom site
    let mut union_find = UF::new(num_sites);

    // set_random_open_sites(&mut union_find);
    connect_virtual_sites(&mut union_find, n);

    sites_open[0] = true;
    sites_open[num_sites - 1] = true;
    
    Percolator {
      sites_open,
      union_find,
    }
  }

  // pub fn open(site: usize) {}
  // pub fn isOpen(site: usize) {}
  // pub fn isFull(site: usize) {}
  // pub fn numberOfOpenSites() -> u32 {}
  // pub fn percolates() -> bool {}
}

// fn set_random_open_sites(union_find: &mut UF<size>) {}

fn connect_virtual_sites(union_find: &mut UF<usize>, n: usize) {
  let top_virtual = 0;

  (1..=n)
    .for_each(|site| union_find.union(site, top_virtual));

  let total_sites = usize::pow(n, 2);
  println!("total sites: {}", &total_sites);

  let bottom_row_start = dbg!(total_sites - n + 1);
  let bottom_row_end = dbg!(bottom_row_start + 2);
  let bottom_virtual = dbg!(total_sites + 1);

  (bottom_row_start..=bottom_row_end)
    .for_each(|site| union_find.union(dbg!(site), bottom_virtual));
  
  // println!("{:?}", &union_find);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_init() {
    let expected: Vec<usize> = vec![0, 0, 0, 0, 4, 5, 6, 10, 10, 10, 10];
    let expected_open = vec![true, false, false, false, false, false, false, false, false, false, true];
    let perc = Percolator::new(3);

    let sites = &perc.union_find.sites;

    assert_eq!(&expected, sites);
    assert_eq!(&expected_open, &perc.sites_open);

    // println!("{:#?}", &perc);
  }
}
