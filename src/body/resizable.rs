use bevy::prelude::*;

use crate::{particle::{ParticleTrajectory, ParticleProperties}, spring::{Spring, SpringProperties}};

use super::{SoftBodyMassPoints, SoftBodySprings};


#[derive(Component, Default, Reflect, Debug)]
#[reflect(Debug, Default)]
pub struct ResizableSoftBodyProperties {
    pub target_volume: f32,
    pub dims: Vec2,
    pub is_quad: bool,
}


#[derive(Bundle)]
pub struct ResizableSoftBody {
    pub mass_points: SoftBodyMassPoints,
    pub springs: SoftBodySprings,
    pub properties: ResizableSoftBodyProperties,

    #[bundle(ignore)]
    cached_transform: Transform,
    #[bundle(ignore)]
    spring_properties: SpringProperties,
}

impl ResizableSoftBody {
    pub fn square(transform: Transform) -> Self {
        let mut particles = Vec::new();
        let transform_mat = transform.compute_matrix();

        for i in 0..2 {
            for j in 0..2 {
                let x = i as f32 / 2.0 - 0.5;
                let y = j as f32 / 2.0 - 0.5;
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
            properties: ResizableSoftBodyProperties {
                target_volume: (transform.scale.x * transform.scale.y).abs(),
                dims: transform.scale.xy(),
                is_quad: true,
                ..default()
            },
            cached_transform: transform,
            spring_properties: SpringProperties::default(),
        }
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
    pub fn tesselate_from_dims(self) -> Self {
        let dists = self.cached_transform.scale / Vec3::new(2.0, 2.0, 1.0);
        let hypot = (dists.x * dists.x + dists.y * dists.y).sqrt();

        self.tesselate_from_dist(hypot + 0.0001)
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