mod random;
mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;
mod material;
mod color;

use std::{rc::Rc, f64::consts::PI};

use hit::HitableList;
use sphere::Sphere;
use camera::Camera;
use vec::{Point3, Vec3};
use color::Color;
use material::{Lambertian, Metal, Dielectric};
	
const IMAGE_FILENAME: &str = "test.png";        // Image filename

const ASPECT_RATIO: f64 = 16.0 / 9.0;           // Ratio of image width over height
const IMAGE_WIDTH: u32 = 512;                   // Rendered image width in pixel count
const SAMPLES_PER_PIXEL: u32 = 100;             // Count of random samples for each pixel
const MAX_DEPTH: u32 = 50;                      // Maximum number of ray bounces into scene

const VFOV: f64 = 90.0;                         // Vertical FOV
const LOOK_FROM: Point3 = Point3::new(-2.0, 2.0, 1.0);
const LOOK_AT: Point3 = Point3::new(0.0, 0.0, -1.0);
const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);

fn main() {
    use std::time::Instant;
    
    let camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO, VFOV, SAMPLES_PER_PIXEL, MAX_DEPTH, LOOK_FROM, LOOK_AT, VUP);
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_left_2 = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    // Scene configuration
    let world: HitableList = 
        HitableList::new(vec![
            Rc::new(
                Sphere::new(
                    Point3::new(0.0, 0.0, -1.0), 
                    0.5, 
                    material_center
                )
            ),
            Rc::new(
                Sphere::new(
                    Point3::new(-1.0, 0.0, -1.0),
                    0.5,
                    material_left
                )
            ),
            Rc::new(
                Sphere::new(
                    Point3::new(-1.0, 0.0, -1.0),
                    -0.4,
                    material_left_2
                )
            ),
            Rc::new(
                Sphere::new(
                    Point3::new(1.0, 0.0, -1.0),
                    0.5,
                    material_right
                )
            ),
            Rc::new(
                Sphere::new(
                    Point3::new(0.0, -100.5, -1.0),
                    100.0,
                    material_ground
                )
            ),
        ]);

    let now = Instant::now();
    camera.render(&world, IMAGE_FILENAME);
    let elapsed = now.elapsed();
    println!("Elapsed {:?}", elapsed)
}
