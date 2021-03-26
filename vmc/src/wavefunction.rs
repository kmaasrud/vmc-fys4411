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
                if r <= 0.0043 {
                    f *= 0.;
                } else {
                    f *= 1. - 0.0043 / r;
                }
            }
        }
        g * f 
    }

    /// Evaluate the wavefunction using only the single-particle part. Returns an f64 representing
    /// the wavefunction value.
    pub fn evaluate_non_interacting(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum: f64 = particles.iter().map(|x| -self.alpha * x.squared_sum()).sum();
        squared_position_sum.exp()
    }

    // Returns the Laplacian of the wavefunction evaluated numerically at state of 'particles'.
    pub fn laplace(&self, particles: &mut Vec<Particle>) -> f64 {
        let h = 0.0001;
        let mut second_deriv = 0.;
        let wf = self.evaluate(&particles);

        for i in 0..particles.len() {
            for dim in 0..particles[i].dim {
                particles[i].bump_at_dim(dim, h); // Initial position +h
                let wf_plus = self.evaluate(&particles);

                particles[i].bump_at_dim(dim, -2. * h); // Initial position -h
                let wf_minus = self.evaluate(&particles);

                particles[i].bump_at_dim(dim, h); // Reset back to initial position

                second_deriv -= wf_plus + wf_minus - 2. * wf; 
            }
        }
        0.5 * second_deriv / h.powi(2)
    }

    /// Returns the Laplacian of the wavefunction evaluated analytically at state of 'particles'.
    pub fn laplace_analytical(&self, particles: &Vec<Particle>) -> f64 {
        let mut laplace: f64 = 0.;
        let factor1 = 2. * self.alpha;
        let factor2 = 2. * self.alpha * self.beta;
        for particle in particles {
            for dim in 0..particle.dim {
                match dim + 1 {
                    1 | 2 => {
                        laplace += factor1 * (particle.position[dim] as f64).powi(2) - 1.;
                    }
                    _ => {
                        laplace += self.beta * (factor2 * (particle.position[dim] as f64).powi(2) - 1.);
                    }
                }
            }
        };
        - self.alpha * laplace * self.evaluate(particles)
    }

    /* pub fn gradient(&self, particle: Particle) -> Vec<f64> {
        particle.position.iter().map(|x| -2. * self.alpha * x).collect()
    } */

    /// Returns the gradient of the wavefunction with regards to alpha
    pub fn gradient_alpha(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum_sum: f64 = particles.iter().map(|x| - x.squared_sum_scaled_z(&self.beta)).sum();
        squared_position_sum_sum
    }

    /// Calculates the quantum force of a particle not interacting with its surrounding particles
    pub fn quantum_force_non_interacting(&self, particle: &Particle) -> Vec<f64> {
        particle.position.iter().map(|x| -4. * self.alpha * x).collect()
    }

    ///  Takes input particle index and finds it's quantum force by evaluating its wavefunction
    pub fn quantum_force(&self, particle: &Particle) -> Vec<f64> {
        let mut qforce: Vec<f64> = vec![0.; particle.dim];

        for dim in 0..particle.dim {
            match dim + 1 {
                1 | 2 => {
                    qforce[dim] = -4. * self.alpha * particle.position[dim] * (self.alpha * particle.squared_sum_scaled_z(&self.beta)).exp();
                }
                _ => {
                    qforce[dim] = -4. * self.alpha * self.beta * particle.position[dim] * (self.alpha * particle.squared_sum_scaled_z(&self.beta)).exp();
                }
            }
        }
        // let mut qforce: Vec<f64> = self.quantum_force_non_interacting(&particles[i]);
        // let mut factor: f64;
        // let mut r: f64;
        /* if particles[i].dim > 2 {
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
        } */
        qforce
    }
}
