use bevy::{prelude::*, ecs::schedule::ScheduleLabel};



#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PhysicsSimulationSchedule;


#[derive(Resource)]
pub struct PhysicsSimulationSettings {
    pub num_substeps: u32,

    pub sub_dt: f32,
}

impl Default for PhysicsSimulationSettings {
    fn default() -> Self {
        Self {
            num_substeps: 4,
            
            sub_dt: 0.0,
        }
    }
}


pub fn run_physics_sim_schedule(world: &mut World) {
    let time = world.resource::<Time>();
    let dt = time.delta_seconds();
    {
        let mut sim_settings = world.resource_mut::<PhysicsSimulationSettings>();
        sim_settings.sub_dt = dt / sim_settings.num_substeps as f32;
    }
    let sim_settings = world.resource::<PhysicsSimulationSettings>();
    
    for _ in 0..sim_settings.num_substeps {
        world.run_schedule(PhysicsSimulationSchedule);
    }
}