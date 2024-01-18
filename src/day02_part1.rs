use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 02 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day02_input_demo1.txt");
    let input = include_str!("../assets/day02_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut dim: Vec<_> = line
                .split('x')
                .map(|d| d.parse::<usize>().unwrap())
                .collect();
            if dim.len() != 3 {
                panic!("should be 3 dimensions");
            }
            dim.sort();
            let a = dim[0] * dim[1];
            let b = dim[1] * dim[2];
            let c = dim[2] * dim[0];
            3 * a + 2 * b + 2 * c
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("2x3x4"), 58);
        assert_eq!(get_answer("1x1x10"), 43);
        assert_eq!(
            get_answer(include_str!("../assets/day02_input.txt")),
            1606483
        );
    }
}
