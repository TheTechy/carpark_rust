use std::time::Instant;

#[derive(Clone)]
pub struct Vehicle {
	pub number_plate:	String,
	pub time_entered:	Instant,
	pub time_elapsed:	f32,
	pub charge_amount:	f32
}

impl Vehicle {
	// Factory pattern object
	pub fn vehicle_enters(num_plate: &String) -> Vehicle {
		Vehicle {number_plate: num_plate.to_string(), time_entered: Instant::now(), time_elapsed:0.00, charge_amount:0.00}
	}

	// fn display_plate(&self) {
	pub fn display_plate(&mut self) -> String {
		return self.number_plate.to_string();
	}

	pub fn calculate_charge(&mut self) -> f32 {
		self.time_elapsed = self.time_entered.elapsed().as_secs_f32();
		if self.time_elapsed > 10.0 {
			self.charge_amount = (self.time_elapsed - 10.0) * 0.01;
		}
		return self.charge_amount;
	}
}