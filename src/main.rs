enum Face{
    TopLeft,
    Left,
    BottomLeft,
    TopRight,
    Right,
    BottomRight,
}

enum Direction{
    Clockwise,
    CounterClockwise
}

struct Move{
    face: Face,
    direction: Direction,
}

impl Move{
    fn new(face: Face, direction: Direction) -> Self {
        Self { face, direction }
    }
}

struct SinglePuzzle{
    slots: Vec<u8>,
}

impl SinglePuzzle{
    pub fn new_solved() -> Self {
        Self { slots: (0..=24).collect() }
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
}
fn main() {
    println!("Hello, world!");
}
