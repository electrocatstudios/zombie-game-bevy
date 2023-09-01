use bevy::prelude::*;
use std::path::Path;

use super::GameDetails;

use crate::{GAME_WIDTH,GAME_HEIGHT,BUFFER_WIDTH,BUFFER_HEIGHT};
use crate::game::*;
use crate::utils::*;
use crate::bullet::*;

#[derive(Component)]
pub struct Player {
    loc: Vec2,
    mouse: Vec2,
    _hit_box: Vec2
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
            transform: Transform::from_xyz(100.0, 100.0, 3.0).with_scale(Vec3::splat(0.5)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ))
    .insert(Player{
        loc: Vec2::new(100.0,100.0),
        mouse: Vec2::new(0.0,0.0),
        _hit_box: Vec2::new(150.0,150.0)
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
    mut game_details: ResMut<GameDetails>
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

    // Don't go out of bounds
    if player.loc.x < BUFFER_WIDTH {
        player.loc.x = BUFFER_WIDTH;
    }
    if player.loc.y < BUFFER_HEIGHT {
        player.loc.y = BUFFER_HEIGHT;
    }

    let max_x = game_details.width as f32 * GAME_WIDTH;
    if player.loc.x <= GAME_WIDTH / 2.0 {
        transform.translation.x = player.loc.x - GAME_WIDTH/2.0;
        game_details.offset_x = 0.0;
    } else if player.loc.x >= max_x - (GAME_WIDTH/2.0) {
        transform.translation.x = player.loc.x - (max_x - (GAME_WIDTH/2.0));
        game_details.offset_x = max_x - GAME_WIDTH;
    } else {
        transform.translation.x = 0.0;
        game_details.offset_x = player.loc.x - GAME_WIDTH/2.0;
    }

    if player.loc.x >= max_x - BUFFER_WIDTH{
        player.loc.x = max_x - BUFFER_WIDTH;
    }

    let max_y = game_details.height as f32 * GAME_HEIGHT;
    if player.loc.y <= GAME_HEIGHT / 2.0 {
        transform.translation.y = player.loc.y - GAME_HEIGHT/2.0;    
        game_details.offset_y = 0.0;
    } else if player.loc.y >= max_y - (GAME_HEIGHT/2.0) {
        transform.translation.y = player.loc.y - (max_y - (GAME_HEIGHT/2.0));
        game_details.offset_y = max_y - GAME_HEIGHT;
    } else {
        transform.translation.y = 0.0;
        game_details.offset_y = player.loc.y - GAME_HEIGHT/2.0;
    }

    if player.loc.y > max_y - BUFFER_HEIGHT {
        player.loc.y = max_y - BUFFER_HEIGHT;
    }
    
    // Rotate to face the mouse cursor
    let direction = player.mouse - transform.translation.truncate(); 
    let angle_to_target =  normalize_angle(
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

pub fn fire_controller(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    players: Query<(&Player, &Transform)>,
    game_details: Res<GameDetails>,
){
    if players.is_empty() {
        return;
    }

    let (player,transform) = players.single();

    if buttons.just_pressed(MouseButton::Left) {
        // Get player location and spawn a new bullet
        let direction = transform.translation.truncate() - player.mouse; 
        let angle_to_target =  normalize_angle(
            direction.y.atan2(direction.x)
        );

        let texture_path = Path::new("images").join("objects").join("bullet.png");
        let texture_handle = asset_server.load(texture_path);
    
        // println!("Fire.... pos: {:?}, angle: {:?}",player.loc, angle_to_target);

        commands.spawn((
                SpriteBundle {
                    texture: texture_handle,
                    transform: Transform::from_xyz(
                        player.loc.x-game_details.offset_x - (GAME_WIDTH/2.0), 
                        player.loc.y-game_details.offset_y - (GAME_HEIGHT/2.0)
                        , 2.0
                    ).with_scale(Vec3::splat(1.0)),
                    ..default()
                },
            )) 
            .insert(Bullet{
                loc: Vec2::new(player.loc.x, player.loc.y),
                angle: angle_to_target - (std::f32::consts::PI/2.0),
                hit_box: Vec2::new(10.0, 20.0),
                damage: 1
            })
            .insert(OnGameScreen);

    }
}