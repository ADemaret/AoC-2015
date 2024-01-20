use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 14 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day14_input_demo1.txt");
    let input = include_str!("../assets/day14_input.txt");

    println!("La réponse est {}", get_answer(input, 2503_f32));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str, total_time: f32) -> usize {
    // parse
    let mut deers = HashMap::new();
    input.lines().for_each(|line| {
        let (name, parts) = line.split_once(' ').unwrap();
        let values: Vec<_> = parts
            .split_whitespace()
            .filter_map(|x| x.parse::<f32>().ok())
            .collect();
        deers.insert(name, values);
    });
    //println!("{:?}", deers);

    // distances
    let mut distances: HashMap<String, usize> = HashMap::new();
    let mut bonus = HashMap::new();
    for time in 1..=total_time as usize {
        let mut win_dist = 0_f32;
        let mut winners = Vec::new();
        for deer in &deers {
            let d = (time as f32 / (deer.1[1] + deer.1[2])).floor() * deer.1[0] * deer.1[1]
                + deer.1[1].min(time as f32 % (deer.1[1] + deer.1[2])) * deer.1[0];
            distances.insert(deer.0.to_string(), d as usize);
            // println!(
            //     "après {time}, {} a avancé de {d} ({}*{}*{} + {}*{})",
            //     deer.0,
            //     (time as f32 / (deer.1[1] + deer.1[2])).floor(),
            //     deer.1[0],
            //     deer.1[1],
            //     deer.1[1].min(time as f32 % (deer.1[1] + deer.1[2])),
            //     deer.1[0],
            // );
            if d > win_dist {
                win_dist = d;
                winners.clear();
                winners.push(*deer.0);
            } else if d == win_dist {
                winners.push(*deer.0);
            }
        }
        for winner in winners.clone() {
            let actual_bonus = if bonus.get(winner).is_some() {
                bonus.get(winner).unwrap() + 1
            } else {
                1
            };
            bonus.insert(winner, actual_bonus);
            // println!("{} +1", winner);
        }
        //println!("{:?}",bonus);
    }
    //println!("{:?}", bonus);

    *bonus.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day14_input_demo1.txt"), 1000_f32),
            689
        );
        assert_eq!(
            get_answer(include_str!("../assets/day14_input.txt"), 2503_f32),
            1084
        );
    }
}
