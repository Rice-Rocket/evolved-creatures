use bevy::{prelude::*, ecs::schedule::ScheduleLabel};

use crate::prelude::initialize_bodies;



#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct RigidBodySimulationSchedule;


#[derive(Resource, Reflect, Debug)]
#[reflect(Debug, Default)]
pub struct RigidBodySimulationSettings {
    pub num_substeps: u32,
    pub running: bool,
    pub speed: f32,
    pub startup_time_buffer: f32,

    pub initialized: bool,
    pub sub_dt: f32,
}

impl Default for RigidBodySimulationSettings {
    fn default() -> Self {
        Self {
            num_substeps: 4,
            startup_time_buffer: 1.0,
            speed: 1.0,
            running: true,

            initialized: false,
            sub_dt: 0.0,
        }
    }
}


pub(crate) fn run_physics_sim_schedule(world: &mut World) {
    let time = world.resource::<Time>();
    let dt = time.delta_seconds();
    let elapsed = time.elapsed_seconds();

    {
        let mut sim_settings = world.resource_mut::<RigidBodySimulationSettings>();
        sim_settings.sub_dt = dt / sim_settings.num_substeps as f32 * sim_settings.speed;
        
        if !sim_settings.initialized {
            sim_settings.initialized = true;
            let init_bodies_system = world.register_system(initialize_bodies);
            world.run_system(init_bodies_system).unwrap();
        }
    }

    let sim_settings = world.resource::<RigidBodySimulationSettings>();

    if elapsed < sim_settings.startup_time_buffer || !sim_settings.running {
        return;
    }
    
    for _ in 0..sim_settings.num_substeps {
        world.run_schedule(RigidBodySimulationSchedule);
    }
}