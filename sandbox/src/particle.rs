/// Struct that represents a single particle.
///
/// # Attributes
///
/// - `position: Vec<f64>` - The position of the particle, with an arbitrary dimensionality.
/// - `dimensions: u8` - The dimensionality of `position`.
#[derive(Debug)]
pub struct Particle {
    pub position: Vec<f64>,
    pub dimensions: u8,
}

impl Particle {
    /// Creates a new Particle based on its initial position. Takes in
    /// a slice of `f64`s (`&[f64]`) and converts it to `Vec<f64>`.
    /// The dimensionality is found based on the initial position.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let boson = Particle::new(&[1., 2., 3.])
    /// ```
    pub fn init_position(initial_position: Vec<f64>) -> Particle {
        Particle{
            position: initial_position.to_vec(),
            dimensions: initial_position.len() as u8,
        }
    }

    pub fn new_position(&mut self, position:  Vec<f64>) {
        self.position = position;
    }

    pub fn adjust_position(&mut self, step: f64 , dim: u8) {
        self.position[dim] += step
    }

    pub fn pos_squared(&self) -> f64 {
        self.position.iter().map(|x| x * x). sum()
    }
}
