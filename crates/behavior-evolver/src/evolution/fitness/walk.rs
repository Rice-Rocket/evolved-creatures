use bevy::math::{Vec2, Vec3, Vec3Swizzles};

use super::{EvolutionFitnessEval, FitnessEvalInput};

pub struct WalkFitnessEval {
    max_height: f32,
    init_pos: Vec2,
    init_length: f32,
}


impl EvolutionFitnessEval for WalkFitnessEval {
    fn eval_start(&mut self, input: FitnessEvalInput) {
        let mut max_y = f32::MIN;
        let mut max_point = Vec3::splat(f32::MIN);
        let mut min_point = Vec3::splat(f32::MAX);
        input.limbs.iter().for_each(|(transform, _)| {
            let c = transform.translation;
            let x = transform.local_x() * transform.scale.x;
            let y = transform.local_y() * transform.scale.y;
            let z = transform.local_z() * transform.scale.z;

            let min_p = (c + x + y + z)
                .min(c + x + y - z)
                .min(c + x - y + z)
                .min(c - x + y + z)
                .min(c + x - y - z)
                .min(c - x + y - z)
                .min(c - x - y + z)
                .min(c - x - y - z);
            let max_p = (c + x + y + z)
                .max(c + x + y - z)
                .max(c + x - y + z)
                .max(c - x + y + z)
                .max(c + x - y - z)
                .max(c - x + y - z)
                .max(c - x - y + z)
                .max(c - x - y - z);

            max_point = max_point.max(max_p);
            min_point = min_point.min(min_p);
            max_y = max_y.max(max_p.y);
        });

        self.max_height = max_y;
        self.init_length = (max_point - min_point).xz().length();

        let (mut total_pos, mut count) = (Vec2::ZERO, 0.0);
        input.limbs.iter().for_each(|(transform, _)| {
            let volume = transform.scale.x * transform.scale.y * transform.scale.z;
            count += volume;
            total_pos += transform.translation.xz() * volume;
        });
        self.init_pos = total_pos / if count != 0.0 { count } else { 1.0 };
    }

    fn eval_continuous(&mut self, _input: FitnessEvalInput) {
        // let (mut total_pos, mut count) = (Vec2::ZERO, 0.0);
        // input.limbs.iter().for_each(|(transform, _)| {
        //     let volume = transform.scale.x * transform.scale.y *
        // transform.scale.z;     count += volume;
        //     total_pos += transform.translation.xz() * volume;
        // });
        // let new_pos = total_pos / if count != 0.0 { count } else { 1.0 };
    }

    fn final_eval(&self, input: FitnessEvalInput) -> f32 {
        let mut max_point = Vec3::splat(f32::MIN);
        let mut min_point = Vec3::splat(f32::MAX);
        input.limbs.iter().for_each(|(transform, _)| {
            let c = transform.translation;
            let x = transform.local_x() * transform.scale.x;
            let y = transform.local_y() * transform.scale.y;
            let z = transform.local_z() * transform.scale.z;

            let min_p = (c + x + y + z)
                .min(c + x + y - z)
                .min(c + x - y + z)
                .min(c - x + y + z)
                .min(c + x - y - z)
                .min(c - x + y - z)
                .min(c - x - y + z)
                .min(c - x - y - z);
            let max_p = (c + x + y + z)
                .max(c + x + y - z)
                .max(c + x - y + z)
                .max(c - x + y + z)
                .max(c + x - y - z)
                .max(c - x + y - z)
                .max(c - x - y + z)
                .max(c - x - y - z);

            max_point = max_point.max(max_p);
            min_point = min_point.min(min_p);
        });
        let end_length = (max_point - min_point).xz().length();
        let length_diff = (end_length - self.init_length).abs();

        let (mut total_pos, mut count) = (Vec2::ZERO, 0.0);
        input.limbs.iter().for_each(|(transform, _)| {
            let volume = transform.scale.x * transform.scale.y * transform.scale.z;
            count += volume;
            total_pos += transform.translation.xz() * volume;
        });
        let end_pos = total_pos / if count != 0.0 { count } else { 1.0 };

        let res = (end_pos - self.init_pos).length()
            - if self.max_height > 3.0 { self.max_height * self.max_height * 2.0 } else { 0.0 }
            - if length_diff > 1.0 { length_diff * length_diff } else { 0.0 };
        if res.is_finite() {
            res
        } else {
            -1000000000000.0
        }
    }
}

impl Default for WalkFitnessEval {
    fn default() -> Self {
        Self { max_height: -1.0, init_length: 0.0, init_pos: Vec2::ZERO }
    }
}
