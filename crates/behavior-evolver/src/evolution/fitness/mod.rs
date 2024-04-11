pub mod jump;
pub mod walk;

use bevy::transform::components::Transform;
use bevy_rapier3d::dynamics::Velocity;


pub struct FitnessEvalInput {
    pub limbs: Vec<(Transform, Velocity)>,
    pub test_time: usize,
}


pub trait EvolutionFitnessEval {
    fn eval_continuous(&mut self, input: FitnessEvalInput);
    fn final_eval(&self) -> f32;
}
