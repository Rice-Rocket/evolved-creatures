use bevy::prelude::*;

use crate::body::SoftBodyMassPoints;


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
        for particle in mass_points.particles.iter() {
            gizmos.circle_2d(particle.0.position.xy(), 3.0, Color::RED);
        }
    }
}