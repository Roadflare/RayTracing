use crate::scene::{ColorType, Light, Material, Plane, Scene, Sphere, Texture, Triangle};
use crate::vectors::Vector;
use sdl2::pixels::Color;
use std::sync::LazyLock; //Black magic

static ZERO_VECTOR: Vector = Vector {
    x: 0.,
    y: 0.,
    z: 0.,
};

pub static SCENE1: LazyLock<Scene> = LazyLock::new(|| {
    Scene::make(
        vec![
            // Zelena sfera
            Sphere::make(
                &Vector::make(0.5, 0.0, -1.0),
                1.0,
                Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 0)),
                    reflectivity: Some(0.3),
                    transparency: None
                },
            ),
            // Rdeča sfera
            Sphere::make(
                &Vector::make(0.5, 0.0, -3.0),
                0.5,
                Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 0)),
                    reflectivity: Some(0.7),
                    transparency: None
                },
            ),
            // Modra sfera
            Sphere::make(
                &Vector::make(0.5, 0.0, 2.0),
                1.5,
                Material {
                    color: ColorType::Solid(Color::RGB(0, 0, 255)),
                    reflectivity: Some(0.2),
                    transparency: None
                },
            ),
        ],
        vec![], // triangles
        vec![Plane {
            point: Vector::make(0.0, -2.0, 0.0),
            normal: Vector::make(0.0, 1.0, 0.0),
            material: Material {
                color: ColorType::Solid(Color::RGB(100, 100, 100)),
                reflectivity: None,
                transparency: None
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
                radius: 400.0,
                material: Material {
                    color: ColorType::Function(Box::new(move |point: Vector| {
                        let (u, v) = Texture::sphere_uv(Vector::make(3.0, 2.0, 0.0), 400., point);
                        SUN_TEXTURE.uv_pixel_from_texture(u, v)
                    })),
                    reflectivity: None,
                    transparency: None
                },
            },
            Sphere {
                center: Vector::make(3.0, -1.0, 0.0),
                radius: 1.5,
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 0, 255)),
                    reflectivity: None,
                    transparency: None
                },
            },
        ],
        vec![],
        vec![Plane {
            point: Vector::make(0.0, -2.0, 0.0),
            normal: Vector::make(0.0, 1.0, 0.0), // y = -1 ravnina
            material: Material {
                color: ColorType::Solid(Color::RGB(100, 100, 100)),
                reflectivity: None,
                transparency: None
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
                    reflectivity: Some(0.3),
                    transparency: None
                },
            },
            Sphere {
                center: Vector::make(0.0, 0.0, 0.0),
                radius: 0.8,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 0)),
                    reflectivity: Some(0.3),
                    transparency: None
                },
            },
            Sphere {
                center: Vector::make(0.0, 0.0, 1.5),
                radius: 0.5,
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 0, 255)),
                    reflectivity: Some(0.3),
                    transparency: None
                },
            },
        ],
        vec![],
        vec![Plane {
            point: Vector::make(0.0, -2.0, 0.0),
            normal: Vector::make(0.0, 1.0, 0.0), // y = -1 ravnina
            material: Material {
                color: ColorType::Solid(Color::RGB(100, 100, 100)),
                reflectivity: None,
                transparency: None
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
                    reflectivity: Some(0.3),
                    transparency: None
                },
            },
            Sphere {
                center: Vector::make(0.0, 1.0, -1.0),
                radius: 0.7,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 0)),
                    reflectivity: Some(0.3),
                    transparency: None
                },
            },
            Sphere {
                center: Vector::make(0.0, -1.0, 0.0),
                radius: 0.7,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 255, 255)),
                    reflectivity: Some(0.3),
                    transparency: None
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
                    reflectivity: Some(0.3),
                    transparency: None
                },
            },
            Sphere {
                center: Vector::make(0.5, 0.0, -3.0),
                radius: 0.5,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 255)),
                    reflectivity: Some(0.3),
                    transparency: None
                },
            },
            Sphere {
                center: Vector::make(0.5, 0.0, 2.0),
                radius: 1.5,
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 255)),
                    reflectivity: Some(0.3),
                    transparency: None
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
                    reflectivity: None,
                    transparency: None
                },
            },
            Sphere {
                center: Vector::make(1.0, 0.0, 0.0),
                radius: 0.1,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 0)),
                    reflectivity: None,
                    transparency: None
                },
            },
            Sphere {
                center: Vector::make(0.0, 1.0, 0.0),
                radius: 0.1,
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 0)),
                    reflectivity: None,
                    transparency: None
                },
            },
            Sphere {
                center: Vector::make(0.0, 0.0, 1.0),
                radius: 0.1,
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 0, 255)),
                    reflectivity: None,
                    transparency: None
                },
            },
            Sphere {
                center: Vector::make(1.0, 1.0, 0.0),
                radius: 0.1,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 255, 0)),
                    reflectivity: None,
                    transparency: None
                },
            },
            Sphere {
                center: Vector::make(0.0, 1.0, 1.0),
                radius: 0.1,
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 255)),
                    reflectivity: None,
                    transparency: None
                },
            },
            Sphere {
                center: Vector::make(1.0, 0.0, 1.0),
                radius: 0.1,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 255)),
                    reflectivity: None,
                    transparency: None
                },
            },
            Sphere {
                center: Vector::make(1.0, 1.0, 1.0),
                radius: 0.1,
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 255, 255)),
                    reflectivity: None,
                    transparency: None
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
                    reflectivity: None,
                    transparency: None
                },
            ),
            Triangle::make(
                Vector::make(0.0, 1.0, 0.0),
                Vector::make(1.0, 0.0, 0.0),
                Vector::make(1.0, 1.0, 0.0),
                Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 0)),
                    reflectivity: None,
                    transparency: None
                },
            ),
            Triangle::make(
                Vector::make(0.0, 0.0, 1.0),
                Vector::make(0.0, 1.0, 0.0),
                Vector::make(0.0, 0.0, 0.0),
                Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 0)),
                    reflectivity: None,
                    transparency: None
                },
            ),
            Triangle::make(
                Vector::make(0.0, 1.0, 0.0),
                Vector::make(0.0, 0.0, 1.0),
                Vector::make(0.0, 1.0, 1.0),
                Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 0)),
                    reflectivity: None,
                    transparency: None
                },
            ),
            Triangle::make(
                Vector::make(1.0, 0.0, 0.0),
                Vector::make(0.0, 0.0, 1.0),
                Vector::make(1.0, 0.0, 1.0),
                Material {
                    color: ColorType::Solid(Color::RGB(0, 0, 255)),
                    reflectivity: None,
                    transparency: None
                },
            ),
            Triangle::make(
                Vector::make(1.0, 0.0, 0.0),
                Vector::make(0.0, 0.0, 1.0),
                Vector::make(0.0, 0.0, 0.0),
                Material {
                    color: ColorType::Solid(Color::RGB(0, 0, 255)),
                    reflectivity: None,
                    transparency: None
                },
            ),
            Triangle::make(
                Vector::make(1.0, 0.0, 1.0),
                Vector::make(0.0, 1.0, 1.0),
                Vector::make(0.0, 0.0, 1.0),
                Material {
                    color: ColorType::Solid(Color::RGB(255, 255, 0)),
                    reflectivity: None,
                    transparency: None
                },
            ),
            Triangle::make(
                Vector::make(0.0, 1.0, 1.0),
                Vector::make(1.0, 0.0, 1.0),
                Vector::make(1.0, 1.0, 1.0),
                Material {
                    color: ColorType::Solid(Color::RGB(255, 255, 0)),
                    reflectivity: None,
                    transparency: None
                },
            ),
            Triangle::make(
                Vector::make(1.0, 0.0, 1.0),
                Vector::make(1.0, 1.0, 0.0),
                Vector::make(1.0, 0.0, 0.0),
                Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 255)),
                    reflectivity: None,
                    transparency: None
                },
            ),
            Triangle::make(
                Vector::make(1.0, 1.0, 0.0),
                Vector::make(1.0, 0.0, 1.0),
                Vector::make(1.0, 1.0, 1.0),
                Material {
                    color: ColorType::Solid(Color::RGB(255, 0, 255)),
                    reflectivity: None,
                    transparency: None
                },
            ),
            Triangle::make(
                Vector::make(1.0, 1.0, 0.0),
                Vector::make(0.0, 1.0, 1.0),
                Vector::make(1.0, 1.0, 1.0),
                Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 255)),
                    reflectivity: None,
                    transparency: None
                },
            ),
            Triangle::make(
                Vector::make(1.0, 1.0, 0.0),
                Vector::make(0.0, 1.0, 1.0),
                Vector::make(0.0, 1.0, 0.0),
                Material {
                    color: ColorType::Solid(Color::RGB(0, 255, 255)),
                    reflectivity: None,
                    transparency: None
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

pub static SCENE_H: LazyLock<Scene> = LazyLock::new(|| {
    Scene::make(
        vec![Sphere::make(
            &ZERO_VECTOR,
            1.,
            Material {
                color: ColorType::Solid(Color::RGB(30, 30, 30)),
                reflectivity: Some(0.05),
                transparency: None
            },
        )],
        vec![],
        vec![
            Plane {
                point: Vector::make(3., 0., 0.),
                normal: Vector::make(-1., 0., 0.),
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 150, 150)),
                    reflectivity: Some(0.95),
                    transparency: None
                },
            },
            Plane {
                point: Vector::make(-3., 0., 0.),
                normal: Vector::make(1., 0., 0.),
                material: Material {
                    color: ColorType::Solid(Color::RGB(150, 255, 255)),
                    reflectivity: Some(0.95),
                    transparency: None
                },
            },
            Plane {
                point: Vector::make(0., 3., 0.),
                normal: Vector::make(0., -1., 0.),
                material: Material {
                    color: ColorType::Solid(Color::RGB(150, 255, 150)),
                    reflectivity: Some(0.95),
                    transparency: None
                },
            },
            Plane {
                point: Vector::make(0., -3., 0.),
                normal: Vector::make(0., 1., 0.),
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 150, 255)),
                    reflectivity: Some(0.95),
                    transparency: None
                },
            },
            Plane {
                point: Vector::make(0., 0., 3.),
                normal: Vector::make(0., 0., -1.),
                material: Material {
                    color: ColorType::Solid(Color::RGB(150, 150, 255)),
                    reflectivity: Some(0.95),
                    transparency: None
                },
            },
            Plane {
                point: Vector::make(0., 0., -3.),
                normal: Vector::make(0., 0., 1.),
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 255, 150)),
                    reflectivity: Some(0.95),
                    transparency: None
                },
            },
        ],
        vec![
            Light {
                position: Vector::make(2., 0., 0.),
                intensity: 1.,
            },
            Light {
                position: Vector::make(-2., 0., 0.),
                intensity: 1.,
            },
        ],
        0.5,
    )
});

