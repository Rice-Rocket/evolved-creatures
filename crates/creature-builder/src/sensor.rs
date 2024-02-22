use bevy::{prelude::*, ecs::system::SystemParam};
use bevy_rapier3d::prelude::*;

use super::{config::{ActiveCollisionTypes, CreatureBuilderConfig}, builder::placement::LimbAttachFace};


#[derive(PartialEq, Eq, Clone, Copy, Component)]
pub enum ContactFilterTag {
    GroundGroup,
    LimbGroup
}


#[derive(Component, Clone)]
pub struct LimbCollisionSensor {
    pub ty: LimbCollisionType,
    pub face: LimbAttachFace,
}


#[derive(Clone, PartialEq)]
pub enum LimbCollisionType {
    SelfCollision,
    GroundCollision,
    None
}


#[derive(SystemParam)]
pub struct ContactFilter<'w, 's> {
    pub(crate) tags: Query<'w, 's, &'static ContactFilterTag>,
    pub(crate) config: Res<'w, CreatureBuilderConfig>,
}

impl BevyPhysicsHooks for ContactFilter<'_, '_> {
    fn filter_contact_pair(&self, context: PairFilterContextView) -> Option<SolverFlags> {
        let Some(tag1) = self.tags.get(context.collider1()).ok().copied() else { return None };
        let Some(tag2) = self.tags.get(context.collider2()).ok().copied() else { return None };
        
        let mut limb_ground_collision = false;
        let mut limb_limb_collision = false;
        if tag1 == ContactFilterTag::LimbGroup && tag2 == ContactFilterTag::LimbGroup {
            limb_limb_collision = true;
        }

        if tag1 == ContactFilterTag::LimbGroup && tag2 == ContactFilterTag::GroundGroup {
            limb_ground_collision = true;
        }

        if tag2 == ContactFilterTag::LimbGroup && tag1 == ContactFilterTag::GroundGroup {
            limb_ground_collision = true;
        }

        if limb_ground_collision && self.config.collision_types.contains(ActiveCollisionTypes::LIMB_VS_GROUND) {
            Some(SolverFlags::COMPUTE_IMPULSES)
        } else if limb_limb_collision && self.config.collision_types.contains(ActiveCollisionTypes::LIMB_VS_LIMB) {
            Some(SolverFlags::COMPUTE_IMPULSES)
        } else {
            Some(SolverFlags::empty())
        }
    }
}


pub(crate) fn update_sensor_status(
    mut collision_events: EventReader<CollisionEvent>,
    tags: Query<&ContactFilterTag>,
    mut sensors: Query<&mut LimbCollisionSensor>,
    context: Res<RapierContext>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity_1, entity_2, _flags) = collision_event {
            let Some(contact_pair) = context.contact_pair(*entity_1, *entity_2) else { continue };
            let (_contact_manifold, contact_view) = contact_pair.find_deepest_contact().unwrap();
            
            let face_1 = LimbAttachFace::from_point(contact_view.local_p1());
            let face_2 = LimbAttachFace::from_point(contact_view.local_p2());

            let Ok(tag_1) = tags.get(*entity_1) else { continue };
            let Ok(tag_2) = tags.get(*entity_2) else { continue };

            if *tag_1 == ContactFilterTag::LimbGroup && *tag_2 == ContactFilterTag::GroundGroup {
                let Ok(mut sensor) = sensors.get_mut(*entity_1) else { continue };
                sensor.face = face_1.clone();
                sensor.ty = LimbCollisionType::GroundCollision;
            }
            if *tag_2 == ContactFilterTag::LimbGroup && *tag_1 == ContactFilterTag::GroundGroup {
                let Ok(mut sensor) = sensors.get_mut(*entity_2) else { continue };
                sensor.face = face_2.clone();
                sensor.ty = LimbCollisionType::GroundCollision;
            }
            if *tag_1 == ContactFilterTag::LimbGroup && *tag_2 == ContactFilterTag::LimbGroup {
                let Ok(mut sensor_1) = sensors.get_mut(*entity_1) else { continue };
                sensor_1.face = face_1;
                sensor_1.ty = LimbCollisionType::SelfCollision;
                let Ok(mut sensor_2) = sensors.get_mut(*entity_2) else { continue };
                sensor_2.face = face_2;
                sensor_2.ty = LimbCollisionType::SelfCollision;
            }
        }

        else if let CollisionEvent::Stopped(entity_1, entity_2, _flags) = collision_event {
            if let Ok(mut sensor_1) = sensors.get_mut(*entity_1) {
                sensor_1.ty = LimbCollisionType::None;
            }
            if let Ok(mut sensor_2) = sensors.get_mut(*entity_2) {
                sensor_2.ty = LimbCollisionType::None;
            }
        }
    }
}