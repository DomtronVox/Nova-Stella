use specs::{
    System, 
    Component, 
    VecStorage, 
    ReadStorage, 
    Join,
};

use macroquad::{
    models::draw_sphere,
    color::Color,
    math::Vec3,
};

pub use nova_stella_sim::physics::Satellite;


///Component that holds graphical information for an orbital body.
///Note that mass should be in Kg and Vectors are in meters.
#[derive(Component)]
#[storage(VecStorage)]
pub struct SatelliteGFX {
    //model: ModelType, //aka sphere for planets, random convex shape for asteroids/etc, or decal for ships/stations
    //texture: ?, //Texture to apply to the above model.
    pub color: Color,
}


///System that handles advancing the position of all Satellites
pub struct OrbitDisplaySystem {

}

impl<'a> System<'a> for OrbitDisplaySystem {

    type SystemData = (ReadStorage<'a, Satellite>,
                       ReadStorage<'a, SatelliteGFX>);
                       

    fn run(&mut self, (physics, graphics): Self::SystemData) {

        for (physical, graphic) in (&physics, &graphics).join() {
            //macroquad uses GLM instead of nalgebra_glm so we have to translate types
            let position = Vec3::new(physical.position.x*10.,
                                     physical.position.y*10.,
                                     physical.position.z*10.);

            draw_sphere(position, 1., None, graphic.color);

        }
    
    }
}
