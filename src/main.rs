mod camera;
mod hittable;
mod material;
mod ray;
mod utility;
mod vec3;
mod world;

use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

use indicatif::ParallelProgressIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::time::Instant;

const SAMPLES_MAX: i32 = 1000;
const MAX_DEPTH: i32 = 25;
const WIDTH: i32 = 400;
const ASPECT_RATIO: f64 = 16. / 9.;
const HEIGHT: i32 = (WIDTH as f64 / ASPECT_RATIO) as i32;

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(12)
        .build_global()
        .unwrap();
    let now = Instant::now();

    let world: world::HittableList = world::notrandomscene();
    let lookfrom: Vec3 = Vec3 { e: [13., 2., 3.] };
    let lookat: Vec3 = Vec3 { e: [0., 0., 0.] };
    let upward: Vec3 = Vec3 { e: [0., 1., 0.] };
    //let dist_to_focus = (&lookfrom - &lookat).length();
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let cam = camera::set_camera(
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        lookfrom,
        lookat,
        upward,
        20.,
    );
    let buf: Vec<_> = (0..HEIGHT)
        .into_par_iter()
        .progress_count(HEIGHT as u64)
        .map(|j| renderline(&cam, &world, HEIGHT - j))
        .flatten()
        .collect();
    image::save_buffer(
        format!("C:/Users/itsmr/Desktop/rust safe space/output-folder/render.png"),
        &buf,
        WIDTH as u32,
        HEIGHT as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();
    println!("done");
    println!("{}", now.elapsed().as_millis());
}
fn write_colour(c: &Color) -> [u8; 3] {
    let mut r = c.x();
    let mut g = c.y();
    let mut b = c.z();
    let scale = 1.0 / SAMPLES_MAX as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    return [
        (256. * utility::clamp(r, 0., 0.999)) as u8,
        (256. * utility::clamp(g, 0., 0.999)) as u8,
        (256. * utility::clamp(b, 0., 0.999)) as u8,
    ];
}
fn renderline(cam: &camera::Camera, world: &world::HittableList, j: i32) -> Vec<u8> {
    let mut line: Vec<u8> = Vec::new();
    for i in 0..(WIDTH) {
        let mut pixelcolor: Color = Color { e: [0., 0., 0.] };
        for _s in 0..SAMPLES_MAX {
            let u = (i as f64 + utility::randdouble()) / (WIDTH as f64 - 1.0);
            let v = (j as f64 + utility::randdouble()) / (HEIGHT as f64 - 1.0);

            let r: Ray = cam.get_ray(u, v);
            pixelcolor += ray::raycolour(&r, &world, MAX_DEPTH);
        }
        line.extend_from_slice(&write_colour(&pixelcolor));
    }
    return line;
}
