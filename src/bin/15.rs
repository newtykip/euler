/*
Problem 15 - Lattice paths

Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
How many such routes are there through a 20×20 grid?
*/

fn binomial(n: usize, k: usize) -> Result<usize, &'static str> {
    if k > n {
        return Err("k must be less than n!");
    } else if k == 0 {
        return Ok(1);
    } else if k > n / 2 {
        return binomial(n, n - k);
    }

    return Ok(n * binomial(n - 1, k - 1).unwrap() / k);
}

fn count_lattice_paths(width: usize, height: usize) -> usize {
    // All paths have width + height segments. k can either be width or height
    // (symmetry of binomial coefficients)
    if width != height {
        return binomial(width + height, width).unwrap();
    }
    // (2n, n)
    else {
        let mut result: usize = 1;

        for i in 1..width {
            result *= (width + i) / i;
        }

        return result;
    }
}

fn main() {
    let routes = count_lattice_paths(20, 20);

    println!("The amount of routes through a 20x20 grid is {routes}");
}
