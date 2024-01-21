use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 15 - Part 2 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day15_input_demo1.txt");
    let input = include_str!("../assets/day15_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> isize {
    // parse
    let formulas: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.split([':', ' ', ','])
                .filter_map(|str| str.parse::<isize>().ok())
                .collect()
        })
        .collect();
    // println!("{:?}", formulas);

    // find max
    let mut max = 0_isize;
    let mut product;

    // astuce pour avoir un nombre de boucles imbriquées
    // dont le nombre connu seulement au runtime
    let nbr_max = 100 + 1; // les valeurs vont de 0 à 100 = 101 valeurs différentes
    let cnt_max = usize::pow(nbr_max, formulas.len() as u32);
    // println!("il va y avoir {} valeurs au compteur",cnt_max);
    for i in 0..cnt_max {
        let mut qtt_ingredient = Vec::new();
        let mut zz = i as f32;
        qtt_ingredient.push(i % nbr_max);
        for _ in 1..formulas.len() {
            zz = zz / nbr_max as f32;
            zz = f32::floor(zz);
            qtt_ingredient.push(zz as usize % nbr_max);
        }
        // }
        if qtt_ingredient.iter().sum::<usize>() == 100 {
            let mut valid = true;
            let mut p: Vec<isize> = Vec::new();
            for i in 0..formulas[0].len() {
                let mut qtt: isize = 0;
                for ing_id in 0..formulas.len() {
                    qtt += qtt_ingredient[ing_id] as isize * formulas[ing_id][i];
                }
                p.push(qtt);
            }
            if p[4] == 500 { // 500 calories               
             
                for pp in &p {
                    if *pp < 0_isize {
                        valid = false;
                    }
                }
                if valid {
                    product = p.iter().product();
                    product = product/500;
                    // println!("with {:?} : {:?} => {}",qtt_ingredient,p,product);
                    if product > max {
                        max = product;
                    }
                }
            }
        }
    }
    // println!("max = {max}");
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day15_input_demo1.txt")),
            57600000
        );
        assert_eq!(
            get_answer(include_str!("../assets/day15_input.txt")),
            15862900
        );
    }
}
