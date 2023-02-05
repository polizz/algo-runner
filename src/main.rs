use algorithms::Simulation as MonteCarlo;
use algorithms::get_all_grid_paths;

mod args;
use args::*;

fn main() {
    let top: TopLevel = argh::from_env();

    if let SubCommand::Percolate(PercolateArgs { trials, n } ) = top.command {
        println!("args: n => {}, trials => {}", n, trials);

        let mut sim = MonteCarlo::new(n, trials as u32);
        
        sim.start();
    } else if let SubCommand::GridTravel(GridTravelArgs { rows, columns } ) = top.command {
        println!("Total paths through {}x{} grid is: {}", rows, columns, get_all_grid_paths(rows, columns));
    }
}
