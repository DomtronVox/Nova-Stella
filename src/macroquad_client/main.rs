extern crate nalgebra_glm as glm; 

use macroquad::prelude::*;
use megaui_macroquad::draw_megaui;

use nova_stella_sim::StarSystemSimulation;

use specs::{WorldExt, Builder, RunNow};

mod ui_state;
use ui_state::{UIStateMachine, ControlData};

mod orbit_gfx_system;
use orbit_gfx_system::{Satellite, SatelliteGFX, OrbitDisplaySystem};



#[macroquad::main("Nova Stella")]
async fn main() {    
    
    let mut simulation = StarSystemSimulation::new();
    let mut ui_statemachine = UIStateMachine::new();

    //register graphical components
    simulation.ecs.register::<SatelliteGFX>();

    //setup control tracking structure
    simulation.ecs.insert(ControlData::new());
    
    while ui_statemachine.running {

        //update model
        simulation.update(0.1);

        //setup what's needed for drawing
        clear_background(BLACK);

        //to draw the ui we need to reset the camera incase anything else changed it
        set_default_camera();
        ui_statemachine.update(&mut simulation); 
        
        //draw everything
        draw_megaui();
        next_frame().await   
    }

    //TODO run simulation shutdown here.

}
