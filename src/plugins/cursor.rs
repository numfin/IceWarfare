mod spawn_cursor;
mod update_cursor_position;

use bevy::prelude::*;

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cursor::spawn_cursor)
            .add_systems(Update, update_cursor_position::update_cursor_position);
    }
}

#[derive(Component)]
pub struct GameCursor;
