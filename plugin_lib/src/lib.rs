#[repr(C)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[macro_export]
macro_rules! version {
    () => {
        
            pub use plugin_lib::{Version, __api_version_inner};
            #[no_mangle]
            pub extern "C" fn __api_version() -> *const Version {
                __api_version_inner()
            }
        
    };
}

#[inline]
pub fn __api_version_inner() -> *const Version {
    let version = Version {
        major: 0,
        minor: 0,
        patch: 0,
    };
    let layout = std::alloc::Layout::new::<Version>();
    let ptr = unsafe { std::alloc::alloc(layout) };
    unsafe { ptr.cast::<Version>().write(version) };
    ptr as _
}
