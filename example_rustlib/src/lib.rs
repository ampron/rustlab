// src/lib.rs
#![crate_type = "dylib"]

extern crate libc;
use std::ffi::CStr;

#[repr(C)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[no_mangle]
pub extern "C" fn hello(name: *const libc::c_char) {
    let buf_name = unsafe { CStr::from_ptr(name).to_bytes() };
    let str_name = String::from_utf8(buf_name.to_vec()).unwrap();
    println!("Hello {}!", str_name);
}

#[no_mangle]
pub extern "C" fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[no_mangle]
pub unsafe extern "C" fn create_vec3() -> *mut Point3 {
    let p = Box::new(Point3{ x: 0.0, y: 0.0, z: 0.0 });
    Box::into_raw(p)
}

#[no_mangle]
pub unsafe extern "C" fn free_vec3(rawp_x: *mut Point3) {
    let p_x = Box::from_raw(rawp_x);
    drop(p_x);
}

#[no_mangle]
pub unsafe extern "C" fn mult_array(rawp_a: *mut libc::c_double, len: libc::c_int) {
    assert!(rawp_a != std::ptr::null_mut());
    assert!(0 <= len);
    let array: &mut [f64] = std::slice::from_raw_parts_mut(rawp_a as *mut f64, len as usize);
    // multiply each element of the array by 3
    for x in array {
        (*x) *= 3.0;
    }
}