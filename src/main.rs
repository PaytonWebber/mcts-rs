mod mcts;
mod state;
mod tic_tac_toe;

use mcts::Mcts;
use state::State;
use tic_tac_toe::TicTacToe;

fn main() {
    let mut game = TicTacToe::new();

    while !game.is_terminal() {
        let mut mcts = Mcts::new(game.clone(), 1.0);
        let action = mcts.search(10000);
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
