use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 12 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day12_input_demo1.txt");
    let input = include_str!("../assets/day12_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> isize {
    let values = find_values(input);
    values.iter().sum()
}

fn find_values(input: &str) -> Vec<isize> {
    let vec = input
        .lines()
        .flat_map(|line| {
            line.split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter(|str| !str.is_empty())
                .map(|str| str.parse::<isize>().unwrap() )
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    //println!("{:#?}", vec);
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo1.txt")),
            18
        );
        assert_eq!(get_answer(include_str!("../assets/day12_input.txt")), 156366);
    }
}
