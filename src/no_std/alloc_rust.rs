use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr::null_mut;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::SeqCst;
#[cfg(not(feature = "std"))]
use crate::mc_panic;
#[cfg(not(feature = "std"))]
use crate::no_std::panic;
#[cfg(not(feature = "std"))]
use const_format::formatcp;

const ARENA_SIZE: usize = 8 * 1024;
const MAX_SUPPORTED_ALIGN: usize = 256;

#[repr(C, align(256))]
pub struct ArenaAlloc {
    arena: UnsafeCell<[u8; ARENA_SIZE]>,
    remaining: AtomicUsize,
}

#[cfg(feature = "alloc_rust")]
#[global_allocator]
static ALLOCATOR: ArenaAlloc = ArenaAlloc {
    arena: UnsafeCell::new([0x55; ARENA_SIZE]),
    remaining: AtomicUsize::new(ARENA_SIZE),
};

unsafe impl Sync for ArenaAlloc {}

unsafe impl GlobalAlloc for ArenaAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
        // So we can safely use a mask to ensure alignment without worrying about UB.
        let align_mask_to_round_down = !(align - 1);

        if align > MAX_SUPPORTED_ALIGN {
            return null_mut();
        }

        let mut allocated = 0;
        if self
            .remaining
            .fetch_update(SeqCst, SeqCst, |mut remaining| {
                if size > remaining {
                    return None;
                }
                remaining -= size;
                remaining &= align_mask_to_round_down;
                allocated = remaining;
                Some(remaining)
            })
            .is_err()
        {
            return null_mut();
        };
        (self.arena.get() as *mut u8).add(allocated)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[alloc_error_handler]
fn alloc_error_handler(_layout: Layout) -> ! {
    mc_panic!("allocation error!!!!!\n");
    loop {}
}
