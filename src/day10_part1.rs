use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 10 - Part 1 --");
    let now = Instant::now();

    println!("La réponse est {}", get_answer("3113322113",40));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str, times:usize) -> usize {
    let mut suite :Vec<usize> = input.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

    for _ in 0..times {
        suite = conway(&suite);
    }
    suite.len()
}

fn conway(suite : &[usize]) -> Vec<usize> {
    let mut new_suite :Vec<usize> = Vec::new();
    
    let mut iter = suite.iter();
    let mut nbr_item = 0;
    let mut val = suite[0];
    let mut next = iter.next();
    while next.is_some() {
        let new_val = *next.unwrap();
        if new_val == val {
            nbr_item += 1;
        } else {
            new_suite.push(nbr_item);
            new_suite.push(val);
            nbr_item = 1;
            val = new_val;
        }
        next = iter.next();
    }
    new_suite.push(nbr_item);
    new_suite.push(val);
    //println!("{:?}",new_suite);

    new_suite
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("1",5), 6);
        assert_eq!(get_answer("3113322113",40), 329356);
    }
}
