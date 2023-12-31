use bevy::prelude::*;

use crate::prelude::{RigidBodyState, RigidBodyProperties};

use super::{RBJointType, RBJointProperties};




pub(crate) fn apply_joint_connection_force<T: RBJointType + Component>(
    mut gizmos: Gizmos,
    joints: Query<(&T, &RBJointProperties), Without<RigidBodyState>>,
    mut bodies: Query<(&mut RigidBodyState, &RigidBodyProperties), Without<T>>,
) {
    for (joint_type, joint) in joints.iter() {
        for (pos_1, pos_2) in joint_type.connection_points(joint) {
            let (f, f1);
            let (locked_1, locked_2);
            let (p, p1);
            {
                let (state_1, props_1) = bodies.get(joint.body_1).unwrap();
                let (state_2, props_2) = bodies.get(joint.body_2).unwrap();
    
                p = state_1.globalize(pos_1 * props_1.scale * 0.5);
                p1 = state_2.globalize(pos_2 * props_2.scale * 0.5);
    
                locked_1 = props_1.locked;
                locked_2 = props_2.locked;
    
                let d = p1 - p;
                let dist = d.length();

                if dist == 0.0 { continue };

                let kt = joint.friction * 4.0;
                let kn = (1.0 - joint.damping).powi(10);
                let kc = joint.stiffness * 1000.0;

                let v = state_1.velocity_at_point(p);
                let vvel = state_2.velocity_at_point(p1);
                let vdiff = v - vvel;
                let n = d / dist;
                let vn = vdiff.dot(n) * n;
                let vt = (vn * n - vdiff) * kt;
                let b = (kc * props_1.mass).sqrt() * 2.0 * kn;
                f = (d * kc - b * vn + vt) * props_1.mass;

                let vn1 = (-vdiff).dot(-n) * -n;
                let vt1 = (vn1 * -n + vdiff) * kt;
                let b1 = (kc * props_2.mass).sqrt() * 2.0 * kn;
                f1 = (-d * kc - b1 * vn1 + vt1) * props_2.mass;
            }
    
            gizmos.line(p, p1, Color::GREEN);
    
            if !locked_1 {
                let (mut state_1, _) = bodies.get_mut(joint.body_1).unwrap();
                state_1.apply_force(p, f);
                state_1.apply_force(p, -f1);
            }
            
            if !locked_2 {
                let (mut state_2, _) = bodies.get_mut(joint.body_2).unwrap();
                state_2.apply_force(p1, -f);
                state_2.apply_force(p1, f1);
            }
        }
    }
}


pub(crate) fn apply_joint_limit_force_bend<T: RBJointType + Component>(
    joints: Query<(&T, &RBJointProperties), Without<RigidBodyState>>,
    mut bodies: Query<(&mut RigidBodyState, &RigidBodyProperties), Without<T>>,
) {
    for (joint_type, joint) in joints.iter() {
        let (f1, f2);
        let (locked_1, locked_2);
        let (p, reflected_p);
        {
            let (state_1, props_1) = bodies.get(joint.body_1).unwrap();
            let (state_2, props_2) = bodies.get(joint.body_2).unwrap();

            locked_1 = props_1.locked;
            locked_2 = props_2.locked;

            let p1 = state_1.globalize(joint.position_1 * props_1.scale * 0.5);
            let p2 = state_2.globalize(joint.position_2 * props_2.scale * 0.5);
            p = 0.5 * p1 + 0.5 * p2;

            let to_rb1 = state_1.position - p;
            let to_rb2 = state_2.position - p;

            let to_rb1_dist = to_rb1.length();
            let to_rb2_dist = to_rb2.length();

            let rb1_to_rb2 = (state_1.position - state_2.position).normalize();
            let on_far_edge = state_1.position + (p - state_1.position).dot(rb1_to_rb2) * rb1_to_rb2;
            let height = (p - on_far_edge).length();

            let cos_max = (std::f32::consts::FRAC_PI_2 - joint.joint_limits.x.min(joint_type.locked_limits().x) * 0.5 * std::f32::consts::PI).cos();
            let max_ref_height = 0.5 * cos_max * to_rb1_dist + 0.5 * cos_max * to_rb2_dist;

            if max_ref_height > height {
                continue
            }

            reflected_p = on_far_edge;

            // gizmos.line(state_1.position, state_2.position, Color::PURPLE);
            // gizmos.line(state_1.position, p, Color::TEAL);
            // gizmos.line(state_2.position, p, Color::TEAL);

            let d = reflected_p - p;
            let dist2 = d.length_squared();

            if dist2 == 0.0 { continue };

            let dist = dist2.sqrt();
            let dnorm = dist - max_ref_height;

            let kt = joint.limit_friction * 4.0;
            let kn = (1.0 - joint.limit_damping).powi(10);
            let kc = joint.limit_stiffness * 1000.0;

            let v = state_1.velocity_at_point(p);
            let vvel = state_1.velocity_at_point(reflected_p);
            let vdiff = v - vvel;
            let n = d / dist;
            let vn = vdiff.dot(n) * n;
            let vt = (vn * n - vdiff) * kt;
            let b = (kc * props_1.mass).sqrt() * 2.0 * kn;
            f1 = (n * dnorm * kc - b * vn + vt) * props_1.mass;

            let v = state_2.velocity_at_point(p);
            let vvel = state_2.velocity_at_point(reflected_p);
            let vdiff = v - vvel;
            let vn = vdiff.dot(n) * n;
            let vt = (vn * n - vdiff) * kt;
            let b = (kc * props_2.mass).sqrt() * 2.0 * kn;
            f2 = (n * dnorm * kc - b * vn + vt) * props_2.mass;
        }

        if !locked_1 {
            let (mut state_1, _) = bodies.get_mut(joint.body_1).unwrap();
            let arm = p - state_1.position;
            state_1.torque += arm.cross(f1);
        }
        
        if !locked_2 {
            let (mut state_2, _) = bodies.get_mut(joint.body_2).unwrap();
            let arm = p - state_2.position;
            state_2.torque += arm.cross(f2);
        }
    }
}

