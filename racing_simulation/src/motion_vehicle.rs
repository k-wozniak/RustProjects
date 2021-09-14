use crate::vehicle::Vehicle;

pub struct MotionVehicle {
    pub vehicle: Vehicle,
    pub current_v: f32,
    pub current_dis: f32,
    pub finish_time: Option<f32>,
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

    pub fn travel(&mut self, dtime: f32) {
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

    pub fn is_finished(&self, distance: i32) -> bool {
        self.current_dis >= distance as f32
    }
}