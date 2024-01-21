use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 19 - Part 2 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day19_input_demo2.txt");
    let input = include_str!("../assets/day19_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    // parse
    // on va chercher de la chaîne d'arrivé à la chaîne ne départ
    // comme ça, quand la taille diminue, on est dans le bon sens
    // on va donc remplacer de "to" en "from"
    let (rep, start_molecule) = input.split_once("\n\n").unwrap();
    let replacements: Vec<_> = rep
        .lines()
        .map(|line| line.split_once(" => ").unwrap())
        .collect();
    // println!("replacements = {:?}", replacements);
    // println!("molecule = {}", molecule);

    let mut compteur = 1;
    let mut molecules: Vec<String> = Vec::new();
    molecules.push(start_molecule.to_string());
    loop {
        let mut new_molecules: Vec<String> = Vec::new();
        for molecule in &molecules {
            for index in 0..molecule.len() {
                for (from, to) in &replacements {
                    let chunk = molecule
                        .chars()
                        .skip(index)
                        .take(to.len())
                        .collect::<String>();
                    if chunk == *to {
                        let mut buff = String::from("");
                        buff.push_str(molecule.chars().take(index).collect::<String>().as_str());
                        buff.push_str(from);
                        buff.push_str(
                            molecule
                                .chars()
                                .skip(index + to.len())
                                .collect::<String>()
                                .as_str(),
                        );
                        new_molecules.push(buff.clone());
                        //println!("on replace {} par {} -> {}", chunk, to, buff);
                        if buff == "e" {
                            // println!("Trouvé !");
                            return compteur;
                        }
                    }
                }
            }
        }
        new_molecules.sort();
        new_molecules.dedup();
        // let min_len = new_molecules.iter().map(|x| x.len()).min().unwrap();
        // let max_len = new_molecules.iter().map(|x| x.len()).max().unwrap();
        new_molecules.sort_by_key(|a| a.len());

        // println!(
        //     " size = {} to {} in {} molecules",
        //     min_len,
        //     max_len,
        //     new_molecules.len()
        // );
        // on ne garde que les 40 plus courtes chaînes
        for i in (0..new_molecules.len()).rev() {
            if i > 40 {
                new_molecules.remove(i);
            }
        }
        // if new_molecules.len() < 10 {
        //     println!("{:#?}",new_molecules);
        // }

        molecules.clear();
        for nm in new_molecules {
            molecules.push(nm);
        }
        compteur += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day19_input_demo2.txt")),
            6
        );
        assert_eq!(get_answer(include_str!("../assets/day19_input.txt")), 195);
    }
}
