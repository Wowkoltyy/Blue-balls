#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod vectors;

use std::cmp::{max, min};
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::vectors::{Vector2D, Vector3D};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const BOX_SIZE: i16 = 64;

const MAX_COLOR: f32 = 64f32;
const MIN_COLOR: f32 = 64f32;


fn check_overflow(color: u8, max_c: u8, min_c: u8) -> u8 {
    return u8::max(u8::min(color, max_c), min_c)
}

fn sphere(ro: Vector3D, rd: Vector3D, r: f32) -> Vector2D {
    let b = ro & rd;
    let c = (ro & ro) - r * r;
    let mut h = b * b - c;

    if h < 0.0 {
        return Vector2D{ x: -1.0, y: -1.0 }
    }
    h = f32::sqrt(h);
    Vector2D { x: -b - h, y: -b + h }
}

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    light: Vector3D,
    i: u32,
}



fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    })
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            light: Vector3D { x: -0.5, y: 0.5, z: -1.0 }.norm(),
            i: 0,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        self.light = Vector3D{x: f32::sin(self.i as f32 * 0.01), y: f32::cos(self.i as f32 * 0.01), z: -1.0};
        self.i = (self.i + 1) & 4294967295
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {

            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let uv: Vector2D = (Vector2D { x: x as f32, y: y as f32 } / Vector2D { x: WIDTH as f32, y: HEIGHT as f32 } * 2.0 - 1.0) * Vector2D {x: WIDTH as f32 / HEIGHT as f32, y: 1.0};
            let ro: Vector3D = Vector3D { x: -5.0, y: 0.0, z: 0.0 };
            let rd: Vector3D = Vector3D { x: 1.0, y: uv.x, z: uv.y }.norm();

            let ix1: f32 = sphere(ro, rd, 2.0).x;

            let ro2: Vector3D = Vector3D { x: -3.0, y: 2.0, z: -2.0 };
            let rd2: Vector3D = Vector3D { x: 1.0, y: uv.x, z: uv.y }.norm();

            let ix2: f32 = sphere(ro2, rd2, 2.0).x;

            let ix: f32 = f32::max(ix1, ix2);

            // Rgba8UnormSrgb
            // [0xRED, 0xGREEN, 0xBLUE, 0xALPHA]

            let rgba = if ix > 0.0 {
                let itPoint = ro + rd * ix;
                let n = itPoint.norm();
                let diff = n & self.light;
                let color = check_overflow((diff * 20.0) as i32 as u8, 0xff, 3);
                [0x00, 0x00, color, 0xff]
            } else {
                [0x00, 0x00, 0x00, 0xff]
            };


            pixel.copy_from_slice(&rgba);
        }
    }
}