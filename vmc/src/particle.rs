pub struct Particle {
    pub position: Vec<f64>,
    dimensions: u8,
}

impl Particle {
    fn new(initial_position: &[f64]) -> Particle {
        Particle{
            position: initial_position.to_vec(),
            dimensions: initial_position.len() as u8,
        }
    }
}