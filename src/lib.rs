
#![feature(macro_rules)]

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

mod c;
mod macros;
mod tty;

struct IocpLoop {
    factory: IocpFactory,
    work: Vec<proc(): Send>,
    remotes: Vec<(uint, Box<Callback + Send>)>,
    next_remote: uint,
    messages: Arc<Exclusive<Vec<Message>>>,
    idle: Option<Box<Callback + Send>>,
    idle_active: Option<Arc<atomics::AtomicBool>>,
}

impl IocpLoop {
    fn new() -> IocpLoop {
        IocpLoop {
            factory: IocpFactory::new(),
            work: vec![],
            idle: None,
            idle_active: None,
            next_remote: 0,
            remotes: vec![],
            messages: Arc::new(Exclusive::new(Vec::new())),
        }
    }
    fn work(&mut self) {
        while self.work.len() > 0 {
            for work in mem::replace(&mut self.work, vec![]).move_iter() {
                work();
            }
        }
    }
    fn remote_work(&mut self) {
        let messages = unsafe {
            mem::replace(&mut *self.messages.lock(), Vec::new())
        };
        for message in messages.move_iter() {
            self.message(message);
        }
    }
    fn message(
        &mut self,
        message: Message
    ) {
        match message {
            RunRemote(i) => {
                match self.remotes.mut_iter().find(|& &(id, _)| id == i) {
                    Some(&(_, ref mut f)) => f.call(),
                    None => unreachable!()
                }
            }
            RemoveRemote(i) => {
                match self.remotes.iter().position(|&(id, _)| id == i) {
                    Some(i) => { self.remotes.remove(i).unwrap(); }
                    None => unreachable!()
                }
            }
        }
    }
    fn idle(&mut self) {
        match self.idle {
            Some(ref mut idle) => {
                if self.idle_active.get_ref().load(atomics::SeqCst) {
                    idle.call();
                }
            }
            None => {}
        }
    }
    fn has_idle(&self) -> bool {
        self.idle.is_some() && self.idle_active.get_ref().load(atomics::SeqCst)
    }
}

impl EventLoop for IocpLoop {
    fn run(&mut self) {
        while self.remotes.len() > 0 || self.work.len() > 0 || self.has_idle() {
            self.work();
            self.remote_work();
            if self.has_idle() {
                self.idle();
                continue
            }
            unsafe {
                let mut messages = self.messages.lock();
                if self.remotes.len() > 0 && messages.len() == 0 &&
                   self.work.len() == 0 {
                    messages.wait()
                }
            }
        }
    }
    fn callback(
        &mut self,
        cb: proc(): Send
    ) {
        self.work.push(cb);
    }
    fn pausable_idle_callback(
        &mut self,
        cb: Box<Callback + Send>
    ) -> Box<PausableIdleCallback + Send> {
        iocpassert!(self.idle.is_none());
        self.idle = Some(cb);
        let a = Arc::new(atomics::AtomicBool::new(true));
        self.idle_active = Some(a.clone());
        box BasicPausable { active: a } as Box<PausableIdleCallback + Send>
    }
    fn remote_callback(
        &mut self,
        cb: Box<Callback + Send>
    ) -> Box<RemoteCallback + Send> {
        let id = self.next_remote;
        self.next_remote += 1;
        self.remotes.push((id, cb));
        box BasicRemote::new(self.messages.clone(), id) as
            Box<RemoteCallback + Send>
    }
    fn io<'a>(
        &'a mut self
    ) -> Option<&'a mut IoFactory> {
        Some(&mut self.factory as &mut IoFactory)
    }
    fn has_active_io(&self) -> bool {
        false
    }
}

enum Message {
    RunRemote(uint),
    RemoveRemote(uint)
}

struct BasicRemote {
    queue: Arc<Exclusive<Vec<Message>>>,
    id: uint,
}

impl BasicRemote {
    fn new(
        queue: Arc<Exclusive<Vec<Message>>>,
        id: uint
    ) -> BasicRemote {
        BasicRemote { queue: queue, id: id }
    }
}

impl RemoteCallback for BasicRemote {
    fn fire(&mut self) {
        let mut queue = unsafe { self.queue.lock() };
        queue.push(RunRemote(self.id));
        queue.signal();
    }
}

impl Drop for BasicRemote {
    fn drop(&mut self) {
        let mut queue = unsafe { self.queue.lock() };
        queue.push(RemoveRemote(self.id));
        queue.signal();
    }
}

struct BasicPausable {
    active: Arc<atomics::AtomicBool>,
}

impl PausableIdleCallback for BasicPausable {
    fn pause(&mut self) {
        self.active.store(false, atomics::SeqCst);
    }
    fn resume(&mut self) {
        self.active.store(true, atomics::SeqCst);
    }
}

impl Drop for BasicPausable {
    fn drop(&mut self) {
        self.active.store(false, atomics::SeqCst);
    }
}

struct IocpFactory {
    iocp: c::HANDLE,
}

impl IocpFactory {
    fn new() -> IocpFactory {
        IocpFactory {
            iocp: mut_null(),
        }
    }
}

