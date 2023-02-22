
extern crate glium;

use glium::glutin;
use glium::Surface;

fn main() {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {

        let mut target = display.draw();
        target.clear_color(0.5, 0.8, 1.0, 1.0);
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() + 
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            }, 
            _ => (),
        }
    });

}
