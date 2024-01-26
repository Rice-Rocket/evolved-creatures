use bevy::prelude::*;


pub enum LimbAttachFace {
    PosX, 
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

impl LimbAttachFace {
    pub fn direction(&self) -> Vec3 {
        match *self {
            LimbAttachFace::PosX => Vec3::X,
            LimbAttachFace::NegX => Vec3::NEG_X,
            LimbAttachFace::PosY => Vec3::Y,
            LimbAttachFace::NegY => Vec3::NEG_Y,
            LimbAttachFace::PosZ => Vec3::Z,
            LimbAttachFace::NegZ => Vec3::NEG_Z,
        }
    }
    pub fn on_tangent_plane(&self, p: Vec2) -> Vec3 {
        match *self {
            LimbAttachFace::PosX => Vec3::new(0.0, p.x, p.y),
            LimbAttachFace::NegX => Vec3::new(0.0, p.x, p.y),
            LimbAttachFace::PosY => Vec3::new(p.x, 0.0, p.y),
            LimbAttachFace::NegY => Vec3::new(p.x, 0.0, p.y),
            LimbAttachFace::PosZ => Vec3::new(p.x, p.y, 0.0),
            LimbAttachFace::NegZ => Vec3::new(p.x, p.y, 0.0),
        }
    }
}


/// The relative translation, orientation, and scale of a limb in comparison to another
pub struct LimbRelativePlacement {
    pub attach_face: LimbAttachFace,
    /// The percentage (in range [-1, 1]) to place the limb
    pub attach_position: Vec2,
    pub orientation: Quat,
    pub scale: Vec3,
}

pub struct LimbPosition {
    pub transform: Transform,
    pub parent_local_anchor: Vec3,
    pub local_anchor: Vec3,
}

impl LimbRelativePlacement {
    pub fn create_transform(&self, parent: Transform) -> LimbPosition {
        let orientation = parent.rotation * self.orientation;
        let attach_point = parent.scale * (self.attach_face.direction() + self.attach_face.on_tangent_plane(self.attach_position));

        let to_body_mid = self.orientation * self.attach_face.direction();
        let translation = attach_point + to_body_mid * (parent.scale * self.scale).dot(self.attach_face.direction());
        
        let global_translation = (parent.rotation * translation) + parent.translation;
        let global_attach_point = (parent.rotation * attach_point) + parent.translation;
        let local_anchor = self.orientation.inverse() * (global_attach_point - global_translation);

        LimbPosition {
            transform: Transform::from_translation(global_translation)
                .with_rotation(orientation)
                .with_scale(parent.scale * self.scale),
            parent_local_anchor: attach_point,
            local_anchor,
        }
    }
}