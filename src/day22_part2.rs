use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
// use crate::utils::pause;
//use crate::utils::math;

const DEBUG : bool = false;

pub fn main() {
    println!("-- Advent of Code - Day 22 - Part 1 --");
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
    let temp_armor = 0;
    let temp_damage = 0;
    let temp_mana = 0;

    fight(
        boss,
        player,
        // &spells,
        &mut turn,
        temp_armor,
        temp_damage,
        temp_mana,
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
    lt_shield: usize,
    lt_poison: usize,
    lt_recharge: usize,
) -> Option<isize> {

    if player.hit_points <= 1 || player.mana <= 0 { // part2
        if DEBUG {println!("player is dead");}
        return None;
    }

    // none if no spell to cast
    let mut min_mana = None;
   
    let spells = ["MMiss", "Drain", "Shield", "Poison", "Recharge"];

    'all_spells: for spell in spells {
    
        // initialisations /////////////////////////////////////
        let cost: isize;
        let mut new_lt_shield = lt_shield;
        let mut new_lt_poison = lt_poison;
        let mut new_lt_recharge = lt_recharge;
        let mut new_boss = boss.clone();
        let mut new_player = player.clone();

        if DEBUG {
            println!("\n-- Player turn ({}) --",*turn);
            println!("- Player has {} hit points, {} armor, {} mana",new_player.hit_points, new_player.armor, new_player.mana);
            println!("- Boss has {} hit points",boss.hit_points);
        }

        new_player.hit_points -= 1; // part2
        
        // avant même un choix, on épuise (peut-être) les spells longs
        // on paie le spell (si possible)
        // recharge d'un tour PRECEDENT, s'il agit encore
        // au début du tour du joueur
        // utilisable pour sont cast du spell
        if new_lt_shield > 0 {
            new_player.armor = 7;
            new_lt_shield -= 1; // on consomme un effet
            if DEBUG {println!("Shield's timer is now {}.",new_lt_shield);}
            if new_lt_shield == 0 {
                if DEBUG {println!("Shield wears off, decreasing armor by 7.");}
                new_player.armor = 0;
            }
        }
        // poison => après avoir su payer le spell
        //
        if new_lt_recharge > 0 {
            new_player.mana += 101;
            new_lt_recharge -= 1; // on consomme un effet
            if DEBUG {println!("Recharge provides 101 mana; its timer is now {}.",new_lt_recharge);}
        }
        
        // choix du spell
        match spell {
            "MMiss" => {
                cost = 53;
                if DEBUG {println!("Player casts Magic Missile, dealing 4 damage.");}
            }
            "Drain" => {
                cost = 73;
                if DEBUG {println!("Player casts {}, dealing 2 damage, and healing 2 hit points.",spell);}
            }
            "Shield" => {
                cost = 113;
                if DEBUG {println!("Player casts {}, increasing armor by 7.",spell);}
                // si le précédent est encore actif, ce spell est refusé
                if new_lt_shield > 0 {
                    if DEBUG {println!("refused");}
                    continue 'all_spells;
                } else {
                    new_lt_shield = 6;
                }
            }
            "Poison" => {
                cost = 173;
                if DEBUG {println!("Player casts {}.",spell);}
                if new_lt_poison > 1 {
                    // s'il est à un on va l'utiliser immédiatement ci-dessous
                    if DEBUG {println!("refused");}
                    continue 'all_spells;
                } // else {
                    // new_lt_poison = 6; // later
                // }
            }
            "Recharge" => {
                cost = 229;
                if DEBUG {println!("Player casts {}.",spell);}
                // si le précédent est encore actif, ce spell est refusé
                if new_lt_recharge > 0 {
                    if DEBUG {println!("refused");}
                    continue 'all_spells;
                } else {
                    new_lt_recharge = 5;
                }
            }
            _ => unreachable!("should be a valid spell"),
        }

        // player turn ////////////////////////////////////////////

        // mana suffisante ?
        if cost > new_player.mana {
            if DEBUG {println!("too expensive");}
            continue 'all_spells;
        } else {
            // on paie le spell
            new_player.mana -= cost;
        }

        // poison ici !! (pour ne pas tuer le boss si on n'a plus de mana)
        // not directly at cast
        if spell=="Poison" {
            if new_lt_poison == 1 { // il restait 1 effet (qu'on utilise) et on recast tout de suite
            new_boss.hit_points -= 3;
            new_lt_poison = 6; // on consomme un effet
            if DEBUG {println!("Poison deals 3 damage; its timer is now {}.",new_lt_poison);}
            }
            new_lt_poison = 6;
        } else if new_lt_poison > 0 {
            new_boss.hit_points -= 3;
            new_lt_poison -= 1; // on consomme un effet
            if DEBUG {println!("Poison deals 3 damage; its timer is now {}.",new_lt_poison);}
        }

        // other effects
        match spell {
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
            if DEBUG {println!("boss is dead");}
            return Some(cost);
        }

        // then the boss /////////////////////////////////////////
        if DEBUG {
            println!("\n-- Boss turn ({}) --",*turn);
            println!("- Player has {} hit points, {} armor, {} mana",new_player.hit_points, new_player.armor, new_player.mana);
            println!("- Boss has {} hit points",new_boss.hit_points);
        }

        if new_lt_shield > 0 {
            new_player.armor = 7;
            new_lt_shield -= 1; // on consomme un effet
            if DEBUG {println!("Shield's timer is now {}.",new_lt_shield);}
            if new_lt_shield == 0 {
                if DEBUG {println!("Shield wears off, decreasing armor by 7.");}
                new_player.armor = 0;
            }
        } else {
            new_player.armor = 0;
        }
        if new_lt_poison > 0 {
            new_boss.hit_points -= 3;
            new_lt_poison -= 1; // on consomme un effet
            if DEBUG {println!("Poison deals 3 damage; its timer is now {}.",new_lt_poison);}
        }
        if new_lt_recharge > 0 {
            new_player.mana += 101;
            new_lt_recharge -= 1; // on consomme un effet
            if DEBUG {
                println!("Recharge provides 101 mana; its timer is now {}.",new_lt_recharge);
                if new_lt_recharge == 0 {
                    println!("Recharge wears off.");
                }
            }
        }

        // si ce qui précède a tué le boss, il ne se bat plus
        if new_boss.hit_points <= 0 {
            if DEBUG {println!("boss is dead");}
            return Some(cost);
        }

        // fight
        let damage = (new_boss.damage - new_player.armor).max(1);
        new_player.hit_points -= damage;
        if DEBUG {println!("Boss attacks for {} - {} = {} damage!",new_boss.damage, new_player.armor, damage);}

        *turn += 1;
        let new_mana = fight(
            new_boss,
            new_player,
            //spells,
            turn,
            new_lt_shield,
            new_lt_poison,
            new_lt_recharge,
        );
        *turn -= 1;

        if new_mana.is_some() {
            min_mana = Some(min_mana.unwrap_or(isize::MAX).min(new_mana.unwrap() + cost));
        }
    }
    min_mana
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(), Some(1309));
    }
}
