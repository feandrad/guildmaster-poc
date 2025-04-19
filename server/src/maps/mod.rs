use crate::math::shape::{IShape, Shape, Vector2};
use spacetimedb::SpacetimeType;

pub mod tavern_outside;

#[derive(SpacetimeType, Clone, Debug)]
pub struct Coords {
    pub map_id: String,
    pub position: Vector2,
}

#[derive(SpacetimeType, Clone, Debug)]
pub struct Area {
    pub shape: Shape,
    pub coords: Coords,
}

impl Area {
    pub fn contains_point(&self, point: &Vector2) -> bool {
        match &self.shape {
            Shape::Rectangle(rect) => rect.contains_point(point),
            Shape::Circle(circle) => circle.contains_point(point),
            Shape::Triangle(triangle) => triangle.contains_point(point),
        }
    }
}

#[derive(SpacetimeType, Clone, Debug)]
pub struct TransitionZone {
    pub from_map: String,
    pub area: Area,
    pub destination: Coords,
}

#[derive(SpacetimeType, Clone, Debug)]
pub struct Map {
    pub id: String,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub entities: Vec<u32>,
    pub areas: Vec<Area>,
    pub transition_zones: Vec<TransitionZone>,
}
