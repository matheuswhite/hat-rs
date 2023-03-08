#![no_std]
#![no_builtins]

mod zbus_channels;

extern crate panic_halt;
extern crate hat;
extern crate hat_macros;
extern crate alloc;

use core::time::Duration;
use hat::*;
use hat::zbus::WorkQueueBackend;
use crate::zbus_channels::{ping_c_ref, pong_c_ref};

type ZbusChannel<T> = zbus::Channel<T, WorkQueueBackend>;

#[hat_macros::main]
#[hat_macros::tasks(ping_task)]
pub async fn pong_task() -> TaskResult {
    let pong_channel = unsafe { ZbusChannel::<i32>::load(&pong_c_ref) };
    let pong_sub = pong_channel.new_subscriber();

    for _ in 0..3 {
        let pong_data = pong_sub.wait_msg().await;
        log!("pong_data: %d", pong_data);
    }

    Ok(())
}

pub async fn ping_task() -> TaskResult {
    let ping_channel = unsafe { ZbusChannel::load(&ping_c_ref) };

    for i in 0..3 {
        ping_channel.publish(i).await;
        log!("Waiting 1 sec...");
        delay(Duration::from_millis(1000)).await;
        log!("1 sec was done");
    }

    Ok(())
}
