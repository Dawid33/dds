use std::error::Error;
use std::ffi::CString;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use std::sync::mpsc::Receiver;
use glfw::Callback;
use glfw::ErrorCallback;
use glfw::WindowEvent;
use simplelog::*;
use log::*;
use vk_sys::EntryPoints;
use vk_sys::InstancePointers;
use glfw::Context;

use crate::vulkan::{VulkanHandle, self};

pub struct WindowWrapper {
    pub vulkan_handle : VulkanHandle,
    pub glfw_window : glfw::Window,
    pub glfw_instance : glfw::Glfw,
    pub glfw_window_events : Receiver<(f64, WindowEvent)>,
}

impl WindowWrapper {
    pub fn new(width : u32, height : u32) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        // This callback is called when GLFW encounters an error.
        let callback : Option<ErrorCallback<()>> = Some(Callback {
            f : |err, err_string ,userdata| {
                // If using vulkan, GLFW throws a NoWindowContex error Saying that it cant create a window 
                // without OpenGL even though it can.
                if err != glfw::Error::NoWindowContext {
                    warn!("{}", err);
                }
            },
            data : (),
        });

        // GLFW instance
        let mut glfw = glfw::init(callback).unwrap();
        
        if glfw.vulkan_supported() {
            glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
        } else {
            panic!("Vulkan is not supported on this machine.");
        }
        
        let (mut window, events) = Self::create_and_init_window(width, height, &mut glfw)?;
        
        let vulkan_handle = Self::create_vulkan_handle(&mut window)?;
        
        Ok(Self {
            vulkan_handle,
            glfw_window : window,
            glfw_instance : glfw,
            glfw_window_events : events,
        })
    }

    fn create_and_init_window(width : u32, height : u32, glfw : &mut glfw::Glfw) -> Result<(glfw::Window, Receiver<(f64, WindowEvent)>), Box::<dyn Error>> {
        // Make sure the window is visible.
        glfw.window_hint(glfw::WindowHint::Visible(true));

        //FIXME - Remove this once I have figured out window resizing.
        glfw.window_hint(glfw::WindowHint::Resizable(false));

        let (mut window, events) = glfw.create_window(width, height, "DDS", glfw::WindowMode::Windowed)
            .unwrap_or_else(|| { 
                error!("Cannot create window with GLFW.");
                panic!();
            });
        window.set_key_polling(true);
        window.make_current();
        Ok((window,events))
    }

    fn create_vulkan_handle(window : &mut glfw::Window) -> Result<VulkanHandle, Box<dyn Error>> {
        let mut entry_points: EntryPoints = EntryPoints::load(|func| {
            window.get_instance_proc_address(0, func.to_str().unwrap()) as *const c_void
        });

        let vulkan_builder = unsafe {
            crate::vulkan::VulkanBuilder::new(&mut entry_points)
        }?;

        let mut instance_ptrs: InstancePointers = InstancePointers::load(|func| {
            window.get_instance_proc_address(vulkan_builder.get_instance(), func.to_str().unwrap()) as *const c_void
        });

        Ok(vulkan_builder.build(instance_ptrs))
    }
}

unsafe fn destroy_instance(instance: vk_sys::Instance, instance_ptrs: &mut vk_sys::InstancePointers) {
    
}