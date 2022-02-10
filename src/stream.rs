use crate::*;

// pub struct BreadcastStream {
//     pub(crate) fd: FileDesc,
//     pub(crate) _name: String,
// }
//
// pub struct IsotpStream {
//     pub(crate) fd: FileDesc,
//     pub(crate) _name: String,
// }

// Ownership movement should occur here. Read deeper into the FileDesc/OwnedFd system?
pub struct RawStream {
    pub(crate) fd: FileDesc,
    pub(crate) _name: String,
}

impl RawStream {
    pub fn recv(&mut self, frame: &mut CANFDFrame, flags: c_int) -> Result<usize, std::io::Error> {
        self.recvfrom(frame, flags, None)
    }

    pub fn recvfrom(
        &mut self,
        frame: &mut CANFDFrame,
        flags: c_int,
        src_addr: Option<&mut CANAddr>,
    ) -> Result<usize, std::io::Error> {
        let (addr, addrlen): (*mut libc::sockaddr, *mut libc::socklen_t) = match src_addr {
            Some(_) => {
                panic!("unimplemented recvfrom with filled sender")
            }
            // Some(addr) => (
            //     (&mut addr.inner as *mut CANAddrInner) as *mut libc::sockaddr,
            //     (std::mem::size_of::<CANAddrInner>() as c_uint) as *mut libc::socklen_t,
            // ),
            None => (std::ptr::null_mut(), std::ptr::null_mut()),
        };

        unsafe {
            let ret = libc::recvfrom(
                self.as_raw_fd(),
                (frame as *mut CANFDFrame) as *mut c_void,
                std::mem::size_of::<CANFDFrame>(),
                flags,
                addr,
                addrlen,
            );
            if ret < 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(ret as usize)
        }
    }

    pub fn send(&self, frame: &CANFDFrame, flags: c_int) -> Result<usize, std::io::Error> {
        self.sendto(frame, flags, None)
    }

    pub fn sendto(
        &self,
        frame: &CANFDFrame,
        flags: c_int,
        dest_addr: Option<&CANAddr>,
    ) -> Result<usize, std::io::Error> {
        let (addr, addr_len): (*const libc::sockaddr, libc::socklen_t) = match dest_addr {
            Some(_) => {
                panic!("unimplemented sendto with filled sender")
            }
            None => (std::ptr::null(), 0),
        };

        unsafe {
            let ret = libc::sendto(
                self.as_raw_fd(),
                (frame as *const CANFDFrame) as *const c_void,
                std::mem::size_of::<CANFDFrame>(),
                flags,
                addr,
                addr_len,
            );
            if ret < 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(ret as usize)
        }
    }
}

impl Read for RawStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut frame: CANFDFrame = CANFDFrame::new();
        self.recv(&mut frame, 0)?;
        buf[..(frame.len as usize)].copy_from_slice(&frame.data[..(frame.len as usize)]);
        Ok(frame.len as usize)
    }
}

impl AsRawFd for RawStream {
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }
}

impl IntoRawFd for RawStream {
    fn into_raw_fd(self) -> RawFd {
        self.fd.into_raw_fd()
    }
}
