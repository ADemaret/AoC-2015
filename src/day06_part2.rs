use std::time::Instant;

// personal functions
// use crate::utils::grid2d::Grid2D;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 06 - Part 2 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day06_input_demo1.txt");
    let input = include_str!("../assets/day06_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> isize {
    const SIZE: usize = 1000;
    let mut grid = vec![vec![0; SIZE]; SIZE];

    // parse
    input.lines().for_each(|line| {
        let instruction: String = line.chars().take(7).collect();
        let coords: Vec<_> = line
            .split([',', ' '])
            .map(|s| s.chars().filter(|c| c.is_ascii_digit()).collect::<String>())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        // println!("{:?}",coords);
        assert_eq!(coords.len(), 4);
        match instruction.as_str() {
            "turn on" => {
                turn(&mut grid, &coords, 1);
            }
            "turn of" => {
                turn(&mut grid, &coords, -1);
            }
            "toggle " => {
                turn(&mut grid, &coords, 2);
            }
            _ => unreachable!("should be a valid instruction"),
        }
    });

    // print(&grid);

    let mut total = 0;
    for l in 0..SIZE {
        for c in 0..SIZE {
            total += grid[l][c];
        }
    }
    total
}

fn turn(grid: &mut [Vec<isize>], coords: &[isize], val: isize) {
    for l in coords[0]..=coords[2] {
        for c in coords[1]..=coords[3] {
            grid[l as usize][c as usize] += val;
            if grid[l as usize][c as usize] < 0 {
                grid[l as usize][c as usize] = 0;
            }
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
            17836115
        );
    }
}