static SUN_TEXTURE: LazyLock<Texture> = LazyLock::new(|| Texture::from_file("textures/sun.jpg"));
static MERCURY_TEXTURE: LazyLock<Texture> =
    LazyLock::new(|| Texture::from_file("textures/mercury.jpg"));
static VENERA_TEXTURE: LazyLock<Texture> =
    LazyLock::new(|| Texture::from_file("textures/venera.jpg"));
static EARTH_TEXTURE: LazyLock<Texture> =
    LazyLock::new(|| Texture::from_file("textures/earth.jpg"));
static MARS_TEXTURE: LazyLock<Texture> = LazyLock::new(|| Texture::from_file("textures/mars.jpg"));
static JUPITER_TEXTURE: LazyLock<Texture> =
    LazyLock::new(|| Texture::from_file("textures/jupiter.jpg"));
static SATURN_TEXTURE: LazyLock<Texture> =
    LazyLock::new(|| Texture::from_file("textures/saturn.jpg"));
static URANUS_TEXTURE: LazyLock<Texture> =
    LazyLock::new(|| Texture::from_file("textures/uranus.jpg"));
static NEPTUNE_TEXTURE: LazyLock<Texture> =
    LazyLock::new(|| Texture::from_file("textures/neptune.jpg"));

pub static SCENE_J: LazyLock<Scene> = LazyLock::new(|| {
    Scene::make(
        vec![
            // Sun
            Sphere::make(
                &Vector::make(13., 0., 0.),
                5.,
                Material {
                    color: ColorType::Function(Box::new(move |point: Vector| {
                        let (u, v) = Texture::sphere_uv(Vector::make(13., 0., 0.), 5., point);
                        SUN_TEXTURE.uv_pixel_from_texture(u, v)
                    })),
                    reflectivity: None,
                    transparency: None
                },
            ),
            // Mercury
            Sphere::make(
                &Vector::make(13., 0., 7.),
                1.,
                Material {
                    color: ColorType::Function(Box::new(move |point: Vector| {
                        let (u, v) = Texture::sphere_uv(Vector::make(13., 0., 7.), 1., point);
                        MERCURY_TEXTURE.uv_pixel_from_texture(u, v)
                    })),
                    reflectivity: None,
                    transparency: None
                },
            ),
            // Venera
            Sphere::make(
                &Vector::make(13., 0., 9.),
                1.,
                Material {
                    color: ColorType::Function(Box::new(move |point: Vector| {
                        let (u, v) = Texture::sphere_uv(Vector::make(13., 0., 9.), 1., point);
                        VENERA_TEXTURE.uv_pixel_from_texture(u, v)
                    })),
                    reflectivity: None,
                    transparency: None
                },
            ),
            // Earth
            Sphere::make(
                &Vector::make(13., 0., 11.),
                1.,
                Material {
                    color: ColorType::Function(Box::new(move |point: Vector| {
                        let (u, v) = Texture::sphere_uv(Vector::make(13., 0., 11.), 1., point);
                        EARTH_TEXTURE.uv_pixel_from_texture(u, v)
                    })),
                    reflectivity: None,
                    transparency: None
                },
            ),
            // Mars
            Sphere::make(
                &Vector::make(13., 0., 13.),
                1.,
                Material {
                    color: ColorType::Function(Box::new(move |point: Vector| {
                        let (u, v) = Texture::sphere_uv(Vector::make(13., 0., 13.), 1., point);
                        MARS_TEXTURE.uv_pixel_from_texture(u, v)
                    })),
                    reflectivity: None,
                    transparency: None
                },
            ),
            // Jupiter
            Sphere::make(
                &Vector::make(13., 0., 20.),
                3.,
                Material {
                    color: ColorType::Function(Box::new(move |point: Vector| {
                        let (u, v) = Texture::sphere_uv(Vector::make(13., 0., 20.), 3., point);
                        JUPITER_TEXTURE.uv_pixel_from_texture(u, v)
                    })),
                    reflectivity: None,
                    transparency: None
                },
            ),
            // Saturn
            Sphere::make(
                &Vector::make(13., 0., 29.),
                3.,
                Material {
                    color: ColorType::Function(Box::new(move |point: Vector| {
                        let (u, v) = Texture::sphere_uv(Vector::make(13., 0., 29.), 3., point);
                        SATURN_TEXTURE.uv_pixel_from_texture(u, v)
                    })),
                    reflectivity: None,
                    transparency: None
                },
            ),
            // Uranus
            Sphere::make(
                &Vector::make(13., 0., 36.),
                2.,
                Material {
                    color: ColorType::Function(Box::new(move |point: Vector| {
                        let (u, v) = Texture::sphere_uv(Vector::make(13., 0., 36.), 2., point);
                        URANUS_TEXTURE.uv_pixel_from_texture(u, v)
                    })),
                    reflectivity: None,
                    transparency: None
                },
            ),
            // Neptune
            Sphere::make(
                &Vector::make(13., 0., 42.),
                2.,
                Material {
                    color: ColorType::Function(Box::new(move |point: Vector| {
                        let (u, v) = Texture::sphere_uv(Vector::make(13., 0., 42.), 2., point);
                        NEPTUNE_TEXTURE.uv_pixel_from_texture(u, v)
                    })),
                    reflectivity: None,
                    transparency: None
                },
            ),
        ],
        vec![],
        vec![
            Plane {
                point: Vector::make(5000., 0., 0.),
                normal: Vector::make(-1., 0., 0.),
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 0, 0)),
                    reflectivity: None,
                    transparency: None
                },
            },
            Plane {
                point: Vector::make(-5000., 0., 0.),
                normal: Vector::make(1., 0., 0.),
                material: Material {
                    color: ColorType::Solid(Color::RGB(0, 0, 0)),
                    reflectivity: None,
                    transparency: None
                },
            },
        ],
        vec![Light {
            position: Vector::make(900., 45., 90.),
            intensity: 1.,
        }],
        1.,
    )
});

