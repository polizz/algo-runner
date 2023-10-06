use super::Percolator;

use rand::Rng;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Simulation {
  pub percolator: Percolator,
  trials: u32,
}

impl Simulation {
  pub fn new(n: usize, trials: u32) -> Self {
    let percolator = Percolator::new(n);

    Simulation { percolator, trials }
  }

  pub fn calc_pstar(&self) -> f64 {
    self.percolator.number_of_open_sites() as f64 / self.percolator.num_sites as f64
  }

  pub fn summary(&self, results: Vec<f64>, elapsed: Duration) {
    let trials_f64 = self.trials as f64;
    let mean_p = results.iter().sum::<f64>() / trials_f64;

    let s_dev = results
      .iter()
      .map(|&p_star| {
        let x = p_star - mean_p;
        x.powi(2)
      })
      .sum::<f64>()
      / ((self.trials - 1) as f64);

    let conf_denom = (1.96 * s_dev.sqrt()) / trials_f64.sqrt();
    let conf_lo = mean_p - conf_denom;
    let conf_hi = mean_p + conf_denom;

    let seconds = elapsed.as_secs();
    let millis = elapsed.as_millis();

    println!(
      "Total Trials: {}, completed in {} seconds ({} milliseconds).",
      self.trials, seconds, millis
    );
    println!("p* Mean: {mean_p}");
    println!("stddev: {s_dev}");
    println!("95% confidence interval: [{conf_lo}, {conf_hi}]");
  }

  pub fn start(&mut self) {
    let now = Instant::now();

    let mut rng = rand::thread_rng();
    let num_sites = self.percolator.num_sites + 1;
    let mut results = vec![0.0; self.trials as usize];

    (0..self.trials).for_each(|_| {
      // println!("Trial {trial}...");

      while !self.percolator.percolates() {
        let next_open_site = rng.gen_range(1..num_sites);

        self.percolator.open(next_open_site);
      }

      results.push(self.calc_pstar());
    });

    self.summary(results, now.elapsed());
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_init() {
    let mut sim = Simulation::new(500, 200);

    sim.start();

    // println!("{:?}", sim.percolator.union_find);
  }
}
