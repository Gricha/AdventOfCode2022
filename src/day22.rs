use crate::utils::read_input;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn apply_turn(self, turn: Turn) -> Self {
        match turn {
            Turn::Left => match self {
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Up => Direction::Left,
            },
            Turn::Right => match self {
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
            },
        }
    }

    fn get_value(&self) -> i32 {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
enum Move {
    Turn(Turn),
    Step(i32),
}

#[derive(Debug)]
struct Position {
    coordinates: (usize, usize),
    direction: Direction,
}

// 0 - nothing
// 1 - moved
// 2 - hit the wall
fn special_move(board: &[[i32; 220]; 220], position: &mut Position) -> i32 {
    if position.direction == Direction::Left {
        // pick up left edges
        if position.coordinates.0 == 51
            && position.coordinates.1 <= 50
            && position.coordinates.1 >= 1
        {
            let coordinate_delta = position.coordinates.1 - 1;

            let new_coordinates = (1, 150 - coordinate_delta);

            if board[new_coordinates.0][new_coordinates.1] == 1 {
                position.coordinates = (1, 150 - coordinate_delta);
                position.direction = Direction::Right;
                return 1;
            }
            if board[new_coordinates.0][new_coordinates.1] == 2 {
                return 2;
            }
        }

        if position.coordinates.0 == 51
            && position.coordinates.1 <= 100
            && position.coordinates.1 >= 51
        {
            let coordinate_delta = position.coordinates.1 - 51;
            let new_coordinates = (1 + coordinate_delta, 101);
            if board[new_coordinates.0][new_coordinates.1] == 1 {
                position.coordinates = new_coordinates;
                position.direction = Direction::Down;
                return 1;
            }
            if board[new_coordinates.0][new_coordinates.1] == 2 {
                return 2;
            }
        }

        if position.coordinates.0 == 1
            && position.coordinates.1 >= 101
            && position.coordinates.1 <= 150
        {
            let coordinate_delta = position.coordinates.1 - 101;
            let new_coordinates = (51, 50 - coordinate_delta);
            if board[new_coordinates.0][new_coordinates.1] == 1 {
                position.coordinates = new_coordinates;
                position.direction = Direction::Right;
                return 1;
            }
            if board[new_coordinates.0][new_coordinates.1] == 2 {
                return 2;
            }
        }

        if position.coordinates.0 == 1
            && position.coordinates.1 >= 151
            && position.coordinates.1 <= 200
        {
            let coordinate_delta = position.coordinates.1 - 151;
            let new_coordinates = (51 + coordinate_delta, 1);

            if board[new_coordinates.0][new_coordinates.1] == 1 {
                position.coordinates = new_coordinates;
                position.direction = Direction::Down;
                return 1;
            }
            if board[new_coordinates.0][new_coordinates.1] == 2 {
                return 2;
            }
        }
    }

    if position.direction == Direction::Right {
        if position.coordinates.0 == 150
            && position.coordinates.1 >= 1
            && position.coordinates.1 <= 50
        {
            let coordinate_delta = position.coordinates.1 - 1;
            let new_coordinates = (100, 150 - coordinate_delta);
            if board[new_coordinates.0][new_coordinates.1] == 1 {
                position.coordinates = new_coordinates;
                position.direction = Direction::Left;
                return 1;
            }
            if board[new_coordinates.0][new_coordinates.1] == 2 {
                return 2;
            }
        }

        if position.coordinates.0 == 100
            && position.coordinates.1 >= 51
            && position.coordinates.1 <= 100
        {
            let coordinate_delta = position.coordinates.1 - 51;
            let new_coordinates = (101 + coordinate_delta, 50);
            if board[new_coordinates.0][new_coordinates.1] == 1 {
                position.coordinates = new_coordinates;
                position.direction = Direction::Up;
                return 1;
            }
            if board[new_coordinates.0][new_coordinates.1] == 2 {
                return 2;
            }
        }

        if position.coordinates.0 == 100
            && position.coordinates.1 >= 101
            && position.coordinates.1 <= 150
        {
            let coordinate_delta = position.coordinates.1 - 101;
            let new_coordinates = (150, 50 - coordinate_delta);
            if board[new_coordinates.0][new_coordinates.1] == 1 {
                position.coordinates = new_coordinates;
                position.direction = Direction::Left;
                return 1;
            }
            if board[new_coordinates.0][new_coordinates.1] == 2 {
                return 2;
            }
        }

        if position.coordinates.0 == 50
            && position.coordinates.1 >= 151
            && position.coordinates.1 <= 200
        {
            let coordinate_delta = position.coordinates.1 - 151;
            let new_coordinates = (51 + coordinate_delta, 150);
            if board[new_coordinates.0][new_coordinates.1] == 1 {
                position.coordinates = new_coordinates;
                position.direction = Direction::Up;
                return 1;
            }
            if board[new_coordinates.0][new_coordinates.1] == 2 {
                return 2;
            }
        }
    }

    if position.direction == Direction::Up {
        if position.coordinates.1 == 1
            && position.coordinates.0 >= 51
            && position.coordinates.0 <= 100
        {
            let coordinate_delta = position.coordinates.0 - 51;
            let new_coordinates = (1, 151 + coordinate_delta);
            if board[new_coordinates.0][new_coordinates.1] == 1 {
                position.coordinates = new_coordinates;
                position.direction = Direction::Right;
                return 1;
            }
            if board[new_coordinates.0][new_coordinates.1] == 2 {
                return 2;
            }
        }

        if position.coordinates.1 == 1
            && position.coordinates.0 >= 101
            && position.coordinates.0 <= 150
        {
            let coordinate_delta = position.coordinates.0 - 101;
            let new_coordinates = (1 + coordinate_delta, 200);
            if board[new_coordinates.0][new_coordinates.1] == 1 {
                position.coordinates = new_coordinates;
                position.direction = Direction::Up;
                return 1;
            }
            if board[new_coordinates.0][new_coordinates.1] == 2 {
                return 2;
            }
        }

        if position.coordinates.1 == 101
            && position.coordinates.0 >= 1
            && position.coordinates.0 <= 50
        {
            let coordinate_delta = position.coordinates.0 - 1;
            let new_coordinates = (51, 51 + coordinate_delta);
            if board[new_coordinates.0][new_coordinates.1] == 1 {
                position.coordinates = new_coordinates;
                position.direction = Direction::Right;
                return 1;
            }
            if board[new_coordinates.0][new_coordinates.1] == 2 {
                return 2;
            }
        }
    }

    if position.direction == Direction::Down {
        if position.coordinates.1 == 200
            && position.coordinates.0 >= 1
            && position.coordinates.0 <= 50
        {
            let coordinate_delta = position.coordinates.0 - 1;
            let new_coordinates = (101 + coordinate_delta, 1);
            if board[new_coordinates.0][new_coordinates.1] == 1 {
                position.coordinates = new_coordinates;
                position.direction = Direction::Down;
                return 1;
            }
            if board[new_coordinates.0][new_coordinates.1] == 2 {
                return 2;
            }
        }

        if position.coordinates.1 == 150
            && position.coordinates.0 >= 51
            && position.coordinates.0 <= 100
        {
            let coordinate_delta = position.coordinates.0 - 51;
            let new_coordinates = (50, 151 + coordinate_delta);
            if board[new_coordinates.0][new_coordinates.1] == 1 {
                position.coordinates = new_coordinates;
                position.direction = Direction::Left;
                return 1;
            }
            if board[new_coordinates.0][new_coordinates.1] == 2 {
                return 2;
            }
        }

        if position.coordinates.1 == 50
            && position.coordinates.0 >= 101
            && position.coordinates.0 <= 150
        {
            let coordinate_delta = position.coordinates.0 - 101;
            let new_coordinates = (100, 51 + coordinate_delta);
            if board[new_coordinates.0][new_coordinates.1] == 1 {
                position.coordinates = new_coordinates;
                position.direction = Direction::Left;
                return 1;
            }
            if board[new_coordinates.0][new_coordinates.1] == 2 {
                return 2;
            }
        }
    }

    0
}

fn try_move(
    board: &[[i32; 220]; 220],
    row_spans: &[(usize, usize); 220],
    col_spans: &[(usize, usize); 220],
    position: &mut Position,
    should_special: bool,
) -> bool {
    if should_special {
        let res = special_move(board, position);
        if res == 1 {
            return true;
        }
        if res == 2 {
            return false;
        }
    }
    let destination = match position.direction {
        Direction::Right => {
            let mut destination = (position.coordinates.0 + 1, position.coordinates.1);

            if !should_special && board[destination.0][destination.1] == 0 {
                destination = (
                    row_spans[destination.1].0.try_into().unwrap(),
                    destination.1,
                );
            }

            destination
        }
        Direction::Left => {
            let mut destination = (position.coordinates.0 - 1, position.coordinates.1);

            if !should_special && board[destination.0][destination.1] == 0 {
                destination = (
                    row_spans[destination.1].1.try_into().unwrap(),
                    destination.1,
                );
            }

            destination
        }
        Direction::Down => {
            let mut destination = (position.coordinates.0, position.coordinates.1 + 1);

            if !should_special && board[destination.0][destination.1] == 0 {
                destination = (
                    destination.0,
                    col_spans[destination.0].0.try_into().unwrap(),
                );
            }

            destination
        }
        Direction::Up => {
            let mut destination = (position.coordinates.0, position.coordinates.1 - 1);

            if !should_special && board[destination.0][destination.1] == 0 {
                destination = (
                    destination.0,
                    col_spans[destination.0].1.try_into().unwrap(),
                );
            }

            destination
        }
    };

    if board[destination.0][destination.1] == 2 {
        false
    } else {
        position.coordinates = destination;
        true
    }
}

pub fn run_easy() {
    let input = read_input("inputs/day22.txt");

    let mut board = [[0; 220]; 220];

    let mut input_slices = input.split(|x| x.is_empty());
    let mut starting_position_coordinates = (0, 0);

    let map_slice = input_slices.next().unwrap();
    for (j, line) in map_slice.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            match c {
                ' ' => {}
                '.' => {
                    if starting_position_coordinates == (0, 0) {
                        starting_position_coordinates = (i + 1, j + 1);
                    }
                    board[i + 1][j + 1] = 1;
                }
                '#' => board[i + 1][j + 1] = 2,
                _ => unreachable!(),
            }
        }
    }

    let moves_slice = input_slices.next().unwrap().iter().next().unwrap().clone();

    let mut moves = vec![];
    let mut curr_number = 0;

    for char in moves_slice.chars() {
        if char.is_ascii_digit() {
            curr_number *= 10;
            curr_number += char as i32 - '0' as i32;
        } else {
            moves.push(Move::Step(curr_number));
            curr_number = 0;
            match char {
                'L' => moves.push(Move::Turn(Turn::Left)),
                'R' => moves.push(Move::Turn(Turn::Right)),
                _ => unreachable!(),
            }
        }
    }
    if curr_number > 0 {
        moves.push(Move::Step(curr_number));
    }

    let mut row_spans = [(0, 0); 220];
    let mut col_spans = [(0, 0); 220];

    for j in 0..220 {
        let mut column_start = 5000;
        let mut column_end = 5000;
        for i in 0..220 {
            if board[i][j] > 0 {
                if column_start == 5000 {
                    column_start = i;
                }
                column_end = i
            }
        }
        row_spans[j] = (column_start, column_end);
    }

    for i in 0..220 {
        let mut row_start = 5000;
        let mut row_end = 5000;
        for j in 0..220 {
            if board[i][j] > 0 {
                if row_start == 5000 {
                    row_start = j;
                }
                row_end = j;
            }
        }
        col_spans[i] = (row_start, row_end);
    }

    let mut position = Position {
        coordinates: starting_position_coordinates,
        direction: Direction::Right,
    };

    for mv in moves {
        match mv {
            Move::Turn(t) => position.direction = position.direction.apply_turn(t),
            Move::Step(n) => {
                for i in 0..n {
                    let result = try_move(&board, &row_spans, &col_spans, &mut position, false);
                    if !result {
                        break;
                    }
                }
            }
        }
    }

    dbg!(
        position.coordinates.1 * 1000
            + position.coordinates.0 * 4
            + position.direction.get_value() as usize
    );
}

