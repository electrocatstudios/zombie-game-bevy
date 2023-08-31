use bevy::{app::AppExit, prelude::*};
use bevy::sprite::collide_aabb::collide;
use rand::Rng; 

use crate::{GAME_WIDTH,GAME_HEIGHT};
use crate::game::Zombie;
use crate::blood;

#[derive(Component)]
pub struct Bullet {
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

pub fn bullet_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bullets: Query<(Entity, &mut Bullet, &mut Transform,), (With<Bullet>, Without<Zombie>)>,
    mut zombies: Query<(Entity, &mut Zombie, &mut Transform), (With<Zombie>, Without<Bullet>)>,
){
    let mut rng = rand::thread_rng();

    for (bullet_entity, mut bullet, bullet_transform) in bullets.iter_mut() {
        for (zombie_entity, mut zombie, zombie_transform) in zombies.iter_mut() {
            if collide(bullet_transform.translation, bullet.hit_box, zombie_transform.translation, zombie.hit_box).is_some() {
                let cur_pos = Vec2::new(zombie_transform.translation.x,zombie_transform.translation.y);
                
                for _ in 0..rng.gen_range(2..4){
                    let angle_diff = rng.gen_range(-std::f32::consts::PI/3.0..std::f32::consts::PI/3.0);
                    blood::add_blood_spatter(&mut commands, &asset_server, cur_pos, bullet.angle - (std::f32::consts::PI/2.0) + angle_diff);
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