use std::rc::Rc;

use super::hit::{HitRecord, Hitable};
use super::ray::Ray;
use super::vec::{Point3, Vec3, dot};
use crate::material::Material;

pub struct Sphere {
    center: Point3,
    radius: f64,
	material: Rc<dyn Material>,
}

impl Sphere {
	pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
		Sphere { 
			center: center, 
			radius: radius,
			material: material
		}
	}
}

impl Hitable for Sphere {

    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin() - self.center;
        let a: f64 = ray.dir().length_squared();
        let half_b: f64 = dot(oc, ray.dir());
        let c: f64 = oc.length_squared() - self.radius * self.radius;

        let discriminant: f64 = half_b * half_b - a * c;
		
		if discriminant < 0.0 {
            return None;
		}
					 
		let sqrtd: f64 = discriminant.sqrt();

		// Find the nearest root that lies in the acceptable range
		let mut root: f64 = (-half_b - sqrtd) / a;

		if root < ray_tmin || root > ray_tmax {
			root = (-half_b + sqrtd) / a;

			if root < ray_tmin || root > ray_tmax {
				return None;
			}
		}

		let hit_t: f64 = root;
		let hit_point: Vec3 = ray.at(hit_t);

		let outward_normal: Vec3 = (hit_point - self.center) / self.radius;
		let hit_front_face: bool = dot(ray.dir(), outward_normal) < 0.0;
		let hit_normal: Vec3 = if hit_front_face { outward_normal } else { -outward_normal };

        Some(HitRecord {
			p: hit_point, 
			normal: hit_normal,
			t: hit_t,
			front_face: hit_front_face,
			material: self.material.clone()
		})

    }
}