use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 20 - Part 2 --");
    let now = Instant::now();

    let input = 29_000_000;

    let rep = get_answer(input);
    if rep.is_some() {
        println!("La réponse est {}", rep.unwrap());
    } else {
        println!("Pas de solution");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: u64) -> Option<u64> {
    let mut visited_houses : Vec<usize> = vec![0;input as usize];
    for i in 1_u64..input {
        // on trouve les diviseurs
        // ex : 100 = 1,2,4,5,10,20,25,50,100
        let mut d = divisors::get_divisors(i);
        if i > 1 {
            d.push(1);
        }
        if !d.contains(&i) {
            d.push(i);
        }
        // on retient combien de maisons les elfes ont visités
        d.iter().for_each(|x| visited_houses[*x as usize]+=1);

        // on multiplie chacun par 11
        // ex : 10,20,40,50,100,200,250,500,1000
        // on les additionne s'ils sont apparus max 50 fois
        // ex : 2170
        let cadeaux: u64 = d.iter().filter(|x| visited_houses[**x as usize]<=50).map(|x| x * 11).sum();
        if cadeaux >= input {
            // println!("c'est la maison {} qui a reçu {} cadeaux", i, cadeaux);
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {        
        assert_eq!(get_answer(29_000_000), Some(705600));
    }
}
