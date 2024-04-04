use std::collections::HashMap;
use std::sync::OnceLock;

struct Signal {
    action: Box<dyn Fn() -> ()>,
}

struct GlobalData {
    signals: HashMap<libc::c_int, Signal>,
}

static mut GLOBAL_DATA: OnceLock<GlobalData> = OnceLock::new();

impl GlobalData {
    fn get() -> &'static Self {
        unsafe {
            GLOBAL_DATA.get_or_init(|| GlobalData {
                signals: HashMap::new(),
            })
        }
    }
}

extern "C" fn handler(sig: libc::c_int, _info: *mut libc::siginfo_t, _data: *mut libc::c_void) {
    let global_data = GlobalData::get();

    (global_data.signals.get(&sig).unwrap().action)();
}

pub fn register<P>(signal: libc::c_int, pipe: P)
where
    P: std::os::fd::IntoRawFd + 'static + Sync + Send,
{
    let raw_fd = pipe.into_raw_fd();

    let action = move || {
        let data = b"X" as *const _ as *const _;
        unsafe { libc::write(raw_fd, data, 1) };
    };

    let _ = GlobalData::get();
    unsafe {
        GLOBAL_DATA.get_mut().unwrap().signals.insert(
            signal,
            Signal {
                action: Box::new(action),
            },
        )
    };

    let mut new: libc::sigaction = unsafe { core::mem::zeroed() };
    new.sa_sigaction = handler as usize;
    new.sa_flags = libc::SA_SIGINFO;
    let mut old: libc::sigaction = unsafe { core::mem::zeroed() };

    unsafe { libc::sigaction(libc::SIGWINCH, &new, &mut old) };
}
