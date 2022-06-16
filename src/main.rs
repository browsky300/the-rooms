#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)] // all of these are temporary and are used while the main() function isnt finished, REMOVE THESE LATER

use std::io; // for read line
use regex::Regex; // for splitting player input

fn main() {

	let mut stage_id = 0i8; // each stage has its own number, a room may have more than one stage.
	let mut player_input = String::from("");
	

	print_response(stage_id);
    enter_to_continue();
	stage_id = 1;
	print_response(stage_id); // these four lines are temporary and are incorporated into the main game loop. REMOVE WHEN GAME LOOP IS FINISHED



	/* loop { // this is the main game loop
		if stage_id == 255 { // if game is beaten
			break;
		}
		printResponse(stage_id); // print text based on the stage id
		io::stdin().read_line(&mut player_input).expect("error"); // read player input
		if stage_id == 0 {
			stage_id = 1;
			continue;
		}
		textParser(player_input); // parse it and return a tuple containing 2 words
		stage_id = logicDecider(goob1, goob2, stage_id) // send the stage id and input data to the logic decider which will figure out what the player wanted to say and modify the stage id based on action
	} */
	
}

fn enter_to_continue() {
	{
		let mut goober = String::new(); 
        io::stdin().read_line(&mut goober).expect("error");
		// this is not very good code
	};
}

fn text_parser(input_string: String) {
	
}

fn print_response(n: i8) { // refer to this to find out what the stage numbers mean
	let mut _response = String::from("");
	match n {
		0 => _response = "THE ROOMS\n\nsimple text adventure game in Rust.\n\nPRESS ENTER TO START".to_string(),
		1 => _response = "You awake on a comfortable bed PLACEHOLDER TEXT\n\nWhat do you do?".to_string(),
		2 => _response = "You find a key in between the couch cushions. It looks like it can be used to open the door.\n\nWhat do you do?".to_string(),
		_ => _response = "INVALID STAGE ID".to_string(),
	};
	println!("{}\n", _response);
}

fn logic_decider(w1: &str, w2: &str, id: i8) -> i8 {
	match id {
		1 => match w1 {
			"search" | "look" | "check" => match w2 {
				"bed" => {println!("There is nothing under the bed.");
				return id;},
				"couch" => {return 2i8;},
				_ => {println!("Can't search that!");
				return id;}
			}
			_ => {println!("Can't do that!");
			return id;}
		},
		_ => {println!("INVALID STAGE ID");
		return id;}
	};
}