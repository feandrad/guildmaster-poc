use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Clone, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(SpacetimeType, Clone, Debug)]
pub enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle),
}

pub trait IShape {
    fn contains_point(&self, point: &Vector2) -> bool;
    // You can add more methods as needed, e.g. area, perimeter, etc.
}

#[derive(SpacetimeType, Clone, Debug)]
pub struct Rectangle {
    pub origin: Vector2, // bottom-left corner
    pub width: f32,
    pub height: f32,
}

impl IShape for Rectangle {
    fn contains_point(&self, point: &Vector2) -> bool {
        point.x >= self.origin.x && point.x <= self.origin.x + self.width &&
        point.y >= self.origin.y && point.y <= self.origin.y + self.height
    }
}

#[derive(SpacetimeType, Clone, Debug)]
pub struct Circle {
    pub center: Vector2,
    pub radius: f32,
}

impl IShape for Circle {
    fn contains_point(&self, point: &Vector2) -> bool {
        let dx = point.x - self.center.x;
        let dy = point.y - self.center.y;
        (dx * dx + dy * dy) <= self.radius * self.radius
    }
}

#[derive(SpacetimeType, Clone, Debug)]
pub struct Triangle {
    pub a: Vector2,
    pub b: Vector2,
    pub c: Vector2,
}

impl IShape for Triangle {
    fn contains_point(&self, point: &Vector2) -> bool {
        barycentric_contains(&self.a, &self.b, &self.c, point)
    }
}

fn barycentric_contains(a: &Vector2, b: &Vector2, c: &Vector2, p: &Vector2) -> bool {
    let v0 = Vector2 { x: c.x - a.x, y: c.y - a.y };
    let v1 = Vector2 { x: b.x - a.x, y: b.y - a.y };
    let v2 = Vector2 { x: p.x - a.x, y: p.y - a.y };
    let dot00 = v0.x * v0.x + v0.y * v0.y;
    let dot01 = v0.x * v1.x + v0.y * v1.y;
    let dot02 = v0.x * v2.x + v0.y * v2.y;
    let dot11 = v1.x * v1.x + v1.y * v1.y;
    let dot12 = v1.x * v2.x + v1.y * v2.y;
    let denom = dot00 * dot11 - dot01 * dot01;
    if denom == 0.0 { return false; }
    let inv_denom = 1.0 / denom;
    let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
    let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;
    (u >= 0.0) && (v >= 0.0) && (u + v < 1.0)
}
