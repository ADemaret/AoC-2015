use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 24 - Part 2 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day24_input_demo1.txt");
    let input = include_str!("../assets/day24_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    // parse
    let mut weights: Vec<usize> = input.lines().map(|w| w.parse().unwrap()).collect();
    weights.sort();
    weights.reverse();

    let total_weight: usize = weights.iter().sum();
    let group_weight = (total_weight as f32 / 4.0) as usize;
    // println!("chaque compartiment contiendra {} kg de colis",group_weight);

    // powersets
    let all_subs = powerset(group_weight, &weights);
    // println!("all_subs = {}",all_subs.len());
    let mut subs = all_subs
        .iter()
        .filter(|&s| s.into_iter().sum::<usize>() == group_weight)
        .collect::<Vec<_>>();
    // println!("subs = {}",subs.len());
    subs.sort_by_key(|s| s.len());
    let min_len = subs[0].len();

    let mut entanglements = Vec::new();
    for s in subs {
        if s.len() == min_len {
            let qe = s.iter().product::<usize>();
            // println!("{:?} => qe = {}",s,qe);
            entanglements.push(qe);
        }
    }
    let min = entanglements.iter().min().unwrap();
    // println!("smaller qe = {}",min);
    *min
}

///
/// all possible combinaisons of weights
///
fn powerset(max: usize, s: &[usize]) -> Vec<Vec<usize>> {
    let mut subsets: Vec<Vec<usize>> = vec![];
    let empty: Vec<usize> = vec![];
    subsets.push(empty);

    let mut updated: Vec<Vec<usize>> = vec![];
    let mut minimum_items = s.len();
    for item in s {
        for mut sub in subsets.clone() {
            if sub.iter().sum::<usize>() < max {
                sub.push(*item);
                updated.push(sub);
            }
        }
        // si on arrive au total avec un minimum de colis, on ne tient plus compte des
        // combinaisons avec plus de colis
        if minimum_items == s.len() {
            for u in &updated {
                if u.iter().sum::<usize>() == max {                
                    minimum_items = u.len();
                }
            }
        }
        // et on efface même les combinaisons plus longues
        if minimum_items < s.len() {
            for i in (0..updated.len()).rev() {
                if updated[i].len() > minimum_items {
                    updated.remove(i);
                }
            }
        }
        subsets.append(&mut updated);
    }

    subsets
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day24_input_demo1.txt")),
            44
        );
        assert_eq!(
            get_answer(include_str!("../assets/day24_input.txt")),
            74850409
        );
    }
}
