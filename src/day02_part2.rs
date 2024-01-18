use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 02 - Part 2 --");
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
            let a = 2 * dim[0] + 2 * dim[1];
            let b = dim[0] * dim[1] * dim[2];
            a + b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("2x3x4"), 34);
        assert_eq!(get_answer("1x1x10"), 14);
        assert_eq!(
            get_answer(include_str!("../assets/day02_input.txt")),
            3842356
        );
    }
}
