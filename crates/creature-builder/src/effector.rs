use std::ops::Index;

use bevy_rapier3d::dynamics::JointAxis;

use super::{expr::Expr, builder::placement::LimbAttachFace, sensor::{LimbCollisionSensor, LimbCollisionType}};

pub struct CreatureJointEffector {
    pub expr: Expr,
}


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