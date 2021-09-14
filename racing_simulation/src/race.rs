use crate::track::Track;
use crate::motion_vehicle::MotionVehicle;

pub struct Race {
    pub track: Track,
    pub vehicles: Vec<MotionVehicle>,
    pub time_step: f32,
}

impl Race {
    pub fn simulate_with_dashboard(&mut self) {
        let mut time: f32 = 0.00;
        let mut cars_to_finish = self.vehicles.len();
        
        while cars_to_finish > 0 {
            time += self.time_step;

            for v in &mut self.vehicles {
                v.travel(self.time_step);
                
                if v.finish_time == None {
                    if v.is_finished(self.track.length) {
                        v.finish_time = Some(time);
                        cars_to_finish -= 1;
                    }
                }
            }

            self.print_dashboard(&self.vehicles, time)
        }
        
        println!("Race Finished.");
    }

    pub fn print_dashboard(&self, vehicles: &Vec<MotionVehicle>, current_time: f32) {
        std::process::Command::new("clear").status().unwrap(); // Clear the terminal
        
        for v in vehicles {
            let time = match v.finish_time {
                None => current_time,
                Some(t) => t,
            };

            println!("Vehicle: {}\t Number: {}\t Time: {:.2}\t D: {:.2}",
                v.vehicle.name, v.vehicle.number, time, v.current_dis);
        }
    }
}