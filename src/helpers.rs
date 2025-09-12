use crate::single_puzzle::{Move, Face, Direction};
use crate::scramble::Scramble;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn to_string_face(face: Face) -> &'static str {
    match face {
        Face::TopLeft => "TL",
        Face::Left => "L",
        Face::BottomLeft => "BL",
        Face::TopRight => "TR",
        Face::Right => "R",
        Face::BottomRight => "BR",
    }
}

pub fn to_string_direction(direction: Direction) -> &'static str {
    match direction {
        Direction::Clockwise => "CW",
        Direction::CounterClockwise => "CCW",
    }
}

pub fn get_all_moves() -> Vec<Move> {
    let all_faces = [
        Face::TopLeft,
        Face::Left,
        Face::BottomLeft,
        Face::TopRight,
        Face::Right,
        Face::BottomRight,
    ];
    let all_directions = [Direction::Clockwise, Direction::CounterClockwise];
    let mut all_moves = Vec::new();
    for face in all_faces.iter() {
        for direction in all_directions.iter() {
            all_moves.push(Move::new(*face, *direction));
        }
    }
    all_moves
}

pub fn get_random_scramble(num_moves: usize) -> Scramble {
    let all_faces = [
        Face::TopLeft,
        Face::Left,
        Face::BottomLeft,
        Face::TopRight,
        Face::Right,
        Face::BottomRight,
    ];
    let all_directions = [Direction::Clockwise, Direction::CounterClockwise];
    let mut rng = thread_rng();
    let mut scramble = Vec::new();
    for _ in 0..num_moves {
        let face = all_faces.choose(&mut rng).unwrap();
        let direction = all_directions.choose(&mut rng).unwrap();
        scramble.push(Move::new(*face, *direction));
    }
    Scramble { moves: scramble }
}

pub fn get_color(num: u8) -> u8 {
    match num {
        0 | 4 | 5 | 23 => 0,    // white
        1 | 2 | 3 | 6 => 1,     // red
        7 | 8 | 9 | 10 => 2,    // blue
        11 | 12 | 13 | 14 => 3, // orange
        15 | 16 | 17 | 18 => 4, // green
        19 | 20 | 21 | 22 => 5, // yellow
        _ => panic!("Invalid number"),
    }
}

pub fn calculate_neighbours(num: u8) -> Vec<u8> {
    match num {
        0 => vec![1, 5],
        1 => vec![0, 2],
        2 => vec![1, 3, 6],
        3 => vec![2, 4],
        4 => vec![3, 5, 21],
        5 => vec![0, 4, 23],
        6 => vec![2, 7],
        7 => vec![6, 8],
        8 => vec![7, 9, 10],
        9 => vec![3, 8, 13],
        10 => vec![8, 11],
        11 => vec![10, 12],
        12 => vec![11, 13, 14],
        13 => vec![9, 12, 17],
        14 => vec![12, 15],
        15 => vec![14, 16],
        16 => vec![15, 17, 18],
        17 => vec![13, 16, 21],
        18 => vec![17, 19],
        19 => vec![18, 21],
        20 => vec![19, 21, 22],
        21 => vec![4, 17, 20],
        22 => vec![21, 23],
        23 => vec![5, 22],
        _ => panic!("Invalid number"),
    }
}

pub fn permutations(input: Vec<u8>) -> Vec<Vec<u8>> {
    fn helper(prefix: Vec<u8>, remainder: Vec<u8>, acc: &mut Vec<Vec<u8>>) {
        if remainder.is_empty() {
            acc.push(prefix);
        } else {
            for i in 0..remainder.len() {
                let mut next_prefix = prefix.clone();
                next_prefix.push(remainder[i]);
                let mut next_remainder = remainder.clone();
                next_remainder.remove(i);
                helper(next_prefix, next_remainder, acc);
            }
        }
    }
    let mut acc = Vec::new();
    helper(Vec::new(), input, &mut acc);
    acc
}
