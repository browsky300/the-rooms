use std::io; // for read line
use regex::Regex; // for splitting player input

fn main() {

	let mut stage_id = 0i8; // each stage has its own number, a room may have more than one stage.
	let mut player_input = String::from("");

	print_response(stage_id);
    enter_to_continue();

	stage_id = 1;
	print_response(stage_id);
	io::stdin().read_line(&mut player_input).expect("error");
	text_parser(player_input);
	
}

fn enter_to_continue() {
	{
		let mut goober = String::new(); 
        io::stdin().read_line(&mut goober).expect("error");
		// this is not very good code
	}
}

#[allow(unused_variables)]
fn text_parser(input_string: String) {
	
}

fn print_response(n: i8) {
	let mut _response = String::from("");
	match n {
		0 => _response = "THE ROOMS\n\nsimple text adventure game in Rust.\n\nPRESS ENTER TO START".to_string(),
		1 => _response = "You awake on a comfortable bed PLACEHOLDER TEXT\n\nWhat do you do?".to_string(),
		2 => _response = "You have found the key to open the door.\n\nWhat do you do?".to_string(),
		_ => _response = "INVALID STAGE ID".to_string(),
	}
	println!("{}\n", _response);
}