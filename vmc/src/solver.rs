use rand::Rng;
use crate::HarmonicTrap;

pub struct SimpleMetropolis {
    pub curr_step: f64,
    pub next_step: f64,
    pub acceptance_factor: f64,
    pub rand: f64,
    pub time: i64,
}

impl SimpleMetropolis {
    pub fn solve(&mut self, init_step: f64, n: u64) {
        self.curr_step = init_step;
        let mut rng = rand::thread_rng(); // Initializing mutable random number generator

        for i in 0..n {
            
            self.next_step = self.curr_step + (rng.gen::<f64>()-1.); // self.gen_rand_state();
            //println!("Curr_step: {}, next_step: {}",self.curr_step, self.next_step);
            println!("Curr_step: {}", self.curr_step);
            self.acceptance_factor= self.acceptance_factor();
            self.rand = rng.gen::<f64>(); // Pulling a float in [0,1) from the rng thread. This does not actually include 1, is that a problem?
            // Now we need a proposal distribution and an acceptance distribution. The probability distribution(of transition) is their product.

            if self.rand < self.acceptance_factor {
                self.curr_step = self.next_step;
            } else {
                //if the random number is above acceptance factor, then we stay!
                continue;
            }
        }
    }
    fn gaussian(curr:f64, new:f64) -> f64 {
        let width:f64 = 1.;
        let g: f64 = (-(curr-new).powi(2)/(2.*width.powi(2))).exp();
        g
    }

    fn gen_rand_state(&self) -> f64 {
        //Generates next_step according to g(next_step|curr_step)
        2.
    }

    fn acceptance_factor(&self) -> f64 {
        //Finds acceptance factor

        //Calculating Hastings Ratio
        let trap_old:f64 = HarmonicTrap::spherical(&mut HarmonicTrap{ mass: 1., omega_ho: 1., omega_z: 1. },self.curr_step);
        let trap_next:f64 = HarmonicTrap::spherical(&mut HarmonicTrap{ mass: 1., omega_ho: 1., omega_z: 1. },self.next_step);
        let hastings_ratio:f64 = trap_old/trap_next;
        //println!("HR: {}", hastings_ratio);
        if hastings_ratio < 1. { //Maybe tricky syntax. One of the statements will happen, thus one of them must be returned. Cray
            hastings_ratio
        } else{
            1.
        }

    }
}
