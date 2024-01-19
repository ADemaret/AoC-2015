use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

const DEBUG: bool = false;

#[derive(Default, Debug)]
enum Kind {
    #[default]
    Default,
    Not,
    And,
    Or,
    Lshift,
    Rshift,
}

#[derive(Default, Debug)]
struct Gates {
    kind: Kind,
    in_val: Option<u16>,
    in1: String,
    in2: String,
    out: String,
    done: bool,
}

pub fn main() {
    println!("-- Advent of Code - Day 07 - Part 1 --");
    let now = Instant::now();

    //println!("La réponse est {}", get_answer(include_str!("../assets/day07_input_demo1.txt"), "f".to_string()));
    println!("La réponse est {}", get_answer(include_str!("../assets/day07_input.txt"), "a".to_string()));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str, wire: String) -> u16 {
    // parse
    let mut gates = parse(input);

    let mut wires: HashMap<String, u16> = HashMap::new();
    loop {
        // on va passer plusieurs fois dans toute la liste
        // tant qu'il reste des signaux non traités
        let all_done = run_signal(&mut gates, &mut wires);
        if all_done {
            break;
        }
    }
    if DEBUG {
        println!("---");
        println!("valeurs finales :");
        for w in &wires {
            println!("{:?}", w);
        }
    }

    let a = *wires.get(&wire).unwrap();
    if DEBUG {
        println!("valeur de {} = {}", wire, a);
    }
    a
}

fn run_signal(gates: &mut [Gates], wires: &mut HashMap<String, u16>) -> bool {
    for g in gates.iter_mut() {
        if !g.done {
            match g.kind {
                Kind::Default => {
                    // toujours faisable
                    if g.in1.is_empty() {
                        // si on  a une valeur
                        wires.insert(g.out.clone(), g.in_val.unwrap());
                        g.done = true;
                        if DEBUG {
                            println!("{} -> {}", g.in_val.unwrap(), g.out);
                        }
                    } else {
                        // si on a un autre wire
                        if wires.get(&g.in1).is_some() {
                            let val = *wires.get(&g.in1).unwrap();
                            wires.insert(g.out.clone(), val);
                            g.done = true;
                            if DEBUG {
                                println!("{}({}) -> {}", g.in1, val, g.out);
                            }
                        }
                    }
                }
                Kind::Not => {
                    if wires.get(&g.in1).is_some() {
                        let val = *wires.get(&g.in1).unwrap();
                        wires.insert(g.out.clone(), !val);
                        g.done = true;
                        if DEBUG {
                            println!("!{}({}) -> {} = {}", g.in1, val, g.out, !val);
                        }
                    }
                }
                Kind::And => {
                    if g.in_val.is_some() {
                        if wires.get(&g.in2).is_some() {
                            let val1 = g.in_val.unwrap();
                            let val2 = *wires.get(&g.in2).unwrap();
                            let val = val1 & val2;
                            wires.insert(g.out.clone(), val);
                            g.done = true;
                            if DEBUG {
                                println!(
                                    "{}({}) & {}({}) -> {} = {}",
                                    g.in1, val1, g.in2, val2, g.out, val
                                );
                            }
                        }
                    } else if wires.get(&g.in1).is_some() && wires.get(&g.in2).is_some() {
                        let val1 = *wires.get(&g.in1).unwrap();
                        let val2 = *wires.get(&g.in2).unwrap();
                        let val = val1 & val2;
                        wires.insert(g.out.clone(), val);
                        g.done = true;
                        if DEBUG {
                            println!(
                                "{}({}) & {}({}) -> {} = {}",
                                g.in1, val1, g.in2, val2, g.out, val
                            );
                        }
                    }
                }
                Kind::Or => {
                    if wires.get(&g.in1).is_some() && wires.get(&g.in2).is_some() {
                        let val1 = *wires.get(&g.in1).unwrap();
                        let val2 = *wires.get(&g.in2).unwrap();
                        let val = val1 | val2;
                        wires.insert(g.out.clone(), val);
                        g.done = true;
                        if DEBUG {
                            println!(
                                "{}({}) | {}({}) -> {} = {}",
                                g.in1, val1, g.in2, val2, g.out, val
                            );
                        }
                    }
                }
                Kind::Lshift => {
                    if wires.get(&g.in1).is_some() {
                        let val1 = *wires.get(&g.in1).unwrap();
                        let val = val1 << g.in_val.unwrap();
                        wires.insert(g.out.clone(), val);
                        g.done = true;
                        if DEBUG {
                            println!(
                                "{}({}) << {} -> {} = {}",
                                g.in1,
                                val1,
                                g.in_val.unwrap(),
                                g.out,
                                val
                            );
                        }
                    }
                }
                Kind::Rshift => {
                    if wires.get(&g.in1).is_some() {
                        let val1 = *wires.get(&g.in1).unwrap();
                        let val = val1 >> g.in_val.unwrap();
                        wires.insert(g.out.clone(), val);
                        g.done = true;
                        if DEBUG {
                            println!(
                                "{}({}) >> {} -> {} = {}",
                                g.in1,
                                val1,
                                g.in_val.unwrap(),
                                g.out,
                                val
                            );
                        }
                    }
                }
            }
        }
    }

    let mut all_done = true;
    for g in gates {
        if !g.done {
            all_done = false;
            break;
        }
    }
    if all_done {
        return true;
    }
    false
}

