use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 23 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day23_input_demo1.txt");
    let input = include_str!("../assets/day23_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> isize {
    // parse
    let instructions: Vec<(&str, &str, isize)> = input
        .lines()
        .map(|line| {
            let mut chunks = line
                .split([' ', ','])
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>();
            let mut val = 0;
            match chunks[0] {
                "hlf" | "tpl" | "inc" => {}
                "jmp" => {
                    val = chunks[1].parse().unwrap();
                    chunks[1] = "";
                }
                "jie" | "jio" => val = chunks[2].parse().unwrap(),
                _ => unreachable!("should be a valid instruction"),
            }
            (chunks[0], chunks[1], val)
        })
        .collect();

    let mut register_a = 0;
    let mut register_b = 0;
    let mut current = 0;
    loop {
        if current >= instructions.len() {
            // println!("out");
            return register_b;
        }
        // println!("line {} : {:?}", current, instructions[current]);
        match instructions[current].0 {
            "hlf" => {
                if instructions[current].1 == "a" {
                    register_a = ((register_a as f32) / 2.0) as isize;
                    // println!(" register_a => {}", register_a);
                } else {
                    register_b = ((register_b as f32) / 2.0) as isize;
                    // println!(" register_b => {}", register_b);
                }
                current += 1;
            }
            "tpl" => {
                if instructions[current].1 == "a" {
                    register_a *= 3;
                    // println!(" register_a => {}", register_a);
                } else {
                    register_b *= 3;
                    // println!(" register_b => {}", register_b);
                }
                current += 1;
            }
            "inc" => {
                if instructions[current].1 == "a" {
                    register_a += 1;
                    // println!(" register_a => {}", register_a);
                } else {
                    register_b += 1;
                    // println!(" register_b => {}", register_b);
                }
                current += 1;
            }
            "jmp" => {
                let new_current = current as isize + instructions[current].2;
                if new_current < 0 || new_current > instructions.len() as isize - 1 {
                    // println!("out");
                    return register_b;
                } else {
                    current = new_current as usize;
                }
            }
            "jie" => {
                if (instructions[current].1 == "a" && register_a % 2 == 0)
                    || (instructions[current].1 == "b" && register_b % 2 == 0)
                {
                    let new_current = current as isize + instructions[current].2;
                    if new_current < 0 || new_current > instructions.len() as isize - 1 {
                        // println!("out");
                        return register_b;
                    } else {
                        current = new_current as usize;
                    }
                } else {
                    // println!("cond not satisfied");
                    current += 1;
                }
            }
            "jio" => {
                if (instructions[current].1 == "a" && register_a == 1)
                    || (instructions[current].1 == "b" && register_b == 1)
                {
                    let new_current = current as isize + instructions[current].2;
                    if new_current < 0 || new_current > instructions.len() as isize - 1 {
                        // println!("out");
                        return register_b;
                    } else {
                        current = new_current as usize;
                    }
                } else {
                    // println!("cond not satisfied");
                    current += 1;
                }
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day23_input_demo1.txt")),
            2
        );
        assert_eq!(get_answer(include_str!("../assets/day23_input.txt")), 255);
    }
}
