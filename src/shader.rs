use gl::types::*;
use gl::{
    CompileShader, CreateShader, GetProgramInfoLog, GetProgramiv, GetShaderInfoLog, GetShaderiv,
    ShaderSource, CreateProgram, AttachShader, LinkProgram, DeleteShader, UseProgram,
    GetUniformLocation, Uniform1i, Uniform1f
};
use std::{ffi::CString, fs::File, io::prelude::*};

pub struct Shader {
    id: GLuint,
}
impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str, geometry_path: Option<&str>) -> Self {
        // Reads code from paths provided
        let vert_code = read_shader_file(vertex_path, "vertex");
        let frag_code = read_shader_file(fragment_path, "fragment");

        // Returns an Option<String> with code if geometry shader path is provided, othervise Null
        let geom_code = match geometry_path {
            Some(geometry_path) => Some(read_shader_file(geometry_path, "geometry")),
            None => None,
        };

        // Creates and compiles shaders from code
        let (vertex, fragment);
        unsafe {
            vertex = create_shader(vert_code, gl::VERTEX_SHADER);
            fragment = create_shader(frag_code, gl::FRAGMENT_SHADER);
        }
        // Creates and compiles geometry shader if present
        let geometry = match geom_code {
            Some(geom_code) => {
                let geometry;
                unsafe {
                    geometry = create_shader(geom_code, gl::GEOMETRY_SHADER)
                }
                Some(geometry)
            }
            None => None
        };

        // Creates a program
        let id: GLuint;
        unsafe {
            id = CreateProgram();

            // Attaches shaders to program
            AttachShader(id, vertex);
            AttachShader(id, fragment);
            if let Some(geometry) = geometry {
                AttachShader(id, geometry);
            }
            // Links and checks if program is valid
            LinkProgram(id);
            program_checker(id);

            // Deletes shaders as they are no longer needed
            DeleteShader(vertex);
            DeleteShader(fragment);
            if let Some(geometry) = geometry {
                DeleteShader(geometry);
            }
        }

        Self { id }
    }
    pub unsafe fn use_program(&self) {
        UseProgram(self.id);
    }
    pub unsafe fn set_bool(&self, name: &str, value: bool) {
        let name_cstring = std::ffi::CString::new(name).unwrap();
        Uniform1i(GetUniformLocation(self.id, name_cstring.as_ptr()), value as GLint);
    }
    pub unsafe fn set_int(&self, name: &str, value: GLint) {
        let name_cstring = std::ffi::CString::new(name).unwrap();
        Uniform1i(GetUniformLocation(self.id, name_cstring.as_ptr()), value);
    }
    pub unsafe fn set_float(&self, name: &str, value: GLfloat) {
        let name_cstring = std::ffi::CString::new(name).unwrap();
        Uniform1f(GetUniformLocation(self.id, name_cstring.as_ptr()), value);
    }
}
fn read_shader_file(path: &str, shader_name: &str) -> CString {
    // fromatting error messanges
    let file_err_msg = format!("Failed opening {} file", shader_name);
    let c_string_err_msg = format!("Failed to convert {} code to CString", shader_name);

    // Reading file
    let mut file = File::open(path).expect(&file_err_msg);
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();

    // Converting code to a CString and returning the result
    CString::new(code).expect(&c_string_err_msg)
}

unsafe fn create_shader(shader_code: CString, shader_type: GLenum) -> GLuint {
    let shader = CreateShader(shader_type);
    ShaderSource(shader, 1, &shader_code.as_ptr(), std::ptr::null());
    CompileShader(shader);
    shader_compilation_checker(shader);
    shader
}

pub unsafe fn shader_compilation_checker(shader: u32) {
    let mut success = 0;
    GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

    if success == 0 {
        let mut len = 0;
        GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

        let mut buffer = Vec::with_capacity(len as usize + 1);
        buffer.extend([b' '].iter().cycle().take(len as usize));
        let err = std::ffi::CString::from_vec_unchecked(buffer);

        GetShaderInfoLog(
            shader,
            512,
            std::ptr::null_mut(),
            err.as_ptr() as *mut GLchar,
        );
        println!("{}", err.to_string_lossy());
    }
}

pub unsafe fn program_checker(program: u32) {
    let mut success = 0;
    GetProgramiv(program, gl::COMPILE_STATUS, &mut success);

    if success == 0 {
        let mut len = 0;
        GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

        let mut buffer = Vec::with_capacity(len as usize + 1);
        buffer.extend([b' '].iter().cycle().take(len as usize));
        let err = std::ffi::CString::from_vec_unchecked(buffer);

        GetProgramInfoLog(
            program,
            512,
            std::ptr::null_mut(),
            err.as_ptr() as *mut GLchar,
        );
        println!("{}", err.to_string_lossy());
    }
}
