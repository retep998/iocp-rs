
mod tty;

struct Factory {
    iocp: c::HANDLE,
}

impl Factory {
    fn new() -> Factory {
        Factory {
            iocp: mut_null(),
        }
    }
}

impl IoFactory for Factory {
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
