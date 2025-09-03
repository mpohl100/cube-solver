use rand::seq::SliceRandom;
use rand::thread_rng;

// make face be stored with 3 bits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Face {
    TopLeft = 0b001,
    Left = 0b010,
    BottomLeft = 0b100,
    TopRight = 0b011,
    Right = 0b101,
    BottomRight = 0b110,
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

// make direction stored as 1 bit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Clockwise = 0b1,
    CounterClockwise = 0b0,
}

fn to_string_direction(direction: Direction) -> &'static str {
    match direction {
        Direction::Clockwise => "CW",
        Direction::CounterClockwise => "CCW",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Move {
    face: Face,
    direction: Direction,
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
            to_string_face(self.face),
            to_string_direction(self.direction)
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Scramble {
    moves: Vec<Move>,
}

impl Scramble {
    pub fn invert(&self) -> Self {
        let mut inverted_moves = self.moves.clone();
        inverted_moves.reverse();
        for mv in &mut inverted_moves {
            *mv = mv.get_inverted_move();
        }
        Scramble {
            moves: inverted_moves,
        }
    }

    pub fn concat(&self, other: Scramble) -> Self {
        let mut new_moves = self.moves.clone();
        new_moves.extend(other.moves);
        Scramble { moves: new_moves }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SinglePuzzle {
    with_opposite_move: bool,
    scramble: Option<Scramble>,
    slots: Vec<u8>,
    colors: Vec<u8>,
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

    fn new_solved(with_opposite_move: bool) -> Self {
        Self {
            scramble: None,
            slots: (0..=23).collect(),
            colors: (0..=23).map(get_color).collect(),
            with_opposite_move,
        }
    }

    fn new_scrambled(scramble: Scramble, with_opposite_move: bool) -> Self {
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

    fn deduce_colors(&mut self) {
        self.colors = self.slots.iter().map(|&num| get_color(num)).collect();
    }

    fn apply_scramble(&mut self, scramble: Scramble) {
        self.scramble = Some(scramble.clone());
        for mv in scramble.moves {
            self.apply_move(mv, true);
        }
        self.colors = self.slots.iter().map(|&num| get_color(num)).collect();
    }

    fn apply_move(&mut self, mv: Move, apply_opposite: bool) {
        if self.with_opposite_move && apply_opposite {
            let opposite_mv = mv.get_opposite_move();
            self.apply_move(opposite_mv, false);
        }
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
            }
            Face::TopRight => {
                // Apply top-right move
                match mv.direction {
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
            }
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
            }
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
                }
            }
        }
    }

    pub fn get_solved_states(with_opposite_move: bool) -> Vec<Self> {
        let top_area = permutations([0, 4, 5, 23].to_vec()).into_iter().take(4);
        let top_right_area = permutations([1, 2, 3, 6].to_vec()).into_iter().take(4);
        let bottom_right_area = permutations([7, 8, 9, 10].to_vec()).into_iter().take(4);
        let bottom_area = permutations([11, 12, 13, 14].to_vec()).into_iter().take(4);
        let bottom_left_area = permutations([15, 16, 17, 18].to_vec()).into_iter().take(4);
        let top_left_area = permutations([19, 20, 21, 22].to_vec()).into_iter().take(4);

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

    fn apply_cycle(&mut self, nums: Vec<u8>) {
        let first_one = nums[0];
        for i in 0..nums.len() - 1 {
            self.slots[nums[i] as usize] = nums[i + 1];
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

fn get_random_scramble(num_moves: usize) -> Scramble {
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

fn get_color(num: u8) -> u8 {
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

struct Batch {
    batch_size: usize,
    states: Vec<SinglePuzzle>,
}

impl Batch {
    fn new(batch_size: usize) -> Self {
        Batch {
            batch_size,
            states: Vec::with_capacity(batch_size),
        }
    }

    fn is_full(&self) -> bool {
        self.states.len() >= self.batch_size
    }

    fn add_state(&mut self, state: SinglePuzzle) {
        self.states.push(state);
    }

    fn sort_states(&mut self) {
        self.states.sort();
    }
}

struct ReachableStates {
    _depth: usize,
    batch_size: usize,
    batches: Vec<Batch>,
}

impl ReachableStates {
    fn new(depth: usize, puzzle: SinglePuzzle, batch_size: usize) -> Self {
        let mut reachable_states = Self {
            _depth: depth,
            batch_size,
            batches: Vec::new(),
        };
        reachable_states.compute_reachable(
            depth,
            &get_all_moves(),
            Scramble { moves: Vec::new() },
            puzzle,
        );
        reachable_states
    }

    fn print_first_5(&self) {
        let mut count = 0;
        for batch in &self.batches {
            for state in &batch.states {
                println!("{:?}", state);
                count += 1;
                if count >= 5 {
                    return;
                }
            }
        }
    }

    fn compute_reachable(
        &mut self,
        depth: usize,
        all_moves: &Vec<Move>,
        scramble: Scramble,
        puzzle: SinglePuzzle,
    ) {
        for mv in all_moves.iter() {
            if depth == 0 {
                let mut cloned_puzzle = puzzle.clone();
                let mut new_scramble = scramble.clone();
                new_scramble.moves.push(mv.clone());
                cloned_puzzle.apply_scramble(new_scramble.clone());

                // Add to current batch, create/sort new batch if needed
                if self.batches.is_empty() || self.batches.last().unwrap().is_full() {
                    self.batches.push(Batch::new(self.batch_size));
                }
                let batch = self.batches.last_mut().unwrap();
                batch.add_state(cloned_puzzle);
                if batch.is_full() {
                    batch.sort_states();
                }
            } else {
                let mut new_scramble = scramble.clone();
                new_scramble.moves.push(mv.clone());
                self.compute_reachable(depth - 1, all_moves, new_scramble, puzzle.clone());
            }
        }
    }

    fn overlaps(&self, other: &Self) -> Option<Scramble> {
        // Efficient batch-wise search since batches are sorted
        let mut i = 0;
        let mut j = 0;
        // do two nested for loops over all batches
        for i_batch in 0..self.batches.len() {
            for j_batch in 0..other.batches.len() {
                let batch_a = &self.batches[i_batch];
                let batch_b = &other.batches[j_batch];
                i = 0;
                j = 0;
                while i < batch_a.states.len() && j < batch_b.states.len() {
                    match batch_a.states[i].cmp(&batch_b.states[j]) {
                        std::cmp::Ordering::Equal => {
                            let first_part_of_scramble = batch_a.states[i].get_scramble();
                            let second_part_of_scramble = batch_b.states[j].get_scramble().invert();
                            return Some(first_part_of_scramble.concat(second_part_of_scramble));
                        }
                        std::cmp::Ordering::Less => i += 1,
                        std::cmp::Ordering::Greater => j += 1,
                    }
                }
            }
        }
        None
    }
}

#[allow(unreachable_code)]
fn main() {
    // print the size of Direction, Face and Move in bits
    println!(
        "Size of Direction: {}",
        std::mem::size_of::<Direction>() * 8
    );
    println!("Size of Face: {}", std::mem::size_of::<Face>() * 8);
    println!("Size of Move: {}", std::mem::size_of::<Move>() * 8);
    println!("Size of usize: {}", std::mem::size_of::<usize>() * 8);
    println!("Size of u8: {}", std::mem::size_of::<u8>() * 8);
    let with_opposite_move = true;
    let batch_size = 1_000_000;
    let scramble = get_random_scramble(50);
    println!("Scramble: {:?}", scramble);
    let scrambled_puzzle = SinglePuzzle::new_scrambled(scramble.clone(), with_opposite_move);
    let depth = 7;
    let reachable_states = ReachableStates::new(depth, scrambled_puzzle, batch_size);
    reachable_states.print_first_5();
    let all_solved_states = SinglePuzzle::get_solved_states(with_opposite_move);
    for (i, solved_state) in all_solved_states.iter().enumerate() {
        println!("Checking solved state {}...", i);
        let reachable_from_solved = ReachableStates::new(depth, solved_state.clone(), batch_size);
        let solve = reachable_from_solved.overlaps(&reachable_states);
        match solve {
            Some(solution) => {
                println!("Found a solution with {} moves:", solution.moves.len());
                for mv in solution.moves {
                    print!("{}", mv.to_string());
                }
                println!();
                return;
            }
            None => {
                println!("No solution found for this solved state.");
            }
        }
        break;
    }
}
