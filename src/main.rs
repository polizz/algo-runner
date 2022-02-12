use argh;

use percolate::Percolator;

mod args;
use args::*;

fn main() {
    let TopLevel {
        command: SubCommand::Percolate(PercolateArgs { trials, n } )
    } = argh::from_env();

    println!("args: n => {}, trials => {}", n, trials);

    let perc = Percolator::new(n);
    println!("{:?}", &perc);
}
