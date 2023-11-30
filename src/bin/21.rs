/*
Problem 21 - Amicable Numbers

Let $d(n)$ be defined as the sum of proper divisors of $n$ (numbers less than $n$ which divide evenly into $n$).
If $d(a) = b$ and $d(b) = a$, where $a \ne b$, then $a$ and $b$ are an amicable pair and each of $a$ and $b$ are called amicable numbers.
For example, the proper divisors of $220$ are $1, 2, 4, 5, 10, 11, 20, 22, 44, 55$ and $110$; therefore $d(220) = 284$. The proper divisors of $284$ are $1, 2, 4, 71$ and $142$; so $d(284) = 220$.
Evaluate the sum of all the amicable numbers under $10000$.
*/

fn proper_divisors(number: usize) -> Vec<usize> {
    let mut divisors = vec![1];

    for divisor in 2..=(number / 2) {
        if number % divisor == 0 {
            divisors.push(divisor);
        }
    }

    divisors
}

fn d(number: usize) -> usize {
    proper_divisors(number).iter().sum()
}

fn amicable_numbers(upper_bound: usize) -> Vec<usize> {
    let mut amicable_numbers = Vec::new();

    for a in 2..upper_bound {
        let b = d(a);

        if a != b && d(b) == a {
            amicable_numbers.push(a);
        }
    }

    amicable_numbers
}

pub fn main() {
    let sum = amicable_numbers(10_000).iter().sum::<usize>();

    println!("The sum of the first 9,999 amicable numbers is {sum}!");
}
