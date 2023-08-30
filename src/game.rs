use bevy::{app::AppExit, prelude::*,sprite::MaterialMesh2dBundle}; 
use std::path::Path;

use super::{despawn_screen,MainGameState,menu::MenuState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(MainGameState::Game), game_setup)
            .add_systems(Update, (menu_return_check,game_update).run_if(in_state(MainGameState::Game)))
            .add_systems(OnExit(MainGameState::Game), despawn_screen::<OnGameScreen>);
    }
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    commands
        .spawn(Camera2dBundle::default())
        .insert(OnGameScreen);
    
    let texture_path = Path::new("images").join("zombie").join("zombie.png");
    let texture_handle = asset_server.load(texture_path);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(200.0, 200.0), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 3 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(0.5)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ))
    .insert(OnGameScreen);
}

fn game_update(
    time: Res<Time>,
    _game_state: ResMut<NextState<MainGameState>>,
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