use std::time::Instant;

// personal functions
// use crate::utils::grid2d::Grid2D;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 06 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day06_input_demo1.txt");
    let input = include_str!("../assets/day06_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    const SIZE: usize = 1000;
    let mut grid = vec![vec![false; SIZE]; SIZE];

    // parse
    input.lines().for_each(|line| {
        let instruction: String = line.chars().take(7).collect();
        let coords: Vec<_> = line
            .split([',', ' '])
            .map(|s| s.chars().filter(|c| c.is_ascii_digit()).collect::<String>())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        // println!("{:?}",coords);
        assert_eq!(coords.len(), 4);
        match instruction.as_str() {
            "turn on" => {
                turn(&mut grid, &coords, true);
            }
            "turn of" => {
                turn(&mut grid, &coords, false);
            }
            "toggle " => {
                toggle(&mut grid, &coords);
            }
            _ => unreachable!("should be a valid instruction"),
        }
    });

    let mut total = 0;
    for l in 0..SIZE {
        for c in 0..SIZE {
            if grid[l][c] {
                total += 1;
            }
        }
    }
    total
}

fn turn(grid: &mut [Vec<bool>], coords: &[usize], val: bool) {
    for l in coords[0]..=coords[2] {
        for c in coords[1]..=coords[3] {
            grid[l][c] = val;
        }
    }
}

fn toggle(grid: &mut [Vec<bool>], coords: &[usize]) {
    for l in coords[0]..=coords[2] {
        for c in coords[1]..=coords[3] {
            grid[l][c] = !grid[l][c];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day06_input.txt")),
            569999
        );
    }
}
