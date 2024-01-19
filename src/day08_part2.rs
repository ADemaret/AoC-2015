use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 08 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day08_input_demo1.txt");
    let input = include_str!("../assets/day08_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let code = line.chars().count();
            let string = serde_json::to_string(&line).unwrap();
            let len = string.chars().count();
            //println!("{} ({}) => {} ({})",line,code,string,len);
            len - code
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day08_input_demo1.txt")),
            19
        );
        assert_eq!(get_answer(include_str!("../assets/day08_input.txt")), 2117);
    }
}
