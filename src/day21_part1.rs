use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 21 - Part 1 --");
    let now = Instant::now();

    let input = include_str!("../assets/day21_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug)]
struct ShopItemCategory {
    weapons: Vec<ShopItem>,
    armors: Vec<ShopItem>,
    rings: Vec<ShopItem>,
}

#[derive(Debug, Clone)]
struct Properties {
    hit_points: isize,
    damage: isize,
    armor: isize,
}
#[derive(Debug, Clone)]
struct ShopItem {
    name: String,
    cost: usize,
    damage: isize,
    armor: isize,
}

fn get_answer(input: &str) -> usize {
    // parse input
    let boss = parse_input(input);

    // parse shop
    let shop = include_str!("../assets/day21_shop_input.txt");
    let parts: Vec<_> = shop.split("\n\n").collect();
    let shop_items = ShopItemCategory {
        weapons: get_shop_items(parts[0]),
        armors: get_shop_items(parts[1]),
        rings: get_shop_items(parts[2]),
    };

    // shopping options
    let possibilities = get_shopping_options(&shop_items);

    // new properties
    let mut properties = Vec::new();
    for p in possibilities {
        properties.push(get_properties(p, &shop_items));
    }

    properties.sort_by_key(|x| x.cost);

    for p in properties {
        let player = Properties {
            hit_points: 100,
            damage: p.damage,
            armor: p.armor,
        };
        //println!("bought {} at {} => {:?}",p.name, p.cost,player);
        if win_the_fight(&mut boss.clone(), &mut player.clone()) {
            // println!("the player wins with the following equipment : {}", p.name);
            return p.cost;
        }
    }
    0
}

fn get_properties(possibilities: Vec<Vec<usize>>, shop_items: &ShopItemCategory) -> ShopItem {
    let mut name = String::new();
    let mut cost = 0;
    let mut damage = 0;
    let mut armor = 0;
    for w in &possibilities[0] {
        name.push_str(shop_items.weapons[*w].name.as_str());
        name.push_str(", ");
        cost += shop_items.weapons[*w].cost;
        damage += shop_items.weapons[*w].damage;
        armor += shop_items.weapons[*w].armor;
    }
    for a in &possibilities[1] {
        name.push_str(shop_items.armors[*a].name.as_str());
        name.push_str(", ");
        cost += shop_items.armors[*a].cost;
        damage += shop_items.armors[*a].damage;
        armor += shop_items.armors[*a].armor;
    }
    for r in &possibilities[2] {
        name.push_str(shop_items.rings[*r].name.as_str());
        name.push_str(", ");
        cost += shop_items.rings[*r].cost;
        damage += shop_items.rings[*r].damage;
        armor += shop_items.rings[*r].armor;
    }
    ShopItem {
        name,
        cost,
        damage,
        armor,
    }
}

fn parse_input(input: &str) -> Properties {
    let b: Vec<_> = input
        .lines()
        .map(|line| {
            let (_, val) = line.split_once(": ").unwrap();
            val.parse::<isize>().unwrap()
        })
        .collect();
    Properties {
        hit_points: b[0],
        damage: b[1],
        armor: b[2],
    }
}

fn get_shop_items(input: &str) -> Vec<ShopItem> {
    input
        .lines()
        .map(|line| {
            let chunks: Vec<_> = line.split_whitespace().collect();
            let cost = chunks[1].parse().unwrap();
            let damage = chunks[2].parse().unwrap();
            let armor = chunks[3].parse().unwrap();
            ShopItem {
                name: chunks[0].to_string(),
                cost,
                damage,
                armor,
            }
        })
        .collect()
}

fn get_shopping_options(shop_items: &ShopItemCategory) -> Vec<Vec<Vec<usize>>> {
    let mut possibilities = Vec::new();

    // we must buy one weapon
    for w_id in 0..shop_items.weapons.len() {
        possibilities.push(vec![vec![w_id]]);
    }

    // armor is optional (none or one)
    let mut new_armor = Vec::new();
    for p in possibilities.iter_mut() {
        let p_clone = p.clone();
        p.push(Vec::new());
        for a in 0..shop_items.armors.len() {
            let mut new_vec = p_clone.clone();
            new_vec.push(vec![a]);
            new_armor.push(new_vec);
        }
    }
    for np in new_armor {
        possibilities.push(np);
    }

    let mut new_ring = Vec::new(); // nouvel objet
                                   // rings is optional (none or one or 2 but different)
    for p in possibilities.iter_mut() {
        let p_clone = p.clone();
        p.push(Vec::new());
        for r1 in 0..shop_items.rings.len() {
            let mut new_vec = p_clone.clone();
            new_vec.push(vec![r1]);
            new_ring.push(new_vec);
            for r2 in 0..shop_items.rings.len() {
                if r1 != r2 {
                    let mut new_vec = p_clone.clone();
                    new_vec.push(vec![r1, r2]);
                    new_ring.push(new_vec);
                }
            }
        }
    }
    for np in new_ring {
        possibilities.push(np);
    }

    possibilities
}

fn win_the_fight(boss: &mut Properties, player: &mut Properties) -> bool {
    // let mut turn = 1;
    loop {
        // the player attacks first
        boss.hit_points -= (player.damage - boss.armor).max(1);
        let boss_is_dead = boss.hit_points <= 0;

        // then the boss
        player.hit_points -= (boss.damage - player.armor).max(1);
        let player_is_dead = player.hit_points <= 0;

        if boss_is_dead {
            // println!("boss is dead at round {turn}");
            return true;
        }
        if player_is_dead {
            //println!("player is dead at round {turn}");
            return false;
        }
        // turn += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(include_str!("../assets/day21_input.txt")), 78);
    }
}
