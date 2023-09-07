use std::time::Duration;

use super::components::{Frog, Lake, Vehicle};
use super::resources::VehicleSpawnTimer;
use crate::game::demo::components::SafeHaven;
use crate::game::{BLOCK_LENGTH, WINDOW_X, WINDOW_Y};
use bevy::prelude::*;
use bevy::sprite::collide_aabb;
use common::events::EndGame;
use common::AppState;

const FROG_COLOR: Color = Color::YELLOW_GREEN;
const FROG_WIDTH: f32 = 40.;
const FROG_HEIGHT: f32 = 30.;
const FROG_SIZE: Vec2 = Vec2::new(FROG_WIDTH, FROG_HEIGHT);
const LAKE_SIZE: Vec2 = Vec2::new(WINDOW_X, 6. * BLOCK_LENGTH);
const SAFE_HAVEN_SIZE: Vec2 = Vec2::new(BLOCK_LENGTH, BLOCK_LENGTH);
const VEHICLES: [Vehicle; 10] = [
    Vehicle::new(70., true, true, Color::ORANGE, 60., 4.0),
    Vehicle::new(60., false, true, Color::ANTIQUE_WHITE, 70., 6.),
    Vehicle::new(60., true, true, Color::VIOLET, 80., 4.4),
    Vehicle::new(60., false, true, Color::SEA_GREEN, 70., 4.8),
    Vehicle::new(90., true, true, Color::CRIMSON, 40., 6.),
    Vehicle::new(150., true, false, Color::TOMATO, 80., 6.),
    Vehicle::new(100., false, false, Color::MAROON, 70., 4.),
    Vehicle::new(250., false, false, Color::MAROON, 120., 5.),
    Vehicle::new(100., true, false, Color::TOMATO, 70., 4.),
    Vehicle::new(200., false, false, Color::MAROON, 120., 4.5),
];

pub fn init_timers(mut commands: Commands) {
    let timers: Vec<f32> = VEHICLES.iter().map(|x| x.timer_seconds).collect();
    let mut vec: Vec<Timer> = timers
        .iter()
        .map(|x| Timer::from_seconds(*x, TimerMode::Repeating))
        .collect();
    for (i, v) in vec.iter_mut().enumerate() {
        v.set_elapsed(Duration::from_secs_f32(timers[i]));
    }
    commands.insert_resource(VehicleSpawnTimer(vec));
}

pub fn tick_timers(mut vehicle_timers: ResMut<VehicleSpawnTimer>, time: Res<Time>) {
    for timer in &mut vehicle_timers.0 {
        timer.tick(time.delta());
    }
}

pub fn spawn_scenario(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::AQUAMARINE,
                custom_size: Some(LAKE_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(WINDOW_X / 2., WINDOW_Y - 3. * BLOCK_LENGTH, 0.),
            ..default()
        },
        Lake,
    ));
    commands.spawn_batch(vec![
        SpriteBundle {
            sprite: Sprite {
                color: Color::INDIGO,
                custom_size: Some(Vec2::new(WINDOW_X, BLOCK_LENGTH)),
                ..default()
            },
            transform: Transform::from_xyz(WINDOW_X / 2., BLOCK_LENGTH / 2., 0.),
            ..default()
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::INDIGO,
                custom_size: Some(Vec2::new(WINDOW_X, BLOCK_LENGTH)),
                ..default()
            },
            transform: Transform::from_xyz(
                WINDOW_X / 2.,
                6. * BLOCK_LENGTH + BLOCK_LENGTH / 2.,
                0.,
            ),
            ..default()
        },
    ]);
    const START_SAFE_X_OFFSET: f32 = 20.;
    const SAFE_X_OFFSET: f32 = 126.;
    let safe_spots: [(SpriteBundle, SafeHaven); 5] = core::array::from_fn(|x| {
        (
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(SAFE_HAVEN_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(
                    START_SAFE_X_OFFSET + BLOCK_LENGTH / 2. + (SAFE_X_OFFSET) * x as f32,
                    WINDOW_Y - BLOCK_LENGTH / 2.,
                    0.5,
                ),
                ..default()
            },
            SafeHaven,
        )
    });
    commands.spawn_batch(safe_spots);
}

pub fn spawn_frog(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: FROG_COLOR,
                custom_size: Some(FROG_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(WINDOW_X / 2., BLOCK_LENGTH / 2., 1.),
            ..default()
        },
        Frog(None),
    ));
}

pub fn respawn_frog(mut commands: Commands, frog_query: Query<Entity, With<Frog>>) {
    if let Ok(entity) = frog_query.get_single() {
        commands.entity(entity).despawn();
    }
    spawn_frog(commands);
}

