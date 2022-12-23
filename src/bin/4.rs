/*
Problem 4 - Largest palindrome product

A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
Find the largest palindrome made from the product of two 3-digit numbers.
*/

fn largest_pallindromic_number(lower_bound: usize, upper_bound: usize) -> usize {
	let mut products: Vec<usize> = vec![];
	
	for i in lower_bound..(upper_bound + 1) {
		for j in lower_bound..(upper_bound + 1) {
			products.push(i * j);
		}
	}

	// Filter for pallindromic numbers
	let mut pallindromic = products
		.iter()
		.filter(|x| x.to_string() == x.to_string().chars().rev().collect::<String>())
		.collect::<Vec<&usize>>();

	// Return the largest value
	pallindromic.sort();
	pallindromic.reverse();

	return *pallindromic[0];
}

fn main() {
	let number = largest_pallindromic_number(100, 999);
	
	println!("The largest pallindromic number made from the product of two three-digit numbers is {number}");
}
