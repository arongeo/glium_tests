
extern crate glium;
extern crate image;

use glium::glutin;
use glium::Surface;
use glium::implement_vertex;
use glium::uniform;

use std::io::Cursor;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coordinates: [f32; 2],
}

impl Vertex {
    pub fn new(x_pos: f32, y_pos: f32, x_tex_coord: f32, y_tex_coord: f32) -> Self {
        Vertex {
            position: [x_pos, y_pos],
            tex_coordinates: [x_tex_coord, y_tex_coord],
        }
    }
}

fn main() {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("../res/obama.jpg")), image::ImageFormat::Jpeg).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    implement_vertex!(Vertex, position, tex_coordinates);


    let vertex1 = Vertex::new(-0.5, -0.5, 0.0, 0.55);
    let vertex2 = Vertex::new(0.0, 0.5, 0.2, 1.0);
    let vertex3 = Vertex::new(0.5, -0.25, 1.0, 0.55);

    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coordinates;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coordinates;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = -0.5;

    event_loop.run(move |event, _, control_flow| {

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

        t += 0.002;

        let mut target = display.draw();
        target.clear_color(0.5, 0.8, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            tex: &texture,
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

    });

}
