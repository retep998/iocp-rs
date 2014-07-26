
#![feature(macro_rules)]
#![allow(unused_variable)]

extern crate libc;
extern crate native;

use std::os;
use std::rt::rtio;
use std::rt::rtio::{IoError};

// Has to be before everything else
mod macros;

mod c;
mod event_loop;
mod io;

pub fn event_loop() -> Box<rtio::EventLoop + Send> {
    box event_loop::Loop::new() as Box<rtio::EventLoop + Send>
}

fn unimpl() -> IoError {
    IoError {
        code: libc::ERROR_CALL_NOT_IMPLEMENTED as uint,
        extra: 0,
        detail: Some("not yet supported by the `iocp` backend".to_string()),
    }
}

fn last_error() -> IoError {
    let errno = os::errno() as uint;
    IoError {
        code: os::errno() as uint,
        extra: 0,
        detail: Some(os::error_string(errno)),
    }
}
