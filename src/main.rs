
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Face{
    TopLeft,
    Left,
    BottomLeft,
    TopRight,
    Right,
    BottomRight,
}

fn to_string_face(face: Face) -> &'static str {
    match face {
        Face::TopLeft => "TL",
        Face::Left => "L",
        Face::BottomLeft => "BL",
        Face::TopRight => "TR",
        Face::Right => "R",
        Face::BottomRight => "BR",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction{
    Clockwise,
    CounterClockwise
}

fn to_string_direction(direction: Direction) -> &'static str {
    match direction {
        Direction::Clockwise => "CW",
        Direction::CounterClockwise => "CCW",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Move{
    face: Face,
    direction: Direction,
}



impl Move{
    pub fn new(face: Face, direction: Direction) -> Self {
        Self { face, direction }
    }

    pub fn get_inverted_move(&self) -> Self{
        match self.direction {
            Direction::Clockwise => Move::new(self.face, Direction::CounterClockwise),
            Direction::CounterClockwise => Move::new(self.face, Direction::Clockwise),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {};", to_string_face(self.face), to_string_direction(self.direction))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SinglePuzzle{
    slots: Vec<u8>,
}

impl SinglePuzzle{
    fn new_solved() -> Self {
        Self { slots: (0..=24).collect() }
    }

    fn new_scramled(moves: Vec<Move>) -> Self {
        let mut puzzle = SinglePuzzle{
            slots: vec![
                0,1,2,3,
                4,5,6,7,
                8,9,10,11,
                12,13,14,15,
                16,17,18,19,
                20,21,22,23
            ],
        };
        for mv in moves {
            puzzle.apply_move(mv);
        }
        puzzle
    }

    fn apply_move(&mut self, mv: Move) {
        // Apply the move to the puzzle state
        match mv.face {
            Face::TopLeft => {
                // Apply top-left move
                match mv.direction {
                    Direction::Clockwise => {
                        // Rotate clockwise
                        let first_one = self.slots[5];
                        self.slots[5] = self.slots[23];
                        self.slots[23] = self.slots[22];
                        self.slots[22] = self.slots[20];
                        self.slots[20] = self.slots[21];
                        self.slots[21] = self.slots[4];
                        self.slots[5] = first_one;
                    }
                    Direction::CounterClockwise => {
                        // Rotate counter-clockwise
                        let first_one = self.slots[5];
                        self.slots[5] = self.slots[4];
                        self.slots[4] = self.slots[21];
                        self.slots[21] = self.slots[20];
                        self.slots[20] = self.slots[22];
                        self.slots[22] = self.slots[23];
                        self.slots[23] = first_one;
                    }
                }
            }
            Face::Left => {
                // Apply left move
                match mv.direction {
                    Direction::Clockwise => {
                        let first_one = self.slots[20];
                        self.slots[20] = self.slots[19];
                        self.slots[19] = self.slots[18];
                        self.slots[18] = self.slots[16];
                        self.slots[16] = self.slots[17];
                        self.slots[17] = self.slots[21];
                        self.slots[21] = first_one;                    }
                    Direction::CounterClockwise => {
                        let first_one = self.slots[20];
                        self.slots[20] = self.slots[21];
                        self.slots[21] = self.slots[17];
                        self.slots[17] = self.slots[16];
                        self.slots[16] = self.slots[18];
                        self.slots[18] = self.slots[19];
                        self.slots[19] = first_one;
                    }
                }
            }
            Face::BottomLeft => {
                // Apply bottom-left move
                match mv.direction {
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
                }
            },
            Face::TopRight => {
                // Apply top-right move
                match mv.direction  {
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
                }
            },
            Face::Right => {
                // Apply right move
                match mv.direction {
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
                }
            },
            Face::BottomRight => {
                // Apply bottom-right move
                match mv.direction {
                    Direction::Clockwise => {
                        let first_one = self.slots[9];
                        self.slots[9] = self.slots[13];
                        self.slots[13] = self.slots[12];
                        self.slots[12] = self.slots[11];
                        self.slots[11] = self.slots[10];
                        self.slots[10] = self.slots[8];
                        self.slots[8] = first_one;
                    },
                    Direction::CounterClockwise => {
                        let first_one = self.slots[9];
                        self.slots[9] = self.slots[8];
                        self.slots[8] = self.slots[10];
                        self.slots[10] = self.slots[11];
                        self.slots[11] = self.slots[12];
                        self.slots[12] = self.slots[13];
                        self.slots[13] = first_one;
                    }
                }
            }
        }
    }

    pub fn get_solved_states() -> Vec<Self> {
        let top_area = permutations([0,4,5,23].to_vec()).into_iter().take(4);
        let top_right_area = permutations([1,2,3,6].to_vec()).into_iter().take(4);
        let bottom_right_area = permutations([7,8,9,10].to_vec()).into_iter().take(4);
        let bottom_area = permutations([11,12,13,14].to_vec()).into_iter().take(4);
        let bottom_left_area = permutations([15,16,17,18].to_vec()).into_iter().take(4);
        let top_left_area = permutations([19,20,21,22].to_vec()).into_iter().take(4);
        
        let mut results = Vec::new();
        for tp in top_area {
            for trp in top_right_area.clone() {
                for brp in bottom_right_area.clone() {
                    for bp in bottom_area.clone() {
                        for blp in bottom_left_area.clone() {
                            for tlp in top_left_area.clone() {
                                let mut puzzle = SinglePuzzle::new_solved();
                                puzzle.apply_cycle(tp.clone());
                                puzzle.apply_cycle(trp.clone());
                                puzzle.apply_cycle(brp.clone());
                                puzzle.apply_cycle(bp.clone());
                                puzzle.apply_cycle(blp.clone());
                                puzzle.apply_cycle(tlp.clone());
                                results.push(puzzle);
                            }
                        }
                    }
                }
            }
        }
        results
    }

    fn apply_cycle(&mut self, nums: Vec<u8>){
        let first_one = nums[0];
        for i in 0..nums.len()-1{
            self.slots[nums[i] as usize] = nums[i+1];
        }
        self.slots[*nums.last().unwrap() as usize] = first_one;
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

fn get_all_moves() -> Vec<Move> {
    let all_faces = [Face::TopLeft, Face::Left, Face::BottomLeft, Face::TopRight, Face::Right, Face::BottomRight];
    let all_directions = [Direction::Clockwise, Direction::CounterClockwise];
    let mut all_moves = Vec::new();
    for face in all_faces.iter() {
        for direction in all_directions.iter() {
            all_moves.push(Move::new(*face, *direction));
        }
    }
    all_moves
}

fn get_random_scramble(num_moves: usize) -> Vec<Move> {
    let all_faces = [Face::TopLeft, Face::Left, Face::BottomLeft, Face::TopRight, Face::Right, Face::BottomRight];
    let all_directions = [Direction::Clockwise, Direction::CounterClockwise];
    let mut rng = thread_rng();
    let mut scramble = Vec::new();
    for _ in 0..num_moves {
        let face = all_faces.choose(&mut rng).unwrap();
        let direction = all_directions.choose(&mut rng).unwrap();
        scramble.push(Move::new(*face, *direction));
    }
    scramble
}

fn main() {
    let scramble = get_random_scramble(50);
    let scrambled_puzzle = SinglePuzzle::new_scramled(scramble.clone());
    let all_solved_states = SinglePuzzle::get_solved_states();
    let depth = 4;
    let all_moves = get_all_moves();

}
