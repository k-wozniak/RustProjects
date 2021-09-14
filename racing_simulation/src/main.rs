pub mod vehicle;
pub mod track;
pub mod race;
pub mod motion_vehicle;

use crate::vehicle::Vehicle;
use crate::track::Track;
use crate::race::Race;
use crate::motion_vehicle::MotionVehicle;

fn main() {
    let car1 = Vehicle {
        name: String::from("Williams"),
        number: 52,
        acceleration: 10.0,
        max_speed: 70.,
    };

    let car2 = Vehicle {
        name: String::from("Mclaren"),
        number: 1,
        acceleration: 15.0,
        max_speed: 60.,
    };

    let car3 = Vehicle {
        name: String::from("Ford   "),
        number: 22,
        acceleration: 20.0,
        max_speed: 55.,
    };

    let race_track = Track {
        length: 1000,
    };

    let mut race = Race {
        track: race_track,
        vehicles: vec![
            MotionVehicle::new(car1),
            MotionVehicle::new(car2),
            MotionVehicle::new(car3),
            ],
        time_step: 0.01,
    };

    race.simulate_with_dashboard();
}