use crate::flux::{cameras::*, materials::*, shapes::*, *};

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
        let floor_material = std::rc::Rc::new(MatteMaterial::new(glam::Vec3::ONE));
        let rose_gold_metal = {
            let rose_gold = glam::vec3(0.72, 0.45, 0.20);
            std::rc::Rc::new(MetalMaterial::new(rose_gold, 0.05))
        };
        let matte = std::rc::Rc::new(MatteMaterial::new(glam::vec3(0.1, 0.2, 0.5)));
        let mirror_metal = std::rc::Rc::new(MetalMaterial::new(glam::Vec3::splat(0.7), 0.0));
        let glass_material =
            std::rc::Rc::new(DielectricMaterial::new(glam::vec3(0.7, 0.9, 0.7), 1.5));

        let floor = {
            let shape = Box::new(Floor::new());
            Primitive::new(shape, floor_material.clone())
        };

        let left_sphere = {
            let shape = Box::new(Sphere::new(glam::vec3(-2.0, 1.0, 0.0), 1.0));
            Primitive::new(shape, mirror_metal.clone())
        };
        let middle_sphere = {
            let shape = Box::new(Sphere::new(glam::vec3(0.0, 1.0, 0.0), 1.0));
            Primitive::new(shape, matte.clone())
        };
        let right_sphere = {
            let shape = Box::new(Sphere::new(glam::vec3(2.0, 1.0, 0.0), 1.0));
            Primitive::new(shape, rose_gold_metal.clone())
        };

        let glass_sphere = {
            let radius = 0.35;
            let shape = Box::new(Sphere::new(
                glam::vec3(-radius, radius, -5.0 * radius),
                radius,
            ));
            Primitive::new(shape, glass_material.clone())
        };

        vec![
            floor,
            left_sphere,
            middle_sphere,
            right_sphere,
            glass_sphere,
        ]
    };

    let camera = {
        let look_from = glam::vec3(0.0, 2.5, -5.0);
        let look_at = glam::vec3(0.0, 0.5, 0.0);
        Box::new(PerspectiveCamera::new(
            glam::uvec2(512, 512),
            look_from,
            look_at,
            55.0,
            0.025,
            look_at.distance(look_from),
        ))
    };

    Scene::new(primitives, camera)
}
