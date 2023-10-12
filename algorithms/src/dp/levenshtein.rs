#![allow(dead_code)]
use std::fmt;
static MAX_LEN: usize = 12;

#[derive(Clone, Debug, Copy)]
pub enum EditType {
  Match,
  Substitution,
  Insert,
  Delete,
  Unset,
}

impl std::fmt::Display for EditType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = match self {
      EditType::Match => "M",
      EditType::Substitution => "S",
      EditType::Insert => "I",
      EditType::Delete => "D",
      EditType::Unset => "U",
    };
    write!(f, "{}", s)
  }
}

#[derive(Clone, Debug)]
pub struct Operation {
  cost: usize,
  operation: EditType,
}

struct LevenVecDisplay(LevenOpMatrix);
type LevenOpMatrix = Vec<Vec<Operation>>;

impl std::fmt::Display for LevenVecDisplay {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for m in 0..self.0.len() {
      for n in 0..self.0[0].len() {
        let Operation { cost, operation } = self.0[m][n];
        let _ = write!(f, " {}_{:0>2} ", operation, cost);
      }
      let _ = write!(f, "\n");
    }

    Ok(())
  }
}

fn get_match_cost(c1: &u8, c2: &u8) -> usize {
  if c1 == c2 {
    0
  } else {
    1
  }
}

fn get_delete_cost(_c: &u8) -> usize {
  1
}

fn get_insert_cost(_c: &u8) -> usize {
  1
}

pub fn get_distance_matrix(pattern: &[u8], target: &[u8], cells: &mut LevenOpMatrix) {
  debug_assert!(pattern.len() > 0);
  debug_assert!(target.len() > 0);

  let mut match_cost: usize;
  let mut delete_cost: usize;
  let mut insert_cost: usize;

  for m in 1..=target.len() {
    for n in 1..=pattern.len() {
      match_cost = cells[m - 1][n - 1].cost + get_match_cost(&pattern[m - 1], &target[n - 1]);
      delete_cost = cells[m][n - 1].cost + get_delete_cost(&pattern[n - 1]);
      insert_cost = cells[m - 1][n].cost + get_insert_cost(&target[m - 1]);

      let mut min_op = Operation {
        cost: match_cost,
        operation: EditType::Match,
      };

      if delete_cost < min_op.cost {
        min_op.cost = delete_cost;
        min_op.operation = EditType::Delete;
      };

      if insert_cost < min_op.cost {
        min_op.cost = insert_cost;
        min_op.operation = EditType::Insert;
      };

      // dbg!(match_cost, delete_cost, insert_cost);
      cells[m][n] = min_op;
    }
  }
}

mod tests {
  use super::*;

  #[test]
  fn smoke() {
    let target = "you should".as_bytes();
    let pattern = "thou shalt".as_bytes();

    let mut cells = vec![
      vec![
        Operation {
          cost: 0,
          operation: EditType::Unset
        };
        MAX_LEN
      ];
      MAX_LEN
    ];

    for m in 0..cells.len() {
      cells[m][0] = Operation {
        cost: m,
        operation: EditType::Insert,
      }
    }

    for n in 0..cells[0].len() {
      cells[0][n] = Operation {
        cost: n,
        operation: EditType::Delete,
      }
    }
    // let dist = get_distance_matrix(pattern, target);
    get_distance_matrix(&pattern, &target, &mut cells);
    // dbg!(&cells);
    println!("{}", LevenVecDisplay(cells) as LevenVecDisplay);

    assert_eq!(true, true);
  }
}
