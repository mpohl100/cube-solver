mod single_puzzle;
mod scramble;
mod reachable;
mod helpers;
mod puzzle_trait;

use clap::Parser;
use single_puzzle::SinglePuzzle;
use scramble::Scramble;
use reachable::ReachableStates;
use helpers::get_random_scramble;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Use opposite move logic
    #[arg(long, default_value_t = false)]
    with_opposite_move: bool,
    /// Batch size
    #[arg(long, default_value_t = 1_000_000)]
    batch_size: usize,
    /// Number of score weakens
    #[arg(long, default_value_t = 3)]
    num_score_weakens: usize,
    /// Number of moves in scramble
    #[arg(long, default_value_t = 50)]
    scramble_moves: usize,
    /// Starting depth
    #[arg(long, default_value_t = 10)]
    start_depth: usize,
    /// Maximum depth
    #[arg(long, default_value_t = 13)]
    max_depth: usize,
}

fn main() {
    let args = Args::parse();
    let scramble = get_random_scramble(args.scramble_moves);
    for i in args.start_depth..args.max_depth {
        let found_solution = find_solution(
            i,
            scramble.clone(),
            args.with_opposite_move,
            args.batch_size,
            args.num_score_weakens,
        );
        if found_solution {
            break;
        }
    }
}

fn find_solution(
    depth: usize,
    scramble: Scramble,
    with_opposite_move: bool,
    batch_size: usize,
    num_score_weakens: usize,
) -> bool {
    let store_directory = "reachable_batches".to_string();
    println!("Depth: {}, Scramble: {:?}", depth, scramble);
    let scrambled_puzzle = SinglePuzzle::new_scrambled(scramble.clone(), with_opposite_move);
    let reachable_states = ReachableStates::new(
        depth,
        scrambled_puzzle,
        batch_size,
        store_directory.clone(),
        with_opposite_move,
        num_score_weakens,
        true,
    );
    reachable_states.print_first_5(with_opposite_move);
    let all_solved_states = SinglePuzzle::get_solved_states(with_opposite_move);
    for (i, solved_state) in all_solved_states.iter().enumerate() {
        println!("Checking solved state {}...", i);
        let solved_store_directory = format!("{}_solved_{}", store_directory, i);
        let reachable_from_solved = ReachableStates::new(
            depth,
            solved_state.clone(),
            batch_size,
            solved_store_directory.clone(),
            with_opposite_move,
            num_score_weakens,
            false,
        );
        let solve = reachable_from_solved.overlaps(&reachable_states, with_opposite_move);
        match solve {
            Some(solution) => {
                println!("Found a solution with {} moves:", solution.moves.len());
                for mv in solution.moves {
                    print!("{}", mv.to_string());
                }
                println!();
                std::fs::remove_dir_all(&store_directory).ok();
                std::fs::remove_dir_all(&solved_store_directory).ok();
                return true;
            }
            None => {
                println!("No solution found for this solved state.");
            }
        }
        std::fs::remove_dir_all(&solved_store_directory).ok();
        break;
    }
    std::fs::remove_dir_all(&store_directory).ok();
    false
}