use bevy::prelude::*;
use bevy_rapier3d::dynamics::{GenericJoint, ImpulseJoint, JointAxis};

use super::effector::{CreatureJointEffectors, CreatureJointEffector};


#[derive(Component)]
pub struct CreatureJoint;


#[derive(Clone)]
pub struct CreatureJointBuilder {
    pub(crate) parent: Entity,
    pub(crate) data: GenericJoint,
    pub(crate) effectors: CreatureJointEffectors,
}

impl Default for CreatureJointBuilder {
    fn default() -> Self {
        Self {
            parent: Entity::PLACEHOLDER,
            data: GenericJoint::default(),
            effectors: CreatureJointEffectors::default(),
        }
    }
}

impl CreatureJointBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_parent(mut self, parent: Entity) -> Self {
        self.parent = parent;
        self
    }
    pub fn with_generic_joint(mut self, generic_joint: GenericJoint) -> Self {
        self.data = generic_joint;
        self
    }
    pub fn with_effector(mut self, effector: CreatureJointEffector, axis: JointAxis) -> Self {
        self.effectors.insert(effector, axis);
        self
    }
    pub fn with_effectors(mut self, effectors: CreatureJointEffectors) -> Self {
        self.effectors = effectors;
        self
    }
    pub fn finish(self) -> (ImpulseJoint, CreatureJointEffectors, CreatureJoint) {
        (ImpulseJoint::new(self.parent, self.data), self.effectors, CreatureJoint)
    }
}