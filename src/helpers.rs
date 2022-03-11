use std::f32::consts::PI;
use bevy::math::Vec3;
use bevy_rapier3d::na::ComplexField;
use rand::Rng;

pub(crate) fn random_vector(length: f32) -> Vec3 {
    use rand::thread_rng;

    let phi = thread_rng().gen_range(0.0..=PI * 2.0);
    let cos_theta = thread_rng().gen_range(-1.0..=1.0);

    let theta = cos_theta.acos();

    Vec3::new(
        theta.sin() * phi.cos(),
        theta.sin() * phi.sin(),
        theta.cos(),
    )
    .normalize()
        * length
}
