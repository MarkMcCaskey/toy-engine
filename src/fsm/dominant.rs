//! The overall state of the game
//! i.e. loading/paused/in menu/playing
//! This will be fully static

pub trait DominantState {
    pub fn draw(&self);
    pub fn handle_input(&self);
}

