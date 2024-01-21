use std::time::Instant;

// personal functions
use crate::utils::grid2d::Grid2D;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 18 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day18_input_demo1.txt");
    let input = include_str!("../assets/day18_input.txt");

    println!("La réponse est {}", get_answer(input, 100));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str, nbr: usize) -> usize {
    let mut grid: Grid2D = Grid2D::new(input);
    //grid.print();
    let mut new_grid = grid.clone();

    for _ in 0..nbr {
        for l in 0..grid.max_l {
            for c in 0..grid.max_c {
                new_grid.grid[l][c] = get_new_state(&grid, l, c);
            }
        }
        grid = new_grid.clone();
        //grid.print();
    }
    grid.count_occurences('#')
}

fn get_new_state(grid: &Grid2D, l0: usize, c0: usize) -> char {
    let mut nbr_on = 0;
    for l in (l0 as isize - 1)..=(l0 as isize + 1) {
        for c in (c0 as isize - 1)..=(c0 as isize + 1) {
            if l >= 0
                && c >= 0
                && l < grid.max_l as isize
                && c < grid.max_c as isize
                && (l as usize, c as usize) != (l0, c0)
                && grid.grid[l as usize][c as usize] == '#'
            {
                nbr_on += 1;
            }
        }
    }
    if grid.grid[l0][c0] == '#' {
        if nbr_on == 2 || nbr_on == 3 {
            '#'
        } else {
            '.'
        }
    } else {
        // off
        if nbr_on == 3 {
            '#'
        } else {
            '.'
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day18_input_demo1.txt"), 4),
            4
        );
        assert_eq!(
            get_answer(include_str!("../assets/day18_input.txt"), 100),
            814
        );
    }
}
