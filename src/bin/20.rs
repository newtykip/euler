/*
Problem 20 - Factorial Digit Sum

$n!$ means $n \times (n - 1) \times \cdots \times 3 \times 2 \times 1$.
For example, $10! = 10 \times 9 \times \cdots \times 3 \times 2 \times 1 = 3628800$, and the sum of the digits in the number $10!$ is $3 + 6 + 2 + 8 + 8 + 0 + 0 = 27$.
Find the sum of the digits in the number $100!$.
*/

fn multiply(digits: &mut Vec<u8>, multiplier: usize) {
    let mut carry = 0usize;

    for digit in digits.iter_mut() {
        let product = *digit as usize * multiplier + carry;
        *digit = (product % 10) as u8;
        carry = product / 10;
    }

    while carry > 0 {
        digits.push((carry % 10) as u8);
        carry /= 10;
    }
}

pub fn main() {
    let mut digits = vec![1];

    for num in 2..=100 {
        multiply(&mut digits, num);
    }

    let sum: usize = digits.iter().map(|&d| d as usize).sum();

    print!("The sum of the digits in the number 100! is {}!", sum);
}
