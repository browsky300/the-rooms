#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)] // all of these are temporary and are used while the main() function isnt finished, REMOVE THESE LATER

use std::io; // for read line

fn main() {

	let mut stage_id = 0i8; // each stage has its own number, a room may have more than one stage.
	let mut player_input = String::from("");



	loop { // this is the main game loop
		if stage_id == 255 { // if game is beaten
			break;
		}
		print_response(stage_id); // print text based on the stage id
		io::stdin().read_line(&mut player_input).expect("error"); // read player input
		if stage_id == 0 { // bypass the game logic flow if on the title screen
			stage_id = 1;
			continue;
		}
		// text_parser(player_input); // i need to figure out how to make this work
		stage_id = logic_decider("goob1", "goob2", stage_id); // send the stage id and input data to the logic decider which will figure out what the player wanted to say and modify the stage ID based on action
	}
	
}

fn enter_to_continue() {
	{
		let mut goober = String::new(); 
        io::stdin().read_line(&mut goober).expect("error");
		// this is not very good code
	};
}

fn text_parser(input_string: String) {
	let mut result = input_string.split_whitespace();
}

fn print_response(n: i8) { // refer to this to find out what the stage numbers mean
	let mut _response = String::from("");
	match n {
		0 => _response = String::from("THE ROOMS\n\nsimple text adventure game in Rust.\n\nPRESS ENTER TO START"),
		1 => _response = String::from("You awake on a comfortable bed PLACEHOLDER TEXT\n\nWhat do you do?"),
		2 => _response = String::from("You find a key in between the couch cushions. It looks like it can be used to open the door.\n\nWhat do you do?"),
		3 => _response = String::from("You use the key you found and open the door PLACEHOLDER TEXT WATCH THE NEWS"),
		_ => _response = String::from("INVALID STAGE ID")
	};
	println!("{}\n", _response);
}

fn logic_decider(w1: &str, w2: &str, id: i8) -> i8 {
	match id {
		1 => match w1 {
			"search" | "look" | "check" | "see" => match w2 {
				"bed" => {println!("There is nothing under the bed.");
				return id;},
				"couch" => {return 2i8;},
				"door" => {println!("The door is locked.");
				return id;},
				"lamp" => {println!("The yellow light of the lamp illuminates the room. Unfortunately, there is no way to turn it off.");
				return id;},
				_ => {println!("Can't search that!");
				return id;}
			}
			"open" => match w2 {
				"door" => {println!("The door is locked.");
				return id;}
				_ => {println!("Can't do that!");
				return id;}
			_ => {println!("Can't do that!");
			return id;}
			}
		},
		2 => match w1 {
			"search" | "look" | "check" | "see" => match w2 {
				"bed" => {println!("There is nothing under the bed.");
				return id;},
				"couch" => {println!("There is nothing else in the couch cushions.");
				return id;},
				"door" => {println!("It looks like the key you found fits the door.");
				return id;},
				"lamp" => {println!("The yellow light of the lamp illuminates the room. Unfortunately, there is no way to turn it off.");
				return id;},
				_ => {println!("Can't search that!");
				return id;}
			},
			"open" => match w2 {
				"door" => {return 3;},
				_ => {println!("Can't do that!");
				return id;}
			}
			_ => {println!("Can't do that!");
			return id;}
		},
		3 => match w1 {
			"search" | "look" | "check" | "see" => match w2 {
				"tv" | "television" | "monitor" => {println!("The flickering words on the TV read 'Watch the NEWS'. Turning the dials does nothing.");
				return id;},
				"door" => {println!("The door is locked. There is no keyhole.");
				return id;},
				"keypad" | "device" | "numpad" => {return 4;}
			},
			_ => {println!("Can't do that!");
			return id;}
		},
		4 => match w1 {
			"enter" => match w2 {
				"9362" => {return 5;},
				_ => {println!("INCORRECT CODE");
				return 3;}
			},
			_ => {println!("error this should not happen"); // this should be autofilled so if this condition is triggered something bad happened
			return id;}
		},
		_ => {println!("error this should not happen");
		return id;}
	};
	return 0;
}