use bevy::{prelude::*};
use bevy_rapier2d::prelude::*;
use nalgebra::{Vector2, vector};

use crate::particles;

pub struct PlayerMovement {
    pub speed: f32,
}

pub struct PlayerShooting {
    smoke_mat: Handle<ColorMaterial>,
}

pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_parameters: Res<RapierConfiguration>,
    mut query: Query<(&PlayerMovement, &mut RigidBodyVelocity)>
) {
    if let Ok((player, mut rb_vels)) = query.single_mut() {
        let mut y_movement = 0.0;
        let mut x_movement = 0.0; 
        if keyboard_input.pressed(KeyCode::W) {
            y_movement += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            y_movement -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            x_movement -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            x_movement += 1.0;
        }

        let mut movement = vector![x_movement, y_movement];
        if movement != Vector2::zeros() 
        {
            movement = movement.normalize() * (1.0 / rapier_parameters.scale) * player.speed;
        }

        rb_vels.linvel = movement;

        //println!("Moving ({}, {}) based on input ({}, {}", movement.x, movement.y, x_movement, y_movement);
    }
}

pub fn player_shoot_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    query: Query<(&PlayerShooting, &Transform)>
) {
    if let Ok((player, transform)) = query.single() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            println!("player position: {}", transform.translation);

            let block_size = 50.0;

            commands.spawn()
                .insert(particles::BurstParticleEmitter {
                    quantity: 100,
                    existence_time: 0.0,
                })
                .insert(particles::ParticleEmissionParams {
                    speed_min: 10.0,
                    speed_max: 100.0,
                    particle_drag: 0.01,
                    particle_size: Vec2::new(20.0, 20.0),
                    lifetime_min: 3.0,
                    lifetime_max: 10.0,
                    material: player.smoke_mat.clone(),
                })
                .insert(Transform::from_translation(transform.translation))
                .insert(crate::lighting::DynamicLightBlocker{size: block_size})
                .insert_bundle(ColliderBundle {
                    position: [transform.translation.x / rapier_config.scale, transform.translation.y / rapier_config.scale].into(),
                    shape: ColliderShape::ball(block_size * 0.5 / rapier_config.scale),
                    collider_type: ColliderType::Sensor,
                    ..Default::default()
                })
                ;
            };
    }
}

pub fn setup_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rapier_config: Res<RapierConfiguration>,
    asset_server: Res<AssetServer>,
) {
    // Load sprite
    let circle_texture_handle: Handle<Texture> = asset_server.load("sprites/circle.png");

    let sprite_size_x = 40.0;
    let sprite_size_y = 40.0;

    let collider_size_x = sprite_size_x / rapier_config.scale;
    let collider_size_y = sprite_size_y / rapier_config.scale;

    commands
    .spawn()
    .insert_bundle(SpriteBundle {
        material: materials.add(circle_texture_handle.into()),
        sprite: Sprite::new(Vec2::new(sprite_size_x, sprite_size_y)),
        ..Default::default()
    })
    .insert_bundle(RigidBodyBundle {
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        ..Default::default()
    })
    .insert_bundle(ColliderBundle {
        position: [collider_size_x / 2.0, collider_size_y / 2.0].into(),
        shape: ColliderShape::ball(collider_size_x * 0.5),
        ..Default::default()
    })
    .insert(ColliderPositionSync::Discrete)
    .insert(PlayerMovement {speed: 200.0})
    .insert(PlayerShooting {smoke_mat: materials.add(Color::rgb(0.0, 0.3, 0.5).into())})
    .insert(crate::lighting::DynamicLightBlocker{size: 20.0})
    ;
}
