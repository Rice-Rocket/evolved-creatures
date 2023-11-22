use bevy::prelude::*;


pub trait Collider {
    fn normal(&self, pos: Vec3) -> Vec3;
    fn exit_vector(&self, pos: Vec3) -> Option<Vec3>;
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Debug, Default)]
pub struct ColliderProperties {
    pub elasticity: f32,
    pub friction: f32,
    pub restitution: f32,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Debug, Default)]
pub struct HalfSpace {
    pub normal: Vec3,
    pub k: f32,
}

impl Collider for HalfSpace {
    fn normal(&self, _pos: Vec3) -> Vec3 {
        self.normal
    }
    fn exit_vector(&self, pos: Vec3) -> Option<Vec3> {
        let to_pos = pos - self.normal * self.k;
        let dist = to_pos.dot(self.normal);
        if dist > 0.0 {
            return None;
        }
        return Some(-dist * self.normal);
    }
}