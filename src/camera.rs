
use crate::{
	random::random_f64,
	vec::{Vec3, Point3, cross},
	color::{Color, BLACK, WHITE},
	hit::{Hitable, HitableList},
	ray::Ray
};

use image::{ImageBuffer, RgbImage};

use std::f64::INFINITY;

pub struct Camera {
	image_width: u32,		// Rendered image width in pixel count
	image_height: u32,  	// Rendered image height in pixel count
	aspect_ratio: f64,		// Ratio of image width over height
	vfov: f64,				// Vertical view angel (FOV)
	center: Vec3,			// Camera center
	pixel00_loc: Vec3,		// Location of pixel 0, 0
	pixel_delta_u: Vec3,	// Offset to pixel to the right
	pixel_delta_v: Vec3,	// Offset to pixel below
	samples_per_pixel: u32, // Count of random samples for each pixel
	max_depth: u32,			// Maximum number of ray bounces into scene
	u: Vec3,				// Camera frame basis vectors
	v: Vec3,
	w: Vec3,
	defocus_disk_u: Vec3,	// Defocus disk horizontal radius
	defocus_disk_v: Vec3,	// Defocus disk horizontal radius
	defocus_angle: f64,		// Variation angle of rays through each pixel
}

impl Camera {
	pub fn new(image_width: u32, aspect_ratio: f64, vfov: f64, samples_per_pixel: u32, max_depth: u32, look_from: Point3, look_at: Point3, vup: Vec3, defocus_angle: f64, focus_dist: f64) -> Camera {
		let mut image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
		image_height = if image_height < 1 { 1 } else { image_height };

		let camera_center: Vec3 = look_from;

		// Determinate viewport dimensions
		let theta: f64 = vfov.to_radians();
		let h: f64 = (theta / 2.0).tan();
		let viewport_height: f64 = 2.0 * h * focus_dist;
		let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

		// Calculate the u,v,w unit basis vectors for the camera coordinate frame
		let w: Vec3 = (look_from - look_at).unit_vector();
		let u: Vec3 = cross(vup, w).unit_vector();
		let v: Vec3 = cross(w, u);

		// Calculate the vector accross the horizontal and down the vertical viewport edges
		let viewport_u: Vec3 = viewport_width as f64 * u;	// Vector across viewport horizontal edge
		let viewport_v: Vec3 = viewport_height * -v;		// Vector down viewport vertical edge

		// Calculate the horizontal and vertical delta vecgor from pixel to pixel
		let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
		let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

		// Calculate the location of the upper left pixel
		let viewport_upper_left: Vec3 = 
			camera_center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;

		// Calculate the camera defocus disk basis vectors.
		let defocus_radius: f64 = focus_dist * (defocus_angle / 2.0).to_radians().tan();
		let defocus_disk_u: Vec3 = u * defocus_radius;
		let defocus_disk_v: Vec3 = v * defocus_radius;

		Camera {
			image_width: image_width,
			image_height: image_height,
			aspect_ratio: aspect_ratio,
			vfov: vfov,
			center: camera_center,
			pixel00_loc: viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v),
			pixel_delta_u: pixel_delta_u,
			pixel_delta_v: pixel_delta_v,
			samples_per_pixel: samples_per_pixel,
			max_depth: max_depth,
			u: u,
			v: v,
			w: w,
			defocus_angle: defocus_angle,
			defocus_disk_u: defocus_disk_u,
			defocus_disk_v: defocus_disk_v
		}
	}

	pub fn render(&self, world: &HitableList, image_filename: &str) {
		  
	    // Image creation
 	   let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);

		print!("Rendering");
		for y in 0..self.image_height {
		//for y in self.image_height / 2..=self.image_height / 2 {
			print!(".");
			for x in 0..self.image_width {
				let mut pixel_color: Vec3 = Vec3::zeros();
				for _sample in 0..self.samples_per_pixel {
					let ray: Ray = self.get_ray(x, y);
					pixel_color += self.ray_color(&ray, world, self.max_depth);
				}
				
				// Write the final color
				pixel_color = (pixel_color * (1.0 / self.samples_per_pixel as f64))
					.sqrt()
					.clamp(0.0, 0.999); 

//				println!("[{}, {}] -> {}", x, y, pixel_color);

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
		// Get a randomly-sampled camera ray for the pixel at location i,j, originating from
        // the camera defocus disk.
		let pixel_center : Vec3 = 
			self.pixel00_loc + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v);
		let pixel_sample: Vec3 = pixel_center + self.pixel_sample_square();
		
		let ray_origin: Vec3 = if self.defocus_angle <= 0.0 {
				self.center
			} 
			else {
				self.defocus_disk_sample()
			};
		let ray_direction:Vec3 = pixel_sample - ray_origin;

		Ray::new(ray_origin, ray_direction)
	}

	fn pixel_sample_square(&self) -> Vec3 {
		let px: f64 = -0.5 + random_f64();
		let py: f64 = -0.5 + random_f64();
		(px * self.pixel_delta_u) + (py * self.pixel_delta_v)
	}

	fn ray_color(&self, ray: &Ray, world: &HitableList, depth: u32) -> Color {

		// If we've exceeded the ray bounce limit, no more light is gathering
		if depth > 0 {
			match world.hit(ray, 0.001, INFINITY) {
				Some(hit) => {
					match hit.material.scatter(ray, &hit) {
						Some(ray_interaction) => {
							ray_interaction.attenuation() * self.ray_color(&ray_interaction.scattered(), world, depth - 1)
						},
						None => {
							BLACK
						}
					}
				},
				None => {
					let unit_direction: Vec3 = ray.dir().unit_vector();
					let a: f64 = 0.5 * (unit_direction.y + 1.0);
					(1.0 - a) * WHITE + a * Color::new(0.5, 0.7, 1.0)
				}
			}
		} else {
			// If we've exceeded the ray bounce limit, no more light is gathered
			WHITE
		}
	}	

	fn defocus_disk_sample(&self) -> Point3 {
		// Returns a random point in the camera defocus disk.
        let p: Point3 = Vec3::random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
	}
}