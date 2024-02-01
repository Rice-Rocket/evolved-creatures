use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;


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
    pub fn orientation(&self) -> Quat {
        match *self {
            LimbAttachFace::PosX => Quat::from_rotation_z(FRAC_PI_2),
            LimbAttachFace::NegX => Quat::from_rotation_z(-FRAC_PI_2),
            LimbAttachFace::PosY => Quat::IDENTITY,
            LimbAttachFace::NegY => Quat::from_rotation_arc(Vec3::Y, Vec3::NEG_Y),
            LimbAttachFace::PosZ => Quat::from_rotation_x(FRAC_PI_2),
            LimbAttachFace::NegZ => Quat::from_rotation_x(-FRAC_PI_2),
        }
    }
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::PosX,
            1 => Self::NegX,
            2 => Self::PosY,
            3 => Self::NegY,
            4 => Self::PosZ,
            5 => Self::NegZ,
            _ => { panic!("Cannot index into LimbAttachFace with index {}", index) }
        }
    }
}


/// The relative translation, orientation, and scale of a limb in comparison to its parent
pub struct LimbRelativePlacement {
    pub attach_face: LimbAttachFace,
    /// The percentage along the attach face (in range `[-1, 1]`) to place the limb
    pub attach_position: Vec2,
    /// The orientation of the limb relative to its attached face
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
        let actual_orientation = self.orientation * self.attach_face.orientation();

        let orientation = parent.rotation * actual_orientation; 
        let attach_point = parent.scale * (self.attach_face.direction() + self.attach_face.on_tangent_plane(self.attach_position));

        // broken part
        let to_body_mid = self.orientation * self.attach_face.direction();
        let translation = attach_point + to_body_mid * (parent.scale * self.scale).dot(Vec3::Y);
        
        let global_translation = (parent.rotation * translation) + parent.translation;
        let global_attach_point = (parent.rotation * attach_point) + parent.translation;
        let local_anchor = actual_orientation.inverse() * (global_attach_point - global_translation);

        LimbPosition {
            transform: Transform::from_translation(global_translation)
                .with_rotation(orientation)
                .with_scale(parent.scale * self.scale),
            parent_local_anchor: attach_point,
            local_anchor,
        }
    }
}