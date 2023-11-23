use bevy::prelude::*;

use crate::{collision::{Collider, ColliderProperties}, sim::PhysicsSimulationSettings, body::SoftBodyMassPoints};


#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
pub struct ParticleAccelerateSet;


#[derive(Reflect, Debug, Clone)]
#[reflect(Debug, Default)]
pub struct ParticleProperties {
    pub mass: f32,
    pub restitution: f32,
}

impl Default for ParticleProperties {
    fn default() -> Self {
        Self {
            mass: 1.0,
            restitution: 1.0,
        }
    }
}

#[derive(Default, Reflect, Debug, Clone)]
#[reflect(Debug, Default)]
pub struct ParticleTrajectory {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub old_acceleration: Vec3,
}

pub fn apply_collision<T: Component + Collider>(
    colliders: Query<(&T, &ColliderProperties)>,
    mut bodies: Query<&mut SoftBodyMassPoints>,
) {
    for (collider, collider_props) in colliders.iter() {
        for mut particles in bodies.iter_mut() {
            for (particle, props) in particles.0.iter_mut() {
                match collider.exit_vector(particle.position) {
                    Some(exit_vec) => {
                        let dir = exit_vec.normalize();
                        let normal_vel = particle.velocity.dot(dir);
                        let tangent_vel = particle.velocity - normal_vel * dir;
                        particle.acceleration += exit_vec * collider_props.elasticity;
                        particle.acceleration += -normal_vel * dir * collider_props.restitution;
                        particle.acceleration += -tangent_vel * collider_props.friction;
                    },
                    None => (),
                }
            }
        }
    }
}

pub fn apply_particle_gravity(
    mut bodies: Query<&mut SoftBodyMassPoints>,
) {
    for mut particles in bodies.iter_mut() {
        for (particle, props) in particles.0.iter_mut() {
            particle.acceleration += Vec3::new(0.0, -300.0, 0.0) * props.mass;
        }
    }
}

pub fn update_particle_positions(
    mut bodies: Query<&mut SoftBodyMassPoints>,
    settings: Res<PhysicsSimulationSettings>,
) {
    let dt = settings.sub_dt;
    for mut particles in bodies.iter_mut() {
        for (particle, props) in particles.0.iter_mut() {
            particle.position = particle.position + particle.velocity * dt + 0.5 * particle.old_acceleration / props.mass * dt * dt;
        }
    }
}

pub fn update_particle_velocities(
    mut bodies: Query<&mut SoftBodyMassPoints>,
    settings: Res<PhysicsSimulationSettings>,
) {
    let dt = settings.sub_dt;
    for mut particles in bodies.iter_mut() {
        for (particle, props) in particles.0.iter_mut() {
            particle.velocity = particle.velocity + 0.5 * (particle.old_acceleration + particle.acceleration) / props.mass * dt;
            particle.old_acceleration = particle.acceleration;
            particle.acceleration = Vec3::ZERO;
        }
    }
}