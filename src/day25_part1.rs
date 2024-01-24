use std::{f64::MAX_10_EXP, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 25 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day25_input_demo1.txt");
    //let input = include_str!("../assets/day25_input.txt");

    println!("La réponse est {}", get_answer(/*input*/));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer() -> usize {
    let mut code = 20151125;
    let mut ligne = 1;
    let mut max_ligne = 1;
    let mut colonne = 1;
    //println!("on a {} en L {}, C {}",code,ligne,colonne);
    loop {
        if ligne > 1 {
            ligne -= 1;
            colonne +=1;
        } else {
            ligne = max_ligne + 1;
            max_ligne = ligne;
            colonne = 1;
        }        
        code = code * 252533 % 33554393;
        //println!("on a {} en L {}, C {}",code,ligne,colonne);
        if ligne == 3010 && colonne == 3019 {
            return code;
        }
    }        
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        // assert_eq!(get_answer(include_str!("../assets/day25_input_demo1.txt")), 0);
        assert_eq!(get_answer(), 8997277);
    }
}
