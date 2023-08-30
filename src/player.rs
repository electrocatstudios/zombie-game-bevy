use bevy::{app::AppExit, prelude::*};
use bevy::input::mouse::MouseMotion;
use std::vec;
use std::path::Path;

use crate::{GAME_WIDTH,GAME_HEIGHT};
use crate::game::*;
use crate::utils::*;

#[derive(Component)]
pub struct Player {
    loc: Vec2,
    mouse: Vec2,
}

pub fn create_player( 
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
){
    let texture_path = Path::new("images").join("player").join("player.png");
        let texture_handle = asset_server.load(texture_path);
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(200.0, 200.0), 4, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices { first: 0, last: 3 };

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.5)),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .insert(Player{
            loc: Vec2::new(0.0,0.0),
            mouse: Vec2::new(0.0,0.0),
        })
        .insert(OnGameScreen);
}

const PLAYER_MOVE_SPEED: f32 = 150.0;

pub fn player_mover(
    time: Res<Time>,
    mut players: Query<(
        &mut Player,
        &mut Transform,
    )>,
    keys: Res<Input<KeyCode>>,
){
    if players.is_empty() {
        return;
    }
    
    let (mut player, mut transform) = players.single_mut();

    if keys.pressed(KeyCode::W){
        player.loc.y += PLAYER_MOVE_SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::S){
        player.loc.y -= PLAYER_MOVE_SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::A){
        player.loc.x -= PLAYER_MOVE_SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::D){
        player.loc.x += PLAYER_MOVE_SPEED * time.delta_seconds();
    }

    transform.translation.y = player.loc.y;
    transform.translation.x = player.loc.x;

    // Rotate to face the mouse cursor
    let direction = player.mouse - player.loc; 
    let mut angle_to_target =  normalize_angle(
        direction.y.atan2(direction.x)
    );

    transform.rotation = Quat::from_rotation_z(angle_to_target + (std::f32::consts::PI/2.0));
}

pub fn track_mouse(
    mut motion_evr: EventReader<CursorMoved>,
    mut players: Query<&mut Player>
){
    if players.is_empty(){
        return;
    }
    let mut player = players.single_mut();

    for ev in motion_evr.iter() {
        player.mouse.x = ev.position.x - (GAME_WIDTH/2.0);
        player.mouse.y = -1.0 * (ev.position.y - (GAME_HEIGHT/2.0));
        // println!("Standing at: {:?}, Pointing at: {:?}", player.loc, player.mouse);
    }
}