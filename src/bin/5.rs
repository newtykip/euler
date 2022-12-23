/*
Problem 5 - Smallest multiple

2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
What is the smallest positive number that is  evenly divisible  by all of the numbers from 1 to 20?
*/

// Euclidean algorithm for finding GCD
fn gcd(a: usize, b: usize) -> usize {
	let mut x = a;
	let mut y = b;
	let mut r: usize = 1; // temporary initial value

	while r != 0 {
		if x < y {
			let temp = x;
			x = y;
			y = temp;
		}

		r = x % y;

		if r == 0 {
			break;
		} else {
			x = y;
			y = r;
		}
	}

	return y;
}

fn first_value_divisible_by(start: usize, end: usize) -> usize { 
	if start > end {
		panic!("You can not start on a value higher than your end value!");
	}

	let mut result: usize = 1;

	for i in start..(end + 1) {
		result = (result * i) / gcd(result, i);
	}

	return result;
}

fn main() {
	let number = first_value_divisible_by(1, 20);

	println!("The smallest number that is divisible by all integers between 1 and 20 is {number}");
}
