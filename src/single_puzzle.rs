use crate::helpers::{calculate_neighbours, get_color};
use crate::puzzle_trait::PuzzleTrait;
use crate::scramble::Scramble;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Face {
    TopLeft = 0b001,
    Left = 0b010,
    BottomLeft = 0b100,
    TopRight = 0b011,
    Right = 0b101,
    BottomRight = 0b110,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    Clockwise = 0b1,
    CounterClockwise = 0b0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Move {
    pub face: Face,
    pub direction: Direction,
}

impl Move {
    pub fn new(face: Face, direction: Direction) -> Self {
        Self { face, direction }
    }

    pub fn get_inverted_move(&self) -> Self {
        match self.direction {
            Direction::Clockwise => Move::new(self.face, Direction::CounterClockwise),
            Direction::CounterClockwise => Move::new(self.face, Direction::Clockwise),
        }
    }

    pub fn get_opposite_move(&self) -> Self {
        let opposite_face = match self.face {
            Face::TopLeft => Face::BottomRight,
            Face::Left => Face::Right,
            Face::BottomLeft => Face::TopRight,
            Face::TopRight => Face::BottomLeft,
            Face::Right => Face::Left,
            Face::BottomRight => Face::TopLeft,
        };
        let opposite_direction = match self.direction {
            Direction::Clockwise => Direction::CounterClockwise,
            Direction::CounterClockwise => Direction::Clockwise,
        };
        Move::new(opposite_face, opposite_direction)
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {};",
            crate::helpers::to_string_face(self.face),
            crate::helpers::to_string_direction(self.direction)
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SinglePuzzle {
    pub with_opposite_move: bool,
    pub scramble: Option<Scramble>,
    pub slots: Vec<u8>,
    pub colors: Vec<u8>,
}

impl PartialOrd for SinglePuzzle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SinglePuzzle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.colors.cmp(&other.colors)
    }
}

impl SinglePuzzle {
    pub fn get_scramble(&self) -> Scramble {
        match &self.scramble {
            Some(scramble) => scramble.clone(),
            None => Scramble { moves: Vec::new() },
        }
    }

    pub fn new_solved(with_opposite_move: bool) -> Self {
        Self {
            scramble: None,
            slots: (0..=23).collect(),
            colors: (0..=23).map(get_color).collect(),
            with_opposite_move,
        }
    }

    pub fn new_scrambled(scramble: Scramble, with_opposite_move: bool) -> Self {
        let mut puzzle = SinglePuzzle {
            scramble: Some(scramble.clone()),
            slots: vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23,
            ],
            colors: (0..=23).map(get_color).collect(),
            with_opposite_move,
        };
        for mv in scramble.moves {
            puzzle.apply_move(mv, true);
        }
        puzzle.deduce_colors();
        puzzle
    }

    pub fn calculate_score(&self) -> i64 {
        let mut score = 0;
        for (i, &color) in self.colors.iter().enumerate() {
            let neighbours = calculate_neighbours(i as u8);
            for &neighbour in neighbours.iter() {
                if self.colors[neighbour as usize] == color {
                    score += 1;
                }
            }
        }
        score
    }

    pub fn deduce_colors(&mut self) {
        self.colors = self.slots.iter().map(|&num| get_color(num)).collect();
    }

    pub fn apply_scramble(&mut self, scramble: Scramble) {
        self.scramble = Some(scramble.clone());
        for mv in scramble.moves {
            self.apply_move(mv, true);
        }
        self.colors = self.slots.iter().map(|&num| get_color(num)).collect();
    }

    pub fn apply_move(&mut self, mv: Move, apply_opposite: bool) {
        if self.with_opposite_move && apply_opposite {
            let opposite_mv = mv.get_opposite_move();
            self.apply_move(opposite_mv, false);
        }
        // ...existing code for move application...
        match mv.face {
            Face::TopLeft => match mv.direction {
                Direction::Clockwise => {
                    let first_one = self.slots[5];
                    self.slots[5] = self.slots[23];
                    self.slots[23] = self.slots[22];
                    self.slots[22] = self.slots[20];
                    self.slots[20] = self.slots[21];
                    self.slots[21] = self.slots[4];
                    self.slots[5] = first_one;
                }
                Direction::CounterClockwise => {
                    let first_one = self.slots[5];
                    self.slots[5] = self.slots[4];
                    self.slots[4] = self.slots[21];
                    self.slots[21] = self.slots[20];
                    self.slots[20] = self.slots[22];
                    self.slots[22] = self.slots[23];
                    self.slots[23] = first_one;
                }
            },
            Face::Left => match mv.direction {
                Direction::Clockwise => {
                    let first_one = self.slots[20];
                    self.slots[20] = self.slots[19];
                    self.slots[19] = self.slots[18];
                    self.slots[18] = self.slots[16];
                    self.slots[16] = self.slots[17];
                    self.slots[17] = self.slots[21];
                    self.slots[21] = first_one;
                }
                Direction::CounterClockwise => {
                    let first_one = self.slots[20];
                    self.slots[20] = self.slots[21];
                    self.slots[21] = self.slots[17];
                    self.slots[17] = self.slots[16];
                    self.slots[16] = self.slots[18];
                    self.slots[18] = self.slots[19];
                    self.slots[19] = first_one;
                }
            },
            Face::BottomLeft => match mv.direction {
                Direction::Clockwise => {
                    let first_one = self.slots[17];
                    self.slots[17] = self.slots[16];
                    self.slots[16] = self.slots[15];
                    self.slots[15] = self.slots[14];
                    self.slots[14] = self.slots[12];
                    self.slots[12] = self.slots[13];
                    self.slots[13] = first_one;
                }
                Direction::CounterClockwise => {
                    let first_one = self.slots[17];
                    self.slots[17] = self.slots[13];
                    self.slots[13] = self.slots[12];
                    self.slots[12] = self.slots[14];
                    self.slots[14] = self.slots[15];
                    self.slots[15] = self.slots[16];
                    self.slots[16] = first_one;
                }
            },
            Face::TopRight => match mv.direction {
                Direction::Clockwise => {
                    let first_one = self.slots[0];
                    self.slots[0] = self.slots[5];
                    self.slots[5] = self.slots[4];
                    self.slots[4] = self.slots[3];
                    self.slots[3] = self.slots[2];
                    self.slots[2] = self.slots[1];
                    self.slots[1] = first_one;
                }
                Direction::CounterClockwise => {
                    let first_one = self.slots[0];
                    self.slots[0] = self.slots[1];
                    self.slots[1] = self.slots[2];
                    self.slots[2] = self.slots[3];
                    self.slots[3] = self.slots[4];
                    self.slots[4] = self.slots[5];
                    self.slots[5] = first_one;
                }
            },
            Face::Right => match mv.direction {
                Direction::Clockwise => {
                    let first_one = self.slots[2];
                    self.slots[2] = self.slots[3];
                    self.slots[3] = self.slots[9];
                    self.slots[9] = self.slots[8];
                    self.slots[8] = self.slots[7];
                    self.slots[7] = self.slots[6];
                    self.slots[6] = first_one;
                }
                Direction::CounterClockwise => {
                    let first_one = self.slots[2];
                    self.slots[2] = self.slots[6];
                    self.slots[6] = self.slots[7];
                    self.slots[7] = self.slots[8];
                    self.slots[8] = self.slots[9];
                    self.slots[9] = self.slots[3];
                    self.slots[3] = first_one;
                }
            },
            Face::BottomRight => match mv.direction {
                Direction::Clockwise => {
                    let first_one = self.slots[9];
                    self.slots[9] = self.slots[13];
                    self.slots[13] = self.slots[12];
                    self.slots[12] = self.slots[11];
                    self.slots[11] = self.slots[10];
                    self.slots[10] = self.slots[8];
                    self.slots[8] = first_one;
                }
                Direction::CounterClockwise => {
                    let first_one = self.slots[9];
                    self.slots[9] = self.slots[8];
                    self.slots[8] = self.slots[10];
                    self.slots[10] = self.slots[11];
                    self.slots[11] = self.slots[12];
                    self.slots[12] = self.slots[13];
                    self.slots[13] = first_one;
                }
            },
        }
    }

    pub fn get_solved_states(with_opposite_move: bool) -> Vec<Self> {
        let top_area = crate::helpers::permutations(vec![0, 4, 5, 23])
            .into_iter()
            .take(4);
        let top_right_area = crate::helpers::permutations(vec![1, 2, 3, 6])
            .into_iter()
            .take(4);
        let bottom_right_area = crate::helpers::permutations(vec![7, 8, 9, 10])
            .into_iter()
            .take(4);
        let bottom_area = crate::helpers::permutations(vec![11, 12, 13, 14])
            .into_iter()
            .take(4);
        let bottom_left_area = crate::helpers::permutations(vec![15, 16, 17, 18])
            .into_iter()
            .take(4);
        let top_left_area = crate::helpers::permutations(vec![19, 20, 21, 22])
            .into_iter()
            .take(4);

        let mut results = Vec::new();
        for tp in top_area {
            for trp in top_right_area.clone() {
                for brp in bottom_right_area.clone() {
                    for bp in bottom_area.clone() {
                        for blp in bottom_left_area.clone() {
                            for tlp in top_left_area.clone() {
                                let mut puzzle = SinglePuzzle::new_solved(with_opposite_move);
                                puzzle.apply_cycle(tp.clone());
                                puzzle.apply_cycle(trp.clone());
                                puzzle.apply_cycle(brp.clone());
                                puzzle.apply_cycle(bp.clone());
                                puzzle.apply_cycle(blp.clone());
                                puzzle.apply_cycle(tlp.clone());
                                puzzle.deduce_colors();
                                results.push(puzzle);
                            }
                        }
                    }
                }
            }
        }
        results
    }

    pub fn apply_cycle(&mut self, nums: Vec<u8>) {
        let first_one = nums[0];
        for i in 0..nums.len() - 1 {
            self.slots[nums[i] as usize] = nums[i + 1];
        }
        self.slots[*nums.last().unwrap() as usize] = first_one;
    }

    pub fn from_scramble_and_slots(
        scramble: Option<Scramble>,
        slots: Vec<u8>,
        with_opposite_move: bool,
    ) -> Self {
        let mut puzzle = SinglePuzzle {
            scramble,
            slots: slots.clone(),
            colors: vec![0; slots.len()],
            with_opposite_move,
        };
        puzzle.deduce_colors();
        puzzle
    }

    pub fn save_binary_to_file(&self, writer: &mut impl std::io::Write) {
        let scramble = self.get_scramble();
        let moves_len = scramble.moves.len() as u8;
        writer.write_all(&[moves_len]).unwrap();
        for mv in scramble.moves.iter() {
            writer
                .write_all(&[mv.face as u8, mv.direction as u8])
                .unwrap();
        }
        for slot in self.slots.iter() {
            writer.write_all(&[*slot]).unwrap();
        }
        writer.write_all(b"\n").unwrap();
    }
}

