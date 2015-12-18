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
fn problem0001_validation() {
	assert_eq!(problem0001(10), 23);
}

// 
//Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
//1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
//
pub fn problem0002(upto: u32) -> u32 {
	// I'll try to implement an interator for generating the Fibonacci sequence
	// so we don't have to recurse
	struct Fibonacci {
		curr: u32,
		next: u32,
	}

	impl Iterator for Fibonacci {
		type Item = u32;
		fn next(&mut self) -> Option<u32> {
			let new_next = self.curr + self.next;
			self.curr = self.next;
			self.next = new_next;

			// 'Some' will always be returned, this is strange, but I need to accept it as we need to return Option<T> ;)
			Some(self.curr)
		}
	}

	fn fibonacci() -> Fibonacci {
		Fibonacci { curr: 1, next: 1 }
	}

	// Now we need to:
	// 1. Take numbers while they are smaller than our limit
	// 2. Only keep the even-valued ones
	// 3. Add these up
	fibonacci().take_while(|&x| x < upto)
	           .filter(|&x| x%2==0)
		   .fold(0, |x, acc| acc + x)
}

#[test]
fn problem0002_validation() {
	assert_eq!(problem0002(100), 44);
}

//
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?
//
pub fn problem0003(number: u64) -> u64 {
	extern crate primal;
	unimplemented!();
}

#[test]
fn problem0003_validation() {
	assert_eq!(problem0003(13195), 29);
}
