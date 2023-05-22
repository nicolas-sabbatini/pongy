use bevy::prelude::*;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(set_up_player);
    }
}

fn set_up_player() {}
