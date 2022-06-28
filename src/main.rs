use std::io;

const VERSION: &str = "1.0.0-RC1";

fn main() {
	let mut stage_id: u8 = 0; // each stage has its own number, a room may have more than one stage
	loop { // this is the main game loop
		let mut player_input = String::from("");
		if stage_id == 9 {
			println!("\n\nGAME OVER");
			io::stdin().read_line(&mut player_input).expect("error");
			break;
		};
		if stage_id == 10 {
			println!("\n\nThanks for playing!");
			io::stdin().read_line(&mut player_input).expect("error");
			break;
		};
		print_response(stage_id); // print text based on the stage id
		io::stdin().read_line(&mut player_input).expect("error"); // read player input
		let split = player_input.split(" ");
		let mut ivec = split.collect::<Vec<&str>>();
		ivec.push("filler");
		stage_id = logic_decider(ivec[0].trim(), ivec[1].trim(), stage_id); // takes input and decides what to do next
	}
}

fn print_response(id: u8) {
	match id {
		0 => println!("THE ROOMS\nversion {}\n\nPRESS ENTER TO START\n", VERSION),
		4 => println!("\nENTER CODE\n"),
		8 => println!("Enter the code:\n"),
		_ => println!("\nWhat do you do?\n")
	};
}

fn logic_decider(w1: &str, w2: &str, id: u8) -> u8 {
	println!();
	match id {
		0 => {println!("You awake on a comfortable bed, but find yourself in a room that you don't recognize. In it is the\nbed, a beige couch, a tall lamp, and a wooden door on the wall.\n\nHOW TO PLAY: enter two words to perform an action.\nEXAMPLE: 'search bed'\n\nIf you need a hint you can type 'hint'.\n");
		return 1;}
		1 => match w1 {
			"search" | "look" | "check" | "see" | "examine" => match w2 {
				"bed" => {println!("There is nothing under the bed.");
				return id;},
				"couch" => {println!("You found a key in between the couch cushions. It looks like it can open the door.");
				return 2;},
				"door" => {println!("The door has a silver keyway. It is locked.");
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
			"hint" => {println!("Try searching everything in the room.");
			return id;},
			_ => {println!("Can't do that!");
			return id;}
		},
		2 => match w1 {
			"search" | "look" | "check" | "see" | "examine" => match w2 {
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
				"door" => {println!("You insert the key into the door, and it swings open. Beyond the door is a much smaller and darker\nroom. In the corner there is a small TV that shows the words 'Watch the NEWS'. On the ground is an\narrow that looks to signify north. On the north wall is a metal door, a keypad, and the number 9\nwritten in red paint. Similarly, the east wall has a 3, the south wall has a 2, and the west wall\nhas a 6.");
				return 3;},
				_ => {println!("Can't do that!");
				return id;}
			},
			"hint" => {println!("You can open the door now.");
			return id;},
			_ => {println!("Can't do that!");
			return id;}
		},
		3 => match w1 {
			"search" | "look" | "check" | "see" | "examine" => match w2 {
				"tv" | "television" | "monitor" | "screen" => {println!("The flickering words on the TV read 'Watch the NEWS'. Turning the dials does nothing.");
				return id;},
				"door" => {println!("The door is locked. There is no keyhole.");
				return id;},
				"keypad" | "device" | "numpad" => return 4,
				"wall" | "walls" | "numbers" => {println!("The north wall has a 9, the east wall has a 3, the south wall has a 2 and the west wall has a 6.");
				return id;},
				"north" => {println!("The north wall has a 9.");
				return id;},
				"east" => {println!("The east wall has a 3.");
				return id;},
				"south" => {println!("The south wall has a 2.");
				return id;},
				"west" => {println!("The north wall has a 6.");
				return id;},
				_ => {println!("Can't search that!");
				return id;}
			},
			"hint" => {println!("What could NEWS really stand for?");
			return id;}
			_ => {println!("Can't do that!");
			return id;}
		},
		4 => match w1 {
			"9362" => {println!("CODE CORRECT\n\nYou hear a quiet thud, and the thick metal door swings open. You enter what was once a room, but is\nnow in ruins. The walls are cracked, and there is a huge pile of rubble piled up on one wall. Some light\nseeps through it, illuminating the dark room. On the floor lie a pamphlet, briefcase, and a bomb.\nOn the wall oppsite of the rubble, you see a sturdy locker and a bronze statue.");
			return 5;},
			"hint" => {println!("The code is related to the numbers on the wall and the message on the TV.");
			return id;},
			_ => {println!("CODE INCORRECT");
			return 3;}
		},
		5 => match w1 {
			"search" | "look" | "check" | "see" | "examine" => match w2 {
				"bomb" => {println!("A bomb with a keypad. It's better not to touch this right now.");
				return id;},
				"briefcase" => {println!("A locked briefcase. There is a keyhole on the outside, but the key you found in the first room won't fit.");
				return id;},
				"statue" => {println!("A bronze statue. Next to it there is a nameplate that says 'Goliath'. It doesn't seem like it will\nhelp with getting out of here.");
				return id;},
				"locker" => {println!("There is nothing inside the locker.");
				return id;},
				"rubble" => {println!("A large pile of rubble is packed up on one wall. You can't move it on your own.");
				return id;},
				"pamphlet" => {println!("A pamphlet. It reads:\n\nHOW TO USE THE BOMB\n\n- Enter the code\n- Press START button\n- Wait 30 seconds\n\nAs you pick up the pamphlet, a small key falls out.");
				return 6;},
				_ => {println!("Can't search that!");
				return id;}
			},
			"open" => match w2 {
				"briefcase" => {println!("The briefcase is locked. There is a keyhole on the outside, but the key you found in the first room won't fit.");
				return id;},
				"locker" => {println!("There is nothing inside the locker.");
				return id;},
				_ => {println!("Can't open that!");
				return id;}
			},
			"hint" => {println!("Try searching everything in the room.");
			return id;},
			_ => {println!("Can't do that!");
			return id;}
		},
		6 => match w1 {
			"search" | "look" | "check" | "see" | "examine" => match w2 {
				"bomb" => {println!("A bomb with a keypad. It's better not to touch this right now.");
				return id;},
				"briefcase" => {println!("It looks like the key you found can be used to open the briefcase.");
				return id;},
				"statue" => {println!("A bronze statue. Next to it there is a nameplate that says 'Goliath'. It doesn't seem like it will\nhelp with getting out of here.");
				return id;},
				"locker" => {println!("There is nothing inside the locker.");
				return id;},
				"rubble" => {println!("A large pile of rubble is packed up on one wall. You can't move it on your own.");
				return id;},
				"pamphlet" => {println!("A pamphlet. It reads:\n\nHOW TO USE THE BOMB\n\n- Enter the code\n- Press START button\n- Wait 30 seconds");
				return id;},
				_ => {println!("Can't search that!");
				return id;}
			},
			"open" => match w2 {
				"briefcase" => {println!("You use the key to open the briefcase. In it is a small piece of paper that has the numbers 7355608\nwritten on it.");
				return 7;},
				"locker" => {println!("There is nothing inside the locker.");
				return id;},
				_ => {println!("Can't open that!");
				return id;}
			},
			"hint" => {println!("You can use the key to open the briefcase.");
			return id;},
			_ => {println!("Can't do that!");
			return id;}
		},
		7 => match w1 {
			"search" | "look" | "check" | "see" | "examine" => match w2 {
				"bomb" => {println!("A bomb with a keypad. Now that you know the code you can set it to blow up the pile of rubble. Make\nsure you enter the code carefully!");
				return 8;},
				"briefcase" => {println!("Inside the briefcase is a small piece of paper that has the numbers 7355608 written on it.");
				return id;},
				"statue" => {println!("A bronze statue. Next to it there is a nameplate that says 'Goliath'. It doesn't seem like it will\nhelp with getting out of here.");
				return id;},
				"locker" => {println!("There is nothing inside the locker.");
				return id;},
				"rubble" => {println!("A large pile of rubble is packed up on one wall. You can't move it on your own.");
				return id;},
				"pamphlet" => {println!("A pamphlet. It reads:\n\nHOW TO USE THE BOMB\n\n- Enter the code\n- Press START button\n- Wait 30 seconds");
				return id;},
				_ => {println!("Can't search that!");
				return id;}
			},
			"open" => match w2 {
				"briefcase" => {println!("Inside the briefcase is a small piece of paper that has the numbers 7355608 written on it.");
				return id;},
				"locker" => {println!("There is nothing inside the locker.");
				return id;},
				_ => {println!("Can't open that!");
				return id;}
			},
			"hint" => {println!("Now that you have the code, you can examine the bomb.");
			return id;},
			_ => {println!("Can't do that!");
			return id;}
		},
		8 => match w1 {
			"7355608" => {println!("You carefully set the bomb code and press START. As the bomb counts down, you look for a place to\nhide. The only thing that might work is the locker, so you climb into it. When it reaches zero, you\nsee a flash of white through the cracks before a deafening boom roars through the room. As you step\nout of the locker, you see that the plan worked, and the pile of rubble is gone. You step through\nthe crack and leave this place.");
			return 10;},
			"hint" => {println!("Enter the code you found."); // >:(
			return id;},
			_ => {println!("The bomb gives off a long beep before your world goes white. In your last moments, you realize you entered the code\nwrong.");
			return 9;}
		},
		_ => {println!("ERROR INVALID STAGE ID, this should not happen");
		return id;}	
	}
}