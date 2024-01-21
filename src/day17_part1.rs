use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 17 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day17_input_demo1.txt");
    let input = include_str!("../assets/day17_input.txt");

    println!("La réponse est {}", get_answer(input, 150));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str, total: usize) -> usize {
    let mut containers: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();
    containers.sort();

    // toutes les possibilités y compris celles qui n'arrivent pas au total
    let mut possibilities: Vec<Vec<usize>> = Vec::new();
    possibilities.push(Vec::new());
    for c in (0..containers.len()).rev() {
        // println!("on a {:?}",possibilities);
        fill_possibilities(&containers, total, c, &mut possibilities);
    }
    // println!("{:?}", possibilities);
    // on ne garde que celles dont la valeur est bonne
    let mut val = 0;
    for p in possibilities {
        if get_total(&containers, &p) == total {
            // println!("bonne combinaison : {:?}", p);
            val += 1;
        }
    }
    val
}

fn fill_possibilities(
    containers: &Vec<usize>,
    total: usize,
    c: usize,
    possibilities: &mut Vec<Vec<usize>>,
) {
    let mut to_add: Vec<Vec<_>> = Vec::new();
    let mut new_possibilities;
    // println!("container de taille {}", containers[c]);
    for p in possibilities.iter_mut() {
        // println!("on a actuellement {:?} pour un total de {}",p,get_total(&containers,&p));
        let p_clone = p.clone();
        let reste = total - get_total(containers, p);
        let qtt_max = if containers[c] <= reste { 1 } else { 0 };
        // println!("on peut en ajouter de 0 à {qtt_max} à {:?}",p);
        for i in 0..=qtt_max {
            //qtt_max {
            if i == 0 {
                p.push(i);
            } else {
                new_possibilities = Vec::new();
                for item in &p_clone {
                    new_possibilities.push(*item);
                }
                new_possibilities.push(i);
                to_add.push(new_possibilities);
            }
        }
    }
    for x in to_add {
        possibilities.push(x);
    }
}

fn get_total(containers: &Vec<usize>, p: &Vec<usize>) -> usize {
    let mut total = 0;
    for i in 0..p.len() {
        total += p[i] * containers[containers.len() - i - 1];
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day17_input_demo1.txt"), 25),
            4
        );
        assert_eq!(
            get_answer(include_str!("../assets/day17_input.txt"), 150),
            4372
        );
    }
}
