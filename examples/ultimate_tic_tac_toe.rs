use mcts_rs::{Mcts, State};

#[derive(Debug, Clone)]
struct UltimateTicTacToe {
    board_x: [u16; 9],
    board_o: [u16; 9],
    macro_board_x: u16,
    macro_board_o: u16,
    current_player: usize,
    legal_actions: Vec<(u8, u8, u8)>,
}

const WIN_PATTERNS: [u16; 8] = [
    // rows
    0b111000000,
    0b000111000,
    0b000000111,
    // cols
    0b100100100,
    0b010010010,
    0b001001001,
    // diags
    0b100010001,
    0b001010100,
];

impl State for UltimateTicTacToe {
    type Action = (u8, u8, u8); // Mini-board, Row, Col

    fn default_action() -> Self::Action {
        (0, 0, 0)
    }

    fn player_has_won(&self, player: usize) -> bool {
        let board = match player {
            0_usize => self.macro_board_x,
            _ => self.macro_board_o,
        };
        for &pattern in WIN_PATTERNS.iter() {
            if (board & pattern) == pattern {
                return true;
            }
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
        let mut board_x_clone = self.board_x.clone();
        let mut board_o_clone = self.board_o.clone();
        if self.current_player == 0 {
            set_bit(
                &mut board_x_clone,
                action.0 as usize,
                action.1 * 3 + action.2,
            );
        } else {
            set_bit(
                &mut board_o_clone,
                action.0 as usize,
                action.1 * 3 + action.2,
            );
        }
        let mut macro_board_clone_x = self.macro_board_x.clone();
        let mut macro_board_clone_o = self.macro_board_o.clone();
        update_macro_board(
            board_x_clone,
            board_o_clone,
            &mut macro_board_clone_x,
            &mut macro_board_clone_o,
            action.0 as usize,
        );
        let current_player = 1 - self.current_player;
        let legal_actions = determine_legal_actions(
            board_x_clone,
            board_o_clone,
            macro_board_clone_x,
            macro_board_clone_o,
            (action.1 * 3 + action.2) as usize,
        );
        UltimateTicTacToe {
            board_x: board_x_clone,
            board_o: board_o_clone,
            macro_board_x: macro_board_clone_x,
            macro_board_o: macro_board_clone_o,
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
                let mut row_segments: Vec<String> = Vec::with_capacity(3);
                for big_col in 0..3 {
                    let mini_board_index = big_row * 3 + big_col;
                    let mut segment = String::new();
                    for sub_col in 0..3 {
                        let pos = sub_row * 3 + sub_col;
                        let mask = 1 << pos;

                        if (self.board_x[mini_board_index] & mask) != 0 {
                            segment.push('X');
                        } else if (self.board_o[mini_board_index] & mask) != 0 {
                            segment.push('O');
                        } else {
                            segment.push(' ');
                        }
                        if sub_col < 2 {
                            segment.push('|');
                        }
                    }
                    row_segments.push(segment);
                }
                println!(
                    " {} || {} || {}",
                    row_segments[0], row_segments[1], row_segments[2]
                );
            }
            if big_row < 2 {
                println!("=======||=======||=======");
            }
        }
        println!();
    }
}

fn update_macro_board(
    board_x: [u16; 9],
    board_o: [u16; 9],
    macro_board_x: &mut u16,
    macro_board_o: &mut u16,
    board_to_check: usize,
) {
    let mini_board_x = board_x[board_to_check];
    let mini_board_o = board_o[board_to_check];

    for &pattern in WIN_PATTERNS.iter() {
        if (mini_board_x & pattern) == pattern {
            *macro_board_x |= 1 << board_to_check;
            return;
        } else if (mini_board_o & pattern) == pattern {
            *macro_board_o |= 1 << board_to_check;
            return;
        }
    }
}

fn determine_legal_actions(
    board_x: [u16; 9],
    board_o: [u16; 9],
    macro_board_x: u16,
    macro_board_o: u16,
    next_board: usize,
) -> Vec<(u8, u8, u8)> {
    let mut actions: Vec<(u8, u8, u8)> = Vec::with_capacity(81);
    let pos = 1 << next_board;
    if ((macro_board_x & pos) == 0 && (macro_board_o & pos) == 0)
        && ((board_x[next_board] | board_o[next_board]) != 0x1FF)
    {
        for pos in 0..9 {
            let mask = 1 << pos;
            if (board_x[next_board] & mask) == 0 && (board_o[next_board] & mask) == 0 {
                actions.push((next_board as u8, pos / 3, pos % 3));
            }
        }
    } else {
        for i in 0..9 {
            for pos in 0..9 {
                let mask = 1 << pos;
                if (board_x[i] & mask) == 0 && (board_o[i] & mask) == 0 {
                    actions.push((i as u8, pos / 3, pos % 3));
                }
            }
        }
    }
    actions
}

fn set_bit(board: &mut [u16; 9], mini_board: usize, pos: u8) {
    board[mini_board] |= 1 << pos;
}

impl UltimateTicTacToe {
    pub fn new() -> UltimateTicTacToe {
        let mut legal_actions: Vec<(u8, u8, u8)> = Vec::with_capacity(81);
        for i in 0..9 {
            for j in 0..3 {
                for k in 0..3 {
                    legal_actions.push((i, j, k));
                }
            }
        }
        UltimateTicTacToe {
            board_x: [0; 9],
            board_o: [0; 9],
            macro_board_x: 0,
            macro_board_o: 0,
            current_player: 0,
            legal_actions,
        }
    }
}

fn main() {
    let mut game = UltimateTicTacToe::new();
    while !game.is_terminal() {
        let mut mcts = Mcts::new(game.clone(), 1.4142356237);
        let action = mcts.search(1500);
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
