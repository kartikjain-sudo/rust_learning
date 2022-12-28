
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
    fn get_year(&self) -> i32;
}

impl VehicleInfo for Vehicle {
    fn get_vehicle_info(&self) -> String {
        format!("{} {} {}", self.class, self.model, self.year)
    }

    fn get_year(&self) -> i32 {
        self.year
    }
}

fn main() {
    // println!("Hello, world!");
    let car = Vehicle {
        class: String::from("S class"),
        model: String::from("BMW X5"),
        year: 2019,
    };
    print!("{}", car.get_vehicle_info());
}
