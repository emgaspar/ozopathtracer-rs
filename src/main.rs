mod random;
mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;
mod material;
mod color;

use std::rc::Rc;

use hit::HitableList;
use sphere::Sphere;
use camera::Camera;
use vec::{Point3, Vec3};
use color::Color;
use material::{Lambertian, Metal, Dielectric};

use crate::random::{random_f64, random_f64_range};
	
const IMAGE_FILENAME: &str = "test.png";        // Image filename

const ASPECT_RATIO: f64 = 16.0 / 9.0;           // Ratio of image width over height
const IMAGE_WIDTH: u32 = 1200;                  // Rendered image width in pixel count
const SAMPLES_PER_PIXEL: u32 = 100;             // Count of random samples for each pixel
const MAX_DEPTH: u32 = 50;                      // Maximum number of ray bounces into scene

const VFOV: f64 = 20.0;                         // Vertical FOV
const LOOK_FROM: Point3 = Point3::new(13.0, 2.0, 3.0);
const LOOK_AT: Point3 = Point3::new(0.0, 0.0, -1.0);
const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);

const DEFOCUS_ANGLE: f64 = 0.6;
const FOCUS_DIST: f64 = 10.0;


fn main() {
    use std::time::Instant;
    
    let camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO, VFOV, SAMPLES_PER_PIXEL, MAX_DEPTH, LOOK_FROM, LOOK_AT, VUP, DEFOCUS_ANGLE, FOCUS_DIST);
    
    let material_ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let mut world: HitableList = 
        HitableList::new(vec![
            Rc::new(
                Sphere::new(
                    Point3::new(0.0, -1000.0, 0.0), 
                    1000.0, 
                    material_ground
                )
            )
        ]);

    for a in -11 .. 11 {
        for b in -11 .. 11 {
            let choose_material = random_f64();
            let center: Point3 = Point3::new(a as f64 + 0.9 * random_f64(), 0.2, b as f64 + 0.9 * random_f64());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    world.add(Rc::new(
                        Sphere::new(
                            center,
                            0.2,
                            Rc::new(
                                Lambertian::new(Color::random() * Color::random())
                            )
                        )
                    ));
                }
                else if choose_material < 0.95 {
                    world.add(Rc::new(
                        Sphere::new(
                            center,
                            0.2,
                            Rc::new(
                                Metal::new(
                                    Color::random_range(0.5, 1.0),
                                    random_f64_range(0.0, 0.5)
                                )
                            )
                        )
                    ));
               }
               else {
                    world.add(Rc::new(
                        Sphere::new(
                            center,
                            0.2,
                            Rc::new(
                                Dielectric::new(1.5)
                            )
                        )
                    ));
                }
            }
        }
    }
    
    world.add(Rc::new(
        Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            Rc::new(Dielectric::new(1.5))
        )
    ));

    world.add(Rc::new(
        Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)))
        )
    ));

    world.add(Rc::new(
        Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0))
        )
    ));

    let now = Instant::now();
    camera.render(&world, IMAGE_FILENAME);
    let elapsed = now.elapsed();
    println!("Elapsed {:?}", elapsed)
}