impl PuzzleTrait for SinglePuzzle {
    fn get_scramble(&self) -> Scramble {
        self.get_scramble()
    }
    fn calculate_score(&self) -> i64 {
        self.calculate_score()
    }
    fn apply_scramble(&mut self, scramble: Scramble) {
        self.apply_scramble(scramble)
    }
    fn save_binary_to_file(&self, writer: &mut impl std::io::Write) {
        self.save_binary_to_file(writer)
    }
    fn load_binary_from_file(
        reader: &mut impl std::io::Read,
        with_opposite_move: bool,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        read_binary_from_file(reader, with_opposite_move)
    }
}

pub fn read_binary_from_file(
    reader: &mut impl std::io::Read,
    with_opposite_move: bool,
) -> Option<SinglePuzzle> {
    let mut moves_len_buf = [0u8; 1];
    if reader.read_exact(&mut moves_len_buf).is_err() {
        return None;
    }
    let moves_len = moves_len_buf[0] as usize;
    let mut moves = Vec::new();
    let mut mv_buf = [0u8; 2];
    for _ in 0..moves_len {
        if reader.read_exact(&mut mv_buf).is_err() {
            return None;
        }
        let face = match mv_buf[0] {
            0b001 => Face::TopLeft,
            0b010 => Face::Left,
            0b100 => Face::BottomLeft,
            0b011 => Face::TopRight,
            0b101 => Face::Right,
            0b110 => Face::BottomRight,
            _ => return None,
        };
        let direction = match mv_buf[1] {
            0b1 => Direction::Clockwise,
            0b0 => Direction::CounterClockwise,
            _ => return None,
        };
        moves.push(Move::new(face, direction));
    }
    let mut slots = vec![0u8; 24];
    if reader.read_exact(&mut slots).is_err() {
        return None;
    }
    // skip newline if present
    let mut nl = [0u8; 1];
    let _ = reader.read_exact(&mut nl);
    let scramble = if moves.is_empty() {
        None
    } else {
        Some(crate::scramble::Scramble { moves })
    };
    Some(SinglePuzzle::from_scramble_and_slots(
        scramble,
        slots,
        with_opposite_move,
    ))
}
