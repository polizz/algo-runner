use percolate::Simulation;

mod args;
use args::*;

fn main() {
    let TopLevel {
        command: SubCommand::Percolate(PercolateArgs { trials, n } )
    } = argh::from_env();

    println!("args: n => {}, trials => {}", n, trials);

    let mut sim = Simulation::new(n, trials as u32);
    
    sim.start();
}
