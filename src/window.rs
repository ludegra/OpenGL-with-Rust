use std::sync::mpsc::Receiver;
use glfw::{Glfw, Window as GlfwWindow, WindowMode, GLProc, Context};

pub struct Window {
    pub window: GlfwWindow,
    pub events: Receiver<(f64, glfw::WindowEvent)>
}
impl Window {
    pub fn new(glfw: &mut Glfw, scr_width: u32, scr_height: u32, name: &str, window_mode: WindowMode) -> Self {
        let (window, events) = glfw.create_window(scr_width, scr_height, name, window_mode)
            .expect("Failed creating window");
        
        Self { window, events }
    }
    pub fn make_context_current(&mut self) {
        self.window.make_current()
    }
    pub fn get_proc_address(&mut self, procname: &str) -> GLProc {
        self.window.get_proc_address(procname)
    }
    pub fn set_event_loop<T>(&mut self, mut event_loop: T)
    where
        T: FnMut(&mut Window)
    {
        while !self.window.should_close(){
            event_loop(self);
        }
    }
}