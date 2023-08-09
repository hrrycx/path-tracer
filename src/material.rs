use crate::ray;
use crate::ray::Ray;
use crate::utility;
use crate::vec3;
use crate::vec3::{Color, Vec3};

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &ray::HitRecord,
        attentuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
pub struct Diffuse {
    pub albedo: Color,
}
impl Material for Diffuse {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &ray::HitRecord,
        attentuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = &rec.normal + &utility::randunit();
        if scatter_direction.nearzero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray {
            origin: rec.p,
            direction: scatter_direction,
        };
        *attentuation = self.albedo;
        true
    }
}
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &ray::HitRecord,
        attentuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = vec3::reflect(&vec3::unit(&r_in.direction), &rec.normal);
        *scattered = Ray {
            origin: rec.p,
            direction: &reflected + &(self.fuzz * &utility::randinsphere()),
        };
        *attentuation = self.albedo;
        let x = vec3::dot(&scattered.direction, &rec.normal);
        return x > 0.;
    }
}
pub struct Dielectric {
    pub ir: f64,
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &ray::HitRecord,
        attentuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attentuation = Color { e: [1., 1., 1.] };
        let refractratio: f64;
        if rec.front_face {
            refractratio = 1. / self.ir
        } else {
            refractratio = self.ir
        }
        let costheta = vec3::dot(&-&vec3::unit(&r_in.direction), &rec.normal).min(1.0);
        let sintheta = (1. - (costheta * costheta)).sqrt();
        let cannotrefract = refractratio * sintheta > 1.;
        let direction: Vec3;
        if cannotrefract || reflectance(costheta, refractratio) > utility::randdouble() {
            direction = vec3::reflect(&vec3::unit(&r_in.direction), &rec.normal)
        } else {
            direction = refract(&vec3::unit(&r_in.direction), &rec.normal, refractratio);
        }
        *scattered = Ray {
            origin: rec.p,
            direction: direction,
        };
        true
    }
}
fn reflectance(cos: f64, refractratio: f64) -> f64 {
    let mut r0 = (1. - refractratio) / (1. + refractratio);
    r0 = r0 * r0;
    return r0 + (1. - r0) * (1. - cos).powi(5);
}
fn refract(u: &Vec3, norm: &Vec3, ir: f64) -> Vec3 {
    let costheta = vec3::dot(&-u, norm).min(1.0);
    let perp: Vec3 = ir * &(u + &(costheta * norm));
    let par: Vec3 = -(((1.0 - perp.lengthsquared()).abs()).sqrt()) * norm;
    &perp + &par
}
