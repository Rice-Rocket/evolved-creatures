use bevy::prelude::*;

use crate::body::{SoftBodyMassPoints, SoftBodySprings, constrained::{SoftBodyReferenceMassPoints, ConstraintProperties}, resizable::ResizableSoftBodyProperties};


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


pub fn apply_constraint_force(
    mut bodies: Query<(&mut SoftBodyMassPoints, &mut SoftBodyReferenceMassPoints, &ConstraintProperties)>,
) {
    for (mut points, mut ref_points, props) in bodies.iter_mut() {
        let mut avg_points_position = Vec3::ZERO;
        let mut avg_refs_position = Vec3::ZERO;
        for (point, ref_point) in points.0.iter().zip(ref_points.0.iter()) {
            avg_points_position += point.0.position;
            avg_refs_position += ref_point.0;
        }
        avg_points_position /= ref_points.0.len() as f32;
        avg_refs_position /= ref_points.0.len() as f32;
        let avg_position = avg_points_position - avg_refs_position;
        
        let mut avg_rotation = 0f32;
        for (point, ref_point) in points.0.iter().zip(ref_points.0.iter()) {
            let edge_1 = (point.0.position - avg_points_position).normalize();
            let edge_2 = (ref_point.0 - avg_refs_position).normalize();
            let theta = edge_1.dot(edge_2).clamp(-1.0, 1.0).acos();
            avg_rotation += theta;
        }
        avg_rotation /= ref_points.0.len() as f32;
        
        let mut transform = Transform::from_translation(avg_position);
        transform.rotate_around(avg_points_position, Quat::from_euler(EulerRot::ZXY, -avg_rotation, 0.0, 0.0));
    
        for (point, ref_point) in points.0.iter_mut().zip(ref_points.0.iter_mut()) {
            let transformed_ref = transform.transform_point(ref_point.0);
            let ref_vel = ref_point.0 - ref_point.1;

            let pv = transformed_ref - point.0.position;
            let dist = pv.length();
            if dist > -0.001 && dist < 0.001 { continue };

            let pn = pv / dist;
            let vel_dif = ref_vel - point.0.velocity;
            let proj_vel_mag = pn.dot(vel_dif) * props.damping * pn;
            let f = pn * props.stiffness * dist + proj_vel_mag;

            point.0.acceleration += f;
            ref_point.1 = ref_point.0;
        }
    }
}


pub fn resize_springs(
    mut bodies: Query<(&mut SoftBodySprings, &mut ResizableSoftBodyProperties)>,
) {
    for (mut springs, mut props) in bodies.iter_mut() {
        if props.is_quad {
            let area = props.dims.x * props.dims.y;
            let mut dim_ratio = (props.target_volume / area).sqrt();
            dim_ratio = dim_ratio + (1.0 - dim_ratio) * 0.99;

            for spring in springs.0.iter_mut() {
                spring.properties.rest_length *= dim_ratio;
            }

            props.dims *= dim_ratio;
        }
    }
}