use std::mem;

pub fn fill_random(buf: &mut [u8]) {
    unsafe {
        libc::getrandom(buf.as_mut_ptr() as _, buf.len(), 0);
    }
}

pub unsafe fn setsockopt<T>(fd: i32, opt: libc::c_int, val: T) -> i32 {
    let val = &val as *const T as *const libc::c_void;
    let res = libc::setsockopt(
        fd,
        libc::IPPROTO_TCP,
        opt,
        val,
        std::mem::size_of::<T>() as libc::socklen_t,
    );

    return res;
}

pub unsafe fn getsockopt<T>(fd: i32, opt: libc::c_int) -> T {
    let mut val: mem::MaybeUninit<T> = mem::MaybeUninit::uninit();
    let mut len = mem::size_of::<T>() as libc::socklen_t;
    libc::getsockopt(
        fd,
        libc::IPPROTO_TCP,
        opt,
        val.as_mut_ptr().cast(),
        &mut len,
    );

    return val.assume_init();
}
