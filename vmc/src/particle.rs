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
        match self.dim {
            1 => self.position[0].powi(2),
            2 => self.position[0].powi(2) + self.position[1].powi(2),
            3 =>  self.position[0].powi(2) + self.position[1].powi(2) + lambda * self.position[2].powi(2),
            _ => panic!("Dimension should be 1, 2 or 3.")
        }
    }
    
    pub fn distance_to(&self, other: &Particle) -> f64 {
        let result: f64 = other.position.iter()
            .zip(self.position.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum();
        result.sqrt()
    }
}
