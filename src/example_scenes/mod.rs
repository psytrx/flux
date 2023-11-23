mod mat_util;
mod prim_util;

use crate::{
    example_scenes::{
        mat_util::{dielectric, diffuse, matte, metal},
        prim_util::{floor, sphere},
    },
    flux::{
        cameras::*,
        shapes::Quad,
        textures::{ConstantTexture, ImageTexture},
        *,
    },
};

#[allow(dead_code)]
pub enum ExampleScene {
    MaterialDemo,
    CornellBox,
}

pub fn load_example_scene(scene: ExampleScene) -> Scene {
    match scene {
        ExampleScene::MaterialDemo => material_demo(),
        ExampleScene::CornellBox => cornell_box(),
    }
}

fn cornell_box() -> Scene {
    let primitives = {
        let white = matte(0.73, 0.73, 0.73);
        let red = matte(0.65, 0.05, 0.05);
        let green = matte(0.12, 0.45, 0.15);

        let size = 100.0;
        let dfl = glam::vec3(-size, -size, -size);
        let dfr = glam::vec3(size, -size, -size);
        let dbr = glam::vec3(size, -size, size);
        let dbl = glam::vec3(-size, -size, size);
        let ufl = glam::vec3(-size, size, -size);
        let ufr = glam::vec3(size, size, -size);
        let ubr = glam::vec3(size, size, size);
        let ubl = glam::vec3(-size, size, size);

        let floor = Primitive::new(Box::new(Quad::new([dfl, dfr, dbr, dbl])), white.clone());
        let ceiling = Primitive::new(Box::new(Quad::new([ufl, ufr, ubr, ubl])), white.clone());
        let left_wall = Primitive::new(Box::new(Quad::new([dfl, dbl, ubl, ufl])), red.clone());
        let right_wall = Primitive::new(Box::new(Quad::new([dfr, dbr, ubr, ufr])), green.clone());
        let back_wall = Primitive::new(Box::new(Quad::new([dbl, dbr, ubr, ubl])), white.clone());

        let light = Primitive::new(
            Box::new(Quad::new([
                glam::vec3(size / 6.0, size - 1.0, -size / 6.0),
                glam::vec3(size / 6.0, size - 1.0, size / 6.0),
                glam::vec3(-size / 6.0, size - 1.0, size / 6.0),
                glam::vec3(-size / 6.0, size - 1.0, -size / 6.0),
            ])),
            diffuse(25.0, 25.0, 25.0),
        );

        vec![floor, ceiling, left_wall, right_wall, back_wall, light]
    };

    let camera = {
        let look_from = glam::vec3(0.0, 0.0, -320.0);
        let look_at = glam::Vec3::ZERO;
        Box::new(PerspectiveCamera::new(
            glam::uvec2(1024, 1024),
            look_from,
            look_at,
            50.0,
            0.025,
            look_at.distance(look_from),
        ))
    };

    let background = Box::new(ConstantTexture::new(glam::Vec3::ZERO));

    Scene::new(primitives, camera, background)
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
        let look_from = glam::vec3(0.0, 3.0, -6.0);
        let look_at = glam::vec3(0.0, 0.5, 0.0);
        Box::new(PerspectiveCamera::new(
            glam::uvec2(800, 450),
            look_from,
            look_at,
            65.0,
            0.025,
            look_at.distance(look_from),
        ))
    };

    let background = Box::new(ImageTexture::new(
        image::open("./assets/hdr/ennis.exr").unwrap(),
    ));

    Scene::new(primitives, camera, background)
}
