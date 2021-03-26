use crate::Particle;


pub struct WaveFunction {
    pub alpha: f64,
    pub beta: f64,
}

impl WaveFunction {
    /// Evaluate the wavefunction using only the single-particle part. Returns an f64 representing
    /// the wavefunction value.
    pub fn evaluate_non_interacting(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
        (- self.alpha * squared_position_sum).exp()
    }

    /// Evaluate the full wavefunction over particles: &Vec<Particles>. Returns an f64 representing
    /// the wavefunction value.
    pub fn evaluate(&self, particles: &Vec<Particle>) -> f64 {
        let mut r: f64; 
        let mut jastrow: f64 = 1.;
        let a: f64 = 0.0043;

        // Jastrow interaction
        for i in 0..particles.len() {
            for j in i+1..particles.len() {
                r = particles[i].distance_to(&particles[j]);
                // Check against hard-core diameter
                if r <= a {
                    jastrow *= 0.;
                } else {
                    jastrow *= 1. - a / r;
                }
            }
        }
        self.evaluate_non_interacting(particles) * jastrow
    }


    pub fn laplace_non_interacting(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
        - 2. * (particles[0].dim * particles.len()) as f64 * self.alpha + 4. * self.alpha.powi(2) * squared_position_sum
    }

    pub fn laplace(&self, particles: &Vec<Particle>) -> f64 {
        let mut laplace = 0.;
        for (i, particle) in particles.iter().enumerate() {
            laplace += self.laplace_spf(particle);
            let gradient_spf = self.gradient_spf(particle);
            let gradient_interaction = self.gradient_interaction(i, particles);
            for dim in 0..particle.dim {
                laplace += 2. * gradient_spf[dim] * gradient_interaction[dim] + gradient_interaction[dim].powi(2);
            }
            laplace += self.laplace_interaction(i, particles);
        }
        laplace
    }

    fn laplace_spf(&self, particle: &Particle) -> f64 {
        -2. * self.alpha * (particle.dim as f64 - 1. + self.beta) + 4. * self.alpha.powi(2) * particle.squared_sum_scaled_z(&self.beta)
    }

    fn gradient_spf(&self, particle: &Particle) -> Vec<f64> {
        let mut gradient = particle.position.clone();
        if gradient.len() > 2 { gradient[2] *= self.beta; }
        gradient.iter().map(|x| -2. * self.alpha * x).collect()
    }

    fn laplace_interaction(&self, i: usize, particles: &Vec<Particle>) -> f64 {
        let mut laplace = 0.;
        let a: f64 = 0.0043;
        let a2 = a.powi(2);
        let factor = a * (particles[i].dim as f64 - 1.);

        for j in 0..particles.len() {
            if i == j { continue }
            let distance: f64 = particles[i].distance_to(&particles[j]);
            let distance_minus_a = distance - a;
            laplace += factor / (distance.powi(2) * distance_minus_a) + (a2 - 0.0086 * distance) / (distance.powi(2) * distance_minus_a.powi(2));
        }
        laplace
    }

    fn gradient_interaction(&self, i: usize, particles: &Vec<Particle>) -> Vec<f64> {
        let mut gradient = vec![0.; particles[i].dim];
        let a: f64 = 0.0043;

        for j in 0..particles.len() {
            if i == j { continue }
            let distance: f64 = particles[i].distance_to(&particles[j]);
            for dim in 0..particles[i].dim {
                gradient[dim] += a * (particles[i].position[dim] - particles[j].position[dim]) / (distance.powi(2) * (distance - a));
            }
        }
        gradient
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


    // Returns the Laplacian of the wavefunction evaluated numerically at state of 'particles'.
    pub fn laplace_numeric(&self, particles: &mut Vec<Particle>) -> f64 {
        let h: f64 = 0.0001;
        let h2 = h.powi(2);
        let mut laplace = 0.;
        let wf = self.evaluate_non_interacting(&particles);

        for i in 0..particles.len() {
            for dim in 0..particles[i].dim {
                particles[i].bump_at_dim(dim, h); // Initial position +h
                let wf_plus = self.evaluate_non_interacting(&particles);

                particles[i].bump_at_dim(dim, -2. * h); // Initial position -h
                let wf_minus = self.evaluate_non_interacting(&particles);

                particles[i].bump_at_dim(dim, h); // Reset back to initial position

                laplace += (wf_plus - 2. * wf + wf_minus) / h2; 
            }
        }
        laplace / wf
    }


    /// Returns the gradient of the wavefunction with regards to alpha
    pub fn gradient_alpha(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum_sum: f64 = particles.iter().map(|x| x.squared_sum_scaled_z(&self.beta)).sum();
        - squared_position_sum_sum
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
