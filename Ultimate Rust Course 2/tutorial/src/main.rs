use rusty_engine::prelude::*;


struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

impl Default for GameState{
    fn default() -> Self {
        GameState {
            high_score: 0,
            current_score: 0,
            enemy_labels: Vec::new(),
            spawn_timer: Timer::from_seconds(1.0, false),
        }
    } 
}


fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    game_state.current_score += 1;
    println!("Current score: {}", game_state.current_score);
}

fn main() {
    let mut game = Game::new();

    game.add_logic(game_logic);
    game.run(GameState::default());
}
