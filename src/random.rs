use fastrand;

#[inline]
pub fn random_f64() -> f64 {
	fastrand::f64()	
}

#[inline]
pub fn random_f64_range(min: f64, max: f64) -> f64 {
	min + fastrand::f64() * (max - min)
}