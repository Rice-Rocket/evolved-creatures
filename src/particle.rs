use bevy::prelude::*;


#[derive(Component)]
pub struct Particle {
    pub position_old: Vec3,
    pub acceleration: Vec3,
}

pub fn apply_particle_gravity(
    mut particles: Query<&mut Particle>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for mut particle in particles.iter_mut() {
        particle.acceleration += Vec3::new(0.0, -50.0, 0.0) * dt;
    }
}

pub fn integrate_particles(
    mut particles: Query<(&mut Particle, &mut Transform)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for (mut particle, mut transform) in particles.iter_mut() {
        let vel = transform.translation - particle.position_old;
        particle.position_old = transform.translation;
        transform.translation = transform.translation + vel + particle.acceleration * dt * dt;
        particle.acceleration = Vec3::ZERO;
    }
}