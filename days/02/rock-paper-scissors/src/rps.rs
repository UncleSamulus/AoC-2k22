//! # Compute score given by a specific play of 'Rock Paper Scissors'
//! 
//! For instance:
//! 
//! ```text
//! A Y
//! B X
//! C Z
//! ```
//! Where A is what the other player plays, Y is what the second player plays on the first round
//! 
//! A corresponds to Rock, B to Paper, C to Scissors.
//! 
//! Similarly Y corresponds to Rock, X to Paper, Z to Scissors.
//! 
//! Scissors beats Paper, Paper beats Rock, Rock beats Scissors
//! 
//! - A Win gives 6 points
//! - A Draw gives 3 points
//! - A Lose gives 0 points
//! 
//! - Playing Rock gives 1 points
//! - Playing Paper gives 2 points
//! - Playing Scissors gives 3 points
//! 

/// Compute the score of a game round
pub fn compute_score(other_player: char, second_player: char) -> u32 {
    let mut score = 0;
    let other_available_moves = "ABC";
    let my_available_moves = "XYZ";
    let other_move_index = other_available_moves.find(other_player).unwrap();
    let moves_points = [1, 2, 3];
    let move_point_index = my_available_moves.find(second_player).unwrap();
    // println!("Move point index: {}", move_point_index);
    let move_point = moves_points[move_point_index];
    score += move_point;
    let my_move_index = my_available_moves.find(second_player).unwrap();
    let has_won = my_move_index == (other_move_index + 1) % 3;
    if has_won {
        score += 6;
    } else if other_move_index == my_move_index {
        score += 3;
    } 
    // else score += 0;
    score
}

/// Compute the score of a game
pub fn compute_game_score(game: &str) -> u32 {
    let mut score = 0;
    for line in game.lines() {
        let parts = line.split_whitespace();
        let other_player = parts.clone().nth(0).unwrap().chars().nth(0).unwrap();
        let second_player = parts.clone().nth(1).unwrap().chars().nth(0).unwrap();
        // print!("{} vs {} ", other_player, second_player);
        score += compute_score(other_player, second_player);
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_score() {
        assert_eq!(compute_score('A', 'Y'), 2+6);
        // TODO add more tests
    }

    #[test]
    fn test_compute_game_score() {
        let game = "A Y";
        assert_eq!(compute_game_score(game), 2+6);
        // TODO add more tests
    }
}