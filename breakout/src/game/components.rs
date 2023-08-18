use bevy::prelude::Component;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Block(pub usize);

#[derive(Component)]
pub struct Ball {
    pub is_going_right: bool,
    pub is_going_up: bool,
    pub x_speed: f32,
    pub y_speed: f32,
}

const BALL_X_SPEED: f32 = 500.;
const BALL_Y_SPEED: f32 = 200.;

impl Default for Ball {
    fn default() -> Self {
        Ball {
            is_going_right: true,
            is_going_up: true,
            x_speed: BALL_X_SPEED,
            y_speed: BALL_Y_SPEED,
        }
    }
}
