use super::{UIState, StateEvent, UIStateMachine, ControlData, OrbitViewState};
use nova_stella_sim::StarSystemSimulation;

use crate::orbit_gfx_system::{Satellite, SatelliteGFX, OrbitDisplaySystem};

use specs::{WorldExt, Builder};

use macroquad::color::*;

pub struct ConfigureGameState {

}

impl ConfigureGameState {

    pub fn configure_game(&self, simulation: &mut StarSystemSimulation) {
        //TODO In future this should take config info run proc gen stuff and populate simulation with the needed data

        //sun
        simulation.ecs
            .create_entity()
            .with(Satellite::new(2.0e30, glm::vec3(0.,0.,0.), glm::vec3(0.,0.,0.)))
            .with(SatelliteGFX { color: ORANGE })
            .build();
        
        //murcury
        simulation.ecs
            .create_entity()
            .with(Satellite::new(
                3.302e23, 
                glm::vec3(3.035190192722169E-02, 2.790679556868577E-02, 3.074371759775011E-01), 
                glm::vec3(3.364232464219209E-02, 2.943676972041208E-03, -1.742147408904596E-03)
            ))
            .with(SatelliteGFX { color: BLUE })
            .build();

        //venus
        simulation.ecs
            .create_entity()
            .with(Satellite::new(
                3.302e23, 
                glm::vec3(1.406392155647568E-01, -1.790857591635183E-02, -7.136007406362744E-01), 
                glm::vec3(-1.970865346910845E-02, -1.084601457064069E-03, 3.840307496361571E-03)
            ))
            .with(SatelliteGFX { color: RED })
            .build();
    }

}



impl UIState for ConfigureGameState {


    fn ui_logic(&mut self, state_machine: &mut UIStateMachine, sim: &mut StarSystemSimulation) {
    }

    fn state_logic(&mut self, state_machine: &mut UIStateMachine, sim: &mut StarSystemSimulation) {
        self.configure_game(sim);
        state_machine.handle_event(
            StateEvent::ChangeState( Box::new(OrbitViewState::new()) )
        );
    }
}

/*
pub fn new(game_data_path: &str) -> Self {
        let mut asset_db = AssetDatabase::new();

        load_campaign_data(game_data_path, &mut asset_db);


        ecs::register_components(&mut world);
        ecs::create_test_entities(&mut world);

        //insert none ECS data into the world
        world.insert(asset_db); 
        world.insert(ControlData { move_left: false, move_right: false, move_up: false, move_down: false });
*/
