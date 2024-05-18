use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Ray Tracing Demo")
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                let frame = pixels.frame_mut();
                // Here you can manipulate each pixel in the `frame` buffer
                // For example, let's clear the screen with a color
                for pixel in frame.chunks_exact_mut(4) {
                    pixel.copy_from_slice(&[0x88, 0xb9, 0x39, 0xff]); // ARGB: Blue
                }
                pixels.render().unwrap();
            }
            _ => (),
        }
    });
}
