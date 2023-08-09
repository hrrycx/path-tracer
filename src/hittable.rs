use crate::material;
use crate::ray;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut ray::HitRecord) -> bool;
}
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Arc<dyn material::Material + Sync + Send + 'static>,
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut ray::HitRecord) -> bool {
        let oc: Vec3 = &r.origin - &self.center;
        let a = r.direction.lengthsquared();
        let hb = vec3::dot(&oc, &r.direction);
        let c = oc.lengthsquared() - self.radius * self.radius;
        let discriminant = hb * hb - a * c;
        if discriminant < 0. {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-hb - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-hb + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = &(&rec.p - &self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();
        true
    }
}
