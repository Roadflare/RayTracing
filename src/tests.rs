use crate::scene::Scene;
use crate::vectors::Vector;
use crate::scene::{ColorType, Light, Material, Sphere, Triangle};
use sdl2::pixels::Color;
use std::sync::LazyLock; //Black magic

pub static SCENE1: LazyLock<Scene> = LazyLock::new(||
	Scene::make(
	vec![
		Sphere::make(&Vector::make(0.5, 0.0, -1.0), 1.0, Material {color: ColorType::Solid(Color::RGB(0,255,0))}),
		Sphere::make(&Vector::make(0.5, 0.0, -3.0), 0.5, Material {color: ColorType::Solid(Color::RGB(255,0,0))}),
		Sphere::make(&Vector::make(0.5, 0.0, 2.0), 1.5, Material {color: ColorType::Solid(Color::RGB(0,0,255))})
		],
	vec![],
	vec![
		Light{position: Vector::make(0.5, 0.0, -5.0), intensity: 1.0},
		Light{position: Vector::make(0.5, 0.0, -10.0), intensity: 1.0}],
	0.3,));

pub static SCENE2: LazyLock<Scene> = LazyLock::new(||
	Scene::make(
	vec![
		Sphere{
			center: Vector {x: 3.0, y: 2.0, z: 0.0}, radius: 1.0, 
			material: Material {color: ColorType::Solid(Color::RGB(0,255,0))}
		},
		Sphere{
			center: Vector {x: 3.0, y: -1.0, z: 0.0}, radius: 1.5, 
			material: Material {color: ColorType::Solid(Color::RGB(0,0,255))}
		}],
	vec![],
	vec![
		Light{position: Vector::make(3.0, 15.0, 0.0), intensity: 1.0}],
	0.3,));

pub static SCENE3: LazyLock<Scene> = LazyLock::new(||
	Scene::make(
	vec![
		Sphere{
			center: Vector {x: 0.0, y: 0.0, z: -1.5}, radius: 0.5, 
			material: Material {color: ColorType::Solid(Color::RGB(0,255,0))}
		},
		Sphere{
			center: Vector {x: 0.0, y: 0.0, z: 0.0}, radius: 0.8, 
			material: Material {color: ColorType::Solid(Color::RGB(255,0,0))}
		},
		Sphere{
			center: Vector {x: 0.0, y: 0.0, z: 1.5}, radius: 0.5,
			material: Material {color: ColorType::Solid(Color::RGB(0,0,255))}
		}],
	vec![],
	vec![
		Light{position: Vector::make(0.0, 0.0, -5.0), intensity: 1.0},
		Light{position: Vector::make(0.0, 0.0, 5.0), intensity: 1.0}],
	0.3,));

pub static SCENE4: LazyLock<Scene> = LazyLock::new(||
	Scene::make(
        vec![
            Sphere{
                center: Vector {x: 0.0, y: 1.0, z: 1.0}, radius: 0.7, 
                material: Material {color: ColorType::Solid(Color::RGB(255,255,0))}
            },
            Sphere{
                center: Vector {x: 0.0, y: 1.0, z: -1.0}, radius: 0.7, 
                material: Material {color: ColorType::Solid(Color::RGB(255,0,0))}
            },
            Sphere{
                center: Vector {x: 0.0, y: -1.0, z: 0.0}, radius: 0.7,
                material: Material {color: ColorType::Solid(Color::RGB(255,255,255))}
            }],
		vec![],
        vec![
            Light{position: Vector::make(0.0, 2.0, -4.0), intensity: 0.7},
            Light{position: Vector::make(0.0, -2.0, 4.0), intensity: 0.5}],
        0.3,));

pub static SCENE5: LazyLock<Scene> = LazyLock::new(||
	Scene::make(
        vec![
            Sphere{
                center: Vector {x: 0.5, y: 0.0, z: -1.0}, radius: 1.0, 
                material: Material {color: ColorType::Solid(Color::RGB(144, 144, 144))}
            },
            Sphere{
                center: Vector {x: 0.5, y: 0.0, z: -3.0}, radius: 0.5, 
                material: Material {color: ColorType::Solid(Color::RGB(255,0,255))}
            },
            Sphere{
                center: Vector {x: 0.5, y: 0.0, z: 2.0}, radius: 1.5,
                material: Material {color: ColorType::Solid(Color::RGB(0,255,255))}
            }],
		vec![],
        vec![
            Light{position: Vector::make(-5.0, 0.0, -20.0), intensity: 0.75},
            ],
        0.15));

pub static SCENE6: LazyLock<Scene> = LazyLock::new(||
	Scene::make(
		vec![
			Sphere::make(&Vector::make(-5., -5., -5.), 0.5, Material{ color: ColorType::Solid(Color::RGB(255, 100, 255))})
		],
		vec![
			Triangle::make(
				Vector::make(0.5, -0.5, -0.5), Vector::make(1.5, 0.5, 0.5), Vector::make(1., 1., -1.),
				Material{ color: ColorType::Solid(Color::RGB(255, 100, 255))})
		],
		vec![
			Light{position: Vector::make(1., 1., 0.), intensity: 0.75},
		], 0.2));