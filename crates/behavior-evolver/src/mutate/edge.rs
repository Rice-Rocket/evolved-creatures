use std::ops::Range;

use bevy::math::{Quat, Vec2, Vec3};
use bevy_rapier3d::dynamics::JointAxesMask;
use creature_builder::{
    builder::{
        node::LimbConnection,
        placement::{LimbAttachFace, LimbRelativePlacement},
    },
    effector::CreatureJointEffectors,
};
use rand::{rngs::ThreadRng, Rng};
use rand_distr::Normal;

use super::MutateFieldParams;


pub struct RandomEdgeParams {
    pub placement_pos: Range<f32>,
    pub placement_scale: Range<f32>,
    pub lock_front_rot: bool,
    pub limit_axes: Range<f32>,
}

impl RandomEdgeParams {
    pub fn build_edge(&self, rng: &mut ThreadRng) -> LimbConnection {
        let normal_distr = Normal::new(0f32, 1f32).unwrap();
        let mut dir =
            Vec3::new(rng.sample(normal_distr), rng.sample(normal_distr), rng.sample(normal_distr)).try_normalize().unwrap_or(Vec3::X);
        if self.lock_front_rot {
            dir.y = dir.y.abs()
        };

        let mut limit_axes = [[0f32; 2]; 6];
        for axis in limit_axes.iter_mut() {
            *axis = [rng.gen_range(self.limit_axes.clone()), rng.gen_range(self.limit_axes.clone())];
        }

        LimbConnection {
            placement: LimbRelativePlacement {
                attach_face: LimbAttachFace::from_index(rng.gen_range(0..6)),
                attach_position: Vec2::new(rng.gen_range(self.placement_pos.clone()), rng.gen_range(self.placement_pos.clone())),
                orientation: Quat::from_axis_angle(dir, rng.gen_range(0f32..std::f32::consts::TAU)),
                scale: Vec3::new(
                    rng.gen_range(self.placement_scale.clone()),
                    rng.gen_range(self.placement_scale.clone()),
                    rng.gen_range(self.placement_scale.clone()),
                ),
            },
            locked_axes: JointAxesMask::LIN_AXES,
            limit_axes,
            effectors: CreatureJointEffectors::new([None, None, None, None, None, None]),
        }
    }
}

impl Default for RandomEdgeParams {
    fn default() -> Self {
        Self { placement_pos: -1f32..1f32, placement_scale: 0.5..2.0, lock_front_rot: true, limit_axes: 0f32..std::f32::consts::PI }
    }
}


pub struct MutateEdgeParams {
    pub placement_face_freq: f32,
    pub placement_pos: MutateFieldParams,
    pub placement_rot: MutateFieldParams,
    pub placement_scale: MutateFieldParams,
    pub limit_axes: MutateFieldParams,
}

impl MutateEdgeParams {
    pub fn set_scale(&mut self, inv_scale: f32) {
        self.placement_face_freq *= inv_scale;
        self.placement_pos.set_scale(inv_scale);
        self.placement_rot.set_scale(inv_scale);
        self.placement_scale.set_scale(inv_scale);
        self.limit_axes.set_scale(inv_scale);
    }
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
        if self.params.placement_pos.change(self.rng) {
            self.edge.placement.attach_position.x = self.params.placement_pos.mutate(self.rng, self.edge.placement.attach_position.x);
            self.edge.placement.attach_position.y = self.params.placement_pos.mutate(self.rng, self.edge.placement.attach_position.y);
        };
        if self.params.placement_rot.change(self.rng) {
            let (from_axis, from_angle) = self.edge.placement.orientation.to_axis_angle();
            let to_angle =
                (if self.rng.gen_bool(0.5) { from_angle + std::f32::consts::FRAC_PI_2 } else { from_angle - std::f32::consts::FRAC_PI_2 })
                    .rem_euclid(std::f32::consts::TAU);
            let perp_axis = Self::perpendicular(from_axis);
            let to_axis = Quat::from_axis_angle(from_axis, self.rng.gen_range(0f32..std::f32::consts::TAU)) * perp_axis;
            let to_quat = Quat::from_axis_angle(to_axis, to_angle);
            self.edge.placement.orientation =
                self.edge.placement.orientation.slerp(to_quat, self.params.placement_rot.sample(self.rng).abs().min(1.0));
        };

        for i in 0..3 {
            if self.params.placement_scale.change_scaled(self.rng, 3.0) {
                self.edge.placement.scale[i] = self.params.placement_scale.mutate(self.rng, self.edge.placement.scale[i]);
            };
        }

        for i in 0..6 {
            for j in 0..2 {
                if self.params.limit_axes.change_scaled(self.rng, 12.0) {
                    self.edge.limit_axes[i][j] = self.params.limit_axes.mutate(self.rng, self.edge.limit_axes[i][j]);
                };
            }
        }
    }
}

impl<'a> From<MutateEdge<'a>> for &'a LimbConnection {
    fn from(val: MutateEdge<'a>) -> Self {
        val.into_inner()
    }
}
