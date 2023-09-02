use bevy::prelude::*; 
use std::path::Path;

use super::{
    despawn_screen,
    MainGameState,
    menu::MenuState,
    player,
    bullet,
    blood,
    zombie,
    GameDetails
};

use crate::{GAME_WIDTH,GAME_HEIGHT};

pub struct GamePlugin;

use crate::player::Player;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(MainGameState::Game), game_setup)
            .add_systems(Update, (
                menu_return_check,
                background_mapper,
                game_update,
                zombie::zombie_mover,
                zombie::zombie_checker,
                player::player_mover,
                player::track_mouse,
                player::fire_controller,
                bullet::bullet_mover,
                bullet::bullet_collision,
                blood::update_blood_spatter,
            ).run_if(in_state(MainGameState::Game)))
            .add_systems(OnExit(MainGameState::Game), despawn_screen::<OnGameScreen>);
    }
}

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
struct BackgroundTile{
    x: u32,
    y: u32,
}

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    game_details: Res<GameDetails>
){
    commands
        .spawn(Camera2dBundle::default())
        .insert(OnGameScreen);
    
    player::create_player(&mut commands, &asset_server, &mut texture_atlases);

    // Scenery and Background
    {
        for x in 0..game_details.width {
            for y in 0..game_details.height {
                let texture_path = Path::new("images").join("scenery").join("street_scene.png");
                let texture_handle = asset_server.load(texture_path);
                
                commands.spawn((
                    SpriteBundle {
                        texture: texture_handle,
                        transform: Transform::from_xyz(x as f32 * GAME_WIDTH, y as f32 * GAME_HEIGHT, -1.0).with_scale(Vec3::splat(1.0)),
                        ..default()
                    },
                ))
                .insert(BackgroundTile{
                    x: x,
                    y: y,
                })
                .insert(OnGameScreen);    
            }
        }   
    }
}

fn game_update(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn menu_return_check(
    keys: Res<Input<KeyCode>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<MainGameState>>,
){
    if keys.just_pressed(KeyCode::Escape){
        game_state.set(MainGameState::Menu);
        menu_state.set(MenuState::Main);
        return;
    }
}

fn background_mapper(
    mut _commands: Commands,
    game_details: Res<GameDetails>,
    mut backgrounds: Query<(&BackgroundTile, &mut Transform), (Without<Player>,With<BackgroundTile>)>,
){
    for (tile, mut transform) in backgrounds.iter_mut() {
        transform.translation.x = (tile.x as f32 * GAME_WIDTH) - game_details.offset_x;
        transform.translation.y = (tile.y as f32 * GAME_HEIGHT) - game_details.offset_y;
    }
}