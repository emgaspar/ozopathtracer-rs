use std::rc::Rc;

use crate::{
	material::Material,
	vec::{Vec3, Point3},
	ray::Ray
};

pub struct HitRecord {
	pub p: Point3,
	pub normal: Vec3,
	pub t: f64,
	pub front_face: bool,
	pub material: Rc<dyn Material>
}

pub trait Hitable {
	fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

pub struct HitableList {
	objects: Vec<Rc<dyn Hitable>>
}

impl HitableList {
	pub fn new(objects: Vec<Rc<dyn Hitable>>) -> HitableList {
		HitableList { 
			objects: objects 
		}
	}

	pub fn add(&mut self, object: impl Hitable + 'static) {
		self.objects.push(Rc::new(object))
	}
}

impl Hitable for HitableList {
	fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
		let mut hit_anything: Option<HitRecord> = None;
		let mut closest_so_far: f64 = ray_tmax;

		for object in self.objects.iter() {
			if let Some(hit) = object.hit(ray, ray_tmin, closest_so_far) {
				closest_so_far = hit.t;
				hit_anything = Some(hit);
			}
		}

		hit_anything
	}

}