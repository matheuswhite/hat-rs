extern crate hat;

extern "C" {
    #[link_name = "ping"]
    pub static ping_c_ref: hat::zbus::struct_zbus_channel;
    #[link_name = "pong"]
    pub static pong_c_ref: hat::zbus::struct_zbus_channel;
}
