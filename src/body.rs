use bevy::prelude::*;

use crate::particle::{ParticleTrajectory, ParticleProperties};


#[derive(Component, Reflect, Debug, Default)]
#[reflect(Debug, Default)]
pub struct SoftBodyMassPoints {
    pub particles: Vec<(ParticleTrajectory, ParticleProperties)>,
}


#[derive(Bundle)]
pub struct SoftBody {
    pub mass_points: SoftBodyMassPoints,
}

impl SoftBody {
    /// Rectangle with given dimensions and transformation with no springs
    pub fn rect(dims: IVec2, transform: Transform) -> Self {
        let mut particles = Vec::new();
        let transform_mat = transform.compute_matrix();

        for i in 0..dims.x {
            for j in 0..dims.y {
                let x = i as f32 / dims.x as f32 - 0.5;
                let y = j as f32 / dims.y as f32 - 0.5;
                let pos = transform_mat * Vec4::new(x, y, 1.0, 1.0);
                particles.push((
                    ParticleTrajectory {
                        position: pos.xyz(),
                        ..default()
                    },
                    ParticleProperties::default(),
                ));
            };
        };

        Self {
            mass_points: SoftBodyMassPoints { particles }
        }
    }

    pub fn with_particle_properties(mut self, properties: ParticleProperties) -> Self {
        for particle in self.mass_points.particles.iter_mut() {
            particle.1 = properties.clone();
        };
        self
    }

    /// Add spring connections to nearby mass points
    pub fn tesselate(mut self) -> Self {
        self
    }
}