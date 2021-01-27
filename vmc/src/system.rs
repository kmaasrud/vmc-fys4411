use crate::Particle;
use rand::Rng;

pub struct System {
    particles: Vec<Particle>,
    dimensionality: u8,
}

impl System {
    pub fn new() -> System {
        System {
            particles: Vec::new(),
            dimensionality: 0,
        }
    }
}

pub struct MHSampler {
    curr_step: f64,
    next_step: f64,
    acceptance_factor: f64,
    rand: f64,
    time: i64,
}

impl MHSampler {
    fn solve(&mut self, init_step: f64, n: u64) {
        self.curr_step = init_step;
        let mut rng = rand::thread_rng(); // Initializing mutable random number generator

        for i in 0..n {
            self.next_step: f64 = self.gen_rand_state();
            self.acceptance_factor: f64 = self.acceptance_factor();
            self.rand: f64 = rng.gen::<f64>(); // Pulling a float between 0 and 1 from the rng thread

            if self.rand < self.acceptance_factor {
                self.curr_step = self.next_step;
            } else {
                //if the random number is above acceptance factor, then we stay!
                continue;
            }
        }
    }

    fn gen_rand_state(&self) -> f64 {
        //Generates next_step according to g(next_step|curr_step)

        212.2 //Example to to avoid error
    }

    fn acceptance_factor(&self) -> f64 {
        //Finds acceptance factor

        0.01 //Example to to avoid error
    }
}
