/*
Problem 26 - Reciprocal Cycles

A unit fraction contains $1$ in the numerator. The decimal representation of the unit fractions with denominators $2$ to $10$ are given:
\begin{align}
1/2 &= 0.5\\
1/3 &=0.(3)\\
1/4 &=0.25\\
1/5 &= 0.2\\
1/6 &= 0.1(6)\\
1/7 &= 0.(142857)\\
1/8 &= 0.125\\
1/9 &= 0.(1)\\
1/10 &= 0.1
\end{align}
Where $0.1(6)$ means $0.166666\cdots$, and has a $1$-digit recurring cycle. It can be seen that $1/7$ has a $6$-digit recurring cycle.
Find the value of $d \lt 1000$ for which $1/d$ contains the longest recurring cycle in its decimal fraction part.
*/

fn cycle_length(d: usize) -> usize {
    let mut remainders = vec![0; d];
    let mut value = 1;
    let mut position = 0;

    while remainders[value] == 0 && value != 0 {
        remainders[value] = position;
        value *= 10;
        value %= d;
        position += 1;
    }

    if value == 0 {
        return 0;
    } else {
        return position - remainders[value];
    }
}

fn longest_cycle(limit: usize) -> usize {
    let mut max_length = 0;
    let mut result = 0;

    for d in 2..limit {
        let length = cycle_length(d);
        if length > max_length {
            max_length = length;
            result = d;
        }
    }

    result
}

pub fn main() {
    let number = longest_cycle(1000);
    println!("The value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part is: {}", number);
}
