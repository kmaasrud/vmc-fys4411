/// Struct that represents a single particle.
#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vec<f64>,
    pub qforce: Vec<f64>,
    pub dim: usize,
}

impl Particle {
    /// Creates a new particle with a given dimensionality.
    /// The particle's initial position is set to 0.
    pub fn new(dim: usize) -> Particle {
        Particle{
            position: vec![0.; dim],
            qforce: vec![0.; dim],
            dim: dim,
        }
    }

    /// Computes the squared sum of each coordinate.
    pub fn squared_sum(&self) -> f64 {
        self.position.iter().map(|x| x.powi(2)).sum()
    }

    /// Computes the squared sum of each coordinate, but the z-component is scaled by a factor
    pub fn squared_sum_scaled_z(&self, factor: &f64) -> f64 {
        match self.dim {
            1 | 2 => self.squared_sum(),
            _ => self.position[0].powi(2) + self.position[1].powi(2) + factor * self.position[2].powi(2),
        }
    }
    
    /// Returns the distance of this particle to other
    pub fn distance_to(&self, other: &Particle) -> f64 {
        let result: f64 = other.position.iter()
            .zip(self.position.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum();
        result.sqrt()
    }

    /// Adds bump_size to the specified position coordinate
    pub fn bump_at_dim(&mut self, dim: usize, bump_size: f64) {
        self.position[dim] += bump_size;
    }
}
