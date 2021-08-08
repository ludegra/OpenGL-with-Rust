use super::vertex::Vertex;
use super::index::Index;

use gl::types::*;
use gl::{
    GenBuffers, BindBuffer, BufferData
};

pub struct Buffer {
    id: GLuint,
    buffer_type: GLenum,
}

impl Buffer {
    pub fn new<T>(buffer_type: GLenum, vertices_len: GLsizeiptr, data: &[T], draw_mode: GLenum) -> Self {
        let mut id = 0;

        unsafe {
            GenBuffers(1, &mut id);
            BindBuffer(buffer_type, id);
            BufferData(
                buffer_type,
                vertices_len,
                data.as_ptr() as *const GLvoid,
                draw_mode
            );
        }

        Self { id, buffer_type }
    }
    pub unsafe fn bind(&self) {
        BindBuffer(self.buffer_type, self.id);
    }
    pub unsafe fn unbind(&self) {
        BindBuffer(self.buffer_type, 0);
    }
}