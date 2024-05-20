use std::clone;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Ray {
    pub origin: Point,
    pub direction: Point,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Shape {
    pub color: Color,
    pub name: String,
}

pub trait RayIntersectable {
    fn intersect_ray(&self, ray: &Ray) -> Option<Point>;
}

pub struct Sphere {
    pub shape: Shape,
    pub center: Point,
    pub radius: f64,
}

impl RayIntersectable for Sphere {
    fn intersect_ray(&self, ray: &Ray) -> Option<Point> {
        let oc = Point {
            x: ray.origin.x - self.center.x,
            y: ray.origin.y - self.center.y,
            z: ray.origin.z - self.center.z,
        };

        let a = ray.direction.x * ray.direction.x
            + ray.direction.y * ray.direction.y
            + ray.direction.z * ray.direction.z;
        let b = 2.0 * (oc.x * ray.direction.x + oc.y * ray.direction.y + oc.z * ray.direction.z);
        let c = oc.x * oc.x + oc.y * oc.y + oc.z * oc.z - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        let point = Point {
            x: ray.origin.x + t * ray.direction.x,
            y: ray.origin.y + t * ray.direction.y,
            z: ray.origin.z + t * ray.direction.z,
        };

        Some(point)
    }
}
