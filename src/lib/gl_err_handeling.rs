use gl::types::*;

#[no_mangle]
pub extern "system" fn opengl_error_handling(
    source: GLenum,
    kind: GLenum,
    id: GLuint,
    severity: GLenum,
    _: GLsizei,
    message: *const GLchar,
    _: *mut GLvoid,
) {
    use colored::Colorize;

    let msg = unsafe { std::ffi::CStr::from_ptr(message) };
    let msg = msg.to_string_lossy();
    println!(
        "{}{} {:#X} {} {:#X} {} {:#X} {} {:#X} {} {}",
        "OpenGL Error:\n\t",
        "Source:".green(),
        source,
        "Kind:".green(),
        kind,
        "Id:".green(),
        id,
        "Severity:".green(),
        severity,
        "\n\tMessage:".green(),
        msg.red()
    );
}