
use super::vec::{Color, Vec3};
use super::hit::{Hitable, HitableList};
use super::ray::Ray;

use image::{ImageBuffer, RgbImage};

use std::f64::INFINITY;

pub struct Camera {
	image_width: u32,		// Rendered image width in pixel count
	image_height: u32,  	// Rendered image height in pixel count
	aspect_ratio: f64,		// Ratio of image width over height
	center: Vec3,			// Camera center
	pixel00_loc: Vec3,		// Location of pixel 0, 0
	pixel_delta_u: Vec3,	// Offset to pixel to the right
	pixel_delta_v: Vec3,	// Offset to pixel below
}

impl Camera {
	pub fn new(image_width: u32, aspect_ratio: f64) -> Camera {
		let mut image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
		image_height = if image_height < 1 { 1 } else { image_height };

		let camera_center: Vec3 = Vec3::new(0.0, 0.0, 0.0);

		// Determinate viewport dimensions
		let focal_length: f64 = 1.0;
		let viewport_height: f64 = 2.0;
		let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

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

		Camera {
			image_width: image_width,
			image_height: image_height,
			aspect_ratio: aspect_ratio,
			center: camera_center,
			pixel00_loc: viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v),
			pixel_delta_u: pixel_delta_u,
			pixel_delta_v: pixel_delta_v
		}
	}

	pub fn render(&self, world: &HitableList) {
		  
	    // Image creation
 	   let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);

		print!("Rendering");
		for y in 0..self.image_height {
			print!(".");
			for x in 0..self.image_width {
				let pixel_center : Vec3 = 
					self.pixel00_loc + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v);
				let ray_direction: Vec3 = (pixel_center - self.center).normalize();

				let ray: Ray = Ray::new(self.center, ray_direction);

				let color = self.ray_color(&ray, world);
				img.put_pixel(
					x, 
					y, 
					image::Rgb([(color.x * 255.99) as u8, (color.y * 255.99) as u8, (color.z * 255.99) as u8]));
			}
		}
		img.save("test.png").unwrap();
		println!(" Completed")
	}

	fn ray_color(&self, ray: &Ray, world: &HitableList) -> Color {

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
	
}