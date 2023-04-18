use argh::FromArgs;

fn default_hashtable_num_buckets() -> usize {
  179
}

#[derive(FromArgs, Debug)]
#[argh(description = "Commands")]
pub struct AppArgs {
  #[argh(subcommand)]
  pub command: SubCommand,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum SubCommand {
  Percolate(PercolateArgs),
  GridTravel(GridTravelArgs),
  CanSum(CanSumArgs),
  HashTableTest(HashTableArgs),
}

#[derive(FromArgs, Debug)]
#[argh(
  subcommand,
  name = "percolate",
  description = "Run percolation simluation"
)]
pub struct PercolateArgs {
  #[argh(option, description = "trials", short = 't')]
  pub trials: usize,
  #[argh(option, description = "n", short = 'n')]
  pub n: usize,
}

#[derive(FromArgs, Debug)]
#[argh(
  subcommand,
  name = "gridtravel",
  description = "How many ways can you travel through a grid"
)]
pub struct GridTravelArgs {
  #[argh(option, description = "rows", short = 'm')]
  pub rows: usize,
  #[argh(option, description = "cols", short = 'n')]
  pub columns: usize,
}

#[derive(FromArgs, Debug)]
#[argh(
  subcommand,
  name = "cansum",
  description = "Can you arrive at a target sum with provided integers"
)]
pub struct CanSumArgs {
  #[argh(option, description = "target", short = 't')]
  pub target: i32,
  #[argh(option, description = "nums", short = 'n')]
  pub nums: Vec<i32>,
}

#[derive(FromArgs, Debug)]
#[argh(
  subcommand,
  name = "hashtable",
  description = "Test characteristics of different hashtable implementations"
)]
pub struct HashTableArgs {
  #[argh(option, description = "path to file to import words from, short = 'f'")]
  pub word_file_path: std::path::PathBuf,
  #[argh(
    option,
    description = "number of buckets to initialize",
    short = 'm',
    default = "default_hashtable_num_buckets()"
  )]
  pub num_buckets: usize,
}
