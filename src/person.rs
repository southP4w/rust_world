use chrono::{Datelike, Utc};
use std::fmt;

#[derive(Debug)]
pub struct Person {
	name: String,
	date_of_birth: String,
}

impl Person {
	pub fn new(name: &str, date_of_birth: &str) -> Self {
		Person {
			name: name.to_string(),
			date_of_birth: date_of_birth.to_string(),
		}
	}

	pub fn say(&self, text: Option<&str>) {
		// Match on the provided optional text
		match text {
			// If there is some text, and it's not empty, format the text to fit lines of max length 100:
			Some(t) if !t.is_empty() => {
				let mut formatted_text = String::new(); // Initialize an empty String to store formatted output
				let words: Vec<&str> = t.split_whitespace().collect(); // Split the text into words by whitespace
				let mut current_line_length = 0; // Keep track of the current line length
				for word in words {
					if current_line_length + word.len() + 1 > 100 {
						// If adding this word would exceed 100 characters:
						formatted_text.push_str("\n\t"); // Start a new line with indentation
						current_line_length = 0; // Reset the line length counter
					} else if current_line_length > 0 {
						// If the current line is not empty:
						formatted_text.push(' '); // Add a space before the next word
						current_line_length += 1; // Account for the added space
					}
					formatted_text.push_str(word); // Add current word to the formatted text
					current_line_length += word.len(); // Update line length with word length
				}
				println!("[{}]:\n\t\"{}\"\n", self.name, formatted_text); // Print formatted text
			}
			_ => {
				println!("[{}]:\n\t{} says nothing.\n", self.name, self.name);
			}
		}
	}

	pub fn calculate_age(&self) -> i32 {
		let birth_year: i32 = self.date_of_birth[4..8].parse::<i32>().unwrap_or(0);
		let current_year: i32 = Utc::now().year();
		current_year - birth_year
	}

	pub fn name(&self) -> &str {
		&self.name
	}
	pub fn date_of_birth(&self) -> &str {
		&self.date_of_birth
	}
	pub fn set_name(&mut self, name: String) {
		self.name = name;
	}
	pub fn set_date_of_birth(&mut self, date_of_birth: String) {
		self.date_of_birth = date_of_birth;
	}
}

impl fmt::Display for Person {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{{{}}}:\n\tAge: {}", self.name, self.calculate_age())
	}
}