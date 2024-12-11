pub trait State: Send + Sync {
    type Action: Send + Sync + Clone;
    fn default_action() -> Self::Action;

    fn player_has_won(&self, player: usize) -> bool;
    fn is_terminal(&self) -> bool;
    fn get_legal_actions(&self) -> Vec<Self::Action>;
    fn to_play(&self) -> usize;
    fn step(&self, action: &Self::Action) -> Self;
    fn reward(&self, to_play: usize) -> f32;
    fn render(&self);
}
