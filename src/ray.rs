use crate::material;
use crate::vec3;
use crate::vec3::{Color, Point3, Vec3};
use crate::world;
use std::sync::Arc;
const INFINITY: f64 = f64::INFINITY;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}
impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        return &self.origin + &(t * &self.direction);
    }
    pub fn write(&self) {
        println!("origin: {:?} direction: {:?}", self.origin, self.direction)
    }
}
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn material::Material + Sync + Send + 'static>,
    pub t: f64,
    pub front_face: bool,
}
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = vec3::dot(&r.direction, &outward_normal) < 0.;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}
pub fn raycolour(r: &Ray, world: &world::HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color { e: [0., 0., 0.] };
    }
    let mut rec: HitRecord = HitRecord {
        p: Vec3 { e: [0., 0., 0.] },
        normal: Vec3 { e: [0., 0., 0.] },
        mat: Arc::new(material::Diffuse {
            albedo: Color { e: [0., 0., 0.] },
        }),
        t: 0.,
        front_face: true,
    };
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered: Ray = Ray {
            origin: Vec3 { e: [0., 0., 0.] },
            direction: Vec3 { e: [0., 0., 0.] },
        };
        let mut attenuation: Color = Color { e: [0., 0., 0.] };
        if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return &attenuation * &raycolour(&scattered, world, depth - 1);
        }
        return Color { e: [0., 0., 0.] };
    }
    let unit_direction = vec3::unit(&r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    return &((1.0 - t) * &Color { e: [1.0, 1.0, 1.0] }) + &(t * &Color { e: [0.5, 0.7, 1.0] });
}
