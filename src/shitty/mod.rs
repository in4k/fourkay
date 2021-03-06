use crate::bindings::Xlib;
use core::mem;
use core::ptr;

#[macro_use]
pub mod println;
pub mod gl_utils;
pub mod gl_wrapper;

pub fn sleep(milliseconds: i64) {
    let mut sleep_timeout = libc::timeval {
        tv_sec: 0,
        tv_usec: milliseconds * 1000,
    };
    unsafe {
        libc::select(
            1,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            &mut sleep_timeout,
        );
    }
}

pub fn xlib_events_ready(display: *mut Xlib::Display) -> i32 {
    unsafe {
        let x11_fd = Xlib::XConnectionNumber(display);
        let mut in_fds: libc::fd_set = mem::uninitialized();
        libc::FD_ZERO(&mut in_fds);
        libc::FD_SET(x11_fd, &mut in_fds);
        let mut select_timeout = libc::timeval {
            tv_sec: 0,
            tv_usec: 2_000,
        };
        let num_ready_fds = libc::select(
            x11_fd + 1,
            &mut in_fds,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut select_timeout,
        );
        num_ready_fds
    }
}

pub mod time {
    use core::mem;
    use core::time::Duration;

    pub fn now() -> libc::timespec {
        let mut time = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        update(&mut time);
        time
    }

    pub fn update(time: &mut libc::timespec) {
        unsafe {
            libc::clock_gettime(libc::CLOCK_REALTIME, time);
        }
    }

    pub fn subtract(left: &libc::timespec, right: &libc::timespec) -> Duration {
        Duration::new(left.tv_sec as u64, left.tv_nsec as u32)
            - Duration::new(right.tv_sec as u64, right.tv_nsec as u32)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_time() {
            let time = now();
            assert_ne!(time.tv_sec, 0);

            let time2 = now();
            let delta = subtract(&time2, &time);
            assert!(delta > Duration::from_secs(0));
        }
    }
}
