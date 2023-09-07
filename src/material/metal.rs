use crate::{
	material::{Material, MaterialRayInteraction},
	ray::Ray,
	hit::HitRecord,
	vec::{Vec3, reflect},
	color::Color
};

pub struct Metal {
	albedo: Color,
	fuzz: f64,
}

impl Metal {
	pub fn new(color: Color, fuzz: f64) -> Metal {
		Metal { 
			albedo: color,
			fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }
		}
	}
}

impl Material for Metal {
	fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<MaterialRayInteraction> {
		let reflected : Vec3 = reflect(ray_in.dir(), hit_record.normal);
		let scattered : Ray = Ray::new(hit_record.p, reflected + self.fuzz * Vec3::random_unit_vector());
		if Vec3::dot(scattered.dir(), hit_record.normal) > 0.0 {
			Some(MaterialRayInteraction::new(self.albedo, scattered))
		} else {
			None
		}
	}
}