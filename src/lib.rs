//! A crate with utilities to determine the number of CPUs available on the
//! current system.
//!
//! Sometimes the CPU will exaggerate the number of CPUs it contains, because it can use
//! [processor tricks] to deliver increased performance when there are more threads. This 
//! crate provides methods to get both the logical and physical numbers of cores.
//!
//! This information can be used as a guide to how many tasks can be run in parallel.
//! There are many properties of the system architecture that will affect parallelism,
//! for example memory access speeds (for all the caches and RAM) and the physical
//! architecture of the processor, so the number of CPUs should be used as a rough guide
//! only.
//!
//!
//! ## Examples
//!
//! Fetch the number of logical CPUs.
//!
//! ```
//! let cpus = num_cpus::get();
//! ```
//!
//! See [`rayon::Threadpool`] for an example of where the number of CPUs could be
//! used when setting up parallel jobs (Where the threadpool example uses a fixed
//! number 8, it could use the number of CPUs).
//!
//! [processor tricks]: https://en.wikipedia.org/wiki/Simultaneous_multithreading
//! [`rayon::ThreadPool`]: https://docs.rs/rayon/1.*/rayon/struct.ThreadPool.html
#![no_std]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]
#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]
#![allow(non_snake_case)]

extern crate sgx_trts;

/// Return core count initialized by urts/trts suite
#[inline]
pub fn get() -> usize {
    sgx_trts::capi::sgx_get_cpu_core_num() as usize
}

/// Returns the number of physical cores of the current system
/// Same as `get()` under sgx
#[inline]
pub fn get_physical() -> usize {
    get()
}
