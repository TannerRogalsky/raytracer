mod support;

use glutin::event::{Event, WindowEvent};
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

#[repr(C)]
#[derive(Default, Clone, Debug)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

fn draw(p: &mut Vec<Pixel>, width: usize, height: usize) {
    let mut i = 0usize;
    for y in (0..height).rev() {
        for x in 0..width {
            let r = (y as f64) / (height as f64);
            let g = (x as f64) / (width as f64);
            let b = 0.2;
            p[i].r = (r * 255.99) as u8;
            p[i].g = (g * 255.99) as u8;
            p[i].b = (b * 255.99) as u8;
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
//    gl.write_pixels(texture, &pixels, width, height);
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
                _ => (),
            },
            _ => (),
        }
    });
}
