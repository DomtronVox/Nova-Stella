use specs::{
    System, 
    Component, 
    VecStorage, 
    Read,
    WriteStorage, 
    Join,
    ParJoin,
};
use rayon::prelude::*; //Parallelization of join 


///Component that holds mass and motion data for an orbital body.
///Note that mass should be in Kg and Vectors are in meters.
#[derive(Component)]
#[storage(VecStorage)]
pub struct Satellite {
    pub mass: f32,
    mu: f32, //GM of this object
    pub position: glm::Vec3, //polar coordinates
    pub velocity: glm::Vec3, //polar coordinates
}

impl Satellite {

        //Construct a new Satellite component usinf GLM 3D Vectors for position and Velocity
        pub fn new(mass: f32, initial_position: glm::Vec3, initial_velocity: glm::Vec3) 
          -> Self {
            
            Satellite {
                mass,
                mu: Satellite::calc_mu(mass),
                position: initial_position,
                velocity: initial_velocity,
            }
        }

        ///Allows the mass of a Satellite to be changed, automatically updates Mu.
        pub fn set_mass(&mut self, mass: f32) {
            if mass > 0.0 {
                self.mass = mass;
                self.mu = Satellite::calc_mu(mass);
            }
        }

        ///Calculates the Mu (Grav-const * mass) of a body
        fn calc_mu(mass: f32) -> f32 {
            1.488e-34 * mass //N-m2/kg2 * kg
        }
}


///System that handles advancing the position of all Satellites
pub struct OrbitMotionSystem {

}


impl<'a> System<'a> for OrbitMotionSystem {

    type SystemData = (Read<'a, crate::DeltaTime>,
                       WriteStorage<'a, Satellite>);
                       

    fn run(&mut self, (delta, mut satellite_data): Self::SystemData) {
        let dt = delta.0;

        //First advance all entities along their orbit based on their current velocity.
        {
            (&mut satellite_data)
                .par_join()
                .for_each(|sat| {
                    sat.position += sat.velocity * dt;
                });
        }

        //Next calculate accelerations based on new satalite positions and adjust velocities
        //Note: to avoid double barrows we are using indexes
        let mut satellites = (&mut satellite_data).join().collect::<Vec<_>>();

        for target in 0..satellites.len() {
            let mut velocity_changes = glm::vec3(0.,0.,0.);

            for effector in 0..satellites.len() {
                //Don't calculate acceleration for ourselves
                if target == effector { continue; }

                //calculate gravety's acceleration from "effector" on current orbital body
                

                let distance = glm::distance(&satellites[target].position, 
                                             &satellites[effector].position);


                let unit_vec = glm::normalize( 
                    &( &satellites[effector].position - satellites[target].position ) 
                );


                //Note: GM / r^2 * <unit vec towards effector> 
                //TODO: formulat includes a - in formula but not sure we want it here. If problems occure try it.
                let grav_accel = satellites[effector].mu / (distance*distance) * unit_vec;

                //adds to all the other changes this cycle and the original velocity
                velocity_changes += grav_accel * dt; //* dt to turn accel to velocity

                println!("mass: {}; dist: {}; acceleration: {}; \n delta v: {}",
                         satellites[target].mass, distance, grav_accel, velocity_changes);
            }

            //update our satellite with the velocity change. 
            satellites[target].velocity += velocity_changes;

            println!("velocity: {}", satellites[target].velocity);
        }
    }
}
