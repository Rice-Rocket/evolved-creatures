use bevy::prelude::*;
use bevy_rapier3d::dynamics::{GenericJoint, ImpulseJoint};


#[derive(Component)]
pub struct CreatureJoint;


pub struct CreatureJointBuilder {
    pub(crate) parent: Entity,
    pub(crate) data: GenericJoint,
}

impl Default for CreatureJointBuilder {
    fn default() -> Self {
        Self {
            parent: Entity::PLACEHOLDER,
            data: GenericJoint::default(),
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
    pub fn finish(self) -> (ImpulseJoint, CreatureJoint) {
        (ImpulseJoint::new(self.parent, self.data), CreatureJoint)
    }
}