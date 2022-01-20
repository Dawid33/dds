#![allow(unused)]
extern crate simplelog;

use std::ffi::CString;
use std::mem;
use std::os::raw::*;
use std::ptr::{self, null};
use simplelog::*;
use log::*;
use glfw::*;
use vk_sys::*;

mod vulkan_utils;
mod window;
mod vulkan;
mod error;

static WINDOW_WIDTH : u32 = 800;
static WINDOW_HEIGHT : u32 = 600;

fn main () -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Logging utility
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();

    let mut window = window::WindowWrapper::new(WINDOW_WIDTH, WINDOW_HEIGHT)?;

    while !window.glfw_window.should_close() {
        window.glfw_instance.poll_events();
        for (_, event) in glfw::flush_messages(&window.glfw_window_events) {
            handle_window_event(&mut window.glfw_window, event);
        }
    }

    Ok(())
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}