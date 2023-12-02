use bevy::prelude::*;

pub mod body;
pub mod collider;
pub mod draw;
pub mod particle;
pub mod sim;
pub mod spring;
pub mod prelude;

use body::{*, constrained::*, resizable::*};
use collider::*;
use draw::*;
use particle::*;
use sim::*;
use spring::*;

pub struct SoftBodySimulationPlugin;

impl Plugin for SoftBodySimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SoftBodySimulationSettings>()
            .add_schedule(Schedule::new(SoftBodySimulationSchedule))
            
            .add_systems(Update, run_physics_sim_schedule)
            .add_systems(SoftBodySimulationSchedule, (
                update_particle_positions.before(ParticleAccelerateSet),
                update_particle_velocities.after(ParticleAccelerateSet),
            ))
            .add_systems(SoftBodySimulationSchedule, (
                apply_particle_gravity,
                apply_spring_force,
                apply_constraint_force,
                resize_springs,
                apply_collision::<HalfSpace>,
                apply_collision::<StaticPolygon>,
            ).in_set(ParticleAccelerateSet).after(update_particle_positions))


            .register_type::<ColliderProperties>()
            .register_type::<HalfSpace>()
            .register_type::<StaticPolygon>()

            .register_type::<ParticleProperties>()
            .register_type::<ParticleTrajectory>()

            .register_type::<SoftBodySimulationSettings>()
            .register_type::<Spring>()
            .register_type::<SpringProperties>()

            .register_type::<SoftBodyMassPoints>()
            .register_type::<SoftBodySprings>()
            .register_type::<SoftBodyReferenceMassPoints>()
            .register_type::<ConstraintProperties>()
            .register_type::<ResizableSoftBodyProperties>();
    }
}


pub struct SoftBodyDrawPlugin;

impl Plugin for SoftBodyDrawPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_gizmo_config)
            .add_systems(Update, (
                draw_particles, 
                draw_springs, 
                draw_colliders
            ));
    }
}