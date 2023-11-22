use bevy::prelude::*;

use crate::collision::{Collider, ColliderProperties};


#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
pub struct ParticleAccelerateSet;


#[derive(Bundle)]
pub struct Particle {
    pub properties: ParticleProperties,
    pub trajectory: ParticleTrajectory,
}

#[derive(Component)]
pub struct ParticleProperties {
    pub mass: f32,
    pub restitution: f32,
}

#[derive(Component, Default)]
pub struct ParticleTrajectory {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub old_acceleration: Vec3,
}

pub fn apply_collision<T: Component + Collider>(
    colliders: Query<(&T, &ColliderProperties)>,
    mut particles: Query<(&mut Transform, &mut ParticleTrajectory, &ParticleProperties), With<ParticleTrajectory>>,
) {
    for (collider, collider_props) in colliders.iter() {
        for (mut transform, mut particle, props) in particles.iter_mut() {
            match collider.exit_vector(transform.translation) {
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

pub fn apply_particle_gravity(
    mut particles: Query<(&mut ParticleTrajectory, &ParticleProperties)>,
) {
    for (mut particle, props) in particles.iter_mut() {
        particle.acceleration += Vec3::new(0.0, -300.0, 0.0) * props.mass;
    }
}

pub fn update_particle_positions(
    mut particles: Query<(&ParticleTrajectory, &mut Transform, &ParticleProperties)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for (particle, mut transform, props) in particles.iter_mut() {
        transform.translation = transform.translation + particle.velocity * dt + 0.5 * particle.old_acceleration / props.mass * dt * dt;
    }
}

pub fn update_particle_velocities(
    mut particles: Query<(&mut ParticleTrajectory, &ParticleProperties)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for (mut particle, props) in particles.iter_mut() {
        particle.velocity = particle.velocity + 0.5 * (particle.old_acceleration + particle.acceleration) / props.mass * dt;
        particle.old_acceleration = particle.acceleration;
        particle.acceleration = Vec3::ZERO;
    }
}