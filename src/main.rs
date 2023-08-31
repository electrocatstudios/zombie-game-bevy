use bevy::{prelude::*}; 
use bevy::window::PresentMode;

mod game;
mod menu;
mod player;
mod utils;
mod bullet;
mod blood; 

pub const GAME_WIDTH: f32 = 1280.0;
pub const GAME_HEIGHT: f32 = 720.0;

#[derive(Clone, Eq, PartialEq, Debug, Hash, States, Default)]
pub enum MainGameState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zombie Game".to_string(),
                resolution: (GAME_WIDTH, GAME_HEIGHT).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_state::<MainGameState>()
        .add_plugins((
            menu::MenuPlugin,
            game::GamePlugin
        ))
        .run();
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}