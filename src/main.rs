#![allow(dead_code)]
#![allow(unused_variables)] // all of these are temporary and are used while the main() function isnt finished, REMOVE THESE LATER

use std::io; // for read line
use std::str;

fn main() { // real

	let mut stage_id = 0u8; // each stage has its own number, a room may have more than one stage.
	let mut player_input = String::from("");
	let mut parsed_input_arr = [String::new(), String::new()];
	let mut word1 = String::new()
	let mut word2 = String::new()
	//let mut return_array



	loop { // this is the main game loop
		if stage_id == 255 {
			break;
		};
		print_response(stage_id); // print text based on the stage id
		io::stdin().read_line(&mut player_input).expect("error"); // read player input
		parsed_input_arr = text_parser(player_input.clone(), stage_id);
		word1 = parsed_input_arr[0].clone();
		word2 = parsed_input_arr[1].clone();
		println!("word 1 is ({}), word2 is ({})", word1, word2);
		stage_id = logic_decider(word1.as_str(), word1.as_str(), stage_id); // send the stage id and input data to the logic decider which will figure out what the player wanted to say and modify the stage ID based on action
	}

	println!("Thanks for playing!");
	enter_to_continue();
	
}

fn enter_to_continue() {
	{
		let mut goober = String::new(); 
        io::stdin().read_line(&mut goober).expect("error");
		// this is not very good code
	};
}

fn text_parser(input_ref: String, stage_id: u8) -> [String; 2] {
	let input_copy = input_ref.clone();
	let split = input_copy.split(" ");
	let mut input_vec: Vec<&str> = split.collect(); // this is bad but i dont care
	input_vec.push("invalid");
	input_vec.push("invalid");
	return [String::from(input_vec[0]), String::from(input_vec[1])];
}

fn print_response(n: u8) { // refer to this to find out what the stage numbers mean
	match n {
		0 => println!("THE ROOMS\n\nsimple text adventure game in Rust.\n\nPRESS ENTER TO START"),
		1 => println!("\nYou awake on a comfortable bed PLACEHOLDER TEXT\n\nWhat do you do?"), // first room
		2 => println!("\nYou find a key in between the couch cushions. It looks like it can be used to open the door.\n\nWhat do you do?"),
		3 => println!("\n PLACEHOLDER TEXT WATCH THE NEWS\n\nWhat do you do?"), // second room
		4 => println!("\nENTER CODE\n\n"),
		5 => println!("\nTHIRD ROOM PLACEHOLDER TEXT\n\n"),
		_ => println!("INVALID STAGE ID")
	};
}

fn logic_decider(w1: &str, w2: &str, id: u8) -> u8 {
	println!();
	match id {
		0 => return 1,
		1 => match w1 {
			"search" | "look" | "check" | "see" => match w2 {
				"bed" => {println!("There is nothing under the bed.");
				return id;},
				"couch" => {println!("Found a key!");
				return 2;},
				"door" => {println!("The door is locked.");
				return id;},
				"lamp" => {println!("The yellow light of the lamp illuminates the room. Unfortunately, there is no way to turn it off.");
				return id;},
				_ => {println!("Can't search that!");
				return id;}
			},
			"open" => match w2 {
				"door" => {println!("The door is locked.");
				return id;},
				_ => {println!("Can't do that!");
				return id;}
			},
			_ => {println!("Can't do that!");
			return id;}
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
				"door" => {println!("You insert the key into the door, and it swings open.");
				return 3;},
				_ => {println!("Can't do that!");
				return id;}
			},
			_ => {println!("Can't do that!");
			return id;}
		},
		3 => match w1 {
			"search" | "look" | "check" | "see" => match w2 {
				"tv" | "television" | "monitor" | "screen" => {println!("The flickering words on the TV read 'Watch the NEWS'. Turning the dials does nothing.");
				return id;},
				"door" => {println!("The door is locked. There is no keyhole.");
				return id;},
				"keypad" | "device" | "numpad" => return 4,
				_ => {println!("Can't search that!");
				return id;}
			},
			_ => {println!("Can't do that!");
			return id;}
		},
		4 => match w1 {
			"9362" => {println!("CODE CORRECT\n\nYou hear a quiet thud, and the thick metal door swings open.");
			return 255;}, // temp id to stop the game, i want to write text parser first
			_ => {println!("CODE INCORRECT");
			return 3;}
		},
		_ => {println!("ERROR INVALID STAGE ID, this should not happen");
		return id;}
		5 => match w1 {
			"search" | "look" | "check" | "see" => match w2 {
				"bomb" => {println!("A bomb with a keypad. It's better not to touch this right now.");
				return id;}
				"briefcase" => {println!("A locked briefcase. There is a keyhole on the outside.");
				return id;}
				"statue" => {println!("A bronze statue. Next to it there is a nameplate that says 'Goliath'. It doesn't seem like it will help with getting out of here.");
				return id;},

			}
		}
	};
}