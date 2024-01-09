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
    // mut gizmos: Gizmos,
    joints: Query<(&T, &RBJointProperties), Without<RigidBodyState>>,
    mut bodies: Query<(&mut RigidBodyState, &RigidBodyProperties), Without<T>>,
) {
    for (joint_type, joint) in joints.iter() {
        for (rot_axis, rot_axis_index) in [(Vec3::X, 0), (Vec3::Z, 1)] {
            let (t1, t2);
            let (locked_1, locked_2);
            {
                let (state_1, props_1) = bodies.get(joint.body_1).unwrap();
                let (state_2, props_2) = bodies.get(joint.body_2).unwrap();
    
                locked_1 = props_1.locked;
                locked_2 = props_2.locked;
    
                let n1 = state_1.globalize_bivec(Vec3::Y);
                let n2 = state_2.globalize_bivec(Vec3::Y);
    
                let plane_norm = state_1.globalize_bivec(rot_axis);
                let n2_proj = (n2 - n2.dot(plane_norm) * plane_norm).normalize();
    
                let norm_y_rot = Quat::from_rotation_arc(plane_norm, Vec3::Y);
                let norm_y_n1 = norm_y_rot * n1;
                let norm_y_n2 = norm_y_rot * n2_proj;
    
                let norm_x_rot = Quat::from_rotation_arc(norm_y_n1, Vec3::X);
                let norm_x_n2 = norm_x_rot * norm_y_n2;
    
                let angle_side = if norm_x_n2.z > 0.0 { 1.0 } else { -1.0 };
    
                let torque_axis_1 = state_1.globalize_bivec(rot_axis);
                let torque_axis_2 = state_2.globalize_bivec(-rot_axis);
                
                let cos_t = norm_y_n1.dot(norm_y_n2);
                let theta = cos_t.clamp(-1.0, 1.0).acos();
                let max_theta = joint.joint_limits[rot_axis_index].min(joint_type.locked_limits()[rot_axis_index]) * std::f32::consts::PI;
                
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
    
                // gizmos.ray(state_1.position, torque_axis_1, Color::BLUE);
                // gizmos.ray(state_2.position, torque_axis_2, Color::BLUE);
                // gizmos.ray(joint.position_1, plane_norm, Color::PURPLE);
    
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
}

pub(crate) fn apply_joint_limit_force_twist<T: RBJointType + Component> (
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
            let max_theta = joint.joint_limits.z.min(joint_type.locked_limits().z) * std::f32::consts::PI;
            
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