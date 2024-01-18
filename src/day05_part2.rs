use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 05 - Part 2 --");
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
            let mut bool1 = false;
            let mut bool2 = false;
            let chars: Vec<_> = line.chars().collect();
            //println!("in {:?} :",chars);
            // il faut deux paires sans overlap
            if !bool1 {
                for i in 0..line.len() - 2 {
                    for j in 2..line.len() - 1 {
                        if i < j
                            && j - i > 1
                            && chars[i] == chars[j]
                            && chars[i + 1] == chars[j + 1]
                        {
                            bool1 = true;
                            //println!("found 2x {}{}",chars[i],chars[i+1]);
                            break;
                        }
                    }
                }
            }
            if bool1 && !bool2 {
                for i in 0..line.len() - 2 {
                    // il faut une paire de même lettres avec 1 char entre
                    if chars[i] == chars[i + 2] {
                        bool2 = true;
                        //println!("found {}{}{}",chars[i],chars[i+1],chars[i+2]);
                        break;
                    }
                }
            }

            if bool1 && bool2 {
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
        assert_eq!(get_answer("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(get_answer("xxyxx"), 1);
        assert_eq!(get_answer("uurcxstgmygtbstg"), 0);
        assert_eq!(get_answer("ieodomkazucvgmuy"), 0);
        assert_eq!(get_answer(include_str!("../assets/day05_input.txt")), 55);
    }
}
