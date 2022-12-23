/*
Problem 7 - 10001st prime

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
What is the 10 001st prime number?
*/

use std::f64::consts::E;

// Implementation of the Sieve of Eratosthenes
// https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
fn find_primes(upper_bound: usize) -> Vec<usize> {
	let mut values: Vec<bool> = (2..(upper_bound + 1)).map(|_| true).collect();
	let mut i = 2;

	// Adjust values array
	while i < (upper_bound as f64).sqrt() as usize {
		if values[i] {
			let mut j = i.pow(2);

			while j < upper_bound {
				values[j] = false;
				j += i;
			}
		}

		i += 1;
	}

	// Find the indexes where the values are true
	let mut primes: Vec<usize> = vec![];

	for i in 0..values.len() {
		if values[i] {
			primes.push(i);
		}
	}
	
	return primes;
}

// Bounds for nth prime: http://en.wikipedia.org/wiki/Prime_number_theorem
// ln n + ln(ln n) - 1 < p_n / n < ln n + ln(ln n), n >= 6
fn upper_bound_for_nth_prime(n: usize) -> usize {
	// The first 5 primes are under 13 (6th prime) 
	if n < 6 {
		return 13;
	}

	let ln_n = (n as f64).log(E);

	return n * (ln_n + ln_n.log(E)).ceil() as usize;
}

fn nth_prime(n: usize) -> usize {
	let primes = find_primes(upper_bound_for_nth_prime(n));
	return primes[n - 1];
}

fn main() {
	let number = nth_prime(10001);

	println!("The 10,001st prime number is {number}");
}
