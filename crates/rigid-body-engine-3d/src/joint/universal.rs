use bevy::prelude::*;

use crate::prelude::{RigidBodyState, RigidBodyProperties};

#[derive(Component, Debug, Reflect)]
#[reflect(Debug, Default)]
pub struct RBUniversalJoint {
    pub body_1: Entity,
    pub body_2: Entity,
    pub position_1: Vec3, // relative translation to body's origin in the body's local space
    pub position_2: Vec3,
    pub stiffness: f32,
    pub damping: f32,
    pub friction: f32,
}
// ! Forces are exerted by joints only
// ! Joints are essentially motors

impl Default for RBUniversalJoint {
    fn default() -> Self {
        Self {
            body_1: Entity::PLACEHOLDER,
            body_2: Entity::PLACEHOLDER,
            position_1: Vec3::ZERO,
            position_2: Vec3::ZERO,
            stiffness: 1.0,
            damping: 0.2,
            friction: 1.0,
        }
    }
}


pub(crate) fn apply_universal_joint_force(
    mut gizmos: Gizmos,
    joints: Query<&RBUniversalJoint, Without<RigidBodyState>>,
    mut bodies: Query<(&mut RigidBodyState, &RigidBodyProperties), Without<RBUniversalJoint>>,
) {
    for joint in joints.iter() {
        let f;
        let (locked_1, locked_2);
        let (p, p1);
        {
            let (state_1, props_1) = bodies.get(joint.body_1).unwrap();
            let (state_2, props_2) = bodies.get(joint.body_2).unwrap();

            p = state_1.globalize(joint.position_1 * props_1.scale * 0.5);
            p1 = state_2.globalize(joint.position_2 * props_2.scale * 0.5);

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
            f = -(d * kc - b * vn + vt) * props_1.mass;
        }

        gizmos.line(p, p1, Color::GREEN);

        if !locked_1 {
            let (mut state_1, _) = bodies.get_mut(joint.body_1).unwrap();
            state_1.apply_force(p, -f);
        }
        
        if !locked_2 {
            let (mut state_2, _) = bodies.get_mut(joint.body_2).unwrap();
            state_2.apply_force(p1, f);
        }
    }
}