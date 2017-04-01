use std::io;
use std::io::prelude::*;
use std::io::stdout;

fn main() {
	loop {
		println!("Hello, welcome to Rust21\nWould you like to play? ");
		println!("1 - Start Game");
		println!("2 - Help");
		println!("3 - Exit");
		print!(">");
		let _ = stdout().flush();
		let mut input = String::new();
		let _ = io::stdin().read_line(&mut input);
		match input.trim() {
			"1" =>  println!("Lets start the game."),
			"2" =>  println!("This is how to play...."),
			"3" =>  break,
			_ =>  println!("'{}' is invalid, please enter 1, 2 or 3.", input.trim())
			}
	}
}

/*
 * interface
 * 		card module
 * 			card
 * 				stores card details
 * 			deck
 * 				shuffle
 * 		game module
 * 			turn loop
 * Welcome!
 * 1 - start game
 * 2 - help
 * 3 - exit
 * > 
 */
