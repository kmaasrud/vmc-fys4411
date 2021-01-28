use rand::thread_rng;
use rand::rngs::ThreadRng;
use rand::distributions::{Uniform, Distribution};
use rand_distr::Normal;
use crate::HarmonicTrap;

pub struct SimpleMetropolis {
    pub curr_step: f64,
    pub next_step: f64,
    rng: ThreadRng,
}

impl SimpleMetropolis {
    pub fn new() -> SimpleMetropolis {
        SimpleMetropolis{ curr_step: 0., next_step: 0., rng: thread_rng(), }
    }
    pub fn solve(&mut self, init_step: f64, n: u64) {
        self.curr_step = init_step;

        // rand::distributions::Uniform represents the uniform distribution. In this case [0,1) - so does not include 1 (TODO: Is this a problem?)
        let uniform = Uniform::new(0., 1.);

        for _ in 0..n {
            // Get next step from some proposed distribution given the current step.
            self.next_step = self.gen_next_step();

            //println!("Curr_step: {}, next_step: {}",self.curr_step, self.next_step);
            println!("Curr_step: {}", self.curr_step);

            if uniform.sample(&mut self.rng) < self.acceptance_factor() {
                self.curr_step = self.next_step;
            } else {
                //if the random number is above acceptance factor, then we stay!
                continue;
            }
            // TODO: The steps need to be stored somewhere, perhaps returned or stored in the struct?
        }
    }

    fn gen_next_step(&mut self) -> f64 {
        // Generates next_step according to rand_distr::Normal
        let normal = Normal::new(self.curr_step, 1.).unwrap();
        normal.sample(&mut self.rng)
    }

    // Finds acceptance factor
    fn acceptance_factor(&self) -> f64 {
        // Calculate Hastings ratio
        let trap_old: f64 = HarmonicTrap::spherical(&mut HarmonicTrap{ mass: 1., omega_ho: 1., omega_z: 1. }, self.curr_step);
        let trap_next: f64 = HarmonicTrap::spherical(&mut HarmonicTrap{ mass: 1., omega_ho: 1., omega_z: 1. }, self.next_step);
        let hastings_ratio: f64 = trap_old / trap_next;
        // println!("HR: {}", hastings_ratio);
        // Return hastings ratio if it is smaller than 1, else 1
        hastings_ratio.min(1.)
    }
}