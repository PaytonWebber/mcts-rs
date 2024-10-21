mod state;
mod tic_tac_toe;

use state::State;
use tic_tac_toe::TicTacToe;

fn main() {
    let mut ttt = TicTacToe::new();
    ttt.render();
    ttt = ttt.step((0, 0));
    ttt.render();
    ttt = ttt.step((0, 1));
    ttt.render();
    ttt = ttt.step((1, 0));
    ttt.render();
    ttt = ttt.step((0, 2));
    ttt.render();
    println!("{}", ttt.is_terminal());
    ttt = ttt.step((2, 0));
    ttt.render();
    println!("Game Over: {}", ttt.is_terminal());
    println!("Winner: {}", ttt.reward(1 - ttt.current_player));
}
