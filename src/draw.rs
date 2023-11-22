use bevy::prelude::*;

use crate::body::{SoftBodyMassPoints, SoftBodySprings};


pub fn setup_gizmo_config(
    mut config: ResMut<GizmoConfig>
) {
    config.line_width = 10.0;
}

pub fn draw_particles(
    mut gizmos: Gizmos,
    mass_point_bodies: Query<&SoftBodyMassPoints>,
) {
    for mass_points in mass_point_bodies.iter() {
        for particle in mass_points.0.iter() {
            gizmos.circle_2d(particle.0.position.xy(), 3.0, Color::RED);
        }
    }
}

pub fn draw_springs(
    mut gizmos: Gizmos,
    bodies: Query<(&SoftBodySprings, &SoftBodyMassPoints)>,
) {
    for (springs, mass_points) in bodies.iter() {
        for spring in springs.0.iter() {
            let p1 = &mass_points.0[spring.p1_idx].0;
            let p2 = &mass_points.0[spring.p2_idx].0;
            gizmos.line_2d(p1.position.xy(), p2.position.xy(), Color::ANTIQUE_WHITE);
        }
    }
}