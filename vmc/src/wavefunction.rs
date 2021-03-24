use crate::Particle;


pub struct WaveFunction {
    pub alpha: f64,
    pub beta: f64,
}

impl WaveFunction {
    /// Evaluate the full wavefunction over particles: &Vec<Particles>. Returns an f64 representing
    /// the wavefunction value.
    pub fn evaluate(&self, particles: &Vec<Particle>) -> f64 {
        let mut r: f64; 
        let mut g: f64 = 1.;
        let mut f: f64 = 1.;
        let n_particles = particles.len();

        for i in 0..n_particles {
            // Normal single-particle wave function
            g *= (-self.alpha * particles[i].squared_sum_scaled_z(&self.beta)).exp();

            // Jastrow interaction (NOTE: Hard-core diameter is hard-coded here)
            for j in i+1..n_particles {
                r = particles[i].distance_to(&particles[j]);
                // Check against hard-core diameter
                if r > 0.0043 {
                    f *= 1. - 0.0043 / r;
                } else {
                    f *= 0.;
                }
            }
        }
        g * f 
    }

    /// Evaluate the wavefunction using only the single-particle part. Returns an f64 representing
    /// the wavefunction value.
    pub fn evaluate_non_interacting(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_prod: f64 = particles.iter().map(|x| -self.alpha * x.squared_sum()).product();
        squared_position_prod.exp()
    }

    /// Returns the Laplacian of the wavefunction evaluated at state of particles: &Vec<Particle>.
    pub fn laplace(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
        let dim = particles[0].dim as f64;
        let n = particles.len() as f64;

        2. * dim * n * self.alpha + 4. * squared_position_sum * self.alpha.powi(2)
    }

    pub fn gradient(&self, particle: Particle) -> Vec<f64> {
        particle.position.iter().map(|x| -2. * self.alpha * x).collect()
    }

    /// Returns the gradient of the wavefunction with regards to alpha
    pub fn gradient_alpha(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_prod: f64 = particles.iter().map(|x| -self.alpha * x.squared_sum_scaled_z(&self.beta)).product();
        squared_position_prod * self.evaluate(particles)
    }

    /// Calculates the quantum force of a particle not interacting with its surrounding particles
    pub fn quantum_force_non_interacting(&self, particle: &Particle) -> Vec<f64> {
        particle.position.iter().map(|x| -4. * self.alpha * x).collect()
    }

    ///  Takes input particle index and finds it's quantum force by evaluating its wavefunction
    pub fn quantum_force(&self, i: usize, particles: &Vec<Particle>) -> Vec<f64> {
        let mut r: f64;
        let mut factor: f64;
        let mut qforce: Vec<f64> = self.quantum_force_non_interacting(&particles[i]);
        if particles[i].dim > 2 {
            qforce[2] *= self.beta; // Multiply the z-component by beta
        }

        // Loop over all other particles in order to calculate quantum force felt from all of them.
        for other in particles {
            // If positions are same, jump to next iteration
            if other.position == particles[i].position { continue }
            r = other.distance_to(&particles[i]);
            factor = 2. * self.alpha / (r.powi(3) - r * self.alpha);

            for i in 0..particles[i].dim {
                qforce[i] += factor * (particles[i].position[i] - other.position[i]);
            }
        }
        qforce
    }
}
