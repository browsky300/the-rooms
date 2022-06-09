use std::io;

fn main() {
    println!("THE ROOMS\n\nsimple text adventure game in Rust.\n\nPRESS ENTER TO START");
    {
        let mut goober = String::new();
        io::stdin().read_line(&mut goober).expect("error 1");

    }
}
