use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
mod shapes;
use shapes::{Color, Point, Ray, RayIntersectable, Shape, Sphere};

struct Camera {
    origin: Point,
    direction: Point,
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Ray Tracing Demo")
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();

    let camera = Camera {
        origin: Point {
            x: -200.0,
            y: 200.0,
            z: 0.0,
        },
        direction: Point {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let scene = vec![
        Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: 50.0,
            },
            radius: 100.0,
            shape: Shape {
                color: Color {
                    r: 200,
                    g: 5,
                    b: 1,
                    a: 255,
                },
                name: "Sphere".to_string(),
            },
        },
        Sphere {
            center: Point {
                x: 300.0,
                y: -250.0,
                z: 50.0,
            },
            radius: 100.0,
            shape: Shape {
                color: Color {
                    r: 200,
                    g: 5,
                    b: 200,
                    a: 255,
                },
                name: "Sphere".to_string(),
            },
        },
    ];

    let background_color = Color {
        r: 0x88,
        g: 0xb9,
        b: 0x39,
        a: 0xff,
    };

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                let frame = pixels.frame_mut();
                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let mut found_intersection = false;
                    let pixel_x = i as u32 % window_size.width;
                    let pixel_y = i as u32 / window_size.width;
                    // println!(
                    //     "Intersection found at pixel: x: {}, y: {}",
                    //     pixel_x, pixel_y
                    // );
                    let ray = Ray {
                        origin: Point {
                            x: (camera.origin.x + pixel_x as f64),
                            y: (camera.origin.y - pixel_y as f64),
                            z: (camera.origin.z),
                        },
                        direction: camera.direction,
                    };
                    for ray_intersectable in scene.iter() {
                        if let Some(_) = ray_intersectable.intersect_ray(&ray) {
                            println!("Intersection found at pixel: {},{}", pixel_x, pixel_y);
                            let color = &ray_intersectable.shape.color;
                            pixel.copy_from_slice(&[color.r, color.g, color.b, color.a]);
                            found_intersection = true;
                            break; // TODO: Implement Z-buffer
                        }
                    }
                    if found_intersection == false {
                        pixel.copy_from_slice(&[
                            background_color.r,
                            background_color.g,
                            background_color.b,
                            background_color.a,
                        ]); // ARGB: Blue
                    }
                    // pixel.copy_from_slice(&[(pixel_x % 255) as u8, (pixel_y % 255) as u8, 0x0, 0xff]); // ARGB: Blue
                }
                pixels.render().unwrap();
            }
            _ => (),
        }
    });
}
