use crate::Particle;


pub struct WaveFunction {
    pub alpha: f64,
    pub beta: f64,
}

impl WaveFunction {
    // --- Evaluation of wavefunctions ---
    /// Evaluate the wavefunction using only the single-particle part. Returns an f64 representing
    /// the wavefunction value.
    pub fn evaluate_non_interacting(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum_scaled_z(&self.beta)).sum();
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

    // --- Laplacian ---
    /// Returns the Laplacian of the wavefunction evaluated numerically at state of 'particles'.
    pub fn laplace(&self, particles: &mut Vec<Particle>, non_interacting: bool) -> f64 {
        let h: f64 = 0.0001;
        let h2 = h.powi(2);
        let mut laplace = 0.;
        let wf = if non_interacting { self.evaluate_non_interacting(&particles) } else { self.evaluate(&particles) };

        for i in 0..particles.len() {
            for dim in 0..particles[i].dim {
                particles[i].bump_at_dim(dim, h); // Initial position +h
                let wf_plus = if non_interacting { self.evaluate_non_interacting(&particles) } else { self.evaluate(&particles) };

                particles[i].bump_at_dim(dim, -2. * h); // Initial position -h
                let wf_minus = if non_interacting { self.evaluate_non_interacting(&particles) } else { self.evaluate(&particles) };

                particles[i].bump_at_dim(dim, h); // Reset back to initial position

                laplace += (wf_plus - 2. * wf + wf_minus) / h2; 
            }
        }
        laplace / wf
    }

    // --- Gradients ---
    /// Returns the gradient for a particle with regards to the non-interacting part of the
    /// wavefunction
    fn gradient_spf(&self, particle: &Particle) -> Vec<f64> {
        let mut gradient = particle.position.clone();
        if gradient.len() > 2 { gradient[2] *= self.beta; }
        gradient.iter().map(|x| - 2. * self.alpha * x).collect()
    }
    /// Returns the gradient for a particle with regards to the interaction-part of the
    /// wavefunction
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
    /// Returns the gradient of the wavefunction with regards to alpha
    pub fn gradient_alpha(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum_sum: f64 = particles.iter().map(|x| x.squared_sum_scaled_z(&self.beta)).sum();
        - squared_position_sum_sum
    }

    // --- Quantum forces ---
    pub fn quantum_force(&self, i: usize, particles: &Vec<Particle>) -> Vec<f64> {
        let quantum_force = self.gradient_spf(&particles[i]).iter()
            .zip(self.gradient_interaction(i, particles).iter())
            .map(|(x, y)| 2. * (x + y))
            .collect();
        quantum_force
    }
    /// Calculates the quantum force of a particle not interacting with its surrounding particles
    pub fn quantum_force_non_interacting(&self, particle: &Particle) -> Vec<f64> {
        self.gradient_spf(particle).iter().map(|x| 2. * x).collect()
    }
}
