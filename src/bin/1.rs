// Problem 1 - Multiples of 3 or 5

// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23. 
// Find the sum of all the multiples of 3 or 5 below 1000. 

use std::collections::HashSet;

fn multiples_of(multipliers: &Vec<usize>, upper_bound: usize) -> HashSet<usize> {
	let mut results: HashSet<usize> = HashSet::new();

	// Find all of the numbers between 1 and the upper bound that are multiples of one of the numbers
	// in the multipliers vec
	for i in 1..upper_bound {
		for number in multipliers {			
			if i % number == 0 {
				results.insert(i);
			}
		}
	}

	return results;
}

fn main() {
	let multiples = multiples_of(&vec![3, 5], 1000);
	let sum: usize = multiples.iter().sum();

	println!("The sum of the multiples of 3 and 5 up until 1000 is {sum}");
}
