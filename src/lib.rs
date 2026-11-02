// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#![cfg_attr(not(clippy), allow(unexpected_cfgs))]
#![cfg_attr(not(clippy), allow(unknown_lints))]

#[allow(unused_macros)]
macro_rules! use_bindings {
    ($bindings:ident) => {
        mod $bindings;
        pub use $bindings::*;
    };
}

macro_rules! platform_binding {
    ($platform:ident, $platform_crypto:ident) => {
        #[cfg(all($platform, not(feature = "ssl"), not(use_bindgen_pregenerated)))]
        use_bindings!($platform_crypto);
    };
}

platform_binding!(universal, universal_crypto);

platform_binding!(aarch64_linux_android, aarch64_linux_android_crypto);
platform_binding!(aarch64_apple_darwin, aarch64_apple_darwin_crypto);
platform_binding!(aarch64_pc_windows_msvc, aarch64_pc_windows_msvc_crypto);
platform_binding!(aarch64_unknown_linux_gnu, aarch64_unknown_linux_gnu_crypto);
platform_binding!(
    aarch64_unknown_linux_musl,
    aarch64_unknown_linux_musl_crypto
);
platform_binding!(i686_pc_windows_msvc, i686_pc_windows_msvc_crypto);
platform_binding!(i686_unknown_linux_gnu, i686_unknown_linux_gnu_crypto);
platform_binding!(
    riscv64gc_unknown_linux_gnu,
    riscv64gc_unknown_linux_gnu_crypto
);
platform_binding!(x86_64_apple_darwin, x86_64_apple_darwin_crypto);
platform_binding!(x86_64_pc_windows_gnu, x86_64_pc_windows_gnu_crypto);
platform_binding!(x86_64_pc_windows_msvc, x86_64_pc_windows_msvc_crypto);
platform_binding!(x86_64_unknown_linux_gnu, x86_64_unknown_linux_gnu_crypto);
platform_binding!(x86_64_unknown_linux_musl, x86_64_unknown_linux_musl_crypto);

#[cfg(use_bindgen_pregenerated)]
#[allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::default_trait_access,
    clippy::doc_markdown,
    clippy::missing_safety_doc,
    clippy::must_use_candidate,
    clippy::possible_missing_else,
    clippy::ptr_as_ptr,
    clippy::ptr_offset_with_cast,
    clippy::pub_underscore_fields,
    clippy::semicolon_if_nothing_returned,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::used_underscore_binding,
    clippy::useless_transmute,
    dead_code,
    improper_ctypes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unpredictable_function_pointer_comparisons,
    unused_imports
)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
#[cfg(use_bindgen_pregenerated)]
pub use generated::*;

#[allow(non_snake_case, clippy::cast_possible_wrap)]
#[must_use]
pub fn ERR_GET_LIB(packed_error: u32) -> i32 {
    ((packed_error >> 24) & 0xFF) as i32
}

#[allow(non_snake_case, clippy::cast_possible_wrap)]
#[must_use]
pub fn ERR_GET_REASON(packed_error: u32) -> i32 {
    (packed_error & 0x0FFF) as i32
}

#[allow(non_snake_case)]
#[must_use]
pub fn ERR_GET_FUNC(_packed_error: u32) -> i32 {
    0
}

#[cfg(feature = "all-bindings")]
use std::os::raw::{c_char, c_long, c_void};

#[cfg(feature = "all-bindings")]
/// Returns the length of a memory BIO's current contents and, if `pp` is non-null,
/// writes a pointer to those contents to `*pp`.
///
/// A non-positive return value can indicate either failure or an empty buffer.
/// Prefer [`BIO_mem_contents`] when these cases need to be distinguished.
///
/// # Safety
///
/// `b` must point to a live, initialized memory BIO and remain valid throughout
/// the call. Any installed callbacks must satisfy the contract of `BIO_ctrl`
/// with `BIO_CTRL_INFO`. The caller must synchronize access to the BIO and its
/// backing storage so that no conflicting access, mutation, or destruction can
/// occur during the call.
///
/// `pp` may be null. Otherwise, it must be aligned and valid for writing one
/// `*mut c_char`, with no conflicting references or overlap with storage accessed
/// by the BIO during the call.
///
/// The output pointer is borrowed: this function transfers no ownership. Treat
/// its contents as read-only, even though the pointer is mutable. The caller
/// must keep the BIO and any externally owned backing buffer alive, and prevent
/// operations that invalidate or change the buffer, for as long as the output
/// is accessed. Access only the returned number of bytes after establishing
/// that the call succeeded and the output is non-null; the data need not be
/// NUL-terminated. An empty buffer can have a null output pointer.
///
/// Calling this function requires an unsafe block:
///
/// ```compile_fail,E0133
/// fn requires_unsafe(bio: *mut aws_lc_sys::BIO, output: *mut *mut std::os::raw::c_char) {
///     aws_lc_sys::BIO_get_mem_data(bio, output);
/// }
/// ```
#[allow(non_snake_case)]
pub unsafe fn BIO_get_mem_data(b: *mut BIO, pp: *mut *mut c_char) -> c_long {
    unsafe { BIO_ctrl(b, BIO_CTRL_INFO, 0, pp.cast::<c_void>()) }
}

pub fn init() {
    unsafe { CRYPTO_library_init() }
}

/// Always `0`: `aws-lc-sys` does not track an AWS-LC FIPS release branch.
#[must_use]
pub const fn fips_version() -> u32 {
    0
}
