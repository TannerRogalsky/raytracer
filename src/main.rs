use raytracer_in_a_weekend::*;

use cgmath::{vec3, InnerSpace, Vector3};
use glutin::event::{Event, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, PossiblyCurrent, WindowedContext};
use rand::prelude::*;

fn inner_size(windowed_context: &WindowedContext<PossiblyCurrent>) -> (usize, usize) {
    let dpi_factor = windowed_context.window().hidpi_factor();
    let size = windowed_context
        .window()
        .inner_size()
        .to_physical(dpi_factor);
    (size.width as usize, size.height as usize)
}

struct App {
    pixels: Vec<Pixel>,
    world: HitTableList<f64>,
    camera: Camera<f64>,
    rng: ThreadRng,
}

impl App {
    fn color(&self, r: &Ray<f64>) -> Vector3<f64> {
        let rec = HitRecord::default();
        match self.world.hit(r, 0.0..std::f64::MAX, &rec) {
            None => {
                let unit_direction = r.direction().normalize();
                let t = 0.5 * (unit_direction.y + 1.0);
                (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
            }
            Some(hit) => {
                let n = hit.get_normal();
                vec3(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5
            }
        }
    }

    fn draw(&mut self, width: usize, height: usize) {
        let mut i = 0usize;
        for y in (0..height).rev() {
            for x in 0..width {
                const AA_STEPS: usize = 10;
                let col = (0..AA_STEPS).fold(vec3(0.0, 0.0, 0.0), |acc, _i| {
                    let u = (x as f64 + self.rng.gen::<f64>()) / (width as f64);
                    let v = (y as f64 + self.rng.gen::<f64>()) / (height as f64);

                    let r = self.camera.ray(u, v);
                    acc + self.color(&r)
                }) / AA_STEPS as f64;

                self.pixels[i].r = (col.x * 255.99) as u8;
                self.pixels[i].g = (col.y * 255.99) as u8;
                self.pixels[i].b = (col.z * 255.99) as u8;

                i += 1;
            }
        }
    }
}

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(glutin::dpi::LogicalSize::new(400.0, 200.0));

    let windowed_context = {
        let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
        unsafe { windowed_context.make_current().unwrap() }
    };

    println!(
        "Pixel format of the window's GL context: {:?}",
        windowed_context.get_pixel_format()
    );

    let gl = support::load(&windowed_context.context());

    let (width, height) = inner_size(&windowed_context);
    println!("Window inner size: {}, {}", width, height);

    let mut app = {
        let mut world = HitTableList::new();
        world.add(Box::new(Sphere::new(vec3(0.0, 0.0, -1.0), 0.5)));
        world.add(Box::new(Sphere::new(vec3(0.0, -100.5, -1.0), 100.0)));

        let mut pixels: Vec<Pixel> = Vec::new();
        pixels.resize(width * height, Pixel::default());

        App {
            pixels,
            world,
            camera: Camera::new(),
            rng: rand::thread_rng(),
        }
    };
    app.draw(width, height);

    let texture = gl.new_texture(&app.pixels, width, height);
    gl.write_pixels(texture, &app.pixels, width, height);
    gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
    windowed_context.swap_buffers().unwrap();

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor = windowed_context.window().hidpi_factor();
                    let size = logical_size.to_physical(dpi_factor);
                    let width = size.width as usize;
                    let height = size.height as usize;
                    windowed_context.resize(size);
                    app.pixels.resize(width * height, Pixel::default());
                }
                WindowEvent::RedrawRequested => {
                    //                    let (width, height) = inner_size(&windowed_context);
                    //                    app.draw(width, height);
                    //                    gl.write_pixels(texture, &app.pixels, width, height);
                    gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                    windowed_context.swap_buffers().unwrap();
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        if key == VirtualKeyCode::Escape {
                            *control_flow = ControlFlow::Exit
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    });
}
