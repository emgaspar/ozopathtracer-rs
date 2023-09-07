mod lambertian;
mod metal;
mod dielectric;

use crate::{
	color::Color,
	ray::Ray,
	hit::HitRecord
};

pub struct MaterialRayInteraction {
	attenuation: Color,
	scattered: Ray,
}

impl MaterialRayInteraction {
	pub fn new(attenuation: Color, scattered: Ray) -> MaterialRayInteraction {
		MaterialRayInteraction { attenuation: attenuation, scattered: scattered }
	}

	pub fn attenuation(&self) -> Color {
		self.attenuation	
	}

	pub fn scattered(&self) -> Ray {
		self.scattered
	}
}

pub trait Material {
	fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<MaterialRayInteraction>;
}

pub use lambertian::Lambertian;
pub use metal::Metal;
pub use dielectric::Dielectric;