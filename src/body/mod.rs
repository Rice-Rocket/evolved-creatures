use bevy::prelude::*;

use crate::{particle::{ParticleTrajectory, ParticleProperties}, spring::Spring};

pub mod standard;
pub mod constrained;
pub mod resizable;


#[derive(Component, Reflect, Debug, Default)]
#[reflect(Debug, Default)]
pub struct SoftBodyMassPoints(pub Vec<(ParticleTrajectory, ParticleProperties)>);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Debug, Default)]
pub struct SoftBodySprings(pub Vec<Spring>);