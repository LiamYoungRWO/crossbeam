#![feature(fnbox)]
#![feature(box_patterns)]
#![feature(box_raw)]
#![feature(const_fn)]
#![feature(optin_builtin_traits)]
#![feature(drain)]

pub mod atomic_option;
pub mod mem;
pub mod queue;

mod raw_thread;
mod cache_padded;