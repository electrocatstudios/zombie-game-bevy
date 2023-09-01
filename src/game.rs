use bevy::prelude::*; 
use std::path::Path;

use super::{despawn_screen,MainGameState,menu::MenuState,player,bullet,blood,GameDetails};
use std::vec::Vec;

use crate::{GAME_WIDTH,GAME_HEIGHT};

pub struct GamePlugin;

use crate::player::Player;
use crate::utils::*;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(MainGameState::Game), game_setup)
            .add_systems(Update, (
                menu_return_check,
                background_mapper,
                game_update,
                zombie_mover,
                zombie_checker,
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

#[derive(Component)]
pub struct Zombie {
    pub pos: Vec2,
    pub loc: Vec::<Vec2>,
    pub cur_loc: usize,
    pub hit_box: Vec2,
    pub health: i32
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

const ZOMBIE_SPEED: f32 = 150.0;

fn zombie_mover(
    time: Res<Time>,
    mut zombies: Query<(
        &mut Zombie,
        &mut Transform,
    )>,
    game_details: Res<GameDetails>
){
    for (mut zombie, mut transform) in &mut zombies {
        // Get where we are and where we are heading
        // let pos = zombie.pos; //transform.translation.truncate();
        let target = Vec2::new(zombie.loc[zombie.cur_loc].x, zombie.loc[zombie.cur_loc].y);
        
        // Calculate angle of rotation (plus fiddle factor)
        let direction = target - zombie.pos;
        let mut angle_to_target =  normalize_angle(
            direction.y.atan2(direction.x)
        );

        angle_to_target = angle_to_target + (std::f32::consts::PI/2.0); // Add 90 degrees because of image rotation
        
        // Now move towards the goal point using the angle of rotation
        let cur_loc = zombie.loc[zombie.cur_loc];
        
        let mut x_met = false;
        if cur_loc.x != zombie.pos.x {
            if (cur_loc.x - zombie.pos.x).abs() <= ZOMBIE_SPEED * time.delta_seconds() {
                zombie.pos.x = cur_loc.x;
                x_met = true;
            }else{
                zombie.pos.x += angle_to_target.sin() * (ZOMBIE_SPEED*time.delta_seconds());
            }
        }else{
            x_met = true;
        }
        
        let mut y_met = false;
        if cur_loc.y != zombie.pos.y {
            if (cur_loc.y - zombie.pos.y).abs() <= ZOMBIE_SPEED * time.delta_seconds() {
                zombie.pos.y = cur_loc.y;
                y_met = true;
            }else{
                zombie.pos.y -= angle_to_target.cos() * (ZOMBIE_SPEED*time.delta_seconds());
            }
        }else{
            y_met = true;
        }

        // Finally apply translation and rotation, taking into account offset
        transform.rotation = Quat::from_rotation_z(angle_to_target);
        transform.translation.x = zombie.pos.x - game_details.offset_x - (GAME_WIDTH/2.0);
        transform.translation.y = zombie.pos.y - game_details.offset_y - (GAME_HEIGHT/2.0);

        // We have hit our mark - move onto next point
        if x_met && y_met {
            zombie.cur_loc += 1;
            if zombie.cur_loc >= zombie.loc.len() {
                zombie.cur_loc = 0;
            }
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

fn zombie_checker(
    mut commands: Commands,
    zombies: Query<&Zombie>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    game_details: Res<GameDetails>
){
    if zombies.is_empty() {
        // Make a zombie
        let texture_path = Path::new("images").join("zombie").join("zombie.png");
        let texture_handle = asset_server.load(texture_path);
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(200.0, 200.0), 4, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices { first: 0, last: 3 };
        
        let mut locations = Vec::<Vec2>::new();
        locations.push(Vec2::new(400.0, 200.0));
        locations.push(Vec2::new(250.0, 450.0)); 
        locations.push(Vec2::new(400.0, 600.0));
        locations.push(Vec2::new(900.0, 600.0));
        locations.push(Vec2::new(900.0, 200.0));
        
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform::from_xyz(
                    locations[0].x - game_details.offset_x - (GAME_WIDTH/2.0),
                    locations[0].y - game_details.offset_y - (GAME_HEIGHT/2.0),
                    2.0
                ).with_scale(Vec3::splat(0.5)),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .insert(Zombie{
            pos: Vec2::new(locations[0].x, locations[0].y),
            loc: locations.clone(),
            cur_loc: 1,
            hit_box: Vec2::new(100.0,100.0),
            health: 5
        })
        .insert(OnGameScreen);

        let animation_indices = AnimationIndices { first: 0, last: 3 };
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform::from_xyz(
                    locations[3].x - game_details.offset_x - (GAME_WIDTH/2.0),
                    locations[3].y - game_details.offset_y - (GAME_HEIGHT/2.0),
                    2.0
                ).with_scale(Vec3::splat(0.5)),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .insert(Zombie{
            pos: Vec2::new(locations[3].x, locations[3].y),
            loc: locations,
            cur_loc: 4,
            hit_box: Vec2::new(100.0,100.0),
            health: 5
        })
        .insert(OnGameScreen);
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