fn parse(input: &str) -> Vec<Gates> {
    let mut gates: Vec<Gates> = Vec::new();
    input.lines().for_each(|line| {
        if line.contains(" AND ") {
            let parts = line
                .split(" AND ")
                .flat_map(|x| x.split(" -> ").collect::<Vec<_>>())
                .collect::<Vec<_>>();
            if parts[0].parse::<u16>().is_ok() {
                // il y a des "1 AND ..."
                gates.push(Gates {
                    kind: Kind::And,
                    in_val: Some(parts[0].parse::<u16>().unwrap()),
                    in2: parts[1].to_string(),
                    out: parts[2].to_string(),
                    ..Default::default()
                });
            } else {
                gates.push(Gates {
                    kind: Kind::And,
                    in1: parts[0].to_string(),
                    in2: parts[1].to_string(),
                    out: parts[2].to_string(),
                    ..Default::default()
                });
            }
        } else if line.contains(" OR ") {
            let parts = line
                .split(" OR ")
                .flat_map(|x| x.split(" -> ").collect::<Vec<_>>())
                .collect::<Vec<_>>();
            gates.push(Gates {
                kind: Kind::Or,
                in1: parts[0].to_string(),
                in2: parts[1].to_string(),
                out: parts[2].to_string(),
                ..Default::default()
            });
        } else if line.contains(" LSHIFT ") {
            let parts = line
                .split(" LSHIFT ")
                .flat_map(|x| x.split(" -> ").collect::<Vec<_>>())
                .collect::<Vec<_>>();
            gates.push(Gates {
                kind: Kind::Lshift,
                in1: parts[0].to_string(),
                in_val: Some(parts[1].parse::<u16>().unwrap()),
                out: parts[2].to_string(),
                ..Default::default()
            });
        } else if line.contains(" RSHIFT ") {
            let parts = line
                .split(" RSHIFT ")
                .flat_map(|x| x.split(" -> ").collect::<Vec<_>>())
                .collect::<Vec<_>>();
            gates.push(Gates {
                kind: Kind::Rshift,
                in1: parts[0].to_string(),
                in_val: Some(parts[1].parse::<u16>().unwrap()),
                out: parts[2].to_string(),
                ..Default::default()
            });
        } else if line.contains("NOT ") {
            let parts = line
                .split("NOT ")
                .flat_map(|x| x.split(" -> ").collect::<Vec<_>>())
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();
            gates.push(Gates {
                kind: Kind::Not,
                in1: parts[0].to_string(),
                out: parts[1].to_string(),
                ..Default::default()
            });
        } else {
            let parts = line.split(" -> ").collect::<Vec<_>>();
            if parts[0].parse::<u16>().is_ok() {
                gates.push(Gates {
                    kind: Kind::Default,
                    in_val: Some(parts[0].parse::<u16>().unwrap()),
                    out: parts[1].to_string(),
                    ..Default::default()
                });
            } else {
                gates.push(Gates {
                    kind: Kind::Default,
                    in1: parts[0].to_string(),
                    out: parts[1].to_string(),
                    ..Default::default()
                });
            }
        }
    });
    if DEBUG {
        println!("{:#?}", gates);
    }
    gates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(include_str!("../assets/day07_input_demo1.txt"), "f".to_string()),492);
        assert_eq!(get_answer(include_str!("../assets/day07_input.txt"), "a".to_string()), 16076);
    }
}
