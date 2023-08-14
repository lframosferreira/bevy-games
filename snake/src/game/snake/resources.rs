use bevy::prelude::Resource;

#[derive(Resource)]
pub struct SnakeCounter {
    pub count: u32,
}

impl Default for SnakeCounter {
    fn default() -> Self {
        // Número inicial de peças no CORPO (excluindo cabeça)
        Self { count: 1 }
    }
}
