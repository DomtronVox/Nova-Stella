use super::{UIState, StateEvent, UIStateMachine};
use nova_stella_sim::StarSystemSimulation;
use super::control_data::ControlData;

//use crate::assets::{AssetDatabase, AssetContainer, load_campaign_data};

use crate::orbit_gfx_system::OrbitDisplaySystem;


use specs::{WorldExt, RunNow};

use macroquad::prelude::*;
use macroquad::{
    models::draw_grid,
    camera::{Camera3D, set_camera},
    input::{is_key_down, KeyCode},
};




pub struct OrbitViewState {
    //various system managers that run systems added to them in parrellel
    //render_dispatcher: Dispatcher<'static, 'static>,
    //input_dispatcher: Dispatcher<'static, 'static>,

    //TODO should likely use a dispatcher in the future.
    orbit_display_system: OrbitDisplaySystem,
}

impl OrbitViewState {

    pub fn new() -> Self {
        OrbitViewState {
            orbit_display_system: OrbitDisplaySystem {},
        }
    }
 
}

impl UIState for OrbitViewState {


    fn ui_logic(&mut self, state_machine: &mut UIStateMachine, sim: &mut StarSystemSimulation) {
        //update all systems related to rendering
        //self.render_dispatcher.dispatch(&mut world);
        set_camera(Camera3D {
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });
        
        draw_grid(20, 1.);

        self.orbit_display_system.run_now(&sim.ecs);

    }

    fn state_logic(&mut self, state_machine: &mut UIStateMachine, sim: &mut StarSystemSimulation) {

        //closure is needed so control_data can go out of scope and be barrowed again 
        //  when running the system later
        {
            let mut control_data = sim.ecs.write_resource::<ControlData>();

            //query microquad for key presses        
            control_data.move_left  = is_key_down(KeyCode::A) || is_key_down(KeyCode::Left);
            control_data.move_right = is_key_down(KeyCode::D) || is_key_down(KeyCode::Right);
            control_data.move_up = is_key_down(KeyCode::W) || is_key_down(KeyCode::Up);
            control_data.move_down = is_key_down(KeyCode::S) || is_key_down(KeyCode::Down);
        }

        //run all systems related to input handling
        //self.input_dispatcher.dispatch(&mut world);
    }

}
