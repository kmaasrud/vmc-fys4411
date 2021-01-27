use rand::Rng;

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
            self.rand: f64 = rng.gen::<f64>(); // Pulling a float in [0,1) from the rng thread. This does not actually include 1, is that a problem?
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
        let rand_state:f64 = 
        212.2 //Example to to avoid error
    }

    fn acceptance_factor(&self) -> f64 {
        //Finds acceptance factor
        let mut acc_fac:f64; //Defining var for acceptance factor

        //Calculating Hastings Ratio
        let hastings_ratio:f64 = (self.gaussian(self.curr_step, self.next_step))/(self.gaussian(self.next_step, self.curr_step));

        if hastings_ratio < 1. {
            acc_fac = hastings_ratio;
        } else{
            acc_fac = 1.;
        }

        acc_fac //Returns the acceptance factor
    }
}
