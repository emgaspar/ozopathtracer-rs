use crate::{
	material::{Material, MaterialRayInteraction},
	ray::Ray,
	hit::HitRecord,
	vec::{Vec3, refract, reflect}, 
	random::random_f64,
	color::WHITE
};

pub struct Dielectric {
	ir : f64,   // Index of refraction
}

impl Dielectric {
	pub fn new(index_of_refraction: f64) -> Dielectric {
		Dielectric { ir: index_of_refraction }
	}

	fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
		// Use Schlick's approximation for reflectance
		let mut r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
		r0 *= r0;
		r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
	}
}

impl Material for Dielectric {
	fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<MaterialRayInteraction> {
		let refraction_ratio : f64 = if hit_record.front_face { 1.0 / self.ir } else { self.ir };
		let unit_direction: Vec3 = ray_in.dir().unit_vector();
		let cos_theta: f64 = -unit_direction.dot(hit_record.normal).min(1.0);
		let sin_theta: f64 = (1.0  - cos_theta.powi(2)).sqrt();

		let cannot_refract: bool = (refraction_ratio * sin_theta) > 1.0;

		let direction: Vec3 = 
			if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > random_f64()
			{
				// Must Reflect 
				reflect(unit_direction, hit_record.normal) 
			} else { 
				// Can Refract
				refract(unit_direction, hit_record.normal, refraction_ratio)
			};

		Some(MaterialRayInteraction::new(WHITE, Ray::new(hit_record.p, direction)))
	}

}
