/*
Problem 16 - Power digit sum

2 15  = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
What is the sum of the digits of the number 2 1000 ?
*/

fn power_digit_sum<const POWER: usize>(base: usize) -> usize {
    let mut digits = [0u8; POWER];
    digits[0] = 1; // 2^0 = 1

    for _ in 0..POWER {
        let mut carry = 0;

        for digit in digits.iter_mut() {
            let result = *digit as usize * base + carry;
            *digit = result as u8 % 10;
            carry = result / 10;
        }
    }

    digits.iter().map(|&x| x as usize).sum()
}

pub fn main() {
    let sum = power_digit_sum::<1000>(2);

    println!("The sum of the digits of the number 2^1000 is {}!", sum);
}
