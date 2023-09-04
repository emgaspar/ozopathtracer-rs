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


fn main() {
    
    let camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO);

    // Scene configuration
    let world: HitableList = 
        HitableList::new(vec![
           Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
           Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0))
        ]);

    camera.render(&world);
}
