
#![feature(macro_rules)]
#![allow(unused_variable)]

extern crate libc;
extern crate native;

use libc::{c_int, c_void, pid_t};
use std::c_str::CString;
use std::mem;
use std::os;
use std::ptr::mut_null;
use std::rt::exclusive::Exclusive;
use std::rt::rtio::{AddrinfoHint, AddrinfoInfo, Callback, CloseBehavior, EventLoop, FileAccess, FileMode, FileStat, IoFactory, IoError, IoResult, PausableIdleCallback, ProcessConfig, RemoteCallback, RtioFileStream, RtioPipe, RtioProcess, RtioSignal, RtioTcpListener, RtioTcpStream, RtioTimer, RtioTTY, RtioUdpSocket, RtioUnixListener, SocketAddr};
use std::sync::Arc;
use std::sync::atomics;

// Has to be before everything else
mod macros;

mod c;
mod event_loop;
mod io;

pub fn event_loop() -> Box<EventLoop + Send> {
    box event_loop::Loop::new() as Box<EventLoop + Send>
}

fn unimpl() -> IoError {
    IoError {
        code: libc::ERROR_CALL_NOT_IMPLEMENTED as uint,
        extra: 0,
        detail: Some("not yet supported by the `native` runtime, maybe try `green`.".to_string()),
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
