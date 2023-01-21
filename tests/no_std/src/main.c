/**
 * @file main.c
 * @author Matheus T. dos Santos (matheus.santos@edge.ufal.br)
 * @brief
 * @version 0.1
 * @date 03/04/2022
 *
 * @copyright Copyright (c) 2022
 *
 */
#include <zephyr/kernel.h>
#include <zephyr/zbus/zbus.h>

ZBUS_CHAN_DECLARE(pong);

void ping_sub_callback(const struct zbus_channel *chan) {
    int *ping_data = zbus_chan_const_msg(chan);
    printk("[c] Ping data: %d\n", *ping_data);
    *ping_data = (*ping_data * 2) + 1;
    zbus_chan_pub(&pong, ping_data, K_NO_WAIT);
}

ZBUS_LISTENER_DEFINE(ping_sub, ping_sub_callback
);

void main(void) {}
