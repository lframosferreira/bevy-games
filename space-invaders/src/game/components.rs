use bevy::prelude::*;

const ALIEN_LENGTH: f32 = 30.;
const ALIEN_SIZE: Vec2 = Vec2::new(ALIEN_LENGTH, ALIEN_LENGTH);
const ALIEN_SPEED: f32 = 25. * 40.;
const ALIEN_COLOR: Color = Color::RED;
const BARRIER_LENGTH: f32 = 80.;
const BARRIER_SIZE: Vec2 = Vec2::new(BARRIER_LENGTH, BARRIER_LENGTH);
const BARRIER_SPEED: f32 = 0.;
const BARRIER_COLOR: Color = Color::GREEN;
const BOSS_WIDTH: f32 = 60.;
const BOSS_HEIGHT: f32 = 30.;
const BOSS_SIZE: Vec2 = Vec2::new(BOSS_WIDTH, BOSS_HEIGHT);
const BOSS_SPEED: f32 = 200.;
const BOSS_COLOR: Color = Color::WHITE;
const BULLET_LENGTH: f32 = 5.;
const BULLET_SIZE: Vec2 = Vec2::new(BULLET_LENGTH, BULLET_LENGTH);
const BULLET_SPEED: f32 = 250.;
const BULLET_COLOR: Color = Color::GREEN;
const LASER_LENGTH: f32 = 8.;
const LASER_SIZE: Vec2 = Vec2::new(LASER_LENGTH, LASER_LENGTH);
const LASER_SPEED: f32 = 250.;
const LASER_COLOR: Color = Color::RED;
const PLAYER_WIDTH: f32 = 40.;
const PLAYER_HEIGHT: f32 = 20.;
const PLAYER_SIZE: Vec2 = Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT);
const PLAYER_SPEED: f32 = 250.;
const PLAYER_COLOR: Color = Color::GREEN;

#[derive(Component, Clone, Copy)]
pub struct Stats {
    width: f32,
    height: f32,
    size: Vec2,
    speed: f32,
    color: Color,
}

impl Stats {
    pub fn new(width: f32, height: f32, size: Vec2, speed: f32, color: Color) -> Self {
        Self {
            width,
            height,
            size,
            speed,
            color,
        }
    }
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
    pub fn size(&self) -> Vec2 {
        self.size
    }
    pub fn speed(&self) -> f32 {
        self.speed
    }
    pub fn color(&self) -> Color {
        self.color
    }
}

#[derive(Component, Clone, Copy)]
pub struct Player {
    pub stats: Stats,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            stats: Stats::new(
                PLAYER_WIDTH,
                PLAYER_HEIGHT,
                PLAYER_SIZE,
                PLAYER_SPEED,
                PLAYER_COLOR,
            ),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct PlayerBullet {
    pub stats: Stats,
}

impl Default for PlayerBullet {
    fn default() -> Self {
        Self {
            stats: Stats::new(
                BULLET_LENGTH,
                BULLET_LENGTH,
                BULLET_SIZE,
                BULLET_SPEED,
                BULLET_COLOR,
            ),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Alien {
    pub stats: Stats,
}

impl Default for Alien {
    fn default() -> Self {
        Self {
            stats: Stats::new(
                ALIEN_LENGTH,
                ALIEN_LENGTH,
                ALIEN_SIZE,
                ALIEN_SPEED,
                ALIEN_COLOR,
            ),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct AlienLaser {
    pub stats: Stats,
}

impl Default for AlienLaser {
    fn default() -> Self {
        Self {
            stats: Stats::new(
                LASER_LENGTH,
                LASER_LENGTH,
                LASER_SIZE,
                LASER_SPEED,
                LASER_COLOR,
            ),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Barrier {
    pub stats: Stats,
}

impl Default for Barrier {
    fn default() -> Self {
        Self {
            stats: Stats::new(
                BARRIER_LENGTH,
                BARRIER_LENGTH,
                BARRIER_SIZE,
                BARRIER_SPEED,
                BARRIER_COLOR,
            ),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Boss {
    pub stats: Stats,
}

impl Default for Boss {
    fn default() -> Self {
        Self {
            stats: Stats::new(BOSS_WIDTH, BOSS_HEIGHT, BOSS_SIZE, BOSS_SPEED, BOSS_COLOR),
        }
    }
}

#[derive(Component)]
pub struct HitPoints(usize);

impl HitPoints {
    pub fn new(points: usize) -> Self {
        HitPoints(points)
    }

    pub fn hit(&mut self) {
        self.0 -= 1
    }

    pub fn increment(&mut self) {
        self.0 += 1
    }

    pub fn points(&self) -> usize {
        self.0
    }
}
