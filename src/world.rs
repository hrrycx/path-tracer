use crate::hittable;
use crate::material;
use crate::ray;
use crate::ray::Ray;
use crate::utility;
use crate::vec3::{Color, Point3, Vec3};
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn hittable::Hittable + Send + Sync + 'static>>,
}
impl HittableList {
    pub fn push(&mut self, item: impl hittable::Hittable + 'static) {
        self.objects.push(Arc::new(item));
    }
    pub fn clear(mut self) {
        self.objects.clear();
    }
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut ray::HitRecord) -> bool {
        let mut hit_anything: bool = false;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }
        return hit_anything;
    }
}

pub fn randomscene() -> HittableList {
    let mut world: HittableList = HittableList {
        objects: Vec::new(),
    };
    let matground = material::Diffuse {
        albedo: Color { e: [0.5, 0.5, 0.5] },
    };
    world.push(hittable::Sphere {
        center: Vec3 {
            e: [0., -1000., -1.],
        },
        radius: 1000.,
        mat: Arc::new(matground),
    });
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utility::randdouble();
            let center: Point3 = Point3 {
                e: [
                    a as f64 + 0.9 * utility::randdouble(),
                    0.2,
                    b as f64 + 0.9 * utility::randdouble(),
                ],
            };
            if choose_mat < 0.8 {
                let mat = material::Diffuse {
                    albedo: &utility::randvec(0., 1.) * &utility::randvec(0., 1.),
                };
                world.push(hittable::Sphere {
                    center: center,
                    radius: 0.2,
                    mat: Arc::new(mat),
                });
            } else if choose_mat < 0.95 {
                let mat = material::Metal {
                    albedo: utility::randvec(0.5, 1.),
                    fuzz: utility::randdoublerange(0., 0.5),
                };
                world.push(hittable::Sphere {
                    center: center,
                    radius: 0.2,
                    mat: Arc::new(mat),
                });
            } else {
                let mat = material::Dielectric { ir: 1.5 };
                world.push(hittable::Sphere {
                    center: center,
                    radius: 0.2,
                    mat: Arc::new(mat),
                });
            }
        }
    }
    let mat1 = material::Dielectric { ir: 1.5 };
    let mat2 = material::Diffuse {
        albedo: Color { e: [0.1, 0.1, 0.1] },
    };
    let mat3 = material::Metal {
        albedo: Color { e: [0.7, 0.6, 0.5] },
        fuzz: 0.05,
    };
    world.push(hittable::Sphere {
        center: Vec3 { e: [0., 1., 0.] },
        radius: 1.,
        mat: Arc::new(mat1),
    });
    world.push(hittable::Sphere {
        center: Vec3 { e: [-4., 1., 0.] },
        radius: 1.,
        mat: Arc::new(mat2),
    });
    world.push(hittable::Sphere {
        center: Vec3 { e: [4., 1., 0.] },
        radius: 1.,
        mat: Arc::new(mat3),
    });

    world
}
pub fn notrandomscene() -> HittableList {
    let mut world: HittableList = HittableList {
        objects: Vec::new(),
    };
    let matground = material::Diffuse {
        albedo: Color { e: [0.5, 0.5, 0.5] },
    };
    world.push(hittable::Sphere {
        center: Vec3 {
            e: [0., -1000., -1.],
        },
        radius: 1000.,
        mat: Arc::new(matground),
    });
    let mat1 = material::Dielectric { ir: 1.5 };
    let mat2 = material::Diffuse {
        albedo: Color { e: [0.1, 0.1, 0.1] },
    };
    let mat3 = material::Metal {
        albedo: Color { e: [0.7, 0.6, 0.5] },
        fuzz: 0.05,
    };
    world.push(hittable::Sphere {
        center: Vec3 { e: [0., 1., 0.] },
        radius: 1.,
        mat: Arc::new(mat1),
    });
    world.push(hittable::Sphere {
        center: Vec3 { e: [-4., 1., 0.] },
        radius: 1.,
        mat: Arc::new(mat2),
    });
    world.push(hittable::Sphere {
        center: Vec3 { e: [4., 1., 0.] },
        radius: 1.,
        mat: Arc::new(mat3),
    });
    world
}
