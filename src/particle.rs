use bevy::prelude::*;

use crate::collision::Collider;


#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
pub struct ParticleAccelerateSet;


#[derive(Component)]
pub struct Particle {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub old_acceleration: Vec3,
}

pub fn apply_collision<T: Component + Collider>(
    colliders: Query<&T>,
    mut particles: Query<&mut Transform, With<Particle>>,
) {
    for collider in colliders.iter() {
        for mut transform in particles.iter_mut() {
            match collider.on_surface(transform.translation) {
                Some(pos) => { transform.translation = pos },
                None => (),
            }
        }
    }
}

pub fn apply_particle_gravity(
    mut particles: Query<&mut Particle>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for mut particle in particles.iter_mut() {
        particle.acceleration += Vec3::new(0.0, -100.0, 0.0) * dt;
    }
}

pub fn update_particle_positions(
    mut particles: Query<(&Particle, &mut Transform)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for (particle, mut transform) in particles.iter_mut() {
        transform.translation = transform.translation + particle.velocity * dt + 0.5 * particle.acceleration * dt * dt;
    }
}

pub fn update_particle_velocities(
    mut particles: Query<&mut Particle>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for mut particle in particles.iter_mut() {
        particle.velocity = particle.velocity + 0.5 * (particle.old_acceleration + particle.acceleration) * dt;
        particle.old_acceleration = particle.acceleration;
    }
}