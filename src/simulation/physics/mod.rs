mod satellite; 
pub use satellite::Satellite;
use satellite::OrbitMotionSystem;



use specs::{World, WorldExt, Dispatcher, DispatcherBuilder};


pub fn register_physics_ecs_elements(ecs: &mut World) {

    ecs.register::<Satellite>();

}

pub fn build_physics_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
    .with(OrbitMotionSystem{}, "Orbit System", &[])
    .build()
}
