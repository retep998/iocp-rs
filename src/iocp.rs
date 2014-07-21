
extern crate libc;

#[allow(non_camel_case_types)]
mod iocp {
    use libc;
    use std::io::{IoResult, IoError};
    use std::io::EndOfFile;
    use std::mem::uninitialized;
    use std::ptr::mut_null;
    type BOOL = libc::c_int;
    type DWORD = libc::c_ulong;
    type HANDLE = *mut VOID;
    type LPCVOID = *const VOID;
    type LPCWSTR = *const WCHAR;
    type LPDWORD = *mut DWORD;
    type LPOVERLAPPED = *mut OVERLAPPED;
    type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
    type LPVOID = *mut VOID;
    type PULONG_PTR = *mut ULONG_PTR;
    type ULONG_PTR = uint;
    type VOID = libc::c_void;
    type WCHAR = u16;
    static CREATE_NEW: DWORD = 1;
    static ERROR_HANDLE_EOF: DWORD = 38;
    static ERROR_IO_PENDING: DWORD = 997;
    static FILE_FLAG_OVERLAPPED: DWORD = 0x40000000;
    static FILE_SHARE_READ: DWORD = 0x00000001;
    static GENERIC_READ: DWORD = 0x80000000;
    static GENERIC_WRITE: DWORD = 0x40000000;
    static INFINITE: DWORD = 0xFFFFFFFF;
    static INVALID_HANDLE_VALUE: HANDLE = -1 as HANDLE;
    static OPEN_EXISTING: DWORD = 3;
    extern "system" {
        fn CreateFileW(
            lpFileName: LPCWSTR,
            dwDesiredAccess: DWORD,
            dwShareMode: DWORD,
            lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
            dwCreationDisposition: DWORD,
            dwFlagsAndAttributes: DWORD,
            hTemplateFile: HANDLE,
        ) -> HANDLE;
        fn CreateIoCompletionPort(
            FileHandle: HANDLE,
            ExistingCompletionPort: HANDLE,
            CompletionKey: ULONG_PTR,
            NumberOfConcurrentThreads: DWORD,
        ) -> HANDLE;
        fn GetLastError() -> DWORD;
        fn GetQueuedCompletionStatus(
            CompletionPort: HANDLE,
            lpNumberOfBytesTransferred: LPDWORD,
            lpCompletionKey: PULONG_PTR,
            lpOverlapped: *mut LPOVERLAPPED,
            dwMilliseconds: DWORD,
        ) -> BOOL;
        fn ReadFile(
            hFile: HANDLE,
            lpBuffer: LPVOID,
            nNumberOfBytesToRead: DWORD,
            lpNumberOfBytesRead: LPDWORD,
            lpOverlapped: LPOVERLAPPED,
        ) -> BOOL;
        fn WriteFile(
            hFile: HANDLE,
            lpBuffer: LPCVOID,
            nNumberOfBytesToWrite: DWORD,
            lpNumberOfBytesWritten: LPDWORD,
            lpOverlapped: LPOVERLAPPED,
        ) -> BOOL;
    }
    struct SECURITY_ATTRIBUTES;
    #[deriving(Show)]
    #[allow(uppercase_variables)]
    struct OVERLAPPED {
        Internal: ULONG_PTR,
        InternalHigh: ULONG_PTR,
        Offset: DWORD,
        OffsetHigh: DWORD,
        hEvent: HANDLE,
    }
    impl OVERLAPPED {
        fn new(pos: u64) -> OVERLAPPED {
            OVERLAPPED {
                Internal: 0,
                InternalHigh: 0,
                Offset: pos as u32,
                OffsetHigh: (pos >> 32) as u32,
                hEvent: mut_null(),
            }
        }
    }
    #[deriving(Show)]
    pub struct File {
        handle: HANDLE,
        port: HANDLE,
        pos: u64,
    }
    impl File {
        pub fn open(name: &Path) -> IoResult<File> {
            let handle = unsafe { CreateFileW(
                utf16(name.as_str().unwrap()).as_ptr(),
                GENERIC_READ,
                FILE_SHARE_READ,
                mut_null(),
                OPEN_EXISTING,
                FILE_FLAG_OVERLAPPED,
                mut_null(),
            ) };
            if handle == INVALID_HANDLE_VALUE {
                return Err(IoError::last_error());
            }
            let port = unsafe {
                CreateIoCompletionPort(handle, mut_null(), 0, 0)
            };
            if port == mut_null() {
                return Err(IoError::last_error());
            }
            Ok(File {
                handle: handle,
                port: port,
                pos: 0,
            })
        }
        pub fn create(name: &Path) -> IoResult<File> {
            let handle = unsafe { CreateFileW(
                utf16(name.as_str().unwrap()).as_ptr(),
                GENERIC_WRITE,
                FILE_SHARE_READ,
                mut_null(),
                CREATE_NEW,
                FILE_FLAG_OVERLAPPED,
                mut_null(),
            ) };
            if handle == INVALID_HANDLE_VALUE {
                return Err(IoError::last_error());
            }
            let port = unsafe {
                CreateIoCompletionPort(handle, mut_null(), 0, 0)
            };
            if port == mut_null() {
                return Err(IoError::last_error());
            }
            Ok(File {
                handle: handle,
                port: port,
                pos: 0,
            })
        }
    }
    impl Reader for File {
        fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
            let mut over = OVERLAPPED::new(self.pos);
            if unsafe { ReadFile(
                self.handle,
                buf.as_mut_ptr() as LPVOID,
                buf.len() as DWORD,
                mut_null(),
                &mut over as LPOVERLAPPED,
            ) } == 0 {
                match unsafe { GetLastError() } {
                    ERROR_IO_PENDING => (),
                    err => return Err(IoError::from_errno(err as uint, true)),
                }
            }
            let mut bytes = unsafe { uninitialized() };
            let mut key = unsafe { uninitialized() };
            let mut over = unsafe { uninitialized() };
            if unsafe { GetQueuedCompletionStatus(
                self.port,
                &mut bytes as LPDWORD,
                &mut key as PULONG_PTR,
                &mut over as *mut LPOVERLAPPED,
                INFINITE,
            ) } == 0 {
                match unsafe { GetLastError() } {
                    ERROR_HANDLE_EOF => return Err(IoError {
                        kind: EndOfFile,
                        desc: "end of file",
                        detail: None,
                    }),
                    err => return Err(IoError::from_errno(err as uint, true)),
                }
            }
            self.pos = self.pos + bytes as u64;
            Ok(bytes as uint)
        }
    }
    impl Writer for File {
        fn write(&mut self, buf: &[u8]) -> IoResult<()> {
            let mut over = OVERLAPPED::new(self.pos);
            if unsafe { WriteFile(
                self.handle,
                buf.as_ptr() as LPCVOID,
                buf.len() as DWORD,
                mut_null(),
                &mut over as LPOVERLAPPED,
            ) } == 0 {
                match unsafe { GetLastError() } {
                    ERROR_IO_PENDING => (),
                    err => return Err(IoError::from_errno(err as uint, true)),
                }
            }
            let mut bytes = unsafe { uninitialized() };
            let mut key = unsafe { uninitialized() };
            let mut over = unsafe { uninitialized() };
            if unsafe { GetQueuedCompletionStatus(
                self.port,
                &mut bytes as LPDWORD,
                &mut key as PULONG_PTR,
                &mut over as *mut LPOVERLAPPED,
                INFINITE,
            ) } == 0 {
                match unsafe { GetLastError() } {
                    ERROR_HANDLE_EOF => return Err(IoError {
                        kind: EndOfFile,
                        desc: "end of file",
                        detail: None,
                    }),
                    err => return Err(IoError::from_errno(err as uint, true)),
                }
            }
            if buf.len() != bytes as uint {
                fail!("ow");
            }
            Ok(())
        }
    }
    fn utf16(s: &str) -> Vec<u16> {
        s.utf16_units().collect::<Vec<u16>>().append_one(0)
    }
}

fn main() {
    let mut file = iocp::File::open(&Path::new("../src/iocp.rs")).unwrap();
    let s = file.read_to_string().unwrap();
    let mut file = iocp::File::create(&Path::new("dump.txt")).unwrap();
    file.write_str(s.as_slice()).unwrap();
}
