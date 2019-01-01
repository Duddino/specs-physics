use crate::PhysicsWorld;
use amethyst::core::Time;
use amethyst::ecs::{Read, System, WriteExpect};

/// Simulates a step of the physics world.
pub struct PhysicsStepperSystem {
    intended_timestep: f32,
    max_timesteps: i32,
    time_accumulator: f32,
}

impl Default for PhysicsStepperSystem {
    fn default() -> Self {
        PhysicsStepperSystem {
            intended_timestep: 1.0 / 60.0,
            max_timesteps: 10,
            time_accumulator: 0.,
        }
    }
}

impl PhysicsStepperSystem {
    pub fn new(intended_timestep: f32, max_timesteps: i32) -> Self {
        PhysicsStepperSystem {
            intended_timestep,
            max_timesteps,
            time_accumulator: 0.,
        }
    }
}

impl<'a> System<'a> for PhysicsStepperSystem {
    type SystemData = (WriteExpect<'a, PhysicsWorld>, Read<'a, Time>);

    // Simulate world using the current time frame
    fn run(&mut self, (mut physical_world, time): Self::SystemData) {
        if physical_world.timestep() != self.intended_timestep {
            warn!("Physics world timestep out of sync with intended timestep! Leave me alone!!!");
            physical_world.set_timestep(self.intended_timestep);
        }

        self.time_accumulator += time.delta_seconds();
        let mut steps = 0;

        while steps <= self.max_timesteps && self.time_accumulator >= self.intended_timestep {
            physical_world.step();
            self.time_accumulator -= self.intended_timestep;
            steps += 1;
        }
    }
}