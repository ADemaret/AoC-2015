use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 05 - Part 1 --");
    let now = Instant::now();

    let input = include_str!("../assets/day05_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut bool2 = false;
            let mut bool3 = false;
            // il faut 3 voyelles
            let bool1 = line.chars().filter(|&c| "aeiou".contains(c)).count() >= 3;

            let chars: Vec<_> = line.chars().collect();
            for i in 0..line.len() - 1 {
                // il faut une lettre qui se répète
                if !bool2 && chars[i] == chars[i + 1] {
                    bool2 = true;
                }
                // il ne faut pas cette suite de lettres : ab, cd, pq, ou xy
                if !bool3 {
                    match chars[i] {
                        'a' => {
                            if chars[i + 1] == 'b' {
                                bool3 = true
                            }
                        }
                        'c' => {
                            if chars[i + 1] == 'd' {
                                bool3 = true
                            }
                        }
                        'p' => {
                            if chars[i + 1] == 'q' {
                                bool3 = true
                            }
                        }
                        'x' => {
                            if chars[i + 1] == 'y' {
                                bool3 = true
                            }
                        }
                        _ => {}
                    }
                }
            }
            if bool1 && bool2 && !bool3 {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("ugknbfddgicrmopn"), 1);
        assert_eq!(get_answer("aaa"), 1);
        assert_eq!(get_answer("jchzalrnumimnmhp"), 0);
        assert_eq!(get_answer("haegwjzuvuyypxyu"), 0);
        assert_eq!(get_answer("dvszwmarrgswjxmb"), 0);
        assert_eq!(get_answer(include_str!("../assets/day05_input.txt")), 255);
    }
}
