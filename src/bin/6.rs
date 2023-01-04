/*
Problem 6 - Sum square difference

The sum of the squares of the first ten natural numbers is,
1^2 + 2^2 + ... + 10^2 = 385
The square of the sum of the first ten natural numbers is,
(1 + 2 + ... + 10)^2 = 55^2 = 3025
Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640.
Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/

const LOWER_BOUND: usize = 1;
const UPPER_BOUND: usize = 100;

fn sum_of_squares(lower_bound: usize, upper_bound: usize) -> Option<usize> {
    if lower_bound > upper_bound {
        return None;
    }

    let mut squares: Vec<usize> = vec![];

    for i in lower_bound..(upper_bound + 1) {
        squares.push(i.pow(2));
    }

    return Some(squares.iter().sum());
}

fn square_of_sum(lower_bound: usize, upper_bound: usize) -> Option<usize> {
    if lower_bound > upper_bound {
        return None;
    }

    return Some(((lower_bound..(upper_bound + 1)).sum::<usize>()).pow(2));
}

fn main() {
    let squared_sum = square_of_sum(LOWER_BOUND, UPPER_BOUND).unwrap();
    let summed_squares = sum_of_squares(LOWER_BOUND, UPPER_BOUND).unwrap();
    let difference = squared_sum - summed_squares;

    println!("For the numbers between {LOWER_BOUND} and {UPPER_BOUND}, the difference between the square of the sum and the sum of the squares is {difference}");
}
