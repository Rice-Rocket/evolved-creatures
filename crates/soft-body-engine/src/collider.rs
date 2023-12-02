use bevy::prelude::*;


pub trait Collider {
    fn exit_vector(&self, pos: Vec3) -> Option<Vec3>;
}

#[derive(Component, Reflect, Debug, Default, Clone)]
#[reflect(Debug, Default)]
pub struct ColliderProperties {
    pub elasticity: f32,
    pub friction: f32,
    pub restitution: f32,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Debug, Default)]
pub struct HalfSpace {
    pub normal: Vec3,
    pub k: f32,
}

impl Collider for HalfSpace {
    fn exit_vector(&self, pos: Vec3) -> Option<Vec3> {
        let to_pos = pos - self.normal * self.k;
        let dist = to_pos.dot(self.normal);
        if dist > 0.0 {
            return None;
        }
        return Some(-dist * self.normal);
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Debug, Default)]
pub struct StaticPolygon {
    pub vertices: Vec<Vec3>,
}

impl Collider for StaticPolygon {
    fn exit_vector(&self, pos: Vec3) -> Option<Vec3> {
        let mut collision = false;
        let mut closest_hit: Option<(f32, Vec3)> = None;

        for i in 0..self.vertices.len() {
            let j = (i + 1) % self.vertices.len();

            let v1 = self.vertices[i];
            let v2 = self.vertices[j];

            let to_pos = pos - v1;
            let to_v2 = v2 - v1;
            
            let dist = to_pos.dot(to_v2) / to_v2.length_squared();
            let point = v1 + to_v2 * dist;
            let dist_to_point = (point - pos).length_squared();

            if let Some(closest) = closest_hit {
                if closest.0 > dist_to_point {
                    closest_hit = Some((dist_to_point, point));
                }
            } else {
                closest_hit = Some((dist_to_point, point));
            }
            
            if ((v1.y > pos.y) != (v2.y > pos.y))
            && (pos.x < (v2.x - v1.x) * (pos.y - v1.y) / (v2.y - v1.y) + v1.x) {
                collision = !collision;
            }
        };

        if !collision { return None };

        return Some(closest_hit.unwrap().1 - pos);
    }
}

impl StaticPolygon {
    pub fn new_square() -> Self {
        Self {
            vertices: vec![
                Vec3::new(-0.5, -0.5, 0.0), Vec3::new(0.5, -0.5, 0.0), 
                Vec3::new(0.5, 0.5, 0.0), Vec3::new(-0.5, 0.5, 0.0)
            ]
        }
    }
    pub fn from_vertices(vertices: Vec<Vec3>) -> Self {
        Self {
            vertices
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        let transform_mat = transform.compute_matrix();
        for vertex in self.vertices.iter_mut() {
            *vertex = (transform_mat * Vec4::new(vertex.x, vertex.y, vertex.z, 1.0)).xyz();
        }

        self
    }
}