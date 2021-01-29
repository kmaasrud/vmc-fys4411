use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::distributions::{Uniform, Distribution};
use crate::HarmonicTrap;

// Trait for Metropolis samplers. 
trait Metropolis {
    // Samples a new step based on `curr_step`. Does not update
    // the value of `self.curr_step`.
    fn sample(&mut self, curr_step: &f64) -> f64 {
        let mut rng = thread_rng();
        let next_step: f64 = self.next_step();
        let uniform = Uniform::new(0., 1.);
        
        if uniform.sample(&mut rng) < self.acceptance_factor(next_step) {
            next_step
        } else {
            *curr_step
        }
    }
    
    fn acceptance_factor(&self, next_step: f64) -> f64;
    fn next_step(&self) -> f64;
}

// Struct for representing a brute force Metropolis algorithm.
// Implements the Metropolis trait.
// 
// ## Example
// 
// ```
// let mut bf_metro = BruteForceMetropolis::new(0.5);
// println!("{}", bf_metro.curr_step);
// bf_metro.make_step();
// println!("{}", bf_metro.curr_step);
// ```
struct BruteForceMetropolis {
    curr_step: f64,
    step_size: f64,
}

impl BruteForceMetropolis {
    // Makes a new `BruteForceMetropolis` struct based on a step size.
    fn new(step_size: f64) -> Self {
        Self{ curr_step: 0., step_size: step_size, }
    }

    // Makes a new step based on `curr_step`. Also updates
    // the value of `self.curr_step`.
    fn step(&mut self) -> f64 {
        self.curr_step = self.sample(&self.curr_step);
        self.curr_step
    }
}

impl Metropolis for BruteForceMetropolis {
    // Calculates the acceptance factor based on the curren step (stored in the struct) and the next step. 
    // TODO: This method needs rewriting
    fn acceptance_factor(&self, next_step: f64) -> f64 {
        let trap_old: f64 = HarmonicTrap::spherical(&mut HarmonicTrap{ mass: 1., omega_ho: 1., omega_z: 1. }, self.curr_step);
        let trap_next: f64 = HarmonicTrap::spherical(&mut HarmonicTrap{ mass: 1., omega_ho: 1., omega_z: 1. }, next_step);
        let hastings_ratio: f64 = trap_old / trap_next;
        // println!("HR: {}", hastings_ratio);
        // Return hastings ratio if it is smaller than 1, else 1
        hastings_ratio.min(1.)
    }
    
    // This is what makes this a brute force method, as `BruteForceMetropolis` only makes
    // a random step in either direction. 
    fn next_step(&self) -> f64 {
        // thread_rng() randomly chooses either `1.` or `-1.`. Don't know if this is the most efficient way, but it should work...
        self.curr_step + [1.,-1.].choose(&mut thread_rng()).unwrap() * self.step_size
    }
}