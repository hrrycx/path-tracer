use crate::ray::Ray;
use crate::utility;
use crate::vec3;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub aspratio: f64,
    pub aperature: f64,
    pub focusdist: f64,
    pub u: Vec3,
    pub v: Vec3,
}
impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = (&self.aperature / 2.) * &utility::randinunitdisk();
        let offset = &(&self.u * rd.x()) + &(&self.v * rd.y());

        return Ray {
            origin: (&self.origin + &offset),
            direction: &(&(&(&self.lower_left_corner + &(u * &self.horizontal))
                + &(v * &self.vertical))
                - &self.origin)
                - &offset,
        };
    }
}
pub fn set_camera(
    aspratio: f64,
    aperature: f64,
    focusdist: f64,
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
    vfov: f64,
) -> Camera {
    let theta = utility::degtorad(vfov);
    let h = (theta / 2.).tan();
    let viewport_height: f64 = 2.0 * h;
    let viewport_width: f64 = aspratio * viewport_height;

    let w = vec3::unit(&(&lookfrom - &lookat));
    let u = vec3::unit(&vec3::cross(&vup, &w));
    let v = vec3::cross(&w, &u);

    let origin = lookfrom;
    let horizontal = focusdist * viewport_width * &u;
    let vertical = focusdist * viewport_height * &v;
    let lower_left_corner =
        &(&(&origin - &(&horizontal / 2.)) - &(&vertical / 2.)) - &(focusdist * &w);
    return Camera {
        origin: origin,
        lower_left_corner: lower_left_corner,
        horizontal: horizontal,
        vertical: vertical,
        aspratio: aspratio,
        aperature: aperature,
        focusdist: focusdist,
        u: u,
        v: v,
    };
}
