use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 12 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day12_input_demo2.txt");
    let input = include_str!("../assets/day12_input.txt");

    let mut i2 = input.to_string();
    remove_whitespace(&mut i2);
    println!("La réponse est {}", get_answer(i2.as_str()));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}
//

fn get_answer(input: &str) -> isize {
    let mut chunks: Vec<(usize, char, String)> = Vec::new();
    let mut level = 0_usize;
    let mut sep = ' ';
    let mut in_red = None;
    let mut contenu = String::from("");
    for c in input.chars() {
        match c {
            '{' => {
                //println!("on rencontre '{{'");
                process_string(&mut chunks, level, sep, &mut contenu, &mut in_red);
                sep = '{';
                level += 1;
            }
            '}' => {
                //println!("on rencontre '}}'");
                process_string(&mut chunks, level, sep, &mut contenu, &mut in_red);
                if in_red.is_some_and(|ir| ir == level) {
                    in_red = None;
                    //println!("in_red is None");
                }
                sep = ' ';
                level -= 1;
            }
            '[' => {
                //println!("on rencontre '['");
                process_string(&mut chunks, level, sep, &mut contenu, &mut in_red);
                sep = '[';
                level += 1;
            }
            ']' => {
                //println!("on rencontre ']'");
                process_string(&mut chunks, level, sep, &mut contenu, &mut in_red);
                if in_red.is_some_and(|ir| ir == level) {
                    in_red = None;
                    //println!("in_red is None");
                }
                sep = ' ';
                level -= 1;
            }
            '\n' => {}
            ch => contenu.push(ch),
        }
    }
    if in_red.is_none() || level < in_red.unwrap() {
        if !contenu.is_empty() {
            if sep == '{' && contenu.contains("red") {
                //in_red = Some(level);
                //println!("-{}, {}, {}", level, sep, contenu);
            } else {
                chunks.push((level, sep, contenu.clone()));
                //println!("+{}, {}, {}", level, sep, contenu);
            }
        }
    }
    //println!("{:#?}", chunks);

    let mut total = 0;
    for (_, _, chunk) in chunks {
        let vec = chunk
            .split(|c: char| !c.is_ascii_digit() && c != '-')
            .filter(|str| !str.is_empty())
            .map(|str| str.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        //println!("{:?}",vec);
        total += vec.iter().sum::<isize>();
    }
    total
}

fn process_string(
    chunks: &mut Vec<(usize, char, String)>,
    level: usize,
    sep: char,
    contenu: &mut String,
    in_red: &mut Option<usize>,
) {
    if !contenu.is_empty() {
        if in_red.is_none() || level < in_red.unwrap() {
            if contenu.contains(":\"red\"") {
                *in_red = Some(level);
                //println!("in_red = {}", in_red.unwrap());
                //println!("-{}, {}, {}", level, sep, contenu);
                // on doit aussi supprimer les contenus précédents de même niveau
                // ou de niveau inférieur jusqu'à la "{"
                if sep != '{' {
                    let mut break_here = false;
                    while !break_here && chunks.last().is_some() && chunks.last().unwrap().0 >= level
                    {
                        if chunks.last().unwrap().0 == level && chunks.last().unwrap().1 == '{' {
                            break_here = true;
                        }
                        //println!("on efface {:?}", chunks.last().unwrap());
                        chunks.remove(chunks.len() - 1);                        
                    }
                }
            } else {
                chunks.push((level, sep, contenu.clone()));
                //println!("+{}, {}, {}", level, sep, contenu);
            }
        } else {
            //println!("-{}, {}, {}", level, sep, contenu);
        }
        contenu.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo2.txt")),
            16
        );
        assert_eq!(get_answer(include_str!("../assets/day12_input.txt")), 96852);
    }
}
