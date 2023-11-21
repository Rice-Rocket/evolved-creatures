use bevy::prelude::*;


pub trait Collider {
    fn on_surface(&self, pos: Vec3) -> Option<Vec3>;
}


#[derive(Component)]
pub struct HalfSpace {
    pub normal: Vec3,
    pub k: f32,
}

impl Collider for HalfSpace {
    fn on_surface(&self, pos: Vec3) -> Option<Vec3> {
        let to_surface = pos - self.normal * self.k;
        let dist = to_surface.dot(self.normal);
        if dist > 0.0 {
            return None;
        }
        return Some(pos - dist * self.normal);
    }
}