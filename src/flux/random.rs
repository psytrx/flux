pub fn random_unit_vector(rng: &mut rand::rngs::StdRng) -> glam::Vec3 {
    random_in_unit_sphere(rng).normalize()
}

fn random_in_unit_sphere(rng: &mut rand::rngs::StdRng) -> glam::Vec3 {
    loop {
        let p = rand::Rng::gen::<glam::Vec3>(rng) * 2.0 - glam::Vec3::ONE;
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
