use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct Health {
    pub value: f32,
}
