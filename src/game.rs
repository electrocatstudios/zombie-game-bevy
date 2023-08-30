use bevy::{app::AppExit, prelude::*,sprite::MaterialMesh2dBundle}; 
use std::path::Path;

use super::{despawn_screen,MainGameState,menu::MenuState,player};
use std::vec::Vec;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(MainGameState::Game), game_setup)
            .add_systems(Update, (
                menu_return_check,
                game_update,
                zombie_mover,
                player::player_mover,
                player::track_mouse,
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
struct Zombie {
    loc: Vec::<Vec2>,
    cur_loc: usize
}

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    commands
        .spawn(Camera2dBundle::default())
        .insert(OnGameScreen);
    
    // Zombie
    {
        let texture_path = Path::new("images").join("zombie").join("zombie.png");
        let texture_handle = asset_server.load(texture_path);
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(200.0, 200.0), 4, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices { first: 0, last: 3 };
        
        let mut locations = Vec::<Vec2>::new();
        locations.push(Vec2::new(-300.0, -200.0));
        locations.push(Vec2::new(-350.0, 0.0)); 
        locations.push(Vec2::new(-300.0, 200.0));
        locations.push(Vec2::new(300.0, 200.0));
        locations.push(Vec2::new(300.0, -200.0));
        
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform::from_xyz(locations[0].x, locations[0].y, 0.0).with_scale(Vec3::splat(0.5)),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .insert(Zombie{
            loc: locations,
            cur_loc: 1
        })
        .insert(OnGameScreen);
    
    }
    
    player::create_player(&mut commands, &asset_server, &mut texture_atlases);
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
){
    for (mut zombie, mut transform) in &mut zombies {
        // Get where we are and where we are heading
        let pos = transform.translation.truncate();
        let target = Vec2::new(zombie.loc[zombie.cur_loc].x, zombie.loc[zombie.cur_loc].y);
        
        // Calculate angle of rotation (plus fiddle factor)
        let direction = target - pos; 
        let mut angle_to_target =  direction.y.atan2(direction.x); //(std::f32::consts::PI * 2.0) -
        if angle_to_target < 0. {
            angle_to_target += 2.0*std::f32::consts::PI;
        }else if angle_to_target > std::f32::consts::PI*2.0{
            angle_to_target -= std::f32::consts::PI*2.0;
        }

        angle_to_target = angle_to_target + (std::f32::consts::PI/2.0); // Add 90 degrees because of image rotation
        transform.rotation = Quat::from_rotation_z(angle_to_target);

        // Now move towards the goal point using the angle of rotation
        let cur_loc = zombie.loc[zombie.cur_loc];
        let mut x_met = false;

        if cur_loc.x != transform.translation.x {
            if (cur_loc.x - transform.translation.x).abs() <= ZOMBIE_SPEED * time.delta_seconds() {
                transform.translation.x = cur_loc.x;
                x_met = true;
            }else{
                transform.translation.x += angle_to_target.sin() * (ZOMBIE_SPEED*time.delta_seconds());
            }
        }else{
            x_met = true;
        }

        let mut y_met = false;

        if cur_loc.y != transform.translation.y {
            if (cur_loc.y - transform.translation.y).abs() <= ZOMBIE_SPEED * time.delta_seconds() {
                transform.translation.y = cur_loc.y;
                y_met = true;
            }else{
                transform.translation.y -= angle_to_target.cos() * (ZOMBIE_SPEED*time.delta_seconds());
            }
        }else{
            y_met = true;
        }

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