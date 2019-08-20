use raytracer_in_a_weekend::*;

extern crate crossbeam;

use cgmath::{vec3, ElementWise, InnerSpace, Vector3};
use glutin::event::{Event, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use rand::prelude::*;
use std::sync::Arc;

struct App {
    pixels: Vec<Pixel>,
    world: HitTableList<f64>,
    camera: Camera<f64>,
}

impl App {
    fn draw(&mut self, width: usize, height: usize) {
        const AA_STEPS: usize = 100;
        let mut i = 0usize;
        let (send, recv) = crossbeam::bounded(width * height);

        crossbeam::scope(|scope| {
            let world = &self.world;
            let camera = &self.camera;

            for y in (0..height).rev() {
                for x in 0..width {
                    let send = send.clone();
                    scope.spawn(move |_| {
                        let mut rng = rand::thread_rng();
                        let col = (0..AA_STEPS)
                            .map(|_i| {
                                let u = (x as f64 + rng.gen::<f64>()) / (width as f64);
                                let v = (y as f64 + rng.gen::<f64>()) / (height as f64);
                                camera.ray(u, v)
                            })
                            .fold(vec3(0.0, 0.0, 0.0), |acc, ray| acc + color(world, &ray, 0))
                            / AA_STEPS as f64;
                        send.send((
                            i,
                            Pixel {
                                r: (col.x.sqrt() * 255.99) as u8,
                                g: (col.y.sqrt() * 255.99) as u8,
                                b: (col.z.sqrt() * 255.99) as u8,
                            },
                        ))
                        .unwrap()
                    });

                    i += 1;
                }
            }
        })
        .expect("A child thread panicked");

        drop(send);

        for (i, pixel) in recv {
            self.pixels[i] = pixel;
        }
    }
}

fn color(world: &HitTableList<f64>, r: &Ray<f64>, depth: usize) -> Vector3<f64> {
    if depth < 50 {
        match world.hit(r, 0.001..std::f64::MAX) {
            None => {
                let unit_direction = r.direction().normalize();
                let t = 0.5 * (unit_direction.y + 1.0);
                (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
            }
            Some(hit) => {
                if let Some((attenuation, ray)) = hit.get_material().scatter(r, &hit) {
                    attenuation.mul_element_wise(color(world, &ray, depth + 1))
                } else {
                    vec3(0.0, 0.0, 0.0)
                }
            }
        }
    } else {
        vec3(0.0, 0.0, 0.0)
    }
}

fn gen_world(rng: &mut ThreadRng) -> HitTableList<f64> {
    let mut list = HitTableList::new();
    list.add(Box::new(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(vec3(0.5, 0.5, 0.5))),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = vec3(
                (a as f64) + 0.9 * rng.gen::<f64>(),
                0.2,
                (b as f64) + 0.9 * rng.gen::<f64>(),
            );
            if choose_mat < 0.8 {
                list.add(Box::new(Sphere::new(
                    center,
                    0.2,
                    Arc::new(Lambertian::new(vec3(
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                    ))),
                )));
            } else if choose_mat < 0.95 {
                list.add(Box::new(Sphere::new(
                    center,
                    0.2,
                    Arc::new(Metal::new(
                        vec3(
                            0.5 * (1.0 + rng.gen::<f64>()),
                            0.5 * (1.0 + rng.gen::<f64>()),
                            0.5 * (1.0 + rng.gen::<f64>()),
                        ),
                        0.5 * rng.gen::<f64>(),
                    )),
                )));
            } else {
                list.add(Box::new(Sphere::new(
                    center,
                    0.2,
                    Arc::new(Dielectric::new(1.5)),
                )));
            }
        }
    }
    list.add(Box::new(Sphere::new(
        vec3(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    list.add(Box::new(Sphere::new(
        vec3(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(vec3(0.4, 0.2, 0.1))),
    )));
    list.add(Box::new(Sphere::new(
        vec3(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(vec3(0.7, 0.6, 0.5), 0.0)),
    )));
    list
}

fn main() {
    const WIDTH: usize = 200;
    const HEIGHT: usize = 100;
    const WINDOW_SCALE: f64 = 4.0;

    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(
            WIDTH as f64 * WINDOW_SCALE,
            HEIGHT as f64 * WINDOW_SCALE,
        ));

    let windowed_context = {
        let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
        unsafe { windowed_context.make_current().unwrap() }
    };

    println!(
        "Pixel format of the window's GL context: {:?}",
        windowed_context.get_pixel_format()
    );

    let gl = support::load(&windowed_context.context());

    let mut app = {
        let mut rng = rand::thread_rng();
        let world = gen_world(&mut rng);

        let mut pixels: Vec<Pixel> = Vec::new();
        pixels.resize(WIDTH * HEIGHT, Pixel::default());

        let camera = {
            let origin = vec3(13.0, 2.0, 3.0);
            let look_at = vec3(0.0, 0.0, 0.0);
            Camera::new(
                origin,
                look_at,
                Vector3::unit_y(),
                20.0,
                WIDTH as f64 / HEIGHT as f64,
                0.1,
                10.0,
            )
        };

        App {
            pixels,
            world,
            camera,
        }
    };
    let start = std::time::Instant::now();
    app.draw(WIDTH, HEIGHT);
    let end = std::time::Instant::now();
    println!("Frame took {:?}ms", (end - start).as_millis());

    let texture = gl.new_texture(&app.pixels, WIDTH, HEIGHT);
    gl.write_pixels(texture, &app.pixels, WIDTH, HEIGHT);
    gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
    windowed_context.swap_buffers().unwrap();

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::RedrawRequested => {
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
