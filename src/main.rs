// CARPARK
// There is a carpark with 10 spaces
// Log each vehicle license plate as it enters
// Allow 10 seconds of free time within the carpark before starting to charge at £0.01 per second
// Parking bays are allocated sequentially 0..10
// Keep track of how many space are free and if all 10 are full, display a message saying full and disallow entry
// When a car exits, generate an invoice for time spent and display on screen
// Applications has three options
// Q: Quit application
// 1: Vehicle enters
// 2: Vehicle leaves
// 3: List vehicles with time parked and current cost for each

/// STD IMPORTS
// use std::{io::{self, Write}, vec};
use std::io::{self, Write};

/// CUSTOM IMPORTS
mod clear_screen;
mod vehicle;
use crate::vehicle::Vehicle;

fn main_menu(num_vehicles: u8) {
	// Clear the screen
	clear_screen::clr();
	println!("********************");
	println!("***** CAR PARK *****");
	println!("********************");
	println!("** Parked count: {}", num_vehicles);
	println!("********************\n");
	println!("******* MENU *******");
	println!("q: Quit");
	println!("1: Vehicle IN");
	println!("2: Vehicle OUT");
	println!("3: List");
	println!("Enter a menu item:");
	io::stdout().flush().unwrap();
}

fn main() {
	// Holds details of vehicle structs
	let mut vehicles: Vec<vehicle::Vehicle> = Vec::new();

	//Application loop
	loop {
		// User input for 
		let mut menu_selection = String::new();

		// Display the menu and wait for input
		main_menu(vehicles.len().try_into().unwrap());

		io::stdin()
			.read_line(&mut menu_selection)
			.expect("Failed to read line");
		let menu_letter = menu_selection.chars().nth(0).unwrap();
		match menu_letter {
			'q' => {
				println!("Quitting... bye");
				return
			},
			'1' => { // Incomming vehicle
				println!("Enter the vehicle license plate:");
				let mut incomming_vehicle = String::new();
				io::stdin()
					.read_line(&mut incomming_vehicle)
					.expect("Failed to read line");
			
				vehicles.push(Vehicle::vehicle_enters(&incomming_vehicle));
			},
			'2' => { // Outgoing vehicle			
				clear_screen::clr();
				let mut vehicle_selection = String::new();
				let vehicle_to_charge: u32;
				println!("************************************");
				println!("**** CHOOSE A VEHICLE TO CHARGE ****");
				println!("#  REG\t\t\t£");
				for i in 0..vehicles.len() {
					println!("{}  {}\t\t\t{:>0.2}", i, &vehicles[i].display_plate().trim(), &vehicles[i].calculate_charge());
				}

				println!("Enter a number: (e.g. 1): ");
				io::stdin()
					.read_line(&mut vehicle_selection)
					.expect("Failed to read line");

				vehicle_to_charge = vehicle_selection
				.trim()
				.parse()
				.expect("Wanted a number");

				vehicles.remove(vehicle_to_charge.try_into().unwrap());
			}
			'3' => { // List vehicles
				clear_screen::clr();
				let mut tmp = String::new();
				println!("**** VEHICLES ****");
				print!("REG\t\t\t£\n");
				for cars in &mut vehicles {
					// io::s`tdout().flush().unwrap();
					print!("REG: {}\t\t£{:>0.2}\n", cars.display_plate().trim(), cars.calculate_charge());
				}
				println!("**** ENTER TO CONTINUE ****");
				io::stdin()
					.read_line(&mut tmp)
					.expect("Failed to read line");
			},
			_ => { // Eh!
				println!("Incorrect selection:");
				println!("**** ENTER TO CONTINUE ****");
					io::stdin()
					.read_line(&mut menu_selection)
					.expect("Failed to read line");
				}
		}
	}
}