impl IoFactory for IocpFactory {
    fn tcp_connect(
        &mut self,
        addr: SocketAddr,
        timeout: Option<u64>
    ) -> IoResult<Box<RtioTcpStream + Send>> {
        iocpabort!("tcp_connect")
    }
    fn tcp_bind(
        &mut self,
        addr: SocketAddr
    ) -> IoResult<Box<RtioTcpListener + Send>> {
        iocpabort!("tcp_bind")
    }
    fn udp_bind(
        &mut self,
        addr: SocketAddr
    ) -> IoResult<Box<RtioUdpSocket + Send>> {
        iocpabort!("udp_bind")
    }
    fn unix_bind(
        &mut self,
        path: &CString
    ) -> IoResult<Box<RtioUnixListener + Send>> {
        iocpabort!("unix_bind")
    }
    fn unix_connect(
        &mut self,
        path: &CString,
        timeout: Option<u64>
    ) -> IoResult<Box<RtioPipe + Send>> {
        iocpabort!("unix_connect")
    }
    fn get_host_addresses(
        &mut self,
        host: Option<&str>,
        servname: Option<&str>,
        hint: Option<AddrinfoHint>
    ) -> IoResult<Vec<AddrinfoInfo>> {
        iocpabort!("get_host_addresses")
    }
    fn fs_from_raw_fd(
        &mut self,
        fd: c_int,
        close: CloseBehavior
    ) -> Box<RtioFileStream + Send> {
        iocpabort!("fs_from_raw_fd")
    }
    fn fs_open(
        &mut self,
        path: &CString,
        fm: FileMode,
        fa: FileAccess
    ) -> IoResult<Box<RtioFileStream + Send>> {
        iocpabort!("fs_open")
    }
    fn fs_unlink(
        &mut self,
        path: &CString
    ) -> IoResult<()> {
        iocpabort!("fs_unlink")
    }
    fn fs_stat(
        &mut self,
        path: &CString
    ) -> IoResult<FileStat> {
        iocpabort!("fs_stat")
    }
    fn fs_mkdir(
        &mut self,
        path: &CString,
        mode: uint
    ) -> IoResult<()> {
        iocpabort!("fs_mkdir")
    }
    fn fs_chmod(
        &mut self,
        path: &CString,
        mode: uint
    ) -> IoResult<()> {
        iocpabort!("fs_chmod")
    }
    fn fs_rmdir(
        &mut self,
        path: &CString
    ) -> IoResult<()> {
        iocpabort!("fs_rmdir")
    }
    fn fs_rename(
        &mut self,
        path: &CString,
        to: &CString
    ) -> IoResult<()> {
        iocpabort!("fs_rename")
    }
    fn fs_readdir(
        &mut self,
        path: &CString,
        flags: c_int
    ) -> IoResult<Vec<CString>> {
        iocpabort!("fs_readdir")
    }
    fn fs_lstat(
        &mut self,
        path: &CString
    ) -> IoResult<FileStat> {
        iocpabort!("fs_lstat")
    }
    fn fs_chown(
        &mut self,
        path: &CString,
        uid: int,
        gid: int
    ) -> IoResult<()> {
        iocpabort!("fs_chown")
    }
    fn fs_readlink(
        &mut self,
        path: &CString
    ) -> IoResult<CString> {
        iocpabort!("fs_readlink")
    }
    fn fs_symlink(
        &mut self,
        src: &CString,
        dst: &CString
    ) -> IoResult<()> {
        iocpabort!("fs_symlink")
    }
    fn fs_link(
        &mut self,
        src: &CString,
        dst: &CString
    ) -> IoResult<()> {
        iocpabort!("fs_link")
    }
    fn fs_utime(
        &mut self,
        src: &CString,
        atime: u64,
        mtime: u64
    ) -> IoResult<()> {
        iocpabort!("fs_utime")
    }
    fn timer_init(
        &mut self
    ) -> IoResult<Box<RtioTimer + Send>> {
        iocpabort!("timer_init")
    }
    fn spawn(
        &mut self,
        cfg: ProcessConfig
    ) -> IoResult<(Box<RtioProcess + Send>, Vec<Option<Box<RtioPipe + Send>>>)> {
        iocpabort!("spawn")
    }
    fn kill(
        &mut self,
        pid: pid_t,
        signal: int
    ) -> IoResult<()> {
        iocpabort!("kill")
    }
    fn pipe_open(
        &mut self,
        fd: c_int
    ) -> IoResult<Box<RtioPipe + Send>> {
        iocpabort!("pipe_open")
    }
    fn tty_open(
        &mut self,
        fd: c_int,
        readable: bool
    ) -> IoResult<Box<RtioTTY + Send>> {
        if tty::is_tty(fd) {
            Ok(box tty::TTY::new(fd) as Box<RtioTTY + Send>)
        } else {
            Err(IoError {
                code: libc::ERROR_INVALID_HANDLE as uint,
                extra: 0,
                detail: None,
            })
        }
    }
    fn signal(
        &mut self,
        signal: int,
        cb: Box<Callback + Send>
    ) -> IoResult<Box<RtioSignal + Send>> {
        iocpabort!("signal")
    }
}

pub fn event_loop() -> Box<EventLoop + Send> {
    box IocpLoop::new() as Box<EventLoop + Send>
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
