use argh::FromArgs;

#[derive(FromArgs, Debug)]
#[argh(description = "Commands")]
pub struct TopLevel {
    #[argh(subcommand)]
    pub command: SubCommand,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum SubCommand {
    Percolate(PercolateArgs),
    GridTravel(GridTravelArgs)
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "percolate", description = "Run percolation simluation")]
pub struct PercolateArgs {
    #[argh(option,  description = "trials", short = 't')]
    pub trials: usize,
    #[argh(option, description = "n", short = 'n')]
    pub n: usize,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "gridtravel", description = "How many ways can you travel through a grid")]
pub struct GridTravelArgs {
    #[argh(option,  description = "rows", short = 'm')]
    pub rows: usize,
    #[argh(option, description = "cols", short = 'n')]
    pub columns: usize,
}