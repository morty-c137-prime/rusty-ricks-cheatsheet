/**
 * @author Richard Alvarez
 */

// CHAPTER 4.  OWNERSHIP

fn main()
{

	// 1. Each value in Rust has a variable that’s called its owner.
	// 2. There can only be one owner at a time.
	// 3. When the owner goes out of scope, the value will be dropped.
	let q = {
		// s is not valid, non-exsistent
		let s = "hello";
		s // s can be used.
	} // s is dropped

	// stored on heap!
	// str literals are on the stack.
	let mut string = String::from("hello");
	string.push_str(", world!");
}