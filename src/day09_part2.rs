use std::{
    collections::HashMap,
    time::Instant,
};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 09 - Part 2 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day09_input_demo1.txt");
    let input = include_str!("../assets/day09_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    let mut graph: HashMap<(usize, usize), usize> = HashMap::new();
    let mut index = HashMap::new();
    parse_input_file(input, &mut graph, &mut index);

    let mut distances = Vec::new();
    for start_node in &index {
        let mut other: Vec<usize> = Vec::new();
        for node in &index {
            if *node.1 != *start_node.1 {
                other.push(*node.1);
            }
        }
        let dist = tsp(&graph, *start_node.1, &mut other);
        distances.push(dist);        
    }
    *distances.iter().max().unwrap()
}

///
/// Traveling Salesperson Algorithm 
/// or
/// Held–Karp algorithm
/// modified for longuest path
/// 
fn tsp(
    graph: &HashMap<(usize, usize), usize>,
    current_node: usize,
    other: &mut Vec<usize>,
) -> usize {
    if other.is_empty() {
        return 0;
    } else {
        let mut dists = Vec::new();
        // pour tous les noeuds voisins
        let other_clone = other.clone();
        for node in other {
            let d1k = graph.get(&(current_node, *node)).unwrap();
            let mut other_minus_k = other_clone.clone();
            let k_index = other_minus_k.iter().position(|&x| x == *node).unwrap();
            other_minus_k.remove(k_index);
            let tsp_k = tsp(graph, *node, &mut other_minus_k);
            dists.push(d1k + tsp_k);
        }
        *dists.iter().max().unwrap()
    }
}

fn parse_input_file(
    input: &str,
    graph: &mut HashMap<(usize, usize), usize>,
    index: &mut HashMap<String, usize>,
) {
    input.lines().for_each(|line| {
        let (villes, dist) = line.split_once(" = ").unwrap();
        let (orig, dest) = villes.split_once(" to ").unwrap();
        if index.get(&orig.to_string()).is_none() {
            index.insert(orig.to_string(), index.len());
        }
        if index.get(&dest.to_string()).is_none() {
            index.insert(dest.to_string(), index.len());
        }
        let orig_id = index.get(orig).unwrap();
        let dest_id = index.get(dest).unwrap();
        graph.insert((*orig_id, *dest_id), dist.parse().unwrap());
        graph.insert((*dest_id, *orig_id), dist.parse().unwrap()); // both directions
    });
    // println!("{:?}", index);
    // println!("{:?}", graph);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day09_input_demo1.txt")),
            982
        );
        assert_eq!(get_answer(include_str!("../assets/day09_input.txt")), 804);
    }
}
