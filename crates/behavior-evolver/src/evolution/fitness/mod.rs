pub mod jump;

use bevy::math::Vec3;


pub struct FitnessEvalInput {
    pub position: Vec3,
    pub velocity: Vec3,
}


pub trait EvolutionFitnessEval {
    fn eval_continuous(&mut self, input: FitnessEvalInput);
    fn final_eval(&self) -> f32;
}
