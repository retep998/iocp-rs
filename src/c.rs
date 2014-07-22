// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;

pub type HANDLE = *mut VOID;
pub type VOID = libc::c_void;

pub static ERROR_ILLEGAL_CHARACTER: libc::c_int = 582;
pub static ENABLE_ECHO_INPUT: libc::DWORD = 0x4;
pub static ENABLE_EXTENDED_FLAGS: libc::DWORD = 0x80;
pub static ENABLE_INSERT_MODE: libc::DWORD = 0x20;
pub static ENABLE_LINE_INPUT: libc::DWORD = 0x2;
pub static ENABLE_PROCESSED_INPUT: libc::DWORD = 0x1;
pub static ENABLE_QUICK_EDIT_MODE: libc::DWORD = 0x40;

extern "system" {
    // FIXME - pInputControl should be PCONSOLE_READCONSOLE_CONTROL
    pub fn ReadConsoleW(
        hConsoleInput: libc::HANDLE,
        lpBuffer: libc::LPVOID,
        nNumberOfCharsToRead: libc::DWORD,
        lpNumberOfCharsRead: libc::LPDWORD,
        pInputControl: libc::LPVOID
    ) -> libc::BOOL;
    pub fn WriteConsoleW(
        hConsoleOutput: libc::HANDLE,
        lpBuffer: libc::types::os::arch::extra::LPCVOID,
        nNumberOfCharsToWrite: libc::DWORD,
        lpNumberOfCharsWritten: libc::LPDWORD,
        lpReserved: libc::LPVOID
    ) -> libc::BOOL;
    pub fn GetConsoleMode(
        hConsoleHandle: libc::HANDLE,
        lpMode: libc::LPDWORD
    ) -> libc::BOOL;
    pub fn SetConsoleMode(
        hConsoleHandle: libc::HANDLE,
        lpMode: libc::DWORD
    ) -> libc::BOOL;
}
