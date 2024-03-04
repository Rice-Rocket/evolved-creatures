use bevy::math::{Quat, Vec3};
use creature_builder::builder::{node::LimbConnection, placement::LimbAttachFace};
use rand::{rngs::ThreadRng, Rng};

use super::MutateFieldParams;

pub struct MutateEdgeParams {
    pub placement_face_freq: f32,
    pub placement_pos: MutateFieldParams,
    pub placement_rot: MutateFieldParams,
    pub placement_scale: MutateFieldParams,
    pub limit_axes: MutateFieldParams,
}

pub struct MutateEdge<'a> {
    pub edge: &'a mut LimbConnection,
    pub rng: &'a mut ThreadRng,
    pub params: &'a MutateEdgeParams,
}

impl<'a> MutateEdge<'a> {
    pub fn new(edge: &'a mut LimbConnection, rng: &'a mut ThreadRng, params: &'a MutateEdgeParams) -> Self {
        Self { edge, rng, params }
    }

    pub fn inner(&'a self) -> &'a LimbConnection {
        self.edge
    }
    pub fn into_inner(self) -> &'a LimbConnection {
        self.edge
    }

    fn perpendicular(u: Vec3) -> Vec3 {
        let a = u.abs();
        if a.x <= a.y && a.x <= a.z {
            Vec3::new(0.0, -u.z, u.y)
        } else if a.y <= a.z {
            Vec3::new(-u.z, 0.0, u.x)
        } else {
            Vec3::new(-u.y, u.x, 0.0)
        }
    }
    pub fn mutate(&mut self) {
        if self.rng.gen_bool(self.params.placement_face_freq as f64) {
            self.edge.placement.attach_face = LimbAttachFace::from_index(self.rng.gen_range(0usize..6usize));
        };
        if self.params.placement_pos.change(&mut self.rng) {
            self.edge.placement.attach_position.x = (self.edge.placement.attach_position.x + self.params.placement_pos.sample(&mut self.rng)).clamp(-1.0, 1.0);
            self.edge.placement.attach_position.y = (self.edge.placement.attach_position.y + self.params.placement_pos.sample(&mut self.rng)).clamp(-1.0, 1.0);
        };
        if self.params.placement_rot.change(&mut self.rng) {
            let (from_axis, from_angle) = self.edge.placement.orientation.to_axis_angle();
            let to_angle = (if self.rng.gen_bool(0.5) { from_angle + std::f32::consts::FRAC_PI_2 } else { from_angle - std::f32::consts::FRAC_PI_2 })
                .rem_euclid(std::f32::consts::TAU);
            let perp_axis = Self::perpendicular(from_axis);
            let to_axis = Quat::from_axis_angle(from_axis, self.rng.gen_range(0f32..std::f32::consts::TAU)) * perp_axis;
            let to_quat = Quat::from_axis_angle(to_axis, to_angle);
            self.edge.placement.orientation = self.edge.placement.orientation.slerp(to_quat, self.params.placement_rot.sample(&mut self.rng).abs().min(1.0));
        };

        for i in 0..3 {
            if self.params.placement_scale.change(&mut self.rng) {
                self.edge.placement.scale[i] += self.params.placement_scale.sample(&mut self.rng);
            };
        }

        for i in 0..6 {
            for j in 0..2 {
                if self.params.limit_axes.change(&mut self.rng) {
                    self.edge.limit_axes[i][j] += self.params.limit_axes.sample(&mut self.rng);
                };
            }
        }
    }
}

impl<'a> Into<&'a LimbConnection> for MutateEdge<'a> {
    fn into(self) -> &'a LimbConnection {
        self.into_inner()
    }
}
