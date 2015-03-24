# uwsgi-rust
uWSGI and Rust integration plugin


This plugin is an experiemntal effort to support Rust runtime into uWSGI.

Consider it a proof of concept.

Installation
------------

Usage
-----

Currently uWSGI expects Rust apps to be managed as shared/dynamic libraries:

```rust
#![crate_type = "dylib"]
#![feature(collections)] 

use std::collections::HashMap;

#[no_mangle]
pub extern fn application(environ: HashMap<&str, &str>) -> (String, Vec<(String, String)>, Vec<Vec<u8>>) {

	for (k, v) in environ.iter() {
		println!("{} = {}", *k, *v);
	}

        let mut v = vec![];
        v.push((String::from_str("Content-Type"), String::from_str("text/plain")));
        v.push((String::from_str("Foo"), String::from_str("Bar")));

        let mut body = vec![];

        body.push(String::from_str("Hello").into_bytes());

        return (String::from_str("200 OK"), v, body);
}
```

running

```sh
rustc app.rs
```

will result in libapp.so/libapp.dylib

This new library can be loaded in the uWSGI core with the standard `--dlopen` option. Finally you only need to tell uWSGI which rust function to execute ('application' in this case) at every request:

```ini
[uwsgi]
; load the rust plugin (if needed, rememebr you can make monolithic builds)
plugin = rust
; bind to http port 8080
http-socket = :8080
; load the library app
dlopen = ./libapp.so
; set 'application' as the entry point
rust-fn = application

; enable master
master = true
; spawn 10 threads in each process/worker
threads = 10
; spawn 8 processes/workers
processes = 8
```

Status
------

prefork and layzy-apps modes are both supported

preforking works

multithreading works (included spawning threads from rust)

Notes
-----
