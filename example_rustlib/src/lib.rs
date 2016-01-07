// src/lib.rs
#![crate_type = "dylib"]

use std::collections::VecDeque;
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

struct SumBuffer {
    kern_buff: VecDeque<f64>,
    kern_sum: f64,
}

impl SumBuffer {
    pub fn from_range(x_rng: &[f64]) -> SumBuffer {
        let mut sbuf = SumBuffer{ kern_buff: VecDeque::with_capacity(x_rng.len() + 1), kern_sum: 0.0 };
        for x in x_rng {
            sbuf.kern_buff.push_back(*x);
            sbuf.kern_sum = sbuf.kern_sum + x;
        }
        return sbuf;
    }
    
    pub fn push(&mut self, x: f64) {
        self.kern_buff.push_back(x);
        let y: f64 = self.kern_buff.pop_front().unwrap();
        self.kern_sum = self.kern_sum + x - y;
    }
    
    pub fn average(&self) -> f64 {
        self.kern_sum / (self.kern_buff.len() as f64)
    }
}

fn interal_nn_smooth(x: &[f64], n: usize) -> Vec<f64> {
    let w = 2 * n + 1;
    let mut sbuf = SumBuffer::from_range(&x[0..w]);
    
    let mut sx = vec![0.0; x.len() - 2 * n];
    let mut i_sx = 0;
    for i in n..(x.len() - n - 1) {
        sbuf.push(x[i]);
        sx[i_sx] = sbuf.average();
        
        i_sx = i_sx + 1;
    }
    return sx;
}

#[no_mangle]
pub unsafe extern "C" fn nn_smooth(rawp_a: *mut libc::c_double, len: libc::c_int, n: libc::c_int) {
    assert!(rawp_a != std::ptr::null_mut());
    assert!(0 <= len);
    assert!(0 <= n && n < len / 4);
    let array: &mut [f64] = std::slice::from_raw_parts_mut(rawp_a as *mut f64, len as usize);
    let sm_array: Vec<f64> = interal_nn_smooth(array, n as usize);
    let mut i_sm = 0;
    let un = n as usize;
    for i in un..(array.len() - un - 1) {
        array[i] = sm_array[i_sm];
        i_sm = i_sm + 1;
    }
}