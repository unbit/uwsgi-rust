use std::collections::HashMap;

#[no_mangle]
pub extern fn application(environ: HashMap<&str, &str>) -> (String, Vec<(String, String)>, Vec<Vec<u8>>) {

	for (k, v) in environ.iter() {
		println!("{} = {}", *k, *v);
	}

        let mut v = vec![];
        v.push((String::from("Content-Type"), String::from("text/plain")));
        v.push((String::from("Foo"), String::from("Bar")));

        let mut body = vec![];

        body.push(String::from("Hello").into_bytes());

        return (String::from("200 OK"), v, body);
}
