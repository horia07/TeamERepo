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
