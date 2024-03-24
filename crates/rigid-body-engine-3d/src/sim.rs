use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

use crate::prelude::initialize_bodies;


#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct RigidBodySimulationSchedule;


#[derive(Resource, Reflect, Debug)]
#[reflect(Debug, Default)]
pub struct RigidBodySimulationSettings {
    pub num_substeps: u32,
    pub speed: f32,

    pub startup_time_buffer: f32,
    pub running: bool,
    pub pause_countdown: f32,

    pub step_dt: f32,
    pub queued_steps: u32,

    pub initialized: bool,
    pub sub_dt: f32,
}

impl Default for RigidBodySimulationSettings {
    fn default() -> Self {
        Self {
            num_substeps: 4,
            speed: 1.0,

            startup_time_buffer: 1.0,
            running: true,
            pause_countdown: 0.0,

            step_dt: 1.0 / 60.0,
            queued_steps: 0,

            initialized: false,
            sub_dt: 0.0,
        }
    }
}

impl RigidBodySimulationSettings {
    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn pause_for(&mut self, seconds: f32) {
        self.pause_countdown = seconds;
    }

    pub fn play(&mut self) {
        self.running = true;
    }

    pub fn step(&mut self) {
        self.queued_steps += 1;
    }
}


pub(crate) fn run_physics_sim_schedule(world: &mut World) {
    let time = world.resource::<Time>();
    let dt = time.delta_seconds();
    let elapsed = time.elapsed_seconds();

    {
        let mut sim_settings = world.resource_mut::<RigidBodySimulationSettings>();
        sim_settings.sub_dt = if sim_settings.running {
            dt / sim_settings.num_substeps as f32 * sim_settings.speed
        } else if sim_settings.queued_steps > 0 {
            sim_settings.queued_steps -= 1;
            sim_settings.step_dt / sim_settings.num_substeps as f32
        } else {
            return;
        };

        if sim_settings.pause_countdown > 0.0 {
            sim_settings.pause_countdown -= dt;
            return;
        }

        if !sim_settings.initialized {
            sim_settings.initialized = true;
            let init_bodies_system = world.register_system(initialize_bodies);
            world.run_system(init_bodies_system).unwrap();
        }
    }

    let sim_settings = world.resource::<RigidBodySimulationSettings>();

    if elapsed < sim_settings.startup_time_buffer {
        return;
    }

    for _ in 0..sim_settings.num_substeps {
        world.run_schedule(RigidBodySimulationSchedule);
    }
}
