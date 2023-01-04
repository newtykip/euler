/*
Problem 3 - Largest prime factor

The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143 ?
*/

fn largest_prime_factor(number: usize) -> usize {
    // All even numbers can be divided by 2
    if number % 2 == 0 {
        return 2;
    }

    let mut i = 3;
    let mut num = number.clone();
    let mut primes: Vec<usize> = vec![];

    while num != 1 {
        if num % i == 0 {
            num /= i;
            primes.push(i);
        } else {
            // Test next odd number
            i += 2;
        }
    }

    // The largest prime factor will be at the end of the vec
    return primes[primes.len() - 1];
}

fn main() {
    let factor = largest_prime_factor(600851475143);
    println!("The largest prime factor of 600851475143 is {factor}");
}
