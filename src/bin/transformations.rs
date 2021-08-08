use opengl_learning::set_glfw_window_hints;
use opengl_learning::shader_data::{
    buffer::Buffer,
    index::TriangleIndices,
    vertex::{Pos, PosTexVertex, Tex},
    vertex_array::VertexArray,
};
use opengl_learning::{
    gl_err_handeling::opengl_error_handling, shader::Shader, texture::Texture, window::Window,
};

use std::mem::size_of;

use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint, WindowMode};

use gl::types::*;
use gl::{Clear, ClearColor, DebugMessageCallback, DrawElements, PolygonMode, Viewport};

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    set_glfw_window_hints!(
        glfw,
        WindowHint::ContextVersionMajor(4),
        WindowHint::ContextVersionMinor(1),
        WindowHint::OpenGlProfile(OpenGlProfileHint::Core)
    );

    let mut window = Window::new(
        &mut glfw,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        "Gib me the sex",
        WindowMode::Windowed,
    );

    window.make_context_current();

    gl::load_with(|s| window.get_proc_address(s));

    unsafe {
        DebugMessageCallback(Some(opengl_error_handling), std::ptr::null());

        Viewport(0, 0, SCREEN_WIDTH as GLsizei, SCREEN_HEIGHT as GLsizei);
        PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
    }

    let shader = Shader::new(
        "assets/shaders/learning_transformations/default.vert",
        "assets/shaders/learning_transformations/default.frag",
        None,
    );

    let vertices = [
        PosTexVertex::new(Pos(-0.5, 0.5, 0.0), Tex(0.0, 1.0)), // top left
        PosTexVertex::new(Pos(0.5, 0.5, 0.0), Tex(1.0, 1.0)),  // top right
        PosTexVertex::new(Pos(-0.5, -0.5, 0.0), Tex(0.0, 0.0)), // bottom left
        PosTexVertex::new(Pos(0.5, -0.5, 0.0), Tex(1.0, 0.0)), // bottom right
    ];

    let indices = [TriangleIndices(0, 1, 3), TriangleIndices(0, 2, 3)];

    let vao = VertexArray::new();
    unsafe {
        vao.bind();
    }

    let vbo = Buffer::new(
        gl::ARRAY_BUFFER,
        (size_of::<PosTexVertex>() * vertices.len()) as GLsizeiptr,
        &vertices,
        gl::STATIC_DRAW,
    );
    let ebo = Buffer::new(
        gl::ELEMENT_ARRAY_BUFFER,
        (size_of::<PosTexVertex>() * vertices.len()) as GLsizeiptr,
        &indices,
        gl::STATIC_DRAW,
    );

    vao.set_attrib_pointers::<PosTexVertex>();

    let texture1 = Texture::new("assets/textures/Mrs. Immortal.jpg", gl::TEXTURE0);
    let texture2 = Texture::new("assets/textures/Jesus.jpg", gl::TEXTURE1);

    unsafe {
        shader.use_program();

        shader.set_int("texture1", 0);
        shader.set_int("texture2", 1);
    }

    let event_loop = |window: &mut Window| {
        unsafe {
            ClearColor(0.2, 0.3, 0.3, 1.0);
            Clear(gl::COLOR_BUFFER_BIT);
            texture1.activate_and_bind();
            texture2.activate_and_bind();
            shader.use_program();
            vao.bind();
            DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
        window.window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window.events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.window.set_should_close(true)
                }
                _ => {}
            }
        }
    };

    window.set_event_loop(event_loop);
}
