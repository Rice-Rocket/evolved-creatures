use bevy::prelude::*;

use crate::{particle::{ParticleTrajectory, ParticleProperties}, spring::{Spring, SpringProperties}};

use super::{SoftBodyMassPoints, SoftBodySprings};


#[derive(Component, Reflect, Debug, Default)]
#[reflect(Debug, Default)]
pub struct SoftBodyReferenceMassPoints(pub Vec<(Vec3, Vec3)>);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Debug, Default)]
pub struct ConstraintProperties {
    pub stiffness: f32,
    pub damping: f32,
}


#[derive(Bundle)]
pub struct ConstrainedSoftBody {
    pub mass_points: SoftBodyMassPoints,
    pub springs: SoftBodySprings,
    pub reference_points: SoftBodyReferenceMassPoints,
    pub constraint_props: ConstraintProperties,

    #[bundle(ignore)]
    cached_transform: Transform,
    #[bundle(ignore)]
    spring_properties: SpringProperties,
}

impl ConstrainedSoftBody {
    /// Rectangle with given dimensions and transformation with no springs
    pub fn rect(dims: IVec2, transform: Transform) -> Self {
        let mut particles = Vec::new();
        let transform_mat = transform.compute_matrix();

        for i in 0..dims.x {
            for j in 0..dims.y {
                if (dims.x - 1 > i && i > 0) && (dims.y - 1 > j && j > 0) { continue };

                let x = i as f32 / dims.x as f32 - 0.5;
                let y = j as f32 / dims.y as f32 - 0.5;
                let pos = transform_mat * Vec4::new(x, y, 1.0, 1.0);
                particles.push((
                    ParticleTrajectory {
                        position: Vec3::new(pos.x, pos.y, 0.0),
                        ..default()
                    },
                    ParticleProperties::default(),
                ));
            };
        };

        Self {
            mass_points: SoftBodyMassPoints(particles.clone()),
            springs: SoftBodySprings(Vec::new()),
            reference_points: SoftBodyReferenceMassPoints(particles.iter().map(|x| (x.0.position, x.0.position)).collect()),
            constraint_props: ConstraintProperties::default(),
            cached_transform: transform,
            spring_properties: SpringProperties::default(),
        }
    }

    pub fn with_properties(mut self, properties: ConstraintProperties) -> Self {
        self.constraint_props = properties;
        self
    }
    pub fn with_spring_properties(mut self, properties: SpringProperties) -> Self {
        for spring in self.springs.0.iter_mut() {
            spring.properties = properties.clone();
        };
        self.spring_properties = properties;
        self
    }
    pub fn with_particle_properties(mut self, properties: ParticleProperties) -> Self {
        for particle in self.mass_points.0.iter_mut() {
            particle.1 = properties.clone();
        };
        self
    }

    pub fn tesselate_from_dist(mut self, min_dist: f32) -> Self {
        let mut connections: Vec<(usize, usize)> = Vec::new();
        let min_dist2 = min_dist * min_dist;

        for (i, p1) in self.mass_points.0.iter().enumerate() {
            let p1_pos = p1.0.position;

            for (j, p2) in self.mass_points.0.iter().enumerate() {
                if i == j { continue };

                let p2_pos = p2.0.position;
                let dist2 = (p2_pos - p1_pos).length_squared();

                if dist2 <= min_dist2 {
                    if !connections.contains(&(i, j)) && !connections.contains(&(j, i)) {
                        connections.push((i, j));
                    }
                }
            }
        }

        self.springs.0.clear();

        for connection in connections.iter() {
            self.springs.0.push(Spring {
                p1_idx: connection.0,
                p2_idx: connection.1,
                properties: self.spring_properties.clone(),
            })
        }

        self
    }
    pub fn tesselate_from_dims(self, dims: IVec2) -> Self {
        let dists = self.cached_transform.scale / Vec3::new(dims.x as f32, dims.y as f32, 1.0);
        self.tesselate_from_dist(dists.x.max(dists.y) + 0.0001)
    }

    pub fn set_spring_rest_lengths(mut self) -> Self {
        for spring in self.springs.0.iter_mut() {
            let p1 = &self.mass_points.0[spring.p1_idx].0;
            let p2 = &self.mass_points.0[spring.p2_idx].0;
            spring.properties.rest_length = (p1.position - p2.position).length();
        }

        self
    }
}