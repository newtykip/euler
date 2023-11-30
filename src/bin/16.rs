/*
Problem 16 - Power digit sum

2 15  = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
What is the sum of the digits of the number 2 1000 ?
*/

use num_bigint::BigUint;

fn get_digits(number: usize) -> Vec<u32> {
    return number
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
}

fn power_digit_sum(base: usize, power: u32) -> usize {
    let answer = BigUint::new(get_digits(base))
        .pow(power)
        .to_string()
        .chars()
        .collect::<Vec<char>>();

    let mut sum = 0;

    for i in 0..answer.len() {
        let number = answer[i].to_string().parse::<usize>().unwrap();

        sum += number;
    }

    sum
}

pub fn main() {
    let sum = power_digit_sum(2, 1000);

    println!("The sum of the digits of the number 2^1000 is {}!", sum);
}
