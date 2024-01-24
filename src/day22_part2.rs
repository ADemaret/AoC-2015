use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 22 - Part 2 --");
    let now = Instant::now();

    let answer = get_answer();
    if answer.is_some() {
        println!("La réponse est {}", answer.unwrap());
    } else {
        println!("pas de réponse trouvée")
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug, Clone)]
struct Players {
    mana: isize,
    hit_points: isize,
    damage: isize,
    armor: isize,
}

fn get_answer() -> Option<isize> {
    // parse input
    let boss = Players {
        hit_points: 58,
        damage: 9,
        mana: 0,
        armor: 0,
    };

    let player = Players {
        hit_points: 50,
        damage: 0,
        mana: 500,
        armor: 0,
    };

    let mut turn = 1;
    //let mut mana_spend = 0;
    let temp_armor = (0, 0);
    let temp_damage = (0, 0);
    let temp_mana = (0, 0);

    let debug_cost = 0;
    fight(
        boss,
        player,
        // &spells,
        &mut turn,
        &temp_armor,
        &temp_damage,
        &temp_mana,
        //debug_cost,
    )
}

///
/// fight is a depth first search
///
fn fight(
    boss: Players,
    player: Players,
    // spells: &Vec<Spell>,
    turn: &mut usize,
    lt_shield: &(isize, usize), // value, last turn
    lt_poison: &(isize, usize),
    lt_recharge: &(isize, usize),
    //debug_cost: isize,
) -> Option<isize> {
    // trouvé
    if boss.hit_points <= 0 {
        //println!("boss is dead at cost {} after {} turns", debug_cost, turn);
        return Some(0);
    }
    if player.hit_points <= 2 {
        // part 2
        //println!("player is dead");
        return None;
    }

    // none if no spell to cast
    let mut min_mana = None;

    let spells = ["MMiss", "Drain", "Shield", "Poison", "Recharge"];

    'all_spells: for spell in spells.iter().rev() {
        // for _ in 0..*turn {
        //     print!("  ");
        // }
        // println!("{} : {}", turn, spell);

        // initialisations /////////////////////////////////////
        let cost: isize;
        let mut new_lt_shield = *lt_shield;
        let mut new_lt_poison = *lt_poison;
        let mut new_lt_recharge = *lt_recharge;
        let mut new_boss = boss.clone();
        let mut new_player = player.clone();
        //let mut new_debug_cost = debug_cost;

        // part2
        new_player.hit_points -= 1;

        // avant même un choix, on épuise (peut-être) les spells longs
        // on paie le spell (si possible)
        // recharge d'un tour PRECEDENT, s'il agit encore
        // au début du tour du joueur
        // utilisable pour sont cast du spell
        if new_lt_shield.1 > 0 {
            new_player.armor = new_lt_shield.0;
            new_lt_shield.1 -= 1; // on consomme un effet
        }
        // poison = après avoir su payer le spell
        if new_lt_recharge.1 > 0 {
            new_player.mana += new_lt_recharge.0;
            new_lt_recharge.1 -= 1; // on consomme un effet
        }

        match *spell {
            "MMiss" => {
                cost = 53;
            }
            "Drain" => {
                cost = 73;
            }
            "Shield" => {
                cost = 113;
                // si le précédent est encore actif, ce spell est refusé
                if new_lt_shield.1 > 0 {
                    // println!("refused");
                    continue 'all_spells;
                } else {
                    new_lt_shield = (7, 5); // 5 turn, not 6, to follow same logic
                }
            }
            "Poison" => {
                cost = 173;
                // si le précédent est encore actif, ce spell est refusé
                if new_lt_poison.1 > 1 {
                    // s'il est à un on va l'utiliser immédiatement ci-dessous
                    // println!("refused");
                    continue 'all_spells;
                } else {
                    new_lt_poison = (3, 6);
                }
            }
            "Recharge" => {
                cost = 229;
                // si le précédent est encore actif, ce spell est refusé
                if new_lt_recharge.1 > 0 {
                    // println!("refused");
                    continue 'all_spells;
                } else {
                    new_lt_recharge = (101, 5);
                }
            }
            _ => unreachable!("should be a valid spell"),
        }

        // player turn ////////////////////////////////////////////

        // mana suffisante ?
        if cost > new_player.mana {
            // println!("too expensive");
            continue 'all_spells;
        } else {
            // on paie le spell
            new_player.mana -= cost;
            //new_debug_cost += cost;
        }

        // poison ici !! (pour ne pas tuer le boss si on n'a plus de mana)
        if new_lt_poison.1 > 0 {
            new_boss.hit_points -= new_lt_poison.0;
            new_lt_poison.1 -= 1; // on consomme un effet
        }

        // other effects
        match *spell {
            "MMiss" => {
                new_boss.hit_points -= 4;
            }
            "Drain" => {
                new_boss.hit_points -= 2;
                new_player.hit_points += 2;
            }
            _ => {}
        }

        // si ce qui précède a tué le boss, il ne se bat plus
        if new_boss.hit_points <= 0 {
            // println!(
            //     "boss is dead at cost {} after {} turns",
            //     new_debug_cost, turn
            // );
            return Some(cost);
        }

        // then the boss /////////////////////////////////////////
        if new_lt_shield.1 > 0 {
            new_player.armor = new_lt_shield.0;
            new_lt_shield.1 -= 1; // on consomme un effet
        } else {
            new_player.armor = 0;
        }
        if new_lt_poison.1 > 0 {
            new_boss.hit_points -= new_lt_poison.0;
            new_lt_poison.1 -= 1; // on consomme un effet
        }
        if new_lt_recharge.1 > 0 {
            new_player.mana += new_lt_recharge.0;
            new_lt_recharge.1 -= 1; // on consomme un effet
        }

        // si ce qui précède a tué le boss, il ne se bat plus
        if new_boss.hit_points <= 0 {
            // println!(
            //     "boss is dead at cost {} after {} turns",
            //     new_debug_cost, turn
            // );
            return Some(cost);
        }

        // fight
        new_player.hit_points -= (new_boss.damage - new_player.armor).max(1);

        //pause::pause();

        *turn += 1;
        let new_mana = fight(
            new_boss,
            new_player,
            //spells,
            turn,
            &new_lt_shield,
            &new_lt_poison,
            &new_lt_recharge,
            //new_debug_cost,
        );
        *turn -= 1;

        if new_mana.is_some() {
            min_mana = Some(min_mana.unwrap_or(isize::MAX).min(new_mana.unwrap() + cost));
        }
    }
    min_mana
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_total() {
//         assert_eq!(get_answer(), Some(78));
//     }
// }
