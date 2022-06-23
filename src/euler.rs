//
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.
//
#[cfg(not(alt))]
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
	// Some funky syntax I remember from Haskell and Elixir
	multiples.keys().fold(0, |x, acc| acc+x)
}

#[cfg(alt)]
pub fn problem0001(upto: u32) -> u32 {
	let res = (1..upto).filter(|x| x % 3 == 0 || x % 5 == 0).fold(0, |x, acc| x + acc);
	res
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
	let limit: usize = number as usize;
	// StreamingSieve would be more memory efficient, but does not have a factor() method
	let sieve = primal::Sieve::new(100000);
	// We are only interested in the last prime factor as it will be the largest
	// the second part of the vector does not interest in this problem
	let (x, _) = sieve.factor(limit).unwrap().pop().unwrap();
	x as u64
}

#[test]
fn problem0003_validation() {
	assert_eq!(problem0003(13195), 29);
}

//
// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.
//
pub fn problem0004(digits: u32) -> u64 {
	let max: u64=10u64.pow(digits)-1;
	let mut res = 0;
	let mut done = false;
	let mut x = max;
	while !done {
		res = x*max;
		if is_palindrome_0004(res) {
			done = true;
		} 
		x -= 1;
	}
	res
}

#[test]
fn problem0004_validation() {
	assert_eq!(problem0004(2), 9009);
}

fn is_palindrome_0004(digits: u64) -> bool {
	let string: String = digits.to_string();
	let backwards: String = string.chars().rev().collect();
	string == backwards
}

#[test]
fn is_palindrome_0004_validation() {
	let x: u64 = 91;
	let y: u64 = 99;
	assert!(is_palindrome_0004(x*y));
	assert!(is_palindrome_0004(12321_u64));
	assert!(!is_palindrome_0004(1234_u64));
}

//
// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
//
pub fn problem0005(range: u32) -> u32 {
	// This can be quite easy using primal. The smallest positive number divisible by all natural numbers from 1 to x
	// is the product of the maximal powers of primes in range.
	//
	// For 1..10 you take all primes from 1..10 and generate the powers so that x^y < 10
	//
	// I did this in Haskell as problem0005 = foldr1 lcm [1..20]
	//
	extern crate primal;
	let mut _primes = Vec::new();
	_primes = primal::Primes::all().take_while(|x| *x < range as usize).collect();
	let mut res = 1;
	for prime in _primes {
		let mut x = prime;
		while x * prime <= range as usize {
			x *= prime;
		}
		res *= x;
	}
	res as u32
}

#[test]
fn problem0005_validation() {
	assert_eq!(problem0005(10), 2520);
}

// 
// The sum of the squares of the first ten natural numbers is,
// 1^2 + 2^2 + ... + 10^2 = 385
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)^2 = 55^2 = 3025
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
//
#[cfg(not(alt))]
pub fn problem0006(range: u64) -> u64 {
	let sumsq = (1..range+1).map(|x| x*x).fold(0, |x, acc| acc + x);
	let mut sqsum: u64 = (1..range+1).fold(0, |x, acc| acc + x);
	sqsum = sqsum * sqsum;
	sqsum-sumsq
}

#[cfg(alt)]
pub fn problem0006(range: u64) -> u64 {
	// The sum of 1..n is n(n+1)/2
	fn sqsum(n: u64) -> u64 {
		let sum = n*(n+1)/2;
		sum * sum
	}
	// The sum of the squares from 1..n is n(n+1)(2n+1)/6
	fn sumsq(n: u64) -> u64 {
		n*(n+1)*(2*n+1)/6
	}
	sqsum(range)-sumsq(range)
}

#[test]
fn problem0006_validation() {
	assert_eq!(problem0006(10), 2640);
}

// 
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.  
// What is the 10 001st prime number?
//
pub fn problem0007(nth: usize) -> usize {
	extern crate primal;
	primal::StreamingSieve::nth_prime(nth)
}

