use core::alloc::{GlobalAlloc, Layout};

#[repr(C, align(256))]
pub struct MempoolAlloc;

#[cfg(feature = "alloc_rtos")]
#[global_allocator]
static MEMPOOL_ALLOC: MempoolAlloc = MempoolAlloc;

unsafe impl Sync for MempoolAlloc {}

unsafe impl GlobalAlloc for MempoolAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = heap_alloc(layout.align(), layout.size()) as *mut u8;
        if ret as usize & (layout.align() - 1) != 0 {
            panic!("Rust unsatisfied alloc alignment\n");
        } else {
            ret
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        heap_free(ptr as *mut ());
    }
}

#[alloc_error_handler]
fn alloc_error_handler(_layout: Layout) -> ! {
    panic!("allocation error!!!!!\n")
}

extern "C" {
    fn heap_alloc(align: usize, bytes: usize) -> *const ();
    fn heap_free(mem: *const ());
}
