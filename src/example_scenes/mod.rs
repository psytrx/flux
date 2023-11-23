mod mat_util;
mod prim_util;

use crate::{
    example_scenes::{
        mat_util::{dielectric, matte, metal},
        prim_util::{floor, sphere},
    },
    flux::{cameras::*, *},
};

pub enum ExampleScene {
    MaterialDemo,
}

pub fn load_example_scene(scene: ExampleScene) -> Scene {
    match scene {
        ExampleScene::MaterialDemo => material_demo(),
    }
}

fn material_demo() -> Scene {
    let primitives = {
        let white_matte = matte(1.0, 1.0, 1.0);
        let rose_gold = metal(0.72, 0.45, 0.2, 0.05);
        let blue_matte = matte(0.1, 0.2, 0.5);
        let mirror = metal(0.7, 0.7, 0.7, 0.0);
        let glass = dielectric(0.7, 0.9, 0.7, 1.5);

        let floor = floor(white_matte.clone());
        let left_sphere = sphere(-2.0, 1.0, 0.0, 1.0, mirror.clone());
        let middle_sphere = sphere(0.0, 1.0, 0.0, 1.0, blue_matte.clone());
        let right_sphere = sphere(2.0, 1.0, 0.0, 1.0, rose_gold.clone());
        let glass_sphere = sphere(-1.0, 0.4, -2.0, 0.4, glass.clone());

        vec![
            floor,
            left_sphere,
            middle_sphere,
            right_sphere,
            glass_sphere,
        ]
    };

    let camera = {
        let look_from = glam::vec3(0.0, 2.5, -6.0);
        let look_at = glam::vec3(0.0, 0.5, 0.0);
        Box::new(PerspectiveCamera::new(
            glam::uvec2(1024, 576),
            look_from,
            look_at,
            65.0,
            0.025,
            look_at.distance(look_from),
        ))
    };

    Scene::new(primitives, camera)
}
