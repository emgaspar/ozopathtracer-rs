mod vec;
mod ray;

use vec::{Vec3, Point3, Color};
use ray::Ray;

use image::{ImageBuffer, RgbImage};

fn main() {
    // Image
    let image_width: u32 = 256;
    let aspect_ratio: f64 = 16.0 / 9.0;

    // Calculate the image height, and ensure that it's at least 1
    let mut image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    image_height = if image_height < 1 { 1 } else { image_height };
    
    // Image creation
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: Point3= Point3::new(0.0, 0.0, 0.0);

    // Calculate the vector accross the horizontal and down the vertical viewport edges
    let viewport_u: Vec3 = Vec3::new(viewport_width as f64, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height as f64 , 0.0);

    // Calculate the horizontal and vertical delta vecgor from pixel to pixel
    let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
    let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left: Vec3 = 
        camera_center - Vec3::new(0.0, 0.0, focal_length) 
        - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc: Vec3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);


    for y in 0..image_height {
        for x in 0..image_width {
            let pixel_center : Vec3 = 
                pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
            let ray_direction: Vec3 = pixel_center - camera_center;

            let ray: Ray = Ray::new(pixel_center, ray_direction);

            let color = ray_color(&ray);
            img.put_pixel(
                x, 
                y, 
                image::Rgb([(color.x * 255.99) as u8, (color.y * 255.99) as u8, (color.z * 255.99) as u8]));
        }
    }
    img.save("test.png").unwrap();
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction: Vec3 = ray.dir().normalize();
    let a: f64 = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc: Vec3 = ray.origin() - *center;
    let a: f64 = vec::dot(ray.dir(), ray.dir());
    let b: f64 = 2.0 * vec::dot(oc, ray.dir());
    let c: f64 = vec::dot(oc, oc) - radius * radius;
    let discriminant: f64 = b * b - 4.0 * a * c;
    discriminant >= 0.0
}
