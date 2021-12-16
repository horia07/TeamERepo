use std::{io, mem, os::unix::io::AsRawFd};

pub unsafe fn setsockopt<T>(fd: i32, level: libc::c_int, opt: libc::c_int, val: T) -> i32 {
    let val = &val as *const T as *const libc::c_void;
    let res = libc::setsockopt(
        fd,
        level,
        opt,
        val,
        std::mem::size_of::<T>() as libc::socklen_t,
    );

    return res;
}

pub unsafe fn getsockopt<T>(fd: i32, level: libc::c_int, opt: libc::c_int) -> io::Result<T> {
    let mut val: mem::MaybeUninit<T> = mem::MaybeUninit::uninit();
    let mut len = mem::size_of::<T>() as libc::socklen_t;
    wrap_io_err(libc::getsockopt(
        fd,
        level,
        opt,
        val.as_mut_ptr().cast(),
        &mut len,
    ))?;

    return Ok(val.assume_init());
}

pub fn wrap_io_err(res: i32) -> Result<i32, io::Error> {
    match res {
        -1 => return Err(io::Error::last_os_error()),
        _ => Ok(res),
    }
}

pub unsafe fn sendfile<I, O>(out_fd: O, in_fd: I, count: usize) -> io::Result<usize>
where
    I: AsRawFd,
    O: AsRawFd,
{
    let offset: libc::off_t = 0;

    let res = libc::sendfile(out_fd.as_raw_fd(), in_fd.as_raw_fd(), offset as _, count);

    match res {
        -1 => Err(io::Error::last_os_error()),
        n => Ok(n as usize),
    }
}
