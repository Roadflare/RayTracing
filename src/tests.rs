use crate::scene::Scene;
use crate::vectors::Vector;
use crate::scene::{ColorType, Light, Material, Sphere};
use sdl2::libc::PR_CAP_AMBIENT;
use sdl2::pixels::Color;
use std::sync::LazyLock; //Black magic

pub static scene1: LazyLock<Scene> = LazyLock::new(||
	Scene::make(
	vec![
		Sphere{
			center: Vector {x: 0.0, y: 0.0, z: 0.0}, radius: 1.0, 
			material: Material {color: ColorType::Solid((Color::RGB(255,255,0)))}
		},
		Sphere{
			center: Vector {x: 0.0, y: 0.0, z: -2.0}, radius: 0.5, 
			material: Material {color: ColorType::Solid(Color::RGB(255,0,0))}
		}],
	vec![
		Light{position: Vector::make(0.0, 0.0, -5.0), intensity: 1.0}],
	0.3,));

pub static scene2: LazyLock<Scene> = LazyLock::new(||
	Scene::make(
	vec![
		Sphere{
			center: Vector {x: 0.0, y: 0.0, z: 0.0}, radius: 1.0, 
			material: Material {color: ColorType::Solid((Color::RGB(255,255,0)))}
		},
		Sphere{
			center: Vector {x: 0.0, y: 0.0, z: -2.0}, radius: 0.5, 
			material: Material {color: ColorType::Solid(Color::RGB(255,0,0))}
		}],
	vec![
		Light{position: Vector::make(0.0, 0.0, -5.0), intensity: 1.0}],
	0.3,));

pub static scene3: LazyLock<Scene> = LazyLock::new(||
	Scene::make(
	vec![
		Sphere{
			center: Vector {x: 0.0, y: 0.0, z: 0.0}, radius: 1.0, 
			material: Material {color: ColorType::Solid((Color::RGB(255,255,0)))}
		},
		Sphere{
			center: Vector {x: 0.0, y: 0.0, z: -2.0}, radius: 0.5, 
			material: Material {color: ColorType::Solid(Color::RGB(255,0,0))}
		}],
	vec![
		Light{position: Vector::make(0.0, 0.0, -5.0), intensity: 1.0}],
	0.3,));

pub static scene4: LazyLock<Scene> = LazyLock::new(||
	Scene::make(
	vec![
		Sphere{
			center: Vector {x: 0.0, y: 0.0, z: 0.0}, radius: 1.0, 
			material: Material {color: ColorType::Solid((Color::RGB(255,255,0)))}
		},
		Sphere{
			center: Vector {x: 0.0, y: 0.0, z: -2.0}, radius: 0.5, 
			material: Material {color: ColorType::Solid(Color::RGB(255,0,0))}
		}],
	vec![
		Light{position: Vector::make(0.0, 0.0, -5.0), intensity: 1.0}],
	0.3,));

pub fn draw_placeholder(scene: &Scene) -> () {
	panic!()
}