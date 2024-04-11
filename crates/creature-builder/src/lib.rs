pub mod builder;
pub mod config;
pub mod effector;
pub mod expr;
pub mod joint;
pub mod limb;
pub mod sensor;


use std::collections::{hash_map::Entry, HashMap};

use bevy::prelude::*;
use bevy_rapier3d::dynamics::{ExternalImpulse, ImpulseJoint, Velocity};
use config::CreatureBuilderConfig;
use effector::{CreatureContext, CreatureJointEffectors, JointContext};
use joint::CreatureJoint;
use limb::CreatureLimb;
use sensor::{update_sensor_status, LimbCollisionSensor};
use serde::{Deserialize, Serialize};


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct CreatureId(pub usize);


pub struct CreatureBehaviorConfig {
    pub max_force: f32,
    pub max_rel_linvel: f32,
    pub max_angvel: f32,
    pub disable_behavior: bool,
}

impl Default for CreatureBehaviorConfig {
    fn default() -> Self {
        Self { max_force: 0.05, max_rel_linvel: 10.0, max_angvel: 10.0, disable_behavior: false }
    }
}


fn behavior_main(
    time: Res<Time>,
    mut joints: Query<(&CreatureJoint, &ImpulseJoint, &CreatureJointEffectors, Entity), With<CreatureJoint>>,
    mut limbs: Query<(&LimbCollisionSensor, &Transform, &mut ExternalImpulse, &mut Velocity), With<CreatureLimb>>,
    config: Res<CreatureBuilderConfig>,
) {
    if config.behavior.disable_behavior {
        return;
    }

    let mut creature_contexts = HashMap::new();
    let mut joint_indices = HashMap::new();

    for (i, (joint_data, joint, _effectors, entity)) in joints.iter().enumerate() {
        match creature_contexts.entry(joint_data.creature) {
            Entry::Vacant(entry) => {
                let mut context = CreatureContext::new();
                let parent_contacts = limbs.get(joint.parent).unwrap().0;
                let child_contacts = limbs.get(entity).unwrap().0;
                let parent_transform = *limbs.get(joint.parent).unwrap().1;
                let child_transform = *limbs.get(entity).unwrap().1;
                let joint_context = JointContext::new(parent_contacts, child_contacts, &parent_transform, &child_transform);
                context.set_time(time.elapsed_seconds());
                context.add_joint(joint_context);

                joint_indices.insert(i, 0);
                entry.insert(context);
            },
            Entry::Occupied(mut entry) => {
                let parent_contacts = limbs.get(joint.parent).unwrap().0;
                let child_contacts = limbs.get(entity).unwrap().0;
                let parent_transform = *limbs.get(joint.parent).unwrap().1;
                let child_transform = *limbs.get(entity).unwrap().1;
                let context = JointContext::new(parent_contacts, child_contacts, &parent_transform, &child_transform);

                joint_indices.insert(i, entry.get().len());
                entry.get_mut().add_joint(context);
            },
        }
    }

    for (_, joint, _, entity) in joints.iter() {
        limbs.get_mut(joint.parent).unwrap().2.torque_impulse = Vec3::ZERO;
        limbs.get_mut(entity).unwrap().2.torque_impulse = Vec3::ZERO;
    }

    for (i, (joint_data, joint, effectors, entity)) in joints.iter_mut().enumerate() {
        creature_contexts.get_mut(&joint_data.creature).unwrap().set_current_joint(joint_indices[&i]);
        let child_transform = *limbs.get(entity).unwrap().1;
        let context = creature_contexts.get(&joint_data.creature).unwrap();

        for (i, effector) in effectors.effectors.iter().enumerate() {
            let Some(effector) = effector else { continue };
            let force = effector.expr.evaluate(context);

            let (axis, rotational) = match i {
                0 => (Vec3::X, false),
                1 => (Vec3::Y, false),
                2 => (Vec3::Z, false),
                3 => (Vec3::X, true),
                4 => (Vec3::Y, true),
                5 => (Vec3::Z, true),
                _ => unreachable!(),
            };

            if rotational {
                let rot_axis = child_transform.rotation * axis;

                let torque = rot_axis * force.0.clamp(-config.behavior.max_force, config.behavior.max_force);
                limbs.get_mut(joint.parent).unwrap().2.torque_impulse += -torque;
                limbs.get_mut(entity).unwrap().2.torque_impulse += torque;
            }
        }
    }
}


fn clamp_velocity(mut limbs: Query<(&mut Velocity, &CreatureLimb)>, config: Res<CreatureBuilderConfig>) {
    let mut creature_vels: HashMap<CreatureId, Vec3> = HashMap::new();
    for (vel, limb) in limbs.iter() {
        match creature_vels.entry(limb.creature) {
            Entry::Vacant(entry) => {
                entry.insert(vel.linvel / limb.limb_count as f32);
            },
            Entry::Occupied(mut entry) => {
                let linvel = entry.get();
                *entry.get_mut() = *linvel + vel.linvel / limb.limb_count as f32
            },
        };
    }

    for (mut vel, limb) in limbs.iter_mut() {
        let creature_vel = creature_vels.get(&limb.creature).unwrap();
        vel.linvel = vel.linvel.clamp_length_max(config.behavior.max_rel_linvel + creature_vel.length());
        vel.angvel = vel.angvel.clamp_length_max(config.behavior.max_angvel);
    }
}


pub struct CreatureBuilderPlugin;


impl Plugin for CreatureBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CreatureBuilderConfig>().add_systems(Update, (update_sensor_status, behavior_main, clamp_velocity));
    }
}
