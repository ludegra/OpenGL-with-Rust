use super::vertex::Vertex;

use std::mem::size_of;
use gl::types::*;
use gl::{
    GenVertexArrays, BindVertexArray, VertexAttribPointer, EnableVertexAttribArray
};

pub struct VertexArray {
    id: GLuint,
}
impl VertexArray {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            GenVertexArrays(1, &mut id);
        }

        Self { id }
    }
    pub fn set_attrib_pointers<T>(&self)
        where T: Vertex
    {
        unsafe {
            self.bind();
        }
        let definition = T::get_definition();
        let fields = definition.fields;
        let mut previous = 0;

        for i in 0..fields.len() {
            let field = &fields[i];
            let field_size = field.size;

            unsafe {
                VertexAttribPointer(
                    i as GLuint, 
                    field_size, 
                    gl::FLOAT,
                    gl::FALSE,
                    size_of::<T>() as GLsizei,
                    (previous * size_of::<GLfloat>()) as *const GLvoid
                );
                EnableVertexAttribArray(i as GLuint);
            }
            previous += field_size as usize;
        }
        unsafe {
            self.unbind();
        }
    }
    pub unsafe fn bind(&self) {
        BindVertexArray(self.id);
    }
    pub unsafe fn unbind(&self) {
        BindVertexArray(0);
    }
}