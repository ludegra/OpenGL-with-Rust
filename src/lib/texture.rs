use gl::types::*;
use gl::{ActiveTexture, BindTexture, GenTextures, GenerateMipmap, TexImage2D, TexParameteri};
use image::{ DynamicImage, ImageBuffer };

pub struct Texture {
    id: GLuint,
    texture_number: GLenum,
}

impl Texture {
    pub fn new(path: &str, texture_number: GLenum) -> Self {
        let data = image::open(path).unwrap().into_rgba8();
        let (width, height) = (data.width() as GLsizei, data.height() as GLsizei);

        let dyn_data = DynamicImage::ImageRgba8(data).flipv();
        let data = match dyn_data {
            DynamicImage::ImageRgba8(data) => data,
            _ => ImageBuffer::new(0, 0)
        };
        let data = data.into_raw();

        let mut id = 0;
        unsafe {
            GenTextures(1, &mut id);
            activate_and_bind_priv(texture_number, id);
            set_params();
            TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const GLvoid,
            );
            GenerateMipmap(gl::TEXTURE_2D);
        }

        Texture {
            id: id,
            texture_number: texture_number,
        }
    }
    pub unsafe fn activate_and_bind(&self) {
        activate_and_bind_priv(self.texture_number, self.id);
    }
}
unsafe fn activate_and_bind_priv(texture_number: GLenum, id: GLuint) {
    ActiveTexture(texture_number);
    BindTexture(gl::TEXTURE_2D, id);
}

unsafe fn set_params() {
    TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_WRAP_S,
        gl::REPEAT as GLint,
    );
    TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_WRAP_T,
        gl::REPEAT as GLint,
    );
    TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR_MIPMAP_LINEAR as GLint,
    );
    TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MAG_FILTER,
        gl::LINEAR as GLint
    );
}