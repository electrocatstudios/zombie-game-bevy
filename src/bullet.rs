use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::Rng; 

use crate::{GAME_WIDTH,GAME_HEIGHT,GameDetails};
use crate::zombie::Zombie;
use crate::blood;

#[derive(Component)]
pub struct Bullet {
    pub loc: Vec2,
    pub angle: f32,
    pub hit_box: Vec2,
    pub damage: i32
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
    game_details: Res<GameDetails>
){
    for (entity, mut bullet, mut transform) in bullets.iter_mut(){
        transform.rotation = Quat::from_rotation_z(bullet.angle);
        bullet.loc.x += bullet.angle.sin() * (BULLET_SPEED * time.delta_seconds());
        bullet.loc.y -= bullet.angle.cos() * (BULLET_SPEED * time.delta_seconds());
        transform.translation.x = bullet.loc.x - game_details.offset_x - (GAME_WIDTH/2.0);
        transform.translation.y = bullet.loc.y - game_details.offset_y - (GAME_HEIGHT/2.0);

        // Catch-all to make sure bullet doesn't live forever, but it should hit an object, ideally
        if bullet.loc.x < 0.0 || bullet.loc.x >= GAME_WIDTH * game_details.width as f32 
            || bullet.loc.y < 0.0 || bullet.loc.y >= GAME_HEIGHT * game_details.height as f32 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn bullet_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    bullets: Query<(Entity, &Bullet, &Transform,), (With<Bullet>, Without<Zombie>)>,
    mut zombies: Query<(Entity, &mut Zombie, &mut Transform), (With<Zombie>, Without<Bullet>)>,
    game_details: Res<GameDetails>
){
    let mut rng = rand::thread_rng();

    for (bullet_entity, bullet, bullet_transform) in bullets.iter() {
        for (zombie_entity, mut zombie, zombie_transform) in zombies.iter_mut() {
            if collide(bullet_transform.translation, bullet.hit_box, zombie_transform.translation, zombie.hit_box).is_some() {
                let cur_pos = Vec2::new(zombie.pos.x,zombie.pos.y);
                
                for _ in 0..rng.gen_range(2..4){
                    let angle_diff = rng.gen_range(-std::f32::consts::PI/3.0..std::f32::consts::PI/3.0);
                    blood::add_blood_spatter(
                        &mut commands,
                        &game_details,
                        &asset_server,
                        cur_pos,
                        bullet.angle - (std::f32::consts::PI/2.0) + angle_diff
                    );
                }
                
                zombie.health -= bullet.damage;
                commands.entity(bullet_entity).despawn();
                if zombie.health <= 0 {
                    commands.entity(zombie_entity).despawn();
                }
            }
        }
    }
}