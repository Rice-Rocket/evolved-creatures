use bevy::prelude::*;

use crate::body::{SoftBodyMassPoints, SoftBodySprings};


#[derive(Reflect, Debug, Default, Clone)]
#[reflect(Debug, Default)]
pub struct Spring {
    pub p1_idx: usize,
    pub p2_idx: usize,
    pub properties: SpringProperties,
}

#[derive(Reflect, Debug, Default, Clone, Component)]
#[reflect(Debug, Default)]
pub struct SpringProperties {
    pub stiffness: f32,
    pub rest_length: f32,
    pub damping: f32,
}


pub fn apply_spring_force(
    mut bodies: Query<(&mut SoftBodyMassPoints, &SoftBodySprings)>
) {
    for (mut particles, springs) in bodies.iter_mut() {
        for spring in springs.0.iter() {
            let p1 = &particles.0[spring.p1_idx].0;
            let p2 = &particles.0[spring.p2_idx].0;

            let pv = p2.position - p1.position;
            let dist = pv.length();
            if dist > -0.001 && dist < 0.001 { continue };

            let pn = pv / dist;
            let dx = spring.properties.rest_length - dist;

            let vel_diff = p2.velocity - p1.velocity;
            let proj_vel_mag = pn.dot(vel_diff) * spring.properties.damping * pn;
            let f = -pn * spring.properties.stiffness * dx + proj_vel_mag;

            particles.0[spring.p1_idx].0.acceleration += f;
            particles.0[spring.p2_idx].0.acceleration -= f;
        }
    }
}