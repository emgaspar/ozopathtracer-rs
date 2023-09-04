mod vec;
mod ray;
mod hit;
mod sphere;

use std::f64::INFINITY;

use vec::{Vec3, Point3, Color};
use ray::Ray;
use hit::{HitableList, Hitable};
use image::{ImageBuffer, RgbImage};
use sphere::Sphere;
	
// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 512;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

// Camera
const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
const CAMERA_CENTER: Point3= Point3::new(0.0, 0.0, 0.0);

fn main() {
    
    // Image creation
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // Calculate the vector accross the horizontal and down the vertical viewport edges
    let viewport_u: Vec3 = Vec3::new(VIEWPORT_WIDTH as f64, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -VIEWPORT_HEIGHT as f64 , 0.0);

    // Calculate the horizontal and vertical delta vecgor from pixel to pixel
    let pixel_delta_u: Vec3 = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v: Vec3 = viewport_v / IMAGE_HEIGHT as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left: Vec3 = 
        CAMERA_CENTER - Vec3::new(0.0, 0.0, FOCAL_LENGTH) 
        - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc: Vec3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Scene configuration
    let scene: HitableList = 
        HitableList::new(vec![
           Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
           Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0))
        ]);

    print!("Rendering");
    for y in 0..IMAGE_HEIGHT {
        print!(".");
        for x in 0..IMAGE_WIDTH {
            let pixel_center : Vec3 = 
                pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
            let ray_direction: Vec3 = (pixel_center - CAMERA_CENTER).normalize();

            let ray: Ray = Ray::new(CAMERA_CENTER, ray_direction);

            let color = ray_color(&ray, &scene);
            img.put_pixel(
                x, 
                y, 
                image::Rgb([(color.x * 255.99) as u8, (color.y * 255.99) as u8, (color.z * 255.99) as u8]));
        }
    }
    img.save("test.png").unwrap();
    println!(" Completed")
}

fn ray_color(ray: &Ray, world: &HitableList) -> Color {

    match world.hit(ray, 0.0, INFINITY) {
        Some(hit) => {
            0.5 * (hit.normal + Color::new(1.0, 1.0, 1.0))
        },
        None => {
            let unit_direction: Vec3 = ray.dir().normalize();
            let a: f64 = 0.5 * (unit_direction.y + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }
}
