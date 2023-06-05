// use std::thread;
use std::sync::atomic::{
    Ordering::{Relaxed, Acquire, Release}, 
    AtomicU64, AtomicPtr
};

pub fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let mut key = KEY.load(Relaxed);

    if key == 0 {
        key = key + 1;
        match KEY.compare_exchange(0, key, Relaxed, Relaxed) {
            Ok(_) => key,
            Err(k) => k,
        }
    } else {
        key
    }
}

pub fn get_data() -> &'static String {
    static PTR: AtomicPtr<String> = AtomicPtr::new(std::ptr::null_mut());
    let mut p = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(String::from("Sankar boro")));
        if let Err(e) = PTR.compare_exchange(std::ptr::null_mut(), p, Release, Acquire) {
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    } else {
        // key
    }
    unsafe { &*p }
}