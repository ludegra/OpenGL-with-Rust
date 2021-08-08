use gl::types::*;

pub trait Index {}

pub struct TriangleIndices(pub GLuint, pub GLuint, pub GLuint);