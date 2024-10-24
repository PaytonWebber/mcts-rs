pub trait State {
    fn player_has_won(&self, player: usize) -> bool;
    fn is_terminal(&self) -> bool;
    fn get_legal_actions(&self) -> Vec<(usize, usize)>;
    fn to_play(&self) -> usize;
    fn step(&self, action: (usize, usize)) -> Self;
    fn reward(&self, to_play: usize) -> f32;
    fn render(&self);
}
