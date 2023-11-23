use bevy::{prelude::*, window::PrimaryWindow};

use crate::{collision::{HalfSpace, StaticPolygon}, body::{SoftBodyMassPoints, SoftBodySprings}};


pub fn setup_gizmo_config(
    mut config: ResMut<GizmoConfig>
) {
    config.line_width = 5.0;
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

pub fn draw_colliders(
    mut gizmos: Gizmos,
    polygons: Query<&StaticPolygon>,
    halfspaces: Query<&HalfSpace>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    for poly in polygons.iter() {
        for i in 0..poly.vertices.len() {
            let j = (i + 1) % poly.vertices.len();

            let v1 = poly.vertices[i];
            let v2 = poly.vertices[j];

            gizmos.line_2d(v1.xy(), v2.xy(), Color::GRAY);
        }
    }

    for halfspace in halfspaces.iter() {
        let tangent = Vec2::new(halfspace.normal.y, -halfspace.normal.x);
        let p = halfspace.normal * halfspace.k;

        gizmos.ray_2d(p.xy(), tangent * window.width(), Color::GRAY);
        gizmos.ray_2d(p.xy(), -tangent * window.width(), Color::GRAY);
    }
}