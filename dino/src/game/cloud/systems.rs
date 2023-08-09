use super::components::Cloud;
use super::resources::CloudSpawnTimer;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const CLOUD_SPEED: f32 = 450.0;

pub fn tick_cloud_spawn_timer(mut cloud_spawn_timer: ResMut<CloudSpawnTimer>, time: Res<Time>) {
    cloud_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_clouds_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    cloud_spawn_timer: Res<CloudSpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    if cloud_spawn_timer.timer.finished() {
        let window: &Window = window_query.get_single().unwrap();
        let mut rng: ThreadRng = rand::thread_rng();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    window.width(),
                    rng.gen_range(0.6..0.9) * window.height(),
                    0.0,
                ),
                texture: asset_server.load("sprites/cloud.png"),
                ..default()
            },
            Cloud {},
        ));
    }
}

pub fn despawn_clouds_over_time(
    mut commands: Commands,
    mut cloud_query: Query<(&Transform, Entity), With<Cloud>>,
) {
    for (cloud_transform, cloud_entity) in cloud_query.iter_mut() {
        if cloud_transform.translation.x < 0.0 {
            commands.entity(cloud_entity).despawn();
        }
    }
}

pub fn move_clouds_over_time(mut cloud_query: Query<&mut Transform, With<Cloud>>, time: Res<Time>) {
    for mut cloud_transform in cloud_query.iter_mut() {
        cloud_transform.translation.x -= CLOUD_SPEED * time.delta_seconds();
    }
}
