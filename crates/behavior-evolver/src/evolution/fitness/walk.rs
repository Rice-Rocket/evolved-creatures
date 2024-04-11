use bevy::math::Vec3Swizzles;

use super::{EvolutionFitnessEval, FitnessEvalInput};

pub struct WalkFitnessEval {
    avg_vel: f32,
}


impl EvolutionFitnessEval for WalkFitnessEval {
    fn eval_continuous(&mut self, input: FitnessEvalInput) {
        let (mut total_vel, mut count) = (0.0, 0.0);
        input.limbs.iter().for_each(|(transform, vel)| {
            let volume = transform.scale.x * transform.scale.y * transform.scale.z;
            count += volume;
            total_vel += vel.linvel.xz().length() * volume;
        });
        self.avg_vel += total_vel / count / input.test_time as f32;
    }

    fn final_eval(&self) -> f32 {
        self.avg_vel
    }
}

impl Default for WalkFitnessEval {
    fn default() -> Self {
        Self { avg_vel: -1.0 }
    }
}
