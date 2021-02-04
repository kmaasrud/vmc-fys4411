use rand::thread_rng;
use rand::distributions::{Uniform, Distribution};

// Trait for Metropolis samplers. 
trait Metropolis {
    // Samples a new step based on `curr_step`. Does not update
    // the value of `self.curr_step`.
    fn do_change(&mut self) -> bool {
        let mut rng = thread_rng();
        let uniform = Uniform::new(0., 1.);
        
        if uniform.sample(&mut rng) < self.acceptance_factor() {
            true
        } else {
            false
        }
    }
    
    fn acceptance_factor(&self) -> f64;
    fn next_step(&mut self);
}

// Struct for representing a brute force Metropolis algorithm.
// Implements the Metropolis trait.
// 
// ## Example
// 
// ```
// let mut bf_metro = BruteForceMetropolis::new(0.5);
// bf_metro.curr_step = vec![1.0, 2.4, 0.6];
// println!("{}", bf_metro.curr_step);
// bf_metro.step();
// println!("{}", bf_metro.curr_step);
// ```
pub struct BruteForceMetropolis {
    // `curr_step` describes the N particles with D dimensions as a N * D dimensional vector.
    // This is computationally preferrable and allows for more general code. Do keep this in mind.
    curr_step: Vec<f64>,
    next_step: Vec<f64>,
    step_size: f64,
}

impl BruteForceMetropolis {
    // Makes a new `BruteForceMetropolis` struct based on a step size.
    fn new(step_size: f64) -> Self {
        // Initialize with random `curr_step`. Just setting empty for now, so the vector needs to be filled.
        Self{ curr_step: vec![], next_step: vec![], step_size: step_size, }
    }

    // Makes a new step based on `curr_step`. Also updates
    // the value of `self.curr_step`.
    fn step(&mut self) -> &Vec<f64> {
        self.next_step();
        if self.do_change() {
            &self.curr_step
        } else {
            &self.next_step
        }
    }
}

impl Metropolis for BruteForceMetropolis {
    // Calculates the acceptance factor based on the current step (stored in the struct) and the next step. 
    // TODO: This could perhaps actually be implemented as part of the trait.
    fn acceptance_factor(&self) -> f64 {
        // TODO: We need WaveFunction structs, the below is just random rubbish now
        let wave_function_old: f64 = self.curr_step.iter().sum();
        let wave_function_new: f64 = self.next_step.iter().sum();
        
        // Not random rubbish anymore
        let hastings_ratio: f64 = wave_function_new.powi(2) / wave_function_old.powi(2);

        // Return hastings ratio if it is smaller than 1, else 1
        hastings_ratio.min(1.)
    }
    
    // This is what makes this a brute force method, as `BruteForceMetropolis` only makes
    // a random step in either direction. 
    fn next_step(&mut self) {
        // thread_rng() randomly chooses either `1.` or `-1.`. Don't know if this is the most efficient way, but it should work...
        let uniform = Uniform::new(0., 1.);
        let mut next_step: Vec<f64> = vec![];
        for i in 0..self.curr_step.len() {
            // This is what Morten does in [this example](https://compphysics.github.io/ComputationalPhysics2/doc/pub/week2/html/week2.html#___sec11)
            // I trust him being correct, but I am not totally sure about the `- 0.5` part...
            next_step.push(self.curr_step[i]
                + (uniform.sample(&mut thread_rng()) - 0.5) * self.step_size);
        }
        self.next_step = next_step;
    }
}