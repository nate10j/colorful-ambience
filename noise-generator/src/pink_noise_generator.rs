pub struct PinkNoiseGenerator {
    rows: Vec<f32>
}

impl PinkNoiseGenerator {
    pub fn new(octave: usize) -> Self {
        Self { rows: vec![0.0, octave as f32] }
    }

    pub fn sample(&mut self) -> f32 {
        let num_rows = self.rows.len();

        let random = fastrand::f32() * 2.0 - 1.0;
        let k = fastrand::usize(..num_rows);
        self.rows[k] = random;
        
        let sum: f32 = self.rows.iter().sum();
        sum / num_rows as f32
    }
}
