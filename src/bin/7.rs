/*
Problem 7 - 10001st prime

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
What is the 10 001st prime number?
*/

use std::f64::consts::E;

// Implementation of the Sieve of Eratosthenes
// https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
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

fn nth_prime(n: usize) -> Option<usize> {
    if n < 1 {
        return None;
    }

    let primes = find_primes(upper_bound_for_nth_prime(n));
    return Some(primes[n - 1]);
}

fn main() {
    let number = nth_prime(10001).unwrap();

    println!("The 10,001st prime number is {number}");
}
