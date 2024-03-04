use rand::{rngs::ThreadRng, Rng};
use rand_distr::Normal;

pub mod node;
pub mod edge;


pub struct MutateFieldParams {
    /// The frequency at which this field is changed
    pub f: f32,
    /// The distribution to sample when chosing a new value
    pub d: Normal<f32>,
}

impl MutateFieldParams {
    pub fn new(freq: f32, mean: f32, std_dev: f32) -> Result<Self, rand_distr::NormalError> {
        Ok(Self {
            f: freq,
            d: Normal::new(mean, std_dev)?,
        })
    }
    pub fn sample(&self, rng: &mut ThreadRng) -> f32 {
        rng.sample(self.d)
    }
    pub fn change(&self, rng: &mut ThreadRng) -> bool {
        rng.gen_bool(self.f as f64)
    }
}
