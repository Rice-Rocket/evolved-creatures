use std::collections::HashMap;

use bevy_rapier3d::prelude::*;
use bevy::prelude::*;

use crate::{CreatureId, sensor::{LimbCollisionSensor, ContactFilterTag, LimbCollisionType}};



#[derive(Component, Clone, Debug)]
pub struct CreatureLimb {
    pub creature: CreatureId,
}



#[derive(Bundle, Clone, Debug)]
pub struct CreatureLimbBundle {
    pub(crate) limb: CreatureLimb,
    pub(crate) name: Name,
    pub(crate) sensor: LimbCollisionSensor,
    pub(crate) filter_tag: ContactFilterTag,

    // Rigid body
    pub(crate) rb: RigidBody,
    pub(crate) ccd: Ccd,
    pub(crate) velocity: Velocity,
    pub(crate) gravity: GravityScale,
    pub(crate) impulses: ExternalImpulse,
    pub(crate) active_events: ActiveEvents,
    pub(crate) active_hooks: ActiveHooks,

    // Collider
    pub(crate) collider: Collider,
    pub(crate) friction: Friction,
    pub(crate) restitution: Restitution,
    pub(crate) mass: ColliderMassProperties,

    // Material and Mesh
    pub(crate) mesh: Handle<Mesh>,
    pub(crate) material: Handle<StandardMaterial>,

    // Transform
    pub(crate) transform: Transform,
    pub(crate) global_transform: GlobalTransform,

    // Visibility
    pub(crate) visibility: Visibility,
    pub(crate) inherited_visibility: InheritedVisibility,
    pub(crate) view_visibility: ViewVisibility,

    #[bundle(ignore)]
    color: Color,
}




impl Default for CreatureLimbBundle {
    fn default() -> Self {
        CreatureLimbBundle {
            limb: CreatureLimb { creature: CreatureId(0) },
            name: Name::new("()"),
            sensor: LimbCollisionSensor { faces: [LimbCollisionType::None; 6], entities: HashMap::new() },
            filter_tag: ContactFilterTag::LimbGroup,

            rb: RigidBody::Dynamic,
            ccd: Ccd::enabled(),
            velocity: Velocity { linvel: Vec3::ZERO, angvel: Vec3::ZERO },
            gravity: GravityScale(1.0),
            impulses: ExternalImpulse { impulse: Vec3::ZERO, torque_impulse: Vec3::ZERO },
            active_events: ActiveEvents::COLLISION_EVENTS,
            active_hooks: ActiveHooks::FILTER_CONTACT_PAIRS,

            collider: Collider::cuboid(1.0, 1.0, 1.0),
            friction: Friction { coefficient: 0.3, combine_rule: CoefficientCombineRule::Average },
            restitution: Restitution { coefficient: 0.0, combine_rule: CoefficientCombineRule::Average },
            mass: ColliderMassProperties::Density(1.0),

            mesh: Handle::default(),
            material: Handle::default(),

            transform: Transform::from_translation(Vec3::ZERO),
            global_transform: GlobalTransform::default(),

            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),

            color: Color::WHITE,
        }
    }
}




impl CreatureLimbBundle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_size(mut self, half_size: Vec3) -> Self {
        self.collider.set_scale(half_size, 0);
        self.transform.scale = half_size;
        self
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
    pub fn with_initial_force(mut self, force: Vec3) -> Self {
        self.impulses.impulse = force;
        self
    }
    pub fn with_initial_torque(mut self, torque: Vec3) -> Self {
        self.impulses.torque_impulse = torque;
        self
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Name::new(name);
        self
    }
    pub fn with_creature(mut self, id: CreatureId) -> Self {
        self.limb.creature = id;
        self
    }
    pub fn with_density(mut self, density: f32) -> Self {
        self.mass = ColliderMassProperties::Density(density);
        self
    }
    pub fn with_friction(mut self, friction: f32) -> Self {
        self.friction.coefficient = friction;
        self
    }
    pub fn with_restitution(mut self, restitution: f32) -> Self {
        self.restitution.coefficient = restitution;
        self
    }
    pub fn finish(mut self, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>) -> Self {
        self.mesh = meshes.add(Mesh::from(shape::Box::new(2.0, 2.0, 2.0)));
        self.material = materials.add(StandardMaterial::from(self.color));
        self
    }
}

