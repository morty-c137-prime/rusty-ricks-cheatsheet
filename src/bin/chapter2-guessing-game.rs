/**
 * @author Richard Alvarez
 */

// CHAPTER 2.  GUESSING GAME
// I will take notes on these chapters, however
// this one is pretty straight forward.
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main()
{
	println!("Guess the nuber!");
	
	let secret_number = rand::thred_rng().gen_range(1, 101);

	loop 
	{
		println!("Please input your guess.");

		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
		.expect("Failed to read line...");

		let guess: u32 = guess.trim().parse() 
		{
			Ok(num) => num,
			Err(_) => continue,
		}
		println!("You guessed: {}", guess);

		match secret_number.cmp(&guess) 
		{
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => 
			{
				println!("Just right, you win!"),
				break;
			}
		}
	}
}