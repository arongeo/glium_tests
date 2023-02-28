
#[macro_use]
extern crate glium;
extern crate image;
extern crate cgmath;

use cgmath::{Vector2, Matrix4};
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
    pub fn new(x_pos: f32, y_pos: f32, tex_pos_x: f32, tex_pos_y: f32) -> Self {
        Vertex {
            position: [x_pos, y_pos],
            tex_coordinates: [tex_pos_x, tex_pos_y],
        }
    }
}


const vertex_shader_src: &str = r#"
    #version 140

    in vec2 position;
    in vec2 tex_coordinates;
    out vec2 v_tex_coords;

    uniform mat4 matrix;

    void main() {
        if (gl_VertexID % 4 == 0) {
            v_tex_coords = vec2(0.0, 1.0);
        } else if (gl_VertexID % 4 == 1) {
            v_tex_coords = vec2(1.0, 1.0);
        } else if (gl_VertexID % 4 == 2) {
            v_tex_coords = vec2(0.0, 0.0);
        } else {
            v_tex_coords = vec2(1.0, 0.0);
        }
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;

const fragment_shader_src: &str = r#"
    #version 140

    in vec2 v_tex_coords;
    out vec4 color;

    uniform sampler2D tex;

    void main() {
        color = texture(tex, v_tex_coords);
    }
"#;

const WINDOW_WIDTH:  f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

fn main() {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new().with_inner_size(glutin::dpi::Size::Logical(glutin::dpi::LogicalSize { width: WINDOW_WIDTH as f64, height: WINDOW_HEIGHT as f64 })).with_title("asd");
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();
    
    implement_vertex!(Vertex, position, tex_coordinates);

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();


    let image_data = image::load(Cursor::new(&include_bytes!("../res/obama.jpg")), image::ImageFormat::Jpeg).unwrap().to_rgba8();
    let image_dimensions = image_data.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image_data.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();
    
    let rectangle_position = Vector2 {
        x: (WINDOW_WIDTH)  / 2.0,
        y: (WINDOW_HEIGHT) / 2.0,
    };

    let mut rectangle_size = Vector2 {
        x: 300.0,
        y: 300.0,
    };

    let vbuf = glium::VertexBuffer::empty_dynamic(&display, 4).unwrap();

    let indices_data: Vec<u16> = vec![0, 1, 2, 1, 2, 3];
    let ibuf = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices_data).unwrap();

    let matrix = Into::<[[f32; 4]; 4]>::into(cgmath::ortho(
        0.0,
        WINDOW_WIDTH,
        0.0,
        WINDOW_HEIGHT,
        -1.0,
        1.0
    ));

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

        // Time to calculateee
        let left = rectangle_position.x - rectangle_size.x / 2.0;
        let right = rectangle_position.x + rectangle_size.x / 2.0;
        let bottom = rectangle_position.y - rectangle_size.y / 2.0;
        let top = rectangle_position.y + rectangle_size.y / 2.0;
        vbuf.write(&vec![
            Vertex::new(left, top, 0.0, 0.0),
            Vertex::new(right, top, 0.0, 0.0),
            Vertex::new(left, bottom, 0.0, 0.0),
            Vertex::new(right, bottom, 0.0, 0.0),
        ]);


        let mut target = display.draw();
        target.clear_color(0.5, 0.8, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: matrix,
            tex: &texture,
        };

        target.draw(&vbuf, &ibuf, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

    });

}
