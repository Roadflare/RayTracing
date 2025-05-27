use crate::scene::{ColorType, Light, Material, Plane, Scene, Sphere, Triangle};
use crate::vectors::Vector;
use sdl2::pixels::Color;
use std::sync::LazyLock; //Black magic

pub static SCENE1: LazyLock<Scene> = LazyLock::new(|| {
    Scene::make(
        vec![
            // Zelena sfera
            Sphere::make(
                &Vector::make(0.5, 0.0, -1.0),
                1.0,
                Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 0)),
                    reflectivity: 0.3,
                },
            ),
            // Rdeča sfera
            Sphere::make(
                &Vector::make(0.5, 0.0, -3.0),
                0.5,
                Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 0)),
                    reflectivity: 0.7,
                },
            ),
            // Modra sfera
            Sphere::make(
                &Vector::make(0.5, 0.0, 2.0),
                1.5,
                Material {
                    color: ColorType::Solid(Color::RGB(0, 0, 255)),
                    reflectivity: 0.2,
                },
            ),
        ],
        vec![], // triangles
        vec![Plane {
            point: Vector::make(0.0, -2.0, 0.0),
            normal: Vector::make(0.0, 1.0, 0.0),
            material: Material {
                color: ColorType::Solid(Color::RGB(100, 100, 100)),
                reflectivity: 0.0,
            },
        }],
        vec![
            Light {
                position: Vector::make(0.5, 0.0, -5.0),
                intensity: 1.0,
            },
            Light {
                position: Vector::make(0.5, 0.0, -10.0),
                intensity: 1.0,
            },
        ],
        0.3, // ambient light
    )
});

pub static SCENE2: LazyLock<Scene> = LazyLock::new(|| {
    Scene::make(
        vec![
            Sphere {
                center: Vector::make(3.0, 2.0, 0.0),
                radius: 1.0,
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 0)),
                    reflectivity: 0.0,
                },
            },
            Sphere {
                center: Vector::make(3.0, -1.0, 0.0),
                radius: 1.5,
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 0, 255)),
                    reflectivity: 0.0,
                },
            },
        ],
        vec![],
        vec![Plane {
            point: Vector::make(0.0, -2.0, 0.0),
            normal: Vector::make(0.0, 1.0, 0.0), // y = -1 ravnina
            material: Material {
                color: ColorType::Solid(Color::RGB(100, 100, 100)),
                reflectivity: 0.0,
            },
        }],
        vec![Light {
            position: Vector::make(3.0, 15.0, 0.0),
            intensity: 1.0,
        }],
        0.3,
    )
});

pub static SCENE3: LazyLock<Scene> = LazyLock::new(|| {
    Scene::make(
        vec![
            Sphere {
                center: Vector::make(0.0, 0.0, -1.5),
                radius: 0.5,
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 0)),
                    reflectivity: 0.3,
                },
            },
            Sphere {
                center: Vector::make(0.0, 0.0, 0.0),
                radius: 0.8,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 0)),
                    reflectivity: 0.3,
                },
            },
            Sphere {
                center: Vector::make(0.0, 0.0, 1.5),
                radius: 0.5,
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 0, 255)),
                    reflectivity: 0.3,
                },
            },
        ],
        vec![],
        vec![Plane {
            point: Vector::make(0.0, -2.0, 0.0),
            normal: Vector::make(0.0, 1.0, 0.0), // y = -1 ravnina
            material: Material {
                color: ColorType::Solid(Color::RGB(100, 100, 100)),
                reflectivity: 0.0,
            },
        }],
        vec![
            Light {
                position: Vector::make(0.0, 0.0, -5.0),
                intensity: 1.0,
            },
            Light {
                position: Vector::make(0.0, 0.0, 5.0),
                intensity: 1.0,
            },
        ],
        0.3,
    )
});

pub static SCENE4: LazyLock<Scene> = LazyLock::new(|| {
    Scene::make(
        vec![
            Sphere {
                center: Vector::make(0.0, 1.0, 1.0),
                radius: 0.7,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 255, 0)),
                    reflectivity: 0.3,
                },
            },
            Sphere {
                center: Vector::make(0.0, 1.0, -1.0),
                radius: 0.7,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 0)),
                    reflectivity: 0.3,
                },
            },
            Sphere {
                center: Vector::make(0.0, -1.0, 0.0),
                radius: 0.7,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 255, 255)),
                    reflectivity: 0.3,
                },
            },
        ],
        vec![],
        vec![],
        vec![
            Light {
                position: Vector::make(0.0, 2.0, -4.0),
                intensity: 0.7,
            },
            Light {
                position: Vector::make(0.0, -2.0, 4.0),
                intensity: 0.5,
            },
        ],
        0.3,
    )
});

pub static SCENE5: LazyLock<Scene> = LazyLock::new(|| {
    Scene::make(
        vec![
            Sphere {
                center: Vector::make(0.5, 0.0, -1.0),
                radius: 1.0,
                material: Material {
                    color: ColorType::Solid(Color::RGB(144, 144, 144)),
                    reflectivity: 0.3,
                },
            },
            Sphere {
                center: Vector::make(0.5, 0.0, -3.0),
                radius: 0.5,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 255)),
                    reflectivity: 0.3,
                },
            },
            Sphere {
                center: Vector::make(0.5, 0.0, 2.0),
                radius: 1.5,
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 255)),
                    reflectivity: 0.3,
                },
            },
        ],
        vec![],
        vec![],
        vec![Light {
            position: Vector::make(-5.0, 0.0, -20.0),
            intensity: 0.75,
        }],
        0.15,
    )
});

