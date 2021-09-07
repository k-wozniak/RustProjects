fn main() {
    let car1 = Vehicle {
        number: 52,
        acceleration: 1.0,
        max_speed: 70.,
    };

    let car2 = Vehicle {
        number: 1,
        acceleration: 5.0,
        max_speed: 60.,
    };

    let car3 = Vehicle {
        number: 22,
        acceleration: 10.0,
        max_speed: 55.,
    };

    let race_track = Track {
        length: 2000,
    };

    let mut race = Race {
        track: race_track,
        vehicles: vec![car1, car2, car3],
        time_step: 0.01,
    };

    race.simulate_with_dashboard();
}

#[derive(Debug, Clone, Copy)]
struct Vehicle {
    number: i32,
    acceleration: f32,  // ms-2
    max_speed: f32,     // ms-1
}

struct MotionVehicle {
    vehicle: Vehicle,
    current_v: f32,
    current_dis: f32,
    finish_time: Option<f32>,
}

struct Track {
    length: i32,        // m
}

struct Race {
    track: Track,
    vehicles: Vec<Vehicle>,
    time_step: f32,
}


impl MotionVehicle {
    pub fn new(vehicle: Vehicle) -> Self {
        Self {
            vehicle,
            current_v: 0.0,
            current_dis: 0.0,
            finish_time: None,
        }
    }

    fn travel(&mut self, dtime: f32) {
        if self.finish_time == None {
            if self.current_v >= self.vehicle.max_speed {
                self.current_v = self.vehicle.max_speed;
                self.current_dis += dtime * self.current_v;
            } 
            else {
                self.current_dis += self.current_v*dtime + (0.5 * self.vehicle.acceleration * dtime.powi(2));
                self.current_v += self.vehicle.acceleration * dtime;
            }
        }
    }

    fn is_finished(&self, distance: i32) -> bool {
        self.current_dis >= distance as f32
    }
}

impl Race {
    fn simulate_with_dashboard(&mut self) {
        let mut vehicles: Vec<MotionVehicle> = self.vehicles.iter()
            .map(|&v| MotionVehicle::new(v))
            .collect();
        
        let mut time: f32 = 0.00;
        let mut cars_to_finish = self.vehicles.len();
        
        while cars_to_finish > 0 {
            time += self.time_step;

            for v in &mut vehicles {
                v.travel(self.time_step);
                
                if v.finish_time == None {
                    if v.is_finished(self.track.length) {
                        v.finish_time = Some(time);
                        cars_to_finish -= 1;
                    }
                }
            }

            self.print_dashboard(&vehicles, time)
        }
        
        println!("Race Finished.");
    }

    fn print_dashboard(&self, vehicles: &Vec<MotionVehicle>, current_time: f32) {
        std::process::Command::new("clear").status().unwrap(); // Clear the terminal
        
        for v in vehicles {
            let time = match v.finish_time {
                None => current_time,
                Some(t) => t,
            };

            println!("Vehicle: {}\t Time: {:.2}\t D: {:.2}", v.vehicle.number, time, v.current_dis);
        }
    }

    fn simulate(&self) -> Option<&Vehicle> {
        let mut min_time: Option<f32> = None;
        let mut fastests_vehicle: Option<&Vehicle> = None;

        for v in &self.vehicles {
            let time: f32 = (self.track.length as f32) / (v.max_speed as f32);

            if None == min_time || min_time > Some(time) {
                min_time = Some(time.clone());
                fastests_vehicle = Some(v);
            }
        }

        fastests_vehicle
    }
}