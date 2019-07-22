use raytracer_in_a_weekend::*;

use cgmath::{vec3, InnerSpace, Vector3};
use glutin::event::{Event, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, PossiblyCurrent, WindowedContext};

fn inner_size(windowed_context: &WindowedContext<PossiblyCurrent>) -> (usize, usize) {
    let dpi_factor = windowed_context.window().hidpi_factor();
    let size = windowed_context
        .window()
        .inner_size()
        .to_physical(dpi_factor);
    (size.width as usize, size.height as usize)
}

type Vecf = Vector3<f64>;

fn unit_vector(v: &Vecf) -> Vecf {
    v / v.magnitude()
}

fn hit_sphere(center: &Vecf, radius: f64, r: &Ray<f64>) -> f64 {
    let oc: Vecf = r.origin() - center;
    let a = r.direction().dot(r.direction().to_owned());
    let b = 2.0 * oc.dot(r.direction().to_owned());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn color(r: &Ray<f64>) -> Vecf {
    {
        let sphere_center = vec3(0.0, 0.0, -1.0);
        let t = hit_sphere(&sphere_center, 0.5, r);
        if t > 0.0 {
            let v = r.point_at_parameter(t) - vec3(0.0, 0.0, -1.0);
            let n = unit_vector(&v);
            return vec3(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
        }
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
}

fn draw(p: &mut Vec<Pixel>, width: usize, height: usize) {
    let lower_left_corner = vec3(-2.0, -1.0, -1.0);
    let horizontal = vec3(4.0, 0.0, 0.0);
    let vertical = vec3(0.0, 2.0, 0.0);
    let origin = vec3(0.0, 0.0, 0.0);

    let mut i = 0usize;
    for y in (0..height).rev() {
        for x in 0..width {
            let u = (x as f64) / (width as f64);
            let v = (y as f64) / (height as f64);

            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = color(&r);

            p[i].r = (col.x * 255.99) as u8;
            p[i].g = (col.y * 255.99) as u8;
            p[i].b = (col.z * 255.99) as u8;

            i += 1;
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
    let mut pixels: Vec<Pixel> = Vec::new();
    pixels.resize(width * height, Pixel::default());
    draw(&mut pixels, width, height);
    let texture = gl.new_texture(&pixels, width, height);
    gl.write_pixels(texture, &pixels, width, height);
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
                    pixels.resize(width * height, Pixel::default());
                }
                WindowEvent::RedrawRequested => {
                    let (width, height) = inner_size(&windowed_context);
                    draw(&mut pixels, width, height);
                    gl.write_pixels(texture, &pixels, width, height);
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
