/*
Problem 9 - Special Pythagorean triplet

A Pythagorean triplet is a set of three natural numbers,  a  <  b  <  c , for which,
a  2  +  b  2  =  c  2
For example, 3 2  + 4 2  = 9 + 16 = 25 = 5 2 .
There exists exactly one Pythagorean triplet for which  a  +  b  +  c  = 1000. Find the product  abc .
*/

//! Notes:
// a + b + c = n
// c = n - a - b

// a^2 + b^2 = c^2
// a^2 + b^2 = (n - a - b)^2
// 0 = 2ab - 2an - 2bn + n^2
// 2an - n^2 = b(2a - 2n)
// b = (2an - n^2)/(2a - 2n)

// For a given value a, with a sum of n
// b = (2an - n^2)/(2a - 2n)
// c = n - a - b

fn triplet_with_sum(sum: isize) -> Option<(isize, isize, isize)> {
    for a in 1..(sum + 1) {
        let b: isize = ((2 * a * sum) - sum.pow(2)) / (2 * (a - sum));
        let c: isize = sum - a - b;

        if a.pow(2) + b.pow(2) == c.pow(2) {
            return Some((a, b, c));
        }
    }

    return None;
}

fn main() {
    let (a, b, c) = triplet_with_sum(1000).unwrap();

    println!("a = {a}, b = {b}, c = {c} // abc = {}", a * b * c);
}
