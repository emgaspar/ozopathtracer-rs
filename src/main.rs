mod random;
mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;
mod material;

use std::rc::Rc;

use hit::HitableList;
use sphere::Sphere;
use camera::Camera;
use vec::{Point3, Color};
use material::{Lambertian, Metal};
	
const ASPECT_RATIO: f64 = 16.0 / 9.0;           // Ratio of image width over height
const IMAGE_WIDTH: u32 = 512;                   // Rendered image width in pixel count
const SAMPLES_PER_PIXEL: u32 = 100;             // Count of random samples for each pixel
const MAX_DEPTH: u32 = 50;                      // Maximum number of ray bounces into scene
const IMAGE_FILENAME: &str = "test.png";        // Image filename

fn main() {
    use std::time::Instant;
    
    let camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO, SAMPLES_PER_PIXEL, MAX_DEPTH);

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

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
