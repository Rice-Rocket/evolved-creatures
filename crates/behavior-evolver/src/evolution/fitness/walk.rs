use bevy::math::{Vec2, Vec3Swizzles};

use super::{EvolutionFitnessEval, FitnessEvalInput};

pub struct WalkFitnessEval {
    max_height: f32,
    avg_vel: Vec2,
    prev_pos: Vec2,
}


impl EvolutionFitnessEval for WalkFitnessEval {
    fn eval_start(&mut self, input: FitnessEvalInput) {
        let mut maxi = f32::MIN;
        input.limbs.iter().for_each(|(transform, _)| {
            let c = transform.translation;
            let x = transform.local_x() * transform.scale.x;
            let y = transform.local_y() * transform.scale.y;
            let z = transform.local_z() * transform.scale.z;
            let max_y = (c + x + y + z)
                .y
                .max((c + x + y - z).y)
                .max((c + x - y + z).y)
                .max((c - x + y + z).y)
                .max((c + x - y - z).y)
                .max((c - x + y - z).y)
                .max((c - x - y + z).y)
                .max((c - x - y - z).y);
            maxi = maxi.max(max_y);
        });

        self.max_height = self.max_height.max(maxi);

        let (mut total_pos, mut count) = (Vec2::ZERO, 0.0);
        input.limbs.iter().for_each(|(transform, _)| {
            let volume = transform.scale.x * transform.scale.y * transform.scale.z;
            count += volume;
            total_pos += transform.translation.xz() * volume;
        });
        self.prev_pos = total_pos / if count != 0.0 { count } else { 1.0 };
    }

    fn eval_continuous(&mut self, input: FitnessEvalInput) {
        let (mut total_pos, mut count) = (Vec2::ZERO, 0.0);
        input.limbs.iter().for_each(|(transform, _)| {
            let volume = transform.scale.x * transform.scale.y * transform.scale.z;
            count += volume;
            total_pos += transform.translation.xz() * volume;
        });
        let new_pos = total_pos / if count != 0.0 { count } else { 1.0 };

        let vel = new_pos - self.prev_pos;
        self.prev_pos = new_pos;

        self.avg_vel += vel / input.test_time as f32;
    }

    fn final_eval(&self, _input: FitnessEvalInput) -> f32 {
        let res = self.avg_vel.length() - if self.max_height > 2.0 { self.max_height * self.max_height * 1.5 } else { 0.0 };
        if res.is_finite() {
            res
        } else {
            -1000000000000.0
        }
    }
}

impl Default for WalkFitnessEval {
    fn default() -> Self {
        Self { max_height: -1.0, avg_vel: Vec2::ZERO, prev_pos: Vec2::ZERO }
    }
}
