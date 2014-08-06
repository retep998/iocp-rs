
use c::{ERROR_INVALID_HANDLE};
use c::{HANDLE};
use libc::{c_int, pid_t};
use std::c_str::CString;
use std::mem::transmute;
use std::ptr::mut_null;
use std::rt::rtio;
use std::rt::rtio::{IoError, IoResult};
use std::sync::{Once, ONCE_INIT};

mod tty;

static mut IOCP_INIT: Once = ONCE_INIT;
static mut IOCP: *mut Iocp = 0 as *mut Iocp;

struct Iocp {
    handle: HANDLE,
}

impl Drop for Iocp {
    fn drop(&mut self) {
    }
}

pub struct Factory {
    iocp: &'static mut Iocp,
}

impl Factory {
    pub fn new() -> Factory {
        unsafe {
            IOCP_INIT.doit(|| {
                let iocp = box Iocp {
                    handle: mut_null(),
                };
                IOCP = transmute(iocp);
            });
            Factory {
                iocp: &mut *IOCP,
            }
        }
    }
}

impl rtio::IoFactory for Factory {
    fn tcp_connect(
        &mut self,
        addr: rtio::SocketAddr,
        timeout: Option<u64>
    ) -> IoResult<Box<rtio::RtioTcpStream + Send>> {
        iocpabort!("tcp_connect")
    }
    fn tcp_bind(
        &mut self,
        addr: rtio::SocketAddr
    ) -> IoResult<Box<rtio::RtioTcpListener + Send>> {
        iocpabort!("tcp_bind")
    }
    fn udp_bind(
        &mut self,
        addr: rtio::SocketAddr
    ) -> IoResult<Box<rtio::RtioUdpSocket + Send>> {
        iocpabort!("udp_bind")
    }
    fn unix_bind(
        &mut self,
        path: &CString
    ) -> IoResult<Box<rtio::RtioUnixListener + Send>> {
        iocpabort!("unix_bind")
    }
    fn unix_connect(
        &mut self,
        path: &CString,
        timeout: Option<u64>
    ) -> IoResult<Box<rtio::RtioPipe + Send>> {
        iocpabort!("unix_connect")
    }
    fn get_host_addresses(
        &mut self,
        host: Option<&str>,
        servname: Option<&str>,
        hint: Option<rtio::AddrinfoHint>
    ) -> IoResult<Vec<rtio::AddrinfoInfo>> {
        iocpabort!("get_host_addresses")
    }
    fn fs_from_raw_fd(
        &mut self,
        fd: c_int,
        close: rtio::CloseBehavior
    ) -> Box<rtio::RtioFileStream + Send> {
        iocpabort!("fs_from_raw_fd")
    }
    fn fs_open(
        &mut self,
        path: &CString,
        fm: rtio::FileMode,
        fa: rtio::FileAccess
    ) -> IoResult<Box<rtio::RtioFileStream + Send>> {
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
    ) -> IoResult<rtio::FileStat> {
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
    ) -> IoResult<rtio::FileStat> {
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
    ) -> IoResult<Box<rtio::RtioTimer + Send>> {
        iocpabort!("timer_init")
    }
    fn spawn(
        &mut self,
        cfg: rtio::ProcessConfig
    ) -> IoResult<(Box<rtio::RtioProcess + Send>, Vec<Option<Box<rtio::RtioPipe + Send>>>)> {
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
    ) -> IoResult<Box<rtio::RtioPipe + Send>> {
        iocpabort!("pipe_open")
    }
    fn tty_open(
        &mut self,
        fd: c_int,
        readable: bool
    ) -> IoResult<Box<rtio::RtioTTY + Send>> {
        if tty::is_tty(fd) {
            Ok(box tty::TTY::new(fd) as Box<rtio::RtioTTY + Send>)
        } else {
            Err(IoError {
                code: ERROR_INVALID_HANDLE as uint,
                extra: 0,
                detail: None,
            })
        }
    }
    fn signal(
        &mut self,
        signal: int,
        cb: Box<rtio::Callback + Send>
    ) -> IoResult<Box<rtio::RtioSignal + Send>> {
        iocpabort!("signal")
    }
}
