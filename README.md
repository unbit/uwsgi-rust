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

Status
------

prefork and layzy-apps modes are both supported

preforking works

multithreading works (included spawning threads from rust)

Notes
-----
