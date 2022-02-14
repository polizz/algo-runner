#[derive(Debug)]
pub struct UF<T> {
    pub sites: Vec<T>,
    pub tree_depths: Vec<u32>,
}

impl UF<usize> {
    pub fn new(n: usize) -> Self {
        let mut sites = vec![0; n];
        let tree_depths = vec![1; n];

        sites.iter_mut()
            .enumerate()
            .for_each(|(ix, site)| {
                *site = ix;
            });

        UF {
            sites,
            tree_depths,
        }
    }

    pub fn root_of(&mut self, mut i: usize) -> usize {
        while self.sites[i] != i {
            self.sites[i] = self.sites[self.sites[i]];
            i = self.sites[i];
        }

        i
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root_of(p) ==  self.root_of(q)
    } 

    pub fn union(&mut self, p: usize, q: usize) {
        let p_id = self.sites[p];
        let q_id = self.sites[q];

        if p_id == q_id {
            ()
        }

        let p_root = self.root_of(p);
        let q_root = self.root_of(q);

        // attach the smaller tree to the root of the bigger tree
        if self.tree_depths[p_root] > self.tree_depths[q_root] {
            self.sites[q_root] = p_root;
            self.tree_depths[q_root] = self.tree_depths[q_root] + self.tree_depths[p_root];
        } else {
            self.sites[p_root] = q_root;
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

        assert_eq!([3, 3], &uf.sites[2..=3]);
        assert_eq!(2, uf.tree_depths[2]);

        uf.union(4, 3);

        assert_eq!([3, 3], &uf.sites[3..=4]);
        assert_eq!(2, uf.tree_depths[4]);
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

        assert_eq!(4, uf.root_of(2));
    }
}