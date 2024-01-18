use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 01 - Part 1 --");
    let now = Instant::now();

    let input = include_str!("../assets/day01_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> isize {
    let mut up = 0;
    let mut down = 0;
    input.chars().for_each(|c| match c {
        '(' => up += 1,
        ')' => down += 1,
        _ => unreachable!("unknown char"),
    });
    up - down
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("(())"), 0);
        assert_eq!(get_answer("()()"), 0);
        assert_eq!(get_answer("((("), 3);
        assert_eq!(get_answer("(()(()("), 3);
        assert_eq!(get_answer("))((((("), 3);
        assert_eq!(get_answer("())"), -1);
        assert_eq!(get_answer("))("), -1);
        assert_eq!(get_answer(")))"), -3);
        assert_eq!(get_answer(")())())"), -3);
        assert_eq!(get_answer(include_str!("../assets/day01_input.txt")), 280);
    }
}