#[test]
fn problem0007_validation() {
	assert_eq!(problem0007(6), 13);
}

// 
// The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.
// 73167176531330624919225119674426574742355349194934
// 96983520312774506326239578318016984801869478851843
// 85861560789112949495459501737958331952853208805511
// 12540698747158523863050715693290963295227443043557
// 66896648950445244523161731856403098711121722383113
// 62229893423380308135336276614282806444486645238749
// 30358907296290491560440772390713810515859307960866
// 70172427121883998797908792274921901699720888093776
// 65727333001053367881220235421809751254540594752243
// 52584907711670556013604839586446706324415722155397
// 53697817977846174064955149290862569321978468622482
// 83972241375657056057490261407972968652414535100474
// 82166370484403199890008895243450658541227588666881
// 16427171479924442928230863465674813919123162824586
// 17866458359124566529476545682848912883142607690042
// 24219022671055626321111109370544217506941658960408
// 07198403850962455444362981230987879927244284909188
// 84580156166097919133875499200524063689912560717606
// 05886116467109405077541002256983155200055935729725
// 71636269561882670428252483600823257530420752963450
// Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
//

pub fn problem0008(len: usize) -> u64 {
	const INPUT: &str = r"
	73167176531330624919225119674426574742355349194934
	96983520312774506326239578318016984801869478851843
	85861560789112949495459501737958331952853208805511
	12540698747158523863050715693290963295227443043557
	66896648950445244523161731856403098711121722383113
	62229893423380308135336276614282806444486645238749
	30358907296290491560440772390713810515859307960866
	70172427121883998797908792274921901699720888093776
	65727333001053367881220235421809751254540594752243
	52584907711670556013604839586446706324415722155397
	53697817977846174064955149290862569321978468622482
	83972241375657056057490261407972968652414535100474
	82166370484403199890008895243450658541227588666881
	16427171479924442928230863465674813919123162824586
	17866458359124566529476545682848912883142607690042
	24219022671055626321111109370544217506941658960408
	07198403850962455444362981230987879927244284909188
	84580156166097919133875499200524063689912560717606
	05886116467109405077541002256983155200055935729725
	71636269561882670428252483600823257530420752963450
	";

	// TODO: filter out all windows with a 0, using a filter_map might work?
	INPUT.chars()
	     .filter_map(|c| c.to_digit(10))
	     .map(|n| n as u64)
	     .collect::<Vec<_>>()
	     .windows(len)
	     .map(|window| window.iter().product())
	     .max()
	     .unwrap()
}

#[test]
fn problem0008_validation() {
	assert_eq!(problem0008(4), 5832);
}

// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a^2 + b^2 = c^2
//
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
//
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.
//
pub fn problem0009(mysum: u64) -> u64 {
	let mut res=0;

	'outer: for c in 3u64..mysum-3 {
		for b in 2u64..c {
			if b<c && b+c<mysum {
				let a: u64=mysum-(b+c);
				if a>0 && a<b && a*a+b*b==c*c {
					res=a*b*c;
					break 'outer;
				}
			}	
		}
	}
	res
}

#[test]
fn problem0009_validation() {
	assert_eq!(problem0009(3+4+5),3*4*5);
}

pub fn problem0010(limit: usize) -> usize {
	extern crate primal;
	primal::Primes::all().take_while(|x| *x < limit).sum()
}

#[test]
fn problem0010_validation() {
	assert_eq!(problem0010(10), 17);
}

