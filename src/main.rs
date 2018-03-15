extern crate glutin;
extern crate gl;

mod support;

use glutin::GlContext;
use glutin::GlRequest;
use glutin::Api;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("A fantastic window!");
    let context = glutin::ContextBuilder::new(); // .with_gl(GlRequest::Specific(Api::OpenGlEs, (2, 0)));
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    let _ = unsafe { gl_window.make_current() };

    println!("Pixel format of the window's GL context: {:?}", gl_window.get_pixel_format());

    support::load(&gl_window);

    events_loop.run_forever(|event| {
        // println!("{:?}", event);
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => return glutin::ControlFlow::Break,
                glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                _ => (),
            },
            _ => ()
        }

        support::draw_frame([0.0, 1.0, 0.0, 1.0]);
        let _ = gl_window.swap_buffers();
        glutin::ControlFlow::Continue
    });
}
