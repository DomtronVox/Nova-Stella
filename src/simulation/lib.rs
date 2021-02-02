extern crate nalgebra_glm as glm;

//pull in all the ECS registration functions
pub mod physics; 
use physics::{register_physics_ecs_elements, build_physics_dispatcher};


use specs::{World, WorldExt, Dispatcher};


#[derive(Default)]
pub struct DeltaTime(f32);


//holds all info to simulate one solar system
pub struct StarSystemSimulation {
    pub ecs: World,
    physics_dispatcher: Dispatcher<'static, 'static>,
}


impl StarSystemSimulation {

    /// Create a new basic solar system simulation.
    pub fn new() -> Self {

        let mut ecs = World::new();
        register_physics_ecs_elements(&mut ecs);
        build_physics_dispatcher();

        ecs.insert(DeltaTime(0.0));

        StarSystemSimulation{
            ecs,
            physics_dispatcher: build_physics_dispatcher(),
        }
    }

    /// Update all systems for the simulation.
    pub fn update(&mut self, dt:f32) {
        { //update delta time
            let mut delta = self.ecs.write_resource::<DeltaTime>();
            *delta = DeltaTime(dt);
        }

        //update each set of systems.
        self.physics_dispatcher.dispatch(&mut self.ecs);
    }

}



fn createTestEntities(sim: &mut StarSystemSimulation) {

    sim.ecs.create_entity();
    //sim.ecs.create_entity().with();

}
