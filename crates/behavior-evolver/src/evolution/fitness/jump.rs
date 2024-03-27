use super::{EvolutionFitnessEval, FitnessEvalInput};

pub struct JumpFitnessEval {
    max_height: f32,
}


impl EvolutionFitnessEval for JumpFitnessEval {
    fn eval_continuous(&mut self, input: FitnessEvalInput) {
        self.max_height = self.max_height.max(input.position.y);
    }

    fn final_eval(&self) -> f32 {
        self.max_height
    }
}

impl Default for JumpFitnessEval {
    fn default() -> Self {
        Self { max_height: -1.0 }
    }
}
