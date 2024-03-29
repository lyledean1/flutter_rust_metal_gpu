use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_get_value(port_: i64, example: *mut wire_uint_32_list) {
    wire_get_value_impl(port_, example)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_uint_32_list_0(len: i32) -> *mut wire_uint_32_list {
    let ans = wire_uint_32_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<Vec<u32>> for *mut wire_uint_32_list {
    fn wire2api(self) -> Vec<u32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_32_list {
    ptr: *mut u32,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
