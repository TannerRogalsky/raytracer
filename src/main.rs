mod support;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, WindowedContext, PossiblyCurrent};

fn inner_size(windowed_context: &WindowedContext<PossiblyCurrent>) -> (usize, usize) {
    let dpi_factor = windowed_context.window().hidpi_factor();
    let size = windowed_context.window().inner_size().to_physical(dpi_factor);
    (size.width as usize, size.height as usize)
}

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("A fantastic window!");

    let windowed_context =
        ContextBuilder::new().build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    println!(
        "Pixel format of the window's GL context: {:?}",
        windowed_context.get_pixel_format()
    );

    let gl = support::load(&windowed_context.context());

    let (width, height) = inner_size(&windowed_context);
    let mut pixels : Vec<u8> = Vec::new();
    pixels.resize(width * height * 3, 0u8);
    let texture = gl.new_texture(&pixels, width, height);
    let mut frame = 0usize;

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        frame += 1;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor = windowed_context.window().hidpi_factor();
                    let size = logical_size.to_physical(dpi_factor);
                    windowed_context.resize(size);
                    pixels.resize(size.width as usize * size.height as usize * 3, 0);
                }
                WindowEvent::RedrawRequested => {
                    println!("{}", (frame % 128) as u8);
                    let (width, height) = inner_size(&windowed_context);
                    for i in 0..pixels.len() {
                        pixels[i] = (frame % 128) as u8;
                    }
                    gl.write_pixels(texture, &pixels, width, height);
                    gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                    windowed_context.swap_buffers().unwrap();
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            _ => (),
        }
    });
}