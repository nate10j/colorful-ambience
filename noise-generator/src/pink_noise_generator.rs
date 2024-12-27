const PINK_MAX_RANDOM_ROWS: u32 = 30;
const PINK_RANDOM_BITS: u32 = 24;
const PINK_RANDOM_SHIFT: u32 = (std::mem::size_of::<u32>() * 8) as u32 - PINK_RANDOM_BITS;

pub struct PinkNoiseGenerator {
    rows: Vec<f32>,
    running_sum: f32,
    index: usize,
    index_mask: usize,
    scalar: f32,
}

impl PinkNoiseGenerator {
    pub fn new(num_rows: usize) -> Self {
        let index_mask = (1 << num_rows) - 1;
        let pmax = (num_rows + 1) * (1 << (PINK_RANDOM_BITS - 1));
        let rows = vec![0.0; num_rows];
        Self {
            rows,
            running_sum: 0.0,
            index: 0,
            index_mask,
            scalar: 1.0 / pmax as f32,
        }
    }

    pub fn sample(&mut self) -> f32 {
        self.index = (self.index + 1) & self.index_mask;

        if self.index != 0 {
            let mut num_zeros = 0;
            let mut n = self.index;
            while (n & 1) == 0 {
                n >>= 1;
                num_zeros += 1;
            }

            self.running_sum -= self.rows[num_zeros];
            let new_random = (fastrand::u32(..) >> PINK_RANDOM_SHIFT) as f32;
            self.running_sum += new_random;
            self.rows[num_zeros] = new_random;
        }

        let new_random = (fastrand::u32(..) >> PINK_RANDOM_SHIFT) as f32;
        let sum = self.running_sum + new_random;
        self.scalar * sum - 1.0
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self, Write};
    use crate::PinkNoiseGenerator;

    #[test]
    fn pink_noise_samples() -> io::Result<()> {
        let mut file = File::create("pink_noise_samples.txt")?;
        let mut pink_noise_generator = PinkNoiseGenerator::new(16);
        for _ in 0..5000 {
            writeln!(file, "{}", pink_noise_generator.sample())?; 
        }

        Ok(())
    }
}
