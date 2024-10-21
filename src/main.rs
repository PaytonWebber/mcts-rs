mod mcts;
mod state;
mod tic_tac_toe;

use mcts::Mcts;
use state::State;
use tic_tac_toe::TicTacToe;

use rand::seq::SliceRandom;

fn main() {
    let mut game = TicTacToe::new();

    // Randomly select the first action
    let action = game.legal_actions.choose(&mut rand::thread_rng()).unwrap();
    game = game.step(*action);

    while !game.is_terminal() {
        let mut mcts = Mcts::new(game.clone(), 1.4);
        let action = mcts.search(1000);
        game = game.step(action);
        game.render();
    }

    println!("Game over!");
    if game.player_has_won(0) {
        println!("Player 0 wins!");
    } else if game.player_has_won(1) {
        println!("Player 1 wins!");
    } else {
        println!("Draw!");
    }
}
