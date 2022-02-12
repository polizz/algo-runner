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
    Percolate(PercolateArgs)
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "percolate", description = "Run percolation simluation")]
pub struct PercolateArgs {
    #[argh(option,  description = "trials", short = 't')]
    pub trials: usize,
    #[argh(option, description = "n", short = 'n')]
    pub n: usize,
}