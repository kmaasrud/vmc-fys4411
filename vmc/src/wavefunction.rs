use crate::Particle;

pub trait WaveFunction {
    fn evaluate(&self, particles: &Vec<Particle>) -> f64;
    fn laplace(&self, particles: &Vec<Particle>) -> f64;
    fn gradient(&self, particle: Particle) -> Vec<f64>;
    // Here I'm just going by what @Schoyen has done, let's see if it works
    fn drift_force(&self, particle: Particle) -> Vec<f64> {
        self.gradient(particle).iter().map(|x| x * 2.).collect()
    }
    fn quantum_force(&self, particle: Particle, particles: &Vec<Particle>) -> Vec<f64>;
    fn greens(&self, particle: Particle, particles: &Vec<Particle>, step_size: f64) -> f64;
}


// TODO: Docstring
#[derive(Debug)]
pub struct GaussianWaveFunction {
    alpha: f64,
    beta: f64,
}

impl GaussianWaveFunction {
    pub fn new(alpha: f64) -> Self {
        GaussianWaveFunction { alpha: alpha, beta: alpha}
    }
}

// TODO: Check all these impls with our own theory. Just going by @mhjensen and @Schoyens word here.
impl WaveFunction for GaussianWaveFunction {
    fn evaluate(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
        (-self.alpha * squared_position_sum).exp()
    }

    fn laplace(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
        let dim = particles[0].dim as f64;
        let n = particles.len() as f64;

        2. * dim * n * self.alpha + 4. * squared_position_sum * self.alpha.powi(2)
    }

    fn gradient(&self, particle: Particle) -> Vec<f64> {
        particle.position.iter().map(|x| -2. * self.alpha * x).collect()
    }

    //  Takes input particle, find quantum force by evaluating its wavefunction
    fn quantum_force(&self, particle: Particle, particles: &Vec<Particle>) -> Vec<f64> {
        //Loop over all other particles in order to calculate quantum force felt from all of them.
        //THIS SHOULD BE PARALELLIZED
        let mut r: f64;
        let mut deno: f64;
        let mut qforce: Vec<f64>;
        for i in 0..particle.dim {qforce.push(0.)} //Creating vector elements for all the dimensions

        for otherparticle in particles {
            //Radius: Distance between the chosen particle, and all the others.
            r = 0.;
            for i in 0..particle.dim {
                r += (otherparticle.position[i]-particle.position[i]).powi(2);
            }
            r = r.sqrt();
            deno = 1.0/(1.+self.beta*r);

            for i in 0..particle.dim {
                qforce[i] += -2.*particle.position[i]*self.alpha*(particle.position[i]-otherparticle.position[i])*deno*deno/r;
            }
        }
        //After this loop, all qforce vectors should be summed to create the total quantum force.
        //MORE: https://compphysics.github.io/ComputationalPhysics2/doc/pub/week4/html/week4-reveal.html slide 11
        qforce
    }
    //  Takes the input particle + quantum_force, inserts in greens func and returns
    fn greens(&self, particle: Particle, particles: &Vec<Particle>, step_size: f64) -> f64 {
        let langevin_part: Vec<f64> = * exp(-(y-x));
        1./((4. * 3.14*0.5*step_size).powf(3.*(particles.len() as f64)/2.)) 
    }
}
