use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 04 - Part 1 --");
    let now = Instant::now();

    let input = "ckczppom".to_string();

    println!("La réponse est {}", get_answer(&input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    let mut i = 0;
    loop {
        let str = format!("{input}{i}");
        let md5 = format!("{:?}", md5::compute(&str));
        //println!("le md5 de {str} est {:?}",md5);
        if md5.chars().take(6).collect::<String>() == "000000" {
            break;
        }
        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("ckczppom"), 3938038);
    }
}
