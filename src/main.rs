mod random;
mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;

use hit::{HitableList};
use sphere::Sphere;
use camera::Camera;
use vec::Point3;
	
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 512;
const SAMPLES_PER_PIXEL: u32 = 10;

fn main() {
    use std::time::Instant;
    
    let camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO, SAMPLES_PER_PIXEL);

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
