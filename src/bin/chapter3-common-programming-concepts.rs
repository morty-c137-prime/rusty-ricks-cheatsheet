/**
 * @author Richard Alvarez
 */

// CHAPTER 3: COMMON PROGRAMMING CONCEPTS

fn main()
{
	// Variables and Mutability
	// In Rust, variables are immutable by default.
	let x = 5;
	println!("The value of x is: {}", x);
	// Uncommenting the line below throws an error.
	// x = 6;
	// println("The value of x is: {}", x);
	
	// This does what you would expect.
	let mut xx = 5;
	xx = 7;
	println!("The value of xx is: {}", xx);

	// The benefist of mutability is simple:
	// Rather than copying large data, one
	// should mutate it. Rather than mutating
	// small data structures one might copy it.
	
	// Only can be set to a constant expression, demand 
	// a type declaration. 
	const god: u32 = 5;

	// Shadowing
	let y = 5;
	let y = y + 1; 
	let y = y * 2;
	// y == 12
	println!("The value of y is: {}", y);
	// The difference between mut and shadowing
	// is that you declare a new variable. This matters
	// when we talk about type.
	let spaces = "   "; // str literal
	let spaces = spaces.len(); // Number

	// Data types
	// All of these are stored on the stack.
	// Scalar type - single value: integers, floats, bools 
	// and chars.
	// Compound type - varient types in one: tuples, arrays 	

	let six = another_function(5);

	// Control Flow
	if six != 0
	{
		println!("Hurdurr");
	}
	else 
	{
		println!("HUWUH?");
	}

	let six = if six == 6
	{
		six + 1;
	}
	else
	{
		six - 1; 
	};

	let a = [1, 2, 3, 4, 5];
	for e in a.iter()
	{
		println!("The value of e is: {}", e);
	} 

	println!("-10 F in C: {}", fahrenheitToCelsius(-10.0));
	println!("-28.89  C in F: {}", celsiusToFahrenheit(-28.89));
	println!("10th fib: {}", fib(10));
}


// Functions
fn another_function(x: i32) -> i32
{
	// Function bodies are full of statements, ending
	// with an expression.
	println!("{}", x);
	x + 1
}

// Practice
fn celsiusToFahrenheit(c: f64) -> f64
{
	c * 1.8 + 32.0
}

fn fahrenheitToCelsius(f: f64) -> f64
{
	(f - 32.0) / 1.8
}

fn fib(n: i32) -> i32
{
	if n <= 1
	{
		n
	}
	else {
		fib(n-1) + fib(n-2)
	}
}