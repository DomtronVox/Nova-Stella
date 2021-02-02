extern crate nalgebra_glm as glm; 

use macroquad::prelude::*;

use nova_stella_sim::StarSystemSimulation;

use specs::{WorldExt, Builder, RunNow};

mod orbit_gfx_system;
use orbit_gfx_system::{Satellite, SatelliteGFX, OrbitDisplaySystem};



#[macroquad::main("Nova Stella")]
async fn main() {    
    
    let mut simulation = StarSystemSimulation::new();

    simulation.ecs.register::<SatelliteGFX>();

    simulation.ecs
        .create_entity()
        .with(Satellite::new(2.0e30, glm::vec3(0.,0.,0.), glm::vec3(0.,0.,0.)))
        .with(SatelliteGFX { color: RED })
        .build();

    simulation.ecs
        .create_entity()
        .with(Satellite::new(
            3.302e23, 
            glm::vec3(3.035190192722169E-02, 2.790679556868577E-02, 3.074371759775011E-01), 
            glm::vec3(3.364232464219209E-02, 2.943676972041208E-03, -1.742147408904596E-03)
        ))
        .with(SatelliteGFX { color: BLUE })
        .build();

    let mut orbit_display_system = OrbitDisplaySystem {};


    set_camera(Camera3D {
        position: vec3(-20., 15., 0.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    });

    loop {

        //update model
        simulation.update(0.1);

        //setup what's needed for drawing
        clear_background(BLACK);


        draw_grid(20, 1.);

        //draw_sphere(vec3(0., 0., 0.), 1., None, RED);

        //dispatch relevant gfx systems for the current context to update
        orbit_display_system.run_now(&simulation.ecs);
        
        //draw everything
        next_frame().await   
    }

    loop {
        //dispatch relevant gfx systems for the current context to update
        orbit_display_system.run_now(&simulation.ecs);

        draw_grid(20, 1.);

        next_frame().await
    }
}