pub fn run_hard() {
    let input = read_input("inputs/day22.txt");

    let mut board = [[0; 220]; 220];

    let mut input_slices = input.split(|x| x.is_empty());
    let mut starting_position_coordinates = (0, 0);

    let map_slice = input_slices.next().unwrap();
    for (j, line) in map_slice.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            match c {
                ' ' => {}
                '.' => {
                    if starting_position_coordinates == (0, 0) {
                        starting_position_coordinates = (i + 1, j + 1);
                    }
                    board[i + 1][j + 1] = 1;
                }
                '#' => board[i + 1][j + 1] = 2,
                _ => unreachable!(),
            }
        }
    }

    let moves_slice = input_slices.next().unwrap().iter().next().unwrap().clone();

    let mut moves = vec![];
    let mut curr_number = 0;

    for char in moves_slice.chars() {
        if char.is_ascii_digit() {
            curr_number *= 10;
            curr_number += char as i32 - '0' as i32;
        } else {
            moves.push(Move::Step(curr_number));
            curr_number = 0;
            match char {
                'L' => moves.push(Move::Turn(Turn::Left)),
                'R' => moves.push(Move::Turn(Turn::Right)),
                _ => unreachable!(),
            }
        }
    }
    if curr_number > 0 {
        moves.push(Move::Step(curr_number));
    }

    let mut row_spans = [(0, 0); 220];
    let mut col_spans = [(0, 0); 220];

    for j in 0..220 {
        let mut column_start = 5000;
        let mut column_end = 5000;
        for i in 0..220 {
            if board[i][j] > 0 {
                if column_start == 5000 {
                    column_start = i;
                }
                column_end = i
            }
        }
        row_spans[j] = (column_start, column_end);
    }

    for i in 0..220 {
        let mut row_start = 5000;
        let mut row_end = 5000;
        for j in 0..220 {
            if board[i][j] > 0 {
                if row_start == 5000 {
                    row_start = j;
                }
                row_end = j;
            }
        }
        col_spans[i] = (row_start, row_end);
    }

    let mut side_thickness = i32::MAX;
    for i in 0..220 {
        let thickness = row_spans[i].1 - row_spans[i].0 + 1;

        if thickness > 1 {
            side_thickness = std::cmp::min((thickness) as i32, side_thickness);
        }
    }

    let mut position = Position {
        coordinates: starting_position_coordinates,
        direction: Direction::Right,
    };

    for mv in moves {
        match mv {
            Move::Turn(t) => position.direction = position.direction.apply_turn(t),
            Move::Step(n) => {
                for i in 0..n {
                    let result = try_move(&board, &row_spans, &col_spans, &mut position, true);
                    if !result {
                        break;
                    }
                }
            }
        }
    }

    dbg!(
        position.coordinates.1 * 1000
            + position.coordinates.0 * 4
            + position.direction.get_value() as usize
    );
}
