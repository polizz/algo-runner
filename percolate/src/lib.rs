#[derive(Debug)]
pub struct Percolator<T> {
    pub sites: Vec<T>,
}

impl Percolator<usize> {
    pub fn new(n: usize) -> Percolator<usize> {
        let mut sites_vec = vec![0; n];

        sites_vec.iter_mut()
            .enumerate()
            .for_each(|(ix, site)| {
                *site = ix;
            });

        Percolator {
            sites: sites_vec,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let result = vec![0, 1, 2, 3, 4];
        let percolator = Percolator::new(5);

        assert_eq!(result, percolator.sites);
    }
}
