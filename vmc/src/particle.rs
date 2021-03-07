/// Struct that represents a single particle.
///
/// # Attributes
///
/// - `position: Vec<f64>` - The position of the particle, with an arbitrary dimensionality.
/// - `dimensions: u8` - The dimensionality of `position`.
#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vec<f64>,
    pub qforce: Vec<f64>,
    pub dim: usize,
}

impl Particle {
    /// Creates a new particle with a given dimensionality.
    /// The particle's initial position is set to 0.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let boson = Particle::new(3)
    /// ```
    pub fn new(dim: usize) -> Particle {
        Particle{
            position: vec![0.; dim],
            qforce: vec![0.; dim],
            dim: dim,
        }
    }

    /// Computes the sum of each coordinate squared
    pub fn squared_sum(&self) -> f64 {
        self.position.iter().map(|x| x.powi(2)).sum()
    }

    pub fn squared_sum_scaled_z(&self, lambda: &f64) -> f64 {
        self.position[0].powi(2) + self.position[1].powi(2) + lambda * self.position[2].powi(2)
    }

    // Takes input particle, find quantum force by evaluating its wavefunction
    // QUESTION: Is this fn dead and replaced by WaveFunction::quantum_force?
    pub fn quantum_force(&self, particles: &Vec<Particle>) -> Vec<f64> {
        // Loop over all other particles in order to calculate quantum force felt from all of them.
        // TODO: THIS SHOULD BE PARALELLIZED
        let mut r: f64;
        let mut deno: f64;
        let mut qforce: Vec<f64> = Vec::new();
        for _ in 0..self.dim {qforce.push(0.)} //Creating vector elements for all the dimensions

        for otherparticle in particles {
            // Radius: Distance between the chosen particle, and all the others.
            r = 0.;
            for i in 0..self.dim {
                r += (otherparticle.position[i]-self.position[i]).powi(2);
            }
            r = r.sqrt();
            deno = 1.0/(1.+self.beta*r);

            for i in 0..self.dim {
                qforce[i] += -2.*self.position[i]*self.alpha*(self.position[i]-otherparticle.position[i])*deno*deno/r;
            }
        }
        // After this loop, all qforce vectors should be summed to create the total quantum force.
        // MORE: https://compphysics.github.io/ComputationalPhysics2/doc/pub/week4/html/week4-reveal.html slide 11
        qforce
    }
}
