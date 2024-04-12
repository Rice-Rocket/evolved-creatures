use super::{EvolutionFitnessEval, FitnessEvalInput};

pub struct JumpFitnessEval {
    max_height: f32,
}


impl EvolutionFitnessEval for JumpFitnessEval {
    fn eval_start(&mut self, _input: FitnessEvalInput) {}

    fn eval_continuous(&mut self, input: FitnessEvalInput) {
        let mut mini = f32::MAX;
        input.limbs.iter().for_each(|(transform, _)| {
            let c = transform.translation;
            let x = transform.local_x() * transform.scale.x;
            let y = transform.local_y() * transform.scale.y;
            let z = transform.local_z() * transform.scale.z;
            let min_y = (c + x + y + z)
                .y
                .min((c + x + y - z).y)
                .min((c + x - y + z).y)
                .min((c - x + y + z).y)
                .min((c + x - y - z).y)
                .min((c - x + y - z).y)
                .min((c - x - y + z).y)
                .min((c - x - y - z).y);
            mini = mini.min(min_y);
        });
        if mini == f32::MAX {
            mini = f32::MIN;
        }
        self.max_height = self.max_height.max(mini);
    }

    fn final_eval(&self, _input: FitnessEvalInput) -> f32 {
        self.max_height
    }
}

impl Default for JumpFitnessEval {
    fn default() -> Self {
        Self { max_height: -1.0 }
    }
}
