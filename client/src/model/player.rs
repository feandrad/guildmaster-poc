use bevy::ecs::reflect::ReflectComponent;
use bevy::prelude::Component;
use bevy::reflect::Reflect;

#[derive(Component, Clone, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub shape: String,
    pub color: String,
    pub x: i32,
    pub y: i32,
    pub map_id: String,
}
