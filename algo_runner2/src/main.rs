use algorithms::Simulation as MonteCarlo;
use algorithms::get_all_grid_paths;
use data_structures::*;

mod args;
use args::*;

fn main() {
  let args: AppArgs = argh::from_env();

  if let SubCommand::Percolate(PercolateArgs { trials, n }) = args.command {
    println!("args: n => {}, trials => {}", n, trials);

    let mut sim = MonteCarlo::new(n, trials as u32);

    sim.start();
  } else if let SubCommand::GridTravel(GridTravelArgs { rows, columns }) = args.command {
    println!(
      "Total paths through {}x{} grid is: {}",
      rows,
      columns,
      get_all_grid_paths(rows, columns)
    );
  } else if let SubCommand::HashTableTest(HashTableArgs {
    num_buckets,
    word_file_path,
  }) = args.command
  {
    let file_str = get_words_from_file(word_file_path);
    let words: Vec<&str> = file_str.split_ascii_whitespace().collect();
    let mut ht: HashTableLinear<&str, usize> = HashTableLinear::new(num_buckets);

    words.iter().enumerate().for_each(|(ix, val)| {
      #[allow(suspicious_double_ref_op)]
      ht.put(val.clone(), ix);
    });

    // println!("HT: {:?}", &ht.keys());
    // println!("Unique words: {:?}", &ht.keys().iter().filter(|w| w.is_some()).count());

    // words.iter().take(1).for_each(|val| {
    //   ht.get(val);
    // });
    words.iter().for_each(|val| {
      ht.get(val);
    });

    words.iter().for_each(|val| {
      ht.delete(val);
    });
  }
}