pub fn move_frog(
    mut frog_query: Query<(&mut Transform, &mut Frog), With<Frog>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_over_event_writer: EventWriter<EndGame>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut frog)) = frog_query.get_single_mut() {
        let translation = &mut transform.translation;
        if let Some(ride_speed) = frog.0 {
            translation.x += ride_speed * time.delta_seconds();
            // The ride made you go out of bounds, so game over
            if translation.x <= -FROG_WIDTH / 2. || translation.x >= WINDOW_X + FROG_WIDTH / 2. {
                commands.insert_resource(NextState(Some(AppState::GameOver)));
                game_over_event_writer.send(EndGame::new_number(0));
            }
        }
        if keyboard_input.just_pressed(KeyCode::Right)
            && translation.x + BLOCK_LENGTH < WINDOW_X - FROG_WIDTH / 2.
        {
            translation.x += BLOCK_LENGTH;
        }
        if keyboard_input.just_pressed(KeyCode::Left)
            && translation.x - BLOCK_LENGTH > FROG_WIDTH / 2.
        {
            translation.x -= BLOCK_LENGTH;
        }
        if keyboard_input.just_pressed(KeyCode::Up)
            && translation.y + BLOCK_LENGTH < WINDOW_Y - FROG_HEIGHT / 2.
        {
            translation.y += BLOCK_LENGTH;
            // We must reset ride_speed on chaging y
            frog.0 = None;
        }
        if keyboard_input.just_pressed(KeyCode::Down) && translation.y - BLOCK_LENGTH >= 0. {
            translation.y -= BLOCK_LENGTH;
            // We must reset ride_speed on chaging y
            frog.0 = None;
        }
    }
}

pub fn spawn_vehicles(mut commands: Commands, vehicle_timers: Res<VehicleSpawnTimer>) {
    for (i, timer) in vehicle_timers.0.iter().enumerate() {
        if timer.finished() {
            let vehicle = VEHICLES[i];
            let x = if vehicle.moves_to_left {
                WINDOW_X + vehicle.width / 2.
            } else {
                -vehicle.width / 2.
            };
            let lane = if i >= 5 { i + 2 } else { i + 1 };
            let y = BLOCK_LENGTH / 2. + lane as f32 * BLOCK_LENGTH;
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: vehicle.color,
                        custom_size: Some(vehicle.size()),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.5),
                    ..default()
                },
                vehicle,
            ));
        }
    }
}

pub fn move_vehicles(
    mut commands: Commands,
    mut vehicles_query: Query<(&mut Transform, &Vehicle, Entity), With<Vehicle>>,
    time: Res<Time>,
) {
    for (mut transform, vehicle, entity) in vehicles_query.iter_mut() {
        let translation = &mut transform.translation;
        let delta = vehicle.speed * time.delta_seconds();
        if vehicle.moves_to_left {
            if translation.x > -vehicle.width / 2. {
                translation.x -= delta;
            } else {
                commands.entity(entity).despawn();
            }
        } else if translation.x < WINDOW_X + vehicle.width / 2. {
            translation.x += delta;
        } else {
            commands.entity(entity).despawn();
        }
    }
}

pub fn collide(
    vehicles_query: Query<(&Transform, &Vehicle), With<Vehicle>>,
    lake_query: Query<&Transform, With<Lake>>,
    haven_query: Query<&Transform, With<SafeHaven>>,
    mut frog_query: Query<(&Transform, &mut Frog, Entity), With<Frog>>,
    mut game_over_event_writer: EventWriter<EndGame>,
    mut commands: Commands,
) {
    if let Ok((frog_transform, mut frog, entity)) = frog_query.get_single_mut() {
        let frog_translation = frog_transform.translation;
        for (transform, vehicle) in vehicles_query.iter() {
            if vehicle.is_harmful
                && collide_aabb::collide(
                    frog_translation,
                    FROG_SIZE,
                    transform.translation,
                    vehicle.size(),
                )
                .is_some()
            {
                commands.insert_resource(NextState(Some(AppState::GameOver)));
                game_over_event_writer.send(EndGame::new_number(0));
            }
        }
        for haven_transform in haven_query.iter() {
            let haven_translation = haven_transform.translation;
            if collide_aabb::collide(
                haven_translation,
                SAFE_HAVEN_SIZE,
                frog_translation,
                FROG_SIZE,
            )
            .is_some()
                && frog_translation.x <= haven_translation.x + BLOCK_LENGTH / 2.
                && frog_translation.x >= haven_translation.x - BLOCK_LENGTH / 2.
            {
                commands.entity(entity).remove::<Frog>();
                spawn_frog(commands);
                return;
            }
        }
        if let Ok(lake_transform) = lake_query.get_single() {
            if collide_aabb::collide(
                lake_transform.translation,
                LAKE_SIZE,
                frog_translation,
                FROG_SIZE,
            )
            .is_some()
            {
                for (vehicle_transform, vehicle) in vehicles_query.iter() {
                    let vehicle_translation = vehicle_transform.translation;
                    let vehicle_size = vehicle.size();
                    let vehicle_width = vehicle_size.x;
                    if !vehicle.is_harmful
                        && collide_aabb::collide(
                            frog_translation,
                            FROG_SIZE,
                            vehicle_translation,
                            vehicle_size,
                        )
                        .is_some()
                        && frog_translation.x <= vehicle_translation.x + vehicle_width / 2.
                        && frog_translation.x >= vehicle_translation.x - vehicle_width / 2.
                    {
                        frog.0 = Some(if vehicle.moves_to_left {
                            -vehicle.speed
                        } else {
                            vehicle.speed
                        });
                        return;
                    }
                }
                commands.insert_resource(NextState(Some(AppState::GameOver)));
                game_over_event_writer.send(EndGame::new_number(0));
            }
        }
    }
}
