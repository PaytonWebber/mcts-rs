use rand::seq::SliceRandom;

use mcts_rs::{Mcts, State};

#[derive(Debug, Clone)]
pub struct TicTacToe {
    board: [[[u8; 2]; 3]; 3],
    pub current_player: usize,
    pub legal_actions: Vec<(usize, usize)>,
}

const WINNER_MASK: [[(usize, usize); 3]; 8] = [
    // Rows
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    // Columns
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    // Diagonals
    [(0, 0), (1, 1), (2, 2)],
    [(0, 2), (1, 1), (2, 0)],
];

impl State for TicTacToe {
    type Action = (usize, usize);

    fn default_action() -> Self::Action {
        (0, 0)
    }

    fn player_has_won(&self, player: usize) -> bool {
        for line in WINNER_MASK.iter() {
            let [(i0, j0), (i1, j1), (i2, j2)] = *line;
            if self.board[i0][j0][player] == 1
                && self.board[i1][j1][player] == 1
                && self.board[i2][j2][player] == 1
            {
                return true;
            }
        }
        false
    }

    fn is_terminal(&self) -> bool {
        self.player_has_won(0) || self.player_has_won(1) || self.legal_actions.is_empty()
    }
    fn get_legal_actions(&self) -> Vec<(usize, usize)> {
        self.legal_actions.clone()
    }

    fn to_play(&self) -> usize {
        self.current_player
    }

    fn step(&self, action: &(usize, usize)) -> Self {
        let mut new_board = self.board;
        new_board[action.0][action.1][self.current_player] = 1;

        // Create a new vector excluding the taken action
        let mut new_legal_actions = Vec::with_capacity(self.legal_actions.len() - 1);
        for &a in &self.legal_actions {
            if a != *action {
                new_legal_actions.push(a);
            }
        }

        TicTacToe {
            board: new_board,
            current_player: 1 - self.current_player,
            legal_actions: new_legal_actions,
        }
    }

    fn reward(&self, to_play: usize) -> f32 {
        assert!(self.is_terminal());
        if self.player_has_won(to_play) {
            -1.0
        } else if self.player_has_won(1 - to_play) {
            1.0
        } else {
            0.0
        }
    }

    fn render(&self) {
        println!("X: player 0, O: player 1\n");
        for i in 0..3 {
            let mut current_line: Vec<String> = Vec::new();
            for j in 0..3 {
                if self.board[i][j][0] == 1 {
                    current_line.push("X".to_string());
                } else if self.board[i][j][1] == 1 {
                    current_line.push("O".to_string());
                } else {
                    current_line.push(" ".to_string());
                }
            }
            println!(
                "{} | {} | {}",
                current_line[0], current_line[1], current_line[2]
            );
            if i != 2 {
                println!("---------");
            } else {
                println!("\n");
            }
        }
    }
}

impl TicTacToe {
    fn determine_legal_actions(board: &[[[u8; 2]; 3]; 3]) -> Vec<(usize, usize)> {
        let mut legal_actions = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j][0] == 0 && board[i][j][1] == 0 {
                    legal_actions.push((i, j));
                }
            }
        }
        legal_actions
    }
}

impl Default for TicTacToe {
    fn default() -> Self {
        let board = [[[0; 2]; 3]; 3];
        TicTacToe {
            board,
            current_player: 0,
            legal_actions: TicTacToe::determine_legal_actions(&board),
        }
    }
}

fn main() {
    let mut game = TicTacToe::default();

    // Randomly select the first action
    let action = game.legal_actions.choose(&mut rand::thread_rng()).unwrap();
    game = game.step(action);

    while !game.is_terminal() {
        let mut mcts = Mcts::new(game.clone(), 5.0);
        let action = mcts.search(1000);
        game = game.step(&action);
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
