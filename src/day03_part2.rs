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
    let mut current_santa = (0, 0);
    let mut current_robot = (0, 0);
    visited.insert(current_santa);
    for (i, c) in input.chars().enumerate() {
        match c {
            'v' => {
                if i % 2 == 0 {
                    current_santa.0 += 1
                } else {
                    current_robot.0 += 1
                }
            }
            '^' => {
                if i % 2 == 0 {
                    current_santa.0 -= 1
                } else {
                    current_robot.0 -= 1
                }
            }
            '>' => {
                if i % 2 == 0 {
                    current_santa.1 += 1
                } else {
                    current_robot.1 += 1
                }
            }
            '<' => {
                if i % 2 == 0 {
                    current_santa.1 -= 1
                } else {
                    current_robot.1 -= 1
                }
            }
            _ => panic!("should be a direction"),
        }
        if i % 2 == 0 {
            visited.insert(current_santa);
        } else {
            visited.insert(current_robot);
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("^v"), 3);
        assert_eq!(get_answer("^>v<"), 3);
        assert_eq!(get_answer("^v^v^v^v^v"), 11);
        assert_eq!(get_answer(include_str!("../assets/day03_input.txt")), 2639);
    }
}
