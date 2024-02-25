use bevy::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LimbAttachFace {
    PosX, 
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

impl LimbAttachFace {
    pub fn from_point(p: Vec3) -> Self {
        if p.max_element() > p.min_element().abs() {
            if p.x > p.y && p.x > p.z {
                LimbAttachFace::PosX
            } else if p.y > p.z {
                LimbAttachFace::PosY
            } else {
                LimbAttachFace::PosZ
            }
        } else {
            if p.x < p.y && p.x < p.z {
                LimbAttachFace::NegX
            } else if p.y < p.z {
                LimbAttachFace::NegY
            } else {
                LimbAttachFace::NegZ
            }
        }
    }
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
            LimbAttachFace::PosX => Quat::from_rotation_arc(Vec3::Y, Vec3::X),
            LimbAttachFace::NegX => Quat::from_rotation_arc(Vec3::Y, Vec3::NEG_X),
            LimbAttachFace::PosY => Quat::IDENTITY,
            LimbAttachFace::NegY => Quat::from_rotation_arc(Vec3::Y, Vec3::NEG_Y),
            LimbAttachFace::PosZ => Quat::from_rotation_arc(Vec3::Y, Vec3::Z),
            LimbAttachFace::NegZ => Quat::from_rotation_arc(Vec3::Y, Vec3::NEG_Z),
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
    pub fn index(&self) -> usize {
        match *self {
            LimbAttachFace::PosX => 0,
            LimbAttachFace::NegX => 1,
            LimbAttachFace::PosY => 2,
            LimbAttachFace::NegY => 3,
            LimbAttachFace::PosZ => 4,
            LimbAttachFace::NegZ => 5,
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
        let actual_orientation = self.attach_face.orientation() * self.orientation;

        let orientation = parent.rotation * actual_orientation; 
        let attach_point = parent.scale * (self.attach_face.direction() + self.attach_face.on_tangent_plane(self.attach_position));
        let local_anchor = parent.scale * self.scale * Vec3::NEG_Y;

        let global_attach_point = (parent.rotation * attach_point) + parent.translation;
        let to_body_mid = orientation * Vec3::Y;
        let global_translation = global_attach_point + to_body_mid * (parent.scale * self.scale).dot(Vec3::Y);

        LimbPosition {
            transform: Transform::from_translation(global_translation)
                .with_rotation(orientation)
                .with_scale(parent.scale * self.scale),
            parent_local_anchor: attach_point,
            local_anchor,
        }
    }
}