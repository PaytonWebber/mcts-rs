trait State {
    fn is_terminal(&self) -> bool;
    fn get_legal_actions(board: &[[[i32; 2]; 3]; 3]) -> Vec<(usize, usize)>;
    fn step(&self, action: (usize, usize)) -> Self;
    fn reward(&self, to_play: usize) -> f32;
    fn render(&self);
}

const WINNER_MASK: [[[i32; 3]; 3]; 8] = [
    [[1, 1, 1], [0, 0, 0], [0, 0, 0]], // First row
    [[0, 0, 0], [1, 1, 1], [0, 0, 0]], // Second row
    [[0, 0, 0], [0, 0, 0], [1, 1, 1]], // Third row
    [[1, 0, 0], [1, 0, 0], [1, 0, 0]], // First column
    [[0, 1, 0], [0, 1, 0], [0, 1, 0]], // Second column
    [[0, 0, 1], [0, 0, 1], [0, 0, 1]], // Third column
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]], // Main diagonal
    [[0, 0, 1], [0, 1, 0], [1, 0, 0]], // Anti-diagonal
];

#[derive(Debug)]
struct TicTacToe {
    board: [[[i32; 2]; 3]; 3],
    current_player: usize,
    legal_actions: Vec<(usize, usize)>,
}

impl State for TicTacToe {
    fn is_terminal(&self) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if (self.board[i][j][0] == 0) && (self.board[i][j][1] == 0) {
                    return false;
                }
            }
        }
        true
    }

    fn get_legal_actions(board: &[[[i32; 2]; 3]; 3]) -> Vec<(usize, usize)> {
        let mut legal_actions: Vec<(usize, usize)> = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                if (board[i][j][0] == 0) && (board[i][j][1] == 0) {
                    legal_actions.push((i, j));
                }
            }
        }
        legal_actions
    }

    fn step(&self, action: (usize, usize)) -> Self {
        let mut new_board = self.board;
        new_board[action.0][action.1][self.current_player] = 1;

        let mut new_legal_actions = self.legal_actions.clone();
        if let Some(index) = new_legal_actions.iter().position(|&x| x == action) {
            new_legal_actions.remove(index);
        }
        TicTacToe {
            board: new_board,
            current_player: 1 - self.current_player,
            legal_actions: new_legal_actions,
        }
    }

    fn reward(&self, to_play: usize) -> f32 {
        assert!(self.is_terminal());
        // Implement reward logic here
        0.0
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
    fn new() -> Self {
        let board = [[[0; 2]; 3]; 3];
        TicTacToe {
            board,
            current_player: 0,
            legal_actions: TicTacToe::get_legal_actions(&board),
        }
    }

    // Uncomment if clone functionality is needed
    // fn clone(&self) -> Self {
    //     TicTacToe {
    //         board: self.board,
    //         current_player: self.current_player,
    //         legal_actions: self.legal_actions.clone(),
    //     }
    // }
}

fn main() {
    let mut ttt = TicTacToe::new();
    ttt.render();
    println!("{:?}", ttt);
    ttt = ttt.step((0, 0));
    ttt.render();
    println!("{:?}", ttt);
    ttt = ttt.step((0, 1));
    ttt.render();
    println!("{:?}", ttt);
}
