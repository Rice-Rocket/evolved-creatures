use bevy::prelude::*;

pub mod prelude;
pub mod body;
pub mod sim;
pub mod integrate;
pub mod force;
pub mod joint;

use body::*;
use sim::*;
use integrate::*;
use force::*;
use joint::{*, constraint::*, spherical::*, rigid::*, revolute::*};

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

                apply_joint_connection_force::<RBSphericalJoint>,
                apply_joint_connection_force::<RBRigidJoint>,
                apply_joint_connection_force::<RBRevoluteJoint>,

                apply_joint_limit_force_bend::<RBSphericalJoint>,
                apply_joint_limit_force_bend::<RBRigidJoint>,
                apply_joint_limit_force_bend::<RBRevoluteJoint>,
            ).in_set(ApplyForcesSet).after(update_positions))
            .add_systems(Update, update_object_transform.after(run_physics_sim_schedule))


            .register_type::<RBSphericalJoint>()
            .register_type::<RBRigidJoint>()
            .register_type::<RBRevoluteJoint>()
            .register_type::<RBJointProperties>()
            .register_type::<RigidBodySimulationSettings>()
            .register_type::<RigidBodyImpulseAccumulator>()
            .register_type::<RigidBodyProperties>()
            .register_type::<RigidBodyState>();
    }
}