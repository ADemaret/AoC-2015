use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 19 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day19_input_demo1.txt");
    let input = include_str!("../assets/day19_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    // parse
    let (rep, molecule) = input.split_once("\n\n").unwrap();
    let replacements: Vec<_> = rep
        .lines()
        .map(|line| line.split_once(" => ").unwrap())
        .collect();
    // println!("replacements = {:?}", replacements);
    // println!("molecule = {}", molecule);

    let mut new_molecules: Vec<String> = Vec::new();
    for index in 0..molecule.len() {
        for (from, to) in &replacements {
            let chunk = molecule
                .chars()
                .skip(index)
                .take(from.len())
                .collect::<String>();
            if chunk == *from {
                let mut buff = String::from("");
                buff.push_str(molecule.chars().take(index).collect::<String>().as_str());
                buff.push_str(to);
                buff.push_str(
                    molecule
                        .chars()
                        .skip(index + from.len())
                        .collect::<String>()
                        .as_str(),
                );
                new_molecules.push(buff.clone());
                // println!("on replace {} par {} -> {}", chunk, to,buff);
            }
        }
    }
    new_molecules.sort();
    new_molecules.dedup();
    new_molecules.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day19_input_demo1.txt")),
            7
        );
        assert_eq!(get_answer(include_str!("../assets/day19_input.txt")), 509);
    }
}
