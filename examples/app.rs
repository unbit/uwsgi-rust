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
