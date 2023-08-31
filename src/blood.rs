use bevy::{app::AppExit, prelude::*,sprite::MaterialMesh2dBundle}; 
use std::path::Path;

use crate::game::OnGameScreen;

const BLOOD_TTL: f32 = 3.0;
const BLOOD_TTM: f32 = 0.2;
const BLOOD_SPATTER_SPEED: f32 = 380.0;

#[derive(Component)]
pub struct Blood {
    pub ttl: f32,
    pub ttm: f32,
    pub rot: f32,
}

pub fn add_blood_spatter(commands: &mut Commands, asset_server: &AssetServer, loc: Vec2, rot: f32) {
    let texture_path = Path::new("images").join("objects").join("blood_drop_2.png");
    let texture_handle = asset_server.load(texture_path);

    commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            transform: Transform::from_xyz(loc.x, loc.y, 1.0).with_scale(Vec3::splat(3.0)),
            ..default()
        },
    )) 
    .insert(Blood{
        ttl: BLOOD_TTL,
        ttm: BLOOD_TTM,
        rot: rot,
    })
    .insert(OnGameScreen);
}

pub fn update_blood_spatter(
    mut commands: Commands, 
    mut bloods: Query<(Entity, &mut Blood, &mut Transform),>,
    time: Res<Time>,
){
    for (entity, mut blood, mut transform) in bloods.iter_mut() {
        if blood.ttm >= 0.0 {
            // Move the blood
            blood.ttm -= time.delta_seconds();

            transform.translation.x += blood.rot.sin() * BLOOD_SPATTER_SPEED * time.delta_seconds();
            transform.translation.y += blood.rot.sin() * BLOOD_SPATTER_SPEED * time.delta_seconds();
            
        }else{
            // Decay the blood
            blood.ttl -= time.delta_seconds();
            // println!("blood ttl: {:?}", blood.ttl);
    
            if blood.ttl <= 0.0 {
                commands.entity(entity).despawn();
            }
        }
        
    }
}