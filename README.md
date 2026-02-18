# Plantower PMS7003 `#![no_std]` Driver

This crate contains code for controlling the Plantower
PMS7003 PM1.0/2.5/10 sensor in embedded environments.
It is platform-agnostic, which means that any microcontroller
with a HAL whose UART port implements [`embedded-io`](https://crates.io/crates/embedded-io)
traits can use this driver crate.

A potential update adding support for Linux (specifically
Raspberry Pi) may be added, but no promises.
