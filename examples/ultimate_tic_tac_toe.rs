use mcts_rs::{Mcts, State};

#[derive(Debug, Clone)]
struct UltimateTicTacToe {
    board: [u8; 81],
    macro_board: [u8; 9],
    current_player: usize,
    legal_actions: Vec<(usize, usize, usize)>,
}

const P1: u8 = 1;
const P2: u8 = 2;
const TIE: u8 = 3;

impl State for UltimateTicTacToe {
    type Action = (usize, usize, usize); // Mini-board, Row, Col
    fn default_action() -> Self::Action {
        (0, 0, 0)
    }

    fn player_has_won(&self, player: usize) -> bool {
        let board = &self.macro_board;
        let player_val = get_player_val(player);
        for i in 0..3 {
            // Row check
            if board[i * 3] == player_val
                && board[i * 3 + 1] == player_val
                && board[i * 3 + 2] == player_val
            {
                return true;
            }

            // Col check
            if board[i] == player_val && board[i + 3] == player_val && board[i + 6] == player_val {
                return true;
            }
        }
        if (board[0] == player_val && board[4] == player_val && board[8] == player_val)
            || (board[2] == player_val && board[4] == player_val && board[6] == player_val)
        {
            return true;
        }
        false
    }

    fn is_terminal(&self) -> bool {
        self.legal_actions.len() == 0 || self.player_has_won(0) || self.player_has_won(1)
    }

    fn get_legal_actions(&self) -> Vec<Self::Action> {
        self.legal_actions.clone()
    }

    fn to_play(&self) -> usize {
        self.current_player
    }

    fn step(&self, action: Self::Action) -> Self {
        let mut board_clone = self.board.clone();
        let current_player = 1 - self.current_player;
        let player_val = get_player_val(current_player);
        board_clone[action.0 * 9 + action.1 * 3 + action.2] = player_val;

        let mut macro_clone = self.macro_board.clone();
        update_macro_board(&mut macro_clone, &board_clone, action.0);

        let legal_actions =
            determine_legal_actions(action.1 * 3 + action.2, &macro_clone, &board_clone);

        UltimateTicTacToe {
            board: board_clone,
            macro_board: macro_clone,
            current_player,
            legal_actions,
        }
    }

    fn reward(&self, to_play: usize) -> f32 {
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
            for sub_row in 0..3 {
                let mut current_lines: Vec<String> = Vec::new();

                for big_col in 0..3 {
                    let mut sub_line = String::new();

                    for sub_col in 0..3 {
                        // Calculate the index in the 1D array
                        let index = (big_row * 3 + big_col) * 9 + sub_row * 3 + sub_col;

                        match self.board[index] {
                            1 => sub_line.push('X'),
                            2 => sub_line.push('O'),
                            _ => sub_line.push(' '),
                        }

                        if sub_col < 2 {
                            sub_line.push('|');
                        }
                    }

                    current_lines.push(sub_line);
                }

                println!(
                    " {} || {} || {}",
                    current_lines[0], current_lines[1], current_lines[2]
                );
            }

            if big_row < 2 {
                println!("=======||=======||========");
            }
        }

        println!();
    }
}

fn get_player_val(player: usize) -> u8 {
    if player == 0 {
        return P1;
    } else if player == 1 {
        return P2;
    }
    u8::MAX
}

fn update_macro_board(macro_board: &mut [u8; 9], full_board: &[u8; 81], macro_idx: usize) {
    let offset = macro_idx * 9;
    let sub_board = &full_board[offset..offset + 9];
    let mut winner = 0;
    for i in 0..3 {
        if sub_board[i * 3] != 0
            && sub_board[i * 3] == sub_board[i * 3 + 1]
            && sub_board[i * 3 + 1] == sub_board[i * 3 + 2]
        {
            winner = sub_board[i * 3];
            break;
        }
        if sub_board[i] != 0
            && sub_board[i] == sub_board[i + 3]
            && sub_board[i + 3] == sub_board[i + 6]
        {
            winner = sub_board[i];
            break;
        }
    }
    if winner == 0 {
        if sub_board[0] != 0 && sub_board[0] == sub_board[4] && sub_board[4] == sub_board[8] {
            winner = sub_board[0];
        } else if sub_board[2] != 0 && sub_board[2] == sub_board[4] && sub_board[4] == sub_board[6]
        {
            winner = sub_board[2];
        }
    }
    if winner == P1 || winner == P2 {
        macro_board[macro_idx] = winner;
        return;
    }
    if sub_board.iter().all(|&cell| cell != 0) {
        macro_board[macro_idx] = TIE;
    }
}

fn determine_legal_actions(
    next_sub_board: usize,
    macro_board: &[u8; 9],
    full_board: &[u8; 81],
) -> Vec<(usize, usize, usize)> {
    let mut actions = Vec::new();
    if macro_board[next_sub_board] == 0 {
        let start_index = next_sub_board * 9;
        for i in 0..9 {
            if full_board[start_index + i] == 0 {
                actions.push((next_sub_board, i / 3, i % 3));
            }
        }
    } else {
        // We can play on any empty cell in the entire board
        for i in 0..81 {
            if full_board[i] == 0 {
                let board_idx = i / 9;
                let row = (i % 9) / 3;
                let col = (i % 9) % 3;
                actions.push((board_idx, row, col));
            }
        }
    }
    actions
}

impl UltimateTicTacToe {
    pub fn new() -> UltimateTicTacToe {
        let mut legal_actions: Vec<(usize, usize, usize)> = Vec::with_capacity(81);
        for i in 0..9 {
            for j in 0..3 {
                for k in 0..3 {
                    legal_actions.push((i, j, k));
                }
            }
        }
        UltimateTicTacToe {
            board: [0; 81],
            macro_board: [0; 9],
            current_player: 0,
            legal_actions,
        }
    }
}

fn main() {
    let mut game = UltimateTicTacToe::new();
    while !game.is_terminal() {
        let mut mcts = Mcts::new(game.clone(), 1.4142356237);
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
