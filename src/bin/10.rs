/*
Problem 10 - Summation of primes

The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
Find the sum of all the primes below two million.
*/

// Implementation of the Sieve of Eratosthenes
// https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes

use rayon::prelude::*;

fn find_primes(upper_bound: usize) -> Vec<usize> {
    let mut mask = vec![true; upper_bound];
    let mut primes: Vec<usize> = vec![];

    mask[0] = false;
    mask[1] = false;

    for i in 2..upper_bound {
        if mask[i] {
            primes.push(i);

            let mut j = 2 * i;

            while j < upper_bound {
                mask[j] = false;
                j += i;
            }
        }
    }

    return primes;
}

fn sum_of_primes(upper_bound: usize) -> usize {
    let primes = find_primes(upper_bound);
    return primes.par_iter().sum::<usize>();
}

pub fn main() {
    let value = sum_of_primes(2000000);

    println!("The sum of the primes up until 2000000 is {value}");
}
