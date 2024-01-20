use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 13 - Part 2 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day13_input_demo1.txt");
    let input = include_str!("../assets/day13_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> isize {
    let mut graph: HashMap<(usize, usize), isize> = HashMap::new();
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
        let dist = tsp(&graph, *start_node.1,*start_node.1, &mut other);
        distances.push(dist);
    }
    *distances.iter().min().unwrap()
    
}

///
/// Traveling Salesperson Algorithm
/// or
/// Held–Karp algorithm
///
fn tsp(
    graph: &HashMap<(usize, usize), isize>,
    start_node:usize,
    current_node: usize,
    other: &mut Vec<usize>,
) -> isize {
    if other.is_empty() {
        //return 0;
        // on doit ajouter la distance entre le départ et ce point
        return *graph.get(&(current_node, start_node)).unwrap();
    } else {
        let mut dists = Vec::new();
        // pour tous les noeuds voisins
        let other_clone = other.clone();
        for node in other {
            let d1k = graph.get(&(current_node, *node)).unwrap();
            let mut other_minus_k = other_clone.clone();
            let k_index = other_minus_k.iter().position(|&x| x == *node).unwrap();
            other_minus_k.remove(k_index);
            let tsp_k = tsp(graph, start_node,*node, &mut other_minus_k);
            dists.push(d1k + tsp_k);
        }
        *dists.iter().max().unwrap()
    }
}

fn parse_input_file(
    input: &str,
    graph: &mut HashMap<(usize, usize), isize>,
    index: &mut HashMap<String, usize>,
) {
    input.lines().for_each(|line| {
        let (parts, to1) = line.split_once(" happiness units by sitting next to ").unwrap();
        let (to,_) = to1.split_once('.').unwrap();
        let (from,parts2) = parts.split_once(" would ").unwrap();
        let (sign,val) = parts2.split_once(' ').unwrap();        
        let value : isize = if sign == "gain" {
            val.parse().unwrap()
        } else {
            -val.parse::<isize>().unwrap()
        };
        if index.get(&from.to_string()).is_none() {
            index.insert(from.to_string(), index.len());
        }
        if index.get(&to.to_string()).is_none() {
            index.insert(to.to_string(), index.len());
        }
        let from_id = index.get(from).unwrap();
        let to_id = index.get(to).unwrap();
        if !graph.contains_key(&(*to_id,*from_id)) {
            graph.insert((*from_id, *to_id), value);
        } else {
            let current_val = &graph.get(&(*to_id,*from_id)).unwrap().clone();
            graph.insert((*from_id, *to_id), *current_val + value);
            graph.insert((*to_id, *from_id), *current_val + value); // both directions
    }        
    });

    // part2
    index.insert("Me".to_string(),index.len());
    for i in 0..(index.len()-1) {
        graph.insert((index.len()-1,i),0);
        graph.insert((i,index.len()-1),0);
    }

    // println!("{:?}", index);
    // println!("{:?}", graph);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(include_str!("../assets/day13_input.txt")), 640);
    }
}
