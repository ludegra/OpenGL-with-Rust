#[macro_export]
macro_rules! set_glfw_window_hints {
    ($glfw:ident, $($window_hint:expr),+) => {
        $(
            $glfw.window_hint($window_hint);
        )+
    };
}