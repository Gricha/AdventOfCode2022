use std::collections::{HashSet, VecDeque};
use std::fmt::{self, Debug};

use itertools::Itertools;

use crate::utils::read_input;

#[derive(Eq, PartialEq, Hash)]
struct State {
    position: (usize, usize),
    minutes: usize,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Board(Vec<Vec<Terrain>>);

impl Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, line) in self.0.iter().enumerate() {
            for (j, _) in line.iter().enumerate() {
                match &self.0[i][j] {
                    Terrain::Empty => {
                        write!(f, ".").unwrap();
                    }
                    Terrain::Wall => {
                        write!(f, "#").unwrap();
                    }
                    Terrain::Storms(storms) => {
                        if storms.len() > 1 {
                            write!(f, "{}", storms.len()).unwrap();
                        } else {
                            let dir = storms.iter().next().unwrap();
                            match dir {
                                Direction::Up => write!(f, "^").unwrap(),
                                Direction::Down => write!(f, "v").unwrap(),
                                Direction::Left => write!(f, "<").unwrap(),
                                Direction::Right => write!(f, ">").unwrap(),
                            };
                        }
                    }
                }
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Position: {:?}", self.position).unwrap();
        writeln!(f, "Time: {}", self.minutes).unwrap();

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Terrain {
    Empty,
    Wall,
    Storms(Vec<Direction>),
}

impl Terrain {
    fn insert_storm(&mut self, direction: Direction) {
        match self {
            Terrain::Storms(x) => {
                assert!(!x.contains(&direction));
                x.push(direction);
            }
            _ => unreachable!(),
        }
    }

    fn is_empty_storm(&self) -> bool {
        match self {
            Terrain::Storms(x) => x.is_empty(),
            _ => false,
        }
    }
}

fn next_position_in_direction(
    board_size: (usize, usize),
    position: (usize, usize),
    direction: Direction,
) -> (usize, usize) {
    match direction {
        Direction::Up => {
            if position.0 == 1 {
                (board_size.0 - 2, position.1)
            } else if position.0 == 0 {
                //cheating
                (0, position.1)
            } else {
                (position.0 - 1, position.1)
            }
        }
        Direction::Down => {
            if position.0 == board_size.0 - 2 {
                (1, position.1)
            } else {
                (position.0 + 1, position.1)
            }
        }
        Direction::Left => {
            if position.1 == 1 {
                (position.0, board_size.1 - 2)
            } else if position.1 == 0 {
                (position.0, 0)
            } else {
                (position.0, position.1 - 1)
            }
        }
        Direction::Right => {
            if position.1 == board_size.1 - 2 {
                (position.0, 1)
            } else {
                (position.0, position.1 + 1)
            }
        }
    }
}

fn next_board(board: &Board) -> Board {
    let mut next_board = vec![];
    for line in board.0.iter() {
        let mut row = vec![];
        for t in line.iter() {
            if t == &Terrain::Wall {
                row.push(Terrain::Wall);
            } else {
                row.push(Terrain::Storms(Vec::new()))
            }
        }
        next_board.push(row);
    }
    let board_size = (board.0.len(), board.0[0].len());

    for (i, line) in board.0.iter().enumerate() {
        for (j, terr) in line.iter().enumerate() {
            if let Terrain::Storms(storms) = terr {
                for dir in storms.iter() {
                    let next_pos = next_position_in_direction(board_size, (i, j), *dir);
                    next_board[next_pos.0][next_pos.1].insert_storm(*dir);
                }
            }
        }
    }

    let vals = next_board
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(j, terrain)| {
                    if terrain.is_empty_storm() {
                        Some((i, j))
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    for pair in vals.into_iter() {
        next_board[pair.0][pair.1] = Terrain::Empty;
    }

    Board(next_board)
}

fn calculate_duration(
    boards: &Vec<Board>,
    start: (usize, usize),
    end: (usize, usize),
    starting_minute: usize,
) -> usize {
    let board_size = (boards[0].0.len(), boards[0].0[0].len());

    let mut queue = VecDeque::new();
    queue.push_back(State {
        position: start,
        minutes: starting_minute,
    });

    let mut cache = HashSet::new();

    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();
        let board = &boards[state.minutes % boards.len()];

        if state.position == end {
            return state.minutes;
        }

        if cache.contains(&(board, state.position)) {
            continue;
        }

        let (x, y) = state.position;

        let pos_left = next_position_in_direction(board_size, state.position, Direction::Left);
        let pos_right = next_position_in_direction(board_size, state.position, Direction::Right);
        let pos_up = next_position_in_direction(board_size, state.position, Direction::Up);
        let pos_down = next_position_in_direction(board_size, state.position, Direction::Down);

        let next_board = &boards[(state.minutes + 1) % boards.len()];

        if y > 0
            && board.0[x][y - 1] != Terrain::Wall
            && next_board.0[pos_left.0][pos_left.1] == Terrain::Empty
        {
            queue.push_back(State {
                position: (x, y - 1),
                minutes: state.minutes + 1,
                // past_moves: moves,
            })
        }

        if y < board_size.1 - 1
            && board.0[x][y + 1] != Terrain::Wall
            && next_board.0[pos_right.0][pos_right.1] == Terrain::Empty
        {
            queue.push_back(State {
                position: (x, y + 1),
                minutes: state.minutes + 1,
            })
        }

        if x > 0
            && board.0[x - 1][y] != Terrain::Wall
            && next_board.0[pos_up.0][pos_up.1] == Terrain::Empty
        {
            queue.push_back(State {
                position: (x - 1, y),
                minutes: state.minutes + 1,
            })
        }

        if (x + 1, y) == end
            || (x < board_size.0 - 1
                && board.0[x + 1][y] != Terrain::Wall
                && next_board.0[pos_down.0][pos_down.1] == Terrain::Empty)
        {
            queue.push_back(State {
                position: (x + 1, y),
                minutes: state.minutes + 1,
            })
        }

        if x == 0 || next_board.0[x][y] == Terrain::Empty {
            queue.push_back(State {
                position: (x, y),
                minutes: state.minutes + 1,
            });
        }

        cache.insert((board, state.position));
    }
    unreachable!()
}

fn read_data() -> (Vec<Board>, (usize, usize), (usize, usize)) {
    let mut board = vec![];

    let mut start = (usize::MAX, usize::MAX);
    let mut end = (usize::MAX, usize::MAX);

    let input = read_input("inputs/day24.txt");
    for (x, line) in input.iter().enumerate() {
        let mut row = vec![];
        for (y, c) in line.chars().enumerate() {
            if start.0 == usize::MAX && c == '.' {
                start = (x, y);
            }
            if c == '.' {
                end = (x, y);
            }

            match c {
                '.' => row.push(Terrain::Empty),
                '#' => row.push(Terrain::Wall),
                '<' => row.push(Terrain::Storms(vec![Direction::Left])),
                '>' => row.push(Terrain::Storms(vec![Direction::Right])),
                '^' => row.push(Terrain::Storms(vec![Direction::Up])),
                'v' => row.push(Terrain::Storms(vec![Direction::Down])),
                _ => unreachable!(),
            }
        }
        board.push(row);
    }

    let board_size = (board.len(), board[0].len());
    let mut boards = vec![];
    let mut initial = Board(board.clone());
    for _ in 0..(board_size.0 * board_size.1) {
        boards.push(initial.clone());
        initial = next_board(&initial);
    }

    (boards, start, end)
}

pub fn run_easy() {
    let (boards, start, end) = read_data();
    println!("{}", calculate_duration(&boards, start, end, 0));
}

pub fn run_hard() {
    let (boards, start, end) = read_data();

    let step1 = calculate_duration(&boards, start, end, 0);
    let step2 = calculate_duration(&boards, end, start, step1);
    let step3 = calculate_duration(&boards, start, end, step2);
    println!("{}", step3);
}
