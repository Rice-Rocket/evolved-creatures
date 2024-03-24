use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::{
    builder::placement::LimbAttachFace,
    config::{ActiveCollisionTypes, CreatureBuilderConfig},
};


#[derive(PartialEq, Eq, Clone, Copy, Component, Debug)]
pub enum ContactFilterTag {
    GroundGroup,
    LimbGroup,
}


#[derive(Component, Clone, Debug)]
pub struct LimbCollisionSensor {
    pub(crate) faces: [LimbCollisionType; 6],
    pub(crate) entities: HashMap<Entity, LimbAttachFace>,
}

impl Index<LimbAttachFace> for LimbCollisionSensor {
    type Output = LimbCollisionType;

    fn index(&self, index: LimbAttachFace) -> &Self::Output {
        &self.faces[index.index()]
    }
}

impl IndexMut<LimbAttachFace> for LimbCollisionSensor {
    fn index_mut(&mut self, index: LimbAttachFace) -> &mut Self::Output {
        &mut self.faces[index.index()]
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LimbCollisionType {
    SelfCollision,
    GroundCollision,
    None,
}


#[derive(SystemParam)]
pub struct ContactFilter<'w, 's> {
    pub(crate) tags: Query<'w, 's, &'static ContactFilterTag>,
    pub(crate) config: Res<'w, CreatureBuilderConfig>,
}

impl BevyPhysicsHooks for ContactFilter<'_, '_> {
    fn filter_contact_pair(&self, context: PairFilterContextView) -> Option<SolverFlags> {
        let tag1 = self.tags.get(context.collider1()).ok().copied()?;
        let tag2 = self.tags.get(context.collider2()).ok().copied()?;

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

        if (limb_ground_collision && self.config.collision_types.contains(ActiveCollisionTypes::LIMB_VS_GROUND))
            || (limb_limb_collision && self.config.collision_types.contains(ActiveCollisionTypes::LIMB_VS_LIMB))
        {
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
                sensor[face_1] = LimbCollisionType::GroundCollision;
                sensor.entities.insert(*entity_2, face_1);
            }
            if *tag_2 == ContactFilterTag::LimbGroup && *tag_1 == ContactFilterTag::GroundGroup {
                let Ok(mut sensor) = sensors.get_mut(*entity_2) else { continue };
                sensor[face_2] = LimbCollisionType::GroundCollision;
                sensor.entities.insert(*entity_1, face_1);
            }
            if *tag_1 == ContactFilterTag::LimbGroup && *tag_2 == ContactFilterTag::LimbGroup {
                let Ok(mut sensor_1) = sensors.get_mut(*entity_1) else { continue };
                sensor_1[face_1] = LimbCollisionType::SelfCollision;
                sensor_1.entities.insert(*entity_2, face_1);
                let Ok(mut sensor_2) = sensors.get_mut(*entity_2) else { continue };
                sensor_2[face_2] = LimbCollisionType::SelfCollision;
                sensor_2.entities.insert(*entity_1, face_2);
            }
        } else if let CollisionEvent::Stopped(entity_1, entity_2, _flags) = collision_event {
            if let Ok(mut sensor_1) = sensors.get_mut(*entity_1) {
                let contact_face = *sensor_1.entities.get(entity_2).unwrap();
                sensor_1[contact_face] = LimbCollisionType::None;
                sensor_1.entities.remove(entity_2);
            }
            if let Ok(mut sensor_2) = sensors.get_mut(*entity_2) {
                let contact_face = *sensor_2.entities.get(entity_1).unwrap();
                sensor_2[contact_face] = LimbCollisionType::None;
                sensor_2.entities.remove(entity_1);
            }
        }
    }
}
