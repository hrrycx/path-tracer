use crate::vec3;
use crate::vec3::Vec3;
use rand::prelude::*;
const PI: f64 = 3.1415926535897932385;

pub fn degtorad(degrees: f64) -> f64 {
    (degrees * PI) / 180.
}
pub fn randdouble() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
pub fn randdoublerange(min: f64, max: f64) -> f64 {
    min + (max - min) * randdouble()
}
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}
pub fn randvec(min: f64, max: f64) -> Vec3 {
    Vec3 {
        e: [
            randdoublerange(min, max),
            randdoublerange(min, max),
            randdoublerange(min, max),
        ],
    }
}
pub fn randinsphere() -> Vec3 {
    loop {
        let p = randvec(-1., 1.);
        if p.lengthsquared() >= 1. {
            continue;
        }
        return p;
    }
}
pub fn randunit() -> Vec3 {
    vec3::unit(&randinsphere())
}

// this function is bad, find better way
pub fn randinunitdisk() -> Vec3 {
    loop {
        let p = Vec3 {
            e: [randdouble(), randdouble(), 0.],
        };
        if p.lengthsquared() < 1. {
            return p;
        }
    }
}
