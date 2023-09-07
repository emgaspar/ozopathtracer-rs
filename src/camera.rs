
use super::random::random_f64;
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
	samples_per_pixel: u32, // Count of random samples for each pixel
	max_depth: u32,			// Maximum number of ray bounces into scene
}

impl Camera {
	pub fn new(image_width: u32, aspect_ratio: f64, samples_per_pixel: u32, max_depth: u32) -> Camera {
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
			pixel_delta_v: pixel_delta_v,
			samples_per_pixel: samples_per_pixel,
			max_depth: max_depth
		}
	}

	pub fn render(&self, world: &HitableList, image_filename: &str) {
		  
	    // Image creation
 	   let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);

		print!("Rendering");
		for y in 0..self.image_height {
			print!(".");
			for x in 0..self.image_width {
				let mut pixel_color: Vec3 = Vec3::zeros();
				for _sample in 0..self.samples_per_pixel {
					let ray: Ray = self.get_ray(x, y);
					pixel_color += self.ray_color(&ray, self.max_depth, world);
				}
				
				// Write the final color
				pixel_color = (pixel_color * (1.0 / self.samples_per_pixel as f64))
					.sqrt()
					.clamp(0.0, 0.999); 

				img.put_pixel(
					x, 
					y, 
					image::Rgb([(pixel_color.x * 255.99) as u8, (pixel_color.y * 255.99) as u8, (pixel_color.z * 255.99) as u8]));
			}
		}
		img.save(image_filename).unwrap();
		println!(" Completed")
	}

	fn get_ray(&self, x: u32, y: u32) -> Ray
	{
		let pixel_center : Vec3 = 
			self.pixel00_loc + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v);
		let pixel_sample: Vec3 = pixel_center + self.pixel_sample_square();
		
		let ray_origin: Vec3 = self.center;
		let ray_direction:Vec3 = pixel_sample - ray_origin;

		Ray::new(ray_origin, ray_direction)
	}

	fn pixel_sample_square(&self) -> Vec3 {
		let px: f64 = -0.5 + random_f64();
		let py: f64 = -0.5 + random_f64();
		(px * self.pixel_delta_u) + (py * self.pixel_delta_v)
	}

	fn ray_color(&self, ray: &Ray, depth: u32, world: &HitableList) -> Color {

		// If we've exceeded the ray bounce limit, no more light is gathering
		if depth > 0 {
			match world.hit(ray, 0.001, INFINITY) {
				Some(hit) => {
					match hit.material.scatter(ray, &hit) {
						Some(ray_interaction) => {
							ray_interaction.attenuation() * self.ray_color(&ray_interaction.scattered(), depth - 1, world)
						},
						None => {
							Color::zeros()
						}
					}
				},
				None => {
					let unit_direction: Vec3 = ray.dir().normalize();
					let a: f64 = 0.5 * (unit_direction.y + 1.0);
					(1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
				}
			}
		} else {
			Color::ones()
		}
	}	
}