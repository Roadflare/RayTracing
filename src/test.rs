///1.test s praznim trikotnikom

mod vectors;
extern crate sdl2;
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;
use vectors::Vector;

// Test
fn render_triangle(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, width: u32, height: u32, v1: Vector, v2: Vector, v3: Vector) {
    let (x1, y1) = Vector::project(&v1, width as f64, height as f64, 500.0);
    let (x2, y2) = Vector::project(&v2, width as f64, height as f64, 500.0);
    let (x3, y3) = Vector::project(&v3, width as f64, height as f64, 500.0);

    canvas.set_draw_color(Color::RGB(255, 0, 0)); 
    canvas.draw_line((x1, y1), (x2, y2)).unwrap();
    canvas.draw_line((x2, y2), (x3, y3)).unwrap();
    canvas.draw_line((x3, y3), (x1, y1)).unwrap();
}

// Function to rotate a vector around the Y-axis
fn rotate_y(vec: &Vector, angle: f64) -> Vector {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    Vector::make(
        cos_a * vec.x + sin_a * vec.z,
        vec.y,
        -sin_a * vec.x + cos_a * vec.z,
    )
}

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Create an SDL window
    let window = video_subsystem
        .window("3D CPU Rendering", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    // Create a canvas (a framebuffer) for rendering
    let mut canvas = window.into_canvas().build().unwrap();

    // test trikotnika
    let mut v1 = Vector::make(-100.0, 0.0, 500.0);
    let mut v2 = Vector::make(100.0, 0.0, 500.0);
    let mut v3 = Vector::make(0.0, 100.0, 500.0);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut angle = 0.0;

    'running: loop {
        // Poll events
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        // Clear the canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        ///// del testa
        angle += 0.00001;
        v1 = rotate_y(&v1, angle);
        v2 = rotate_y(&v2, angle);
        v3 = rotate_y(&v3, angle);
        render_triangle(&mut canvas, 800, 600, v1, v2, v3);
        /////


        // Present the rendered frame
        canvas.present();

        // Sleep for a bit to maintain ~60 FPS
        std::thread::sleep(Duration::from_millis(16));
    }
}




///2.test s polnim trikotnikom
extern crate sdl2;
mod vectors;
use vectors::Vector;
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;


// Function to interpolate between two points (Bresenham-like approach)
fn interpolate(y1: i32, x1: i32, y2: i32, x2: i32) -> Vec<i32> {
    let mut result = Vec::new();
    if y1 == y2 {
        return vec![x1, x2];
    }

    let mut x = x1 as f32;
    let dx = (x2 - x1) as f32 / (y2 - y1) as f32;
    for _ in y1..=y2 {
        result.push(x as i32);
        x += dx;
    }
    result
}

// Fill a triangle using scanline algorithm
fn fill_triangle(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    v1: (i32, i32),
    v2: (i32, i32),
    v3: (i32, i32),
    color: Color,
) {
    let mut vertices = [v1, v2, v3];
    vertices.sort_by(|a, b| a.1.cmp(&b.1)); // Sort by y-coordinate

    let (x1, y1) = vertices[0];
    let (x2, y2) = vertices[1];
    let (x3, y3) = vertices[2];

    // Interpolate edges
    let mut left_edge = interpolate(y1, x1, y3, x3);
    let mut right_edge = interpolate(y1, x2, y2, x2);
    right_edge.pop();
    right_edge.append(&mut interpolate(y2, x2, y3, x3));

    // Draw horizontal lines between left and right edges
    for y in y1..=y3 {
        let xl = left_edge[(y - y1) as usize];
        let xr = right_edge[(y - y1) as usize];
        canvas.set_draw_color(color);
        canvas.draw_line((xl, y), (xr, y)).unwrap();
    }
}

// Convert a 3D point to 2D screen space (simple projection)
fn project(vertex: &Vector, width: f64, height: f64, fov: f64, zoom: f64) -> (i32, i32) {
    let scale = fov / (fov + vertex.z) + zoom ;
    let x_proj = vertex.x * scale + width / 2.0;
    let y_proj = vertex.y * scale + height / 2.0;
    (x_proj as i32, y_proj as i32)
}

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Create a window
    let window = video_subsystem
        .window("3D Triangle Rasterizer", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Define triangle vertices in 3D space
    let v1 = Vector { x: -1.0, y: -1.0, z: 5.0 };
    let v2 = Vector { x: 1.0, y: -1.0, z: 5.0 };
    let v3 = Vector { x: 0.0, y: 1.0, z: 5.0 };

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        // Clear the screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Project 3D points to 2D screen space
        let screen_v1 = Vector::project(&v1, 800.0, 600.0, 500.0, 100.0);
        let screen_v2 = Vector::project(&v2, 800.0, 600.0, 500.0, 100.0);
        let screen_v3 = Vector::project(&v3, 800.0, 600.0, 500.0, 100.0);

        // Fill the triangle
        fill_triangle(&mut canvas, screen_v1, screen_v2, screen_v3, Color::RGB(255, 0, 0));

        // Present the rendered frame
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}
/////////////////////////////