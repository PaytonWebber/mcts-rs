use mcts_rs::{Mcts, State};

#[derive(Debug, Clone)]
pub struct TicTacToe {
    board_x: u16,
    board_o: u16,
    // Current player: 0 => X, 1 => O
    current_player: usize,
    // Row, col
    legal_actions: Vec<(u8, u8)>,
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

impl State for TicTacToe {
    type Action = (u8, u8);

    fn default_action() -> Self::Action {
        (0, 0)
    }

    fn player_has_won(&self, player: usize) -> bool {
        let board = match player {
            0_usize => self.board_x,
            _ => self.board_o,
        };
        for &pattern in WIN_PATTERNS.iter() {
            if (board & pattern) == pattern {
                return true;
            }
        }
        false
    }

    fn is_terminal(&self) -> bool {
        self.player_has_won(0)
            || self.player_has_won(1)
            || board_is_filled(self.board_x, self.board_o)
    }

    fn get_legal_actions(&self) -> Vec<Self::Action> {
        self.legal_actions.clone()
    }

    fn to_play(&self) -> usize {
        self.current_player
    }

    fn step(&self, action: Self::Action) -> Self {
        let mut new_board_x = self.board_x;
        let mut new_board_o = self.board_o;
        if self.current_player == 0 {
            set_bit(&mut new_board_x, action.0 * 3 + action.1);
        } else {
            set_bit(&mut new_board_o, action.0 * 3 + action.1);
        }
        let current_player = 1 - self.current_player;
        let legal_actions = determine_legal_actions(new_board_x, new_board_o);
        TicTacToe {
            board_x: new_board_x,
            board_o: new_board_o,
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
        for i in (0..3).rev() {
            let mut current_line: Vec<String> = Vec::with_capacity(3);
            for j in 0..3 {
                let pos = i * 3 + j;

                let mask = 1 << pos;
                if (self.board_x & mask) != 0 {
                    current_line.push("X".to_string());
                } else if (self.board_o & mask) != 0 {
                    current_line.push("O".to_string());
                } else {
                    current_line.push(" ".to_string());
                }
            }
            println!(
                " {} | {} | {}",
                current_line[0], current_line[1], current_line[2]
            );
            if i > 0 {
                println!("---------");
            }
        }
        println!();
    }
}

impl TicTacToe {
    fn new() -> TicTacToe {
        let mut legal_actions: Vec<(u8, u8)> = Vec::with_capacity(9);
        for i in 0..3 {
            for j in 0..3 {
                legal_actions.push((i, j));
            }
        }
        TicTacToe {
            board_x: 0,
            board_o: 0,
            current_player: 0,
            legal_actions,
        }
    }
}

fn determine_legal_actions(board_x: u16, board_o: u16) -> Vec<(u8, u8)> {
    let mut actions: Vec<(u8, u8)> = Vec::with_capacity(9);
    for pos in 0..9 {
        let mask = 1 << pos;
        if (board_x & mask) == 0 && (board_o & mask) == 0 {
            actions.push((pos / 3, pos % 3));
        }
    }
    actions
}

fn board_is_filled(board_x: u16, board_o: u16) -> bool {
    (board_x | board_o) == 0x1FF
}

fn set_bit(board: &mut u16, pos: u8) {
    *board |= 1 << pos;
}

fn main() {
    let mut game = TicTacToe::new();

    while !game.is_terminal() {
        let mut mcts = Mcts::new(game.clone(), 0.5);
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