// 
// In the 20×20 grid below, four numbers along a diagonal line have been marked in red.
// 
// 08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
// 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
// 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
// 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
// 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
// 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
// 32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
// 67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
// 24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
// 21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
// 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
// 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
// 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
// 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
// 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
// 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
// 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
// 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
// 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
// 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
// 
// The product of these numbers is 26 × 63 × 78 × 14 = 1788696.
// 
// What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?
// 
// Main issue: There is no smaller sized problem which we can use to verify and for a #[test] :(
pub fn problem0011(factors: usize) -> u64 {
	// I will use the input method I also used for problem 8
	const INPUT: &str = r"
	08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
	49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
	81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
	52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
	22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
	24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
	32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
	67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
	24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
	21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
	78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
	16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
	86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
	19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
	04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
	88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
	04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
	20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
	20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
	01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
	";

	let grid: Vec<Vec<u64>> = INPUT.trim()
	                               .lines()
				       .map(|row| {
				       		row.split_whitespace()
						   .filter_map(|cell| cell.parse().ok())
						   .collect()
						  }
					   )
					.collect();
	
	let cols = grid[0].len();
	let rows = grid.len();

	let mut vecs : Vec<Vec<_>> = vec![];
	vecs.extend((0..rows).map(|col| (0..cols).map(|row| (row,col)).collect())); 	// rows
	vecs.extend((0..cols).map(|row| (0..rows).map(|col| (row,col)).collect())); 	// cols
	
	vecs.extend((0..cols).map(|i| {
		let (x0, y0) = (i, 0);
		(0..cols - x0).map(|j| (x0 + j, y0 + j)).collect()
    	}));
	vecs.extend((0..rows - 1).map(|i| {
		let (x0, y0) = (0, i + 1);
		(0..rows - y0).map(|j| (x0 + j, y0 + j)).collect()
	}));
	vecs.extend((0..cols).map(|i| {
		let (x0, y0) = (i, 0);
		(0..x0 + 1).map(|j| (x0 - j, y0 + j)).collect()
	}));
	vecs.extend((0..rows - 1).map(|i| {
		let (x0, y0) = (cols - 1, i + 1);
		(0..rows - y0).map(|j| (x0 - j, y0 + j)).collect()
	}));
	
	vecs.iter()
	    .map(|cells| {
	    	cells.windows(factors)
		     .map(|nums| nums.iter()
		                     .map(|&(x, y)| grid[x][y])
                       .product()
			 )
		     .max()
		     .unwrap_or(0)
		})
	    .max()
	    .unwrap()
}

#[test]
fn problem0011_2factors_validation() {
	assert_eq!(problem0011(2), 99*97); // row 2, col 3, up+right
}

#[test]
fn problem0011_3factors_validation() {
	assert_eq!(problem0011(3), 97*94*89); // row 15, col 5, up+right
}
//
// The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
//
//Let us list the factors of the first seven triangle numbers:
//  1: 1
//  3: 1,3
//  6: 1,2,3,6
// 10: 1,2,5,10
// 15: 1,3,5,15
// 21: 1,3,7,21
// 28: 1,2,4,7,14,28
// 
// We can see that 28 is the first triangle number to have over five divisors.
// 
// What is the value of the first triangle number to have over five hundred divisors?
// 
pub fn problem0012(limit: u64) -> u64 {
	extern crate primal;
	let sieve = primal::Sieve::new(100000);

	fn nth_triangle(nth: u64) -> u64 {
		nth * (nth + 1) / 2	
	}

	fn num_divisors(number: u64, sieve: &primal::Sieve) -> u64 {
		let mut res: u64 = 1;
		let mut factors = sieve.factor(number as usize).unwrap();
		for _n in 0..factors.len() {
			let (_, y) = factors.pop().unwrap();
			res *= y as u64 + 1;
		}
		res
	}

	let mut x = 1;
	while num_divisors(nth_triangle(x), &sieve) <= limit {
		x += 1
	}

	nth_triangle(x)
}

#[test]
fn problem0012_validation() {
	assert_eq!(problem0012(5),28);
}

pub fn problem0016(n: u32) -> u64 {
    use std::ops::Deref;
    extern crate num_bigint;
    let bignum = num_bigint::BigUint::new([2 as u32].to_vec());
    let mut res: u64 = 0;
    for byte in bignum.pow(n).to_radix_le(10).iter() {
        res += *byte.deref() as u64
    }
    res
}

#[test]
fn problem0016_validation() {
    assert_eq!(problem0016(15),26);
}
