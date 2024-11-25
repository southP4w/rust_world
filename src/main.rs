mod person;

use crossterm::event::{poll, read, Event::Key, KeyCode::F};
use person::Person;
use std::{io::Result, time::Duration};

fn main() -> Result<()> {
	let dave = Person::new("Dave", "01011989");

	println!("{}", dave);
	dave.say(Some(&format!("Hi! My name is {}.", dave.name())));

	loop {
		if poll(Duration::from_millis(500))? {
			if let Key(key_event) = read()? {
				if key_event.code == F(4) {
					break;
				}
			}
		}
	}

	Ok(())
}
