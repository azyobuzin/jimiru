// TODO: 完全に Linux 専用なのをなんとかする

use std::io;
use std::mem;
use std::net;
use std::os::unix::io::*;
use libc;

pub struct UdpSocket {
    fd: RawFd,
}

impl UdpSocket {
    pub fn new() -> io::Result<UdpSocket> {
        let fd = unsafe {
            libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0)
        };
        if fd >= 0 { Ok(UdpSocket { fd }) }
        else { Err(io::Error::last_os_error()) }
    }

    pub fn sendto(&self, buf: &[u8], addr: net::SocketAddrV4) -> io::Result<usize> {
        // SocketAddrV4 の中身はまんま sockaddr_in
        let addr: libc::sockaddr_in = unsafe { mem::transmute(addr) };
        let sent = unsafe {
            libc::sendto(
                self.fd,
                buf.as_ptr() as *const libc::c_void,
                buf.len(),
                0,
                &addr as *const libc::sockaddr_in as *const libc::sockaddr,
                mem::size_of::<libc::sockaddr_in>() as libc::socklen_t
            )
        };
        if sent >= 0 { Ok(sent as usize) }
        else { Err(io::Error::last_os_error()) }
    }
}

impl Drop for UdpSocket {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd); }
    }
}

impl AsRawFd for UdpSocket {
    fn as_raw_fd(&self) -> RawFd { self.fd }
}

impl FromRawFd for UdpSocket {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        UdpSocket { fd }
     }
}

impl IntoRawFd for UdpSocket {
    fn into_raw_fd(self) -> RawFd {
        let fd = self.fd;
        mem::forget(self);
        fd
    }
}
