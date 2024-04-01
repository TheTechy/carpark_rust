/// Clears the termnal
pub fn clr() {
	// clear the screen
	std::process::Command::new("clear").status().unwrap();
}