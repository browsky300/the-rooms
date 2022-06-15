use std::io;

fn main() {

	let mut stageID = 0i8;

	
    println!("THE ROOMS\n\nsimple text adventure game in Rust.\n\nPRESS ENTER TO START");
    enter_to_continue();
	println!("You awake on a comfortable bed PLACEHOLDER TEXT\n\nWhat do you do?");
	
}

fn enter_to_continue() {
	{
		let mut goober = String::new(); // goofy goober code
        io::stdin().read_line(&mut goober).expect("error 1");
		// i am not very good at rust
	}
}

fn text_parser(inputString) {

}