static PEAK: LazyLock<Texture> = LazyLock::new(|| Texture::from_file("textures/peak.jpg"));

pub static PEAK_K: LazyLock<Scene> = LazyLock::new(|| {
    Scene::make(
        vec![Sphere::make(
            &ZERO_VECTOR,
            1.,
            Material {
                color: ColorType::Function(Box::new(move |point: Vector| {
                    let (u, v) = Texture::sphere_uv(ZERO_VECTOR, 2., point);
                    PEAK.uv_pixel_from_texture(u, v)
                })),
                reflectivity: None,
                transparency: None
            },
        )],
        vec![],
        vec![
            Plane {
                point: Vector::make(3., 0., 0.),
                normal: Vector::make(-1., 0., 0.),
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 150, 150)),
                    reflectivity: Some(0.8),
                    transparency: None
                },
            },
            Plane {
                point: Vector::make(-3., 0., 0.),
                normal: Vector::make(1., 0., 0.),
                material: Material {
                    color: ColorType::Solid(Color::RGB(150, 255, 255)),
                    reflectivity: Some(0.8),
                    transparency: None
                },
            },
            Plane {
                point: Vector::make(0., 3., 0.),
                normal: Vector::make(0., -1., 0.),
                material: Material {
                    color: ColorType::Solid(Color::RGB(150, 255, 150)),
                    reflectivity: Some(0.8),
                    transparency: None
                },
            },
            Plane {
                point: Vector::make(0., -3., 0.),
                normal: Vector::make(0., 1., 0.),
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 150, 255)),
                    reflectivity: Some(0.8),
                    transparency: None
                },
            },
            Plane {
                point: Vector::make(0., 0., 3.),
                normal: Vector::make(0., 0., -1.),
                material: Material {
                    color: ColorType::Solid(Color::RGB(150, 150, 255)),
                    reflectivity: Some(0.8),
                    transparency: None
                },
            },
            Plane {
                point: Vector::make(0., 0., -3.),
                normal: Vector::make(0., 0., 1.),
                material: Material {
                    color: ColorType::Solid(Color::RGB(255, 255, 150)),
                    reflectivity: Some(0.8),
                    transparency: None
                },
            },
        ],
        vec![
            Light {
                position: Vector::make(2., 0., 0.),
                intensity: 1.,
            },
            Light {
                position: Vector::make(-2., 0., 0.),
                intensity: 1.,
            },
        ],
        0.7,
    )
});
