use bevy::prelude::*; 
use std::path::Path;

use crate::game::OnGameScreen;
use crate::{GAME_WIDTH, GAME_HEIGHT,GameDetails};

const BLOOD_TTL: f32 = 3.0;
const BLOOD_TTM: f32 = 0.2;
const BLOOD_SPATTER_SPEED: f32 = 380.0;

#[derive(Component)]
pub struct Blood {
    pub ttl: f32,
    pub ttm: f32,
    pub rot: f32,
    pub loc: Vec2, 
}

pub fn add_blood_spatter(
    commands: &mut Commands,
    game_details: &Res<GameDetails>,
    asset_server: &AssetServer,
    loc: Vec2,
    rot: f32,
) {
    let texture_path = Path::new("images").join("objects").join("blood_drop_2.png");
    let texture_handle = asset_server.load(texture_path);

    commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            transform: Transform::from_xyz(
                loc.x - game_details.offset_x - (GAME_WIDTH/2.0), 
                loc.y - game_details.offset_y - (GAME_HEIGHT/2.0), 
                1.0
            ).with_scale(Vec3::splat(3.0)),
            ..default()
        },
    )) 
    .insert(Blood{
        ttl: BLOOD_TTL,
        ttm: BLOOD_TTM,
        rot: rot,
        loc: loc
    })
    .insert(OnGameScreen);
}

pub fn update_blood_spatter(
    mut commands: Commands, 
    mut bloods: Query<(Entity, &mut Blood, &mut Transform),>,
    time: Res<Time>,
    game_details: Res<GameDetails>
){
    for (entity, mut blood, mut transform) in bloods.iter_mut() {
        if blood.ttm >= 0.0 {
            // Move the blood
            blood.ttm -= time.delta_seconds();

            blood.loc.x += blood.rot.sin() * BLOOD_SPATTER_SPEED * time.delta_seconds();
            blood.loc.y += blood.rot.sin() * BLOOD_SPATTER_SPEED * time.delta_seconds();
            
        }else{
            // Decay the blood
            blood.ttl -= time.delta_seconds();
            // println!("blood ttl: {:?}", blood.ttl);
    
            if blood.ttl <= 0.0 {
                commands.entity(entity).despawn();
            }
        }

        // Apply any offset
        transform.translation.x = blood.loc.x - game_details.offset_x - (GAME_WIDTH/2.0);
        transform.translation.y = blood.loc.y - game_details.offset_y - (GAME_HEIGHT/2.0);
    }
}