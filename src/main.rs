
struct Vehicle {
    class: String,
    model: String,
    year: i32,
}

trait VehicleInfo {
    fn new(&mut self, class: String, model: String, year: i32) -> Vehicle {
        Vehicle {
            class,
            model,
            year,
        }
    }
    fn get_vehicle_info(&self) -> String;
    fn get_year(&self) -> i32 {
        self.year
    }
}

impl VehicleInfo for Vehicle {
    fn get_vehicle_info(&self) -> String {
        format!("{} {} {}", self.class, self.model, self.year)
    }
}

fn main() {
    println!("Hello, world!");
}
