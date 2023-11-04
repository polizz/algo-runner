use std::collections::{HashSet, VecDeque};

type Location = (usize, usize);

pub struct IslandSolution;

impl IslandSolution {
  pub fn next_locations(
    l: Location,
    m: usize,
    n: usize,
    seen: &HashSet<Location>,
  ) -> Vec<Location> {
    let (curr_m, curr_n) = l;
    vec![(0, 1), (1, 0), (-1, 0), (0, -1)]
      .iter()
      .filter_map(|(next_m, next_n)| {
        let (new_m, new_n) = (curr_m as i32 + next_m, curr_n as i32 + next_n);

        if new_m >= 0
          && new_m < m as i32
          && new_n >= 0
          && new_n < n as i32
          && !seen.contains(&(new_m as usize, new_n as usize))
        {
          Some((new_m as usize, new_n as usize))
        } else {
          None
        }
      })
      .collect::<Vec<(usize, usize)>>()
  }

  pub fn num_islands_iter(grid: Vec<Vec<char>>) -> i32 {
    let mut queue: VecDeque<Location> = VecDeque::new();
    let mut seen: HashSet<Location> = HashSet::new();
    let M = grid.len();
    let N = grid[0].len();
    let mut island_count = 0;

    for (m, row) in grid.iter().enumerate() {
      for (n, _) in row.iter().enumerate() {
        if grid[m][n] == '1' && !seen.contains(&(m, n)) {
          island_count = island_count + 1;
          queue.push_back((m, n));

          loop {
            if let Some((q_m, q_n)) = queue.pop_front() {
              if grid[q_m][q_n] == '1' {
                let next_locations = IslandSolution::next_locations((q_m, q_n), M, N, &seen);

                next_locations.into_iter().for_each(|loc| {
                  seen.insert(loc);
                  queue.push_back(loc);
                });
              }
            } else {
              break;
            }
          }
        }
      }
    }

    return island_count;
  }

  pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut queue: VecDeque<Location> = VecDeque::new();
    let mut seen: HashSet<Location> = HashSet::new();
    let M = grid.len();
    let N = grid[0].len();
    let mut m = 0_usize;
    let mut n = 0_usize;
    let mut island_count = 0;

    while m < M {
      while n < N {
        if grid[m][n] == '1' && !seen.contains(&(m, n)) {
          island_count = island_count + 1;
          queue.push_back((m, n));

          loop {
            if let Some((q_m, q_n)) = queue.pop_front() {
              if grid[q_m][q_n] == '1' {
                let next_locations = IslandSolution::next_locations((q_m, q_n), M, N, &seen);

                next_locations.into_iter().for_each(|loc| {
                  seen.insert(loc);
                  queue.push_back(loc);
                });
              }
            } else {
              break;
            }
          }
        }

        n = n + 1;
      }

      n = 0;
      m = m + 1;
    }

    return island_count;
  }
}

#[cfg(test)]
mod island_tests {
  use crate::IslandSolution;

  #[test]
  fn get_island_count3() {
    let grid = vec![
      vec!['1', '1', '1', '1', '1', '1'],
      vec!['1', '0', '0', '0', '0', '1'],
      vec!['1', '0', '1', '1', '0', '1'],
      vec!['1', '0', '0', '0', '0', '1'],
      vec!['1', '1', '1', '1', '1', '1'],
    ];

    let count = IslandSolution::num_islands(grid.clone());
    let count2 = IslandSolution::num_islands_iter(grid);
    assert_eq!(2, count);
    assert_eq!(2, count2);
  }

  #[test]
  fn get_island_count1() {
    let grid = vec![
      vec!['1', '1', '1', '1', '0'],
      vec!['1', '1', '0', '1', '0'],
      vec!['1', '1', '0', '0', '0'],
      vec!['0', '0', '0', '0', '0'],
    ];

    let count = IslandSolution::num_islands(grid.clone());
    let count2 = IslandSolution::num_islands_iter(grid);
    assert_eq!(1, count);
    assert_eq!(1, count2);
  }

  #[test]
  fn get_island_count2() {
    let grid = vec![
      vec!['1', '1', '0', '0', '0'],
      vec!['1', '1', '0', '0', '0'],
      vec!['0', '0', '1', '0', '0'],
      vec!['0', '0', '0', '1', '1'],
    ];

    let count = IslandSolution::num_islands(grid.clone());
    let count2 = IslandSolution::num_islands_iter(grid);
    assert_eq!(3, count);
    assert_eq!(3, count2);
  }
}
