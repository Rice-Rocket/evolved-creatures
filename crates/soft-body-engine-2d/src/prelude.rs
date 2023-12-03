pub use crate::{
    body::{
        constrained::{ConstrainedSoftBody, ConstraintProperties},
        resizable::{ResizableSoftBody, ResizableSoftBodyProperties},
        standard::StandardSoftBody,
    },
    sim::SoftBodySimulationSettings,
    particle::ParticleProperties,
    spring::SpringProperties,
    collider::{self, ColliderProperties},
    SoftBodySimulationPlugin, SoftBodyDrawPlugin,
};