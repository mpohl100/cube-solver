use crate::scramble::Scramble;

pub trait PuzzleTrait: Send + Sync + std::fmt::Debug + Clone + Eq + Ord {
    fn get_scramble(&self) -> Scramble;
    fn calculate_score(&self) -> i64;
    fn apply_scramble(&mut self, scramble: Scramble);
    fn save_binary_to_file(&self, writer: &mut impl std::io::Write);
    fn load_binary_from_file(reader: &mut impl std::io::Read, with_opposite_move: bool) -> Option<Self>
    where
        Self: Sized;
    //fn from_scramble_and_slots(scramble: Option<Scramble>, slots: Vec<u8>, with_opposite_move: bool) -> Self;
}
