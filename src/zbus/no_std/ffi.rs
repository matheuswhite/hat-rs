extern "C" {
    fn rs_strlen(string: *const u8) -> usize;
}

pub unsafe fn name_convert(c_str: *const u8) -> &'static str {
    let name_len = rs_strlen(c_str);
    let name_slice = core::slice::from_raw_parts(c_str, name_len);
    core::str::from_utf8_unchecked(name_slice)
}

#[repr(C)]
pub struct k_mutex {}

#[repr(C)]
pub struct sys_slist_t {}

#[repr(C)]
pub struct k_msgq {}

#[repr(C)]
pub struct CZbusChannel {
    pub(crate) name: *const u8,
    message_size: u16,
    pub(crate) user_data: *mut (),
    message: *mut (),
    validator: fn(msg: *const (), msg_size: usize) -> bool,
    mutex: *mut k_mutex,
    runtime_observers: *mut sys_slist_t,
    observers: *const *const CZbusObserver,
}

#[repr(C)]
pub struct CZbusObserver {
    pub(crate) name: *const u8,
    pub(crate) enabled: bool,
    queue: *mut k_msgq,
    callback: fn(chan: *const CZbusChannel),
}
