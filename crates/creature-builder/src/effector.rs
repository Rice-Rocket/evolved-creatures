use std::{collections::HashMap, ops::Index};

use bevy::{
    ecs::component::Component,
    math::{Quat, Vec3},
    transform::components::Transform,
};
use bevy_rapier3d::dynamics::JointAxis;
use serde::{Deserialize, Serialize};

use crate::{
    builder::placement::LimbAttachFace,
    expr::Expr,
    sensor::{LimbCollisionSensor, LimbCollisionType},
};


#[derive(Component, Clone, Debug, Default, Serialize, Deserialize)]
pub struct CreatureJointEffectors {
    /// Ordered: [X, Y, Z, AngX, AngY, AngZ]
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


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatureJointEffector {
    pub expr: Expr,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CreatureContextElement {
    LocalJoint { element: JointContextElement },
    GlobalJoint { element: JointContextElement, joint: usize },
    Time,
}

pub struct CreatureContext {
    joints: Vec<JointContext>,
    current_joint: usize,
    elapsed_time: f32,
}

impl Default for CreatureContext {
    fn default() -> Self {
        Self::new()
    }
}

impl CreatureContext {
    pub fn new() -> Self {
        Self { joints: Vec::new(), current_joint: 0, elapsed_time: 0.0 }
    }

    pub fn add_joint(&mut self, ctx: JointContext) {
        self.joints.push(ctx);
    }

    pub fn set_current_joint(&mut self, index: usize) {
        self.current_joint = index;
    }

    pub fn len(&self) -> usize {
        self.joints.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set_time(&mut self, time: f32) {
        self.elapsed_time = time;
    }

    pub fn index(&self, index: CreatureContextElement) -> Option<f32> {
        match index {
            CreatureContextElement::LocalJoint { element } => Some(self.joints[self.current_joint][element]),
            CreatureContextElement::GlobalJoint { element, joint } => {
                let ctx = self.joints.get(joint)?;
                Some(ctx[element])
            },
            CreatureContextElement::Time => Some(self.elapsed_time),
        }
    }
}


#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum JointContextElement {
    ParentContact { face: LimbAttachFace },
    ChildContact { face: LimbAttachFace },
    JointAxis { axis: JointAxis },
}


#[derive(Clone)]
pub struct JointContext {
    parent_contacts: LimbCollisionSensor,
    child_contacts: LimbCollisionSensor,
    joint_axes: [f32; 6],
}

impl JointContext {
    pub fn new(
        parent_contacts: &LimbCollisionSensor,
        child_contacts: &LimbCollisionSensor,
        parent_transform: &Transform,
        child_transform: &Transform,
    ) -> Self {
        let joint_axes = [
            0.0,
            0.0,
            0.0,
            Self::calc_basis_diff(parent_transform, child_transform, Vec3::X),
            Self::calc_basis_diff(parent_transform, child_transform, Vec3::Y),
            Self::calc_basis_diff(parent_transform, child_transform, Vec3::Z),
        ];
        Self {
            parent_contacts: LimbCollisionSensor { faces: parent_contacts.faces, entities: HashMap::new() },
            child_contacts: LimbCollisionSensor { faces: child_contacts.faces, entities: HashMap::new() },
            joint_axes,
        }
    }

    fn calc_basis_diff(parent_transform: &Transform, child_transform: &Transform, axis: Vec3) -> f32 {
        let axis1 = child_transform.rotation * axis;
        let twist_1 = Self::quat_swing_twist(parent_transform.rotation, axis1).0;
        let twist_2 = Self::quat_swing_twist(child_transform.rotation, axis1).0;
        twist_1.angle_between(twist_2)
    }

    /// Decompose the quaternion on to 2 parts.
    /// 1. Twist - rotation around the "direction" vector
    /// 2. Swing - rotation around axis that is perpendicular to "direction"
    /// vector
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
            JointContextElement::ParentContact { face } => match self.parent_contacts[face] {
                LimbCollisionType::None => &-1.0,
                _ => &1.0,
            },
            JointContextElement::ChildContact { face } => match self.child_contacts[face] {
                LimbCollisionType::None => &-1.0,
                _ => &1.0,
            },
            JointContextElement::JointAxis { axis } => {
                &self.joint_axes[match axis {
                    JointAxis::X => 0,
                    JointAxis::Y => 1,
                    JointAxis::Z => 2,
                    JointAxis::AngX => 3,
                    JointAxis::AngY => 4,
                    JointAxis::AngZ => 5,
                }]
            },
        }
    }
}
