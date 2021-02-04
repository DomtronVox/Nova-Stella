extern crate nalgebra_glm as glm; 

use macroquad::prelude::*;
use megaui_macroquad::draw_megaui;

use nova_stella_sim::StarSystemSimulation;

use specs::{WorldExt, Builder, RunNow};

mod ui_state;
use ui_state::UIStateMachine;

mod orbit_gfx_system;
use orbit_gfx_system::{Satellite, SatelliteGFX, OrbitDisplaySystem};



#[macroquad::main("Nova Stella")]
async fn main() {    
    
    let mut simulation = StarSystemSimulation::new();
    let mut ui_statemachine = UIStateMachine::new();

    simulation.ecs.register::<SatelliteGFX>();

    simulation.ecs
        .create_entity()
        .with(Satellite::new(2.0e30, glm::vec3(0.,0.,0.), glm::vec3(0.,0.,0.)))
        .with(SatelliteGFX { color: ORANGE })
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

    simulation.ecs
        .create_entity()
        .with(Satellite::new(
            3.302e23, 
            glm::vec3(1.406392155647568E-01, -1.790857591635183E-02, -7.136007406362744E-01), 
            glm::vec3(-1.970865346910845E-02, -1.084601457064069E-03, 3.840307496361571E-03)
        ))
        .with(SatelliteGFX { color: RED })
        .build();

    let mut orbit_display_system = OrbitDisplaySystem {};


    let camera = Camera3D {
        position: vec3(-20., 15., 0.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };

    while ui_statemachine.running {

        //update model
        simulation.update(0.1);

        //setup what's needed for drawing
        clear_background(BLACK);

        set_camera(camera);

        draw_grid(20, 1.);

        //dispatch relevant gfx systems for the current context to update
        orbit_display_system.run_now(&simulation.ecs);

        //to draw the ui we need to reset the camera
        set_default_camera();
        ui_statemachine.update(&mut simulation); 
        
        //draw everything
        draw_megaui();
        next_frame().await   
    }

    //TODO run simulation shutdown here.

}
