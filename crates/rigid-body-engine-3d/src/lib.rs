use bevy::prelude::*;

pub mod prelude;
pub mod body;
pub mod sim;
pub mod integrate;
pub mod force;

use body::*;
use sim::*;
use integrate::*;
use force::*;

pub struct RigidBodySimulationPlugin;

impl Plugin for RigidBodySimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RigidBodySimulationSettings>()
            .add_schedule(Schedule::new(RigidBodySimulationSchedule))
        
            .add_systems(Update, run_physics_sim_schedule)
            .add_systems(RigidBodySimulationSchedule, (
                update_positions.before(ApplyForcesSet),
                update_velocities.after(ApplyForcesSet),
            ))
            .add_systems(RigidBodySimulationSchedule, (
                apply_gravity,
                apply_collisions,
                apply_accumulated_impulses,
            ).in_set(ApplyForcesSet).after(update_positions))
            .add_systems(Update, update_object_transform.after(run_physics_sim_schedule))


            .register_type::<RigidBodyProperties>()
            .register_type::<RigidBodyState>();
    }
}