#![allow(unused_mut)]

extern crate dylib;
extern crate libc;

use dylib::DynamicLibrary;
use libc::c_void;
use std::collections::HashMap;
use std::str;
use std::slice;

// global access to the function entry point (could become a vector to support multple apps)
static mut app : Option<extern fn(HashMap<&str, &str>) -> (String, Vec<(String, String)>, Vec<Vec<u8>>)> = None;

// C functions used by Rust
extern {
	fn uwsgi_response_prepare_headers(wsgi_req: *mut c_void, buf: *mut u8, buf_len: u16) -> i32;
	fn uwsgi_response_add_header(wsgi_req: *mut c_void, key: *mut u8, key_len: u16, val: *mut u8, val_len: u16) -> i32;
	fn uwsgi_response_write_body_do(wsgi_req: *mut c_void, buf: *mut u8, buf_len: u64) -> i32;

	fn uwsgi_rust_build_environ(wsgi_req: *mut c_void, environ: &HashMap<&str, &str>) -> i32;
}

// load the function entry point
#[no_mangle]
pub extern fn rust_load_fn(name: *mut u8, name_len: u16) -> i32 {
	let lib = match DynamicLibrary::open(None) {
                Ok(lib) => lib,
                Err(e) => { println!("[rust] {}", e); return -1},
        };

	let fn_name_slice = unsafe { slice::from_raw_parts(name, name_len as usize) };
	let fn_name = match str::from_utf8(fn_name_slice) {
		Ok(s) => s,
		Err(e) => { println!("[rust] {}", e); return -1 },
	};

	unsafe {
        	app = match lib.symbol(fn_name) {
                        Err(e) => { println!("[rust] {}", e); return -1},
                        Ok(f) => Some(std::mem::transmute::<*mut u8, extern fn(HashMap<&str, &str>) -> (String, Vec<(String, String)>, Vec<Vec<u8>>)>(f)),
                }
        };

	return 0;
}

// populate the environ HashMap with CGI vars
#[no_mangle]
pub extern fn rust_add_environ(environ: *mut HashMap<&str, &str>, key: *mut u8, key_len: u16, val: *mut u8, val_len: u16) -> i32 {
	let k = unsafe { slice::from_raw_parts(key, key_len as usize) };
	let sk = match str::from_utf8(k) {
		Ok(s) => s,
		Err(e) => { println!("[rust] {}", e); return -1 },
	};

	let v = unsafe { slice::from_raw_parts(val, val_len as usize) };
	let sv = match str::from_utf8(v) {
		Ok(s) => s,
		Err(e) => { println!("[rust] {}", e); return -1 },
	};

	unsafe {
		(*environ).insert(sk, sv);
	}

	return 0;
}

// run the entry point and send its response to the client
#[no_mangle]
pub extern fn rust_request_handler(wsgi_req: *mut c_void) -> i32 {

	let mut environ = HashMap::new();

	unsafe {
		if uwsgi_rust_build_environ(wsgi_req, &environ) != 0 {
			return -1;
		}
	}

	let entry_point;
	
	unsafe {
		entry_point = match app {
			None => return -1,
			Some(f) => f,
		};
	};

	let (status, headers, body) = entry_point(environ);

	unsafe {
		let ret = uwsgi_response_prepare_headers(wsgi_req, status.as_ptr() as *mut u8, status.into_bytes().len() as u16);
		if ret != 0 {
			return ret;
		}
	}

	for header in headers {
		unsafe {
			let ret = uwsgi_response_add_header(wsgi_req, header.0.as_ptr() as *mut u8, header.0.into_bytes().len() as u16,
				header.1.as_ptr() as *mut u8, header.1.into_bytes().len() as u16);
			if ret != 0 {
				return ret;
			}
		}
	}

	for chunk in body {
		unsafe {
			let ret = uwsgi_response_write_body_do(wsgi_req, chunk.as_ptr() as *mut u8, chunk.len() as u64);
			if ret != 0 {
				return ret;
			}
		}
	}

	return 0;
}
