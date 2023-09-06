mod random;
mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;

use hit::HitableList;
use sphere::Sphere;
use camera::Camera;
use vec::Point3;
	
const ASPECT_RATIO: f64 = 16.0 / 9.0;   // Ratio of image width over height
const IMAGE_WIDTH: u32 = 512;           // Rendered image width in pixel count
const SAMPLES_PER_PIXEL: u32 = 100;     // Count of random samples for each pixel
const MAX_DEPTH: u32 = 50;             // Maximum number of ray bounces into scene

fn main() {
    use std::time::Instant;
    
    let camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO, SAMPLES_PER_PIXEL, MAX_DEPTH);

    // Scene configuration
    let world: HitableList = 
        HitableList::new(vec![
           Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
           Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0))
        ]);

    let now = Instant::now();
    camera.render(&world);
    let elapsed = now.elapsed();
    println!("Elapsed {:?}", elapsed)
}
