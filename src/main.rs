use std::io; // for read line
use std::str;

fn main() {

	let mut stage_id = 0u8; // each stage has its own number, a room may have more than one stage.
	loop { // this is the main game loop
		let mut player_input = String::from("");
		if stage_id == 255 {
			println!("Thanks for playing!");
			io::stdin().read_line(&mut player_input).expect("error");
			break;
		};
		print_response(stage_id); // print text based on the stage id
		io::stdin().read_line(&mut player_input).expect("error"); // read player input
		let split = player_input.split(" ");
		let mut ivec = split.collect::<Vec<&str>>();
		ivec.push("filler");
		stage_id = logic_decider(ivec[0].clone().trim(), ivec[1].clone().trim(), stage_id);
	}
}

fn print_response(id: u8) { // refer to this to find out what the stage numbers mean
	match id {
		0 => println!("THE ROOMS\n\nsimple text adventure game in Rust.\n\nPRESS ENTER TO START\n"),
		1 | 2 | 3 | 5 => println!("\nWhat do you do?\n"), // first room
		4 => println!("\nENTER CODE\n"),
		_ => println!("INVALID STAGE ID (this should not happen)")
	};
}

fn logic_decider(w1: &str, w2: &str, id: u8) -> u8 {
	println!();
	match id {
		0 => {println!("\nYou awake on a comfortable bed, but find yourself in a room that you don't recognize.\nIn it is the bed, a beige couch, a tall lamp, and a wooden door on the wall.");
		return 1;}
		1 => match w1 {
			"search" | "look" | "check" | "see" => match w2 {
				"bed" => {println!("There is nothing under the bed.");
				return id;},
				"couch" => {println!("Found a key! It looks like it can open the door.");
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
				"door" => {println!("You insert the key into the door, and it swings open. Beyond the door is a ");
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
			return 5;},
			_ => {println!("CODE INCORRECT");
			return 3;}
		},
		5 => match w1 {
			"search" | "look" | "check" | "see" => match w2 {
				"bomb" => {println!("A bomb with a keypad. It's better not to touch this right now.");
				return id;},
				"briefcase" => {println!("A locked briefcase. There is a keyhole on the outside.");
				return id;},
				"statue" => {println!("A bronze statue. Next to it there is a nameplate that says 'Goliath'. It doesn't seem like it will help with getting out of here.");
				return id;},
				_ => {println!("Can't search that!");
				return id;}
			},
			_ => {println!("Can't do that!");
			return id;}
		},
		_ => {println!("ERROR INVALID STAGE ID, this should not happen");
		return id;}	
	};
}