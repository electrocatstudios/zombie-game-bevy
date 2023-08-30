use bevy::{app::AppExit, prelude::*};

use crate::{GAME_WIDTH,GAME_HEIGHT};

#[derive(Component)]
pub struct Bullet {
    pub angle: f32
}

const BULLET_SPEED: f32 = 500.0;

pub fn bullet_mover(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(
        Entity,
        &mut Bullet,
        &mut Transform,
    )>,
){
    for (entity, bullet, mut transform) in bullets.iter_mut(){
        transform.rotation = Quat::from_rotation_z(bullet.angle);
        transform.translation.x += bullet.angle.sin() * (BULLET_SPEED * time.delta_seconds());
        transform.translation.y -= bullet.angle.cos() * (BULLET_SPEED * time.delta_seconds());
    
        if transform.translation.x < -(GAME_WIDTH / 2.0) || transform.translation.x > (GAME_WIDTH / 2.0)
            || transform.translation.y < -(GAME_HEIGHT/2.0) || transform.translation.y > (GAME_HEIGHT  / 2.0) {
            commands.entity(entity).despawn();
            // println!("Bullet out of bounds");
        }
    }
}