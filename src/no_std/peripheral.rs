#[cfg(not(feature = "std"))]
extern "C" {
    pub fn peripheral_open(peripheral_kind: usize, config: *const ()) -> usize;
    pub fn peripheral_close(peripheral_kind: usize, id: usize);
    pub fn peripheral_write(peripheral_kind: usize, id: usize, data: *const ());
    pub fn peripheral_read(peripheral_kind: usize, id: usize, data: *mut ());
}
