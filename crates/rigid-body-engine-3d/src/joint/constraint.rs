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


// ? Mess with this to maybe increase stability
const JOINT_LIMIT_DIST: f32 = 1.0;
const JOINT_LIMIT_DIST_2_SQR: f32 = 2.0 * JOINT_LIMIT_DIST * JOINT_LIMIT_DIST;

pub(crate) fn apply_joint_limit_force(
    mut gizmos: Gizmos,
    joints: Query<&RBJointProperties, Without<RigidBodyState>>,
    mut bodies: Query<(&mut RigidBodyState, &RigidBodyProperties)>,
) {
    for joint in joints.iter() {
        let (mut f1_z, mut f2_z) = (None, None);
        let (p1_z, p2_z);
        let (mut f1_x, mut f2_x) = (None, None);
        let (p1_x, p2_x);
        let (locked_1, locked_2);
        {
            let (state_1, props_1) = bodies.get(joint.body_1).unwrap();
            let (state_2, props_2) = bodies.get(joint.body_2).unwrap();

            locked_1 = props_1.locked;
            locked_2 = props_2.locked;

            let ref_d2_x = JOINT_LIMIT_DIST_2_SQR - JOINT_LIMIT_DIST_2_SQR * joint.joint_limits.x.cos();
            let ref_d2_z = JOINT_LIMIT_DIST_2_SQR - JOINT_LIMIT_DIST_2_SQR * joint.joint_limits.z.cos();

            let tan1 = state_1.globalize_bivec(joint.tangent);
            let tan2 = state_2.globalize_bivec(joint.tangent);

            let bitan1 = state_1.globalize_bivec(joint.bitangent);
            let bitan2 = state_2.globalize_bivec(joint.bitangent);

            // let pos = joint.position_1 * 0.5 + joint.position_2 * 0.5;
            let p1 = state_1.globalize(joint.position_1 * props_1.scale * 0.5);
            let p2 = state_2.globalize(joint.position_2 * props_2.scale * 0.5);
            p1_x = tan1 * JOINT_LIMIT_DIST + p1;
            p2_x = tan2 * JOINT_LIMIT_DIST + p2;
            p1_z = bitan1 * JOINT_LIMIT_DIST + p1;
            p2_z = bitan2 * JOINT_LIMIT_DIST + p2;

            let d2_x = p1_x - p2_x;
            let d2_z = p1_z - p2_z;

            for (i, d2, ref_d2, p1, p2) in [
                (0, d2_x, ref_d2_x, p1_x, p2_x),
                (1, d2_z, ref_d2_z, p1_z, p2_z),
            ] {
                if d2.length_squared() > ref_d2 {
                    let ref_d = ref_d2.sqrt();
    
                    let dist = d2.length();
                    let dnorm = ref_d - dist;
    
                    if dist == 0.0 { continue };
                    
                    gizmos.line(p1, p2, Color::BLUE);
    
                    let kt = joint.limit_friction * 4.0;
                    let kn = (1.0 - joint.limit_damping).powi(10);
                    let kc = joint.limit_stiffness * 1000.0;
    
                    let v = state_1.velocity_at_point(p1);
                    let vvel = state_2.velocity_at_point(p2);
                    let vdiff = v - vvel;
                    let n = d2 / dist;
                    let vn = vdiff.dot(n) * n;
                    let vt = (vn * n - vdiff) * kt;
                    let b = (kc * props_1.mass).sqrt() * 2.0 * kn;
                    let f1 = Some((n * dnorm * kc - b * vn + vt) * props_1.mass);
    
                    let vn1 = (-vdiff).dot(-n) * -n;
                    let vt1 = (vn1 * -n + vdiff) * kt;
                    let b1 = (kc * props_2.mass).sqrt() * 2.0 * kn;
                    let f2 = Some((-n * dnorm * kc - b1 * vn1 + vt1) * props_2.mass);

                    if i == 0 {
                        f1_x = f1;
                        f2_x = f2;
                    } else {
                        f1_z = f1;
                        f2_z = f2;
                    }
                }
            }
        }
        if let (Some(f1_tan), Some(f2_tan)) = (f1_x, f2_x) {
            if !locked_1 {
                let (mut state_1, _) = bodies.get_mut(joint.body_1).unwrap();
                state_1.apply_force(p1_x, f1_tan);
                state_1.apply_force(p1_x, -f2_tan);
            }
    
            if !locked_2 {
                let (mut state_2, _) = bodies.get_mut(joint.body_2).unwrap();
                state_2.apply_force(p2_x, -f1_tan);
                state_2.apply_force(p2_x, f2_tan);
            }
        }

        if let (Some(f1_bitan), Some(f2_bitan)) = (f1_z, f2_z) {
            if !locked_1 {
                let (mut state_1, _) = bodies.get_mut(joint.body_1).unwrap();
                state_1.apply_force(p1_z, f1_bitan);
                state_1.apply_force(p1_z, -f2_bitan);
            }
    
            if !locked_2 {
                let (mut state_2, _) = bodies.get_mut(joint.body_2).unwrap();
                state_2.apply_force(p2_z, -f1_bitan);
                state_2.apply_force(p2_z, f2_bitan);
            }
        }
    }
}