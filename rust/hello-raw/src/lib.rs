use std::alloc::{alloc, dealloc, Layout};
use std::mem;

pub extern "C" fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[export_name = "greet"]
pub extern "C" fn __greet_wrapper(
    arg0_ptr: *const u8,
    arg0_len: usize,
) -> *mut String {
    let arg0 = unsafe {
        // This gives us a &[u8] using the pointer and length.
        let slice = std::slice::from_raw_parts(arg0_ptr, arg0_len);
        // This converts &[u8] to &str, telling the compiler that it does not need to
        // check if all characters are UTF-8 -- we guarantee it ourselves.
        std::str::from_utf8_unchecked(slice)
    };
    let _ret = greet(arg0);
    // Here we create a new heap allocation by using a Box that we then consume, thus
    // returning the raw pointer to the heap allocated String.
    Box::into_raw(Box::new(_ret))
}

#[no_mangle]
pub extern "C" fn __malloc(size: usize) -> *mut u8 {
    // `align` is the minimum alignment for a `usize` based on the ABI.
    let align = mem::align_of::<usize>();
    // We attempt to generate a memory layout for the particular size/alignment.
    if let Ok(layout) = Layout::from_size_align(size, align) {
        unsafe {
            if layout.size() > 0 {
                // Allocate the memory.
                let ptr = alloc(layout);
                // A null pointer likely means we're OOM.
                if !ptr.is_null() {
                    return ptr
                }
            } else {
                // Cast and return if the size of the layout isn't positive.
                return align as *mut u8
            }
        }
    }

    // NOTE: by panicing here we increase our Wasm's module size.
    panic!("malloc failed")
}

#[no_mangle]
pub unsafe extern "C" fn __free(ptr: *mut u8, size: usize) {
    if size == 0 {
        return
    }
    let align = mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(size, align);
    dealloc(ptr, layout);
}

#[no_mangle]
pub unsafe extern "C" fn __boxed_str_free(ptr: *mut String) {
    // Since the Box will be the sole owner of the String, the String will be dropped.
    let _b = Box::from_raw(ptr);
}
