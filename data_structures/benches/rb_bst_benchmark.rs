use criterion::{criterion_group, criterion_main, Criterion};

pub fn rb_bst_benchmark(c: &mut Criterion) {
  use data_structures::{TreeStat, BST};

  fn has_perfect_black_balance() {
    let mut bst: BST<&str, usize> = BST::new();

    bst.put("S", 1);
    bst.put("E", 2);
    bst.put("A", 3);
    bst.put("R", 4);
    bst.put("C", 5);
    bst.put("H", 6);
    bst.put("X", 7);
    bst.put("M", 8);
    bst.put("P", 9);
    bst.put("L", 10);

    // let (min, max) = bst.get_min_max_black_depth();

    // println!("tree: {}", &bst);
    // println!("Black depth min: {}, max: {}", min, max);

    // assert_eq!(min, max);
  }

  c.bench_function("rb_bst default", |b| b.iter(|| has_perfect_black_balance()));
}

criterion_group!(benches, rb_bst_benchmark);
criterion_main!(benches);
