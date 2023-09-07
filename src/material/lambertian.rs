use crate::{
	material::{Material, MaterialRayInteraction},
	ray::Ray,
	hit::HitRecord,
	vec::Vec3,
	color::Color
};

pub struct Lambertian {
	albedo: Color,
}

impl Lambertian {
	pub fn new(color: Color) -> Lambertian {
		Lambertian { albedo: color }
	}
}

impl Material for Lambertian {
	fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<MaterialRayInteraction> {
		let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

		if scatter_direction.near_zero() {
			scatter_direction = hit_record.normal;
		}		

		Some(MaterialRayInteraction::new(
			self.albedo,
			Ray::new(hit_record.p, scatter_direction)
		))
	}
}