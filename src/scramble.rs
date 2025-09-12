use crate::single_puzzle::Move;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Scramble {
    pub moves: Vec<Move>,
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
