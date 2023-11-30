/*
Problem 23 - Non-Abundant Sums

A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, the sum of the proper divisors of $28$ would be $1 + 2 + 4 + 7 + 14 = 28$, which means that $28$ is a perfect number.
A number $n$ is called deficient if the sum of its proper divisors is less than $n$ and it is called abundant if this sum exceeds $n$.
As $12$ is the smallest abundant number, $1 + 2 + 3 + 4 + 6 = 16$, the smallest number that can be written as the sum of two abundant numbers is $24$. By mathematical analysis, it can be shown that all integers greater than $28123$ can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.
Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.
*/

const UPPER_LIMIT: u16 = 28123;

fn proper_divisors(number: usize) -> Vec<usize> {
    let mut divisors = vec![1];

    for divisor in 2..=(number / 2) {
        if number % divisor == 0 {
            divisors.push(divisor);
        }
    }

    divisors
}

fn is_abundant(number: usize) -> bool {
    proper_divisors(number).iter().sum::<usize>() > number
}

fn abundant_numbers(lower_bound: usize, upper_bound: usize) -> Vec<usize> {
    let mut abundant_numbers = Vec::new();

    for number in lower_bound..=upper_bound {
        if is_abundant(number) {
            abundant_numbers.push(number);
        }
    }

    abundant_numbers
}

pub fn main() {
    let numbers = abundant_numbers(1, UPPER_LIMIT as usize);
    let mut sums = vec![0; UPPER_LIMIT as usize + 1];

    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            let sum = numbers[i] + numbers[j];

            if sum <= UPPER_LIMIT as usize {
                sums[sum] = 1;
            }
        }
    }

    let answer = sums
        .iter()
        .enumerate()
        .filter(|(_, &sum)| sum == 0)
        .map(|(number, _)| number)
        .sum::<usize>();

    println!(
        "The sum of all the positive integers which cannot be written as the sum of two abundant numbers is {}!",
        answer
    );
}
