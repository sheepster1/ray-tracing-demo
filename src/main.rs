use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder
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
        .with_inner_size(LogicalSize::new(1920, 1080))
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();

    let camera = Camera {
        origin: Point {
            x: 0.0,
            y: 0.0,
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
            id: 1,
            center: Point {
                x: -150.0,
                y: 0.0,
                z: 100.0,
            },
            radius: 150.0,
            shape: Shape {
                color: Color {
                    r: 0xe3,
                    g: 0x03,
                    b: 0xbc,
                    a: 255,
                },
                name: "Pink".to_string(),
            },
        },
        Sphere {
            id: 2,
            center: Point {
                x: 60.0,
                y: 0.0,
                z: 140.0,
            },
            radius: 60.0,
            shape: Shape {
                color: Color {
                    r: 0x03,
                    g: 0x52,
                    b: 0xfc,
                    a: 255,
                },
                name: "Blue".to_string(),
            },
        },
        Sphere {
            id: 3,
            center: Point {
                x: 195.0,
                y: 0.0,
                z: 71.0,
            },
            radius: 70.0,
            shape: Shape {
                color: Color {
                    r: 0x03,
                    g: 0xfc,
                    b: 0x1f,
                    a: 255,
                },
                name: "Green".to_string(),
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
                    let pixel_x = i as u32 % window_size.width;
                    let pixel_y = i as u32 / window_size.width;
                    let mut ray = Ray {
                        origin: Point {
                            x: camera.origin.x + pixel_x as f64 - (window_size.width as f64 / 2.0),
                            y: camera.origin.y + pixel_y as f64 - (window_size.height as f64 / 2.0),
                            z: camera.origin.z,
                        },
                        direction: camera.direction,
                    };
                    let mut ray_origin_intersectable: Option<&Sphere> = None;
                    let mut color: Option<Color> = None;
                    let max_ray_bounce = 5;
                    let mut has_intersection = false;
                    for _ in 0..max_ray_bounce {
                        for ray_intersectable in scene.iter() {
                            let origin_intersectable_id = match ray_origin_intersectable {
                                Some(s) => s.id,
                                None => ray_intersectable.id + 1,
                            };
                            if ray_intersectable.id == origin_intersectable_id {
                                continue;
                            }
                            if let Some(intersection_point) = ray_intersectable.intersect_ray(&ray)
                            {
                                has_intersection = true;
                                let mut normal_vector: Point = Point {
                                    x: intersection_point.x - ray_intersectable.center.x,
                                    y: intersection_point.y - ray_intersectable.center.y,
                                    z: intersection_point.z - ray_intersectable.center.z,
                                };
                                normal_vector.normalize_vector();
                                let mut refraction_vector: Point = Point {
                                    x: ray.direction.x - 2.0 * normal_vector.x,
                                    y: ray.direction.y - 2.0 * normal_vector.y,
                                    z: ray.direction.z - 2.0 * normal_vector.z,
                                };
                                refraction_vector.normalize_vector();

                                ray = Ray {
                                    origin: intersection_point,
                                    direction: refraction_vector,
                                };
                                ray_origin_intersectable = Some(&ray_intersectable);
                                color = Some(Color {
                                    r: (ray_intersectable.shape.color.r / 3)
                                        + (color.unwrap_or(ray_intersectable.shape.color).r / 3)
                                            * 2,
                                    g: (ray_intersectable.shape.color.g / 3)
                                        + (color.unwrap_or(ray_intersectable.shape.color).g / 3)
                                            * 2,
                                    b: (ray_intersectable.shape.color.b / 3)
                                        + (color.unwrap_or(ray_intersectable.shape.color).b / 3)
                                            * 2,
                                    a: (ray_intersectable.shape.color.a / 3)
                                        + (color.unwrap_or(ray_intersectable.shape.color).a / 3)
                                            * 2,
                                });
                                break;
                            }
                        }
                    }
                    if !has_intersection {
                        color = Some(background_color);
                    }
                    pixel.copy_from_slice(&[
                        color.unwrap().r,
                        color.unwrap().g,
                        color.unwrap().b,
                        color.unwrap().a,
                    ]);
                    // for ray_intersectable in scene.iter() {
                    //     if let Some(intersection_point) = ray_intersectable.intersect_ray(&ray) {
                    //         let normal_vector: Point = Point {
                    //             x: intersection_point.x - ray_intersectable.center.x,
                    //             y: intersection_point.y - ray_intersectable.center.y,
                    //             z: intersection_point.z - ray_intersectable.center.z,
                    //         };
                    //         let reflection_vector: Point = Point {
                    //             x: ray.direction.x - 2.0 * normal_vector.x,
                    //             y: ray.direction.y - 2.0 * normal_vector.y,
                    //             z: ray.direction.z - 2.0 * normal_vector.z,
                    //         };
                    //         println!("Intersection found at pixel: {},{}. Intersection is at {},{},{}", pixel_x, pixel_y, intersection_point.x, intersection_point.y, intersection_point.z);
                    //         for ray_intersectable2 in scene.iter() {
                    //             if ray_intersectable2.id == ray_intersectable.id{
                    //                 continue;
                    //             }

                    //             if let Some(intersection_point2) = ray_intersectable2.intersect_ray(&Ray {origin: intersection_point, direction: reflection_vector}) {

                    //                 println!("Intersection found 2at pixel: {},{}. Intersection is at {},{},{}", pixel_x, pixel_y, intersection_point2.x, intersection_point2.y, intersection_point2.z);
                    //             pixel.copy_from_slice(&[
                    //                 (ray_intersectable.shape.color.r /4) + (ray_intersectable2.shape.color.r/4)*3,
                    //                 (ray_intersectable.shape.color.g /4) + (ray_intersectable2.shape.color.g/4)*3,
                    //                 (ray_intersectable.shape.color.b /4) + (ray_intersectable2.shape.color.b/4)*3,
                    //                 (ray_intersectable.shape.color.a /4) + (ray_intersectable2.shape.color.a/4)*3]
                    //             );
                    //             // pixel.copy_from_slice(&[(pixel_x % 255) as u8, (pixel_y % 255) as u8, 0x0, 0xff]); // ARGB: Blue

                    //             found_intersection=true
                    //             }
                    //         }
                    //         if found_intersection {
                    //             break;
                    //         }
                    //         let color = &ray_intersectable.shape.color;
                    //         // pixel.copy_from_slice(&[color.r, color.g, color.b, color.a]);
                    //         pixel.copy_from_slice(&[
                    //             (background_color.r /4) + (color.r/4)*3,
                    //             (background_color.g /4) + (color.g/4)*3,
                    //             (background_color.b /4) + (color.b/4)*3,
                    //             (background_color.a /4) + (color.a/4)*3]
                    //         );
                    //         found_intersection = true;

                    //         break; // TODO: Implement Z-buffer
                    //     }
                    // }
                    // if found_intersection == false {
                    //     pixel.copy_from_slice(&[
                    //         background_color.r,
                    //         background_color.g,
                    //         background_color.b,
                    //         background_color.a,
                    //     ]); // ARGB: Blue
                    // }
                    // pixel.copy_from_slice(&[(pixel_x % 255) as u8, (pixel_y % 255) as u8, 0x0, 0xff]); // ARGB: Blue
                }
                pixels.render().unwrap();
            }
            _ => (),
        }
    });
}
