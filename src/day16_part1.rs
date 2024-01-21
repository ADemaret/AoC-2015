use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 16 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day16_input_demo1.txt");
    let input = include_str!("../assets/day16_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    // parse
    let sues: Vec<HashMap<String, usize>> = input
        .lines()
        .map(|line| {
            let chunks: Vec<_> = line
                .split([' ', ':', ','])
                .filter(|str| !str.is_empty())
                .collect();
            let mut hm: HashMap<String, usize> = HashMap::new();
            hm.insert(chunks[2].to_string(), chunks[3].parse::<usize>().unwrap());
            hm.insert(chunks[4].to_string(), chunks[5].parse::<usize>().unwrap());
            hm.insert(chunks[6].to_string(), chunks[7].parse::<usize>().unwrap());
            hm
        })
        .collect();
    //println!("{:#?}", sues);

    // selection
    for (id, sue) in sues.iter().enumerate() {
        if (sue.get("children").is_none() || sue.get("children").is_some_and(|&x| x == 3))
            && (sue.get("cats").is_none() || sue.get("cats").is_some_and(|&x| x == 7))
            && (sue.get("samoyeds").is_none() || sue.get("samoyeds").is_some_and(|&x| x == 2))
            && (sue.get("pomeranians").is_none() || sue.get("pomeranians").is_some_and(|&x| x == 3))
            && (sue.get("akitas").is_none() || sue.get("akitas").is_some_and(|&x| x == 0))
            && (sue.get("vizslas").is_none() || sue.get("vizslas").is_some_and(|&x| x == 0))
            && (sue.get("goldfish").is_none() || sue.get("goldfish").is_some_and(|&x| x == 5))
            && (sue.get("trees").is_none() || sue.get("trees").is_some_and(|&x| x == 3))
            && (sue.get("cars").is_none() || sue.get("cars").is_some_and(|&x| x == 2))
            && (sue.get("perfumes").is_none() || sue.get("perfumes").is_some_and(|&x| x == 1))
        {
            return id + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(include_str!("../assets/day16_input.txt")), 213);
    }
}
