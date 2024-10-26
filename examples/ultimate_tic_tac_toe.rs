use mcts_rs::{Mcts, State};
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
struct UltimateTicTacToe {
    board: [[[[u8; 2]; 3]; 3]; 9],
    macro_board: [[[u8; 2]; 3]; 3],
    player: u8,
    legal_actions: Vec<(usize, usize, usize)>,
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

impl State for UltimateTicTacToe {
    type Action = (usize, usize, usize);

    fn default_action() -> Self::Action {
        (0, 0, 0)
    }

    fn step(&self, action: (usize, usize, usize)) -> Self {
        let mut new_board = self.board.clone();
        new_board[action.0][action.1][action.2][self.player as usize] = 1;
        let legal_actions =
            if UltimateTicTacToe::mini_board_full(&new_board, action.1 * 3 + action.2) {
                UltimateTicTacToe::find_legal_actions(&new_board, -1)
            } else {
                UltimateTicTacToe::find_legal_actions(&new_board, (action.1 * 3 + action.2) as i8)
            };
        let mut new_macro_board = self.macro_board.clone();
        let new_player = 1 - self.player;

        for board_row in 0..3 {
            for board_col in 0..3 {
                let mut local_win = false;

                for line in WINNER_MASK.iter() {
                    let [(i0, j0), (i1, j1), (i2, j2)] = *line;
                    if new_board[board_row * 3 + board_col][i0][j0][new_player as usize] == 1
                        && new_board[board_row * 3 + board_col][i1][j1][new_player as usize] == 1
                        && new_board[board_row * 3 + board_col][i2][j2][new_player as usize] == 1
                    {
                        local_win = true;
                        break;
                    }
                }

                // If a player has won in a local board, mark it in a macro board
                if local_win {
                    new_macro_board[board_row][board_col][new_player as usize] = 1;
                }
            }
        }

        UltimateTicTacToe {
            board: new_board,
            macro_board: new_macro_board,
            player: new_player,
            legal_actions,
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

        for big_row in 0..3 {
            // Print each line within the three rows of sub-boards
            for sub_row in 0..3 {
                let mut current_lines: Vec<String> = Vec::new();

                for big_col in 0..3 {
                    let mut sub_line = String::new();

                    for sub_col in 0..3 {
                        if self.board[big_row * 3 + big_col][sub_row][sub_col][0] == 1 {
                            sub_line.push('X');
                        } else if self.board[big_row * 3 + big_col][sub_row][sub_col][1] == 1 {
                            sub_line.push('O');
                        } else {
                            sub_line.push(' ');
                        }

                        // Add space between cells within a sub-board
                        if sub_col < 2 {
                            sub_line.push('|');
                        }
                    }

                    current_lines.push(sub_line);
                }

                // Join the lines of the three sub-boards side by side
                println!(
                    " {} || {} || {}",
                    current_lines[0], current_lines[1], current_lines[2]
                );
            }

            // Divider between the large 3x3 sub-boards
            if big_row < 2 {
                println!("=======||=======||========");
            }
        }

        println!();
    }

    fn to_play(&self) -> usize {
        self.player as usize
    }

    fn is_terminal(&self) -> bool {
        self.player_has_won(0) || self.player_has_won(1) || self.legal_actions.is_empty()
    }

    // TODO: change this so that I only need to call it once
    fn player_has_won(&self, player: usize) -> bool {
        for line in WINNER_MASK.iter() {
            let [(i0, j0), (i1, j1), (i2, j2)] = *line;
            if self.macro_board[i0][j0][player] == 1
                && self.macro_board[i1][j1][player] == 1
                && self.macro_board[i2][j2][player] == 1
            {
                return true;
            }
        }
        false
    }

    fn get_legal_actions(&self) -> Vec<(usize, usize, usize)> {
        self.legal_actions.clone()
    }
}

impl UltimateTicTacToe {
    fn new() -> Self {
        let mut legal_actions = Vec::with_capacity(81);

        for i in 0..9 {
            for j in 0..3 {
                for k in 0..3 {
                    legal_actions.push((i, j, k));
                }
            }
        }
        UltimateTicTacToe {
            board: [[[[0; 2]; 3]; 3]; 9],
            macro_board: [[[0; 2]; 3]; 3],
            player: 0,
            legal_actions,
        }
    }

    fn find_legal_actions(
        board: &[[[[u8; 2]; 3]; 3]; 9],
        mini_idx: i8,
    ) -> Vec<(usize, usize, usize)> {
        let mut legal_actions = Vec::new();

        // Check all mini boards
        if mini_idx == -1 {
            for i in 0..9 {
                for j in 0..3 {
                    for k in 0..3 {
                        if board[i][j][k][0] == 0 && board[i][j][k][1] == 0 {
                            legal_actions.push((i, j, k));
                        }
                    }
                }
            }
        } else {
            for i in 0..3 {
                for j in 0..3 {
                    if board[mini_idx as usize][i][j][0] == 0
                        && board[mini_idx as usize][i][j][1] == 0
                    {
                        legal_actions.push((mini_idx as usize, i, j));
                    }
                }
            }
        }
        legal_actions
    }

    fn mini_board_full(board: &[[[[u8; 2]; 3]; 3]; 9], board_idx: usize) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if board[board_idx][i][j][0] == 0 && board[board_idx][i][j][0] == 0 {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let mut game = UltimateTicTacToe::new();
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
