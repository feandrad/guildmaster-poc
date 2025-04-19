use bevy::ecs::reflect::ReflectComponent;
use bevy::prelude::{Component, Vec2};
use bevy::reflect::Reflect;

#[derive(Component, Clone, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Position {
    pub coords: Vec2,
    pub map_id: String,
}

impl Position {
    pub fn distance_to(&self, other: &Self) -> f32 {
        if self.map_id != other.map_id {
            f32::INFINITY
        } else {
            self.coords.distance(other.coords)
        }
    }

    pub fn is_in_same_map(&self, other: &Self) -> bool {
        self.map_id == other.map_id
    }
}
