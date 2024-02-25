use std::{collections::HashMap, ops::Index};

use bevy::{ecs::component::Component, math::{Quat, Vec3}};
use bevy_rapier3d::{dynamics::{ImpulseJoint, JointAxis}, parry::math::Vector};

use super::{expr::Expr, builder::placement::LimbAttachFace, sensor::{LimbCollisionSensor, LimbCollisionType}};


#[derive(Component, Clone)]
pub struct CreatureJointEffectors {
    pub effectors: [Option<CreatureJointEffector>; 6],
}

impl CreatureJointEffectors {
    pub fn new(effectors: [Option<CreatureJointEffector>; 6]) -> Self {
        Self { effectors }
    }
    pub fn insert(&mut self, effector: CreatureJointEffector, axis: JointAxis) {
        match axis {
            JointAxis::X => self.effectors[0] = Some(effector),
            JointAxis::Y => self.effectors[1] = Some(effector),
            JointAxis::Z => self.effectors[2] = Some(effector),
            JointAxis::AngX => self.effectors[3] = Some(effector),
            JointAxis::AngY => self.effectors[4] = Some(effector),
            JointAxis::AngZ => self.effectors[5] = Some(effector),
        };
    }
}

impl Default for CreatureJointEffectors {
    fn default() -> Self {
        Self { effectors: [None, None, None, None, None, None] }
    }
}


#[derive(Clone)]
pub struct CreatureJointEffector {
    pub pos_expr: Expr,
    pub stiffness_expr: Expr,
    pub damping_expr: Expr,
}


#[derive(Clone, Copy)]
pub enum JointContextElement {
    ParentContact { face: LimbAttachFace },
    ChildContact { face: LimbAttachFace },
    JointAxis { axis: JointAxis },
}


pub struct JointContext {
    parent_contacts: LimbCollisionSensor,
    child_contacts: LimbCollisionSensor,
    joint_axes: [f32; 6],
}

impl JointContext {
    pub fn new(parent_contacts: &LimbCollisionSensor, child_contacts: &LimbCollisionSensor, joint: &ImpulseJoint) -> Self {
        let joint_axes = [
            0.0, 
            0.0, 
            0.0,
            Self::calc_basis_diff(joint, *Vector::x_axis()),
            Self::calc_basis_diff(joint, *Vector::y_axis()),
            Self::calc_basis_diff(joint, *Vector::z_axis()),
        ];
        Self {
            parent_contacts: LimbCollisionSensor { faces: parent_contacts.faces, entities: HashMap::new() },
            child_contacts: LimbCollisionSensor { faces: child_contacts.faces, entities: HashMap::new() },
            joint_axes,
        }
    }
    fn calc_basis_diff(joint: &ImpulseJoint, axis: Vector<f32>) -> f32 {
        let axis1 = joint.data.raw.local_frame1 * axis;
        let axis2 = joint.data.raw.local_frame2 * axis;
        let swing_1 = Self::quat_swing_twist(joint.data.local_basis1(), axis1.into()).1;
        let swing_2 = Self::quat_swing_twist(joint.data.local_basis2(), axis2.into()).1;
        swing_1.angle_between(swing_2)
    }
    /// Decompose the quaternion on to 2 parts.
    /// 1. Twist - rotation around the "direction" vector
    /// 2. Swing - rotation around axis that is perpendicular to "direction" vector
    /// 
    /// The rotation can be composed back by
    /// `rotation = swing * twist`
    /// 
    /// From: https://stackoverflow.com/a/22401169
    fn quat_swing_twist(quat: Quat, dir: Vec3) -> (Quat, Quat) {
        let ra = Vec3::new(quat.x, quat.y, quat.z);
        let p = dir * ra.dot(dir);
        let twist = Quat::from_xyzw(p.x, p.y, p.z, quat.w).normalize();
        let swing = quat * twist.conjugate();
        (twist, swing)
    }
}

impl Index<JointContextElement> for JointContext {
    type Output = f32;
    fn index(&self, index: JointContextElement) -> &Self::Output {
        match index {
            JointContextElement::ParentContact { face } => {
                match self.parent_contacts[face] {
                    LimbCollisionType::None => &-1.0,
                    _ => &1.0,
                }
            },
            JointContextElement::ChildContact { face } => {
                match self.child_contacts[face] {
                    LimbCollisionType::None => &-1.0,
                    _ => &1.0,
                }
            },
            JointContextElement::JointAxis { axis } => {
                &self.joint_axes[
                    match axis {
                        JointAxis::X => 0,
                        JointAxis::Y => 1,
                        JointAxis::Z => 2,
                        JointAxis::AngX => 3,
                        JointAxis::AngY => 4,
                        JointAxis::AngZ => 5,
                    }
                ]
            }
        }
    }
}