use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 11 - Part 1 --");
    let now = Instant::now();

    println!("La réponse est {}", get_answer("hxbxwxba"));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(pwd: &str) -> String {
    let mut valid_pwd = String::from(pwd);
    get_next(&mut valid_pwd);

    while !is_valid(&valid_pwd) {
        get_next(&mut valid_pwd);
        //println!("{valid_pwd}");
    }
    valid_pwd.clone()
}

fn get_next(pwd: &mut String) {
    let mut chars: Vec<_> = pwd.chars().map(|c| c as u8).collect();
    for i in (0..chars.len()).rev() {
        chars[i] += 1;
        if chars[i] <= b'z' {
            break;
        } else {
            chars[i] = b'a';
        }
    }
    let mut next_pwd: String = String::new();
    chars
        .iter()
        .map(|&c| c as char)
        .for_each(|c| next_pwd.push(c));
    pwd.clear();
    pwd.push_str(&next_pwd);
}

fn is_valid(pwd: &String) -> bool {
    // println!("{pwd} : ");

    // not i,o,l
    if pwd.contains(['i', 'o', 'l']) {
        // println!("{pwd} contains i,o or l => not valid");
        return false;
    }

    let chars: Vec<_> = pwd.chars().map(|c| c as u8).collect();

    // 3 increasing letters
    let mut rule1 = false;
    for i in 0..(pwd.len() - 2) {
        if chars[i] + 2 == chars[i + 1] + 1 && chars[i + 1] + 1 == chars[i + 2] {
            // println!(
            //     "{pwd} contains {}{}{}",
            //     chars[i],
            //     chars[i + 1],
            //     chars[i + 2]
            // );
            rule1 = true;
            break;
        }
    }

    // 2 pairs of letters
    let mut rule2 = false;
    let mut first_pair: Option<usize> = None;
    for i in 0..(pwd.len() - 1) {
        if chars[i] == chars[i + 1] {
            if first_pair.is_none() {
                first_pair = Some(i);
            }
            if i > first_pair.unwrap() + 1 {
                // println!(
                //     "{pwd} contains {}{} and {}{}",
                //     chars[first_pair.unwrap()] as char,
                //     chars[first_pair.unwrap()+1] as char,
                //     chars[i] as char,
                //     chars[i + 1] as char
                // );
                rule2 = true;
                break;
            }
        }
    }
    rule1 && rule2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(is_valid(&String::from("hijklmmn")), false);
        assert_eq!(is_valid(&String::from("abbceffg")), false);
        assert_eq!(is_valid(&String::from("abbcegjk")), false);
        assert_eq!(get_answer("abcdefgh"), "abcdffaa");
        assert_eq!(get_answer("ghijklmn"), "ghjaabcc");
        assert_eq!(get_answer("hxbxwxba"), "hxbxxyzz");
    }
}
