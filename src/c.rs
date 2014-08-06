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

pub type BOOL = libc::c_int;
pub type HANDLE = *mut VOID;
pub type VOID = libc::c_void;

pub static ERROR_ILLEGAL_CHARACTER: uint = 582;
pub static ERROR_INVALID_HANDLE: uint = 6;
pub static ENABLE_ECHO_INPUT: libc::DWORD = 0x4;
pub static ENABLE_EXTENDED_FLAGS: libc::DWORD = 0x80;
pub static ENABLE_INSERT_MODE: libc::DWORD = 0x20;
pub static ENABLE_LINE_INPUT: libc::DWORD = 0x2;
pub static ENABLE_PROCESSED_INPUT: libc::DWORD = 0x1;
pub static ENABLE_QUICK_EDIT_MODE: libc::DWORD = 0x40;

extern "system" {
    pub fn CloseHandle(
        hObject: HANDLE,
    ) -> BOOL;
    // FIXME - pInputControl should be PCONSOLE_READCONSOLE_CONTROL
    pub fn ReadConsoleW(
        hConsoleInput: HANDLE,
        lpBuffer: libc::LPVOID,
        nNumberOfCharsToRead: libc::DWORD,
        lpNumberOfCharsRead: libc::LPDWORD,
        pInputControl: libc::LPVOID,
    ) -> BOOL;
    pub fn WriteConsoleW(
        hConsoleOutput: HANDLE,
        lpBuffer: libc::types::os::arch::extra::LPCVOID,
        nNumberOfCharsToWrite: libc::DWORD,
        lpNumberOfCharsWritten: libc::LPDWORD,
        lpReserved: libc::LPVOID,
    ) -> BOOL;
    pub fn GetConsoleMode(
        hConsoleHandle: HANDLE,
        lpMode: libc::LPDWORD,
    ) -> BOOL;
    pub fn SetConsoleMode(
        hConsoleHandle: HANDLE,
        lpMode: libc::DWORD,
    ) -> BOOL;
}
