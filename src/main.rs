
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


struct TilePos {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

fn mul_matrices(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut matrix: [[f32; 4]; 4] = [
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0f32],
    ];
    for i in 0..4 {
        for j in 0..4 {
            matrix[i][j] = a[i][j] * b[j][i];
        }
    }
    return matrix;
}

const vertex_shader_src: &str = r#"
    #version 140

    in vec2 position;
    in vec2 tex_coordinates;
    out vec2 v_tex_coords;

    uniform mat4 model_matrix;
    uniform mat4 ortho_matrix;

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
        gl_Position = ortho_matrix * model_matrix * vec4(position, 0.0, 1.0);
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
    
    let TILE_NUM: usize = ((WINDOW_HEIGHT / 40.0).ceil() * (WINDOW_WIDTH / 40.0).ceil()) as usize;

    let mut rectangle_size = Vector2 {
        x: 40.0,
        y: 40.0,
    };

    let vbuf = glium::VertexBuffer::empty_dynamic(&display, 4).unwrap();

    let mut indices_data: Vec<u16> = vec![0, 1, 2, 1, 3, 2];
    let ibuf = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices_data).unwrap();

    let ortho_matrix = Into::<[[f32; 4]; 4]>::into(cgmath::ortho(
        0.0,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        0.0,
        -1.0,
        1.0
    ));

    let mut pos_matrix: [[f32; 4]; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    
    vbuf.write(&vec![
        Vertex::new(0.0, 0.0, 0.0, 0.0),
        Vertex::new(40.0, 0.0, 0.0, 0.0),
        Vertex::new(0.0, 40.0, 0.0, 0.0),
        Vertex::new(40.0, 40.0, 0.0, 0.0),
    ]);

    let mut timer = std::time::Instant::now();
    let mut frames = 0;
    let mut frame_changed = true;
    event_loop.run(move |event, _, control_flow| {

        let next_frame_time = std::time::Instant::now() + 
            std::time::Duration::from_nanos(20_666_667);
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

        if (timer.elapsed().as_secs() >= 1) {
            println!("Frames drawn in 1 sec: {}", frames+1);
        } else {
            frames += 1;
        }

        // FIX THIS, CAP FRAMERATE INSTEAD
        if frame_changed == false {
            return;
        }

        let mut target = display.draw();

        for i in 0..((WINDOW_WIDTH / 40.0).ceil() as usize) {   
            for j in 0..((WINDOW_HEIGHT / 40.0).ceil() as usize) {       
                let pos_matrix = [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [(i as f32) * 40.0, (j as f32) * 40.0, 0.0, 1.0],
                ];

                let uniforms = uniform! {
                    ortho_matrix: ortho_matrix,
                    model_matrix: pos_matrix,
                    tex: &texture,
                };

                target.draw(&vbuf, &ibuf, &program, &uniforms, &Default::default()).unwrap();
            }
        }

        target.finish().unwrap();
        println!("Frame drawn!");
        frame_changed = false;
    });

}
