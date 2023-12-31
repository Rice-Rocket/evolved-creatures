use bevy::prelude::*;

pub mod constraint;
pub mod spherical;
pub mod rigid;
pub mod revolute;


pub trait RBJointType {
    /// The points at which the two bodies should be connected by a zero length spring. 
    fn connection_points(&self, props: &RBJointProperties) -> Vec<(Vec3, Vec3)>;
    /// Equivalent to the `joint_limits` in `RBJointProperties` but is a constant of the joint type
    /// instead of a variable in the standard properties. 
    fn locked_limits(&self) -> Vec2;
}

#[derive(Component, Debug, Reflect)]
#[reflect(Debug, Default)]
pub struct RBJointProperties {
    pub body_1: Entity,
    pub body_2: Entity,
    pub position_1: Vec3, 
    pub position_2: Vec3,
    pub stiffness: f32,
    pub damping: f32,
    pub friction: f32,
    pub limit_stiffness: f32,
    pub limit_damping: f32,
    pub limit_friction: f32,
    pub tangent: Vec3,
    pub bitangent: Vec3,
    /// Determines the maximum bounds of rotation in each degree of freedom. 
    /// 
    /// `x`: 0 - no bend; 1 - full bend
    /// 
    /// `y`: twist
    pub joint_limits: Vec2,
}

impl Default for RBJointProperties {
    fn default() -> Self {
        Self {
            body_1: Entity::PLACEHOLDER,
            body_2: Entity::PLACEHOLDER,
            position_1: Vec3::ZERO,
            position_2: Vec3::ZERO,
            stiffness: 1.0,
            damping: 0.2,
            friction: 1.0,
            limit_stiffness: 1.0,
            limit_damping: 0.0,
            limit_friction: 1.0,
            tangent: Vec3::new(1.0, 0.0, 0.0),
            bitangent: Vec3::new(0.0, 0.0, 1.0),
            joint_limits: Vec2::splat(1.0),
        }
    }
}

impl RBJointProperties {
    pub fn with_relative_rotation(mut self, angle: f32) -> Self {
        let r = Mat2::from_angle(angle);
        let tangent = r * self.tangent.xz();
        let bitangent = r * self.bitangent.xz();
        self.tangent = Vec3::new(tangent.x, 0.0, tangent.y);
        self.bitangent = Vec3::new(bitangent.x, 0.0, bitangent.y);
        self
    }
}


#[derive(Bundle, Default)]
pub struct RBJoint<T: RBJointType + 'static + std::marker::Send + std::marker::Sync + Component> {
    pub ty: T,
    pub props: RBJointProperties,
}