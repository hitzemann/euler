//
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.
//
pub fn problem0001(upto: u32) -> u32 {
	use std::collections::HashMap;
	// For larger problems I assume it is easier
	// to generate all multiples of 3 and 5 instead
	// of checking 1..upto for x%{3,5}==0
	// Storing the multiples as keys in a hashmap
	// makes sure we don't count multiples of 3 AND 5
	// twice e.g. 15
	let mut multiples = HashMap::new();
	for x in 1..upto/3+1 {
		if upto > x*3 {
			multiples.insert(x*3,1);
		}
		if upto > x*5 {
			multiples.insert(x*5,1);
		}
	}
	println!("{:?}",multiples);
	// Some funky syntax I remember from Haskell and Elixir
	multiples.keys().fold(0, |x, acc| acc+x)
}

#[test]
fn problem0001_10() {
	assert_eq!(problem0001(10), 23);
}
