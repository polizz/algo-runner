// for a map of product islands in a store (1's) and walking isles in
// between (0's) find any starting coordinate for each island of products

use std::collections::{HashSet, VecDeque};

type Location = (usize, usize);

pub struct TwoD_PathFinding;

impl TwoD_PathFinding {
  #[inline]
  fn get_candidates_locs(curr_loc: Location, m: usize, n: usize) -> impl Iterator<Item = Location> {
    let (curr_row, curr_col) = curr_loc;

    vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
      .into_iter()
      .filter_map(move |(r, c)| {
        let new_row = curr_row as isize + r;
        let new_col = curr_col as isize + c;

        if new_row >= 0 && new_row < m as isize && new_col >= 0 && new_col < n as isize {
          Some((new_row as usize, new_col as usize))
        } else {
          None
        }
      })
  }

  pub fn get_start_coords_bfs(grid: &mut Vec<Vec<usize>>) -> Vec<Location> {
    let m = grid.len();
    let n = grid[0].len();
    let mut solutions: Vec<Location> = vec![];
    let mut seen: HashSet<Location> = HashSet::new();
    let mut queue: VecDeque<Location> = VecDeque::new();

    for (i, row) in grid.iter().enumerate() {
      for (j, _) in row.iter().enumerate() {
        if grid[i][j] == 1 && !seen.contains(&(i, j)) {
          let found_loc = (i, j);

          solutions.push(found_loc);
          seen.insert(found_loc);
          queue.push_back(found_loc);

          while let Some(loc) = queue.pop_front() {
            let candidates =
              TwoD_PathFinding::get_candidates_locs(loc, m, n).filter_map(|(r, c)| {
                if grid[r][c] == 1 && !seen.contains(&(r, c)) {
                  seen.insert((r, c));
                  Some((r, c))
                } else {
                  None
                }
              });

            queue.extend(candidates);
          }
        }
      }
    }

    solutions
  }

  pub fn get_start_coords_dfs_explicit(grid: &mut Vec<Vec<usize>>) -> Vec<Location> {
    let m = grid.len();
    let n = grid[0].len();
    let mut solutions: Vec<Location> = vec![];
    let mut seen: HashSet<Location> = HashSet::new();

    //explicit stack for DFS
    let mut stack: Vec<Location> = vec![];

    for (i, row) in grid.iter().enumerate() {
      for (j, _) in row.iter().enumerate() {
        if grid[i][j] == 1 && !seen.contains(&(i, j)) {
          let found_loc = (i, j);

          solutions.push(found_loc);
          seen.insert(found_loc);
          stack.push(found_loc);

          while let Some(loc) = stack.pop() {
            stack.extend(
              TwoD_PathFinding::get_candidates_locs(loc, m, n).filter_map(|(r, c)| {
                if grid[r][c] == 1 && !seen.contains(&(r, c)) {
                  seen.insert((r, c));
                  Some((r, c))
                } else {
                  None
                }
              }),
            );
          }
        }
      }
    }

    solutions
  }

  pub fn get_start_coords_dfs_implicit(grid: &Vec<Vec<usize>>) -> Vec<Location> {
    let m = grid.len();
    let n = grid[0].len();
    let mut solutions: Vec<Location> = vec![];
    let mut seen: HashSet<Location> = HashSet::new();

    // recursive helper
    fn mark_connected_points(
      grid: &Vec<Vec<usize>>,
      seen: &mut HashSet<Location>,
      i: usize,
      j: usize,
      m: usize,
      n: usize,
    ) {
      let location = (i, j);
      println!("Marking: i:{i} j:{j}");

      for (i, j) in TwoD_PathFinding::get_candidates_locs(location, m, n) {
        mark_connected_points(grid, seen, i, j, m, n);
      }
    }

    // iterate all points. if a point is a 1, it's a product. if you have not seen that point
    // you have found a new start coordinate for an island.
    // if you have seen it, skip it

    for (i, row) in grid.iter().enumerate() {
      for (j, _) in row.iter().enumerate() {
        if grid[i][j] == 1 && !seen.contains(&(i, j)) {
          println!("found: i:{i} j:{j}");
          let found_loc = (i, j);
          solutions.push(found_loc);
          seen.insert(found_loc);

          // start DFS
          mark_connected_points(grid, &mut seen, i, j, m, n);
        }
      }
    }

    solutions
  }
}

#[cfg(test)]
mod two_d_tests {
  use super::{TwoD_PathFinding, *};

  #[test]
  fn two_d_test_dfs_implicit() {
    let mut grid = vec![
      vec![1, 1, 1, 0, 0],
      vec![1, 1, 0, 0, 0],
      vec![1, 0, 1, 1, 0],
      vec![1, 0, 1, 0, 0],
    ];

    let start_coords = TwoD_PathFinding::get_start_coords_dfs_implicit(&mut grid);
    assert_eq!(vec![(0, 0), (2, 2)], start_coords);
  }

  #[test]
  fn two_d_test_dfs_explicit() {
    let mut grid = vec![
      vec![1, 1, 1, 0, 0],
      vec![1, 1, 0, 0, 0],
      vec![1, 0, 1, 1, 0],
      vec![1, 0, 1, 0, 0],
    ];

    let start_coords = TwoD_PathFinding::get_start_coords_dfs_explicit(&mut grid);
    assert_eq!(vec![(0, 0), (2, 2)], start_coords);
  }

  #[test]
  fn two_d_test_bfs() {
    let mut grid = vec![
      vec![1, 1, 1, 0, 0],
      vec![1, 1, 0, 0, 0],
      vec![1, 0, 1, 1, 0],
      vec![1, 0, 1, 0, 0],
    ];

    let start_coords = TwoD_PathFinding::get_start_coords_bfs(&mut grid);
    assert_eq!(vec![(0, 0), (2, 2)], start_coords);
  }

  #[test]
  fn test_get_candidates_locs() {
    let cands = TwoD_PathFinding::get_candidates_locs((0, 0), 4, 4).collect::<Vec<Location>>();

    assert!(cands.len() == 2);
    assert_eq!(vec![(0, 1), (1, 0)], cands);

    let cands2 = TwoD_PathFinding::get_candidates_locs((1, 1), 4, 4).collect::<Vec<Location>>();
    assert!(cands2.len() == 4);
    assert_eq!(vec![(1, 2), (1, 0), (2, 1), (0, 1)], cands2);

    let cands3 = TwoD_PathFinding::get_candidates_locs((3, 3), 4, 4).collect::<Vec<Location>>();
    assert_eq!(cands3.len(), 2);
    assert_eq!(vec![(3, 2), (2, 3)], cands3);
    // dbg!(cands);
  }
}
