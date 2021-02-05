use super::{UIState, StateEvent, UIStateMachine, ConfigureGameState};
use nova_stella_sim::StarSystemSimulation;



use macroquad::{
    math::vec2,
    window::{screen_width, screen_height}
};
use megaui_macroquad::{
    draw_window,
    megaui::{hash, Vector2},
    WindowParams,
};

/*
use super::playing_state::PlayingState;

use crate::assets::audio::{AudioClip, ClipCategory, Playlist};
use std::path::{PathBuf, Path};
*/

pub struct MainMenuState {
    //music_playlist: Playlist,
}

impl MainMenuState {


    pub fn new() -> MainMenuState {
        //TODO building this playlist is a bit rough. Refine it via a loading function for builtin resources.
        /*let clip1_path: PathBuf = ["builtin", "eclipse.mp3"].iter().collect();
        let clip1 = AudioClip::new( clip1_path, ClipCategory::Music );

        let clip2_path: PathBuf = ["builtin", "in-love.mp3"].iter().collect();
        let clip2 = AudioClip::new(clip2_path, ClipCategory::Music );

        let playlist = Playlist::new(vec![clip1, clip2]);
*/
        MainMenuState {
            //music_playlist: playlist,
        }
    }

}

impl UIState for MainMenuState {

    fn ui_logic(&mut self, state_machine: &mut UIStateMachine, sim: &mut StarSystemSimulation) {
        let main_menu_id = hash!();
        
        //calculate center position
        let menu_size = vec2(512., 80.);
        let menu_pos = vec2(
            (screen_width()  / 2.) - (menu_size.x / 2.), 
            60.
        );
        
        draw_window( main_menu_id, menu_pos, menu_size,
            WindowParams {
                label: "Main Menu".to_string(),
                close_button: false,
                titlebar: false,
                movable: false,
            },
            |ui| {
                //keep main menu window centered
                //TODO frankly very confused why this is needed. We provide a new pos each time so it should be needed.
                ui.move_window(main_menu_id, Vector2::new(menu_pos.x, menu_pos.y));
                
                if ui.button(None, "Start New Game") {  
                    let new_state = ConfigureGameState{};
                    state_machine.handle_event(
                        StateEvent::ChangeState( Box::new(new_state) )
                    );
                }
                ui.button(None, "Load");
                if ui.button(None, "Quit") { 
                    state_machine.handle_event(StateEvent::Shutdown); 
                }                
            },
        );

        //keep menu music playing
        //self.music_playlist.maintain_looping();
    }


    
    fn state_logic(&mut self, state_machine: &mut UIStateMachine, sim: &mut StarSystemSimulation) {
        
    }
}
