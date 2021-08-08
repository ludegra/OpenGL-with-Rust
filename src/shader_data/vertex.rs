use gl::types::*;

pub struct VertexDefinition {
    pub fields: Vec<VertexFieldDefinition>,
}

pub struct VertexFieldDefinition {
    pub name: String,
    pub size: GLint,
}

pub trait Vertex {
    /// Returns the definition of the vertex fields
    fn get_definition() -> VertexDefinition;
}

pub trait VertexField {
    fn get_definition() -> VertexFieldDefinition;
}

pub struct Pos(pub GLfloat, pub GLfloat, pub GLfloat);
pub struct Col(pub GLfloat, pub GLfloat, pub GLfloat);
pub struct Tex(pub GLfloat, pub GLfloat);

impl VertexField for Pos {
    fn get_definition() -> VertexFieldDefinition {
        VertexFieldDefinition {
            name: "position".to_string(),
            size: 3

        }
    }
}
impl VertexField for Col {
    fn get_definition() -> VertexFieldDefinition {
        VertexFieldDefinition {
            name: "color".to_string(),
            size: 3
        }
    }
}
impl VertexField for Tex {
    fn get_definition() -> VertexFieldDefinition {
        VertexFieldDefinition {
            name: "texture position".to_string(),
            size: 2
        }
    }
}

/// A vertex with fields for position, color and texture coordinates
pub struct ColorTexVertex {
    _pos: Pos,
    _col: Col,
    _tex: Tex,
}

impl ColorTexVertex {
    /// Creates and returns a new instance of a ColorTexVertex
    pub fn new(_pos: Pos, _col: Col, _tex: Tex) -> Self {
        Self { _pos, _col, _tex }
    }
}

impl Vertex for ColorTexVertex {
    fn get_definition() -> VertexDefinition {
        let fields = vec![
            Pos::get_definition(),
            Col::get_definition(),
            Tex::get_definition(),
        ];
        VertexDefinition { fields }
    }
}

/// A vertex with fields for position and texture coordinates
pub struct PosTexVertex {
    _pos: Pos,
    _tex: Tex,
}

impl PosTexVertex {
    /// Creates a new instance of a PosTexVertex
    pub fn new(_pos: Pos, _tex: Tex) -> Self {
        Self { _pos, _tex }
    }
}

impl Vertex for PosTexVertex {
    fn get_definition() -> VertexDefinition {
        let fields = vec![
            Pos::get_definition(),
            Tex::get_definition(),
        ];
        VertexDefinition { fields }
    }
}