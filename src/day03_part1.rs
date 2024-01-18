use std::{collections::HashSet, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 03 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day03_input_demo1.txt");
    let input = include_str!("../assets/day03_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut current = (0, 0);
    visited.insert(current);
    for c in input.chars() {
        match c {
            'v' => current.0 += 1,
            '^' => current.0 -= 1,
            '>' => current.1 += 1,
            '<' => current.1 -= 1,
            _ => panic!("should be a direction"),
        }
        visited.insert(current);
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(">"), 2);
        assert_eq!(get_answer("^>v<"), 4);
        assert_eq!(get_answer("^v^v^v^v^v"), 2);
        assert_eq!(get_answer(include_str!("../assets/day03_input.txt")), 2565);
    }
}
