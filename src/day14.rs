use itertools::Itertools;

use crate::utils::read_input;

pub fn run_easy() {
    let lines = read_input("inputs/day14.txt")
        .into_iter()
        .map(|x| {
            x.split(" -> ")
                .map(|z| z.to_string())
                .map(|n| {
                    let vals: (&str, &str) = n.split(',').collect_tuple().unwrap();
                    (
                        vals.0.parse::<usize>().unwrap(),
                        vals.1.parse::<usize>().unwrap(),
                    )
                })
                .collect_vec()
        })
        .collect_vec();

    let mut map = [[0; 800]; 800];

    for crack in lines {
        let mut starting_point = crack[0];
        for point in crack.iter().skip(1) {
            let delta_vector = (
                point.0 as i32 - starting_point.0 as i32,
                point.1 as i32 - starting_point.1 as i32,
            );
            let goes_vertical = delta_vector.0 == 0;

            if goes_vertical {
                let range = if delta_vector.1 < 0 {
                    (delta_vector.1)..=0
                } else {
                    0..=delta_vector.1
                };
                for i in range {
                    map[starting_point.0][(starting_point.1 as i32 + i) as usize] = 1;
                }
            } else {
                let range = if delta_vector.0 < 0 {
                    (delta_vector.0)..=0
                } else {
                    0..=delta_vector.0
                };
                for i in range {
                    map[(starting_point.0 as i32 + i) as usize][starting_point.1] = 1;
                }
            }

            starting_point = *point;
        }
    }

    let mut num = -1;
    loop {
        let mut sand_position = (500, 0);
        num += 1;
        loop {
            if sand_position.1 == 799 {
                println!("{}", num);
                return;
            }
            if map[sand_position.0][sand_position.1 + 1] == 0 {
                sand_position.1 += 1;
            } else if map[sand_position.0 - 1][sand_position.1 + 1] == 0 {
                sand_position.0 -= 1;
                sand_position.1 += 1;
            } else if map[sand_position.0 + 1][sand_position.1 + 1] == 0 {
                sand_position.0 += 1;
                sand_position.1 += 1;
            } else {
                map[sand_position.0][sand_position.1] = 2;
                break;
            }
        }
    }
}

pub fn run_hard() {
    let lines = read_input("inputs/day14.txt")
        .into_iter()
        .map(|x| {
            x.split(" -> ")
                .map(|z| z.to_string())
                .map(|n| {
                    let vals: (&str, &str) = n.split(',').collect_tuple().unwrap();
                    (
                        vals.0.parse::<usize>().unwrap(),
                        vals.1.parse::<usize>().unwrap(),
                    )
                })
                .collect_vec()
        })
        .collect_vec();

    let mut map = [[0; 800]; 800];

    let mut max_y = 0;
    for crack in lines {
        let mut starting_point = crack[0];
        for point in crack.iter().skip(1) {
            let delta_vector = (
                point.0 as i32 - starting_point.0 as i32,
                point.1 as i32 - starting_point.1 as i32,
            );
            let goes_vertical = delta_vector.0 == 0;

            if goes_vertical {
                let range = if delta_vector.1 < 0 {
                    (delta_vector.1)..=0
                } else {
                    0..=delta_vector.1
                };
                for i in range {
                    map[starting_point.0][(starting_point.1 as i32 + i) as usize] = 1;
                    max_y = std::cmp::max(max_y, starting_point.1);
                }
            } else {
                let range = if delta_vector.0 < 0 {
                    (delta_vector.0)..=0
                } else {
                    0..=delta_vector.0
                };
                for i in range {
                    map[(starting_point.0 as i32 + i) as usize][starting_point.1] = 1;
                    max_y = std::cmp::max(max_y, starting_point.1);
                }
            }

            starting_point = *point;
        }
    }

    let floor_level = max_y + 2;
    for i in 0..800 {
        map[i][floor_level] = 1;
    }

    let mut num = 0;
    loop {
        let mut sand_position = (500, 0);
        num += 1;
        loop {
            if map[sand_position.0][sand_position.1 + 1] == 0 {
                sand_position.1 += 1;
            } else if map[sand_position.0 - 1][sand_position.1 + 1] == 0 {
                sand_position.0 -= 1;
                sand_position.1 += 1;
            } else if map[sand_position.0 + 1][sand_position.1 + 1] == 0 {
                sand_position.0 += 1;
                sand_position.1 += 1;
            } else {
                map[sand_position.0][sand_position.1] = 2;

                if sand_position == (500, 0) {
                    println!("{}", num);
                    return;
                }

                break;
            }
        }
    }
}