pub static SCENE6: LazyLock<Scene> = LazyLock::new(|| {
    Scene::make(
        vec![
            Sphere {
                center: Vector::make(0.0, 0.0, 0.0),
                radius: 0.1,
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 0, 0)),
                    reflectivity: 0.0,
                },
            },
            Sphere {
                center: Vector::make(1.0, 0.0, 0.0),
                radius: 0.1,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 0)),
                    reflectivity: 0.0,
                },
            },
            Sphere {
                center: Vector::make(0.0, 1.0, 0.0),
                radius: 0.1,
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 0)),
                    reflectivity: 0.0,
                },
            },
            Sphere {
                center: Vector::make(0.0, 0.0, 1.0),
                radius: 0.1,
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 0, 255)),
                    reflectivity: 0.0,
                },
            },
            Sphere {
                center: Vector::make(1.0, 1.0, 0.0),
                radius: 0.1,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 255, 0)),
                    reflectivity: 0.0,
                },
            },
            Sphere {
                center: Vector::make(0.0, 1.0, 1.0),
                radius: 0.1,
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 255)),
                    reflectivity: 0.0,
                },
            },
            Sphere {
                center: Vector::make(1.0, 0.0, 1.0),
                radius: 0.1,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 255)),
                    reflectivity: 0.0,
                },
            },
            Sphere {
                center: Vector::make(1.0, 1.0, 1.0),
                radius: 0.1,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 255, 255)),
                    reflectivity: 0.0,
                },
            },
        ],
        vec![
            Triangle::make(
                Vector::make(1.0, 0.0, 0.0),
                Vector::make(0.0, 1.0, 0.0),
                Vector::make(0.0, 0.0, 0.0),
                Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 0)),
                    reflectivity: 0.0,
                },
            ),
            Triangle::make(
                Vector::make(0.0, 1.0, 0.0),
                Vector::make(1.0, 0.0, 0.0),
                Vector::make(1.0, 1.0, 0.0),
                Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 0)),
                    reflectivity: 0.0,
                },
            ),
            Triangle::make(
                Vector::make(0.0, 0.0, 1.0),
                Vector::make(0.0, 1.0, 0.0),
                Vector::make(0.0, 0.0, 0.0),
                Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 0)),
                    reflectivity: 0.0,
                },
            ),
            Triangle::make(
                Vector::make(0.0, 1.0, 0.0),
                Vector::make(0.0, 0.0, 1.0),
                Vector::make(0.0, 1.0, 1.0),
                Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 0)),
                    reflectivity: 0.0,
                },
            ),
            Triangle::make(
                Vector::make(1.0, 0.0, 0.0),
                Vector::make(0.0, 0.0, 1.0),
                Vector::make(1.0, 0.0, 1.0),
                Material {
                    color: ColorType::Solid(Color::RGB(0, 0, 255)),
                    reflectivity: 0.0,
                },
            ),
            Triangle::make(
                Vector::make(1.0, 0.0, 0.0),
                Vector::make(0.0, 0.0, 1.0),
                Vector::make(0.0, 0.0, 0.0),
                Material {
                    color: ColorType::Solid(Color::RGB(0, 0, 255)),
                    reflectivity: 0.0,
                },
            ),
            Triangle::make(
                Vector::make(1.0, 0.0, 1.0),
                Vector::make(0.0, 1.0, 1.0),
                Vector::make(0.0, 0.0, 1.0),
                Material {
                    color: ColorType::Solid(Color::RGB(255, 255, 0)),
                    reflectivity: 0.0,
                },
            ),
            Triangle::make(
                Vector::make(0.0, 1.0, 1.0),
                Vector::make(1.0, 0.0, 1.0),
                Vector::make(1.0, 1.0, 1.0),
                Material {
                    color: ColorType::Solid(Color::RGB(255, 255, 0)),
                    reflectivity: 0.0,
                },
            ),
            Triangle::make(
                Vector::make(1.0, 0.0, 1.0),
                Vector::make(1.0, 1.0, 0.0),
                Vector::make(1.0, 0.0, 0.0),
                Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 255)),
                    reflectivity: 0.0,
                },
            ),
            Triangle::make(
                Vector::make(1.0, 1.0, 0.0),
                Vector::make(1.0, 0.0, 1.0),
                Vector::make(1.0, 1.0, 1.0),
                Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 255)),
                    reflectivity: 0.0,
                },
            ),
            Triangle::make(
                Vector::make(1.0, 1.0, 0.0),
                Vector::make(0.0, 1.0, 1.0),
                Vector::make(1.0, 1.0, 1.0),
                Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 255)),
                    reflectivity: 0.0,
                },
            ),
            Triangle::make(
                Vector::make(1.0, 1.0, 0.0),
                Vector::make(0.0, 1.0, 1.0),
                Vector::make(0.0, 1.0, 0.0),
                Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 255)),
                    reflectivity: 0.0,
                },
            ),
        ],
        vec![],
        vec![Light {
            position: Vector::make(1., 1., 0.),
            intensity: 0.75,
        }],
        1.0,
    )
});
//    Vector::make(-1.0, -1.0, -1.0),
//    Vector::make(1.0, -1.0, -1.0),
//    Vector::make(1.0, 1.0, -1.0),
//    Vector::make(-1.0, 1.0, -1.0),
//    Vector::make(-1.0, -1.0, 1.0),
//    Vector::make(1.0, -1.0, 1.0),
//    Vector::make(1.0, 1.0, 1.0),
//    Vector::make(-1.0, 1.0, 1.0),