pub(crate) fn apply_joint_limit_force_twist<T: RBJointType + Component> (
    mut gizmos: Gizmos,
    joints: Query<(&T, &RBJointProperties), Without<RigidBodyState>>,
    mut bodies: Query<(&mut RigidBodyState, &RigidBodyProperties), Without<T>>,
) {
    for (joint_type, joint) in joints.iter() {
        let (t1, t2);
        let (locked_1, locked_2);
        let (center, tan1, tan2);
        {
            let (state_1, props_1) = bodies.get(joint.body_1).unwrap();
            let (state_2, props_2) = bodies.get(joint.body_2).unwrap();

            locked_1 = props_1.locked;
            locked_2 = props_2.locked;

            let p1 = state_1.globalize(joint.position_1 * props_1.scale * 0.5);
            let p2 = state_2.globalize(joint.position_2 * props_2.scale * 0.5);
            tan1 = state_1.globalize_bivec(Vec3::X);
            tan2 = state_2.globalize_bivec(Vec3::X);
            center = 0.5 * p1 + 0.5 * p2;

            let plane_norm = (center - state_2.position).normalize();
            let tan1_proj = (tan1 - tan1.dot(plane_norm) * plane_norm).normalize();

            let norm_y_rot = Quat::from_rotation_arc(plane_norm, Vec3::Y);
            let norm_y_tan1 = norm_y_rot * tan1_proj;
            let norm_y_tan2 = norm_y_rot * tan2;

            let norm_x_rot = Quat::from_rotation_arc(norm_y_tan1, Vec3::X);
            let norm_x_tan2 = norm_x_rot * norm_y_tan2;

            let angle_side = if norm_x_tan2.z > 0.0 { 1.0 } else { -1.0 };

            let torque_axis_1 = (state_1.position - center).normalize();
            let torque_axis_2 = (state_2.position - center).normalize();
            
            let cos_t = norm_y_tan1.dot(norm_y_tan2);
            let theta = cos_t.clamp(-1.0, 1.0).acos();
            let max_theta = joint.joint_limits.y.min(joint_type.locked_limits().y) * std::f32::consts::PI;
            
            if max_theta >= theta { continue };
            
            let d = (max_theta - theta) * angle_side;

            // let kt = joint.limit_friction * 4.0;
            let kn = (1.0 - joint.limit_damping).powi(10);
            let kc = joint.limit_stiffness * 1000.0;

            let v = torque_axis_1.dot(state_1.angular_velocity);
            let vvel = torque_axis_1.dot(state_2.angular_velocity);
            let vdiff = v - vvel;
            let b = (kc * props_1.mass).sqrt() * 2.0 * kn;
            let t1_mag = (d * kc - b * vdiff) * props_1.mass;

            let v = torque_axis_2.dot(state_2.angular_velocity);
            let vvel = torque_axis_2.dot(state_1.angular_velocity);
            let vdiff = v - vvel;
            let b = (kc * props_2.mass).sqrt() * 2.0 * kn;
            let t2_mag = (d * kc - b * vdiff) * props_2.mass;

            gizmos.ray(center, torque_axis_1 * t1_mag, Color::BLUE);

            t1 = torque_axis_1 * t1_mag;
            t2 = torque_axis_2 * t2_mag;
        }

        if !locked_1 {
            let (mut state_1, _) = bodies.get_mut(joint.body_1).unwrap();
            state_1.torque += t1;
            // state_1.torque -= t2;
        }
        
        if !locked_2 {
            let (mut state_2, _) = bodies.get_mut(joint.body_2).unwrap();
            state_2.torque += t2;
            // state_2.torque -= t1;
        }
    }
}