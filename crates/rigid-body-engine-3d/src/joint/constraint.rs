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