
#[cfg(feature = "alloc_bug")]
use std::alloc::{GlobalAlloc, System, Layout};

// Required because rustc doesn't generate the alloc stuff right for IR output
#[cfg(feature = "alloc_bug")]
struct StdAlloc;

#[cfg(feature = "alloc_bug")]
unsafe impl GlobalAlloc for StdAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

#[allow(non_upper_case_globals)]
#[cfg(feature = "alloc_bug")]
#[no_mangle]
static __rust_no_alloc_shim_is_unstable: u8 = 0;

#[allow(non_upper_case_globals)]
#[cfg(feature = "alloc_bug")]
#[no_mangle]
static __rust_alloc_error_handler_should_panic: u8 = 0;

#[cfg(feature = "alloc_bug")]
#[no_mangle]
fn __rust_alloc_error_handler(_size: usize, _align: usize) -> ! {
    loop {

    }
}

#[cfg(feature = "alloc_bug")]
#[global_allocator]
static GLOBAL: StdAlloc = StdAlloc;