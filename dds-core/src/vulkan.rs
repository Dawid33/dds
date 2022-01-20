use std::{ptr::{null, null_mut}, ops::DerefMut};
use std::{mem, os::raw::c_void, ptr};
use crate::{vulkan_utils::*, error::BackendError};
use glfw::Context;
use vk_sys::*;
use log::*;

pub struct VulkanHandle {
    instance : vk_sys::Instance,
    instance_ptrs : vk_sys::InstancePointers,
}

pub struct VulkanHandleInitConfig {

}

pub struct VulkanBuilder {
    instance : vk_sys::Instance,
}

impl VulkanBuilder {
    pub unsafe fn new(entry_points : &mut vk_sys::EntryPoints) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        
        let mut instance: mem::MaybeUninit<vk_sys::Instance> = mem::MaybeUninit::uninit();

        let application_info = vk_sys::ApplicationInfo {
            sType: vk_sys::STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: null(),
            pApplicationName: "DDS".as_ptr() as *const i8,
            applicationVersion: make_api_version(0, 1, 0, 0),
            pEngineName: "No Engine".as_ptr() as *const i8,
            engineVersion: make_api_version(0, 1, 0, 0),
            apiVersion: make_api_version(0,1,0,0),
        };

        let info: InstanceCreateInfo = InstanceCreateInfo {
            sType: vk_sys::STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            pApplicationInfo: &application_info,
            enabledLayerCount: 0,
            ppEnabledLayerNames: ptr::null(),
            //These two should use the extensions returned by window.get_required_instance_extensions
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: ptr::null(),
        };

        let res: vk_sys::Result = entry_points.CreateInstance(
            &info as *const InstanceCreateInfo,
            ptr::null(),
            instance.as_mut_ptr(),
        );

        if res != vk_sys::SUCCESS {
            return Err(Box::new(BackendError{}));
        }
        
        Ok(Self {
            instance : instance.assume_init(),
        })
    }

    pub fn build(self, instance_ptrs : InstancePointers) -> VulkanHandle {
        VulkanHandle {
            instance : self.instance,
            instance_ptrs
        }
    }

    pub fn get_instance(&self) -> vk_sys::Instance {
        return self.instance;
    }
}

impl VulkanHandle {}

impl Drop for VulkanHandle {
    fn drop(&mut self) {
        unsafe {
            self.instance_ptrs.DestroyInstance(self.instance, ptr::null());
        }
    }
}