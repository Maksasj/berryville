use bevy::prelude::*;

#[derive(Component)]
pub struct Rotation {
    pub angle: f32
}

impl Rotation {
    pub fn new(value: f32) -> Self {
        Rotation { angle: value }
    }
}