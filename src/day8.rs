use itertools::Itertools;

use crate::utils::read_input;

pub fn run_easy() {
    let height_map = read_input("inputs/day8.txt")
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|x| x.to_string().parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let columns = height_map[0].len();
    let rows = height_map.len();

    let mut visibility_map = vec![];
    let mut max_height_map = vec![];
    for i in 0..rows {
        let mut row = vec![];
        let mut height_row = vec![];

        for j in 0..columns {
            row.push(false);
            let height = height_map[i][j];
            height_row.push((height, height, height, height));
        }

        visibility_map.push(row);
        max_height_map.push(height_row);
    }

    // borders
    for i in 0..rows {
        for j in 0..columns {
            if i == 0 || j == 0 || i == rows - 1 || j == columns - 1 {
                visibility_map[i][j] = true;
            }
        }
    }

    for i in 0..rows {
        for j in 0..columns {
            if i > 0 {
                max_height_map[i][j].0 =
                    std::cmp::max(max_height_map[i][j].0, max_height_map[i - 1][j].0);
            }
            if j > 0 {
                max_height_map[i][j].1 =
                    std::cmp::max(max_height_map[i][j].1, max_height_map[i][j - 1].1);
            }

            if visibility_map[i][j] {
                continue;
            }

            if max_height_map[i - 1][j].0 < height_map[i][j] {
                visibility_map[i][j] = true;
            }

            if max_height_map[i][j - 1].1 < height_map[i][j] {
                visibility_map[i][j] = true;
            }
        }
    }

    for i in (0..rows).rev() {
        for j in (0..columns).rev() {
            if i < rows - 1 {
                max_height_map[i][j].2 =
                    std::cmp::max(max_height_map[i][j].2, max_height_map[i + 1][j].2);
            }
            if j < columns - 1 {
                max_height_map[i][j].3 =
                    std::cmp::max(max_height_map[i][j].3, max_height_map[i][j + 1].3);
            }

            if visibility_map[i][j] {
                continue;
            }

            if max_height_map[i + 1][j].2 < height_map[i][j] {
                visibility_map[i][j] = true;
            }

            if max_height_map[i][j + 1].3 < height_map[i][j] {
                visibility_map[i][j] = true;
            }
        }
    }
    let sum: u32 = visibility_map
        .iter()
        .map(|row| -> u32 { row.iter().map(|x| *x as u32).sum() })
        .sum();
    println!("{}", sum);
}

pub fn run_hard() {
    let height_map = read_input("inputs/day8.txt")
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|x| x.to_string().parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let columns = height_map[0].len();
    let rows = height_map.len();

    let mut scenic_map = vec![];
    let mut debug_scenic = vec![];
    for _i in 0..rows {
        let mut height_row = vec![];
        let mut debug_scenic_row = vec![];

        for _j in 0..columns {
            height_row.push(1);
            debug_scenic_row.push((0, 0, 0, 0));
        }

        scenic_map.push(height_row);
        debug_scenic.push(debug_scenic_row);
    }

    for i in 0..rows {
        let mut index_of_highest_so_far = [0; 10];
        for j in 0..columns {
            let j_height: usize = height_map[i][j] as usize;
            let earlier_that_was_higher =
                *index_of_highest_so_far[j_height..10].iter().max().unwrap();
            let distance = j - earlier_that_was_higher;
            scenic_map[i][j] *= distance;
            debug_scenic[i][j].0 = distance;

            index_of_highest_so_far[j_height] = j;
        }
    }

    for j in 0..columns {
        let mut index_of_highest_so_far = [0; 10];
        for i in 0..rows {
            let i_height: usize = height_map[i][j] as usize;
            let earlier_that_was_higher =
                *index_of_highest_so_far[i_height..10].iter().max().unwrap();
            let distance = i - earlier_that_was_higher;
            scenic_map[i][j] *= distance;
            debug_scenic[i][j].1 = distance;
            index_of_highest_so_far[i_height] = i;
        }
    }

    for i in (0..rows).rev() {
        let mut index_of_highest_so_far = [(columns - 1); 10];
        for j in (0..columns).rev() {
            let j_height: usize = height_map[i][j] as usize;
            let earlier_that_was_higher =
                *index_of_highest_so_far[j_height..10].iter().min().unwrap();
            let distance = earlier_that_was_higher - j;
            scenic_map[i][j] *= distance;
            debug_scenic[i][j].2 = distance;

            index_of_highest_so_far[j_height] = j;
        }
    }

    for j in (0..columns).rev() {
        let mut index_of_highest_so_far = [(rows - 1); 10];
        for i in (0..rows).rev() {
            let i_height: usize = height_map[i][j] as usize;
            let earlier_that_was_higher =
                *index_of_highest_so_far[i_height..10].iter().min().unwrap();
            let distance = earlier_that_was_higher - i;
            scenic_map[i][j] *= distance;
            debug_scenic[i][j].3 = distance;

            index_of_highest_so_far[i_height] = i;
        }
    }

    let max_scenic = scenic_map
        .iter()
        .map(|r| r.iter().max().unwrap())
        .max()
        .unwrap();
    println!("{}", max_scenic);
}
